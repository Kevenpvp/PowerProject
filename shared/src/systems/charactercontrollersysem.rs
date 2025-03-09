use avian3d::{prelude::*};
use avian3d::prelude::{Collider, SpatialQuery};
use bevy::math::Vec3;
use bevy::prelude::{Commands, Dir3, Entity, Quat, Query, Transform, With};
use leafwing_input_manager::prelude::ActionState;
use crate::bundles::combatant::{WalkingSpeed};
use crate::components::states::{StatesInfos, StatesMachine, StatesMethods, StatesType, StatesValues};
use crate::globalcomponents::{CharacterController, NetworkSide, PlayerCombatant};
use crate::protocol::CombatantActions;

pub fn character_inputs(
    mut commands: Commands,
    mut character_query: Query<(Entity, &ActionState<CombatantActions>, &mut StatesMachine, &NetworkSide, Option<&PlayerCombatant>), (With<CharacterController>)>,
){
    for (entity,action_state,mut states_machine,network_side,player_combatant) in character_query.iter_mut() {
        if player_combatant.is_none() && network_side != &NetworkSide::Server{
            continue;
        }

        let move_dir = action_state.axis_pair(&CombatantActions::Move);

        if move_dir.x != 0.0 || move_dir.y != 0.0 {
            if !states_machine.can_execute(StatesType::Walking) {continue;}

            if !states_machine.current_states.contains_key(&StatesType::Walking) {
                states_machine.execute(&mut commands,entity,network_side,StatesType::Walking,StatesInfos{
                    start: Default::default(),
                    duration: 0.0,
                    source: Default::default(),
                    in_cooldown: false,
                    applied: false,
                    values: Some(StatesValues::Walking(
                        Vec3::new(-move_dir.x,0.0, move_dir.y),
                    )),
                });
            }else {
                states_machine.current_states.get_mut(&StatesType::Walking).unwrap().values = Some(StatesValues::Walking(
                    Vec3::new(-move_dir.x,0.0, move_dir.y),
                ));
            }
        }else {
            if !states_machine.can_execute(StatesType::Idle) {continue;}

            if !states_machine.current_states.contains_key(&StatesType::Idle) {
                states_machine.execute(&mut commands,entity,network_side,StatesType::Idle,StatesInfos{
                    start: Default::default(),
                    duration: 0.0,
                    source: Default::default(),
                    in_cooldown: false,
                    applied: false,
                    values: None,
                });
            }
        }
    }
}

pub fn find_ground(
    mut character_query: Query<(Entity, &Transform, &mut CharacterController, &Collider), (With<CharacterController>)>,
    query: SpatialQuery,
){
    for (_,transform,mut controller, collider) in character_query.iter_mut(){
        let transform_translation = transform.translation;

        let shape_hit_data = query.cast_shape_predicate(collider, transform_translation, Quat::from_rotation_y(0.0),Dir3::Z,&ShapeCastConfig{
            max_distance: 20.0,
            target_distance: 0.0,
            compute_contact_on_penetration: true,
            ignore_origin_penetration: true,
        },&SpatialQueryFilter{
            mask: LayerMask(1),
            excluded_entities: Default::default(),
        },&|entity_found| {
            true
        });

        if let Some(shape_hit_data) = shape_hit_data {
            controller.floor = Some(shape_hit_data.entity);
            controller.floor_position = Some(shape_hit_data.point1)
        }else {
            controller.floor = None;
            controller.floor_position = None;
        }
    }
}

pub fn adjust_collider(
    mut character_query: Query<(Entity, &mut Transform, &mut LinearVelocity, &mut CharacterController, &Collider), (With<CharacterController>, With<Collider>)>,
){
  for (_,mut transform,mut linear_velocity, character_controller,collider) in character_query.iter_mut(){
      if character_controller.floor_position.is_some(){

      }
  }
}

pub fn move_character(
    mut character_query: Query<(Entity, &Transform, &mut CharacterController, &mut LinearVelocity, &StatesMachine, &WalkingSpeed, &NetworkSide), (With<CharacterController>)>,
){
    for (_,_,mut controller, mut linear_velocity, states_machine, walking_speed, n) in character_query.iter_mut() {
        if states_machine.current_states.contains_key(&StatesType::Walking){
            let states_values = &states_machine.current_states[&StatesType::Walking].values;

            if let Some(states_values) = states_values {
                if let StatesValues::Walking(v) = states_values {
                    let new_z = v.z * walking_speed.0;
                    let new_x = v.x * walking_speed.0;

                    if new_z != 0.0{
                        linear_velocity.z = new_z;
                    }

                    if new_x != 0.0{
                        linear_velocity.x = new_x;
                    }
                }
            }
        }
    }
}