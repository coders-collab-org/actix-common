use clap::{Parser, Subcommand};
mod generate;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub commands: Command,
}

#[derive(Subcommand)]
pub enum Command {
  #[command(alias = "g")]
  Generate(generate::Args),
}
