use bevy::asset::AssetServer;
use bevy::prelude::{Added, BuildChildren, Commands, Entity, GltfAssetLabel, Query, Res, SceneRoot, Transform, With};
use lightyear::connection::client::ClientConnection;
use lightyear::prelude::Replicated;
use shared::bundles::combatant::CombatantBundle;
use shared::components::combatant::{CombatantMesh, Health};
use shared::protocol::{CombatantMarker};

pub fn replicate_combatant(
    connection: Res<ClientConnection>,
    mut commands: Commands,
    character_query: Query<Entity, (Added<Replicated>, With<CombatantMarker>)>,
    asset_server: Res<AssetServer>
){
    for entity in &character_query {
        commands
            .entity(entity)
            .insert(CombatantBundle{
                    health: Health::new(50,50),
                    ..Default::default()
                }
            ).with_child((
            CombatantMesh,
            Transform::from_xyz(0.0,-0.8,0.0),
            SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/Default.glb")),
            )
            ));
    }
}