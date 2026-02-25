use bevy::{color::palettes::css::{DARK_GRAY, GRAY}, prelude::*};

use crate::{CountryChoice, main_menu::{load_game_menu::{ButtonQuery, LoadGameTab}, main_menu_tab::{ExitGameButton, LoadGameButton, MainMenu, NewGameButton}, new_game_tab::{ChoiceFlag, NewGameTab}}, politics::countries::CountriesSets};

pub fn new_game_button_system(
    mut interactions: Query<ButtonQuery, (With<NewGameButton>, Changed<Interaction>)>,
    mut mouse_button: ResMut<ButtonInput<MouseButton>>,
    menu: Single<Entity, With<MainMenu>>,
    new_game_tab: Single<Entity, With<NewGameTab>>,
    mut commands: Commands,
    mut image: Single<&mut ImageNode, With<ChoiceFlag>>,
    country_choice: Res<CountryChoice>,
    sets: Res<CountriesSets>,
) {
    for (interaction, mut color) in
        &mut interactions
    {
        match interaction {
            Interaction::Pressed => {
                mouse_button.clear_just_pressed(MouseButton::Left);
                commands.entity(*menu).insert(Visibility::Hidden);
                commands.entity(*new_game_tab).insert(Visibility::Visible);
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

pub fn exit_game_button_system(
    mut interactions: Query<ButtonQuery, (With<ExitGameButton>, Changed<Interaction>)>,
    mut mouse_button: ResMut<ButtonInput<MouseButton>>,
    mut app_exit_writer: MessageWriter<AppExit>,
) {
    for (interaction, mut color) in
        &mut interactions
    {
        match interaction {
            Interaction::Pressed => {
                mouse_button.clear_just_pressed(MouseButton::Left);
                app_exit_writer.write(AppExit::Success);
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

pub fn load_game_button_system(
    mut interactions: Query<ButtonQuery, (With<LoadGameButton>, Changed<Interaction>)>,
    mut mouse_button: ResMut<ButtonInput<MouseButton>>,
    menu: Single<Entity, With<MainMenu>>,
    load_game_tab: Single<Entity, With<LoadGameTab>>,
    mut commands: Commands,
) {
    for (interaction, mut color) in
        &mut interactions
    {
        match interaction {
            Interaction::Pressed => {
                mouse_button.clear_just_pressed(MouseButton::Left);
                commands.entity(*menu).insert(Visibility::Hidden);
                commands.entity(*load_game_tab).insert(Visibility::Visible);
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