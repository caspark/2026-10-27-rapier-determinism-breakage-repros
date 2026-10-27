//! Native-only schedule proof. Apply the patch and run one schedule per process.
use rapier2d::prelude::*;

#[unsafe(no_mangle)]
pub extern "C" fn run() -> u32 {
    assert!(matches!(
        std::env::var("RAPIER_AUDIT_SCHEDULE").as_deref(),
        Ok("early" | "late")
    ));
    rapier2d::rayon::ThreadPoolBuilder::new()
        .num_threads(2)
        .build()
        .unwrap()
        .install(|| {
            let mut world = PhysicsWorld::new();
            world.gravity = Vector::ZERO;
            world.integration_parameters.max_ccd_substeps = 0;

            // 256 element constraints make the solver use two workers.
            for i in 0..254 {
                let p = Vector::new(10.0 + i as f32 * 4.0, 0.0);
                world.insert_soft_body(
                    SoftBodyBuilder::new(vec![p, p + Vector::X])
                        .edges(vec![[0, 1]])
                        .no_surface_collider()
                        .can_sleep(false),
                );
            }
            let target = world.insert_soft_body(
                SoftBodyBuilder::rope(Vector::ZERO, Vector::X * 2.0, 3)
                    .edges(vec![[0, 1], [1, 2]])
                    .bend_edges(Vec::new())
                    .softness(SpringCoefficients::new(1.0, 0.0))
                    .tear_strain(0.1)
                    .tear_smoothing(0.0)
                    .min_piece(1)
                    .no_surface_collider()
                    .can_sleep(false),
            );
            world.soft_bodies[target].set_particle_position(2, Vector::new(3.0, 0.0));
            world.step();
            world
                .soft_bodies
                .iter()
                .filter(|(h, s)| *h == target || s.origin() == Some(target))
                .count() as u32
        })
}

fn main() {
    println!("{:08x}", run());
}
