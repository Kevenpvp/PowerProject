pub mod plugins;
pub mod systems;

use std::collections::HashMap;
use avian3d::debug_render::PhysicsDebugPlugin;
use avian3d::PhysicsPlugins;
use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use plugins::*;
use shared::components::states::StatesMachine;
use shared::globalresources::{CombatantsList, PlayNewAnimation, QueueAnimations};
use shared::systems::animationsystem::{queue_animation, setup_animations,play_animations};
use shared::systems::charactercontrollersysem::{adjust_collider, character_inputs, find_ground, move_character};
use shared::systems::statesmachinesystem::tick;
use crate::systems::camerasystem::{create_camera, mouse_moving_camera, update_camera};
use crate::systems::combatantsystem::{combatant_added};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins,WorldInspectorPlugin::new(), PhysicsPlugins::default(), PhysicsDebugPlugin::default(),clientconnection::ClientConnectionPlugin))
        .register_type::<StatesMachine>()
        .add_event::<PlayNewAnimation>()
        .insert_resource(CombatantsList(HashMap::new()))
        .insert_resource(QueueAnimations(HashMap::new()))
        .add_systems(Startup,create_camera)
        .add_systems(FixedUpdate,(tick,character_inputs,find_ground,adjust_collider,move_character).chain())
        .add_systems(PreUpdate,(combatant_added))
        .add_systems(Update,(setup_animations,queue_animation,play_animations).chain())
        .add_systems(PostUpdate,(mouse_moving_camera,update_camera).chain())
        .run();

}