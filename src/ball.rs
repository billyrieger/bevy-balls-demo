use crate::{
    run_simulation::{InitialConditions, SimulationResult},
    SCALE,
};

use bevy::{
    prelude::*,
    render::{mesh::Indices, pipeline::PrimitiveTopology},
};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct Ball {
    rotation_offset: f32,
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(ball_spawner.system().label("ball_spawner"))
            .add_system(sync_transforms.system().after("ball_spawner"));
    }
}

fn ball_spawner(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    assets_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    intial: Res<InitialConditions>,
    result: Res<SimulationResult>,
) {
    if input.just_pressed(KeyCode::Space) {
        for (i, &position) in intial.ball_positions.iter().enumerate() {
            let final_position: Vec2 = result.positions[&i].translation.into();
            let final_rotation: f32 = result.positions[&i].rotation.angle();
            let scaled_center = (final_position - intial.box_lower_left) / intial.box_size;
            let scaled_radius = intial.ball_radius / intial.box_size;
            commands
                .spawn()
                .insert(Ball {
                    rotation_offset: -final_rotation,
                })
                .insert_bundle(SpriteBundle {
                    sprite: Sprite::new(Vec2::splat(2.0 * intial.ball_radius * SCALE)),
                    mesh: meshes.add(create_mesh(20, scaled_center, scaled_radius)),
                    material: materials.add(assets_server.load("icon.png").into()),
                    ..Default::default()
                })
                .insert_bundle(RigidBodyBundle {
                    position: position.into(),
                    ccd: RigidBodyCcd {
                        ccd_enabled: true,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert_bundle(ColliderBundle {
                    shape: ColliderShape::ball(intial.ball_radius),
                    material: ColliderMaterial {
                        friction: intial.ball_friction,
                        restitution: intial.ball_restitution,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(GeometryBuilder::build_as(
                        &shapes::Circle {
                            radius: intial.ball_radius * SCALE,
                            ..Default::default()
                        },
                        ShapeColors::new(Color::BLACK),
                        DrawMode::Stroke(StrokeOptions::default()),
                        Transform::from_xyz(0.0, 0.0, 1.0),
                    ));
                });
        }
    }
}

fn sync_transforms(mut query: Query<(&Ball, &mut Transform, &RigidBodyPosition)>) {
    for (ball, mut transform, body_position) in query.iter_mut() {
        let x = body_position.position.translation.x;
        let y = body_position.position.translation.y;
        transform.translation = Vec3::new(x, y, 0.0) * SCALE;
        transform.rotation =
            Quat::from_rotation_z(body_position.position.rotation.angle() + ball.rotation_offset);
    }
}

fn create_mesh(n_vertices: u32, center: Vec2, radius: f32) -> Mesh {
    // The coordinates of the vertices in 3D space. The first vertex is the
    // origin, and the rest of the vertices form a circle in the XY plane
    // centered at the origin with diameter 1.
    let mut vertex_positions = vec![[0.0, 0.0, 0.0]];

    // The positions of the vertices on the texture, which are scaled to be
    // between 0 and 1. Like above, the first vertex is the center and the rest
    // form the circle. We have to flip the y-coordinate because the y-axis for
    // textures points down.
    let mut vertex_uv_0s = vec![[center.x, 1.0 - center.y]];

    // The list of triangles that make up the mesh. Each triangle will have one
    // vertex at the center of the circle and the other two on the perimeter.
    let mut triangle_indices = vec![];

    // Go around the circle and add the vertices.
    for i in 1..=n_vertices {
        let theta = (i as f32) * std::f32::consts::TAU / (n_vertices as f32);
        vertex_positions.push([0.5 * theta.cos(), 0.5 * theta.sin(), 0.0]);
        vertex_uv_0s.push([
            center.x + radius * theta.cos(),
            1.0 - (center.y + radius * theta.sin()),
        ]);
        triangle_indices.extend([0, i, i % n_vertices + 1]);
    }

    // We can now create the mesh.
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vertex_positions);
    // Since the circle is in the xy plane, all of the normal vectors point in
    // the z direction.
    mesh.set_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![[0.0, 0.0, 1.0]; n_vertices as usize + 1],
    );
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, vertex_uv_0s);
    mesh.set_indices(Some(Indices::U32(triangle_indices)));

    mesh
}
