use std::{
  fmt::Display,
  fs,
  io::Result,
  path::{Path, PathBuf},
};

use anyhow::Context;

pub enum ChangeType {
  Update(Vec<u8>),
  Create,
  None,
}

pub struct File {
  backup: ChangeType,
  path: PathBuf,
}

impl File {
  pub fn new(path: PathBuf) -> Result<Self> {
    let backup = if path.exists() {
      ChangeType::None
    } else {
      fs::File::create(&path)?;
      ChangeType::Create
    };

    Ok(Self { backup, path })
  }

  pub fn read_to_string(&mut self) -> Result<String> {
    fs::read_to_string(&self.path)
  }

  pub fn read_all(&mut self) -> Result<Vec<u8>> {
    fs::read(&self.path)
  }

  pub fn write(&mut self, buf: &[u8]) -> anyhow::Result<()> {
    self.store_backup()?;

    fs::write(&self.path, buf).with_context(|| format!("Failed to write on {:?}", self.path))
  }

  pub fn write_to_beginning(&mut self, buf: &[u8]) -> Result<()> {
    self.store_backup()?;

    let mut content = self.read_all()?;

    content.splice(0..0, buf.iter().cloned());

    fs::write(&self.path, content)
  }

  pub fn rollback(self) -> Result<()> {
    match self.backup {
      ChangeType::Update(ref data) => fs::write(&self.path, data),
      ChangeType::Create => fs::remove_file(self.path),
      ChangeType::None => Ok(()),
    }
  }

  fn store_backup(&mut self) -> Result<()> {
    if let ChangeType::None = self.backup {
      let old = self.read_all()?;
      self.backup = ChangeType::Update(old);
    }

    Ok(())
  }

  pub fn path(&self) -> &Path {
    &self.path
  }

  pub fn log(&self) {
    if matches!(self.backup, ChangeType::None) {
      return;
    }

    println!("{} {:?}", self.backup, self.path.as_os_str());
  }
}

impl Display for ChangeType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ChangeType::Update(_) => write!(f, "UPDATE"),
      ChangeType::Create => write!(f, "CREATE"),
      ChangeType::None => write!(f, ""),
    }
  }
}
