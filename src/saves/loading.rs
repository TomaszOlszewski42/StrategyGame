use bevy::{color::palettes::css::{BLUE, GREEN, GREY, LIME, RED, SLATE_GREY, WHITE, YELLOW}, prelude::*};
use std::collections::HashMap;
use hexx::Hex;

use crate::{GameState, environment::{buildings::BuildingsAggregate, env_models::{MountainAsset, TreeAsset}, map::{HexMap, TerrainType, create_hexagonal_mesh, spawn_hex_entity}}, errors::my_errors::MyErrors, politics::{countries::{AtWarLists, CountriesSets, CountriesSolidersNumbers, CountryColor, PlayerType}, political_map::Occupied}, saves::saveing::{SerializationHelper, SerializedCountry}};

pub fn load_game_state(
    state: &mut SerializationHelper,
    commands: &mut Commands,    
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    tree_model: &ResMut<TreeAsset>,
    mountain_model: &ResMut<MountainAsset>,
    hex_map: &mut ResMut<HexMap>,
    countries_sets: &ResMut<CountriesSets>,
    occupied: &mut ResMut<Occupied>,
) -> Result<(), MyErrors> {
    set_easy_fields(hex_map, state, materials);
    let mut country_id_map = HashMap::<Entity, usize>::new();
    let mut entity_id_map = HashMap::<Entity, Entity>::new();
    spawn_countries(state, commands, countries_sets, &mut country_id_map, &mut entity_id_map);
    recreate_war_list(state, commands, &mut entity_id_map)?;
    reacreate_hex_owners(state, hex_map, &mut entity_id_map)?;
    hex_map.entities = reacreate_hex_entities_hash_map(state, commands, meshes, materials, tree_model, mountain_model, hex_map);
    hex_map.armies = HashMap::new();
    let soldiers_num = recount_soldiers(state, hex_map, countries_sets, &entity_id_map, &country_id_map)?;
    commands.insert_resource(CountriesSolidersNumbers {numbers: soldiers_num});

    for (hex, owner) in &state.occupied {
        let new_id = match entity_id_map.get(owner) {
            Some(id) => id,
            None => return Err(MyErrors::InconsistentData("Couldn't map entity from old id to new".to_string())),
        };
        occupied.map.insert(*hex, *new_id);
    }
    Ok(())
}

fn set_easy_fields(
    hex_map: &mut ResMut<HexMap>,    
    state: &mut SerializationHelper,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    hex_map.political = state.hex_map.political;
    hex_map.layout = state.hex_map.layout.clone();
    hex_map.highlighted_material = materials.add(Color::Srgba(WHITE));
    hex_map.no_owner_material = materials.add(Color::Srgba(GREY));
    hex_map.selected_material = materials.add(Color::Srgba(YELLOW));
    hex_map.possible_moves_material = materials.add(Color::Srgba(RED));
}

fn spawn_countries(
    state: &mut SerializationHelper,
    commands: &mut Commands,
    countries_sets: &ResMut<CountriesSets>,
    mut country_id_map: &mut HashMap::<Entity, usize>,
    mut entity_id_map: &mut HashMap::<Entity, Entity>,
) {
    for country in &state.countries {
        match country.player_type {
            PlayerType::Player => spawn_player(commands, countries_sets, &mut country_id_map, &mut entity_id_map, country),
            PlayerType::Bot => spawn_bot(commands, countries_sets, &mut country_id_map, &mut entity_id_map, country),
        }
    }
}

fn spawn_bot(
    commands: &mut Commands,
    countries_sets: &ResMut<CountriesSets>,
    country_id_map: &mut HashMap::<Entity, usize>,
    entity_id_map: &mut HashMap::<Entity, Entity>,
    country: &SerializedCountry
) {
    let id = commands.spawn((
        DespawnOnExit(GameState::Game),
        country.id.clone(),
        CountryColor {material: countries_sets.material(country.id.id).clone()},
        PlayerType::Bot,
        countries_sets.flag(country.id.id).clone(),
        country.treasury.clone(),
        country.peace_proposals.clone(),
        countries_sets.soldier(country.id.id).clone(),
    )).id();
    entity_id_map.insert(country.entity, id);
    country_id_map.insert(country.entity, country.id.id);
}

fn spawn_player(
    commands: &mut Commands,
    countries_sets: &ResMut<CountriesSets>,
    country_id_map: &mut HashMap::<Entity, usize>,
    entity_id_map: &mut HashMap::<Entity, Entity>,
    country: &SerializedCountry
) {
    let id = commands.spawn((
        DespawnOnExit(GameState::Game),
        country.id.clone(),
        CountryColor {material: countries_sets.material(country.id.id).clone()},
        PlayerType::Player,
        countries_sets.flag(country.id.id).clone(),
        country.treasury.clone(),
        country.peace_proposals.clone(),
        countries_sets.soldier(country.id.id).clone(),
    )).id();
    entity_id_map.insert(country.entity, id);
    country_id_map.insert(country.entity, country.id.id);
}

