use crate::ball::{BallPlugin, SpawnBallEvent};
use crate::wall::{SpawnWallEvent, WallPlugin};
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_rapier2d::{physics::TimestepMode, prelude::*};

mod ball;
mod wall;

mod systems;

const SCALE: f32 = 20.0;
const WIDTH: f32 = 64.0;
const HEIGHT: f32 = 36.0;
const WIDTH_PX: f32 = WIDTH * SCALE;
const HEIGHT_PX: f32 = HEIGHT * SCALE;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 8 })
        .insert_resource(WindowDescriptor {
            title: "bevy balls demo".to_string(),
            width: WIDTH_PX,
            height: HEIGHT_PX,
            mode: WindowMode::BorderlessFullscreen,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(BallPlugin)
        .add_plugin(WallPlugin)
        .add_event::<SetupPhysicsEvent>()
        .add_startup_system(setup.system())
        .add_startup_system(systems::run_simulation::run_simulation.system())
        .add_system(spawn_balls.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut wall_events: EventWriter<SpawnWallEvent>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    rapier_config.scale = SCALE;
    rapier_config.timestep_mode = TimestepMode::FixedTimestep;
    let polyline = vec![
        [-0.45 * WIDTH, 0.45 * HEIGHT],
        [-0.45 * WIDTH, 0.25 * HEIGHT],
        [-5.0, -5.0],
        [-5.0, -15.0],
        [5.0, -15.0],
        [5.0, -5.0],
        [0.45 * WIDTH, 0.25 * HEIGHT],
        [0.45 * WIDTH, 0.45 * HEIGHT],
    ];
    for points in polyline.windows(2) {
        wall_events.send(SpawnWallEvent {
            start: points[0].into(),
            end: points[1].into(),
        });
    }
}

struct SetupPhysicsEvent;

fn spawn_balls(input: Res<Input<KeyCode>>, mut ball_events: EventWriter<SpawnBallEvent>) {
    if input.just_pressed(KeyCode::Space) {
        let ball_center_left = Vec2::new(-0.35 * WIDTH, 0.46 * HEIGHT);
        let ball_center_right = Vec2::new(0.35 * WIDTH, 0.46 * HEIGHT);
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
