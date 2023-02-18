extern crate proc_macro;
mod args;
use args::Args;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn set_with_grab(args: TokenStream) -> TokenStream {
  let args: Args = match syn::parse(args) {
    Ok(a) => a,
    Err(e) => {
      return e.to_compile_error().into();
    }
  };
  let Args { state, next } = args;
  quote! {
    match #state .set(#next ){
      Ok(_)=>{},
      Err(e)=>{
        bevy::ecs::schedule::StateError::StateAlreadyQueued=>{},
        bevy::ecs::schedule::StateError::AlreadyInState=>{},
        e=>{
          panic!({:?},e);
        }
      }
    };
  }
  .into()
}

#[cfg(test)]
mod intergration_test {
  use super::*;
  use crate::set_with_grab;
  use bevy::prelude::*;
  use quote::quote;
  use rstest::*;

  #[derive(Debug, Clone, PartialEq, Eq, Hash)]
  enum MockState {
    Before,
    After,
  }

  fn setter(state: ResMut<State<MockState>>) {}

  #[fixture]
  fn app() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins).add_state(MockState::Before);
  }

  fn change_state() {}
}
