//! 2D angular limits: no contacts, zero gravity, initially identity rotation.
use rapier2d::prelude::*;

#[unsafe(no_mangle)]
pub extern "C" fn run() -> u32 {
    let mut world = PhysicsWorld::new();
    world.gravity = Vector::ZERO;
    let fixed = world.insert_body(RigidBodyBuilder::fixed());
    let body = world.insert_body(RigidBodyBuilder::dynamic().can_sleep(false));
    world.insert_collider(ColliderBuilder::ball(1.0), Some(body));
    // Strictly positive-width interval, not a zero-width or reversed limit.
    let center = -0.047693253;
    let half_width = 0.0009765625;
    world.insert_impulse_joint(
        fixed,
        body,
        RevoluteJointBuilder::new().limits([center - half_width, center + half_width]),
    );
    world.step();
    world.bodies[body].rotation().im.to_bits()
}

fn main() {
    println!("{:08x}", run());
}
