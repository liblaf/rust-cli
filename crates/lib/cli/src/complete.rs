use anyhow::Result;
use clap::builder::PossibleValue;
use clap::{Args, Command, ValueEnum};
use clap_complete::Shell;

#[derive(Args)]
pub struct Cmd {
    shell: Generator,
}

impl Cmd {
    pub fn run(&self, mut cmd: Command) -> Result<()> {
        match self.shell {
            Generator::Markdown => println!("{}", clap_markdown::help_markdown_command(&cmd)),
            Generator::Shell(shell) => {
                let name = cmd.get_name().to_string();
                clap_complete::generate(shell, &mut cmd, name, &mut std::io::stdout())
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
enum Generator {
    Markdown,
    Shell(Shell),
}

impl ValueEnum for Generator {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Markdown,
            Self::Shell(Shell::Bash),
            Self::Shell(Shell::Elvish),
            Self::Shell(Shell::Fish),
            Self::Shell(Shell::PowerShell),
            Self::Shell(Shell::Zsh),
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            Self::Markdown => Some(PossibleValue::new("markdown")),
            Self::Shell(shell) => shell.to_possible_value(),
        }
    }
}
