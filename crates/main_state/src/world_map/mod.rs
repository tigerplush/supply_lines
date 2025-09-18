use bevy::prelude::*;

use crate::main_states::MainStateSubStates;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(MainStateSubStates::WorldMap), setup);
}

struct CityNode {
    pos: Vec2,
    name: String,
}

impl CityNode {
    const fn new(pos: Vec2, name: String) -> Self {
        CityNode { pos, name }
    }
}

#[derive(Component)]
enum CityType {
    Capital,
    Undeveloped,
    Developed,
}

const TILE_SIZE: f32 = 16.0;

fn setup(
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 16, 16, None, None);
    let handle = layouts.add(layout);

    let nodes = [
        (CityNode::new(Vec2::ZERO, "Capital".to_string()), CityType::Capital),
        (CityNode::new(Vec2::new(10.0, 10.0), "Random Name".to_string()), CityType::Undeveloped),
        (CityNode::new(Vec2::new(10.0, -10.0), "Random Name".to_string()), CityType::Undeveloped),
    ];

    for (node, city_type) in nodes {
        commands
            .spawn((
                Name::from(node.name),
                Transform::from_translation(node.pos.extend(0.0) * TILE_SIZE),
                Sprite {
                    image: asset_server.load("sprites/MasterSimple.png"),
                    texture_atlas: TextureAtlas {
                        layout: handle.clone(),
                        index: 10,
                    }
                    .into(),
                    ..default()
                },
                city_type,
                Pickable::default(),
                DespawnOnExit(MainStateSubStates::WorldMap),
            ))
            .observe(on_released);
    }

    commands.spawn((Camera2d, DespawnOnExit(MainStateSubStates::WorldMap)));
}

fn on_released(trigger: On<Pointer<Release>>, mut next_state: ResMut<NextState<MainStateSubStates>>, query: Query<&CityType>) {
    let Ok(city_type) = query.get(trigger.entity) else {
        return;
    };

    let next = match city_type {
        CityType::Capital => MainStateSubStates::Capital,
        CityType::Undeveloped => MainStateSubStates::City,
        CityType::Developed => return,
    };

    next_state.set(next);
}
