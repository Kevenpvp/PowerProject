use std::collections::VecDeque;
use std::ops::Deref;
use std::sync::Arc;
use avian3d::math::Vector;
use avian3d::prelude::PhysicsLayer;
use bevy::prelude::{AnimationGraph, AnimationNodeIndex, Component, Deref, DerefMut, Entity, FromReflect, Handle, KeyCode, Reflect, Vec3};
use bevy::utils::HashMap;
use lightyear::prelude::{Deserialize, Serialize};

#[derive(Component)]
pub struct Health{
    current: u64,
    max: u64
}

#[derive(Component,PartialEq,Eq)]
pub enum NetworkSide{
    Server,
    Client
}

#[derive(Default,Component, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
pub struct CharacterController{
    pub floor: Option<Entity>,
    pub floor_position: Option<Vector>
}

#[derive(PhysicsLayer, Clone, Copy, Debug, Default, Component, PartialEq, Serialize, Deserialize, Reflect)]
pub enum GameMask{
    #[default]
    Default,
    Combatants
}

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Scale(pub f64);

#[derive(Component)]
pub struct PlayerCombatant;

#[derive(Component,Clone)]
pub struct AnimationsComponent{
    pub node_indices: Vec<AnimationNodeIndex>,
    pub animations_names: HashMap<String,AnimationNodeIndex>,
    pub parent_body: Entity
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
pub struct Source{
    entity: Option<Entity>,
    skill_name: String
}

impl Default for Scale {
    fn default() -> Self {
        Scale(1.0)
    }
}

impl Default for Source{
    fn default() -> Self {
        Self{
            entity: None,
            skill_name: "".into()
        }
    }
}

impl Health {
    pub fn new(current: u64,max: u64) -> Self {
        Self {
            current,
            max
        }
    }
}