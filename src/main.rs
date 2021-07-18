use crate::ball::{BallPlugin, SpawnBallEvent};

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::{physics::TimestepMode, prelude::*};

mod ball;
mod wall;

mod systems;

const SCALE: f32 = 20.0;
const WIDTH: f32 = 64.0;
const HEIGHT: f32 = 36.0;
const WIDTH_PX: f32 = WIDTH * SCALE;
const HEIGHT_PX: f32 = HEIGHT * SCALE;

struct WallVertices(Vec<Vec2>);

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 8 })
        .insert_resource(WindowDescriptor {
            title: "bevy balls demo".to_string(),
            width: WIDTH_PX,
            height: HEIGHT_PX,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(WallVertices(
            vec![
                [-30.0, 17.0],
                [-30.0, 10.0],
                [-5.0, -7.0],
                [-5.0, -17.0],
                [5.0, -17.0],
                [5.0, -7.0],
                [30.0, 10.0],
                [30.0, 17.0],
            ]
            .into_iter()
            .map(Vec2::from)
            .collect(),
        ))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(BallPlugin)
        .add_plugin(ShapePlugin)
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
    wall_vertices: Res<WallVertices>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    rapier_config.scale = SCALE;
    rapier_config.timestep_mode = TimestepMode::FixedTimestep;

    let mut path_builder = PathBuilder::new();
    path_builder.move_to(SCALE * wall_vertices.0[0]);
    for &point in wall_vertices.0.iter().skip(1) {
        path_builder.line_to(SCALE * point);
    }
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &path_builder.build(),
            ShapeColors::new(Color::BLACK),
            DrawMode::Stroke(StrokeOptions::default()),
            Transform::default(),
        ))
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::polyline(
                wall_vertices.0.iter().copied().map(Point::from).collect(),
                None,
            ),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete);
}

struct SetupPhysicsEvent;

fn spawn_balls(input: Res<Input<KeyCode>>, mut ball_events: EventWriter<SpawnBallEvent>) {
    if input.just_pressed(KeyCode::Space) {
        let ball_center_left = Vec2::new(-0.35 * WIDTH, 0.47 * HEIGHT);
        let ball_center_right = Vec2::new(0.35 * WIDTH, 0.47 * HEIGHT);
        for r in -5..=5 {
            for c in -6..=6 {
                let offset = 0.6 * Vec2::new(c as f32, r as f32);
                ball_events.send(SpawnBallEvent {
                    position: ball_center_left + offset,
                    radius: 0.3,
                });
                ball_events.send(SpawnBallEvent {
                    position: ball_center_right + offset,
                    radius: 0.3,
                });
            }
        }
    }
}
