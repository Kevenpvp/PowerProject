use std::collections::VecDeque;
use std::time::Duration;
use bevy::animation::AnimationPlayer;
use bevy::asset::{AssetServer, Assets};
use bevy::gltf::GltfAssetLabel;
use bevy::hierarchy::Parent;
use bevy::prelude::{Added, AnimationGraph, AnimationGraphHandle, AnimationTransitions, Commands, Entity, EventReader, Query, Res, ResMut};
use bevy::utils::HashMap;
use crate::bundles::combatant::CombatantBody;
use crate::globalcomponents::AnimationsComponent;
use crate::globalresources::{PlayNewAnimation, QueueAnimations};

pub fn setup_animations(
    mut commands: Commands,
    mut animation_query: Query<(Entity, &mut AnimationPlayer, &Parent), (Added<AnimationPlayer>)>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    asset_server: Res<AssetServer>,
    parents: Query<&Parent>,
    mut final_parents: Query<(Entity, &CombatantBody)>
){
    for (entity, _, parent) in &mut animation_query.iter_mut() {
        let mut absolute_parent: Option<Entity> = None;

        if let Ok(grandparent) = parents.get(parent.get()) {

            for (entity, combatant_body) in final_parents.iter(){
                if combatant_body.body_entity == Option::from(grandparent.get()) {
                    absolute_parent = Some(entity);
                }
            }
        }

        let absolute_parent = absolute_parent.unwrap();
        let (graph, node_indices) = AnimationGraph::from_clips([
            asset_server.load(GltfAssetLabel::Animation(0).from_asset("animations/Idle.glb")),
            asset_server.load(GltfAssetLabel::Animation(0).from_asset("animations/Walking.glb")),
        ]);

        commands.entity(entity).insert(AnimationTransitions::new()).insert(AnimationGraphHandle(graphs.add(graph))).insert(AnimationsComponent{
            parent_body: absolute_parent,
            animations_names: HashMap::from([
                ("Idle".to_string(),node_indices[0]),
                ("Walking".to_string(), node_indices[1])
            ]),
            node_indices
        });
    }
}

pub fn queue_animation(
    mut ev_playnewanimation: EventReader<PlayNewAnimation>,
    animation_query: Query<(Entity)>,
    mut queue_animations_list: ResMut<QueueAnimations>,
){
    for ev in ev_playnewanimation.read() {
        for (entity) in animation_query.iter() {
            if !queue_animations_list.0.contains_key(&entity){
                queue_animations_list.0.insert(entity,VecDeque::new());
            }

            let mut queue_list = queue_animations_list.0.get_mut(&entity).unwrap();

            if queue_list.contains(&ev.1) {
                queue_list.retain(|x| *x != ev.1);
            }

            queue_list.push_back(ev.1.clone());
        }
    }
}

pub fn play_animations(
    mut animation_query: Query<(&mut AnimationPlayer, &mut AnimationsComponent, &mut AnimationTransitions)>,
    mut queue_animations_list: ResMut<QueueAnimations>,
){
    let queue_animations_list = &mut queue_animations_list.0;

    for (mut animation_player,animation_component,mut animation_transitions) in animation_query.iter_mut() {
        if queue_animations_list.contains_key(&animation_component.parent_body) {

            let queue_animations_list = queue_animations_list.get_mut(&animation_component.parent_body).unwrap();

            while let Some(animation_name) = queue_animations_list.pop_front(){
                animation_transitions.play(&mut *animation_player,animation_component.animations_names[&animation_name],Duration::ZERO).repeat();
            }
        }
    }
}