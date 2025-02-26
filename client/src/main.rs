pub mod plugins;

use avian3d::debug_render::PhysicsDebugPlugin;
use avian3d::PhysicsPlugins;
use bevy::DefaultPlugins;
use bevy::prelude::App;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use plugins::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins,WorldInspectorPlugin::new(), PhysicsPlugins::default(), PhysicsDebugPlugin::default(),clientconnection::ClientConnectionPlugin))
        .run();
}