use std::collections::{HashMap, VecDeque};

use bevy::{prelude::*};
use hexx::{Hex};
use rand::{Rng, rngs::ThreadRng, seq::SliceRandom};

use crate::{environment::{buildings::build_market_bot, env_models::MarketAsset, map::{HexMap, TerrainType}}, errors::my_errors::MyErrors, military::{army_movement::move_army, recruitment::recruit_to_map}, politics::{countries::{AtWarLists, CountriesSolidersNumbers, Country, PeaceProposals, PlayerType, SoldierAsset, Treasury}, political_map::Occupied, war::{end_war, start_war}}};

pub fn run_bots(
    players: &mut Query<(Entity, &PlayerType, &mut PeaceProposals, &mut Treasury, &SoldierAsset), With<Country>>,
    soldiers_nums: &mut ResMut<CountriesSolidersNumbers>,
    countries_wars: &mut ResMut<AtWarLists>,
    map: &mut ResMut<HexMap>,
    commands: &mut Commands,
    market_model: Res<MarketAsset>,
    occupied: &mut ResMut<Occupied>,
) -> Result<(), MyErrors> {
    for (player, player_type, mut peace_proposals, mut treasury, soldier) in players {
        if let PlayerType::Player = player_type {
            continue;
        }
        

        manage_peace_talks(&mut peace_proposals, countries_wars, 
                &player, soldiers_nums, &treasury, occupied, map)?;
        let war_list = match countries_wars.lists.get(&player) {
            Some(vec) => vec.clone(),
            None => return Err(MyErrors::InconsistentData("War Lists".to_string())),
        };

        let Some(&my_num) = soldiers_nums.numbers.get(&player) else {
            return Err(MyErrors::InconsistentData("CountriesSoldiersNumbers Resource".to_string()))
        };

        for (entity, value) in soldiers_nums.numbers.iter() {
            if my_num >= 2 * value && my_num > 0 {
                start_war(player, *entity, countries_wars);
            } 
        }

        if !war_list.is_empty() {
            manage_wars(countries_wars, map, &player, commands, &mut treasury, 
                soldier, occupied, soldiers_nums)?;
        } else {
            manage_during_peace(map, &player, commands, &mut treasury, 
                soldier, &market_model, occupied, soldiers_nums)?;
        }
    }

    Ok(())
}

fn manage_peace_talks(
    proposals: &mut PeaceProposals,
    countries_wars: &mut ResMut<AtWarLists>,
    me: &Entity,
    soldiers_nums: &ResMut<CountriesSolidersNumbers>,
    treasury: &Treasury,
    occupied: &mut ResMut<Occupied>,
    map: &ResMut<HexMap>,
) -> Result<(), MyErrors> {

    if proposals.senders.is_empty() {
        return Ok(())
    }

    let Some(my_nums) = soldiers_nums.numbers.get(me) else {
        return Err(MyErrors::InconsistentData("CountriesSoldiersNumbers Resource (peace talks)".to_string()));
    };
    let mut my_wars = match countries_wars.lists.get(me) {
        Some(list) => list.len(),
        None => return Err(MyErrors::InconsistentData("War Lists".to_string())),
    };
    for i in (0 .. proposals.senders.len()).rev() {
        handle_peace_proposal(proposals, countries_wars, me, soldiers_nums, treasury, occupied, map, &mut my_wars, my_nums, i)?;
    }
    Ok(())
}

fn handle_peace_proposal(
    proposals: &mut PeaceProposals,
    countries_wars: &mut ResMut<AtWarLists>,
    me: &Entity,
    soldiers_nums: &ResMut<CountriesSolidersNumbers>,
    treasury: &Treasury,
    occupied: &mut ResMut<Occupied>,
    map: &ResMut<HexMap>,
    my_wars: &mut usize,
    my_nums: &i32,
    ind: usize,
) -> Result<(), MyErrors> {
    if *my_wars > 1 {
        end_war(*me, proposals.senders[ind], countries_wars, map, occupied);
        proposals.senders.remove(ind);
    } else {
        let Some(enemy_nums) = soldiers_nums.numbers.get(&proposals.senders[ind]) else {
            return Err(MyErrors::InconsistentData("CountriesSoldiersNumbers Resource (peace talks)".to_string()));
        };
        if my_nums + treasury.gold / 10 < *enemy_nums {
            end_war(*me, proposals.senders[ind], countries_wars, map, occupied);    
        }
        proposals.senders.remove(ind);
    }
    Ok(())
}

