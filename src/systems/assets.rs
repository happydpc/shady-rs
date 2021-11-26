use crate::resources::ShadyAssets;
use bevy::prelude::*;

pub fn setup_assets(
    mut commands: Commands,
    mut assets: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(ShadyAssets::load(&mut assets, &asset_server));
}