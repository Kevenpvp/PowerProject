use bevy::app::App;
use bevy::prelude::{Plugin, Update};
use crate::replicate;
use replicate::combatantreplicate::replicate_combatant;

pub struct ReplicatePlugin;

impl Plugin for ReplicatePlugin {
    fn build(&self, app: &mut App) {
       app.add_systems(Update,(replicate_combatant));
    }
}