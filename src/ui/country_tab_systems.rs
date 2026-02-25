use bevy::color::palettes::css::{BEIGE, BLACK, DARK_RED, DARK_SLATE_GRAY, RED, WHITE};
use::bevy::prelude::*;
use crate::{OnDemandSystems, environment::map::{HexMap, SelectedHex}, errors::my_errors::MyErrors, main_menu::load_game_menu::ButtonQuery, military::recruitment::RecruitNumber, politics::{countries::{AtWarLists, Country, Flag, PeaceProposals, PlayerType, Treasury}, war::start_war}, ui::country_tab::{BuildMarketButton, DeclareWarButton, EnemyCountryUI, EnemyFlag, FlagUi, PlayerCountryTab, ProposePeaceButton, RecruitButton, RecruitmentCostText, RecrutimentNumberText, ReduceRecruitsButton, TreasureText, UpRecruitsButton}};

pub fn show_country_tab_system(
    map: Res<HexMap>,
    button: Res<ButtonInput<MouseButton>>,
    country: Query<(&Flag, &Treasury, &PlayerType), With<Country>>,
    mut commands: Commands,
    flag_images: Query<&mut ImageNode, With<FlagUi>>,
    player_ui_base: Single<Entity, With<PlayerCountryTab>>,
    gold_texts: Query<(&mut Text, &mut TreasureText)>,
    selected: Res<SelectedHex>,
    enemy_country_tab: Single<Entity, With<EnemyCountryUI>>,
    war_lists: Res<AtWarLists>,
    enemy_flags: Query<(&mut ImageNode, &EnemyFlag, Entity), Without<FlagUi>>,
) -> Result<(), MyErrors> {
    if !button.just_pressed(MouseButton::Left) {
        return Ok(());
    }

    let Some(clicked_hex) = selected.hex else {
        return Ok(())
    };

    let Some(owner) = map.hex_owners.get(&clicked_hex) else {
        commands.entity(*player_ui_base).insert(Visibility::Hidden);
        commands.entity(*enemy_country_tab).insert(Visibility::Hidden);
        return Ok(())
    };

    let Some(list) = war_lists.lists.get(owner) else {
        return Err(MyErrors::InconsistentData("Couldn't find War List".to_string()))
    };

    for (mut node, num, entity) in enemy_flags {
        if num.0 >= list.len() {
            commands.entity(entity).insert(Visibility::Hidden);
            continue;
        }
        let image = match country.get(list[num.0]) {
            Ok(tuple) => tuple.0,
            Err(_) => return Err(MyErrors::InconsistentData("Couldn't find flag image in query".to_string())),
        };
        node.image = image.image.clone();
        commands.entity(entity).insert(Visibility::Inherited);
    }

    let Ok(country) = country.get(*owner) else {
        return Err(MyErrors::InconsistentData("Couldn't find country in query".to_string()))
    };

    for mut flag in flag_images {
        flag.image = country.0.image.clone();
    }

    for mut text in gold_texts {
        text.0.0 = format!("Gold: {}", country.1.gold);
        text.1.gold_owner = Some(*owner);
    }

    match country.2 {
        PlayerType::Player => {
            commands.entity(*player_ui_base).insert(Visibility::Visible);
            commands.entity(*enemy_country_tab).insert(Visibility::Hidden); 
        },
        PlayerType::Bot => {
            commands.entity(*player_ui_base).insert(Visibility::Hidden);
            commands.entity(*enemy_country_tab).insert(Visibility::Visible);
        },
    };

    Ok(())
}

pub fn up_number_of_recruits_button_system(
    mut commands: Commands,
    interactions: Query<ButtonQuery, (With<UpRecruitsButton>, Changed<Interaction>)>,
    mut recruitment: ResMut<RecruitNumber>,
    mut mouse_button: ResMut<ButtonInput<MouseButton>>,
    systems: Res<OnDemandSystems>,
) -> Result<(), MyErrors> {
    for (interaction, mut bg_color) in interactions {
        match interaction {
            Interaction::Pressed => {
                let Some(systemid) = systems.systems.get("update_recruitment_texts") else {
                    return Err(MyErrors::InconsistentData("No `update_recruitment_texts` registered".to_string()))
                };
                recruitment.up_number(1);
                mouse_button.clear_just_pressed(MouseButton::Left);
                commands.run_system(*systemid);
            },
            Interaction::Hovered => {
                *bg_color = Color::Srgba(BLACK).into();
            }
            Interaction::None => {
                *bg_color = Color::Srgba(DARK_SLATE_GRAY).into();
            }
        }
    }

    Ok(())
}

