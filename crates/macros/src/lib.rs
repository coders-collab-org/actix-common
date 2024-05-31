mod common;
mod controller;
mod route;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn controller(args: TokenStream, input: TokenStream) -> TokenStream {
  controller::expand(input.into(), args.into())
    .unwrap_or_else(|e| e)
    .into()
}
