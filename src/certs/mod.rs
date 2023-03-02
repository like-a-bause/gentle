use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use anyhow::Result;
use tracing::*;
use std::io::Write;
use k8s_openapi::api::core::v1::{Secret};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use k8s_openapi::ByteString;
use kube::{Api, Client};
use kube::api::{Patch, PatchParams};
use rcgen::{DistinguishedName, DnType, DnValue};
use crate::utils;

pub async fn install_cert_manager() -> Result<()> {
    info!("Installing cert-manager");
    utils::run_process("helm",
                       [
                           "upgrade",
                           "--install",
                           "cert-manager",
                           "cert-manager",
                           "--repo",
                           "https://charts.jetstack.io",
                           "--namespace",
                           "cert-manager",
                           "--create-namespace",
                           "--set",
                           "installCRDs=true"
                       ],
    )?;
    Ok(())
}

pub async fn generate_certificate_authority(client: Client) -> Result<()> {
    let ca_path = "tls.crt";
    let private_key_path = "tls.key";

    let private_key;
    let pem;

    if !std::path::Path::new(ca_path).exists() {
        info!("Generating Cert...");
        let alt_names = vec![String::from("Gentle Certificate Authority")];
        let mut params = rcgen::CertificateParams::new(alt_names);
        params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
        let mut dn = DistinguishedName::new();
        dn.push(DnType::OrganizationName, "Gentle GmbH");
        dn.push(DnType::CommonName, DnValue::PrintableString("Gentle Certificate Authority".to_string()));
        params.distinguished_name = dn;
        let ca = rcgen::Certificate::from_params(params)?;

        private_key = ca.serialize_private_key_pem();
        pem = ca.serialize_pem()?;

        // write out crt
        let mut out = File::create(ca_path)?;
        write!(out, "{}", pem)?;

        let mut pkf = File::create(private_key_path)?;
        write!(pkf, "{}", private_key)?;
    } else {
        info!("Certificate already exists. Reading from file...");
        private_key = fs::read_to_string(private_key_path)?;
        pem = fs::read_to_string(ca_path)?;
    }

    let mut secret_data = BTreeMap::new();
    secret_data.insert(String::from("tls.crt"), ByteString(pem.into_bytes()));
    secret_data.insert(String::from("tls.key"), ByteString(private_key.into_bytes()));

    let ca_secret = Secret {
        metadata: ObjectMeta {
            name: Some(String::from("ca-key-pair")),
            namespace: Some(String::from("cert-manager")),
            ..Default::default()
        },
        data: Some(secret_data),
        ..Default::default()
    };
    // Add ca secret
    let api: Api<Secret> = Api::namespaced(client,"cert-manager");
    api.patch("ca-key-pair", &PatchParams::apply("gentle"),&Patch::Apply(ca_secret)).await?;

    utils::apply_kustomization("./infra/cert-manager")?;

    Ok(())
}