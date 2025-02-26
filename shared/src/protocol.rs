use bevy::prelude::{App, Component, Plugin};
use lightyear::prelude::{AppComponentExt, ChannelDirection, ReplicationGroup};
use lightyear::prelude::client::ComponentSyncMode;
use serde::{Deserialize, Serialize};

pub const REPLICATION_GROUP: ReplicationGroup = ReplicationGroup::new_id(1);

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FloorMarker;

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CombatantMarker;

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.register_component::<FloorMarker>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once);

        app.register_component::<CombatantMarker>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once);
    }
}