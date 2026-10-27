// Edge stress only (no topology change); tear smoothing uses primitive f32::exp.
use rapier2d::prelude::*;

#[unsafe(no_mangle)]
pub extern "C" fn run() -> u32 {
    let mut w = PhysicsWorld::new();
    w.gravity = Vector::ZERO;
    let h = w.insert_soft_body(
        SoftBodyBuilder::new(vec![Vector::ZERO, Vector::X])
            .edges(vec![[0, 1]])
            .material(SoftBodyMaterial {
                tear_strain: Some(100.0),
                tear_smoothing: 0.0085,
                ..Default::default()
            })
            .no_surface_collider(),
    );
    w.soft_bodies[h].set_particle_position(1, Vector::X * 2.0);
    w.step();
    w.soft_bodies[h].edges()[0].stress().to_bits()
}

fn main() {
    println!("{:08x}", run());
}
