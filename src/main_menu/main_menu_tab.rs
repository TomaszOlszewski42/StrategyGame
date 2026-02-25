use bevy::{color::palettes::css::{GRAY, SEA_GREEN}, prelude::*};

use crate::GameState;

#[derive(Component)]
pub struct MainMenu;

pub fn spawn_main_menu(
    mut commands: Commands,
) {
    commands.spawn((
        DespawnOnExit(GameState::Menu),
        MainMenu,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            .. default()
        },
        BackgroundColor(Color::Srgba(SEA_GREEN)),
        children![
            new_game_button_bundle(),
            load_game_button_bundle(),
            exit_game_button_bundle(),
        ],
    ));
}

#[derive(Component)]
pub struct NewGameButton;

fn new_game_button_bundle() -> impl Bundle {(
    Node {
        width: Val::Percent(20.0),
        height: Val::Percent(10.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    },
    Button,
    NewGameButton,
    BackgroundColor(Color::Srgba(GRAY)),
    children![(
        DespawnOnExit(GameState::Game),
        Text::new("New Game"),
    )],
)}

#[derive(Component)]
pub struct LoadGameButton;

fn load_game_button_bundle() -> impl Bundle {(
    DespawnOnExit(GameState::Game),
    Node {
        width: Val::Percent(20.0),
        height: Val::Percent(10.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect { left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Percent(2.0), bottom: Val::Px(0.0) },
        ..default()
    },
    Button,
    LoadGameButton,
    BackgroundColor(Color::Srgba(GRAY)),
    children![(
        DespawnOnExit(GameState::Game),
        Text::new("Load Game"),
    )],
)}

#[derive(Component)]
pub struct ExitGameButton;

fn exit_game_button_bundle() -> impl Bundle {(
    DespawnOnExit(GameState::Menu),
    Node {
        width: Val::Percent(20.0),
        height: Val::Percent(10.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect { left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Percent(2.0), bottom: Val::Px(0.0) },
        ..default()
    },
    Button,
    ExitGameButton,
    BackgroundColor(Color::Srgba(GRAY)),
    children![(
        DespawnOnExit(GameState::Game),
        Text::new("Exit"),
    )],
)}

