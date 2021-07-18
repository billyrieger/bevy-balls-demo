use bevy::prelude::*;
use rapier2d::prelude::*;

use crate::WallVertices;
use crate::HEIGHT;
use crate::WIDTH;

fn add_ball(
    rigid_body_set: &mut RigidBodySet,
    collider_set: &mut ColliderSet,
    position: Vec2,
    radius: f32,
) {
    let rigid_body = RigidBodyBuilder::new_dynamic()
        .translation(position.into())
        .build();
    let collider = ColliderBuilder::ball(radius)
        .restitution(0.9)
        .friction(1.0)
        .build();
    let ball_body_handle = rigid_body_set.insert(rigid_body);
    collider_set.insert_with_parent(collider, ball_body_handle, rigid_body_set);
}

pub(crate) fn run_simulation(wall_vertices: Res<WallVertices>) {
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    collider_set.insert(
        ColliderBuilder::polyline(
            wall_vertices.0.iter().copied().map(Point::from).collect(),
            None,
        )
        .friction(1.0)
        .build(),
    );

    let ball_center_left = Vec2::new(-0.35 * WIDTH, 0.46 * HEIGHT);
    let ball_center_right = Vec2::new(0.35 * WIDTH, 0.46 * HEIGHT);
    for r in -3..=3 {
        for c in -3..=3 {
            let offset = Vec2::new(c as f32, r as f32);
            add_ball(
                &mut rigid_body_set,
                &mut collider_set,
                ball_center_left + offset,
                0.5,
            );
            add_ball(
                &mut rigid_body_set,
                &mut collider_set,
                ball_center_right + offset,
                0.5,
            );
        }
    }

    /* Create other structures necessary for the simulation. */
    let gravity = vector![0.0, -9.81];
    let integration_parameters = IntegrationParameters::default();
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut joint_set = JointSet::new();
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = ();
    let event_handler = ();

    /* Run the game loop, stepping the simulation once per frame. */
    let mut steps = 0;
    loop {
        physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut rigid_body_set,
            &mut collider_set,
            &mut joint_set,
            &mut ccd_solver,
            &physics_hooks,
            &event_handler,
        );
        steps += 1;
        if island_manager.active_dynamic_bodies().is_empty() {
            println!("done simulating in {} steps!", steps);
            let mut translations = vec![];
            for (_, body) in rigid_body_set.iter() {
                translations.push(body.translation());
            }
            translations.sort_by_key(|pos| (pos.x * 1000000.0) as i32);
            println!("{:?}", &translations[..3]);
            break;
        }
    }
}
