mod ball;
mod simulation;
mod wall;

use ball::BallPlugin;
use simulation::SimulationPlugin;
use wall::WallPlugin;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::physics::TimestepMode;
use bevy_rapier2d::prelude::*;

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
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(ShapePlugin)
        .add_plugin(BallPlugin)
        .add_plugin(WallPlugin)
        .add_plugin(SimulationPlugin)
        .add_startup_system(setup.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

fn setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    rapier_config.scale = SCALE;
    rapier_config.timestep_mode = TimestepMode::FixedTimestep;
}
