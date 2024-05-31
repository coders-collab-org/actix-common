use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse2, Attribute, Error, ImplItem, ItemImpl, Type};

use crate::{common::Args, route::Route};

struct Controller {
  args: Args,
  attrs: Vec<Attribute>,
  type_: Type,
  routes: Vec<Route>,
  items: Vec<ImplItem>,
}

impl Controller {
  fn new(
    ItemImpl {
      attrs,
      mut items,
      self_ty,
      ..
    }: ItemImpl,
    args: Args,
  ) -> Result<Self, Error> {
    let routes = items
      .iter_mut()
      .map(Route::new)
      .collect::<Result<Vec<_>, Error>>()?
      .into_iter()
      .flatten()
      .collect();

    Ok(Self {
      args,
      attrs,
      routes,
      items,
      type_: *self_ty,
    })
  }
}

impl ToTokens for Controller {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let Controller {
      args,
      attrs,
      type_,
      routes,
      items,
    } = self;
    let path = &args.path;

    let stream = quote! {
      impl #type_ {
        #(#items)*
      }

      impl ::actix_common::Controller for #type_ {
        fn configure(cfg: &mut ::actix_web::web::ServiceConfig) {
          use ::actix_web::web::{self, scope, resource};
          cfg.service(scope(#path)#(.#routes)*);
        }
        fn configure_with_docs(_cfg: &mut ::apistos::web::ServiceConfig) {}
      }
    };

    tokens.extend(stream);
  }
}

pub fn expand(input: TokenStream, args: TokenStream) -> Result<TokenStream, TokenStream> {
  let item = parse2::<ItemImpl>(input.clone())
    .and_then(|m| {
      if m.trait_.is_some() {
        return Err(syn::Error::new(
          Span::call_site(),
          r#"Unsupported impl Trait"#,
        ));
      }

      Ok(m)
    })
    .map_err(|e| input_and_compile_error(input.clone(), e))?;
  let args: Args = parse2(args).map_err(|e| input_and_compile_error(input.clone(), e))?;

  Ok(
    Controller::new(item, args)
      .map_err(Error::into_compile_error)?
      .into_token_stream(),
  )
}

/// Converts the error to a token stream and appends it to the original input.
///
/// Returning the original input in addition to the error is good for IDEs which can gracefully
/// recover and show more precise errors within the macro body.
///
/// See <https://github.com/rust-analyzer/rust-analyzer/issues/10468> for more info.
fn input_and_compile_error(mut item: TokenStream, err: syn::Error) -> TokenStream {
  item.extend(err.to_compile_error());
  item
}