fn manage_wars(
    countries_wars: &ResMut<AtWarLists>,
    map: &mut HexMap,
    me: &Entity,
    commands: &mut Commands,
    treasury: &mut Treasury,
    soldier: &SoldierAsset,
    occupied: &mut ResMut<Occupied>,
    numbers: &mut ResMut<CountriesSolidersNumbers>,
) -> Result<(), MyErrors> {
    let my_hexes: Vec<Hex> = map.hex_owners.iter()
        .filter(|&x| {
            x.1 == me && !occupied.map.contains_key(x.0)
        }).map(|x| {
            *x.0
        }).collect();
    if my_hexes.is_empty() {
        return Ok(())
    }
    let Some(enemies) = countries_wars.lists.get(me) else {
        return Err(MyErrors::InconsistentData("War Lists".to_string()))
    };
    let iterator: Vec<(Hex, i32)> = map.armies.iter()
        .filter(|&x| {
            x.1.owner == *me
        }).map(|x| { (*x.0, x.1.move_points_left) })
        .collect();
    move_armies_for_war(&iterator, map, enemies, me, occupied)?;
    recruit_army_on_random_hex(&my_hexes, treasury, commands, me, map, soldier, numbers)?;
    Ok(())
}

fn recruit_army_on_random_hex(
    my_hexes: &[Hex],
    treasury: &mut Treasury,
    commands: &mut Commands,
    me: &Entity,
    map: &mut HexMap,
    soldier: &SoldierAsset,
    numbers: &mut ResMut<CountriesSolidersNumbers>, 
) -> Result<(), MyErrors> {
    let mut generator = rand::rng();
    let ind = generator.random_range(0 .. my_hexes.len());
    let some_hex = my_hexes[ind];
    let Some(hex_entity) = map.entities.get(&some_hex) else {
        return Err(MyErrors::InconsistentData("Hex Entity Not Found".to_string()))
    };
    recruit_to_map(treasury, (treasury.gold - 10) / 10, (treasury.gold - 10) / 10 * 10, 
        soldier, me, map, some_hex, hex_entity.0, commands, numbers);

    Ok(())
}

fn move_armies_for_war(
    iterator: &Vec<(Hex, i32)>,
    map: &mut HexMap,
    enemies: &[Entity],
    me: &Entity,
    occupied: &mut ResMut<Occupied>,
) -> Result<(), MyErrors> {
    for (hex, moves) in iterator {
        let mut moves_left = *moves;
        let mut coord = *hex;
        while moves_left > 0 {
            let closest = find_closest_enemy_hex(coord, map, enemies, me);
            match closest {
                Some(closest_path) => {
                    for (_, pos) in (0 .. moves_left).zip(closest_path) {
                        let new_pos = pos;
                        move_army( &coord, &new_pos, map, occupied, *me)?;
                        coord = new_pos;
                        moves_left -= 1;
                    }
                },
                None => {
                    break;
                },
            }
        }
    }
    Ok(())
}

fn find_closest_enemy_hex(
    start: Hex,
    map: &HexMap,
    enemies: &[Entity],
    me: &Entity,
) -> Option<Vec<Hex>> {
    let mut queue = VecDeque::<Hex>::new();
    let mut visited = HashMap::<Hex, Hex>::new();
    
    visited.insert(start, Hex {x: -100000, y: -100000});
    queue.push_back(start);
    
    let closest_hex = 'outer: loop {
        let hex = match queue.pop_front() {
            Some(inside) => inside,
            None => break None,
        };
        for neighbour in hex.all_neighbors() {
            if closest_enemy_bfs_neighbour_handle(hex, map, neighbour, enemies, &mut visited, &mut queue, me) {
                break 'outer Some(neighbour);
            }
        }
    };

    reconstruct_bfs_path(closest_hex, &start, &visited)
}

