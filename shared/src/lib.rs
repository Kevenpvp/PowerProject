use std::convert::Into;
use std::time::Duration;
use bevy::prelude::*;
use lightyear::prelude::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub mod bundles;
pub mod protocol;
pub mod globalcomponents;
pub mod components;
pub mod plugins;
pub mod systems;
pub mod globalevents;
pub mod globalresources;

pub const REPLICATION_GROUP: ReplicationGroup = ReplicationGroup::new_id(1);
pub const SERVER_PORT: u16 = 2555;
pub const PROTOCOL_ID: u64 = 1;
pub const PRIVATE_KEY: Key = [5; 32];
pub const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), SERVER_PORT);

pub fn shared_config() -> SharedConfig {
    SharedConfig {
        server_replication_send_interval: Default::default(),
        client_replication_send_interval: Default::default(),
        tick: TickConfig { tick_duration: Duration::from_secs_f64(1.0 / 64.0)},
    }
}