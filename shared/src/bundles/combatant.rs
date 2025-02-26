use bevy::prelude::{default, Bundle, Transform};
use crate::components::combatant::{CombatantBody, CombatantMesh, Health, Name, Type};
use crate::NetworkSide;

#[derive(Bundle)]
pub struct CombatantBundle {
    pub health: Health,
    pub network_side: NetworkSide,
    pub name: Name,
    pub combatant_type: Type,
    pub combatant_body: CombatantBody
}

impl Default for CombatantBundle {
    fn default() -> Self {
        Self {
            health: Health::new(10,10),
            network_side: NetworkSide::Undefined,
            name: Name("Dummy".to_string()),
            combatant_type: Type::Npc,
            combatant_body: CombatantBody,
        }
    }
}