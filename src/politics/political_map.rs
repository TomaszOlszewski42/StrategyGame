use std::collections::HashMap;

use bevy::prelude::*;
use hexx::Hex;

use crate::{GameState, environment::map::{HexMap, SelectedHex, TerrainType}, errors::my_errors::MyErrors, politics::countries::{Country, CountryColor}};

#[derive(Resource, Default)]
pub struct SmallMesh {
    pub mesh: Handle<Mesh>,
}

#[derive(Resource, Default)]
pub struct Occupied {
    pub map: HashMap<Hex, Entity>,
}

#[derive(Component)]
pub struct OccupationEntity;

pub fn update_ownership_colors(
    map: ResMut<HexMap>,
    commands: Commands,
    countries: Query<&CountryColor, With<Country>>,
    selected: Res<SelectedHex>,
) -> Result<(), MyErrors> {
    if map.political {
        change_map_to_political(map, commands, countries, &selected)?;
    } else {
        change_map_to_terrain(map, commands, &selected);
    }

    Ok(())
}

pub fn change_map_mode(
    map: ResMut<HexMap>,
    commands: Commands,
    pressed: Res<ButtonInput<KeyCode>>,
    countries: Query<&CountryColor, With<Country>>,
    selected: Res<SelectedHex>,
) -> Result<(), MyErrors> {
    if !pressed.just_pressed(KeyCode::KeyM) {
        return Ok(())
    }

    if !map.political {
        change_map_to_political(map, commands, countries, &selected)?;
    } else {
        change_map_to_terrain(map, commands, &selected);
    }

    Ok(())
}

fn change_map_to_terrain(
    mut map: ResMut<HexMap>,
    mut commands: Commands,
    selected: &Res<SelectedHex>,
) {
    map.political = false;
    for (hex, (entity, terrain, _)) in map.entities.iter() {
        if let TerrainType::Water(_) = terrain {
            continue
        }
        if let Some(selected_hex) = selected.hex && selected_hex == *hex {
            continue
        }
        commands.entity(*entity).insert(MeshMaterial3d(terrain.material()));
    }
}

fn change_map_to_political(
    mut map: ResMut<HexMap>,
    mut commands: Commands,
    countries: Query<&CountryColor, With<Country>>,
    selected: &Res<SelectedHex>,
) -> Result<(), MyErrors> {
    map.political = true;
    for (hex, (entity, terrain, _)) in map.entities.iter() {
        if let TerrainType::Water(_) = terrain {
            continue
        }
        if let Some(selected_hex) = selected.hex && selected_hex == *hex {
            continue
        }
        match map.hex_owners.get(hex) {
            Some(owner) => {
                match countries.get(*owner) {
                    Ok(color) => {
                        commands.entity(*entity).insert(MeshMaterial3d(color.material.clone()));
                    },
                    Err(_) => {
                        return Err(MyErrors::InconsistentData("Couldn't find country color in query".to_string()));
                    },
                };
            },
            None => { commands.entity(*entity).insert(MeshMaterial3d(map.no_owner_material.clone())); },
        }
    }
    Ok(())
}

pub fn color_occupied_hexes(
    map: Res<HexMap>,
    occupied: Res<Occupied>,
    mut commands: Commands,
    mesh: Res<SmallMesh>,
    entities: Query<Entity, With<OccupationEntity>>,
    countries: Query<&CountryColor>,
) -> Result<(), MyErrors> {
    for entity in entities {
        commands.entity(entity).despawn();
    }
    if !map.political {
        return Ok(());
    }
    for (hex, occupator) in occupied.map.iter() {
        let owner = match countries.get(*occupator) {
            Ok(color) => color,
            Err(_) => return Err(MyErrors::InconsistentData("Couldn't find country color in query".to_string())),
        };
        let pos = map.layout.hex_to_world_pos(*hex);
        commands.spawn((
            DespawnOnExit(GameState::Game),
            Mesh3d(mesh.mesh.clone()),
            MeshMaterial3d(owner.material.clone()),
            Transform::from_xyz(pos.x, 0.01, pos.y),
            OccupationEntity,
        ));
    }

    Ok(())
}