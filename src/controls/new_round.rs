use bevy::prelude::*;
use hexx::Hex;
use crate::{OnDemandSystems, bot_players::bots_system::run_bots, environment::{env_models::MarketAsset, map::HexMap}, errors::my_errors::MyErrors, politics::{countries::{AtWarLists, CountriesSolidersNumbers, Country, PeaceProposals, PlayerType, SoldierAsset, Treasury, recalculate_soldiers_numbers}, political_map::Occupied, war::end_war_with_empty_countries}};

pub fn start_new_round(
    mut map: ResMut<HexMap>,
    systems: Res<OnDemandSystems>,
    mut commands: Commands,
    mut players: Query<(Entity, &PlayerType, &mut PeaceProposals, &mut Treasury, &SoldierAsset), With<Country>>,
    mut countries_wars: ResMut<AtWarLists>,
    market_model: Res<MarketAsset>,
    mut occupied: ResMut<Occupied>,
    mut soldiers_nums: ResMut<CountriesSolidersNumbers>,
) -> Result<(), MyErrors> {
    recalculate_soldiers_numbers(&mut soldiers_nums, &map);
    run_bots(&mut players, &mut soldiers_nums, &mut countries_wars, &mut map, &mut commands, market_model, &mut occupied)?;
    let mut countries= Vec::new();
    for player in &players {
        countries.push(player.0);
    }
    
    end_war_with_empty_countries(&mut countries_wars, &map, &countries, &mut occupied);

    for army in map.armies.values_mut() {
        army.reset_move_points();
    }
    update_countries_gold(players, &map, occupied)?;
    run_systems(&mut commands, &systems)?;
    Ok(())
}

fn run_systems(commands: &mut Commands, systems: &Res<OnDemandSystems>) -> Result<(), MyErrors> {
    let Some(systemid) = systems.systems.get("update_war_flags") else {
        return Err(MyErrors::InconsistentData("OnDemandSystems".to_string()))
    };
    commands.run_system(*systemid);

    let Some(systemid) = systems.systems.get("update_country_tab_gold") else {
        return Err(MyErrors::InconsistentData("OnDemandSystems".to_string()))
    };
    commands.run_system(*systemid);
    
    let Some(systemid) = systems.systems.get("redraw_armies_models") else {
        return Err(MyErrors::InconsistentData("OnDemandSystems".to_string()))
    };
    commands.run_system(*systemid);
    
    let Some(systemid) = systems.systems.get("update_ownership_colors") else {
        return Err(MyErrors::InconsistentData("OnDemandSystems".to_string()))
    };
    commands.run_system(*systemid);

    Ok(())
}

fn update_countries_gold(
    players: Query<(Entity, &PlayerType, &mut PeaceProposals, &mut Treasury, &SoldierAsset), With<Country>>,
    map: &HexMap,
    occupied: ResMut<Occupied>,
) -> Result<(), MyErrors> {
    for (entity, _, _, mut treasury, _) in players {
        let my_hexes: Vec<Hex> = map.hex_owners.iter()
            .filter(|&x| {
                *x.1 == entity && !occupied.map.contains_key(x.0)
            })
            .map(|x| {
                *x.0
            })
            .collect();
        let mut bonus = 0;
        for hex in my_hexes {
            if let Some((_, _, buildings)) = map.entities.get(&hex) {
                bonus += buildings.bonus_gold();
            } else {
                return Err(MyErrors::InconsistentData("Hex Entity not found".to_string()))
            }
        }
        treasury.update_gold(bonus);
    }

    Ok(())
} 