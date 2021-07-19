use crate::simulation::{InitialConditions, SimulationResult};
use crate::SCALE;

use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::pipeline::PrimitiveTopology;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(BallsHaveSpawned(false))
            .add_system(spawn_balls.system());
    }
}

struct Ball;

struct BallsHaveSpawned(bool);

fn spawn_balls(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    assets_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    intial_conditions: Res<InitialConditions>,
    simulation_result: Res<SimulationResult>,
    mut balls_have_spawned: ResMut<BallsHaveSpawned>,
) {
    if input.just_pressed(KeyCode::Space) && !balls_have_spawned.0 {
        // Only spawn the balls once.
        balls_have_spawned.0 = true;

        for (i, &position) in intial_conditions.ball_positions.iter().enumerate() {
            let final_position = Vec2::from(simulation_result.positions[&i].translation);
            let final_rotation = simulation_result.positions[&i].rotation.angle();
            // Scale the final position of the ball to UV coordinates.
            let scaled_center =
                (final_position - intial_conditions.box_lower_left) / intial_conditions.box_size;
            let scaled_radius = intial_conditions.ball_radius / intial_conditions.box_size;
            commands
                .spawn()
                .insert(Ball)
                .insert_bundle(SpriteBundle {
                    sprite: Sprite::new(Vec2::splat(2.0 * intial_conditions.ball_radius * SCALE)),
                    mesh: meshes.add(create_mesh(
                        20,
                        scaled_center,
                        scaled_radius,
                        final_rotation,
                    )),
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
                    shape: ColliderShape::ball(intial_conditions.ball_radius),
                    material: ColliderMaterial {
                        friction: intial_conditions.ball_friction,
                        restitution: intial_conditions.ball_restitution,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(RigidBodyPositionSync::Discrete)
                // Add an outline to each ball as a child entity.
                .with_children(|parent| {
                    parent.spawn_bundle(GeometryBuilder::build_as(
                        &shapes::Circle {
                            radius: intial_conditions.ball_radius * SCALE,
                            ..Default::default()
                        },
                        ShapeColors::new(Color::BLACK),
                        DrawMode::Stroke(StrokeOptions::default()),
                        // Make sure it appears above the ball.
                        Transform::from_xyz(0.0, 0.0, 1.0),
                    ));
                });
        }
    }
}

// Creates a circular mesh at the specified texture coordinates.
fn create_mesh(n_vertices: u32, center: Vec2, radius: f32, angle: f32) -> Mesh {
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

    // Go around the circle and add the circle vertices.
    for i in 1..=n_vertices {
        let theta = (i as f32) * std::f32::consts::TAU / (n_vertices as f32);
        vertex_positions.push([0.5 * theta.cos(), 0.5 * theta.sin(), 0.0]);
        // The UV vertices need to be additionally rotated by the angle
        // calculated from the physics simulation.
        vertex_uv_0s.push([
            center.x + radius * (theta + angle).cos(),
            1.0 - (center.y + radius * (theta + angle).sin()),
        ]);
        triangle_indices.extend([0, i, i % n_vertices + 1]);
    }

    // We can now create the mesh!
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vertex_positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, vertex_uv_0s);
    mesh.set_indices(Some(Indices::U32(triangle_indices)));
    // Since the circle is in the xy plane, all of the normal vectors point in
    // the z direction.
    mesh.set_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![[0.0, 0.0, 1.0]; n_vertices as usize + 1],
    );

    mesh
}
