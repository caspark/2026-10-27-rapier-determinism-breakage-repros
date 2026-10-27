use rapier3d::prelude::*;

#[unsafe(no_mangle)]
pub extern "C" fn run() -> u32 {
    let mut world = PhysicsWorld::new();
    let body = world.insert_soft_body(
        SoftBodyBuilder::new(vec![
            Vector::ZERO,
            Vector::X,
            Vector::Y,
            Vector::new(0.0, 43.0 / 1024.0, 1.0),
        ])
        .dihedrals(vec![[0, 1, 2, 3]]),
    );
    world.soft_bodies[body].dihedrals()[0].rest_angle.to_bits()
}

fn main() {
    println!("{:08x}", run());
}
