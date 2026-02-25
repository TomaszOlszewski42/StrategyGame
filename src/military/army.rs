use std::cmp;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{GameState, environment::map::{HexMap, TerrainType}, errors::my_errors::MyErrors, military::recruitment::ArmyModel};

#[derive(Component, Clone, Serialize, Deserialize)]
pub struct Army {
    pub number_of_soldiers: i32,
    #[serde(skip)]
    pub model: Handle<Scene>,
    pub owner: Entity,
    pub move_points_left: i32,
    pub move_points_max: i32,
}

impl Army {
    pub fn new(
        number_of_soldiers: i32, 
        model: Handle<Scene>, 
        owner: Entity, 
        move_points_left: i32, 
        move_points_max: i32
    ) -> Army {
        Army {number_of_soldiers, model, owner, move_points_left, move_points_max}
    }

    pub fn reset_move_points(&mut self) {
        self.move_points_left = self.move_points_max;
    }

    pub fn merge_with_other(&mut self, other: Army) {
        self.number_of_soldiers += other.number_of_soldiers;
        self.move_points_left = cmp::min(self.move_points_left, other.move_points_left);
    }

    pub fn merge_with_other_ref(&mut self, other: &Army) {
        self.number_of_soldiers += other.number_of_soldiers;
    }

    pub fn battle(army_1: &Army, army_2: &Army) -> Option<Army> {
        if army_1.number_of_soldiers == army_2.number_of_soldiers {
            None
        } else if army_1.number_of_soldiers > army_2.number_of_soldiers {
            Some(Army { 
                number_of_soldiers: army_1.number_of_soldiers - army_2.number_of_soldiers, 
                model: army_1.model.clone(), 
                owner: army_1.owner, 
                move_points_left: army_1.move_points_left, 
                move_points_max: army_1.move_points_max 
            })
        } else {
            Some(Army { 
                number_of_soldiers: army_2.number_of_soldiers - army_1.number_of_soldiers, 
                model: army_2.model.clone(), 
                owner: army_2.owner, 
                move_points_left: army_2.move_points_left, 
                move_points_max: army_2.move_points_max 
            })
        }
    }
}

pub fn redraw_armies_models(
    mut commands: Commands,
    map: ResMut<HexMap>,
    models: Query<Entity, With<ArmyModel>>,
) -> Result<(), MyErrors> {
    for model in models {
        commands.entity(model).despawn();
    }
    for (hex, army) in map.armies.iter() {
        let Some((entity, terrain, _)) = map.entities.get(hex) else {
            return Err(MyErrors::InconsistentData("No Hex Entity for hex that army is standing on".to_string()))
        };

        let transform = match terrain {
            TerrainType::Mountain(_) => Transform::from_xyz(0.0, 0.35, 0.0),
            _ => Transform::from_xyz(0.0, 0.15, 0.0),
        };

        let child_id = commands.spawn((
            DespawnOnExit(GameState::Game),
            SceneRoot(army.model.clone()),
            transform.with_scale(Vec3::new(0.08, 0.15, 0.08)), 
            ArmyModel,
        )).id();
        commands.entity(*entity).add_child(child_id);
    }

    Ok(())
}