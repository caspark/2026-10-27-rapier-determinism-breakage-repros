// Particle 0 y; corotational fitting uses primitive f32::atan2.
// Unit right triangle, moderately stretched and sheared; no thin-element scaling.
use rapier2d::prelude::*;

#[unsafe(no_mangle)]
pub extern "C" fn run() -> u32 {
    let mut w = PhysicsWorld::new();
    w.gravity = Vector::ZERO;
    let h = w.insert_soft_body(
        SoftBodyBuilder::new(vec![Vector::ZERO, Vector::X, Vector::Y])
            .cells(vec![[0, 1, 2]])
            .cell_model(SoftBodyCellModel::Corotational)
            .material(SoftBodyMaterial {
                young_modulus: 100.0,
                ..Default::default()
            })
            .no_surface_collider(),
    );
    w.soft_bodies[h].set_particle_position(1, Vector::new(1.1, 0.5));
    w.step();
    w.soft_bodies[h].particles()[0].position().y.to_bits()
}

fn main() {
    println!("{:08x}", run());
}
