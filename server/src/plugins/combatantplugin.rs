use bevy::app::{App, Update};
use bevy::asset::Assets;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{default, Commands, EventReader, Mesh, Plugin, ResMut};
use lightyear::prelude::NetworkTarget;
use lightyear::prelude::server::{ConnectEvent, Replicate, ReplicationTarget};
use shared::bundles::combatant::CombatantBundle;
use shared::components::combatant::Health;
use shared::protocol::CombatantMarker;
use shared::REPLICATION_GROUP;

pub struct CombatantPlugin;

impl Plugin for CombatantPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_joined);
    }
}

fn player_joined(
    mut connections: EventReader<ConnectEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
){
    for connection in connections.read() {
        let client_id = connection.client_id;

        commands.spawn((CombatantBundle{
            health: Health::new(50,50),
            ..Default::default()
        },Replicate{
            group: REPLICATION_GROUP,
            target: ReplicationTarget {
                target: NetworkTarget::All,
            },
            ..default()
        },CombatantMarker));

        println!("The client id is {}", &client_id);
    }
}