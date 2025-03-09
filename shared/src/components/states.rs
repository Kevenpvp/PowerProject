use std::any::Any;
use std::cmp::PartialEq;
use std::hash::{Hash, Hasher};
use std::ops::{DerefMut, IndexMut};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use avian3d::prelude::LinearVelocity;
use bevy::prelude::{Commands, Component, Entity, Events, FromReflect, Reflect, TypePath, Vec3, World};
use bevy::reflect::{GetTypeRegistration, TypeRegistration};
use bevy::utils::HashMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::globalcomponents::{NetworkSide, Source};
use crate::globalresources::PlayNewAnimation;
use crate::plugins;

#[derive(Eq, Hash, Clone, Debug, Reflect)]
pub enum StatesType {
    Walking,
    Running,
    Stunned,
    Idle,
    Died
}

#[derive(Clone, Debug, PartialEq, Reflect)]
pub enum StatesValues{
    Walking(Vec3),
    Running,
    Stunned,
    Idle,
    Died
}

#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct StatesInfos{
    pub start: Option<u128>,
    pub duration: f32,
    pub source: Source,
    pub in_cooldown : bool,
    pub applied: bool,
    pub values: Option<StatesValues>
}

pub struct StatesSettings {
    pub blacklist: Vec<StatesType>,
    pub stopping: Vec<StatesType>,
    pub stop_all: bool
}

#[derive(Component, Clone, Debug, PartialEq, Reflect)]
pub struct StatesMachine{
    pub current_states: HashMap<StatesType,StatesInfos>,
}

impl From<&StatesMachine> for StatesMachine {
    fn from(state_machine: &StatesMachine) -> Self {
        let mut current_states: HashMap<StatesType,StatesInfos> = HashMap::new();

        for (state_type,states_infos) in &state_machine.current_states {

            let new_state_infos = StatesInfos{
                start: states_infos.start,
                duration: states_infos.duration,
                source: states_infos.source.clone(),
                in_cooldown: states_infos.in_cooldown,
                applied: false,
                values: None
            };

            current_states.insert(state_type.clone(),new_state_infos);
        }

        Self{
            current_states
        }
    }
}

impl PartialEq for StatesType{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (StatesType::Walking, StatesType::Walking) => true,
            (StatesType::Running, StatesType::Running) => true,
            (StatesType::Stunned, StatesType::Stunned) => true,
            (StatesType::Idle, StatesType::Idle) => true,
            (StatesType::Died, StatesType::Died) => true,
            _ => false,
        }
    }
}

impl StatesType{
    pub fn settings(&self) -> StatesSettings {
        match self {
            StatesType::Walking => StatesSettings{
                blacklist: vec![StatesType::Died,StatesType::Stunned],
                stopping: vec![StatesType::Idle,StatesType::Running],
                stop_all: false
            },
            StatesType::Running => StatesSettings{
                blacklist: vec![StatesType::Died,StatesType::Stunned],
                stopping: vec![StatesType::Idle,StatesType::Walking],
                stop_all: false
            },
            StatesType::Stunned => StatesSettings{
                blacklist: vec![StatesType::Died],
                stopping: vec![StatesType::Idle,StatesType::Walking,StatesType::Running],
                stop_all: false
            },
            StatesType::Idle => StatesSettings{
                blacklist: vec![StatesType::Died,StatesType::Stunned],
                stopping: vec![StatesType::Walking,StatesType::Running],
                stop_all: false
            },
            StatesType::Died => StatesSettings{
                blacklist: vec![],
                stopping: vec![],
                stop_all: true
            }
        }
    }

    pub fn stop(&self, mut commands: &mut Commands, entity: Entity, network_side: &NetworkSide, mut state_infos: &mut StatesInfos){
        match self {
            StatesType::Walking => {
                commands.queue(move |world: &mut World| {
                    let mut linear_velocity = world.get_mut::<LinearVelocity>(entity);

                    if let Some(mut linear_velocity) = linear_velocity {
                        //linear_velocity.x = 0.0;
                        //linear_velocity.y = 0.0;
                    }
                });
            }
            StatesType::Running => {}
            StatesType::Stunned => {}
            StatesType::Idle => {}
            StatesType::Died => {}
        }
    }

