use bevy::asset::AssetContainer;
use bevy::prelude::{default, Commands, EventReader, ResMut};
use leafwing_input_manager::prelude::ActionState;
use lightyear::prelude::NetworkTarget;
use lightyear::prelude::server::{ConnectEvent, ControlledBy, Replicate, ReplicationTarget};
use shared::bundles::combatant::{CombatantBundle, Type};
use shared::globalcomponents::{GameMask, Health, NetworkSide};
use shared::globalresources::CombatantsList;
use shared::protocol::{CombatantActions, CombatantMarker};
use shared::REPLICATION_GROUP;

pub fn player_joined(
    mut connections: EventReader<ConnectEvent>,
    mut commands: Commands,
    mut combatants_list: ResMut<CombatantsList>
){
    for connection in connections.read() {
        let client_id = connection.client_id;

        let entity = commands.spawn((CombatantBundle{
            health: Health::new(50,50),
            combatant_type: Type::Player,
            ..Default::default()
        },ActionState::<CombatantActions>::default(),GameMask::Combatants,NetworkSide::Server,Replicate{
            group: REPLICATION_GROUP,
            target: ReplicationTarget {
                target: NetworkTarget::All,
            },
            controlled_by: ControlledBy {
                target: NetworkTarget::Single(client_id),
                ..default()
            },
            ..default()
        },CombatantMarker));

        combatants_list.0.insert(entity.id(),Some(client_id));
    }
}