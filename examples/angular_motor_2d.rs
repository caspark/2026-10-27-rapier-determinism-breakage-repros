//! 2D angular motor: no contacts, zero gravity, initially identity rotation.
use rapier2d::prelude::*;

#[unsafe(no_mangle)]
pub extern "C" fn run() -> u32 {
    let mut world = PhysicsWorld::new();
    world.gravity = Vector::ZERO;
    let fixed = world.insert_body(RigidBodyBuilder::fixed());
    let body = world.insert_body(RigidBodyBuilder::dynamic().can_sleep(false));
    world.insert_collider(ColliderBuilder::ball(1.0), Some(body));
    world
        .multibody_joints
        .insert(fixed, body, RevoluteJointBuilder::new(), true)
        .unwrap();
    // Actuate only the hinge's free DOF: no redundant lock rows.
    let actuator = GenericJointBuilder::new(JointAxesMask::empty()).motor_position(
        JointAxis::AngX,
        0.36494255,
        10.0,
        1.0,
    );
    world.insert_impulse_joint(fixed, body, actuator);
    world.step();
    world.bodies[body].rotation().im.to_bits()
}

fn main() {
    println!("{:08x}", run());
}
