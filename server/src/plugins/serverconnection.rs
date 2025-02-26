use std::collections::HashMap;
use avian3d::collision::Collider;
use avian3d::prelude::RigidBody;
use bevy::app::App;
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::{default, Commands, Cylinder, Entity, EventReader, Mesh, Mesh3d, Plugin, ResMut, Resource, Startup, Update};
use lightyear::prelude::{ClientId, NetworkTarget};
use lightyear::prelude::server::{ConnectEvent, IoConfig, NetConfig, NetcodeConfig, Replicate, ReplicationTarget, ServerCommandsExt, ServerConfig, ServerPlugins, ServerTransport, SyncTarget};
use shared::{shared_config, FloorMarker, PRIVATE_KEY, PROTOCOL_ID, REPLICATION_GROUP, SERVER_ADDR};
use shared::protocol::ProtocolPlugin;

pub struct ServerConnectionPlugin;

fn net_config() -> NetConfig{
    let io_config = IoConfig{
        transport: ServerTransport::UdpSocket(SERVER_ADDR),
        ..default()
    };

    let netcode_config = NetcodeConfig::default()
        .with_protocol_id(PROTOCOL_ID)
        .with_key(PRIVATE_KEY);

    NetConfig::Netcode{
        config: netcode_config,
        io: io_config,
    }
}

fn setup_server_connection() -> ServerPlugins{
    ServerPlugins::new(ServerConfig {
        shared: shared_config(),
        net: vec![net_config()],
        ..default()
    })
}

impl Plugin for ServerConnectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((setup_server_connection(),ProtocolPlugin)).add_systems(Startup,start_server).add_systems(Update, handle_connections);
    }
}

pub fn handle_connections(
    mut connections: EventReader<ConnectEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {

    for connection in connections.read() {
        let client_id = connection.client_id;

        println!("The client id is {}", &client_id);
    }
}

fn start_server(mut commands: Commands) {
    commands.start_server();

    commands.spawn((
        RigidBody::Static,
        Collider::cylinder(50.0, 0.1),
        //Mesh3d(meshes.add(Cylinder::new(50.0, 0.1))),
        //MeshMaterial3d(materials.add(Color::WHITE)),
        FloorMarker,
        Replicate{
            group: REPLICATION_GROUP,
            target: ReplicationTarget {
                target: NetworkTarget::All,
            },
            ..default()
        }
    ));
}