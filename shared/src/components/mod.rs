use bevy::prelude::Component;

pub mod combatant;

#[derive(Component)]
pub struct Scale(pub f64);

impl Default for Scale {
    fn default() -> Self {
        Scale(1.0)
    }
}