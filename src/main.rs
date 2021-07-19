use ball::BallPlugin;
use run_simulation::{run_simulation, InitialConditions};

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::{physics::TimestepMode, prelude::*};

mod ball;
mod run_simulation;

const SCALE: f32 = 20.0;
const WIDTH: f32 = 64.0;
const HEIGHT: f32 = 36.0;
const WIDTH_PX: f32 = WIDTH * SCALE;
const HEIGHT_PX: f32 = HEIGHT * SCALE;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 8 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.3, 0.4)))
        .insert_resource(WindowDescriptor {
            title: "bevy balls demo".to_string(),
            width: WIDTH_PX,
            height: HEIGHT_PX,
            vsync: true,
            ..Default::default()
        })
        .init_resource::<InitialConditions>()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(ShapePlugin)
        .add_plugin(BallPlugin)
        .add_startup_system(setup.system().label("setup"))
        .add_startup_system(spawn_wall.system())
        .add_startup_system(run_simulation.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

fn setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    rapier_config.scale = SCALE;
    rapier_config.timestep_mode = TimestepMode::FixedTimestep;
}

fn spawn_wall(mut commands: Commands, initial_conditions: Res<InitialConditions>) {
    let mut path_builder = PathBuilder::new();
    path_builder.move_to(SCALE * Vec2::from(initial_conditions.wall_vertices[0]));
    for &point in initial_conditions.wall_vertices.iter().skip(1) {
        path_builder.line_to(SCALE * Vec2::from(point));
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
                initial_conditions
                    .wall_vertices
                    .iter()
                    .copied()
                    .map(Point::from)
                    .collect(),
                None,
            ),
            material: ColliderMaterial {
                friction: initial_conditions.wall_friction,
                restitution: initial_conditions.wall_resitution,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete);
}
