use bevy::app::App;
use bevy::prelude::{info, Added, Assets, Color, Commands, Cylinder, Entity, Mesh, Mesh3d, MeshMaterial3d, Plugin, Query, ResMut, StandardMaterial, Update};
use shared::globalcomponents::GameMask;
use shared::protocol::FloorMarker;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,add_floor_cosmetics);
    }
}

fn add_floor_cosmetics(
    mut commands: Commands,
    floor_query: Query<Entity, Added<FloorMarker>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for entity in &floor_query {
        info!(?entity, "Adding cosmetics to floor {:?}", entity);
        commands.entity(entity).insert((
            Mesh3d(meshes.add(Cylinder::new(50.0, 0.1))),
            MeshMaterial3d(materials.add(Color::WHITE)),
            GameMask::Default
        ));
    }
}