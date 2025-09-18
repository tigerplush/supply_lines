use bevy::prelude::*;
use states::AppState;

#[derive(Default)]
pub struct SplashscreenPlugin;

impl Plugin for SplashscreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Splashscreen), setup)
            .add_systems(Update, update.run_if(in_state(AppState::Splashscreen)));
    }
}

#[derive(Component, Deref, DerefMut)]
struct FadeTime(Timer);

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        ImageNode {
            image: asset_server.load("ui/splashscreen.png"),
            ..default()
        },
        DespawnOnExit(AppState::Splashscreen),
        FadeTime(Timer::from_seconds(2.0, TimerMode::Once)),
    ));
    commands.spawn((Camera2d, DespawnOnExit(AppState::Splashscreen)));
}

fn update(
    time: ResMut<Time>,
    mut next_state: ResMut<NextState<AppState>>,
    mut query: Single<&mut FadeTime>,
) {
    query.tick(time.delta());
    if query.is_finished() {
        next_state.set(AppState::MainState);
    }
}
