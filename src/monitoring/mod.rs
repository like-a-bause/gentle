use kube::{Api, Client};
use anyhow::Result;
use k8s_openapi::api::core::v1::Namespace;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::PostParams;
use tracing::*;
use crate::utils::run_process;

pub async fn install_monitoring_stack(client: Client) -> Result<()> {
    // 1. create monitoring namespaces via kube-rs
    let ns: Api<Namespace> = Api::all(client);

    let namespace = Namespace {
        metadata: ObjectMeta {
            name: Some(String::from("monitoring")),
            ..Default::default()
        },
        spec: None,
        status: None,
    };

    let pp = PostParams::default();
    match ns.create(&pp, &namespace).await {
        Ok(ns) => {
            info!("Created Namespace {:?}", ns.metadata.name)
        }
        Err(kube::Error::Api(ae)) => {
            if ae.code == 409 {
                info!("Namespace already created.")
            } else {
                return Err(ae.into());
            }
        }
        Err(e) => { return Err(e.into()); }
    }
    // 2. install kube-prometheus via helm

    info!("Adding repositories");
    run_process("helm", ["repo", "add", "prometheus-community", "https://prometheus-community.github.io/helm-charts"])?;
    run_process("helm", ["repo", "add", "grafana", "https://grafana.github.io/helm-charts"])?;
    run_process("helm", ["repo", "update"])?;

    info!("Installing Prometheus ....");
    run_process("helm",
                [
                    "-f",
                    "./infra/monitoring/kube-prometheus/values.yaml",
                    "-n",
                    "monitoring",
                    "upgrade",
                    "--install",
                    "prometheus",
                    "prometheus-community/kube-prometheus-stack",
                ])?;

    info!("Installing Loki ....");
    run_process("helm",
                [
                    "-f",
                    "./infra/monitoring/loki/values.yaml",
                    "-n",
                    "monitoring",
                    "upgrade",
                    "--install",
                    "loki",
                    "grafana/loki-stack"
                ],
    )?;
    Ok(())
}