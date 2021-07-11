use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod ball;

const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 480.0;
const SCALE: f32 = 20.0;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "bevy balls demo".to_string(),
            width: WIDTH,
            height: HEIGHT,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_system(setup.system())
        .add_startup_system(setup_physics.system())
        .add_system(print_ball_altitude.system())
        .run();
}

fn setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    rapier_config.scale = SCALE;
}

fn setup_physics(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let ground = ColliderBundle {
        shape: ColliderShape::cuboid(WIDTH / 2.0 / SCALE, 5.0 / SCALE),
        ..Default::default()
    };
    commands.spawn_bundle(ground).insert_bundle(SpriteBundle {
        sprite: Sprite::new(Vec2::new(WIDTH, 10.0)),
        material: materials.add(Color::BLACK.into()),
        ..Default::default()
    });

    /* Create the bouncing ball. */
    ball::spawn_ball(
        &mut commands,
        &assets_server,
        &mut meshes,
        &mut materials,
        3.0,
    );
}

fn print_ball_altitude(positions: Query<&RigidBodyPosition>) {
    for rb_pos in positions.iter() {
        println!("Ball altitude: {}", rb_pos.position.translation.vector.y);
    }
}