fn closest_enemy_bfs_neighbour_handle(hex: Hex, map: &HexMap, neighbour: Hex, enemies: &[Entity], 
    visited: &mut HashMap<Hex, Hex>, queue: &mut VecDeque<Hex>, me: &Entity
) -> bool {
    if let Some((_, terrain, _)) = map.entities.get(&neighbour) {
        if let TerrainType::Water(_) = terrain {
            return false;
        }
    } else {
        return false;
    }
    if let Some(owner) = map.hex_owners.get(&neighbour) {
        if owner == me && !visited.contains_key(&neighbour) {
            visited.insert(neighbour, hex);
            queue.push_back(neighbour);
        }
        else if enemies.contains(owner) {
            visited.insert(neighbour, hex);
            return true;
        }
    } else if let std::collections::hash_map::Entry::Vacant(e) = visited.entry(neighbour) {
        e.insert(hex);
        queue.push_back(neighbour);
    }

    false
}

fn reconstruct_bfs_path(
    closest_hex: Option<Hex>,
    start: &Hex,
    visited: &HashMap<Hex, Hex>,
) -> Option<Vec<Hex>> {
    let mut result = Vec::new();
    match closest_hex {
        Some(closest_hex) => {
            let mut changing = closest_hex;
            result.push(closest_hex);
            while let Some(hex) = visited.get(&changing) {
                if hex == start || hex.x < -1000 {
                    break
                }
                changing = *hex;
                result.push(changing);
            }
            result.reverse();
            Some(result)
        },
        None => None,
    }
}

fn manage_during_peace(
    map: &mut HexMap,
    me: &Entity,
    commands: &mut Commands,
    treasury: &mut Treasury,
    soldier_asset: &SoldierAsset,
    market_model: &Res<MarketAsset>,
    occupied: &mut ResMut<Occupied>,
    soldiers_nums: &mut ResMut<CountriesSolidersNumbers>,
) -> Result<(), MyErrors> {
    move_armies_during_peace(map, me, occupied)?;
    let iterator = my_armies_nums(map, me);
    let armies_num = iterator.len();
    let my_hexes = my_hexes(map, me, occupied);
    let mut generator = rand::rng();
    recruit_armies_during_peace(armies_num, treasury, &mut generator, &my_hexes, map, me, commands, soldiers_nums, soldier_asset)?;
    let Some(&my_num) = soldiers_nums.numbers.get(me) else {
        return Err(MyErrors::InconsistentData("Soldiers Nums".to_string()))
    };
    compare_with_other(my_num, soldiers_nums, &mut generator, &iterator, map, armies_num, treasury, commands, me, soldier_asset)?;
    if treasury.gold > 200 {
        while treasury.gold > 100 {
            let ind = generator.random_range(0 .. my_hexes.len());
            let coord = my_hexes[ind];
            build_market_bot(map, &coord, commands, market_model, treasury)?;
        }
    }
    Ok(())
}

fn my_armies_nums(map: &HexMap, me: &Entity) -> Vec<(Hex, i32)> {
    map.armies.iter()
        .filter(|&x| {
            x.1.owner == *me
        })
        .map(|x| { (*x.0, x.1.move_points_left) })
        .collect()
}

fn my_hexes(map: &HexMap, me: &Entity, occupied: &ResMut<Occupied>,) -> Vec<Hex> {
    map.hex_owners.iter()
        .filter(|&x| {
            x.1 == me && !occupied.map.contains_key(x.0)
        })
        .map(|x| {
            *x.0
        })
        .collect()
}

fn recruit_armies_during_peace(armies_num: usize, treasury: &mut Treasury, generator: &mut ThreadRng,
    my_hexes: &[Hex], map: &mut HexMap, me: &Entity, commands: &mut Commands, 
    soldiers_nums: &mut ResMut<CountriesSolidersNumbers>, soldier_asset: &SoldierAsset
) -> Result<(), MyErrors> {
    const MIN_SOLDIERS: i32 = 2;
    if armies_num < 5 {
        for _ in 0 .. (5 - armies_num) {
            if treasury.gold < MIN_SOLDIERS * 10 {
                break;
            }
        
            let ind = generator.random_range(0 .. my_hexes.len());
            let coord = my_hexes[ind];
            let Some((hex_entity, _, _)) = map.entities.get(&coord) else {
                return Err(MyErrors::InconsistentData("Hex Entityt not found".to_string()))
            };
            recruit_to_map(treasury, MIN_SOLDIERS, 10 * MIN_SOLDIERS, 
                soldier_asset, me, map, coord, *hex_entity, commands, soldiers_nums);
        }
    }

    Ok(())
}

