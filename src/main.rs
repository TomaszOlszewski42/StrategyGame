use std::{fs, collections::HashMap};

use bevy::{ecs::{system::SystemId}, input_focus::InputFocus, prelude::*};

use crate::{controls::{camera::{camera_rotation, camera_zoom, move_camera, setup_camera}, new_round::start_new_round}, environment::{buildings::{build_market, redraw_markets}, env_models::{MarketAsset, MountainAsset, TreeAsset, load_models}, map::{HexMap, HighlightedHexes, SelectedHex, highlight_hex, select_hex, setup_map}, misc::setup_missing}, errors::{error_handler::handle_my_errors, my_errors::MyErrors}, main_menu::{load_game_menu::{SavedGamesFiles, actually_lod_button_system, back_to_menu_button_system, item_to_load_system, scrolling_saves_system, spawn_load_game_tab}, main_menu_systems::{exit_game_button_system, load_game_button_system, new_game_button_system}, main_menu_tab::spawn_main_menu, new_game_tab::{back_to_menu_load_tab_button_system, left_arrow_button_system, right_arrow_button_system, spawn_new_game_tab, start_game_button_system}}, military::{army::redraw_armies_models, army_movement::{PossibleMoves, color_player_possible_moves, recolor_old_possible_moves, select_possible_moves}, recruitment::{RecruitNumber, recruitment_system}}, politics::{countries::{CountriesSets, create_countries_sets, setup_countries}, political_map::{Occupied, SmallMesh, change_map_mode, color_occupied_hexes, update_ownership_colors}}, saves::{loading::load_game_state, saveing::{SaveMessage, SerializationHelper, save_world}}, ui::{army_tab::{spawn_army_tab, update_army_tab}, country_tab::spawn_player_country_tabs, country_tab_systems::{build_market_button_system, declare_war_button_system, propose_peace_button_system, recruit_button_system, reduce_number_of_recruits_button_system, show_country_tab_system, up_number_of_recruits_button_system, update_country_tab, update_country_tab_gold, update_recruitment_texts, update_war_flags}, in_game_menu::{continue_button_system, exit_to_menu_button_system, go_to_game_menu_system, save_game_button_system, spawn_in_game_menu}, next_round_button::{create_round_button, next_round_button_system}, saving_window::{saveing_message_window_system, spawn_saveing_message_window}}};


mod controls;
mod environment;
mod politics;
mod ui;
mod military;
mod main_menu;
mod bot_players;
mod saves;
mod errors;

#[derive(Resource)]
struct OnDemandSystems {
    systems: HashMap<String, SystemId>
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Game,
}

fn update_visuals(
    mut commands: Commands,
    systems: Res<OnDemandSystems>,
) {
    match systems.systems.get("redraw_armies_models") {
        Some(id) => commands.run_system(*id),
        None => println!("Eeee nie znalazłem systemu"),
    };

    match systems.systems.get("update_country_tab") {
        Some(id) => commands.run_system(*id),
        None => println!("Eeee nie znalazłem systemu"),
    };

    match systems.systems.get("update_ownership_colors") {
        Some(id) => commands.run_system(*id),
        None => println!("Eeee nie znalazłem systemu"),
    };

    match systems.systems.get("redraw_markets") {
        Some(id) => commands.run_system(*id),
        None => println!("Eeee nie znalazłem systemu"),
    };
}

impl FromWorld for OnDemandSystems {
    fn from_world(world: &mut World) -> Self {
        let mut systems = HashMap::<String, SystemId>::new();
        systems.insert("new_round".to_string(), world.register_system(start_new_round.pipe(handle_my_errors)));
        systems.insert("update_recruitment_texts".to_string(), world.register_system(update_recruitment_texts));
        systems.insert("recruitment".to_string(), world.register_system(recruitment_system.pipe(handle_my_errors)));
        systems.insert("update_country_tab_gold".to_string(), world.register_system(update_country_tab_gold));
        systems.insert("update_country_tab".to_string(), world.register_system(update_country_tab.pipe(handle_my_errors)));
        systems.insert("build_market".to_string(), world.register_system(build_market.pipe(handle_my_errors)));
        systems.insert("update_army_tab".to_string(), world.register_system(update_army_tab));
        systems.insert("redraw_armies_models".to_string(), world.register_system(redraw_armies_models.pipe(handle_my_errors)));
        systems.insert("update_ownership_colors".to_string(), world.register_system(update_ownership_colors.pipe(handle_my_errors)));
        systems.insert("save_world".to_string(), world.register_system(save_world.pipe(handle_my_errors)));
        systems.insert("update_visuals".to_string(), world.register_system(update_visuals));
        systems.insert("redraw_markets".to_string(), world.register_system(redraw_markets));
        systems.insert("update_war_flags".to_string(), world.register_system(update_war_flags.pipe(handle_my_errors)));
        OnDemandSystems { systems }
    }
}

#[derive(Resource, Default)]
pub struct SaveFile {
    path: Option<String>,
}

#[derive(Resource, Default)]
pub struct CountryChoice {
    pub id: usize,
}

