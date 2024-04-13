pub mod get;
pub mod types;

use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::process::Stdio;

use anyhow::Result;
use once_cell::sync::Lazy;
use tokio::process::Command;

async fn bw<I, S>(args: I) -> Result<Vec<u8>>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let cmd = &mut Command::new("bw");
    let cmd = cmd.arg("--nointeraction").args(args);
    let hash = format!("{:?}", cmd);
    static CACHE: Lazy<HashMap<String, Vec<u8>>> = Lazy::new(HashMap::new);
    if let Some(output) = CACHE.get(hash.as_str()) {
        tracing::debug!("Cache Hit: {:?}", hash);
        return Ok(output.to_vec());
    }
    tracing::debug!("Cache Miss: {:?}", hash);
    let cmd = cmd
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit());
    tracing::debug!("{:?}", cmd);
    let child = cmd.spawn()?;
    let output = child.wait_with_output().await?;
    anyhow::ensure!(output.status.success());
    Ok(output.stdout)
}

async fn get<I, S>(args: I) -> Result<Vec<u8>>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    bw(["get".parse::<OsString>()?]
        .into_iter()
        .chain(args.into_iter().map(|s| s.as_ref().to_os_string())))
    .await
}
