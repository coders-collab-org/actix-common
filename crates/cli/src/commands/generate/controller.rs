use anyhow::Result;
use inflector::Inflector;

use super::module::Module;

#[derive(Default)]
pub struct Controller {
  pub imports: Option<String>,
  pub methods: Option<String>,
  pub link: bool,
  pub file_name: String,
}

impl Controller {
  pub fn generate(self, module: &mut Module) -> Result<()> {
    let Controller {
      imports,
      link,
      methods,
      file_name,
    } = self;

    let name = format!("{}Controller", module.name.to_pascal_case());
    let path = format!("\"/{}\"", module.name.to_plural().to_kebab_case());

    const IMPORT_CONTROLLER: &str = "use ::actix_common::controller;";

    let imports = imports.map_or_else(
      || IMPORT_CONTROLLER.to_string(),
      |i| format!("{IMPORT_CONTROLLER}\n{i}"),
    );
    let struct_ = format!("pub struct {name};");

    let controller_attr = format!("#[controller({path})]");

    let impl_ = format!(
      "impl {name} {{{}}}",
      methods.map_or_else(|| "".to_owned(), |m| format!("\n{m}\n"))
    );

    let out = format!("{imports}\n\n{struct_}\n\n{controller_attr}\n{impl_}\n");

    module.create_rust_file(&file_name)?.write(out.as_bytes())?;

    if link {
      module
        .file
        .write_to_beginning(format!("mod {file_name};\n").as_bytes())?;
    }

    Ok(())
  }
}
