# Rapier determinism reproducers

Small, standalone examples that break determinism in Rapier 0.36.1 with `enhanced-determinism` enabled. Each file contains its entire scene and prints one result as f32 hexadecimal bits.

> ## Slop Warning
> 
> These breakages and the reproduction examples here were found by a coding agent - I have attempted to prune down to a minimal repro set and deslop somewhat, but obviously take the results with a grain of salt because I don't know the rapier/parry codebases.

## Run

Rust with rustup is the only prerequisite. The toolchain file selects Rust 1.95.0 and the Wasm target. Cargo builds Wasmtime. No Wasmtime CLI, Node, or browser is needed.

```sh
cargo run --release                                         # batch: native vs both Wasmtime configs
cargo run --release -- compare angular_limits_2d            # one example: native vs both Wasmtime configs
cargo run --release -- native angular_limits_2d             # native only
cargo run --release -- wasm angular_limits_2d               # embedded Wasmtime
cargo run --release -- wasm-deterministic angular_limits_2d

# Run a standalone native example without the batch runner:
cargo run --release --example angular_limits_2d
```

`compare` exits **1 when results differ**, 0 when they match, and 2 for a build/runtime error. Other modes print results for comparison across machines.

The two Wasmtime configurations differ only in these options:

```rust
config.cranelift_nan_canonicalization(true);
config.relaxed_simd_deterministic(true);
```

The deterministic configuration does not fix native/Wasm differences in these examples. Native/Wasm differences reproduce on macOS ARM64, Linux x86-64, and Windows x86-64 (MSVC). Both Wasmtime configurations produce the same Wasm results on all three hosts.

## The Breaking Examples

The `simd_*` fixes use Simba's forced-libm path under `enhanced-determinism`. They do not change the solver's SIMD width.

`(/)` means the output matches at least one other platform. `(x)` means it matches no other platform. These marks do not imply correctness or agreement across all platforms.

### [Deformation damping, 2D](examples/deformation_damping_2d.rs)

Platform `exp` changes particle positions after one step. Fix: use `simd_exp()`.

- macOS ARM64: `3a3766dc` (/)
- Linux x86-64: `3a3766dc` (/)
- Windows x86-64 (MSVC): `3a3766dc` (/)
- Wasm: `3a3766d0` (x)

