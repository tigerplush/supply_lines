use bevy::prelude::*;
use states::AppState;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, SubStates)]
#[source(AppState = AppState::MainState)]
pub(crate) enum MainStateSubStates {
    #[default]
    WorldMap,
    Capital,
    EmbarkationMenu,
    City,
}
