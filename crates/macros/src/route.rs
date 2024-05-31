use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{
  parse2, spanned::Spanned, Attribute, Error, FnArg, Ident, ImplItem, LitStr, Meta, Path, Signature,
};

use crate::common::Args;

pub struct Route {
  pub name: Ident,
  pub descriptions: Vec<Attribute>,
  pub return_json: bool,
  pub sig: Signature,
  pub method_args: MethodArgs,
  pub attrs: Vec<Attribute>,
}

impl Route {
  pub fn new(item: &mut ImplItem) -> Result<Option<Self>, Error> {
    let ImplItem::Fn(item) = item else {
      return Ok(None);
    };

    let name = item.sig.ident.clone();

    let mut method_args = None;
    let mut attrs = vec![];
    let mut descriptions: Vec<Attribute> = vec![];
    let mut return_json = false;

    for i in 0..item.attrs.len() {
      let attr = item.attrs[i].clone();

      if let Ok(m) = MethodType::from_path(attr.path()) {
        if method_args.is_some() {
          return Err(syn::Error::new(
            attr.span(),
            r#"Unsupported to use more than one method"#,
          ));
        }

        if item
          .sig
          .inputs
          .iter()
          .any(|a| matches!(a, FnArg::Receiver(_)))
        {
          return Err(syn::Error::new(attr.span(), r#"Unsupported self method"#));
        }

        method_args = Some(MethodArgs::new(attr.meta, m)?);

        item.attrs.remove(i);

        continue;
      }

      if attr.path().is_ident("doc") {
        descriptions.push(attr);
        continue;
      }

      if attr.path().is_ident("return_json") {
        return_json = true;
        item.attrs.remove(i);
        continue;
      }

      attrs.push(attr.clone());
    }

    let Some(method_args) = method_args else {
      return Ok(None);
    };

    if matches!(item.sig.output, syn::ReturnType::Default) && !return_json {
      return Err(syn::Error::new_spanned(
        item,
        "Function has no return type. Cannot be used as handler (You can return no type if you add #[return_json] attribute",
      ));
    }

    if item.sig.asyncness.is_none() {
      return Err(syn::Error::new_spanned(item, "Function must be async"));
    }

    Ok(Some(Self {
      method_args,
      descriptions,
      return_json,
      sig: item.sig.clone(),
      attrs,
      name,
    }))
  }
}

impl ToTokens for Route {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let Route {
      method_args:
        MethodArgs {
          path,
          guards,
          method,
          wrappers,
          ..
        },
      name,
      ..
    } = self;

    let method = Ident::new(method.as_lower_str(), Span::call_site());

    let stream = quote! {
      route(
        #path,
        web::#method().to(Self::#name)
        #(.guard(#guards))*
        #(.wrap(#wrappers))*
      )

    };

    tokens.extend(stream);
  }
}

pub struct MethodArgs {
  pub path: syn::LitStr,
  pub resource_name: Option<syn::LitStr>,
  pub guards: Vec<Path>,
  pub wrappers: Vec<syn::Expr>,
  pub method: MethodType,
}

impl MethodArgs {
  fn new(meta: Meta, method: MethodType) -> syn::Result<Self> {
    let span = meta.span();
    let mut path = None;
    let mut resource_name = None;
    let mut guards = Vec::new();
    let mut wrappers = Vec::new();

    match meta {
      Meta::Path(_) => {}
      Meta::NameValue(v) => return Err(Error::new(v.span(), "Unsupported meta")),
      Meta::List(list) => {
        let args = parse2::<Args>(list.tokens)?;

        path = args.path;

        for nv in args.options {
          if nv.path.is_ident("name") {
            if let syn::Expr::Lit(syn::ExprLit {
              lit: syn::Lit::Str(lit),
              ..
            }) = nv.value
            {
              resource_name = Some(lit);
            } else {
              return Err(syn::Error::new_spanned(
                nv.value,
                "Attribute name expects literal string",
              ));
            }
          } else if nv.path.is_ident("guard") {
            if let syn::Expr::Path(syn::ExprPath { path, .. }) = nv.value {
              guards.push(path);
            } else {
              return Err(syn::Error::new_spanned(
                nv.value,
                "Attribute guard expects literal string",
              ));
            }
          } else if nv.path.is_ident("wrap") {
            wrappers.push(nv.value);
          } else {
            return Err(syn::Error::new_spanned(
              nv.path,
              "Unknown attribute key is specified; allowed: guard and wrap",
            ));
          }
        }
      }
    }
    Ok(Self {
      path: path.unwrap_or_else(|| LitStr::new("", span)),
      resource_name,
      guards,
      wrappers,
      method,
    })
  }
}

// actix-web-codegen
macro_rules! standard_method_type {
  (
      $($variant:ident, $upper:ident, $lower:ident,)+
  ) => {
      #[derive(Debug, Clone, PartialEq, Eq, Hash)]
      pub enum MethodType {
          $(
              $variant,
          )+
      }

      impl MethodType {
          // fn as_str(&self) -> &'static str {
          //     match self {
          //         $(Self::$variant => stringify!($variant),)+
          //     }
          // }

          fn as_lower_str(&self) -> &'static str {
            match self {
                $(Self::$variant => stringify!($lower),)+
            }
          }

          // fn parse(method: &str) -> Result<Self, String> {
          //     match method {
          //         $(stringify!($upper) => Ok(Self::$variant),)+
          //         _ => Err(format!("HTTP method must be uppercase: `{}`", method)),
          //     }
          // }

          fn from_path(method: &Path) -> Result<Self, ()> {
              match () {
                  $(_ if method.is_ident(stringify!($lower)) => Ok(Self::$variant),)+
                  _ => Err(()),
              }
          }
      }
  };
}

standard_method_type! {
  Get,       GET,     get,
  Post,      POST,    post,
  Put,       PUT,     put,
  Delete,    DELETE,  delete,
  Head,      HEAD,    head,
  Connect,   CONNECT, connect,
  Options,   OPTIONS, options,
  Trace,     TRACE,   trace,
  Patch,     PATCH,   patch,
}
