use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use avian3d::collision::Collider;
use avian3d::prelude::RigidBody;
use bevy::app::App;
use bevy::math::{Dir3, Vec3};
use bevy::prelude::{info, Added, Camera3d, Commands, Entity, Plugin, Query, Res, Startup, Transform, Update, With};
use bevy::utils::default;
use lightyear::client::config::ClientConfig;
use lightyear::connection::client::IoConfig;
use lightyear::prelude::client::{Authentication, ClientCommandsExt, ClientConnection, ClientPlugins, ClientTransport, NetConfig};
use lightyear::prelude::Replicated;
use shared::{shared_config, PRIVATE_KEY, PROTOCOL_ID, SERVER_ADDR};
use shared::protocol::{FloorMarker, ProtocolPlugin};
use crate::plugins::renderer::RenderPlugin;
use crate::plugins::replicateplugin::ReplicatePlugin;

pub struct ClientConnectionPlugin;

const CLIENT_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 4000);

fn net_config() -> NetConfig{
    let io_config = IoConfig{
        transport: ClientTransport::UdpSocket(CLIENT_ADDR),
        ..default()
    };

    let auth = Authentication::Manual {
        server_addr: SERVER_ADDR,
        client_id: 1,
        private_key: PRIVATE_KEY,
        protocol_id: PROTOCOL_ID,
    };

    NetConfig::Netcode {
        auth,
        config: Default::default(),
        io: io_config
    }
}

fn setup_client_connection() -> ClientPlugins {
    ClientPlugins::new(ClientConfig {
        shared: shared_config(),
        net: net_config(),
        ..default()
    })
}

impl Plugin for ClientConnectionPlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins((setup_client_connection(),ProtocolPlugin,ReplicatePlugin,RenderPlugin)).add_systems(Startup,connect_client).add_systems(
            Update,
            handle_new_floor,
        );
    }
}

fn handle_new_floor(
    connection: Res<ClientConnection>,
    mut commands: Commands,
    character_query: Query<Entity, (Added<Replicated>, With<FloorMarker>)>,
) {
    for entity in &character_query {
        commands
            .entity(entity)
            .insert((Collider::cylinder(50.0, 0.1),RigidBody::Static));
    }
}

fn connect_client(mut commands: Commands) {
    commands.connect_client();

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Dir3::Y),
    ));
}