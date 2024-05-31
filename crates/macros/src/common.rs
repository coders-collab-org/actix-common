use actix_router::ResourceDef;
use proc_macro2::Span;
use syn::{
  parse::{Parse, ParseStream},
  punctuated::Punctuated,
  Error, LitStr, MetaNameValue, Token,
};

pub struct Args {
  pub path: Option<LitStr>,
  pub options: Punctuated<MetaNameValue, Token![,]>,
}

impl Parse for Args {
  fn parse(input: ParseStream) -> Result<Self, Error> {
    let path = input
      .parse::<LitStr>()
      .map_err(|mut err| {
        err.combine(Error::new(
          err.span(),
          format!(r#"controller must be start with path, #[macro("<path>"),] found {input}"#),
        ));

        err
      })
      .ok();

    // verify that path pattern is valid
    let _ = path.as_ref().map(|p| ResourceDef::new(p.value()));

    if path.is_some() {
      // if there's no comma, assume that no options are provided
      if !input.peek(Token![,]) {
        return Ok(Self {
          path,
          options: Punctuated::new(),
        });
      }

      // advance past comma separator
      input.parse::<Token![,]>()?;
    }

    // if next char is a literal, assume that it is a string and show multi-path error
    if input.cursor().literal().is_some() {
      return Err(syn::Error::new(
        Span::call_site(),
        r#"Multiple paths specified! There should be only one."#,
      ));
    }

    // zero or more options: name = "foo"
    let options = input.parse_terminated(syn::MetaNameValue::parse, Token![,])?;

    Ok(Self { path, options })
  }
}
