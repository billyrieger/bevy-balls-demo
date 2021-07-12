use crate::ball::{Ball, BallPlugin, SpawnBallEvent};
use crate::wall::{SpawnWallEvent, WallPlugin};
use bevy::prelude::*;
use bevy_rapier2d::{physics::TimestepMode, prelude::*};

mod ball;
mod wall;

const SCALE: f32 = 20.0;
const WIDTH_PX: f32 = 1280.0;
const HEIGHT_PX: f32 = 720.0;
const WIDTH: f32 = WIDTH_PX / SCALE;
const HEIGHT: f32 = HEIGHT_PX / SCALE;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "bevy balls demo".to_string(),
            width: WIDTH_PX,
            height: HEIGHT_PX,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(BallPlugin)
        .add_plugin(WallPlugin)
        .add_event::<SetupPhysicsEvent>()
        .add_startup_system(setup.system())
        .add_system(setup_physics.system())
        .add_system(reset.system())
        .run();
}

fn reset(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    query: Query<Entity, With<Ball>>,
    mut events: EventWriter<SetupPhysicsEvent>,
) {
    if input.just_pressed(KeyCode::Space) {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
        events.send(SetupPhysicsEvent);
    }
}

fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut events: EventWriter<SetupPhysicsEvent>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    rapier_config.scale = SCALE;
    rapier_config.timestep_mode = TimestepMode::FixedTimestep;
    events.send(SetupPhysicsEvent);
}

struct SetupPhysicsEvent;

fn setup_physics(
    mut setup_events: EventReader<SetupPhysicsEvent>,
    mut ball_events: EventWriter<SpawnBallEvent>,
    mut wall_events: EventWriter<SpawnWallEvent>,
) {
    if setup_events.iter().count() > 0 {
        let polyline = vec![
            [-0.45 * WIDTH, 0.45 * HEIGHT],
            [-0.45 * WIDTH, 0.25 * HEIGHT],
            [-0.10 * WIDTH, -0.25 * HEIGHT],
            [-0.10 * WIDTH, -0.45 * HEIGHT],
            [0.10 * WIDTH, -0.45 * HEIGHT],
            [0.10 * WIDTH, -0.25 * HEIGHT],
            [0.45 * WIDTH, 0.25 * HEIGHT],
            [0.45 * WIDTH, 0.45 * HEIGHT],
        ];
        for points in polyline.windows(2) {
            wall_events.send(SpawnWallEvent {
                start: points[0].into(),
                end: points[1].into(),
            });
        }

        let ball_center_left = Vec2::new(-0.35 * WIDTH, 0.35 * HEIGHT);
        let ball_center_right = Vec2::new(0.35 * WIDTH, 0.35 * HEIGHT);
        for r in -3..=3 {
            for c in -3..=3 {
                let offset = Vec2::new(c as f32, r as f32);
                ball_events.send(SpawnBallEvent {
                    position: ball_center_left + offset,
                    radius: 0.5,
                });
                ball_events.send(SpawnBallEvent {
                    position: ball_center_right + offset,
                    radius: 0.5,
                });
            }
        }
    }
}
