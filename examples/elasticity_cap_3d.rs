use rapier3d::prelude::*;

#[unsafe(no_mangle)]
pub extern "C" fn run() -> u32 {
    let mut world = PhysicsWorld::default();
    world.gravity = Vector::ZERO;
    world.integration_parameters.num_solver_iterations = 1;
    world
        .integration_parameters
        .normalized_max_corrective_velocity = 0.001;
    let builder = SoftBodyBuilder::new(vec![
        Vector::ZERO,
        Vector::X,
        Vector::Y,
        Vector::Z * (0.1 + 25.0 / 1024.0),
    ])
    .cells(vec![[0, 1, 2, 3]])
    .cell_model(SoftBodyCellModel::Corotational)
    .no_surface_collider()
    .can_sleep(false);
    let handle = world
        .soft_bodies
        .insert(builder, &mut world.bodies, &mut world.colliders);
    // Intentionally invert the current tetrahedron to activate the small corrective cap.
    world.soft_bodies[handle].set_particle_position(1, Vector::new(-1.5, 0.2, 0.0));
    world.step();
    // v0.36.1: soft_element_constraint_assembly.rs:765 (cell-size powf).
    world.soft_bodies[handle].particles()[0]
        .position()
        .x
        .to_bits()
}

fn main() {
    println!("{:08x}", run());
}
