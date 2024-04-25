use clap::Parser;

mod args;
mod cmd;
mod fmt;
mod ip;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cmd = cmd::Cmd::parse();
    cmd.run().await
}
