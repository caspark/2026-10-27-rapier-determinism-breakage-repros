use rapier2d::glamx::Vec2;
use rapier2d::parry::transformation::utils::push_arc;

#[unsafe(no_mangle)]
pub extern "C" fn run() -> u32 {
    let mut points = Vec::new();
    push_arc(
        Vec2::ZERO,
        Vec2::new(2.0, 3.0),
        Vec2::new(3.0, 1.0),
        3,
        &mut points,
    );
    points[0].x.to_bits()
}

fn main() {
    println!("{:08x}", run());
}
