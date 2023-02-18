# Set with grab

The crate is util for bevy in rust.

# Examples

```rust
fn setter_with_grab(mut state: ResMut<State<MockState>>) {
  // This is a macro of the crate!
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
```

# How to use

```rust
set_with_grab!(stateResourceMutable,variantOfTheState);
```

The errors, StateError::StateAlreadyQueued, and StateError::AlreadyInState, are ignored.  
Add, when other error occurred, the macro will throw a panic.  

# Details

The crate serves a macro. This set a variant to passed state internally.  
Why does internally? ――Because to ignore several errors.  
But do not afraid this crate. These ignored is frequent errors even better code, because a system of bevy is called parallely. So the state mutation does not sync on some thread, and two errors, StateError::StateAlreadyQueued, and StateError::AlreadyInState, is occurred frequently.  
And, the error handling is very dull. So, when like this, you can use this crate!  
By the way, ignoring errors is called "nigiritsubusu" in japanese. The calling can translate to "crash with grabbing" in English. This is origin of the crate name.  

# LICENCE

MIT
