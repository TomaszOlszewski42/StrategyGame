use std::{collections::HashMap, fs::File, sync::{Arc, Mutex}};

use bevy::{prelude::*, tasks::IoTaskPool};
use hexx::Hex;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};

use crate::{environment::map::HexMap, errors::my_errors::MyErrors, politics::{countries::{AtWarLists, CountriesSets, Country, PeaceProposals, PlayerType, Treasury}, political_map::Occupied}};

#[derive(Serialize, Deserialize)]
pub struct SerializationHelper {
    pub hex_map: HexMap,
    pub war_list: AtWarLists,
    pub countries: Vec<SerializedCountry>,
    pub occupied: HashMap<Hex, Entity>,
}

#[derive(Serialize, Deserialize)]
pub struct SerializedCountry {
    pub entity: Entity,
    pub id: Country,
    pub treasury: Treasury,
    pub player_type: PlayerType,
    pub peace_proposals: PeaceProposals,
}

impl SerializedCountry {
    fn new(entity: Entity,
    id: Country,
    treasury: Treasury,
    player_type: PlayerType,
    peace_proposals: PeaceProposals,) -> SerializedCountry {
        SerializedCountry { entity, id, treasury, player_type, peace_proposals }
    }
}

#[derive(Resource, Default)]
pub struct SaveMessage(pub Arc<Mutex<Option<String>>>);

pub fn save_world(
    map: Res<HexMap>,
    war_list: Res<AtWarLists>,
    countries: Query<(Entity, &Country, &Treasury, &PlayerType, &PeaceProposals)>,
    sets: Res<CountriesSets>,
    occupied: Res<Occupied>,
    result: Res<SaveMessage>,
) -> Result<(), MyErrors> {
    let (serialized_struct ,player_name , player_gold) = 
        create_serialization_struct_and_player_info(&map, &war_list, &sets, &occupied, &countries);

    let Ok(serialized) = ron::to_string(&serialized_struct) else {
        return Err(MyErrors::Serialization);
    };

    let time = Local::now();
    let player_gold = player_gold.unwrap_or(-42);
    let player_name = match player_name {
        Some(ok) => ok,
        None => "name_error".to_string(),
    };

    let cloned = result.0.clone();
    detach_task(cloned, time, player_name, player_gold, serialized);
    Ok(())
}

fn create_serialization_struct_and_player_info(
    map: &Res<HexMap>, war_list: &Res<AtWarLists>, sets: &Res<CountriesSets>, occupied: &Res<Occupied>,
    countries: &Query<(Entity, &Country, &Treasury, &PlayerType, &PeaceProposals)>,
) -> (SerializationHelper, Option<String>, Option<i32>) {
    let mut player_name = None;
    let mut player_gold = None;
    let mut serialized_countries = Vec::new();
    for (entity, country, treasury, player_type, peace_proposals) in countries {
        match player_type {
            PlayerType::Player => {
                player_gold = Some(treasury.gold);
                player_name = Some(sets.name(country.id));
            },
            PlayerType::Bot => {},
        }
        let country = SerializedCountry::new(entity, country.clone(), treasury.clone(), player_type.clone(), peace_proposals.clone());
        serialized_countries.push(country);
    } 
    (SerializationHelper { hex_map: (*map).clone(), war_list: (*war_list).clone(), 
        countries: serialized_countries, occupied: occupied.map.clone()}, player_name, player_gold)
}

fn detach_task(cloned: Arc<Mutex<Option<String>>>, time: DateTime<Local>, player_name: String, 
    player_gold: i32, serialized: String
) {
    #[cfg(not(target_arch = "wasm32"))]
    IoTaskPool::get()
        .spawn(async move {
            use std::{fs, io::Write};
            _ = fs::create_dir("./saves"); // ingoruję błąd bo błąd występuje również gdy folder istnieje co jest pożądane
            let res = File::create(format!("saves/[{}]-{}-{}G.txt", time, player_name, player_gold))
                .and_then(|mut file| {
                    match file.write_all(serialized.as_bytes()) {
                        Ok(_) => Ok(()),
                        Err(err) => Err(err),
                    }
                });

            let msg = match res {
                Ok(_) => "Zapisano do pliku".to_string(),
                Err(e) => e.to_string(),
            };

            match cloned.lock() {
                Ok(mut guard) => { *guard = Some(msg) },
                Err(poison) => {
                    let mut guard = poison.into_inner();
                    *guard = Some(msg)
                },
            }
        })
        .detach();
}