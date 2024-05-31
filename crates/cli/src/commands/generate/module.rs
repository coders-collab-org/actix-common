use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::{bail, Context, Result};
use inflector::Inflector;
use inquire::Confirm;

use crate::File;

use super::controller::Controller;

pub struct Module {
  pub file: File,
  pub files: HashMap<String, File>,
  pub path: PathBuf,
  pub name: String,
  pub exist_dir: bool,
}

impl Module {
  pub fn new(modules_path: PathBuf, module_name: String) -> Result<Self> {
    let path = modules_path.join(&module_name);

    let exist_dir = path.exists();

    if exist_dir && !path.is_dir() {
      bail!("{path:?} must be a directory");
    }

    if !exist_dir {
      fs::create_dir(&path)?;
    }

    let file = File::new(path.join("mod.rs"))
      .with_context(|| format!("Failed to get module file {:?}", &path))?;

    Ok(Self {
      file,
      files: HashMap::default(),
      path,
      name: module_name,
      exist_dir,
    })
  }

  pub fn generate(&mut self) -> Result<()> {
    if self.exist_dir {
      bail!(
        "{:?} already exists, please remove it before use this command",
        self.path
      );
    }

    let crud = ask_for_create_crud_code();

    self.create_controller(crud)?;
    self.create_dto(crud)?;
    self.create_service(crud)?;
    self.create_mod()?;

    Ok(())
  }

  pub fn create_rust_file(&mut self, name: &str) -> Result<&mut File> {
    if let None = self.files.get_mut(name) {
      let path = self.path.join(format!("{name}.rs"));

      if path.exists() {
        bail!("{path:?} already exists");
      }

      self.files.insert(name.to_owned(), File::new(path)?);
    };

    Ok(unsafe { self.files.get_mut(name).unwrap_unchecked() })
  }

  pub fn rollback(self) -> Result<()> {
    if !self.exist_dir {
      fs::remove_dir_all(&self.path).context("Failed to delete module directory")?;
      return Ok(());
    }

    self.file.rollback()?;

    for (_, file) in self.files {
      let path = file.path().to_path_buf();

      file
        .rollback()
        .with_context(|| format!("Failed to rollback {:?} file", path))?;
    }

    Ok(())
  }

  pub fn log(&self) {
    self.file.log();

    for file in self.files.values() {
      file.log();
    }
  }

  fn create_mod(&mut self) -> Result<()> {
    let content = "\
    pub mod controller;\n\
    pub mod dto;\n\
    pub mod service;\n\
    ";

    self.file.write(content.as_bytes())
  }

  fn create_service(&mut self, crud: bool) -> Result<()> {
    if crud {
      let create_name = format!("Create{}Dto", self.name.to_pascal_case());
      let update_name = format!("Update{}Dto", self.name.to_pascal_case());

      let content = format!(
        "\
        use super::dto::{{{create_name}, {update_name}}};\n\n\
        pub async fn find_many() -> &'static str {{\n\
           \u{0020}\u{0020}\"find many\"\n\
        }}\n\n\
        pub async fn find_one(id: String) -> String {{\n\
           \u{0020}\u{0020}format!(\"find {{id}}\")\n\
        }}\n\n\
        pub async fn create(_body: {create_name}) -> &'static str {{\n\
           \u{0020}\u{0020}\"create\"\n\
        }}\n\n\
        pub async fn update(id: String, _body: {update_name}) -> String {{\n\
           \u{0020}\u{0020}format!(\"create {{id}}\")\n\
        }}\n\n\
        pub async fn delete(id: String) -> String {{\n\
           \u{0020}\u{0020}format!(\"delete {{id}}\")\n\
        }}\n\
        "
      );
      self
        .create_rust_file("service")?
        .write(content.as_bytes())?;

      return Ok(());
    }

    self.create_rust_file("service")?;

    Ok(())
  }
  fn create_dto(&mut self, crud: bool) -> Result<()> {
    if crud {
      let create_name = format!("Create{}Dto", self.name.to_pascal_case());
      let update_name = format!("Update{}Dto", self.name.to_pascal_case());

      let content = format!(
        "\
        use serde::Deserialize;\n\n\
        #[derive(Debug, Deserialize)]\n\
        pub struct {create_name} {{}}\n\n\
        #[derive(Debug, Deserialize)]\n\
        pub struct {update_name} {{}}\n\
        "
      );
      self.create_rust_file("dto")?.write(content.as_bytes())?;

      return Ok(());
    }

    self.create_rust_file("dto")?;

    Ok(())
  }

  fn create_controller(&mut self, crud: bool) -> Result<()> {
    let mut import = "".to_string();

    let methods = crud.then(|| {
      let name = self.name.to_snake_case();
      let create_name = format!("Create{}Dto", self.name.to_pascal_case());
      let update_name = format!("Update{}Dto", self.name.to_pascal_case());

      let gets = format!(
        "\
        #[get]\n\
        pub async fn get_{name}() -> &'static str {{\n\
         \u{0020}\u{0020}super::service::find_many().await\n\
        }}\
        ",
        name = name.to_plural()
      );

      let get = format!(
        "\
        #[get(\"/{{id}}\")]\n\
        pub async fn get_{name}(path: Path<(String,)>) -> String {{\n\
           \u{0020}\u{0020}let (id,) = path.into_inner();\n\
           \u{0020}\u{0020}super::service::find_one(id).await\n\
        }}\
        ",
      );

      let post = format!(
        "\
        #[post]\n\
        pub async fn create_{name}(body: Json<{create_name}>) -> &'static str {{\n\
           \u{0020}\u{0020}let body = body.into_inner();\n\
           \u{0020}\u{0020}super::service::create(body).await\n\
        }}\
        ",
      );

      let patch = format!(
        "\
        #[patch(\"/{{id}}\")]\n\
        pub async fn update_{name}(path: Path<(String,)>,body: Json<{update_name}>) -> String {{\n\
           \u{0020}\u{0020}let (id,) = path.into_inner();\n\
           \u{0020}\u{0020}let body = body.into_inner();\n\
           \u{0020}\u{0020}super::service::update(id, body).await\n\
        }}\
        ",
      );

      let delete = format!(
        "\
        #[delete(\"/{{id}}\")]\n\
        pub async fn delete_{name}(path: Path<(String,)>) -> String {{\n\
           \u{0020}\u{0020}let (id,) = path.into_inner();\n\
           \u{0020}\u{0020}super::service::delete(id).await\n\
        }}\
        ",
      );

      import += "use actix_web::web::{Path, Json};\n";
      import += &format!("use super::dto::{{{create_name}, {update_name}}};\n");

      format!("{gets}\n\n{get}\n\n{post}\n\n{patch}\n\n{delete}")
    });

    let con = Controller {
      file_name: "controller".to_string(),
      imports: (!import.is_empty()).then(|| import),
      methods,
      link: false,
    };

    con.generate(self)?;

    Ok(())
  }
}

pub fn ask_for_create_crud_code() -> bool {
  Confirm::new("Do you want to create CRUD code? (yes/no)")
    .with_default(true)
    .prompt()
    .unwrap()
}
