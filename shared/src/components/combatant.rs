use avian3d::prelude::{Collider, RigidBody};
use bevy::prelude::{Component, SceneRoot, Transform};
use crate::components::Scale;

#[derive(Component)]
pub struct Health{
    current: u64,
    max: u64
}
#[derive(Component)]
#[require(RigidBody(|| RigidBody::Dynamic) , Collider(|| Collider::capsule(0.3,1.0)), Transform(|| Transform::from_xyz(0.0, 1.0, 0.0)), Scale)]
pub struct CombatantBody;
#[derive(Component)]
#[require(Transform(|| Transform::from_xyz(0.0,-0.9,0.0)), SceneRoot)]
pub struct CombatantMesh;
#[derive(Component)]
pub struct Name(pub String);
#[derive(Component)]
pub enum Type{
    Player,
    Npc
}

impl Health{
    pub fn new(current: u64,max: u64) -> Self {
        Self {
            current,
            max
        }
    }
}