use bevy::{prelude::*};
use hexx::Hex;

use crate::{environment::map::HexMap, politics::{countries::AtWarLists, political_map::Occupied}};

pub fn start_war(
    perpetrator: Entity,
    enemy: Entity,
    countries_wars: &mut ResMut<AtWarLists>,
) {
    if perpetrator == enemy {
        return;
    } 

    if let Some(list) = countries_wars.lists.get_mut(&perpetrator)
        && !list.contains(&enemy) {
            list.push(enemy);
        }

    if let Some(list) = countries_wars.lists.get_mut(&enemy)
        && !list.contains(&perpetrator) {
            list.push(perpetrator);
        }
}

pub fn end_war(
    player_1: Entity,
    player_2: Entity,
    countries_wars: &mut ResMut<AtWarLists>,
    map: &ResMut<HexMap>,
    occupied: &mut ResMut<Occupied>,
) {
    if let Some(list) = countries_wars.lists.get_mut(&player_1) {
        for i in 0 .. list.len() {
            if list[i] == player_2 {
                list.swap_remove(i);
                break;
            }
        }
    }
    if let Some(list) = countries_wars.lists.get_mut(&player_2) {
        for i in 0 .. list.len() {
            if list[i] == player_1 {
                list.swap_remove(i);
                break;
            }
        }
    }
    for (hex, entity) in map.hex_owners.iter() {
        if (*entity == player_1 || *entity == player_2)
            && let Some(&owner) = occupied.map.get(hex)
                && (owner == player_1 || owner == player_2) {
                    occupied.map.remove(hex);
                }
    }
}

pub fn end_war_with_empty_countries(
    lists: &mut ResMut<AtWarLists>,
    map: &ResMut<HexMap>,
    countries: &Vec<Entity>,
    occupied: &mut ResMut<Occupied>,
) {
    for country in countries {
        let my_hexes: Vec<Hex> = map.hex_owners.iter()
            .filter(|&x| {
                x.1 == country
            })
            .map(|x| {
                *x.0
            })
            .collect();
        if !my_hexes.is_empty() {
            continue;
        }
        if let Some(list) = lists.lists.clone().get(country) {
            for item in list {
                end_war(*country, *item, lists, map, occupied);
            }
        }
    }
}
