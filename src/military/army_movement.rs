use bevy::{prelude::*};
use hexx::Hex;
use crate::{environment::map::{HexMap, SelectedHex, TerrainType, recolor_old_selected_hex}, errors::my_errors::MyErrors, military::army::Army, politics::{countries::{AtWarLists, Country, CountryColor, PlayerType}, political_map::Occupied}};

#[derive(Resource, Default, Clone)]
pub struct PossibleMoves {
    pub moves: Vec<Hex>,
}

pub fn recolor_old_possible_moves(
    map: Res<HexMap>,
    possible_moves: Res<PossibleMoves>,
    mut commands: Commands,
    countries: Query<&CountryColor, With<Country>>,
) -> Result<(), MyErrors> {
    for hex in &possible_moves.moves {
        recolor_old_selected_hex(&map, &mut commands, hex, &countries)?;
    };

    Ok(())
}

pub fn select_possible_moves(
    selected_hex_res: Res<SelectedHex>,
    map: Res<HexMap>,
    mut possible_moves: ResMut<PossibleMoves>,
    at_war_with: Res<AtWarLists>,
    countries: Query<&PlayerType>,
) -> Result<(), MyErrors> {
    let (selected_hex, army, owner) = match possible_moves_starting_checks(&selected_hex_res, &map, &mut possible_moves)? {
        Some(tuple) => tuple,
        None => return Ok(()),
    };
    if let Ok(player) = countries.get(*owner) {
        match player {
            PlayerType::Player => {},
            PlayerType::Bot => {
                possible_moves.moves = vec![];
                return Ok(())
            },
        }
    } else {
        return Err(MyErrors::InconsistentData("Failed to get player from query".to_string()))
    }
    let Some(enemies) = at_war_with.lists.get(owner) else {
        return Err(MyErrors::InconsistentData("War Lists".to_string()))
    };
    let posible = possible_moves_vec(army, selected_hex, &map, owner, enemies);
    possible_moves.moves = posible;
    Ok(())
}

fn possible_moves_starting_checks<'a>(
    selected_hex_res: &'a Res<SelectedHex>,
    map: &'a HexMap,
    possible_moves: &'a mut ResMut<PossibleMoves>,
) -> Result<Option<(Hex, &'a Army, &'a Entity)>, MyErrors> {
    let Some(selected_hex) = selected_hex_res.hex else {
        possible_moves.moves = vec![];
        return Ok(None)
    };

    let Some(army) = map.armies.get(&selected_hex) else {
        possible_moves.moves = vec![];
        return Ok(None)
    };

    let Some(owner) = map.hex_owners.get(&selected_hex) else {
        return Err(MyErrors::InconsistentData("Failed to get Hex Entity".to_string()))
    };

    Ok(Some((selected_hex, army, owner)))
}

pub fn possible_moves_vec(army: &Army, selected_hex: Hex, map: &HexMap, owner: &Entity, enemies: &Vec<Entity>) -> Vec<Hex> {
    let mut posible = Vec::<Hex>::new();
    if army.move_points_left > 0 {
        for neighbour in selected_hex.all_neighbors() {
            if let Some((_, terrain, _)) = map.entities.get(&neighbour)
                && let TerrainType::Water(_) = terrain {
                    continue;
                }
            if let Some(neigh_owner) = map.hex_owners.get(&neighbour)
                && neigh_owner != owner && !enemies.contains(neigh_owner) {
                    continue;
                }
            posible.push(neighbour);
        }
    };
    posible
}

pub fn color_player_possible_moves(
    mut commands: Commands,
    map: Res<HexMap>,
    possible_moves: Res<PossibleMoves>,
) {
    for hex in &possible_moves.moves {
        if let Some((entity, _, _)) = map.entities.get(hex) {
            commands.entity(*entity).insert(MeshMaterial3d(map.possible_moves_material.clone()));
        }
    }
}

pub fn move_army(
    old_pos: &Hex,
    new_pos: &Hex,
    map: &mut HexMap,
    occupied: &mut ResMut<Occupied>,
    moving_player: Entity,
) -> Result<(), MyErrors> {
    let owner_of_new = map.hex_owners.clone().remove(new_pos);

    let Some(mut army_old) = map.armies.remove(old_pos) else {
        return Ok(()) // boty czasami to wywołują, nie chce mi się szukać czemu
    };

    if moving_player != army_old.owner {
        map.armies.insert(*old_pos, army_old);
        return Ok(())
    }
    army_old.move_points_left -= 1;

    move_army_match(map, owner_of_new, new_pos, occupied, army_old)
}

fn move_army_match(map: &mut HexMap, owner_of_new: Option<Entity>,
    new_pos: &Hex, occupied: &mut ResMut<Occupied>, army_old: Army
) -> Result<(), MyErrors> {
    let new_army_opt = map.armies.get_mut(new_pos);
    match (owner_of_new, new_army_opt) {
        (None, None) => {
            map.hex_owners.insert(*new_pos, army_old.owner);
            map.armies.insert(*new_pos, army_old);
        },
        (None, Some(_)) => { return Err(MyErrors::InconsistentData("Army on unowned Hex".to_string())) },
        (Some(owner), None) => { some_owner_no_army(map, army_old, new_pos, owner, occupied); },
        (Some(owner), Some(new_army)) => {
            if army_old.owner == new_army.owner {
                new_army.merge_with_other(army_old);
            } else if let Some(winner) = Army::battle(&army_old, new_army) {
                let winner_owner = winner.owner;
                map.armies.insert(*new_pos, winner);
                if owner != winner_owner {
                    map.hex_owners.insert(*new_pos, winner_owner);
                    if !occupied.map.contains_key(new_pos) {
                        occupied.map.insert(*new_pos, owner);
                    } else if let Some(&who) = occupied.map.get(new_pos)
                    && who == winner_owner {
                        occupied.map.remove(new_pos);
                    }
                }
            }
        },
    }
    Ok(())
}

fn some_owner_no_army(map: &mut HexMap, army_old: Army, new_pos: &Hex, owner: Entity, 
    occupied: &mut ResMut<Occupied>
) {
    let army_old_owner = army_old.owner;
        map.hex_owners.insert(*new_pos, army_old.owner);
        map.armies.insert(*new_pos, army_old);
        if occupied.map.contains_key(new_pos) {
            if let Some(&who) = occupied.map.get(new_pos)
                && who == army_old_owner {
                    occupied.map.remove(new_pos);
                }
        } else if owner != army_old_owner {
            occupied.map.insert(*new_pos, owner);
    }
}