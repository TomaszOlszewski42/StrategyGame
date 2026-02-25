use std::sync::MutexGuard;

use bevy::{color::palettes::css::GREY, prelude::*};

use crate::{GameState, saves::saveing::SaveMessage};

#[derive(Resource)]
pub struct SaveMessageTimer(pub Timer);

impl SaveMessageTimer {
    pub fn new() -> Self {
        Self(Timer::from_seconds(2.0, TimerMode::Once))
    }
}

#[derive(Component)]
pub struct SaveWindow;

#[derive(Component)]
pub struct SaveMsgText;

pub fn spawn_saveing_message_window(
    mut commands: Commands,
) {
    commands.spawn((
        DespawnOnExit(GameState::Game),
        Node {
            width: Val::Percent(10.0),
            height: Val::Percent(10.0),
            .. default()
        },
        children![(
            Visibility::Hidden,
            SaveWindow,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(Color::Srgba(GREY)),
            children![(
                Text::new("Lol"),
                SaveMsgText,
            )]
        )]
    ));
}

pub fn saveing_message_window_system(
    window: Single<Entity, With<SaveWindow>>,
    msg: Res<SaveMessage>,
    mut commands: Commands,
    mut timer: ResMut<SaveMessageTimer>,
    mut window_text: Single<&mut Text, With<SaveMsgText>>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    match msg.0.try_lock() {
        Ok(mut guard) => {
            change_message(&mut guard, &window, &mut commands, &mut timer, &mut window_text);
        },
        Err(err) => {
            match err {
                std::sync::TryLockError::Poisoned(poison_error) => {
                    let mut guard = poison_error.into_inner();
                    change_message(&mut guard, &window, &mut commands, &mut timer, &mut window_text);
                },
                std::sync::TryLockError::WouldBlock => {},
            }
        },
    }
}

fn change_message(
    guard: &mut MutexGuard<'_, Option<String>>,
    window: &Single<Entity, With<SaveWindow>>,
    commands: &mut Commands,
    timer: &mut ResMut<SaveMessageTimer>,
    window_text: &mut Single<&mut Text, With<SaveMsgText>>,
) {
    match &mut **guard {
        Some(text) => {
            window_text.0 = text.clone();
            **guard = None;
            timer.0.reset();
            timer.0.unpause();
            commands.entity(**window).insert(Visibility::Visible);
        },
        None => {
            if timer.0.is_finished() {
                commands.entity(**window).insert(Visibility::Hidden);
            }
        },
    } 
}