pub fn reduce_number_of_recruits_button_system(
    mut commands: Commands,
    interactions: Query<ButtonQuery, (With<ReduceRecruitsButton>, Changed<Interaction>)>,
    mut recruitment: ResMut<RecruitNumber>,
    mut mouse_button: ResMut<ButtonInput<MouseButton>>,
    systems: Res<OnDemandSystems>,
) -> Result<(), MyErrors> {
    for (interaction, mut bg_color) in interactions {
        match interaction {
            Interaction::Pressed => {
                let Some(systemid) = systems.systems.get("update_recruitment_texts") else {
                    return Err(MyErrors::InconsistentData("No `update_recruitment_texts` registered".to_string()))
                };
                recruitment.up_number(-1);
                mouse_button.clear_just_pressed(MouseButton::Left);
                commands.run_system(*systemid);
            },
            Interaction::Hovered => {
                *bg_color = Color::Srgba(BLACK).into();
            }
            Interaction::None => {
                *bg_color = Color::Srgba(DARK_SLATE_GRAY).into();
            }
        }
    }

    Ok(())
}

pub fn update_recruitment_texts(
    recruitment: Res<RecruitNumber>,
    mut texts: ParamSet<(
        Single<&mut Text, With<RecruitmentCostText>>, 
        Single<&mut Text, With<RecrutimentNumberText>>
    )>,
) {
    texts.p0().0 = format!("-{}G", recruitment.price);
    texts.p1().0 = format!("+{}", recruitment.number);
}

pub fn update_country_tab(
    mut commands: Commands,
    systems: Res<OnDemandSystems>,
) -> Result<(), MyErrors> {
    let Some(systemid) = systems.systems.get("update_recruitment_texts") else {
        return Err(MyErrors::InconsistentData("No `update_recuitment_texts` registered".to_string()))
    };
    commands.run_system(*systemid);

    let Some(systemid) = systems.systems.get("update_country_tab_gold") else {
        return Err(MyErrors::InconsistentData("No `update_country_tab_gold` registered".to_string()))
    };
    commands.run_system(*systemid);

    let Some(systemid) = systems.systems.get("update_war_flags") else {
        return Err(MyErrors::InconsistentData("No `update_war_flags` registered".to_string()))
    };
    commands.run_system(*systemid);

    Ok(())
}

pub fn update_war_flags(
    map: Res<HexMap>,
    country: Query<(&Flag, &Treasury, &PlayerType), With<Country>>,
    mut commands: Commands,
    selected: Res<SelectedHex>,
    war_lists: Res<AtWarLists>,
    enemy_flags: Query<(&mut ImageNode, &EnemyFlag, Entity), Without<FlagUi>>,
) -> Result<(), MyErrors> {
    let Some(clicked_hex) = selected.hex else {
        return Ok(())
    };
    let Some(owner) = map.hex_owners.get(&clicked_hex) else {
        return Ok(())
    };
    let Some(list) = war_lists.lists.get(owner) else {
        return Err(MyErrors::InconsistentData("Couldn't find war list in query".to_string()))
    };
    for (mut node, num, entity) in enemy_flags {
        if num.0 >= list.len() {
            commands.entity(entity).insert(Visibility::Hidden);
            continue;
        }
        let image = match country.get(list[num.0]) {
            Ok(tuple) => tuple.0,
            Err(_) => return Err(MyErrors::InconsistentData("Couldn't find image in query".to_string())),
        };
        node.image = image.image.clone();
        commands.entity(entity).insert(Visibility::Inherited);
    }
    Ok(())
}

pub fn update_country_tab_gold(
    treasuries: Query<&Treasury, With<Country>>,
    gold_texts: Query<(&mut Text, &TreasureText)>,
) {
    for mut gold_text in gold_texts {
        if let Some(gold_owner) = gold_text.1.gold_owner
            && let Ok(treasure) = treasuries.get(gold_owner) {
                gold_text.0.0 = format!("Gold: {}", treasure.gold);
            }
    }
}

