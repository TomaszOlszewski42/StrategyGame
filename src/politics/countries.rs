use bevy::{color::palettes::css::{DARK_SEA_GREEN, GOLDENROD, TOMATO}, prelude::*};
use hexx::Hex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{CountryChoice, GameState, environment::map::{HexMap, TerrainType}};

#[derive(Component, Serialize, Deserialize, Clone, Copy)]
pub enum PlayerType {
    Player,
    Bot,
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Country {
    pub id: usize,
}

impl Country {
    pub fn new(id: usize) -> Country {
        Country { id }
    }
}

#[derive(Component)]
pub struct CountryColor {
    pub material: Handle<StandardMaterial>,
}

#[derive(Component, Clone)]
pub struct Flag {
    pub image: Handle<Image>,
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Treasury {
    pub gold: i32,
    pub base_income: i32,
}

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct AtWarLists {
    pub lists: HashMap<Entity, Vec<Entity>>,
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct PeaceProposals {
    pub senders: Vec<Entity>
}

impl PeaceProposals {
    pub fn new() -> PeaceProposals {
        PeaceProposals { senders: Vec::new() }
    }
}

#[derive(Resource)]
pub struct CountriesSolidersNumbers {
    pub numbers: HashMap<Entity, i32>
}

impl Treasury {
    pub fn new(gold: i32, base_income: i32) -> Treasury {
        Treasury { gold, base_income }
    }

    pub fn update_gold(&mut self, bonus: i32) {
        self.gold += self.base_income + bonus
    }
}

#[derive(Clone, Component)]
pub struct SoldierAsset {
    pub model: Handle<Scene>,
}

#[derive(Resource)]
pub struct CountrySet {
    pub material: Handle<StandardMaterial>,
    pub flag: Flag,
    pub soldier: SoldierAsset,
    pub name: String,
}

#[derive(Resource, Default)]
pub struct CountriesSets {
    pub sets: Vec<CountrySet>,
}

impl CountriesSets {
    pub fn flag(&self, ind: usize) -> Flag {
        self.sets[ind].flag.clone()
    }

    pub fn material(&self, ind: usize) -> Handle<StandardMaterial> {
        self.sets[ind].material.clone()
    }

    pub fn soldier(&self, ind: usize) -> SoldierAsset {
        self.sets[ind].soldier.clone()
    }

    pub fn name(&self, ind: usize) -> String {
        self.sets[ind].name.clone()
    }
}

pub fn create_countries_sets(
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets_server: Res<AssetServer>,
    mut countries_sets: ResMut<CountriesSets>,
) {
    let kingdom1_material = materials.add(Color::Srgba(TOMATO));
    let kingdom2_material = materials.add(Color::Srgba(GOLDENROD));
    let kingdom3_material = materials.add(Color::Srgba(DARK_SEA_GREEN));

    let flag1 = Flag {image: assets_server.load("human_flag.png")};
    let flag2 = Flag {image: assets_server.load("dwarven_flag.png")};
    let flag3 = Flag {image: assets_server.load("elven_flag.png")};

    let soldier1 = SoldierAsset { model: assets_server.load(GltfAssetLabel::Scene(0).from_asset("kostka_orange.glb")) };
    let soldier2 = SoldierAsset { model: assets_server.load(GltfAssetLabel::Scene(0).from_asset("kostka_yellow.glb")) };
    let soldier3 = SoldierAsset { model: assets_server.load(GltfAssetLabel::Scene(0).from_asset("kostka_green.glb")) };

    let name1 = "crown".to_string();
    let name2 = "pickaxe".to_string();
    let name3 = "leaf".to_string();

    let c1 = CountrySet {material: kingdom1_material, flag: flag1, soldier: soldier1, name: name1 };
    let c2 = CountrySet {material: kingdom2_material, flag: flag2, soldier: soldier2, name: name2 };
    let c3 = CountrySet {material: kingdom3_material, flag: flag3, soldier: soldier3, name: name3 };

    countries_sets.sets = vec![c1, c2, c3];
}

pub fn setup_countries(
    commands: &mut Commands,
    map: &mut ResMut<HexMap>,
    countries_sets: &ResMut<CountriesSets>,
    country_choice: &Res<CountryChoice>,
) {
    let ids = spawn_countries_and_get_ids(commands, countries_sets, country_choice);
    let mut soldiers_num = HashMap::<Entity, i32>::new();
    let mut wars = HashMap::<Entity, Vec<Entity>>::new();
    for id in &ids {
        soldiers_num.insert(*id, 0);
        wars.insert(*id, Vec::new());
    }
    commands.insert_resource(CountriesSolidersNumbers {numbers: soldiers_num});
    commands.insert_resource(AtWarLists {lists: wars});
    let hex1 = Hex::new(-5, -5);
    let hex2 = Hex::new(10, -5);
    let hex3 = Hex::new(-5, 10);
    let hexs = [hex1, hex2, hex3];
    for (id, origin) in ids.iter().zip(hexs.iter()) {
        for hex in origin.circular_range(4.0) {
            let Some((_, terrain, _)) = map.entities.get(&hex) else {
                continue;
            };
            if let TerrainType::Water(_) = terrain {
                continue;
            }
            map.hex_owners.insert(hex, *id);
        }
    }
}

fn spawn_countries_and_get_ids(
    commands: &mut Commands,
    countries_sets: &ResMut<CountriesSets>,
    country_choice: &Res<CountryChoice>,
) -> Vec<Entity> {
    let mut ids = Vec::new();
    for i in 0 .. 3 {
        let player_type = if i == country_choice.id {
            PlayerType::Player
        } else {
            PlayerType::Bot
        };

        let id = commands.spawn((
            DespawnOnExit(GameState::Game),
            Country::new(i), CountryColor {material: countries_sets.material(i)}, 
            player_type, countries_sets.flag(i), Treasury::new(70, 10),
            PeaceProposals::new(),
            countries_sets.soldier(i),
        )).id();

        ids.push(id);
    }
    ids
}

pub fn recalculate_soldiers_numbers(
    nums: &mut ResMut<CountriesSolidersNumbers>,
    map: &ResMut<HexMap>,
) {
    let owners: Vec<Entity> = nums.numbers.keys().cloned().collect();

    for owner in owners {
        nums.numbers.insert(owner, 0);
    }

    for army in map.armies.values() {
        let base = match nums.numbers.get(&army.owner) {
            Some(val) => *val,
            None => 0,
        };

        nums.numbers.insert(army.owner, base + army.number_of_soldiers);
    }
}