use bevy::{color::palettes::css::{DARK_GRAY, GRAY, SEA_GREEN}, prelude::*};

use crate::{CountryChoice, GameState, main_menu::{load_game_menu::ButtonQuery, main_menu_tab::MainMenu}, politics::countries::CountriesSets};

#[derive(Component)]
pub struct NewGameTab;

pub fn spawn_new_game_tab(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let arrow_font: Handle<Font> = asset_server.load("Montserrat-Regular.ttf");

    commands.spawn((
        DespawnOnExit(GameState::Menu),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            .. default()
        },
        NewGameTab,
        Visibility::Hidden,
        BackgroundColor(Color::Srgba(SEA_GREEN)),
        children![
            flag_bundle(),
            arrows_bundle(&arrow_font),
            buttons_bundle(),
        ]        
    ));
}

fn buttons_bundle() -> impl Bundle {(
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(10.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        .. default()
    },
    children![
        back_to_menu_new_game_bundle(),
        start_game_button(),
    ]
)}

#[derive(Component)]
pub struct ChoiceFlag;

fn flag_bundle() -> impl Bundle {(
    Node {
        width: Val::Percent(50.0),
        height: Val::Percent(50.0),
        .. default()
    },
    ImageNode {
        .. default()
    },
    ChoiceFlag,
)}

#[derive(Component)]
pub struct LeftArrowButton;

fn arrow_left_bundle(font: &Handle<Font>) -> impl Bundle {(
    Node {
        width: Val::Percent(20.0),
        height: Val::Percent(80.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect { left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Percent(2.0), bottom: Val::Px(0.0) },
        ..default()
    },
    Button,
    BackgroundColor(Color::Srgba(GRAY)),
    LeftArrowButton,
    children![(
        DespawnOnExit(GameState::Game),
        Text::new("←"),
        TextFont {
            font: font.clone(),
            font_size: 30.0,
            ..default()
        }
    )],
)}

#[derive(Component)]
pub struct RightArrowButton;

fn arrow_right_bundle(font: &Handle<Font>) -> impl Bundle {(
    Node {
        width: Val::Percent(20.0),
        height: Val::Percent(80.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect { left: Val::Px(10.0), right: Val::Px(0.0), top: Val::Percent(2.0), bottom: Val::Px(0.0) },
        ..default()
    },
    Button,
    RightArrowButton,
    BackgroundColor(Color::Srgba(GRAY)),
    children![(
        DespawnOnExit(GameState::Game),
        Text::new("→"),
        TextFont {
            font: font.clone(),
            font_size: 30.0,
            ..default()
        }
    )],
)}

//←→
fn arrows_bundle(font: &Handle<Font>) -> impl Bundle {(
    Node {
        width: Val::Percent(30.0),
        height: Val::Percent(10.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    },
    children![
        arrow_left_bundle(font),
        arrow_right_bundle(font),
    ]
)}

#[derive(Component)] 
pub struct StartGameButton;

fn start_game_button() -> impl Bundle {(
    Node {
        width: Val::Percent(20.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect { left: Val::Px(10.0), right: Val::Px(0.0), top: Val::Percent(0.5), bottom: Val::Px(0.0) },
        ..default()
    },
    Visibility::Inherited,
    Button,
    StartGameButton,
    BackgroundColor(Color::Srgba(GRAY)),
    children![
        DespawnOnExit(GameState::Game),
        Text::new("Start Game"),
    ],
)}

pub fn start_game_button_system(
    mut interactions: Query<
        ButtonQuery,
        (With<StartGameButton>, Changed<Interaction>),
    >,
    mut mouse_button: ResMut<ButtonInput<MouseButton>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in
        &mut interactions
    {
        match interaction {
            Interaction::Pressed => {
                mouse_button.clear_just_pressed(MouseButton::Left);
                game_state.set(GameState::Game);
            }
            Interaction::Hovered => {
                *color = Color::Srgba(DARK_GRAY).into();
            }
            Interaction::None => {
                *color = Color::Srgba(GRAY).into();
            }
        }
    }
}

pub fn left_arrow_button_system(
    mut interactions: Query<ButtonQuery, (With<LeftArrowButton>, Changed<Interaction>)>,
    mut mouse_button: ResMut<ButtonInput<MouseButton>>,
    mut image: Single<&mut ImageNode, With<ChoiceFlag>>,
    mut country_choice: ResMut<CountryChoice>,
    sets: Res<CountriesSets>,
) {
    for (interaction, mut color) in
        &mut interactions
    {
        match interaction {
            Interaction::Pressed => {
                mouse_button.clear_just_pressed(MouseButton::Left);
                if country_choice.id == 0 {
                    country_choice.id = 2;
                } else {
                    country_choice.id -= 1;
                }
                image.image = sets.flag(country_choice.id).image.clone();
            }
            Interaction::Hovered => {
                *color = Color::Srgba(DARK_GRAY).into();
            }
            Interaction::None => {
                *color = Color::Srgba(GRAY).into();
            }
        }
    }
}

pub fn right_arrow_button_system(
    mut interactions: Query<ButtonQuery, (With<RightArrowButton>, Changed<Interaction>)>,
    mut mouse_button: ResMut<ButtonInput<MouseButton>>,
    mut image: Single<&mut ImageNode, With<ChoiceFlag>>,
    mut country_choice: ResMut<CountryChoice>,
    sets: Res<CountriesSets>,
) {
    for (interaction, mut color) in
        &mut interactions
    {
        match interaction {
            Interaction::Pressed => {
                mouse_button.clear_just_pressed(MouseButton::Left);
                country_choice.id += 1;
                country_choice.id %= 3;
                image.image = sets.flag(country_choice.id).image.clone();
            }
            Interaction::Hovered => {
                *color = Color::Srgba(DARK_GRAY).into();
            }
            Interaction::None => {
                *color = Color::Srgba(GRAY).into();
            }
        }
    }
}


#[derive(Component)]
pub struct NewGameTabBackToMenuButton;

fn back_to_menu_new_game_bundle() -> impl Bundle {(
    Node {
        width: Val::Percent(20.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect { left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Percent(0.5), bottom: Val::Px(0.0) },
        ..default()
    },
    Button,
    NewGameTabBackToMenuButton,
    BackgroundColor(Color::Srgba(GRAY)),
    children![(
        DespawnOnExit(GameState::Game),
        Text::new("Back To Menu"),
    )],
)}

pub fn back_to_menu_load_tab_button_system(
    mut interactions: Query<ButtonQuery, (With<NewGameTabBackToMenuButton>, Changed<Interaction>)>,
    mut mouse_button: ResMut<ButtonInput<MouseButton>>,
    main_menu: Single<Entity, With<MainMenu>>,
    new_game_tab: Single<Entity, With<NewGameTab>>,
    mut commands: Commands,
) {
    for (interaction, mut color) in
        &mut interactions
    {
        match interaction {
            Interaction::Pressed => {
                mouse_button.clear_just_pressed(MouseButton::Left);
                commands.entity(*main_menu).insert(Visibility::Visible);
                commands.entity(*new_game_tab).insert(Visibility::Hidden);
            }
            Interaction::Hovered => {
                *color = Color::Srgba(DARK_GRAY).into();
            }
            Interaction::None => {
                *color = Color::Srgba(GRAY).into();
            }
        }
    }
}