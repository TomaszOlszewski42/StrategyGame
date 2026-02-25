use bevy::prelude::*;
use hexx::Hex;
use serde::{Deserialize, Serialize};
use crate::{GameState, OnDemandSystems, environment::{env_models::MarketAsset, map::{HexMap, SelectedHex, TerrainType}}, errors::my_errors::MyErrors, politics::{countries::Treasury, political_map::Occupied}};


#[derive(Component, Clone, Serialize, Deserialize)]
pub enum Building {
    Market
}

impl Building {
    fn bonus_gold(&self) -> i32 {
        match self {
            Building::Market => 2,
        }
    }
}

#[derive(Component, Clone, Serialize, Deserialize)]
pub enum BuildingsAggregate {
    Mountains(Option<Building>),
    Flat([Option<Building>; 3]),
    Empty,
}

impl BuildingsAggregate {
    pub fn new_by_terrain(terrain: &TerrainType) -> BuildingsAggregate {
        match terrain {
            TerrainType::Flat(_) | TerrainType::Forest(_) => Self::new_flat(),
            TerrainType::Mountain(_) => Self::new_mountain(),
            TerrainType::Water(_) => Self::new_empty(),
        }
    }

    pub fn new_mountain() -> BuildingsAggregate {
        BuildingsAggregate::Mountains(None)
    }

    pub fn new_flat() -> BuildingsAggregate {
        BuildingsAggregate::Flat([None, None, None])
    }

    pub fn new_empty() -> BuildingsAggregate {
        BuildingsAggregate::Empty
    }
}

impl BuildingsAggregate {
    pub fn bonus_gold(&self) -> i32 {
        match self {
            BuildingsAggregate::Mountains(building) => {
                if let Some(boxed) = building {
                    boxed.bonus_gold()
                } else {
                    0
                }
            },
            BuildingsAggregate::Flat(arr) => {
                arr.iter().fold(0, |mut sum, opt| {
                    match opt {
                        Some(boxed) => {
                            sum += boxed.bonus_gold(); 
                            sum 
                        },
                        None => sum,
                    }
                })
            },
            BuildingsAggregate::Empty => 0,
        }
    }
}


const PRICE: i32 = 90;

fn set_market_there(
    slot: &mut Option<Building>,
    treasury: &mut Treasury,
    commands: &mut Commands,
    hex: &Entity,
    market_model: &Res<MarketAsset>,
    terrain: &TerrainType,
    flat_idx: u32,
) {
    let market = Building::Market; 
    *slot = Some(market);
    treasury.gold -= PRICE;
    let mut transform: Transform = match terrain {
        TerrainType::Mountain(_) => Transform::from_xyz(0.0, 0.25, 0.0),
        _ => {
            if flat_idx == 0 {
                Transform::from_xyz(-0.12, 0.0, 0.05)
            } else if flat_idx == 1 {
                Transform::from_xyz(0.08,0.0, 0.1)
            } else {
                Transform::from_xyz(0.0, 0.0, -0.1)
            }
        },
    };
    transform = transform.with_scale(Vec3::splat(0.02));
    let child = commands.spawn((DespawnOnExit(GameState::Game), SceneRoot(market_model.scene.clone()), transform)).id();
    commands.entity(*hex).add_child(child);
}

pub fn build_market_bot(
    map: &mut HexMap, hex: &Hex, commands: &mut Commands, market_model: &Res<MarketAsset>, treasury: &mut Treasury,
) -> Result<(), MyErrors> {
    let Some((entity, terrain, buildings)) = map.entities.get_mut(hex) else {
        return Err(MyErrors::InconsistentData("Couldn't find Hex Entity to build market on".to_string()))
    };
    if treasury.gold < PRICE {
        return Ok(())
    };
    match buildings {
        BuildingsAggregate::Mountains(building) => {
            if building.is_none() {
                set_market_there(building, treasury, commands, 
                    entity, market_model, terrain, 0);
            }
        },
        BuildingsAggregate::Flat(arr) => {
            let mut counter = 0;
            for item in arr {
                if item.is_none() {
                    set_market_there(item, treasury, commands, 
                        entity, market_model, terrain, counter);
                    break;
                }
                counter += 1;
            }
        },
        BuildingsAggregate::Empty => return Err(MyErrors::InconsistentData("Somehow trying to build market on water".to_string())),
    }
    Ok(())
}

pub fn build_market(
    selected: Res<SelectedHex>,
    mut map: ResMut<HexMap>,
    mut countries: Query<&mut Treasury>,
    mut commands: Commands,
    market_model: Res<MarketAsset>,
    systems: Res<OnDemandSystems>,
    occupied: Res<Occupied>,
) -> Result<(), MyErrors> {
    let Some(hex) = selected.hex else {
        return Err(MyErrors::InconsistentData("No selected hex but building a market".to_string()))
    };
    if occupied.map.contains_key(&hex) {
        return Ok(())
    }
    let Some(&owner) = map.hex_owners.get(&hex) else {
        return Err(MyErrors::InconsistentData("Couldn't find hex owner".to_string()))
    };
    let Ok(mut treasury) = countries.get_mut(owner) else {
        return Err(MyErrors::InconsistentData("Couldn't find a treasury in query".to_string()))
    };
    build_market_bot(&mut map, &hex, &mut commands, &market_model, &mut treasury)?;
    if let Some(systemid) = systems.systems.get("update_country_tab_gold") {
        commands.run_system(*systemid);
    } else {
        return Err(MyErrors::InconsistentData("No registered `update_country_tab_gold`".to_string()))
    }
    Ok(())
}

pub fn redraw_markets(
    mut commands: Commands,
    map: Res<HexMap>,
    model: Res<MarketAsset>,
) {
    for (entity, terrain, buildings) in map.entities.values() {
        match buildings {
            BuildingsAggregate::Mountains(building) => {
                if building.is_none() {
                    continue;
                }
                let transform = Transform::from_xyz(0.0, 0.25, 0.0).with_scale(Vec3::splat(0.02));
                let child = commands.spawn((DespawnOnExit(GameState::Game), SceneRoot(model.scene.clone()), transform)).id();
                commands.entity(*entity).add_child(child);
            },
            BuildingsAggregate::Flat(arr) => {
                handle_flat_aggregate_markets(&mut commands, entity, arr, terrain, &model);
            },
            BuildingsAggregate::Empty => {},
        }
    }
}

fn handle_flat_aggregate_markets(
    commands: &mut Commands, 
    entity: &Entity, 
    arr: &[Option<Building>; 3], 
    terrain: &TerrainType,
    model: &Res<MarketAsset>,
) {
    for i in 0 .. arr.len() {
        if arr[i].is_none() {
            break;
        }
        let mut transform: Transform = match terrain {
            TerrainType::Mountain(_) => Transform::from_xyz(0.0, 0.25, 0.0),
            _ => {
                if i == 0 {
                    Transform::from_xyz(-0.12, 0.0, 0.05)
                } else if i == 1 {
                    Transform::from_xyz(0.08,0.0, 0.1)
                } else {
                    Transform::from_xyz(0.0, 0.0, -0.1)
                }
            },
        };
        transform = transform.with_scale(Vec3::splat(0.02));
        let child = commands.spawn((DespawnOnExit(GameState::Game), SceneRoot(model.scene.clone()), transform)).id();
        commands.entity(*entity).add_child(child);
    }
}