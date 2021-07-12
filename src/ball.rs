use crate::SCALE;
use bevy::{
    prelude::*,
    render::{mesh::Indices, pipeline::PrimitiveTopology},
};
use bevy_rapier2d::prelude::*;

pub struct Ball;

pub struct SpawnBallEvent {
    pub position: Vec2,
    pub radius: f32,
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<SpawnBallEvent>()
            .add_system(ball_spawner.system());
    }
}

fn ball_spawner(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut events: EventReader<SpawnBallEvent>,
) {
    for ev in events.iter() {
        commands
            .spawn()
            .insert(Ball)
            .insert_bundle(SpriteBundle {
                sprite: Sprite::new(Vec2::new(2.0 * ev.radius * SCALE, 2.0 * ev.radius * SCALE)),
                mesh: meshes.add(create_mesh(100)),
                material: materials.add(assets_server.load("goat.png").into()),
                ..Default::default()
            })
            .insert_bundle(RigidBodyBundle {
                position: ev.position.into(),
                ..Default::default()
            })
            .insert_bundle(ColliderBundle {
                shape: ColliderShape::ball(ev.radius),
                material: ColliderMaterial {
                    restitution: 0.9,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(RigidBodyPositionSync::Discrete);
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
