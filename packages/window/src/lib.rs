use bevy::prelude::*;

mod close_on_esc;

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, close_on_esc::close_on_esc);
    }
}
