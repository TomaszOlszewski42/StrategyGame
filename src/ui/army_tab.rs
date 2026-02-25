use bevy::{color::palettes::css::{BLACK, GREY}, prelude::*};

use crate::{GameState, environment::map::{HexMap, SelectedHex}};

#[derive(Component)]
pub struct SoldiersAmountText;

#[derive(Component)]
pub struct MovePointsText;

#[derive(Component)]
pub struct ArmyTabBase;

pub fn spawn_army_tab(
    commands: &mut Commands, 
) {
    commands.spawn((
        DespawnOnExit(GameState::Game),
        Node {
            margin: UiRect { left: Val::Px(220.0), right: Val::Px(0.0), top: Val::Px(25.0), bottom: Val::Px(0.0) },
            flex_direction: FlexDirection::Column,
            .. default()
        },
        Visibility::Hidden,
        ArmyTabBase,
        children![
            army_tab_soldiers_num(), 
            army_tab_moves_left()
            ]
    ));
}

fn army_tab_soldiers_num() -> impl Bundle {(
    DespawnOnExit(GameState::Game),
    Node {
        ..default()
    },
    BackgroundColor(Color::Srgba(BLACK)),
    children![(
        DespawnOnExit(GameState::Game),
        Text::new("Soldiers num: 42"),
        SoldiersAmountText,
    )]
)}

fn army_tab_moves_left() -> impl Bundle {(
    Node {
        ..default()
    },
    BackgroundColor(Color::Srgba(GREY)),
    children![(
        DespawnOnExit(GameState::Game),
        Text::new("Moves left: 2"),
        MovePointsText,
    )]
)}

pub fn update_army_tab(
    mut commands: Commands,
    army_tab: Single<Entity, With<ArmyTabBase>>,
    selected_hex_res: Res<SelectedHex>,
    map: Res<HexMap>,
    mut texts: ParamSet<(
        Single<&mut Text, With<SoldiersAmountText>>,
        Single<&mut Text, With<MovePointsText>>,
    )>,
) {
    let Some(selected_hex) = selected_hex_res.hex else {
        commands.entity(*army_tab).insert(Visibility::Hidden);
        return
    };

    let Some(army) = map.armies.get(&selected_hex) else {
        commands.entity(*army_tab).insert(Visibility::Hidden);
        return
    };

    commands.entity(*army_tab).insert(Visibility::Visible);
    texts.p0().0 = format!("Soldiers number: {}", army.number_of_soldiers);
    texts.p1().0 = format!("Move points left: {}", army.move_points_left);
}
