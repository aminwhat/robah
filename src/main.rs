use bevy::prelude::*;
use three_d::ThreeDPlugin;
use window::WindowPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WindowPlugin, ThreeDPlugin))
        .run();
}
