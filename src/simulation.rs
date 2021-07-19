use bevy::prelude::*;
use bevy::utils::HashMap;
use rapier2d::prelude::*;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<InitialConditions>()
            .add_startup_system(run_simulation.system());
    }
}

// The initial conditions of the physics simulation.
pub struct InitialConditions {
    pub wall_vertices: Vec<Point<f32>>,
    pub wall_friction: f32,
    pub wall_resitution: f32,
    pub ball_positions: Vec<Vec2>,
    pub ball_radius: f32,
    pub ball_friction: f32,
    pub ball_restitution: f32,
    // The size of the square box where the texture will be shown.
    pub box_size: f32,
    // The lower-left point of the texture box.
    pub box_lower_left: Vec2,
}

impl Default for InitialConditions {
    fn default() -> Self {
        let wall_vertices = vec![
            [-30.0, 17.0],
            [-30.0, 10.0],
            [-5.0, -7.0],
            [-5.0, -17.0],
            [5.0, -17.0],
            [5.0, -7.0],
            [30.0, 10.0],
            [30.0, 17.0],
        ]
        .into_iter()
        .map(Point::from)
        .collect();

        let ball_radius = 0.24;
        let center_left = Vec2::new(-25.0, 14.5);
        let center_right = Vec2::new(25.0, 14.5);
        let mut ball_positions = vec![];
        for r in -7..=7 {
            for c in -7..=7 {
                let (r, c) = (r as f32, c as f32);
                let offset_left = 2.0 * ball_radius * Vec2::new(-c, r);
                let offset_right = 2.0 * ball_radius * Vec2::new(c, r);
                ball_positions.push(center_left + offset_left);
                ball_positions.push(center_right + offset_right);
            }
        }

        Self {
            wall_vertices,
            wall_friction: 0.5,
            wall_resitution: 0.6,
            ball_positions,
            ball_radius,
            ball_friction: 0.9,
            ball_restitution: 0.9,
            box_size: 10.0,
            box_lower_left: Vec2::new(-5.0, -17.0),
        }
    }
}

// The final result of the physics simulation.
pub struct SimulationResult {
    // The resting position of each ball.
    pub positions: HashMap<usize, Isometry<f32>>,
}

pub fn run_simulation(mut commands: Commands, initial_conditions: Res<InitialConditions>) {
    // Set up the physics simulation.
    let gravity = vector![0.0, -9.81];
    let integration_parameters = IntegrationParameters::default();
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut rigid_bodies = RigidBodySet::new();
    let mut colliders = ColliderSet::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut joint_set = JointSet::new();
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = ();
    let event_handler = ();

    // Add the wall to the simulation.
    colliders.insert(
        ColliderBuilder::polyline(initial_conditions.wall_vertices.clone(), None)
            .friction(initial_conditions.wall_friction)
            .restitution(initial_conditions.wall_resitution)
            .build(),
    );

    // Add the balls to the simulation.
    for (i, &position) in initial_conditions.ball_positions.iter().enumerate() {
        let rigid_body = RigidBodyBuilder::new_dynamic()
            .translation(position.into())
            .ccd_enabled(true)
            // The user data allows us to track individual balls.
            .user_data(i as u128)
            .build();
        let collider = ColliderBuilder::ball(initial_conditions.ball_radius)
            .friction(initial_conditions.ball_friction)
            .restitution(initial_conditions.ball_restitution)
            .build();
        let ball_body_handle = rigid_bodies.insert(rigid_body);
        colliders.insert_with_parent(collider, ball_body_handle, &mut rigid_bodies);
    }

    let mut sim_result = SimulationResult {
        positions: HashMap::default(),
    };

    // Run the simulation until everything has stopped moving.
    loop {
        physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut rigid_bodies,
            &mut colliders,
            &mut joint_set,
            &mut ccd_solver,
            &physics_hooks,
            &event_handler,
        );
        if island_manager.active_dynamic_bodies().is_empty() {
            // Record the final positions of each ball.
            for (_, rigid_body) in rigid_bodies.iter() {
                sim_result
                    .positions
                    .insert(rigid_body.user_data as usize, rigid_body.position().clone());
            }
            break;
        }
    }

    // Add the simulation result as a resource so other systems can access it.
    commands.insert_resource(sim_result);
}
