mod dns;

use clap::{CommandFactory, Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use cli::{
    color::ColorInit,
    log::{DefaultLevel, LogInit},
};

#[derive(Parser)]
pub struct Cmd {
    #[command(subcommand)]
    cmd: SubCmd,
    #[arg(
        long,
        default_value = "https://api.cloudflare.com/client/v4",
        global(true)
    )]
    api_url: String,
    #[arg(long, global(true))]
    token: Option<String>,
    #[command(flatten)]
    color: concolor_clap::Color,
    #[command(flatten)]
    verbose: Verbosity<DefaultLevel>,
}

#[derive(Subcommand)]
enum SubCmd {
    Complete(cli::complete::Cmd),
    Dns(dns::Cmd),
}

impl Cmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        self.color.init();
        self.verbose.init();
        match &self.cmd {
            SubCmd::Complete(cmd) => cmd.run(Cmd::command()),
            SubCmd::Dns(cmd) => cmd.run().await,
        }
    }
}
