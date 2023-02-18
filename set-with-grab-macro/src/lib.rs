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
    match #state.set(#next){
      Ok(_)=>{},
      Err(e)=>{
        match e{
          bevy::ecs::schedule::StateError::StateAlreadyQueued=>{},
          bevy::ecs::schedule::StateError::AlreadyInState=>{},
          e=>{
            panic!("{:?}",e);
          }
        }
      }
    }
  }
  .into()
}
