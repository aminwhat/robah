use asset_manager::AssetManagerPlugin;
use bevy::prelude::*;
use std::env;
use three_d::ThreeDPlugin;
use window::WindowPlugin;

pub struct PreludePlugin;

impl Plugin for PreludePlugin {
    fn build(&self, app: &mut App) {
        robah_prelude();

        app.add_plugins((
            DefaultPlugins,
            WindowPlugin,
            ThreeDPlugin,
            AssetManagerPlugin,
        ));
    }
}

pub fn robah_prelude() {
    let assets_path =
        String::from(env::current_dir().unwrap().to_str().unwrap()) + "/packages/asset_manager";
    env::set_var("BEVY_ASSET_ROOT", assets_path);
}
