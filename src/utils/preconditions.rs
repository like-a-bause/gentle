use anyhow::{Context, Result};
use tracing::*;
use which::which;

const NEEDED_CLI_PROGRAMS: &'static [&'static str] = &["kind", "kubectl", "helm", "kustomize", "docker"];

pub fn check() -> Result<()> {
    info!("Checking preconditions");
    for p in NEEDED_CLI_PROGRAMS {
        which(p).with_context(|| format!("Could not locate {0}. Install it with `brew install {0}`", p))?;
    }

    info!("Preconditions met you can bootstrap your cluster.");
    Ok(())
}