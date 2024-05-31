use actix_web::{
  dev::{ServiceFactory, ServiceRequest},
  Error,
};
use apistos::{app::OpenApiWrapper, spec::Spec};

use crate::Controller;

pub trait ApiWrapper: Sized {
  type Wrapper;
  fn doc(self, spec: Spec) -> Self::Wrapper;
  fn controller<C: Controller>(self, con: C) -> Self;
}

pub trait ServiceConfigWrapper: Sized {
  fn controller<C: Controller>(&mut self, con: C) -> &mut Self;
}

impl<T> ApiWrapper for actix_web::App<T>
where
  T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>,
{
  type Wrapper = apistos::app::App<T>;

  fn doc(self, spec: Spec) -> Self::Wrapper {
    OpenApiWrapper::document(self, spec)
  }

  fn controller<C: Controller>(self, con: C) -> Self {
    let _ = con;

    self.configure(C::configure)
  }
}

impl ServiceConfigWrapper for actix_web::web::ServiceConfig {
  fn controller<C: Controller>(&mut self, con: C) -> &mut Self {
    let _ = con;

    C::configure(self);

    self
  }
}

impl<T> ApiWrapper for apistos::app::App<T>
where
  T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>,
{
  type Wrapper = ();

  fn doc(self, _: Spec) {}

  fn controller<C: Controller>(self, con: C) -> Self {
    let _ = con;

    self.configure(C::configure_with_docs)
  }
}

impl ServiceConfigWrapper for apistos::web::ServiceConfig<'_> {
  fn controller<C: Controller>(&mut self, con: C) -> &mut Self {
    let _ = con;

    C::configure_with_docs(self);

    self
  }
}
