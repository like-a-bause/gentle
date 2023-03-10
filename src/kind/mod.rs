use super::utils;
use anyhow::{Context, Result};
use kube::{Client, Config};
use tracing::info;

pub async fn create_cluster() -> Result<()> {
    //kind create cluster --config config.yaml
    let create = utils::run_process("kind", [
        "create",
        "cluster",
        "--config",
        "./infra/kind-config.yaml"]);
    match create {
        Ok(_) => {
            info!("Cluster created.");
            Ok(())
        }
        Err(e) => {
            if e.root_cause().to_string().contains(" failed to create cluster: node(s) already exist for a cluster with the name") {
                info!("Cluster already created. Continue...");
                Ok(())
            } else {
                Err(e.context("Could not create cluster."))
            }
        }
    }?;
    Ok(())
}

pub fn delete_cluster() -> Result<()> {
    utils::run_process("kind", [
        "delete",
        "cluster",
        "--name",
        "gentle"
    ])?;
    Ok(())
}

// Sets the kubectx and returns a client for the gentle cluster
pub async fn gentle_context() -> Result<Client> {
    info!("Setting kubectx to kind-gentle");
    utils::run_process("kubectx", ["kind-gentle"])?;
    info!("Generation client for kind-gentle");
    let config = Config::from_kubeconfig(&kube::config::KubeConfigOptions {
        context: Some(String::from("kind-gentle")),
        cluster: None,
        user: None,
    }).await?;
    Client::try_from(config).context("Could not construct kubernetes client")
}