use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
  parse::{Parse, ParseStream},
  Ident, Token,
};
use thiserror::Error;
use to_syn_error::ToSynError;

#[derive(Error, Debug, ToSynError)]
enum ArgsError {
  #[error("The macro must pass args.")]
  Empty,
  #[error("The macro needs two args.")]
  Single,
  #[error("The macro needs two args only.")]
  TooMany,
  #[error("The macro needs variables as args.")]
  NotIdent,
  #[error("The macro must separate with comma.")]
  NotComma,
  #[error("The second arg of the macro must be enum variant.")]
  NotVariant,
}

pub struct Args {
  pub state: Ident,
  pub next: TokenStream2,
}

impl Parse for Args {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    if input.is_empty() {
      return Err(ArgsError::Empty.to_syn_error(input.span()));
    }
    let state: Ident = match input.parse() {
      Ok(i) => i,
      Err(e) => {
        return Err(ArgsError::NotIdent.to_syn_error(e.span()));
      }
    };
    if input.is_empty() {
      return Err(ArgsError::Single.to_syn_error(input.span()));
    }
    let _comma: Token![,] = match input.parse() {
      Ok(i) => i,
      Err(e) => {
        return Err(ArgsError::NotComma.to_syn_error(e.span()));
      }
    };
    if input.is_empty() {
      return Err(ArgsError::Single.to_syn_error(input.span()));
    }
    let next: Ident = match input.parse() {
      Ok(i) => i,
      Err(e) => {
        return Err(ArgsError::NotIdent.to_syn_error(e.span()));
      }
    };
    let _colon: Token![:] = match input.parse() {
      Ok(i) => i,
      Err(e) => {
        return Err(ArgsError::NotVariant.to_syn_error(e.span()));
      }
    };
    let _colon: Token![:] = match input.parse() {
      Ok(i) => i,
      Err(e) => {
        return Err(ArgsError::NotVariant.to_syn_error(e.span()));
      }
    };
    let variant: Ident = match input.parse() {
      Ok(i) => i,
      Err(e) => {
        return Err(ArgsError::NotIdent.to_syn_error(e.span()));
      }
    };
    let next = quote! {#next::#variant};
    if !input.is_empty() {
      return Err(ArgsError::TooMany.to_syn_error(input.span()));
    }
    Ok(Self { next, state })
  }
}

#[cfg(test)]
mod unittest {
  use super::*;
  use assert_parse::*;
  use quote::quote;
  use rstest::*;

  register_assert!(Args, ArgsError);

  #[rstest]
  fn empty(assert: Assert) {
    let args = quote! {};
    assert.error(args, ArgsError::Empty);
  }

  #[rstest]
  fn not_ident_first(assert: Assert) {
    let args = quote! {1};
    assert.error(args, ArgsError::NotIdent);
  }

  #[rstest]
  fn single_first(assert: Assert) {
    let args = quote! {Mock};
    assert.error(args, ArgsError::Single);
  }

  #[rstest]
  fn not_comma(assert: Assert) {
    let args = quote! {Mock.};
    assert.error(args, ArgsError::NotComma);
  }

  #[rstest]
  fn single_second(assert: Assert) {
    let args = quote! {Mock,};
    assert.error(args, ArgsError::Single);
  }

  #[rstest]
  fn not_ident_second(assert: Assert) {
    let args = quote! {Mock,1};
    assert.error(args, ArgsError::NotIdent);
  }

  #[rstest]
  fn not_variant(assert: Assert) {
    let args = quote! {Mock,MockNext};
    assert.error(args, ArgsError::NotVariant);
  }

  #[rstest]
  fn too_many(assert: Assert) {
    let args = quote! {Mock,MockNext::Variant,Something};
    assert.error(args, ArgsError::TooMany);
  }

  #[rstest]
  fn ok(assert: Assert) {
    let args = quote! {Mock,MockNext::Variant};
    assert.ok(args, |args| {
      assert_eq!(&args.state.to_string(), "Mock");
      assert_eq!(&args.next.to_string(), "MockNext :: Variant");
    });
  }
}
