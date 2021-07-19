use bevy::utils::HashMap;

use bevy::prelude::*;
use rapier2d::prelude::*;

use crate::SimulationSetup;

pub struct SimulationResult {
    pub positions: HashMap<u32, Isometry<f32>>,
}

fn add_ball(
    rigid_body_set: &mut RigidBodySet,
    collider_set: &mut ColliderSet,
    position: Vec2,
    radius: f32,
    n: u128,
) {
    let rigid_body = RigidBodyBuilder::new_dynamic()
        .translation(position.into())
        .ccd_enabled(true)
        .user_data(n)
        .build();
    let collider = ColliderBuilder::ball(radius)
        .restitution(0.9)
        .friction(1.0)
        .build();
    let ball_body_handle = rigid_body_set.insert(rigid_body);
    collider_set.insert_with_parent(collider, ball_body_handle, rigid_body_set);
}

pub(crate) fn run_simulation(
    mut commands: Commands,
    sim_setup: Res<SimulationSetup>,
) {
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    collider_set.insert(
        ColliderBuilder::polyline(
            sim_setup
                .wall_vertices
                .iter()
                .copied()
                .map(Point::from)
                .collect(),
            None,
        )
        .friction(1.0)
        .build(),
    );

    let mut n = 0;
    for &pos in &sim_setup.ball_start_positions {
        add_ball(
            &mut rigid_body_set,
            &mut collider_set,
            pos,
            sim_setup.ball_radius,
            n,
        );
        n += 1;
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

    let mut sim_result = SimulationResult { positions: HashMap::default() };

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
            for (_, rigid_body) in rigid_body_set.iter() {
                sim_result
                    .positions
                    .insert(rigid_body.user_data as u32, *rigid_body.position());
            }
            break;
        }
    }

    commands.insert_resource(sim_result);
}
