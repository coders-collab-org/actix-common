pub trait Controller {
  fn configure(cfg: &mut actix_web::web::ServiceConfig);
  fn configure_with_docs(cfg: &mut apistos::web::ServiceConfig);
}