fn main() {
    App::new()
        .set_error_handler(bevy::ecs::error::debug)
        .init_resource::<CountryChoice>()
        .init_resource::<SaveFile>()
        .init_resource::<HexMap>()
        .init_resource::<PossibleMoves>()
        .init_resource::<MarketAsset>()
        .init_resource::<SelectedHex>()
        .init_resource::<RecruitNumber>()
        .init_resource::<OnDemandSystems>()
        .init_resource::<InputFocus>()
        .init_resource::<TreeAsset>()
        .init_resource::<MountainAsset>()
        .init_resource::<CountriesSets>()
        .init_resource::<SavedGamesFiles>()
        .init_resource::<HighlightedHexes>()
        .init_resource::<SmallMesh>()
        .init_resource::<Occupied>()
        .init_resource::<SaveMessage>()
        .add_systems(Startup, create_countries_sets)
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .add_plugins(MainMenuPlugin)
        .init_state::<GameState>()
        .add_systems(Startup, setup_camera)
        .run();
}

struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Menu), spawn_main_menu)
            .add_systems(OnEnter(GameState::Menu), spawn_new_game_tab)
            .add_systems(OnEnter(GameState::Menu), spawn_load_game_tab.pipe(handle_my_errors))
            .add_systems(Update, (
                new_game_button_system, 
                exit_game_button_system,
                load_game_button_system,
                start_game_button_system,
                actually_lod_button_system,
                left_arrow_button_system,
                right_arrow_button_system,
                item_to_load_system,
                scrolling_saves_system,
                back_to_menu_button_system,
                back_to_menu_load_tab_button_system,
            )
            .run_if(in_state(GameState::Menu))
        )
    ;}
}

fn setup_game(
    asset_server: Res<AssetServer>, mut tree: ResMut<TreeAsset>, mut mountain: ResMut<MountainAsset>,
    mut market: ResMut<MarketAsset>, mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>, mut map: ResMut<HexMap>,
    mut save: ResMut<SaveFile>, mut sets: ResMut<CountriesSets>, systems: Res<OnDemandSystems>,
    country_choice: Res<CountryChoice>, mut highlighted: ResMut<HighlightedHexes>,
    mut selected: ResMut<SelectedHex>, mut occupied: ResMut<Occupied>,
) -> Result<(), MyErrors> {
    occupied.map = HashMap::new();
    highlighted.pointing_at = None;
    selected.hex = None;
    load_models(&asset_server, &mut tree, &mut mountain, &mut market);
    match save.path.clone() {
        Some(path) => { setup_from_save(&tree, &mountain, &mut commands, &mut meshes, &mut materials, 
            &mut map, &mut save, &mut sets, &mut occupied, &path)?;},
        None => {
            setup_map(&mut commands, &mut meshes, &mut materials, &tree, &mountain, &mut map);
            setup_countries(&mut commands, &mut map, &sets, &country_choice);
        },
    }
    spawn_player_country_tabs(&mut commands, asset_server);
    spawn_army_tab(&mut commands);

    match systems.systems.get("update_visuals") {
        Some(id) => commands.run_system(*id),
        None => return Err(MyErrors::InconsistentData("`update_visuals` not registered".to_string())),
    }

    Ok(())
}

fn setup_from_save(
    tree: &ResMut<TreeAsset>,
    mountain: &ResMut<MountainAsset>,
    mut commands: &mut Commands,    
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    mut map: &mut ResMut<HexMap>,
    save: &mut ResMut<SaveFile>,
    sets: &mut ResMut<CountriesSets>,
    mut occupied: &mut ResMut<Occupied>,
    path: &String,
) -> Result<(), MyErrors> {
    let contents =  match fs::read_to_string(path) {
        Ok(ok) => ok,
        Err(err) => {
            return Err(MyErrors::FileReading(err.to_string()))
        },
    };
    let mut state = match ron::from_str::<SerializationHelper>(&contents) {
        Ok(ok) => ok,
        Err(_) => {
            return Err(MyErrors::Deserialization);
        },
    };
    load_game_state(&mut state, &mut commands, &mut meshes, &mut materials, &tree, &mountain, &mut map, &sets, &mut occupied)?;
    save.path = None;
    Ok(())
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Game),(setup_game.pipe(handle_my_errors), setup_missing, spawn_saveing_message_window).chain())
            .add_systems(OnEnter(GameState::Game), (create_round_button, spawn_in_game_menu).chain())
            .add_systems(Update, (move_camera, camera_rotation, camera_zoom) .run_if(in_state(GameState::Game)))
            .add_systems(Update, change_map_mode.pipe(handle_my_errors).run_if(in_state(GameState::Game)))
            .add_systems(Update, (
                    go_to_game_menu_system, continue_button_system,
                    save_game_button_system, exit_to_menu_button_system, saveing_message_window_system,
                ).run_if(in_state(GameState::Game)))
            .add_systems(Update, (
                up_number_of_recruits_button_system.pipe(handle_my_errors), 
                next_round_button_system.pipe(handle_my_errors),
                reduce_number_of_recruits_button_system.pipe(handle_my_errors),
                recruit_button_system.pipe(handle_my_errors),
                build_market_button_system.pipe(handle_my_errors),
                declare_war_button_system.pipe(handle_my_errors),
                propose_peace_button_system.pipe(handle_my_errors),
                select_hex.pipe(handle_my_errors),
                show_country_tab_system.pipe(handle_my_errors),
                recolor_old_possible_moves.pipe(handle_my_errors),
                select_possible_moves.pipe(handle_my_errors),
                color_player_possible_moves,
                highlight_hex.pipe(handle_my_errors),
                color_occupied_hexes.pipe(handle_my_errors),
            ).chain()
            .run_if(in_state(GameState::Game))
        );
    }
}
