mod ip;
mod list;
mod now;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use cli::log::{DefaultLevel, LogInit};
use concolor_clap::ColorChoice;

#[derive(Parser)]
#[clap(color = concolor_clap::color_choice())]
pub struct Cmd {
    #[command(subcommand)]
    cmd: SubCmd,
    #[command(flatten)]
    color: concolor_clap::Color,
    #[command(flatten)]
    verbose: Verbosity<DefaultLevel>,
}

#[derive(Subcommand)]
enum SubCmd {
    Ip(ip::Cmd),
    List(list::Cmd),
    Now(now::Cmd),
}

impl Cmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        match self.color.color {
            ColorChoice::Auto => colored::control::unset_override(),
            ColorChoice::Always => {
                colored::control::set_override(true);
                console::set_colors_enabled(true);
            }
            ColorChoice::Never => {
                colored::control::set_override(false);
                console::set_colors_enabled(false);
            }
        }
        self.verbose.init();
        match &self.cmd {
            SubCmd::Ip(cmd) => cmd.run().await,
            SubCmd::List(cmd) => cmd.run().await,
            SubCmd::Now(cmd) => cmd.run().await,
        }
    }
}
