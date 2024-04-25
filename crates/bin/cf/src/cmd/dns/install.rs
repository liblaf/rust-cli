use std::process::Stdio;

use anyhow::Result;
use clap::Args;
use directories::BaseDirs;

#[derive(Args)]
pub struct Cmd {}

impl Cmd {
    pub async fn run(&self) -> Result<()> {
        let dirs = BaseDirs::new().unwrap();
        let dir = dirs.config_dir();
        let contents = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/ddns.service"));
        let path = dir.join("systemd/user/ddns.service");
        tokio::fs::write(path.as_path(), contents).await?;
        tracing::info!("installed: {:?}", path);
        let path = dir.join("systemd/user/ddns.timer");
        let contents = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/ddns.timer"));
        tokio::fs::write(&path, contents).await?;
        tracing::info!("installed: {:?}", path);
        let mut cmd = tokio::process::Command::new("systemctl");
        cmd.args(["--user", "enable", "--now", "ddns.timer"])
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());
        let status = cmd.status().await?;
        anyhow::ensure!(status.success(), "failed to enable ddns.timer");
        tracing::info!("systemctl --user enable --now ddns.timer");
        Ok(())
    }
}
