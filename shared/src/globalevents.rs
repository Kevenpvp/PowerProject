use bevy::prelude::{AnimationNodeIndex, Event};

#[derive(Event)]
struct PlayAnimation(AnimationNodeIndex);
