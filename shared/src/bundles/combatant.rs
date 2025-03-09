use avian3d::math::Scalar;
use avian3d::prelude::*;
use bevy::prelude::{Bundle, Component, Entity};
use crate::globalcomponents::{Health, Name};
use bevy::prelude::Transform;
use crate::globalcomponents::Scale;
use serde::{Deserialize, Serialize};
use crate::components::states::StatesMachine;
use crate::globalcomponents::CharacterController;

#[derive(Bundle)]
pub struct CombatantBundle {
    pub health: Health,
    pub name: Name,
    pub combatant_type: Type,
    pub combatant_body: CombatantBody,
    pub states_machine: StatesMachine
}

#[derive(Component,Serialize, Deserialize, Clone, Debug, PartialEq,Default)]
pub struct WalkingSpeed(pub Scalar);

#[derive(Component)]
#[require(WalkingSpeed (|| WalkingSpeed(8.0_f32)), LockedAxes (|| LockedAxes::new().lock_rotation_x().lock_rotation_z()) ,CharacterController, Friction(||  Friction::new(50.0_f32)), RigidBody(|| RigidBody::Dynamic) , Collider(|| Collider::capsule(0.3,1.0)), Transform(|| Transform::from_xyz(0.0, 1.0, 0.0)), Scale)]
pub struct CombatantBody{
    pub body_entity: Option<Entity>,
}
#[derive(Component,Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Type{
    Player,
    Npc
}

impl Default for CombatantBundle {

    fn default() -> Self {
        Self {
            health: Health::new(10,10),
            name: Name("Dummy".to_string()),
            combatant_type: Type::Npc,
            combatant_body: CombatantBody{
                body_entity: None,
            },
            states_machine: StatesMachine::default(),
        }
    }
}