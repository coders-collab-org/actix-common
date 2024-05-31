mod controller;
mod module;

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::Subcommand;

use inquire::{validator::Validation, Text};

use controller::Controller;
use module::Module;

#[derive(clap::Args)]
pub struct Args {
  #[arg(short, long, value_name = "MODULES_PATH")]
  path: Option<PathBuf>,

  #[arg(help = "Module name")]
  name: Option<String>,

  #[command(subcommand)]
  command: GenerateCommand,
}

#[derive(Subcommand)]
pub enum GenerateCommand {
  #[command(alias = "cont")]
  Controller,
  #[command(alias = "res")]
  Resources,
}

impl Args {
  pub fn action(self) -> Result<()> {
    let path = self.path.unwrap_or_else(|| ask_for_modules_path());
    let name = self.name.unwrap_or_else(|| ask_for_name());

    let mut module = Module::new(path, name)?;

    let result = match self.command {
      GenerateCommand::Controller => {
        let file_name = ask_for_file_name(&module.path);

        let controller = Controller {
          file_name,
          link: true,
          ..Default::default()
        };

        controller.generate(&mut module)
      }

      GenerateCommand::Resources => module.generate(),
    };

    if let Err(err) = result {
      module.rollback().context("Failed to rollback")?;
      return Err(err);
    }

    module.log();

    Ok(())
  }
}

fn ask_for_modules_path() -> PathBuf {
  let relative_path = Text::new("Enter the path to the modules directory:")
    .with_validator(|path: &str| {
      let path = Path::new(path.trim());
      if path.is_dir() {
        Ok(Validation::Valid)
      } else {
        Ok(Validation::Invalid(
          "The provided path is not a directory.".into(),
        ))
      }
    })
    .prompt()
    .unwrap();

  let mut path = PathBuf::new();

  path.push(relative_path.trim());

  path
}

fn ask_for_name() -> String {
  let name = Text::new("Enter module name:")
    .with_validator(|name: &str| {
      if !name.trim().is_empty() {
        Ok(Validation::Valid)
      } else {
        Ok(Validation::Invalid("Name cannot be empty.".into()))
      }
    })
    .prompt()
    .unwrap();

  name
}

fn ask_for_file_name(path: &Path) -> String {
  let path = path.to_path_buf();

  let name = Text::new("Enter controller file name:")
    .with_default("controller")
    .with_validator(move |name: &str| {
      let path = path.join(format!("{name}.rs"));

      if path.exists() {
        Ok(Validation::Invalid("file already exists.".into()))
      } else {
        Ok(Validation::Valid)
      }
    })
    .prompt()
    .unwrap();

  name
}
