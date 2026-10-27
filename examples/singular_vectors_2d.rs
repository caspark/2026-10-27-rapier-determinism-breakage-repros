use rapier2d::glamx::{Mat2, Svd2};

#[unsafe(no_mangle)]
pub extern "C" fn run() -> u32 {
    let matrix = std::hint::black_box(Mat2::from_cols_array(&[6.0, 1.0, 1.0, 1.0]));
    Svd2::from_matrix(matrix).u.x_axis.y.to_bits()
}

fn main() {
    println!("{:08x}", run());
}
