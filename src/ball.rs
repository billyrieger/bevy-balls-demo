use crate::{systems::run_simulation::SimulationResult, SCALE};
use bevy::{
    prelude::*,
    render::{mesh::Indices, pipeline::PrimitiveTopology},
};
use bevy_rapier2d::prelude::*;

pub struct Ball {
    index: u32,
    rotation_offset: f32,
}
pub struct BallCounter(u32);

pub struct Asleep(bool);

pub struct StartAsleepCheck(Timer);

pub struct SpawnBallEvent {
    pub position: Vec2,
    pub radius: f32,
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Asleep(false))
            .insert_resource(BallCounter(0))
            .insert_resource(StartAsleepCheck(Timer::from_seconds(5.0, false)))
            .add_event::<SpawnBallEvent>()
            .add_system(ball_spawner.system().label("ball_spawner"))
            .add_system(sync_transforms.system().after("ball_spawner"))
            .add_system(check_sleeping.system().after("ball_spawner"));
    }
}

fn sync_transforms(mut query: Query<(&Ball, &mut Transform, &RigidBodyPosition)>) {
    for (ball, mut transform, body_position) in query.iter_mut() {
        let x = body_position.position.translation.x;
        let y = body_position.position.translation.y;
        transform.translation = Vec3::new(x, y, 0.0) * SCALE;
        transform.rotation = Quat::from_rotation_z(body_position.position.rotation.angle() + ball.rotation_offset);
    }
}

fn ball_spawner(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut events: EventReader<SpawnBallEvent>,
    mut counter: ResMut<BallCounter>,
    atlas_handle: Res<Handle<TextureAtlas>>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    sim_result: Res<SimulationResult>,
) {
    let lower_left: Vec2 = Vec2::new(-5.0, -17.0);
    let box_size: f32 = 10.0;
    let texture_size: f32 = 256.0;
    let scaled_ball_size: Vec2 = Vec2::splat(0.6 * SCALE);

    for ev in events.iter() {
        let position = sim_result.positions.get(&counter.0).unwrap();
        let scaled = (Vec2::from(position.translation) - lower_left) * texture_size / box_size;
        atlases
            .get_mut(&*atlas_handle)
            .unwrap()
            .add_texture(bevy::sprite::Rect {
                min: scaled - scaled_ball_size / 2.0,
                max: scaled + scaled_ball_size / 2.0,
            });
        commands
            .spawn()
            .insert(Ball {
                index: counter.0,
                rotation_offset: -position.rotation.angle(),
            })
            .insert_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: counter.0,
                    ..Default::default()
                },
                texture_atlas: atlas_handle.clone(),
                mesh: meshes.add(create_mesh(20)),
                ..Default::default()
            })
            .insert_bundle(RigidBodyBundle {
                position: ev.position.into(),
                ccd: RigidBodyCcd {
                    ccd_enabled: true,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert_bundle(ColliderBundle {
                shape: ColliderShape::ball(ev.radius),
                material: ColliderMaterial {
                    restitution: 0.9,
                    ..Default::default()
                },
                ..Default::default()
            });
        counter.0 += 1;
    }
}

fn check_sleeping(
    time: Res<Time>,
    mut timer: ResMut<StartAsleepCheck>,
    mut all_asleep: ResMut<Asleep>,
    query: Query<(&RigidBodyActivation, &RigidBodyPosition), With<Ball>>,
) {
    if timer.0.tick(time.delta()).finished() && !all_asleep.0 {
        for (activation, _) in query.iter() {
            if !activation.sleeping {
                return;
            }
        }
        all_asleep.0 = true;
        let mut positions = vec![];
        for (_, position) in query.iter() {
            positions.push(position.position.translation);
        }
        positions.sort_by_key(|pos| (pos.x * 1000000.0) as i32);
        println!("{:?}", &positions[..3]);
        println!("asleep!");
    }
}

fn create_mesh(n: u32) -> Mesh {
    let zhat = [0.0, 0.0, 1.0];
    let mut vertex_positions = vec![[0.0, 0.0, 0.0]];
    let mut vertex_normals = vec![zhat];
    let mut vertex_uv_0s = vec![[0.5, 0.5]];
    let mut triangle_indices = vec![];
    for i in 1..=n {
        let theta = (i as f32) * std::f32::consts::TAU / (n as f32);
        let (x, y) = (theta.cos(), theta.sin());
        vertex_positions.push([0.5 * x, 0.5 * y, 0.0]);
        vertex_normals.push(zhat);
        vertex_uv_0s.push([0.5 * (x + 1.0), 1.0 - 0.5 * (y + 1.0)]);
        triangle_indices.extend([0, i, i % n + 1]);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vertex_positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, vertex_normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, vertex_uv_0s);
    mesh.set_indices(Some(Indices::U32(triangle_indices)));

    mesh
}