fn recreate_war_list(
    state: &mut SerializationHelper,
    commands: &mut Commands,
    entity_id_map: &mut HashMap::<Entity, Entity>,
) -> Result<(), MyErrors> {
    let mut wars = HashMap::<Entity, Vec<Entity>>::new();
    for (entity, vec) in &state.war_list.lists {
        let new_entity = match entity_id_map.get(entity) {
            Some(e) => e,
            None => return Err(MyErrors::InconsistentData("Couldn't map entity from old id to new".to_string())),
        };
        let mut enemies = Vec::new();
        for item in vec {
            let mapped = match entity_id_map.get(item) {
                Some(e) => e,
                None => return Err(MyErrors::InconsistentData("Couldn't map entity from old id to new".to_string())),
            };
            enemies.push(*mapped);
        }
        wars.insert(*new_entity, enemies);
    }
    commands.insert_resource(AtWarLists {lists: wars});
    Ok(())
}

fn reacreate_hex_owners(
    state: &mut SerializationHelper,
    hex_map: &mut ResMut<HexMap>,
    entity_id_map: &mut HashMap::<Entity, Entity>,
) -> Result<(), MyErrors> {
    for (hex, id) in &state.hex_map.hex_owners {
        let new_id = match entity_id_map.get(id) {
            Some(e) => e,
            None => return Err(MyErrors::InconsistentData("Couldn't map entity from old id to new".to_string())),
        };
        hex_map.hex_owners.insert(*hex, *new_id);
    }

    Ok(())
}

fn reacreate_hex_entities_hash_map(
    state: &mut SerializationHelper, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>, tree_model: &ResMut<TreeAsset>,
    mountain_model: &ResMut<MountainAsset>, hex_map: &mut ResMut<HexMap>,
) -> HashMap<Hex, (Entity, TerrainType, BuildingsAggregate)> {
    let material_water = materials.add(Color::Srgba(BLUE));
    let material_grass = materials.add(Color::Srgba(LIME));
    let material_forest = materials.add(Color::Srgba(GREEN));
    let material_mountains = materials.add(Color::Srgba(SLATE_GREY));
    let mesh = create_hexagonal_mesh(&hex_map.layout);
    let mesh_handle = meshes.add(mesh);
    let water = TerrainType::Water(material_water.clone());
    let grass = TerrainType::Flat(material_grass.clone());
    let mountains = TerrainType::Mountain(material_mountains.clone());
    let forest = TerrainType::Forest(material_forest.clone());
    let mut hex_entities = HashMap::<Hex, (Entity, TerrainType, BuildingsAggregate)>::new();
    for (hex, (_, terrain, buildings)) in &state.hex_map.entities {
        let terr = match terrain {
            TerrainType::Flat(_) => (grass.clone(), material_grass.clone()),
            TerrainType::Forest(_) => (forest.clone(), material_forest.clone()),
            TerrainType::Mountain(_) => (mountains.clone(), material_mountains.clone()),
            TerrainType::Water(_) => (water.clone(), material_water.clone()),
        };
        
        let id = spawn_hex_entity(commands, &hex_map.layout, hex, &mesh_handle, terr.1, &terr.0, 
            tree_model, mountain_model);
        hex_entities.insert(*hex, (id, terr.0, buildings.clone()));
    };
    hex_entities
}

fn recount_soldiers(
    state: &mut SerializationHelper,
    hex_map: &mut ResMut<HexMap>,
    countries_sets: &ResMut<CountriesSets>,
    entity_id_map: &HashMap::<Entity, Entity>,
    country_id_map: &HashMap::<Entity, usize>,
) -> Result<HashMap<Entity, i32>, MyErrors> {
    let mut soldiers_num = HashMap::<Entity, i32>::new();
    for id in entity_id_map.values() {
        soldiers_num.insert(*id, 0);
    }
    for (hex, army) in &state.hex_map.armies {
        let mut new_army = army.clone();
        let country_id = match country_id_map.get(&army.owner) {
            Some(id) => id,
            None => return Err(MyErrors::InconsistentData("Couldn't map entity to country id".to_string())),
        };
        new_army.model = countries_sets.soldier(*country_id).model.clone();
        let new_owner = match entity_id_map.get(&army.owner) {
            Some(e) => *e,
            None => return Err(MyErrors::InconsistentData("Couldn't map entity from old id to new".to_string())),
        };
        new_army.owner = new_owner;
        hex_map.armies.insert(*hex, new_army);

        if let Some(some) = soldiers_num.insert(new_owner, army.number_of_soldiers) {
            soldiers_num.insert(new_owner, some + army.number_of_soldiers);
        }
    }
    Ok(soldiers_num)
}