//! Two touching balls: Max combines friction -0.0 and +0.0.
//! The observable is contact metadata, not demonstrated trajectory drift.
use rapier2d::prelude::*;

#[unsafe(no_mangle)]
pub extern "C" fn run() -> u32 {
    let mut world = PhysicsWorld::new();
    world.gravity = Vector::ZERO;
    let fixed = world.insert_body(RigidBodyBuilder::fixed());
    world.insert_collider(
        ColliderBuilder::ball(1.0)
            .friction(-0.0)
            .friction_combine_rule(CoefficientCombineRule::Max),
        Some(fixed),
    );
    let moving = world.insert_body(
        RigidBodyBuilder::dynamic()
            .translation(Vector::Y * 1.9)
            .can_sleep(false),
    );
    world.insert_collider(
        ColliderBuilder::ball(1.0)
            .friction(0.0)
            .friction_combine_rule(CoefficientCombineRule::Max),
        Some(moving),
    );
    world.step();
    let pair = world.contact_pairs().next().expect("contact pair");
    pair.manifolds()
        .first()
        .expect("contact manifold")
        .data
        .friction
        .to_bits()
}

fn main() {
    println!("{:08x}", run());
}
