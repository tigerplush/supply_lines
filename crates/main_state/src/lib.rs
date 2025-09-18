use bevy::prelude::*;

use crate::main_states::MainStateSubStates;

mod main_states;

mod world_map;

#[derive(Default)]
pub struct MainStatePlugin;

impl Plugin for MainStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<MainStateSubStates>()
            .add_plugins(world_map::plugin);
    }
}
