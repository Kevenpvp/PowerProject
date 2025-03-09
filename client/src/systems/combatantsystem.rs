use bevy::asset::AssetServer;
use bevy::gltf::GltfAssetLabel;
use bevy::hierarchy::BuildChildren;
use bevy::prelude::{Added, Commands, Component, Entity, Has, InheritedVisibility, KeyCode, Query, Res, ResMut, SceneRoot, Transform, With};
use bevy_inspector_egui::egui::Key::K;
use leafwing_input_manager::prelude::{GamepadStick, InputMap, VirtualDPad};
use lightyear::connection::client::ClientConnection;
use lightyear::prelude::client::NetClient;
use lightyear::prelude::Replicated;
use lightyear::shared::replication::components::Controlled;
use shared::bundles::combatant::{CombatantBody, CombatantBundle, Type};
use shared::globalcomponents::{GameMask, Health, NetworkSide, PlayerCombatant};
use shared::globalresources::CombatantsList;
use shared::protocol::{CombatantActions, CombatantMarker};

const ANIMATION_PATH: &str = "animations/Idle.glb";

pub fn combatant_added(
    connection: Res<ClientConnection>,
    mut commands: Commands,
    character_query: Query<(Entity,Has<Controlled>, &Type), (Added<Replicated>, With<CombatantMarker>)>,
    asset_server: Res<AssetServer>,
    mut combatants_list: ResMut<CombatantsList>,
){
    for (entity,is_controlled, combatant_type) in &character_query {
        let body_child = commands.spawn((
            Transform::from_xyz(0.0,-0.8,0.0),
            SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/Default.glb")),
            ),
        )).id();

        commands.entity(entity)
            .insert(CombatantBundle{
                health: Health::new(50,50),
                combatant_type: combatant_type.clone(),
                combatant_body: CombatantBody{
                    body_entity: Some(body_child),
                },
                ..Default::default()
            }).insert((GameMask::Combatants,InheritedVisibility::VISIBLE,NetworkSide::Client)).add_child(body_child);

        if is_controlled {
            commands.entity(entity).insert(PlayerCombatant).insert(
                InputMap::new([
                    (CombatantActions::Jump,KeyCode::Space),
                ]).with_dual_axis(CombatantActions::Move, GamepadStick::LEFT).with_dual_axis(CombatantActions::Move,VirtualDPad::wasd())
            );
        }

        combatants_list.0.insert(entity,Some(connection.id()));
    }
}