use actix_cli::commands::{Cli, Command};
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
  let cli = Cli::parse();

  match cli.commands {
    Command::Generate(args) => args.action(),
  }
}
