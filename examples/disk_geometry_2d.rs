use rapier2d::prelude::*;

#[unsafe(no_mangle)]
pub extern "C" fn run() -> u32 {
    let disk = SoftBodyBuilder::disk(Vector::ZERO, 1.0, 105);
    disk.particle_positions()[32].y.to_bits()
}

fn main() {
    println!("{:08x}", run());
}
