pub mod plugins;

use avian3d::PhysicsPlugins;
use avian3d::prelude::{PhysicsDebugPlugin};
use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::prelude::{BuildChildren,IntoSystemConfigs};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::plugins::serverconnection::ServerConnectionPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WorldInspectorPlugin::new(), PhysicsPlugins::default(), PhysicsDebugPlugin::default(), ServerConnectionPlugin))
        .run();
}