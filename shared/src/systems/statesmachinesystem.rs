use bevy::prelude::{Commands, Entity, Query, With, Without};
use crate::components::states::{StatesMachine, StatesMethods};
use crate::globalcomponents::{AnimationsComponent, NetworkSide};
use crate::protocol::CombatantMarker;

pub fn tick(
    mut commands: Commands,
    mut character_query: Query<(Entity, &mut StatesMachine, &NetworkSide), (With<CombatantMarker>, Without<AnimationsComponent>)>,
){
    for (entity, mut machine, network_side) in character_query.iter_mut(){
        machine.tick(&mut commands, entity, network_side);
    }
}