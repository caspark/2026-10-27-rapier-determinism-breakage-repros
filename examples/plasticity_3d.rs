use rapier3d::prelude::*;

#[unsafe(no_mangle)]
pub extern "C" fn run() -> u32 {
    let mut world = PhysicsWorld::default();
    world.gravity = Vector::ZERO;
    world.integration_parameters.num_solver_iterations = 1;
    let mut material = SoftBodyMaterial::default();
    material.plastic_yield = 0.00001;
    material.plastic_creep = 60.0;
    material.plastic_max = 10.0;
    let builder = SoftBodyBuilder::new(vec![
        Vector::ZERO,
        Vector::X,
        Vector::Y,
        Vector::Z * (0.1 + 17.0 / 1024.0),
    ])
    .cells(vec![[0, 1, 2, 3]])
    .cell_model(SoftBodyCellModel::Corotational)
    .material(material)
    .no_surface_collider()
    .can_sleep(false);
    let handle = world
        .soft_bodies
        .insert(builder, &mut world.bodies, &mut world.colliders);
    world.soft_bodies[handle].set_particle_position(1, Vector::new(1.5, 0.2, 0.0));
    world.step();
    // v0.36.1: soft_constraint_plasticity.rs:45,58 (determinant normalization).
    world.soft_bodies[handle].particles()[1]
        .rest_position()
        .x
        .to_bits()
}

fn main() {
    println!("{:08x}", run());
}
