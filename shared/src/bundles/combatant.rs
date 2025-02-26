use bevy::prelude::{Bundle};
use crate::components::combatant::{CombatantBody,Health, Name, Type};

#[derive(Bundle)]
pub struct CombatantBundle {
    pub health: Health,
    pub name: Name,
    pub combatant_type: Type,
    pub combatant_body: CombatantBody
}

impl Default for CombatantBundle {
    fn default() -> Self {
        Self {
            health: Health::new(10,10),
            name: Name("Dummy".to_string()),
            combatant_type: Type::Npc,
            combatant_body: CombatantBody,
        }
    }
}