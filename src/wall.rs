use crate::SCALE;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const WALL_WIDTH: f32 = 0.5;

pub struct Wall;

pub struct SpawnWallEvent {
    pub start: Vec2,
    pub end: Vec2,
}

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<SpawnWallEvent>()
            .add_system(wall_spawner.system());
    }
}

fn wall_spawner(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut events: EventReader<SpawnWallEvent>,
) {
    for ev in events.iter() {
        let midpoint = (ev.start + ev.end) / 2.0;
        let length = ev.start.distance(ev.end);
        let angle = Vec2::X.angle_between(ev.end - ev.start);
        commands
            .spawn()
            .insert(Wall)
            .insert_bundle(ColliderBundle {
                shape: ColliderShape::cuboid(0.5 * length, 0.5 * WALL_WIDTH),
                position: (midpoint, angle).into(),
                ..Default::default()
            })
            .insert_bundle(SpriteBundle {
                sprite: Sprite::new(Vec2::new(length * SCALE, WALL_WIDTH * SCALE)),
                material: materials.add(Color::BLACK.into()),
                ..Default::default()
            })
            .insert(ColliderPositionSync::Discrete);
    }
}
