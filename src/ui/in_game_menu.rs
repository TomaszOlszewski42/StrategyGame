use bevy::{color::palettes::css::{DARK_GRAY, GRAY}, prelude::*};

use crate::{GameState, OnDemandSystems, main_menu::load_game_menu::ButtonQuery};

#[derive(Component)]
pub struct ExitToMenuButton;

fn exit_to_menu_bundle() -> impl Bundle {(
    DespawnOnExit(GameState::Game),
    Node {
        width: Val::Percent(80.0),
        height: Val::Percent(15.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect { left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Percent(10.0), bottom: Val::Px(0.0) },
        .. default()
    },
    BackgroundColor(Color::Srgba(GRAY)),
    Button,
    ExitToMenuButton,
    children![(
        DespawnOnExit(GameState::Game),
        Text::new("Exit to Menu"),
    )],
)}

#[derive(Component)]
pub struct ContinueGameButton;

fn continue_game_bundle() -> impl Bundle {(
    DespawnOnExit(GameState::Game),
    Node {
        width: Val::Percent(80.0),
        height: Val::Percent(15.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        .. default()
    },
    BackgroundColor(Color::Srgba(GRAY)),
    Button,
    ContinueGameButton,
    children![(
        DespawnOnExit(GameState::Game),
        Text::new("Continue"),
    )],
)}

#[derive(Component)]
pub struct SaveGameButton;

fn save_game_bundle() -> impl Bundle {(
    DespawnOnExit(GameState::Game),
    Node {
        width: Val::Percent(80.0),
        height: Val::Percent(15.0),
        margin: UiRect { left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Percent(10.0), bottom: Val::Px(0.0) },
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        .. default()
    },
    BackgroundColor(Color::Srgba(GRAY)),
    Button,
    SaveGameButton,
    children![(
        DespawnOnExit(GameState::Game),
        Text::new("Save Game"),
    )],
)}

#[derive(Component)]
pub struct GameMenu;

pub fn spawn_in_game_menu(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(GameState::Game),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            .. default()
        },
        GameMenu,
        Visibility::Hidden,
        children![(
            DespawnOnExit(GameState::Game),
            Node {
                width: Val::Percent(30.0),
                height: Val::Percent(50.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                .. default()
            },
            BackgroundColor(Color::srgb(0.2, 0.12, 0.06)),
            children![
                continue_game_bundle(),
                save_game_bundle(),
                exit_to_menu_bundle(),
            ],
        )],
    ));
}

pub fn go_to_game_menu_system(
    pressed: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    menu: Single<(Entity, &Visibility), With<GameMenu>>,
) {
    if !pressed.just_pressed(KeyCode::Escape) {
        return
    }

    match menu.1 {
        Visibility::Inherited => commands.entity(menu.0).insert(Visibility::Hidden),
        Visibility::Hidden => commands.entity(menu.0).insert(Visibility::Visible),
        Visibility::Visible => commands.entity(menu.0).insert(Visibility::Hidden),
    };
}

// bevy button example
pub fn exit_to_menu_button_system(
    mut interactions: Query<ButtonQuery, (With<ExitToMenuButton>, Changed<Interaction>)>,
    mut mouse_button: ResMut<ButtonInput<MouseButton>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in
        &mut interactions
    {
        match interaction {
            Interaction::Pressed => {
                mouse_button.clear_just_pressed(MouseButton::Left);
                game_state.set(GameState::Menu);
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

// bevy button example
pub fn continue_button_system(
    mut interactions: Query<ButtonQuery, (With<ContinueGameButton>, Changed<Interaction>)>,
    mut mouse_button: ResMut<ButtonInput<MouseButton>>,
    menu: Single<Entity, With<GameMenu>>,
    mut commands: Commands,
) {
    for (interaction, mut color) in
        &mut interactions
    {
        match interaction {
            Interaction::Pressed => {
                mouse_button.clear_just_pressed(MouseButton::Left);
                commands.entity(*menu).insert(Visibility::Hidden);
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

// bevy button example
pub fn save_game_button_system(
    mut interactions: Query<ButtonQuery, (With<SaveGameButton>, Changed<Interaction>)>,
    mut mouse_button: ResMut<ButtonInput<MouseButton>>,
    mut commands: Commands,
    systems: Res<OnDemandSystems>,
) {
    for (interaction, mut color) in
        &mut interactions
    {
        match interaction {
            Interaction::Pressed => {
                mouse_button.clear_just_pressed(MouseButton::Left);
                if let Some(systemid) = systems.systems.get("save_world") {
                    commands.run_system(*systemid);
                }
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