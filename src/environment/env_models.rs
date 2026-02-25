use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct TreeAsset {
    pub scene: Handle<Scene>,
}

#[derive(Resource, Default)]
pub struct MountainAsset {
    pub scene: Handle<Scene>,
}

#[derive(Resource, Default)]
pub struct MarketAsset {
    pub scene: Handle<Scene>
}

pub fn load_models(
    asset_server: &Res<AssetServer>, 
    tree: &mut ResMut<TreeAsset>,
    mountain: &mut ResMut<MountainAsset>,
    market: &mut ResMut<MarketAsset>,
) {
    tree.scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset("tree.glb"));
    mountain.scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset("mountains.glb"));
    market.scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset("stall.glb"));
}
