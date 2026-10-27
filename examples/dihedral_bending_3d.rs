use rapier3d::prelude::*;

#[unsafe(no_mangle)]
pub extern "C" fn run() -> u32 {
    let mut world = PhysicsWorld::default();
    world.gravity = Vector::ZERO;
    world.integration_parameters.num_solver_iterations = 1;
    // Two nondegenerate, overlapping flat triangles give an identical zero rest angle.
    let builder = SoftBodyBuilder::new(vec![Vector::ZERO, Vector::X, Vector::Y, Vector::Y * 2.0])
        .dihedrals(vec![[0, 1, 2, 3]])
        .no_surface_collider()
        .can_sleep(false);
    let handle = world
        .soft_bodies
        .insert(builder, &mut world.bodies, &mut world.colliders);
    world.soft_bodies[handle].set_particle_position(3, Vector::new(0.0, 43.0 / 1024.0, 1.0));
    world.step();
    // v0.36.1: soft_scalar_constraint.rs:88 (runtime primitive acos).
    world.soft_bodies[handle].particles()[0]
        .position()
        .x
        .to_bits()
}

fn main() {
    println!("{:08x}", run());
}
