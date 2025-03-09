use std::collections::{HashMap, VecDeque};
use bevy::prelude::{Entity, Event, Resource};
use lightyear::prelude::ClientId;

#[derive(Resource)]
pub struct CombatantsList(pub HashMap<Entity,Option<ClientId>>);

#[derive(Resource)]
pub struct QueueAnimations(pub HashMap<Entity,VecDeque<String>>);

#[derive(Event,Resource)]
pub struct PlayNewAnimation(pub Entity,pub String);