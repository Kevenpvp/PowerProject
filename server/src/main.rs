pub mod plugins;

use plugins::*;
use avian3d::PhysicsPlugins;
use avian3d::prelude::{Collider, PhysicsDebugPlugin, RigidBody};
use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::prelude::{AssetServer, Assets, BuildChildren, Camera3d, Color, Commands, Cylinder, Dir3, GltfAssetLabel, IntoSystemConfigs, Mesh, Mesh3d, MeshMaterial3d, PointLight, Res, ResMut, SceneRoot, StandardMaterial, Startup, Transform, Vec3};
use bevy::utils::default;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use shared::bundles::combatant::{CombatantBundle};
use shared::components::combatant::{CombatantMesh, Health};
use shared::{NetworkSide};
use crate::plugins::serverconnection::ServerConnectionPlugin;

fn add_combatants(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, asset_server: Res<AssetServer>){
    /*
    commands.spawn(CombatantBundle{
        health: Health::new(50,50),
        network_side: NetworkSide::Server,
        ..Default::default()
    }).with_child((
        CombatantMesh,
        Transform::from_xyz(0.0,-0.8,0.0),
        SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/Default.glb")),
        )
    ));
    */

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Dir3::Y),
    ));
}
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WorldInspectorPlugin::new(), PhysicsPlugins::default(), PhysicsDebugPlugin::default(), ServerConnectionPlugin))
        .add_systems(Startup,add_combatants)
        .run();
}