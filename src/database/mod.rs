use crate::utils;

pub async fn install_postgresql() -> anyhow::Result<()> {
    utils::apply_kustomization("infra/database/postgresql")?;
    Ok(())
}