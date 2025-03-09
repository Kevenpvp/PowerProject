use avian3d::collision::Collider;
use avian3d::prelude::RigidBody;
use bevy::app::App;
use bevy::math::{Dir3, Vec3};
use bevy::pbr::{PointLight};
use bevy::prelude::{default, Camera3d, Commands, Plugin, Startup, Transform};
use lightyear::prelude::{NetworkTarget};
use lightyear::prelude::server::{IoConfig, NetConfig, NetcodeConfig, Replicate, ReplicationTarget, ServerCommandsExt, ServerConfig, ServerPlugins, ServerTransport};
use shared::{shared_config, PRIVATE_KEY, PROTOCOL_ID, REPLICATION_GROUP, SERVER_ADDR};
use shared::globalcomponents::GameMask;
use shared::protocol::{FloorMarker, ProtocolPlugin};

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
        app.add_plugins((setup_server_connection(),ProtocolPlugin)).add_systems(Startup,start_server);
    }
}

fn start_server(mut commands: Commands) {
    commands.start_server();

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

    commands.spawn((
        RigidBody::Static,
        Collider::cylinder(50.0, 0.1),
        FloorMarker,
        GameMask::Default,
        Replicate{
            group: REPLICATION_GROUP,
            target: ReplicationTarget {
                target: NetworkTarget::All,
            },
            ..default()
        }
    ));
}