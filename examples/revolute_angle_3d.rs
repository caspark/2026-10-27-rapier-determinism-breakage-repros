//! Getter-only observable; this is not a stepped-world divergence.
use rapier3d::na::ComplexField;
use rapier3d::prelude::*;

#[unsafe(no_mangle)]
pub extern "C" fn run() -> u32 {
    let x = std::hint::black_box(0.54952586_f32);
    let q = Rotation::from_xyzw(x, 0.0, 0.0, ComplexField::sqrt(1.0 - x * x));
    RevoluteJointBuilder::new(Vector::X)
        .build()
        .angle(&Rotation::IDENTITY, &q)
        .to_bits()
}

fn main() {
    println!("{:08x}", run());
}