[soft_element_constraint_assembly.rs:206-208](https://github.com/dimforge/rapier/blob/v0.36.1/src/dynamics/solver/soft_constraint/soft_element_constraint_assembly.rs#L206-L208)

### [Elasticity, 2D](examples/elasticity_2d.rs)

Platform `atan2` changes a unit triangle's particle positions after one step. Fix: use `simd_atan2()`.

- macOS ARM64: `3af31af8` (/)
- Linux x86-64: `3af31af8` (/)
- Windows x86-64 (MSVC): `3af31af8` (/)
- Wasm: `3af31af7` (x)

[soft_body_shape_matching.rs:17-24](https://github.com/dimforge/rapier/blob/v0.36.1/src/dynamics/soft_body/soft_body_shape_matching.rs#L17-L24)

### [Angular limits, 2D](examples/angular_limits_2d.rs)

Platform `sin_cos` changes body rotation after one step. Fix: use `simd_sin_cos()`.

- macOS ARM64: `bd3f472e` (x)
- Linux x86-64: `bd3f472d` (/)
- Windows x86-64 (MSVC): `bd3f472d` (/)
- Wasm: `bd3f472d` (/)

[joint_constraint_helper.rs:60-64](https://github.com/dimforge/rapier/blob/v0.36.1/src/dynamics/solver/joint_constraint/joint_constraint_helper.rs#L60-L64)

### [Cloth bending, 3D](examples/dihedral_bending_3d.rs)

Platform `acos` changes particle positions after one step. Fix: use `simd_acos()`.

- macOS ARM64: `b05163e3` (/)
- Linux x86-64: `b05163e3` (/)
- Windows x86-64 (MSVC): `b05163e3` (/)
- Wasm: `b05163e4` (x)

[soft_scalar_constraint.rs:87-89](https://github.com/dimforge/rapier/blob/v0.36.1/src/dynamics/solver/soft_constraint/soft_element_constraint/soft_scalar_constraint.rs#L87-L89)

### [Angular motor, 2D](examples/angular_motor_2d.rs)

Platform target-angle `sin` changes a multibody hinge's motor result after one step. Fix: use `simd_sin()` for both current and target angles.

- macOS ARM64: `39a3ae90` (/)
- Linux x86-64: `39a3ae90` (/)
- Windows x86-64 (MSVC): `39a3ae8e` (/)
- Wasm: `39a3ae8e` (/)

[generic_joint_constraint_builder.rs:764-771](https://github.com/dimforge/rapier/blob/v0.36.1/src/dynamics/solver/joint_constraint/generic_joint_constraint_builder.rs#L764-L771)

### [Inverse kinematics, 3D](examples/inverse_kinematics_3d.rs)

Target-dependent matrix kernels change IK displacement for two bodies and one joint. Fix needs design: use a fixed-order product or a portable kernel.

- macOS ARM64: `3c21bb8c` (/)
- Linux x86-64: `3c21bb8c` (/)
- Windows x86-64 (MSVC): `3c21bb8c` (/)
- Wasm: `3c21bb71` (x)

[multibody_ik.rs:68-77](https://github.com/dimforge/rapier/blob/v0.36.1/src/dynamics/joint/multibody_joint/multibody_ik.rs#L68-L77)

### [Revolute angle, 3D](examples/revolute_angle_3d.rs)

Platform `asin` changes the returned angle without a simulation step. Fix: use `simd_asin()`.

- macOS ARM64: `3f94f0a0` (/)
- Linux x86-64: `3f94f0a0` (/)
- Windows x86-64 (MSVC): `3f94f0a0` (/)
- Wasm: `3f94f09f` (x)

[revolute_joint.rs:102-107](https://github.com/dimforge/rapier/blob/v0.36.1/src/dynamics/joint/revolute_joint.rs#L102-L107)

### [Tear smoothing, 2D](examples/tear_smoothing_2d.rs)

Platform `exp` changes public edge stress, without a demonstrated topology difference. Fix: use `simd_exp()`.

- macOS ARM64: `3b65860f` (/)
- Linux x86-64: `3b65860f` (/)
- Windows x86-64 (MSVC): `3b65860f` (/)
- Wasm: `3b65860e` (x)

[soft_body_material.rs:194-200](https://github.com/dimforge/rapier/blob/v0.36.1/src/dynamics/soft_body/soft_body_material.rs#L194-L200)

### [Disk geometry, 2D](examples/disk_geometry_2d.rs)

Platform `sin`/`cos` changes initial vertices of a unit-radius disk. Fix: use `simd_sin_cos()`.

- macOS ARM64: `3f70feb4` (x)
- Linux x86-64: `3f70feb3` (/)
- Windows x86-64 (MSVC): `3f70feb3` (/)
- Wasm: `3f70feb3` (/)

[soft_body_builder_shapes.rs:555-565](https://github.com/dimforge/rapier/blob/v0.36.1/src/dynamics/soft_body/soft_body_builder/soft_body_builder_shapes.rs#L555-L565)

### [Rest angle, 3D](examples/dihedral_rest_angle_3d.rs)

Platform `acos` changes the stored dihedral rest angle. Fix: use `simd_acos()`.

- macOS ARM64: `3fc3b0a9` (/)
- Linux x86-64: `3fc3b0a9` (/)
- Windows x86-64 (MSVC): `3fc3b0a9` (/)
- Wasm: `3fc3b0aa` (x)

[soft_body_builder_mesh_helpers.rs:284-294](https://github.com/dimforge/rapier/blob/v0.36.1/src/dynamics/soft_body/soft_body_builder/soft_body_builder_mesh_helpers.rs#L284-L294)

### [Tube geometry, 3D](examples/tube_geometry_3d.rs)

Platform `sin`/`cos` changes initial tube vertices. Fix: use `simd_sin_cos()`.

- macOS ARM64: `bf70feb4` (x)
- Linux x86-64: `bf70feb3` (/)
- Windows x86-64 (MSVC): `bf70feb3` (/)
- Wasm: `bf70feb3` (/)

[soft_body_builder_shapes.rs:276-279](https://github.com/dimforge/rapier/blob/v0.36.1/src/dynamics/soft_body/soft_body_builder/soft_body_builder_shapes.rs#L276-L279)

### [Arc geometry, 2D](examples/arc_geometry_2d.rs)

Parry's platform `acos` changes generated arc points, without a physics step. Fix: call deterministic trait math in `VectorExt::angle()`.

- macOS ARM64: `3f9f14fc` (/)
- Linux x86-64: `3f9f14fc` (/)
- Windows x86-64 (MSVC): `3f9f14fc` (/)
- Wasm: `3f9f14fb` (x)

[vector_ext.rs:55-60](https://github.com/dimforge/parry/blob/v0.31.1/src/math/vector_ext.rs#L55-L60)

### [Singular vectors, 2D](examples/singular_vectors_2d.rs)

Glamx's platform trig calls change the returned singular vectors, without a physics step. Fix: route those calls through deterministic trait math.

- macOS ARM64: `3e41a567` (/)
- Linux x86-64: `3e41a567` (/)
- Windows x86-64 (MSVC): `3e41a567` (/)
- Wasm: `3e41a568` (x)

[svd2.rs:53-62](https://github.com/dimforge/glamx/blob/v0.3.1/src/svd2.rs#L53-L62)

### [Plasticity, 3D](examples/plasticity_3d.rs)

The two-normalization `powf` path changes plastic rest positions. Fix: use `simd_powf()` for both calls.

- macOS ARM64: `3f71835f` (/)
- Linux x86-64: `3f71835f` (/)
- Windows x86-64 (MSVC): `3f71835f` (/)
- Wasm: `3f71835d` (x)

[soft_constraint_plasticity.rs:38-58](https://github.com/dimforge/rapier/blob/v0.36.1/src/dynamics/solver/soft_constraint/soft_constraints_set/soft_constraint_plasticity.rs#L38-L58)

### [Elastic correction cap, 3D](examples/elasticity_cap_3d.rs)

An inverted tetrahedron with a small corrective cap exposes platform `powf` differences. Fix: use `simd_powf()`.

- macOS ARM64: `b74dcfd2` (/)
- Linux x86-64: `b74dcfd2` (/)
- Windows x86-64 (MSVC): `b74dcfd2` (/)
- Wasm: `b74dcfd3` (x)

[soft_element_constraint_assembly.rs:762-768](https://github.com/dimforge/rapier/blob/v0.36.1/src/dynamics/solver/soft_constraint/soft_element_constraint_assembly.rs#L762-L768)

### [Contact coefficients, 2D](examples/contact_coefficients_2d.rs)

`Max` combines -0/+0 into different friction zero signs, not different body motion. Fix: canonicalize zero signs in combined coefficients.

- macOS ARM64: `00000000` (x)
- Linux x86-64: `80000000` (/)
- Windows x86-64 (MSVC): `80000000` (/)
- Wasm: `80000000` (/)

[coefficient_combine_rule.rs:84](https://github.com/dimforge/rapier/blob/v0.36.1/src/dynamics/coefficient_combine_rule.rs#L84)

## Missed tear with a forced schedule

[This example](examples/missed_tear_2d.rs) produces two rope bodies or one because body reduction can read flags before all workers finish. Fix needs review of synchronization and stage accounting for tear and plastic-flow flags.

- macOS ARM64: early `00000002`, late `00000001` (/)
- Linux x86-64: early `00000002`, late `00000001` (/)
- Windows x86-64 (MSVC): early `00000002`, late `00000001` (/)
- Wasm: unsupported (requires native threads).

[worker.rs:888-920](https://github.com/dimforge/rapier/blob/v0.36.1/src/dynamics/solver/staged_island_solver/worker.rs#L888-L920)

This needs native threads and a patched Rapier checkout because it relies on a theoretical thread scheduling issue (so far it hasn't been reproduced in unmodified Rapier: all 1,000 unmodified runs split the rope normally), so take this one with a helping of salt:

```sh
git clone --branch v0.36.1 --depth 1 https://github.com/dimforge/rapier.git /tmp/rapier
bash patches/apply-writeback-schedule.sh /tmp/rapier
```

Add this section to **this repo's** `Cargo.toml`, with the path to your patched checkout:

```toml
[patch.crates-io]
rapier2d = { path = "/tmp/rapier/crates/rapier2d" }
```

Run each schedule in a separate process:

```sh
RAPIER_AUDIT_SCHEDULE=early cargo run --release --features scheduling-repro --example missed_tear_2d
RAPIER_AUDIT_SCHEDULE=late  cargo run --release --features scheduling-repro --example missed_tear_2d
```

In PowerShell, set `$env:RAPIER_AUDIT_SCHEDULE = 'early'` or `'late'` before the Cargo command.

The outputs are `00000002` and `00000001`: body counts, not float bits. The patch only controls scheduling. It does not change arithmetic, work claims, or the existing barrier.

This example is excluded from the default batch and cannot run on the single-threaded Wasm target. Remove the patch override and scheduling environment variable before returning to the normal batch.

## Other suspicious code

- Inertia diagonalization uses platform trig. No physically valid body reproduction is included.
  - Possible trigger: an asymmetric convex hull whose principal inertias are close enough to change an eigenvector or branch.
  - Source: Rapier's [collider.rs:1103-1105](https://github.com/dimforge/rapier/blob/v0.36.1/src/geometry/collider.rs#L1103-L1105), Parry's [mass_properties.rs:214-230](https://github.com/dimforge/parry/blob/v0.31.1/src/mass_properties/mass_properties.rs#L214-L230), and Glamx's [eigen3.rs:101-106](https://github.com/dimforge/glamx/blob/v0.3.1/src/eigen3.rs#L101-L106).
  - Fix: use deterministic trait math for eigenvalue `acos` and `cos`. A physically valid inertia regression case is still needed.
- Delaunay refinement uses platform sine, `log2`, and `exp2`. No mesh-topology difference was reproduced.
  - Possible trigger: triangles near the angle or refinement thresholds.
  - Source: Rapier's [soft_body_builder_shapes.rs:443-470](https://github.com/dimforge/rapier/blob/v0.36.1/src/dynamics/soft_body/soft_body_builder/soft_body_builder_shapes.rs#L443-L470), Parry's [delaunay_refinement.rs:61-78](https://github.com/dimforge/parry/blob/v0.31.1/src/transformation/volume_mesh/delaunay_refinement.rs#L61-L78), and Spade's [refinement.rs:72-78](https://github.com/Stoeoef/spade/blob/v2.15.1/src/delaunay_core/refinement.rs#L72-L78) and [refinement.rs:958-959](https://github.com/Stoeoef/spade/blob/v2.15.1/src/delaunay_core/refinement.rs#L958-L959).
  - Fix: use deterministic sine, `log2`, and `exp2` throughout refinement. Integration with Spade's generic float API needs review.
- The BVH optimizer uses platform `log2`. Tested final integer counts still match.
  - Possible trigger: a leaf count where a rounding difference changes the integer optimization budget and traversal order.
  - Source: Rapier's [mod.rs:122-128](https://github.com/dimforge/rapier/blob/v0.36.1/src/geometry/broad_phase_bvh/mod.rs#L122-L128) and Parry's [bvh_optimize.rs:24-40](https://github.com/dimforge/parry/blob/v0.31.1/src/partitioning/bvh/bvh_optimize.rs#L24-L40).
  - Fix: use deterministic `log2` in the optimization-budget calculation.
- Equal-key unstable sorts have incomplete tie-breaks. Tests of the actual tuple types match across ARM64/Wasm.
  - Possible trigger: different equal-distance cell ordering or equal-alignment manifold ordering selects another binding/contact.
  - Source: [collision_mesh_binding.rs:243-257](https://github.com/dimforge/rapier/blob/v0.36.1/src/dynamics/soft_body/collision_mesh/collision_mesh_binding.rs#L243-L257), [soft_contacts_pairs.rs:125](https://github.com/dimforge/rapier/blob/v0.36.1/src/geometry/narrow_phase/soft_contacts/soft_contacts_pairs.rs#L125), and [soft_contacts_pairs.rs:163-169](https://github.com/dimforge/rapier/blob/v0.36.1/src/geometry/narrow_phase/soft_contacts/soft_contacts_pairs.rs#L163-L169).
  - Fix: use cell/manifold IDs as tie-breaks in sorting and winner selection.
