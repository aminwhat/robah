use bevy::prelude::*;
use models::FOX_CHARACTER_MODEL;

mod models;

pub struct AssetManagerPlugin;

impl Plugin for AssetManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_fox);
    }
}

fn load_fox(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SceneRoot(
        asset_server.load(GltfAssetLabel::Scene(0).from_asset(FOX_CHARACTER_MODEL)),
    ));
}