// bevy button example
pub fn recruit_button_system(
    mut commands: Commands,
    mut interactions: Query<ButtonQuery, (With<RecruitButton>, Changed<Interaction>)>,
    systems: Res<OnDemandSystems>,
    mut mouse_button: ResMut<ButtonInput<MouseButton>>
) -> Result<(), MyErrors> {
    for (interaction, mut color) in
        &mut interactions
    {
        match interaction {
            Interaction::Pressed => {
                let Some(systemid) = systems.systems.get("recruitment") else {
                    return Err(MyErrors::InconsistentData("No `recruitment` registered".to_string()))
                };
                mouse_button.clear_just_pressed(MouseButton::Left);
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

// bevy button example
pub fn build_market_button_system(
    mut commands: Commands,
    mut interactions: Query<ButtonQuery, (With<BuildMarketButton>, Changed<Interaction>)>,
    systems: Res<OnDemandSystems>,
    mut mouse_button: ResMut<ButtonInput<MouseButton>>
) -> Result<(), MyErrors> {
    for (interaction, mut color) in
        &mut interactions
    {
        match interaction {
            Interaction::Pressed => {
                let Some(systemid) = systems.systems.get("build_market") else {
                    return Err(MyErrors::InconsistentData("No `build_market` registered".to_string()))
                };
                mouse_button.clear_just_pressed(MouseButton::Left);
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

// bevy button example
pub fn declare_war_button_system(
    mut interactions: Query<ButtonQuery, (With<DeclareWarButton>, Changed<Interaction>)>,
    mut mouse_button: ResMut<ButtonInput<MouseButton>>,
    countries: Query<(Entity, &PlayerType)>,
    mut countries_wars: ResMut<AtWarLists>, 
    selected_hex: Res<SelectedHex>,
    map: Res<HexMap>,
    mut commands: Commands,
    systems: Res<OnDemandSystems>,
) -> Result<(), MyErrors> {
    for (interaction, mut color) in &mut interactions {
        match interaction {
            Interaction::Pressed => {
                mouse_button.clear_just_pressed(MouseButton::Left);
                let Some(selected) = selected_hex.hex else {
                    return Err(MyErrors::InconsistentData("Managed to start war without selected hex".to_string()))
                };
                let Some(owner) = map.hex_owners.get(&selected) else {
                    return Err(MyErrors::InconsistentData("Declaring war to unowned hex".to_string()))
                };
                start_war_button_click(&mut commands, &systems, owner, &countries, &mut countries_wars)?;
            }
            Interaction::Hovered => { *color = Color::Srgba(DARK_RED).into(); }
            Interaction::None => { *color = Color::Srgba(RED).into(); }
        }
    }
    Ok(())
}

fn start_war_button_click(commands: &mut Commands, systems: &Res<OnDemandSystems>, owner: &Entity, 
    countries: &Query<(Entity, &PlayerType)>, countries_wars: &mut ResMut<AtWarLists>, 
) -> Result<(), MyErrors> {
    let mut player: Option<Entity> = None;
    for (entity, player_type) in countries {
        match player_type {
            PlayerType::Player => {
                player = Some(entity);
                break;
            },
            PlayerType::Bot => {},
        }
    }

    match player {
        Some(player) => {
            start_war(player, *owner, countries_wars);
            
            let Some(systemid) = systems.systems.get("update_war_flags") else {
                return Err(MyErrors::InconsistentData("No update_war_flags system registered".to_string()))
            };
            commands.run_system(*systemid);

        },
        None => return Err(MyErrors::InconsistentData("Player is declaring war but there is no player entity".to_string())),
    }
    Ok(())
}

// bevy button example
pub fn propose_peace_button_system(
    mut interactions: Query<ButtonQuery, (With<ProposePeaceButton>, Changed<Interaction>)>,
    mut mouse_button: ResMut<ButtonInput<MouseButton>>,
    mut countries: Query<(Entity, &PlayerType, &mut PeaceProposals)>,
    selected_hex: Res<SelectedHex>,
    map: ResMut<HexMap>,
) -> Result<(), MyErrors> {
    for (interaction, mut color) in &mut interactions {
        match interaction {
            Interaction::Pressed => {
                mouse_button.clear_just_pressed(MouseButton::Left);
                let Some(selected) = selected_hex.hex else {
                    return Err(MyErrors::InconsistentData("Proposing peace with no selected hex".to_string()))
                };
                let Some(owner) = map.hex_owners.get(&selected) else {
                    return Err(MyErrors::InconsistentData("Proposing peace to unowned".to_string()))
                };

                try_to_make_peace_button_click(&mut countries, owner)?;
            }
            Interaction::Hovered => { *color = Color::Srgba(BEIGE).into(); }
            Interaction::None => { *color = Color::Srgba(WHITE).into(); }
        }
    }
    Ok(())
}

fn try_to_make_peace_button_click(
    countries: &mut Query<(Entity, &PlayerType, &mut PeaceProposals)>,
    owner: &Entity,
) -> Result<(), MyErrors> {
    let mut player: Option<Entity> = None;
    for (entity, player_type, _) in &mut *countries {
        match player_type {
            PlayerType::Player => {
                player = Some(entity);
                break;
            },
            PlayerType::Bot => {},
        }
    }

    match player {
        Some(player) => {
            let Ok((_, _, mut proposals)) = countries.get_mut(*owner) else {
                return Err(MyErrors::InconsistentData("Couldn't get peace proposals list".to_string()))
            };
            proposals.senders.push(player);
        },
        None => return Err(MyErrors::InconsistentData("Player is proposing peace but there is no player entity".to_string())),
    }
    Ok(())
}