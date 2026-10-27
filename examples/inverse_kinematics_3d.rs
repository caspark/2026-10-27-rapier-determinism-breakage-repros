//! Kinematic API output from Rapier's actual 6x7 body Jacobian, not a trajectory.
use rapier3d::na::{DVector, SVector};
use rapier3d::prelude::*;

#[unsafe(no_mangle)]
pub extern "C" fn run() -> u32 {
    let mut bodies = RigidBodySet::new();
    let root = bodies.insert(RigidBodyBuilder::dynamic().rotation(Vector::new(0.3, 0.2, 0.1)));
    let child = bodies.insert(RigidBodyBuilder::dynamic());
    let mut joints = MultibodyJointSet::new();
    let handle = joints
        .insert(
            root,
            child,
            RevoluteJointBuilder::new(Vector::new(0.3, 0.2, 0.1).normalize())
                .local_anchor1(Vector::new(2.1, 0.7, 1.3)),
            true,
        )
        .unwrap();
    let (multibody, link) = joints.get_mut(handle).unwrap();
    multibody.forward_kinematics(&bodies, true);
    let movement = SVector::<f32, 6>::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    let mut displacement = DVector::zeros(multibody.ndofs());
    multibody.inverse_kinematics_delta(link, &movement, 1.0, &mut displacement);
    displacement[0].to_bits()
}

fn main() {
    println!("{:08x}", run());
}
