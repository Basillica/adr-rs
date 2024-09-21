use anyhow::Result;
use crate::utils::config::AppConfig;

pub(crate) fn main(cfg: &mut AppConfig) -> Result<()> {
    cfg.list_projects();
    Ok(())
}