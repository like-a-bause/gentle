pub mod preconditions;

use std::process::{Command, Stdio};
use anyhow::{anyhow, Context, Result};
use std::ffi::OsStr;

// run a process and handle the error
pub fn run_process<I, S>(command: &str, args: I) -> Result<String>
    where
        I: IntoIterator<Item=S>,
        S: AsRef<OsStr>,
{
    let output = Command::new(command)
        .args(args)
        .output()?;


    if output.status.success() {
        let o = String::from_utf8(output.stdout).unwrap_or(String::from("Output from Command"));
        Ok(o)
    } else {
        let err = String::from_utf8(output.stderr).unwrap_or(String::from("Error"));
        Err(anyhow!("Command failed: {}", err))
    }
}

pub fn apply_kustomization(path: &str) -> Result<()> {
    let kustomize = Command::new("kustomize")
        .args([
            "build",
            path
        ])
        .stdout(Stdio::piped())
        .spawn().context("Could not kutomize")?;

    let kustomize_output = kustomize.stdout.ok_or(anyhow!("Could not extract kustomization output"))?;


    Command::new("kubectl")
        .args([
            "apply",
            "-f",
            "-"
        ])
        .stdin(Stdio::from(kustomize_output))
        .output().context("Could not apply with kubectl")?;
    Ok(())
}