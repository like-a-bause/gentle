use anyhow::Result;
use kube::{Api, Client, ResourceExt};
use k8s_openapi::api::core::v1::{ConfigMap, Pod};
use std::fs;
use kube::api::{DeleteParams, ListParams, PostParams};
use tracing::info;
use crate::{utils};

pub async fn patch_coredns(client: Client) -> Result<()> {
    info!("Patching CoreDns to resolve .test tld to host.docker.internal for in cluster usage");
    let cmf = fs::read_to_string("infra/networking/coredns/coredns-config.yaml")?;

    let cm: ConfigMap = serde_yaml::from_str(&cmf)?;
    let cm_api: Api<ConfigMap> = Api::namespaced(client.clone(), "kube-system");
    cm_api.replace("coredns", &PostParams::default(), &cm).await?;
    let pod_api: Api<Pod> = Api::namespaced(client.clone(), "kube-system");

    let lp = ListParams::default().labels(&format!("k8s-app={}", "kube-dns"));
    for p in pod_api.list(&lp).await? {
        info!("Deleting Pod: {}", p.name_any());
        pod_api.delete(p.name_any().as_str(), &DeleteParams::default()).await?;
    }
    Ok(())
}

pub async fn install_ingress_nginx() -> Result<()> {
    info!("Installing ingress: nginx");
    utils::run_process("helm",
                       [
                           "upgrade",
                           "--install",
                           "ingress-nginx",
                           "ingress-nginx",
                           "--repo",
                           "https://kubernetes.github.io/ingress-nginx",
                           "--namespace",
                           "ingress-nginx",
                           "--create-namespace",
                           "-f",
                           "infra/networking/ingress/nginx/values.yaml"
                       ],
    )?;
    Ok(())
}

pub async fn install_dnsmasq() -> Result<()> {
    info!("Installing Dnsmasq");
    utils::apply_kustomization("infra/networking/dnsmasq")?;
    Ok(())
}