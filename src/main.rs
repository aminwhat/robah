use bevy::prelude::*;
use prelude::PreludePlugin;

mod prelude;

fn main() {
    App::new().add_plugins(PreludePlugin).run();
}