    pub fn apply(&self, mut commands: &mut Commands, entity: Entity, network_side: &NetworkSide, mut state_infos: &mut StatesInfos){
        if state_infos.applied {return;}

        match self {
            StatesType::Walking => {
                if network_side == &NetworkSide::Client {
                    state_infos.applied = true;

                    commands.queue(move |world: &mut World| {
                        let mut event_writer = world.get_resource_mut::<Events<PlayNewAnimation>>().unwrap();

                        event_writer.send(PlayNewAnimation(entity,"Walking".to_string()));
                    });
                }else if network_side == &NetworkSide::Server {

                }
            }
            StatesType::Running => {}
            StatesType::Stunned => {}
            StatesType::Idle => {
                if network_side == &NetworkSide::Client {
                    state_infos.applied = true;

                    commands.queue(move |world: &mut World| {
                        let mut event_writer = world.get_resource_mut::<Events<PlayNewAnimation>>().unwrap();

                        event_writer.send(PlayNewAnimation(entity,"Idle".to_string()));
                    });
                }else if network_side == &NetworkSide::Server {

                }
            }
            StatesType::Died => {}
        }
    }

    pub fn tick(&self, commands: &mut Commands, entity: Entity, network_side: &NetworkSide, mut state_infos: &mut StatesInfos){
        self.apply(commands, entity, network_side, &mut state_infos);

        match self {
            StatesType::Walking => {}
            StatesType::Running => {}
            StatesType::Stunned => {}
            StatesType::Idle => {}
            StatesType::Died => {}
        }
    }
}

impl Default for StatesInfos {
    fn default() -> Self {
        Self{
            start: match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(duration_since) => {Some(duration_since.as_millis())},
                Err(_) => {None}
            },
            duration: 0.0,
            source: Default::default(),
            in_cooldown: false,
            applied: false,
            values: None,
        }
    }
}

impl Default for StatesMachine{
    fn default() -> Self {
        Self{
            current_states: HashMap::from([
                (StatesType::Idle, StatesInfos::default())
            ]),
        }
    }
}

pub trait StatesMethods{
    fn can_execute(&self,new_state: StatesType) -> bool;

    fn execute(&mut self, commands: &mut Commands, entity: Entity, network_side: &NetworkSide, new_state: StatesType,mut states_infos: StatesInfos){}

    fn stop(&mut self, stop_state: &StatesType, commands: &mut Commands, entity: Entity, network_side: &NetworkSide, states_infos: &mut StatesInfos);

    fn tick(&mut self, commands: &mut Commands, entity: Entity, network_side: &NetworkSide) {

    }
}

impl StatesMethods for StatesMachine{
    fn can_execute(&self,new_state: StatesType) -> bool {
        let new_state_settings = new_state.settings();

        for (state_type,_) in &self.current_states {
            if new_state_settings.blacklist.contains(state_type) {
                return false
            }
        }

        true
    }

    fn execute(&mut self, commands: &mut Commands, entity: Entity, network_side: &NetworkSide, new_state: StatesType, mut states_infos: StatesInfos) {
        let new_state_settings = new_state.settings();
        let mut states_cancel_list: Vec<&StatesType> = vec![];
        let clone_states = self.current_states.clone();

        for (state_type,_) in &clone_states {
            if new_state_settings.stopping.contains(state_type) {
                states_cancel_list.push(state_type);
            }
        }

        if new_state_settings.stop_all {
            for (state_type,_) in &clone_states {
                states_cancel_list.push(state_type);
            }
        }

        for state in states_cancel_list {
            self.stop(state, commands, entity, network_side, &mut states_infos);
        }

        self.current_states.insert(new_state,states_infos);
    }

    fn stop(&mut self, stop_state: &StatesType, commands: &mut Commands, entity: Entity, network_side: &NetworkSide, states_infos: &mut StatesInfos) {
        stop_state.stop(commands,entity,network_side,states_infos);
        self.current_states.remove(stop_state);
    }

    fn tick(&mut self, commands: &mut Commands, entity: Entity, network_side: &NetworkSide) {
        for (state,state_infos) in &mut self.current_states {
            state.tick(commands, entity, network_side, state_infos);
        }
    }
}

