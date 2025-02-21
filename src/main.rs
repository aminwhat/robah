use bevy::prelude::*;
use prelude::PreludePlugin;
use startup;

mod prelude;

fn main() {
    let result = startup::launch();

    if result.is_ok() {
        App::new().add_plugins(PreludePlugin).run();
    }
}
