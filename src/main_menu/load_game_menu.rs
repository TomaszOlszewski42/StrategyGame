use std::fs::{read_dir};

use bevy::{color::palettes::css::{DARK_GRAY, GRAY, GREY, SEA_GREEN}, input::{mouse::AccumulatedMouseScroll}, prelude::*};

use crate::{GameState, SaveFile, errors::my_errors::MyErrors, main_menu::main_menu_tab::MainMenu};

#[derive(Resource, Default)]
pub struct SavedGamesFiles {
    files: Vec<String>,
}

#[derive(Component)]
pub struct LoadGameTab;

pub fn spawn_load_game_tab(
    mut commands: Commands,
    mut files: ResMut<SavedGamesFiles>,
) -> Result<(), MyErrors> {
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
        LoadGameTab,
        Visibility::Hidden,
        BackgroundColor(Color::Srgba(SEA_GREEN)),
        children![
            save_choice_bundle(&mut files)?,
            buttons_bundle()
        ]        
    ));

    Ok(())
}

#[derive(Component)]
pub struct FileNum(usize);

#[derive(Component)]
pub struct Scrollable;

fn save_choice_bundle(files: &mut ResMut<SavedGamesFiles>) -> Result<impl Bundle, MyErrors> {
    let names = read_files()?;
    files.files = names.clone();
    let bundle = 
    (
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Percent(50.0),
            height: Val::Percent(50.0),
            ..default()
        },
        children![files_names_block_bundle(names)]
    );

    Ok(bundle)
}

fn files_names_block_bundle(names: Vec<String>) -> impl Bundle {(
    Node {
        flex_direction: FlexDirection::Column, 
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        padding: UiRect { left: Val::Px(15.0), right: Val::Px(15.0), top: Val::Px(15.0), bottom: Val::Px(15.0) },
        overflow: Overflow::scroll_y(),
        ..default()
    },
    Scrollable,
    BackgroundColor(Color::Srgba(GREY)),
    Children::spawn(SpawnIter((0 .. names.len()).zip(names).map(
        move |(ind, name)| {
            (
            Node {
                height: Val::Px(20.0),
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                .. default()
            },
            FileNum(ind),
            Button,
            children![(
                Text::new(name),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
            )]
        )}
    )))
)}

fn read_files() -> Result<Vec<String>, MyErrors> {
    let mut names = Vec::new();
    let dir = match read_dir("saves") {
        Ok(d) => d,
        Err(_) => return Err(MyErrors::NoSaveFolder),
    };
    for entry in dir {
        match entry {
            Ok(ok) => {
                match ok.file_name().to_str() {
                    Some(ok) => names.push(ok.to_string()),
                    None => return Err(MyErrors::Unexpected("Couldn't create str from file name".to_string())),
                }
            },
            Err(err) => println!("{}", err),
        }
    }
    names.reverse();
    Ok(names)
}

fn buttons_bundle() -> impl Bundle {(
    Node {
        width: Val::Percent(50.0),
        height: Val::Percent(20.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        .. default()
    },
    children![
        back_to_menu_button(),
        actually_load_button(),
    ]
)}

#[derive(Component)]
pub struct BackToMenuButton;

fn back_to_menu_button() -> impl  Bundle {(
    Node {
        width: Val::Percent(30.0),
        height: Val::Percent(50.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect { left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Percent(2.0), bottom: Val::Px(0.0) },
        ..default()
    },
    Button,
    BackToMenuButton,
    BackgroundColor(Color::Srgba(GRAY)),
    children![(
        DespawnOnExit(GameState::Game),
        Text::new("Back To Menu"),
    )],
)}

#[derive(Component)]
pub struct ActuallyLoadButton;

fn actually_load_button() -> impl Bundle {(
    Node {
        width: Val::Percent(30.0),
        height: Val::Percent(50.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect { left: Val::Px(30.0), right: Val::Px(0.0), top: Val::Percent(2.0), bottom: Val::Px(0.0) },
        ..default()
    },
    Button,
    ActuallyLoadButton,
    BackgroundColor(Color::Srgba(GRAY)),
    children![
        DespawnOnExit(GameState::Game),
        Text::new("Load"),
    ],
)}


pub fn item_to_load_system(
    mut interactions: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &FileNum,
        ),
        Changed<Interaction>,
    >,
    mut mouse_button: ResMut<ButtonInput<MouseButton>>,
    mut save_file: ResMut<SaveFile>,
    files: Res<SavedGamesFiles>,
) {
    for (interaction, mut color, file_num) in
        &mut interactions
    {
        match interaction {
            Interaction::Pressed => {
                mouse_button.clear_just_pressed(MouseButton::Left);
                save_file.path = Some(format!("saves/{}", files.files[file_num.0]));
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

pub fn actually_lod_button_system(
    mut interactions: Query<ButtonQuery, (With<ActuallyLoadButton>, Changed<Interaction>)>,
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

pub fn scrolling_saves_system(
    scroll_motion: Res<AccumulatedMouseScroll>,
    mut node: Single<(&mut ScrollPosition, &ComputedNode), With<Scrollable>>,
) {
    let mut max_offset = (node.1.content_size() - node.1.size()) * node.1.inverse_scale_factor();
    if max_offset.y < 0.0 {
        max_offset.y = 0.0;
    }
    let delta_zoom = -scroll_motion.delta.y * 10.0;
    node.0.y += delta_zoom;
    node.0.y = node.0.y.clamp(0.0, max_offset.y);
}

pub type ButtonQuery<'a> = (&'a Interaction, &'a mut BackgroundColor);

pub fn back_to_menu_button_system(
    mut interactions: Query<ButtonQuery, (With<BackToMenuButton>, Changed<Interaction>)>,
    mut mouse_button: ResMut<ButtonInput<MouseButton>>,
    main_menu: Single<Entity, With<MainMenu>>,
    loading_tab: Single<Entity, With<LoadGameTab>>,
    mut commands: Commands,
    mut save_file: ResMut<SaveFile>,
) {
    for (interaction, mut color) in
        &mut interactions
    {
        match interaction {
            Interaction::Pressed => {
                mouse_button.clear_just_pressed(MouseButton::Left);
                commands.entity(*main_menu).insert(Visibility::Visible);
                commands.entity(*loading_tab).insert(Visibility::Hidden);
                save_file.path = None;
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
