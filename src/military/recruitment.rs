use bevy::prelude::*;
use hexx::Hex;

use crate::{GameState, OnDemandSystems, environment::map::{HexMap, SelectedHex}, errors::my_errors::MyErrors, military::army::Army, politics::{countries::{CountriesSolidersNumbers, SoldierAsset, Treasury}, political_map::Occupied}};

#[derive(Resource, Default)]
pub struct RecruitNumber {
    pub number: i32,
    pub price: i32,
}

#[derive(Component)]
pub struct ArmyModel;

impl RecruitNumber {
    pub fn up_number(&mut self, up: i32) {
        self.number += up;
        if self.number < 0 {
            self.number = 0;
        }
        self.price = self.number * 10;
    }

    pub fn reset(&mut self) {
        self.number = 0;
        self.price = 0;
    }
}

pub fn recruitment_system(
    selected_hex: Res<SelectedHex>,
    mut map: ResMut<HexMap>,
    mut commands: Commands,
    mut recruit_num: ResMut<RecruitNumber>,
    mut countries: Query<(&mut Treasury, &SoldierAsset)>,
    systems: Res<OnDemandSystems>,
    mut numbers: ResMut<CountriesSolidersNumbers>,
    occupied: Res<Occupied>,
) -> Result<(), MyErrors> {
    if recruit_num.number < 1 {
        return Ok(())
    }
    
    do_check_and_recruit(selected_hex, &mut map, &mut commands, &mut recruit_num, &mut countries, &mut numbers, &occupied)?;

    recruit_num.reset();

    run_systems_after_recruiting(&mut commands, &systems)
}

fn do_check_and_recruit(selected_hex: Res<SelectedHex>,
    map: &mut ResMut<HexMap>,
    commands: &mut Commands,
    recruit_num: &mut ResMut<RecruitNumber>,
    countries: &mut Query<(&mut Treasury, &SoldierAsset)>,
    numbers: &mut ResMut<CountriesSolidersNumbers>,
    occupied: &Res<Occupied>
) -> Result<(), MyErrors> {
    let Some(coord) = selected_hex.hex else {
        return Err(MyErrors::InconsistentData("Recruiting army but no selected hex".to_string()))
    };
    if occupied.map.contains_key(&coord) {
        return Ok(())
    }
    let Some((hex_entity, _, _)) = map.entities.get(&coord) else {
        return Err(MyErrors::InconsistentData("Couldn't find Hex Entity".to_string()))
    };
    let hex_entity = *hex_entity;
    let Some(&owner) = map.hex_owners.get(&coord) else {
        return Err(MyErrors::InconsistentData("Recruiting army but no owner of tile".to_string()))
    };
    let Ok((mut treasury, soldier)) = countries.get_mut(owner) else {
        return Err(MyErrors::InconsistentData("Couldn't find treasury or soldier asset of country".to_string()))
    };
    if treasury.gold < recruit_num.price {
        return Ok(())
    };
    recruit_to_map(&mut treasury, recruit_num.number, recruit_num.price, soldier, 
        &owner, map, coord, hex_entity, commands, numbers);
    Ok(())
}

fn run_systems_after_recruiting(commands: &mut Commands, systems: &Res<OnDemandSystems>) -> Result<(), MyErrors> {
    let Some(systemid) = systems.systems.get("update_country_tab") else {
        return Err(MyErrors::InconsistentData("No `update_country_tab` registered".to_string()))
    };
    commands.run_system(*systemid);

    let Some(systemid) = systems.systems.get("redraw_armies_models") else {
        return Err(MyErrors::InconsistentData("No `redraw_armies_models` registered".to_string()))
    };
    commands.run_system(*systemid);
    Ok(())
}

pub fn recruit_to_map(
    treasury: &mut Treasury,
    soldiers_num: i32,
    price: i32,
    soldier_asset: &SoldierAsset,
    owner: &Entity,
    map: &mut HexMap,
    coord: Hex,
    hex_entity: Entity,
    commands: &mut Commands,
    numbers: &mut ResMut<CountriesSolidersNumbers>,
) {
    let to_insert = match numbers.numbers.get(owner) {
        Some(some) => *some + soldiers_num,
        None => soldiers_num,
    };
    numbers.numbers.insert(*owner, to_insert);
    treasury.gold -= price;
    let mut new_army = Army::new(soldiers_num, soldier_asset.model.clone(), 
        *owner, 0, 2);
    if let Some(old_army) = map.armies.get(&coord) {
        new_army.merge_with_other_ref(old_army);
    };
    map.armies.insert(coord, new_army);
    let child_id = commands.spawn((
        DespawnOnExit(GameState::Game), SceneRoot(soldier_asset.model.clone()),
        Transform::from_xyz(0.0,0.1, 0.0).with_scale(Vec3::new(0.05, 0.1, 0.05)), 
        ArmyModel,
    )).id();
    commands.entity(hex_entity).add_child(child_id);
}