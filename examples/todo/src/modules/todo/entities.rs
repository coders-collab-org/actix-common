use serde::Deserialize;

#[derive(Deserialize)]
pub struct TodoEntity {
  pub name: String,
  pub value: String,
}
