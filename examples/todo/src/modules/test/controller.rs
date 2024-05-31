use super::dto::{CreateTestDto, UpdateTestDto};
use ::actix_common::controller;
use actix_web::web::{Json, Path};

pub struct TestController;

#[controller("/tests")]
impl TestController {
  #[get]
  pub async fn get_tests() -> &'static str {
    super::service::find_many().await
  }

  #[get("/{id}")]
  pub async fn get_test(path: Path<(String,)>) -> String {
    let (id,) = path.into_inner();
    super::service::find_one(id).await
  }

  #[post]
  pub async fn create_test(body: Json<CreateTestDto>) -> &'static str {
    let body = body.into_inner();
    super::service::create(body).await
  }

  #[patch("/{id}")]
  pub async fn update_test(path: Path<(String,)>, body: Json<UpdateTestDto>) -> String {
    let (id,) = path.into_inner();
    let body = body.into_inner();
    super::service::update(id, body).await
  }

  #[delete("/{id}")]
  pub async fn delete_test(path: Path<(String,)>) -> String {
    let (id,) = path.into_inner();
    super::service::delete(id).await
  }
}
