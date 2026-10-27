// Particle 0 y; deformation damping uses primitive f32::exp.
use rapier2d::prelude::*;

#[unsafe(no_mangle)]
pub extern "C" fn run() -> u32 {
    let mut w = PhysicsWorld::new();
    w.gravity = Vector::ZERO;
    let h = w.insert_soft_body(
        SoftBodyBuilder::new(vec![-Vector::X, Vector::ZERO, Vector::X])
            .material(SoftBodyMaterial {
                deformation_damping: 13.1,
                ..Default::default()
            })
            .no_surface_collider(),
    );
    w.soft_bodies[h].set_particle_velocity(1, Vector::Y);
    w.step();
    w.soft_bodies[h].particles()[0].position().y.to_bits()
}

fn main() {
    println!("{:08x}", run());
}
