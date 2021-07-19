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

struct SimulationSetup {
    wall_vertices: Vec<Vec2>,
    ball_start_positions: Vec<Vec2>,
    ball_radius: f32,
}

impl Default for SimulationSetup {
    fn default() -> Self {
        let wall_vertices = vec![
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
        .collect();

        let ball_center_left = Vec2::new(-25.0, 16.0);
        let ball_center_right = Vec2::new(25.0, 16.0);
        let mut ball_start_positions = vec![];
        for r in -5..=5 {
            for c in -6..=6 {
                let offset = 0.6 * Vec2::new(c as f32, r as f32);
                ball_start_positions.push(ball_center_left + offset);
                ball_start_positions.push(ball_center_right + offset);
            }
        }

        Self {
            wall_vertices,
            ball_start_positions,
            ball_radius: 0.3,
        }
    }
}

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
        .init_resource::<SimulationSetup>()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(BallPlugin)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup.system().label("setup"))
        .add_startup_system(systems::run_simulation::run_simulation.system().after("setup"))
        .add_system(spawn_balls.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    sim_setup: Res<SimulationSetup>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    rapier_config.scale = SCALE;
    rapier_config.timestep_mode = TimestepMode::FixedTimestep;

    let mut path_builder = PathBuilder::new();
    path_builder.move_to(SCALE * sim_setup.wall_vertices[0]);
    for &point in sim_setup.wall_vertices.iter().skip(1) {
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
                sim_setup
                    .wall_vertices
                    .iter()
                    .copied()
                    .map(Point::from)
                    .collect(),
                None,
            ),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete);
}

fn spawn_balls(
    input: Res<Input<KeyCode>>,
    sim_setup: Res<SimulationSetup>,
    mut ball_events: EventWriter<SpawnBallEvent>,
) {
    if input.just_pressed(KeyCode::Space) {
        for &position in &sim_setup.ball_start_positions {
            ball_events.send(SpawnBallEvent {
                position,
                radius: 0.3,
            });
        }
    }
}
