use std::collections::HashMap;

use bevy::{asset::RenderAssetUsages, color::palettes::css::{BLUE, GREEN, GREY, LIME, RED, SLATE_GREY, WHITE, YELLOW}, mesh::{Indices, PrimitiveTopology}, prelude::*, window::PrimaryWindow};
use hexx::{Hex, HexLayout, PlaneMeshBuilder, shapes};
use rand::{Rng, rngs::ThreadRng};
use serde::{Deserialize, Serialize};

use crate::{GameState, OnDemandSystems, environment::{buildings::BuildingsAggregate, env_models::{MountainAsset, TreeAsset}}, errors::my_errors::MyErrors, military::{army::Army, army_movement::{PossibleMoves, move_army}}, politics::{countries::{Country, CountryColor}, political_map::Occupied}};

#[derive(Component, Clone, Serialize, Deserialize)]
pub enum TerrainType {
    Flat(#[serde(skip)] Handle<StandardMaterial>),
    Forest(#[serde(skip)] Handle<StandardMaterial>),
    Mountain(#[serde(skip)] Handle<StandardMaterial>),
    Water(#[serde(skip)] Handle<StandardMaterial>),
}

impl TerrainType {
    pub fn material(&self) -> Handle<StandardMaterial> {
        match self {
            TerrainType::Flat(handle) => handle.clone(),
            TerrainType::Forest(handle) => handle.clone(),
            TerrainType::Mountain(handle) => handle.clone(),
            TerrainType::Water(handle) => handle.clone(),
        }
    }
}

#[derive(Resource, Clone, Serialize, Deserialize, Default)]
pub struct HexMap {
    pub political: bool,
    pub layout: HexLayout,
    pub entities: HashMap<Hex, (Entity, TerrainType, BuildingsAggregate)>,
    #[serde(skip)]
    pub highlighted_material: Handle<StandardMaterial>,
    pub hex_owners: HashMap<Hex, Entity>,
    #[serde(skip)]
    pub no_owner_material: Handle<StandardMaterial>,
    pub armies: HashMap<Hex, Army>,
    #[serde(skip)]
    pub selected_material: Handle<StandardMaterial>,
    #[serde(skip)]
    pub possible_moves_material: Handle<StandardMaterial>,
}

#[derive(Resource, Clone, Default)]
pub struct HighlightedHexes {
    pub pointing_at: Option<Hex>
}

#[derive(Resource, Default, Clone, Serialize, Deserialize)]
pub struct SelectedHex {
    pub hex: Option<Hex>
}

pub fn recolor_old_selected_hex(
    map: &HexMap,
    commands: &mut Commands,
    selected_old: &Hex,
    countries: &Query<&CountryColor, With<Country>>,
) -> Result<(), MyErrors> {
    let Some(entity) = map.entities.get(selected_old) else {
        return Ok(()) // selected może być ustawiony na hex, który znajduje się poza mapą
    };

    if let TerrainType::Water(_) = entity.1 {
        commands.entity(entity.0).insert(MeshMaterial3d(entity.1.material()));
        return Ok(())
    };

    if map.political {
        if let Some(owner) = map.hex_owners.get(selected_old) {
            if let Ok(color) = countries.get(*owner) {
                commands.entity(entity.0).insert(MeshMaterial3d(color.material.clone()));
            } else {
                return Err(MyErrors::InconsistentData("Couldn't find country color in query".to_string()))
            }
        } else {
            commands.entity(entity.0).insert(MeshMaterial3d(map.no_owner_material.clone()));
        }
    } else {
        commands.entity(entity.0).insert(MeshMaterial3d(entity.1.material()));
    };

    Ok(())
}

pub fn select_hex(
    mut selected: ResMut<SelectedHex>,
    button: Res<ButtonInput<MouseButton>>,
    mut map: ResMut<HexMap>,
    camera_elems: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    countries: Query<&CountryColor, With<Country>>,
    systems: Res<OnDemandSystems>,
    possible_moves: Res<PossibleMoves>,
    mut occupied: ResMut<Occupied>,
) -> Result<(), MyErrors> {
    if !button.just_pressed(MouseButton::Left) {
        return Ok(());
    }
    let (camera, camera_transform) = (camera_elems.0, camera_elems.1);
    let Ok(clicked_hex) = find_hex_to_highlight(&map, window, camera, camera_transform) else {
        return Ok(())
    };
    handle_old_selected(&mut commands, clicked_hex, &selected, &mut map, &possible_moves, &countries, &mut occupied, &systems)?;
    selected.hex = Some(clicked_hex);
    if let Some(systemid) = systems.systems.get("update_army_tab") {
        commands.run_system(*systemid);
    }
    let Some((entity, _, _)) = map.entities.get(&clicked_hex) else {
        return Ok(()) // sytuacja gdy kliknie się poza mapą
    };
    commands.entity(*entity).insert(MeshMaterial3d(map.selected_material.clone()));
    Ok(())
}

fn handle_old_selected(
    commands: &mut Commands, clicked_hex: Hex, selected: &ResMut<SelectedHex>, map: &mut HexMap, 
    possible_moves: &Res<PossibleMoves>, countries: &Query<&CountryColor, With<Country>>, 
    occupied: &mut ResMut<Occupied>, systems: &Res<OnDemandSystems>
) -> Result<(), MyErrors> {
    if let Some(selected_old) = selected.hex {
        if selected_old == clicked_hex {
            return Ok(())
        }

        recolor_old_selected_hex(&map, commands, &selected_old, &countries)?;
        if possible_moves.moves.contains(&clicked_hex) {
            let me = match map.hex_owners.get(&selected_old) {
                Some(entity) => *entity,
                None => return Err(MyErrors::InconsistentData("Before functions used old selected hex but currently there is none selected".to_string())),
            }; 
            move_army(&selected_old, &clicked_hex, map, occupied, me)?;
            if let Some(systemid) = systems.systems.get("redraw_armies_models") {
                commands.run_system(*systemid);
            } else {
                return Err(MyErrors::InconsistentData("Couldn't find `redraw_armies_models`".to_string()));
            }
        }
    }
    Ok(())
}

fn find_hex_to_highlight(
    map: &HexMap,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Result<Hex, ()> {
    let Some(cursor_pos) = window.cursor_position() else {
        return Err(())
    };
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) else {
        return Err(())
    };
    let Some(dist) = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Dir3::Y)) else {
        return Err(())
    };
    let point = ray.origin + dist * ray.direction;
    Ok(map.layout.world_pos_to_hex(point.xz()))
}

fn recolor_highlighted_hex(
    current: &Hex,
    coord: &Hex,
    map: &Res<HexMap>,
    countries: Query<&CountryColor, With<Country>>,
    commands: &mut Commands,
    highligthed: &mut ResMut<HighlightedHexes>,
    selected: &Res<SelectedHex>,
) -> Result<(), MyErrors> {
    if current == coord {
        return Ok(());
    }
    if let Some(entity) = map.entities.get(current) {
        if let TerrainType::Water(_) = entity.1 {
            commands.entity(entity.0).insert(MeshMaterial3d(entity.1.material()));
            highligthed.pointing_at = None;
            return Ok(())
        }
        
        if let Some(hex) = selected.hex  && hex == *current {
            commands.entity(entity.0).insert(MeshMaterial3d(map.selected_material.clone()));
            return Ok(())
        }

        recolors_based_on_mode(map, commands, current, &countries, entity)?;
        highligthed.pointing_at = None;
    }
    Ok(()) 
}

fn recolors_based_on_mode(
    map: &HexMap, commands: &mut Commands, current: &Hex, countries: &Query<&CountryColor, With<Country>>,
    entity: &(Entity, TerrainType, BuildingsAggregate)
) -> Result<(), MyErrors> {
    if map.political {
        if let Some(owner) = map.hex_owners.get(current) {
            if let Ok(color) = countries.get(*owner) {
                commands.entity(entity.0).insert(MeshMaterial3d(color.material.clone()));
            } else {
                return Err(MyErrors::InconsistentData("Couldn't find country color in query".to_string()))
            }
        } else {
            commands.entity(entity.0).insert(MeshMaterial3d(map.no_owner_material.clone()));
        }
    } else {
        commands.entity(entity.0).insert(MeshMaterial3d(entity.1.material()));
    }

    Ok(())
}

pub fn highlight_hex(
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,
    camera_elems: Single<(&Camera, &GlobalTransform)>,
    map: Res<HexMap>,
    mut highligthed: ResMut<HighlightedHexes>,
    countries: Query<&CountryColor, With<Country>>,
    selected: Res<SelectedHex>,
) -> Result<(), MyErrors> {
    let (camera, camera_transform) = (camera_elems.0, camera_elems.1);
    let Ok(coord) = find_hex_to_highlight(&map, window, camera, camera_transform) else {
        return Ok(());
    };

    if let Some(current) = highligthed.pointing_at {
        recolor_highlighted_hex(&current, &coord, &map, countries, &mut commands, &mut highligthed, &selected)?;
    }

    let Some(entity) = map.entities.get(&coord) else {
        return Ok(());
    };

    commands.entity(entity.0).insert(MeshMaterial3d(map.highlighted_material.clone()));
    highligthed.pointing_at = Some(coord);

    Ok(())
}

fn gen_hex_terrain(
    hex: &Hex,
    land_limit: &i32,
    material_water: &Handle<StandardMaterial>,
    material_grass: &Handle<StandardMaterial>,
    material_forest: &Handle<StandardMaterial>,
    material_mountains: &Handle<StandardMaterial>,
    water: &TerrainType,
    grass: &TerrainType,
    forest: &TerrainType,
    mountains: &TerrainType,
    rng: &mut ThreadRng
) -> (Handle<StandardMaterial>, TerrainType) {
    if hex.x.abs() > *land_limit 
        || hex.y.abs() > *land_limit
        || (hex.x + hex.y).abs() > *land_limit {
        (material_water.clone(), water.clone())
    } else {
        match rng.random::<u32>() % 13 {
            0 ..= 4 => (material_grass.clone(), grass.clone()),
            5 ..= 9 => (material_forest.clone(), forest.clone()),
            10 ..= 11 => (material_mountains.clone(), mountains.clone()),
            12 => (material_water.clone(), water.clone()),
            _ => (material_grass.clone(), grass.clone()),
        }
    }
}

pub fn spawn_hex_entity(
    commands: &mut Commands,  layout: &HexLayout, hex: &Hex, mesh_handle: &Handle<Mesh>, material: Handle<StandardMaterial>,  
    terraint_type: &TerrainType, tree_model: &ResMut<TreeAsset>, mountain_model: &ResMut<MountainAsset>,
) -> Entity {
    let pos = layout.hex_to_world_pos(*hex);
    let bundle = (
        DespawnOnExit(GameState::Game),
        Mesh3d(mesh_handle.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(pos.x, 0.0, pos.y)
    );
    match terraint_type {
        TerrainType::Forest(_) => commands.spawn(bundle).with_children(|parent| {
            for (a, b) in [(0.1, 0.0), (0.0, 0.1), (-0.1, -0.05)] {
                parent.spawn((
                    DespawnOnExit(GameState::Game),
                    SceneRoot(tree_model.scene.clone()),
                    Transform::from_xyz(a, -0.012, b).with_scale(Vec3::splat(0.01)),
                ));
            }
        }).id(),
        TerrainType::Mountain(_) => commands.spawn(bundle).with_child((
            DespawnOnExit(GameState::Game),
            SceneRoot(mountain_model.scene.clone()),
            Transform::from_xyz(0.0,0.0, 0.0).with_scale(Vec3::splat(0.02)),
        )).id(),
        _ => commands.spawn(bundle).id(),
    }
}

fn create_hex_entities(
    commands: &mut Commands,    
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    tree_model: &ResMut<TreeAsset>,
    mountain_model: &ResMut<MountainAsset>,
    layout: &HexLayout,
) -> HashMap<Hex, (Entity, TerrainType, BuildingsAggregate)> {
    let mut rng = rand::rng();
    let material_water = materials.add(Color::Srgba(BLUE));
    let material_grass = materials.add(Color::Srgba(LIME));
    let material_forest = materials.add(Color::Srgba(GREEN));
    let material_mountains = materials.add(Color::Srgba(SLATE_GREY));
    let mesh = create_hexagonal_mesh(layout);
    let mesh_handle = meshes.add(mesh);
    let water = TerrainType::Water(material_water.clone());
    let grass = TerrainType::Flat(material_grass.clone());
    let mountains = TerrainType::Mountain(material_mountains.clone());
    let forest = TerrainType::Forest(material_forest.clone());
    let land_limit = 15;
    shapes::hexagon(Hex::new(0, 0), 20) .map(|hex| {
            let (material, terraint_type) 
            = gen_hex_terrain(&hex, &land_limit, &material_water, &material_grass, &material_forest, 
                &material_mountains, &water, &grass, &forest, &mountains, &mut rng);

            let id = spawn_hex_entity(commands, layout, &hex, &mesh_handle, material, &terraint_type, 
                tree_model, mountain_model);
            let buildings = BuildingsAggregate::new_by_terrain(&terraint_type);
            (hex, (id, terraint_type, buildings))
        }).collect()
}

pub fn setup_map(
    commands: &mut Commands,    
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    tree_model: &ResMut<TreeAsset>,
    mountain_model: &ResMut<MountainAsset>,
    hex_map: &mut ResMut<HexMap>,
) {
    let layout = HexLayout::flat().with_hex_size(0.2);

    let entities= 
        create_hex_entities(commands, meshes, materials, tree_model, mountain_model, &layout);
    let highlighted_material = materials.add(Color::Srgba(WHITE));
    let no_owner_material = materials.add(Color::Srgba(GREY));
    let selected_material = materials.add(Color::Srgba(YELLOW));
    let possible_moves_material = materials.add(Color::Srgba(RED));
    hex_map.political = false;
    hex_map.layout = layout;
    hex_map.highlighted_material = highlighted_material;
    hex_map.entities = entities;
    hex_map.hex_owners = HashMap::new();
    hex_map.no_owner_material = no_owner_material;
    hex_map.armies = HashMap::new();
    hex_map.selected_material = selected_material;
    hex_map.possible_moves_material = possible_moves_material;

}

pub fn create_hexagonal_mesh(layout: &HexLayout) -> Mesh {
    let info = PlaneMeshBuilder::new(layout)
        .facing(Vec3::Y)
        .with_scale(Vec3::splat(1.0))
        .center_aligned()
        .build();
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, info.vertices)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, info.normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, info.uvs)
        .with_inserted_indices(Indices::U16(info.indices))
}