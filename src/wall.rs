use crate::simulation::InitialConditions;
use crate::SCALE;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_wall.system());
    }
}

struct Wall;

fn spawn_wall(mut commands: Commands, initial_conditions: Res<InitialConditions>) {
    // Make the SVG path for the wall by moving to the first point and drawing a
    // line to the rest of the points in order.
    let mut path_builder = PathBuilder::new();
    path_builder.move_to(Vec2::from(initial_conditions.wall_vertices[0]) * SCALE);
    for &point in initial_conditions.wall_vertices.iter().skip(1) {
        path_builder.line_to(Vec2::from(point) * SCALE);
    }

    commands
        .spawn()
        .insert(Wall)
        .insert_bundle(GeometryBuilder::build_as(
            &path_builder.build(),
            ShapeColors::new(Color::BLACK),
            DrawMode::Stroke(StrokeOptions::default()),
            Transform::default(),
        ))
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::polyline(initial_conditions.wall_vertices.clone(), None),
            material: ColliderMaterial {
                friction: initial_conditions.wall_friction,
                restitution: initial_conditions.wall_resitution,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete);
}
