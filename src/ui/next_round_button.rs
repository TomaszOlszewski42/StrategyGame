use bevy::color::palettes::css::{BLACK, DARK_SLATE_GRAY};
use bevy::{prelude::*};
use bevy::ui::Val;

use crate::{GameState, OnDemandSystems, errors::my_errors::MyErrors, main_menu::load_game_menu::ButtonQuery};

#[derive(Component)]
pub struct NextRoundButton;

pub fn create_round_button(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(GameState::Game),
        Node {
            width: Val::Px(150.0),
            height: Val::Px(75.0),
            align_items: AlignItems::Center,
            margin: UiRect { left: Val::Px(0.0), right: Val::Px(20.0), 
                top: Val::Px(0.0), bottom: Val::Px(20.0) },
            align_self: AlignSelf::End,
            justify_self: JustifySelf::End,
            .. default()
        },
        children![(
            DespawnOnExit(GameState::Game),
            NextRoundButton,
            Button,
            Node {
                width: Val::Px(150.0),
                height: Val::Px(75.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                .. default()
            },
            BackgroundColor(Color::BLACK),
            children![(DespawnOnExit(GameState::Game), Text::new("Next round"))]
        )]
    ));
}

// bevy button example
pub fn next_round_button_system(
    mut commands: Commands,
    mut interactions: Query<ButtonQuery, (With<NextRoundButton>, Changed<Interaction>)>,
    systems: Res<OnDemandSystems>,
    mut mouse_button: ResMut<ButtonInput<MouseButton>>
) -> Result<(), MyErrors> {
    for (interaction, mut color) in &mut interactions {
        match interaction {
            Interaction::Pressed => {
                mouse_button.clear_just_pressed(MouseButton::Left);
                let Some(systemid) = systems.systems.get("new_round") else {
                    return Err(MyErrors::InconsistentData("No `new_round` registered".to_string()))
                };
                commands.run_system(*systemid);
                let Some(systemid) = systems.systems.get("update_army_tab") else {
                    return Err(MyErrors::InconsistentData("No `update_army_tab` registered".to_string()))
                };
                commands.run_system(*systemid);
            }
            Interaction::Hovered => {
                *color = Color::Srgba(BLACK).into();
            }
            Interaction::None => {
                *color = Color::Srgba(DARK_SLATE_GRAY).into();
            }
        }
    }

    Ok(())
}
