use bevy::{asset::RenderAssetUsages, color::palettes::css::WHITE, mesh::{Indices, PrimitiveTopology}, prelude::*};
use hexx::{HexLayout, PlaneMeshBuilder};

use crate::{GameState, environment::map::HexMap, politics::political_map::SmallMesh, ui::saving_window::SaveMessageTimer};

pub fn setup_missing(
    mut commands: Commands,
    mut small_mesh: ResMut<SmallMesh>,
    map: Res<HexMap>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        DespawnOnExit(GameState::Game),
        DirectionalLight {
            color: WHITE.into(),
            illuminance: 1_000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(30.0, 50.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let mesh = create_hexagonal_mesh_smaller(&map.layout);
    let mesh_handle = meshes.add(mesh);
    small_mesh.mesh = mesh_handle;

    let new_timer = SaveMessageTimer::new();
    commands.insert_resource(new_timer);
}

pub fn create_hexagonal_mesh_smaller(layout: &HexLayout) -> Mesh {
    let info = PlaneMeshBuilder::new(layout)
        .facing(Vec3::Y)
        .with_scale(Vec3::splat(0.8))
        .center_aligned()
        .build();
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, info.vertices)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, info.normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, info.uvs)
        .with_inserted_indices(Indices::U16(info.indices))
}