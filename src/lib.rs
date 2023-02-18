pub use set_or_grab_macro::set_with_grab;

#[cfg(test)]
mod intergration_test {
  use super::set_with_grab;
  use bevy::prelude::*;
  use rstest::*;

  #[derive(Debug, Clone, PartialEq, Eq, Hash)]
  enum MockState {
    Before,
    After,
  }

  fn setter_with_grab(mut state: ResMut<State<MockState>>) {
    set_with_grab!(state, MockState::After);
    // if there is the under code, it will crash with running, but on testing, an error does not occurred.
    // state.set(MockState::After).unwrap();
  }

  #[fixture]
  fn app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins).add_state(MockState::Before);
    app
  }

  #[rstest]
  fn change_state_with_grab(mut app: App) {
    app.add_system_set(
      SystemSet::on_update(MockState::Before)
        .with_system(setter_with_grab),
    );
    app.update();
    let state = app.world.resource_mut::<State<MockState>>();
    match state.current() {
      MockState::After => {}
      MockState::Before => {
        panic!("the state is before, yet.");
      }
    }
  }
}
