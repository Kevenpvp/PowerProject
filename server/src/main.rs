pub mod plugins;
pub mod systems;

use std::collections::HashMap;
use avian3d::PhysicsPlugins;
use avian3d::prelude::{PhysicsDebugPlugin};
use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::prelude::{BuildChildren, FixedUpdate, IntoSystemConfigs, Update};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use shared::components::states::StatesMachine;
use shared::globalresources::CombatantsList;
use shared::systems::charactercontrollersysem::{adjust_collider, character_inputs, find_ground, move_character};
use shared::systems::statesmachinesystem::tick;
use crate::plugins::serverconnection::ServerConnectionPlugin;
use crate::systems::combatantsystem::player_joined;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WorldInspectorPlugin::new(), PhysicsPlugins::default(), PhysicsDebugPlugin::default(), ServerConnectionPlugin))
        .register_type::<StatesMachine>()
        .insert_resource(CombatantsList(HashMap::new()))
        .add_systems(FixedUpdate,(tick,character_inputs,find_ground,adjust_collider,move_character).chain())
        .add_systems(Update,(player_joined))
        .run();
}