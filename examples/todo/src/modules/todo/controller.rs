use actix_common::controller;
use actix_web::{middleware::Logger, web::Json, Result};

pub struct TodoController;

#[controller("/todo")]
impl TodoController {
  #[get(wrap = Logger::default())]
  pub async fn get_todo() -> Result<Json<()>> {
    Ok(Json(()))
  }
}
