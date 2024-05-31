use super::dto::{CreateTestDto, UpdateTestDto};

pub async fn find_many() -> &'static str {
  "find many"
}

pub async fn find_one(id: String) -> String {
  format!("find {id}")
}

pub async fn create(_body: CreateTestDto) -> &'static str {
  "create"
}

pub async fn update(id: String, _body: UpdateTestDto) -> String {
  format!("create {id}")
}

pub async fn delete(id: String) -> String {
  format!("delete {id}")
}
