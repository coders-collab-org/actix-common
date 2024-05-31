mod modules;

use std::net::Ipv4Addr;

use actix_common::ApiWrapper;
use actix_web::{App, HttpServer};
use modules::todo;

use todo::TodoController;

use std::error::Error;

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
  HttpServer::new(move || App::new().controller(TodoController))
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
    .run()
    .await
}
