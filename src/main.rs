mod kind;
mod certs;
mod utils;
mod monitoring;
mod networking;
mod database;

use tracing::*;
use anyhow::Result;

use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct App {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// check for preconditions
    Check,
    /// bootstrap a cluster
    Bootstrap,
    /// delete a cluster
    Delete,
    /// generate Certificate
    Certificate,
    /// testing out new commands
    Test,
}

impl App {}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let app: App = Parser::parse();

    match &app.command {
        Some(Commands::Check) => {
            utils::preconditions::check()
        }
        Some(Commands::Bootstrap) => {
            info!("Bootstrapping the cluster...");
            kind::create_cluster().await?;
            let client = kind::gentle_context().await?;
            certs::install_cert_manager().await?;
            certs::generate_certificate_authority(client.clone()).await?;
            networking::patch_coredns(client.clone()).await?;
            networking::install_ingress_nginx().await?;
            networking::install_dnsmasq().await?;
            database::install_postgresql().await?;
            monitoring::install_monitoring_stack(client.clone()).await
        }
        Some(Commands::Delete) => {
            info!("Deleting the cluster...");
            kind::delete_cluster()
        }
        Some(Commands::Certificate) => {
            info!("Generating Certificate Authority");
            let client = kind::gentle_context().await?;
            certs::generate_certificate_authority(client).await
        }
        Some(Commands::Test) => {
            info!("test...");
            let _ = kind::gentle_context().await?;
            networking::install_dnsmasq().await
        }
        None => { Ok(()) }
    }
}