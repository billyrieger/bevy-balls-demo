use crate::SCALE;
use bevy::{
    prelude::*,
    render::{mesh::Indices, pipeline::PrimitiveTopology},
};
use bevy_rapier2d::prelude::*;

struct Ball(u32);

#[derive(Bundle)]
struct BallBundle {
    ball: Ball,
    #[bundle]
    sprite: SpriteBundle,
    #[bundle]
    rigid_body: RigidBodyBundle,
    #[bundle]
    collider: ColliderBundle,
    position_sync: RigidBodyPositionSync,
}

fn create_mesh(n: u32) -> Mesh {
    let mut vertex_positions = vec![[0.0, 0.0, 0.0]];
    let mut vertex_normals = vec![[0.0, 0.0, 1.0]];
    let mut vertex_uv_0s = vec![[0.5, 0.5]];
    let mut triangle_indices = vec![];
    for i in 0..n {
        let theta = (i as f32) * std::f32::consts::TAU / (n as f32);
        let x = theta.cos();
        let y = theta.sin();
        vertex_positions.push([x, y, 0.0]);
        vertex_normals.push([0.0, 0.0, 1.0]);
        vertex_uv_0s.push([(x + 1.0) / 2.0, 1.0 - (y + 1.0) / 2.0]);
        triangle_indices.extend([0, i + 1, (i + 1) % n + 1]);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vertex_positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, vertex_normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, vertex_uv_0s);
    mesh.set_indices(Some(Indices::U32(triangle_indices)));

    mesh
}

pub fn spawn_ball(
    commands: &mut Commands,
    assets_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    radius: f32,
) {
    commands.spawn_bundle(BallBundle {
        ball: Ball(0),
        sprite: SpriteBundle {
            sprite: Sprite::new(Vec2::new(radius * SCALE, radius * SCALE)),
            mesh: meshes.add(create_mesh(100)),
            material: materials.add(assets_server.load("goat.png").into()),
            ..Default::default()
        },
        rigid_body: RigidBodyBundle {
            position: Vec2::new(0.0, 10.0).into(),
            ..Default::default()
        },
        collider: ColliderBundle {
            shape: ColliderShape::ball(radius),
            material: ColliderMaterial {
                restitution: 0.7,
                ..Default::default()
            },
            ..Default::default()
        },
        position_sync: RigidBodyPositionSync::Discrete,
    });
}