fn compare_with_other(my_num: i32, soldiers_nums: &mut ResMut<CountriesSolidersNumbers>, generator: &mut ThreadRng,
    iterator: &[(Hex, i32)], map: &mut HexMap, armies_num: usize, treasury: &mut Treasury, commands: &mut Commands,
    me: &Entity, soldier_asset: &SoldierAsset
) -> Result<(), MyErrors> {
    for num in soldiers_nums.numbers.values() {
        if *num > 2 * my_num {
            let ind = generator.random_range(0 .. armies_num);
            let (coord, _) = iterator[ind];
            let Some(hex_entity) = map.entities.get(&coord) else {
                return Err(MyErrors::InconsistentData("Hex Entity not found".to_string()))
            };
            recruit_to_map(treasury, (treasury.gold - 10) / 10, (treasury.gold - 10) / 10 * 10, 
            soldier_asset, me, map, coord, hex_entity.0, commands, soldiers_nums);
            break;
        }
    }

    Ok(())
}

fn move_armies_during_peace(map: &mut HexMap, me: &Entity, occupied: &mut ResMut<Occupied>) -> Result<(), MyErrors> {
    let iterator: Vec<(Hex, i32)> = map.armies.iter()
        .filter(|&x| {
            x.1.owner == *me
        })
        .map(|x| { (*x.0, x.1.move_points_left) })
        .collect();

    for (hex, moves) in &iterator {
        let mut moves_left = *moves;
        let mut coord = *hex;
        while moves_left > 0 {
            let closest = find_closest_unowned_hex(coord, map, me);
            match closest {
                Some(closest_path) => {
                    for (_, pos) in (0 .. moves_left).zip(closest_path) {
                        let new_pos = pos;
                        move_army( &coord, &new_pos, map, occupied, *me)?;
                        coord = new_pos;
                        moves_left -= 1;
                    }
                },
                None => {
                    break;
                },
            }
        }
    }
    Ok(())
}

fn find_closest_unowned_hex(
    start: Hex,
    map: &HexMap,
    me: &Entity
) -> Option<Vec<Hex>> {
    let mut queue = VecDeque::<Hex>::new();
    let mut visited = HashMap::<Hex, Hex>::new();
    
    visited.insert(start, Hex {x: -100000, y: -100000});
    queue.push_back(start);
    
    let mut rng = rand::rng();

    let closest_hex = 'outer: loop {
        let hex = match queue.pop_front() {
            Some(inside) => inside,
            None => break None,
        };
        let mut neighbours = hex.all_neighbors();
        neighbours.shuffle(&mut rng);
        for neighbour in neighbours {
            if closest_unowned_hex_bfs_neighbour_handle(map, neighbour, &mut visited, &mut queue, me, hex) {
                break 'outer Some(neighbour)
            }
        }
    };

    reconstruct_bfs_path(closest_hex, &start, &visited)
}

fn closest_unowned_hex_bfs_neighbour_handle(
    map: &HexMap, neighbour: Hex, visited: &mut HashMap<Hex, Hex>, queue: &mut VecDeque<Hex>, me: &Entity, hex: Hex,
) -> bool {
    if let Some((_, terrain, _)) = map.entities.get(&neighbour) {
                if let TerrainType::Water(_) = terrain {
                    return false;
                }
            } else {
                return false;
            }
            if let Some(owner) = map.hex_owners.get(&neighbour) {
                if owner == me && !visited.contains_key(&neighbour){
                    visited.insert(neighbour, hex);
                    queue.push_back(neighbour);
                }
            } else {
                visited.insert(neighbour, hex);
                return true;
            }

    false
}