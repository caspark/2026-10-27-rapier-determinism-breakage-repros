use rapier3d::prelude::*;

#[unsafe(no_mangle)]
pub extern "C" fn run() -> u32 {
    let tube = SoftBodyBuilder::cloth_tube(Vector::ZERO, Vector::Y, 1.0, 1.0, 105, 2);
    tube.particle_positions()[32].z.to_bits()
}

fn main() {
    println!("{:08x}", run());
}
