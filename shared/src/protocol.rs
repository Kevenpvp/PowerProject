use avian3d::prelude::{AngularVelocity, ComputedMass, ExternalForce, ExternalImpulse, LinearVelocity, Position, Rotation};
use bevy::prelude::{App, Component, Plugin, Reflect, Transform};
use leafwing_input_manager::{Actionlike, InputControlKind};
use lightyear::prelude::{AppComponentExt, ChannelDirection, LeafwingInputPlugin, ReplicationGroup};
use lightyear::prelude::client::{ComponentSyncMode, LeafwingInputConfig, LerpFn};
use serde::{Deserialize, Serialize};
use crate::bundles::combatant::{Type, WalkingSpeed};
use crate::components::states::StatesMachine;
use lightyear::utils::avian3d::{position, rotation};
use lightyear::utils::bevy::TransformLinearInterpolation;
use crate::globalcomponents::CharacterController;

pub const REPLICATION_GROUP: ReplicationGroup = ReplicationGroup::new_id(1);

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FloorMarker;

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CombatantMarker;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect, Serialize, Deserialize)]
pub enum CombatantActions{
    Move,
    Jump
}

impl Actionlike for CombatantActions {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            Self::Move => InputControlKind::DualAxis,
            Self::Jump => InputControlKind::Button,
        }
    }
}

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LeafwingInputPlugin::<CombatantActions> {
            config: LeafwingInputConfig::<CombatantActions> {
                lag_compensation: true,
                packet_redundancy: 3,
                marker: Default::default(),
            },
        });

        app.register_component::<FloorMarker>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once);

        app.register_component::<CharacterController>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once);

        app.register_component::<Type>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once);

        app.register_component::<WalkingSpeed>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Simple);

        app.register_component::<LinearVelocity>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full);

        app.register_component::<CombatantMarker>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once);

        app.register_component::<AngularVelocity>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full);

        app.register_component::<ExternalForce>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full);

        app.register_component::<ExternalImpulse>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full);

        app.register_component::<ComputedMass>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full);

        app.register_component::<Position>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full)
            .add_interpolation_fn(position::lerp)
            .add_correction_fn(position::lerp);

        app.register_component::<Rotation>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full)
            .add_interpolation_fn(rotation::lerp)
            .add_correction_fn(rotation::lerp);

        app.add_interpolation::<Transform>(ComponentSyncMode::None);
        app.add_interpolation_fn::<Transform>(TransformLinearInterpolation::lerp);

    }
}