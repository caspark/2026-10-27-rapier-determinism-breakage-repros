use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use wasmtime::{Config, Engine, Instance, Module, Store};

const EXAMPLES: &[&str] = &[
    "deformation_damping_2d",
    "elasticity_2d",
    "angular_limits_2d",
    "dihedral_bending_3d",
    "angular_motor_2d",
    "inverse_kinematics_3d",
    "revolute_angle_3d",
    "tear_smoothing_2d",
    "disk_geometry_2d",
    "dihedral_rest_angle_3d",
    "tube_geometry_3d",
    "arc_geometry_2d",
    "singular_vectors_2d",
    "plasticity_3d",
    "elasticity_cap_3d",
    "contact_coefficients_2d",
];

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> ExitCode {
    match run() {
        Ok(different) => ExitCode::from(u8::from(different)),
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<bool> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.first().is_some_and(|a| a == "--help" || a == "-h") {
        println!(
            "Usage: cargo run --release -- [compare|native|wasm|wasm-deterministic] [example]"
        );
        println!("Omit the example to run the batch. Default mode: compare.");
        println!("Exit codes: 0 = success/equal, 1 = results differ, 2 = build/runtime error.");
        return Ok(false);
    }
    if args.len() > 2 {
        return Err("Expected a mode and an optional example name. Use --help.".into());
    }
    let mode = args.first().map(String::as_str).unwrap_or("compare");
    if !["compare", "native", "wasm", "wasm-deterministic"].contains(&mode) {
        return Err(format!("Unknown mode: {mode}. Use --help.").into());
    }
    let names: Vec<&str> = match args.get(1) {
        Some(name) if EXAMPLES.contains(&name.as_str()) => vec![name],
        Some(name) => return Err(format!("Unknown example: {name}.").into()),
        None => EXAMPLES.to_vec(),
    };
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if mode == "native" || mode == "compare" {
        build(&root, &names, false)?;
    }
    if mode != "native" {
        build(&root, &names, true)?;
    }
    let default_engine = (mode == "wasm" || mode == "compare").then(Engine::default);
    let deterministic_engine = if mode == "wasm-deterministic" || mode == "compare" {
        let mut config = Config::new();
        config.cranelift_nan_canonicalization(true);
        config.relaxed_simd_deterministic(true);
        Some(Engine::new(&config)?)
    } else {
        None
    };
    if mode == "compare" {
        println!(
            "{:<28} {:>8} {:>8} {:>18}",
            "example", "native", "wasm", "wasm-deterministic"
        );
    }
    let mut native_differences = 0;
    let mut engine_differences = 0;
    for name in &names {
        if mode == "compare" {
            let native = native_result(&root, name)?;
            let wasm = wasm_result(&root, name, default_engine.as_ref().unwrap())?;
            let deterministic = wasm_result(&root, name, deterministic_engine.as_ref().unwrap())?;
            println!(
                "{name:<28} {native:08x} {wasm:08x} {:>18}",
                format!("{deterministic:08x}")
            );
            native_differences += usize::from(native != wasm);
            engine_differences += usize::from(wasm != deterministic);
        } else {
            let value = match mode {
                "native" => native_result(&root, name)?,
                "wasm" => wasm_result(&root, name, default_engine.as_ref().unwrap())?,
                _ => wasm_result(&root, name, deterministic_engine.as_ref().unwrap())?,
            };
            println!("{name} {value:08x}");
        }
    }
    if mode == "compare" {
        eprintln!(
            "Native/Wasm differences: {native_differences}/{}",
            names.len()
        );
        eprintln!(
            "Wasmtime configuration differences: {engine_differences}/{}",
            names.len()
        );
    }
    Ok(native_differences != 0 || engine_differences != 0)
}

fn build(root: &Path, names: &[&str], wasm: bool) -> Result<()> {
    let mut command = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
    command
        .current_dir(root)
        .args(["build", "--locked", "--release", "--target-dir"])
        .arg(root.join("target"));
    if wasm {
        command.args(["--target", "wasm32-unknown-unknown"]);
    }
    for name in names {
        command.args(["--example", name]);
    }
    let status = command.stdout(Stdio::null()).status()?;
    if !status.success() {
        return Err(format!("Example build failed: {status}").into());
    }
    Ok(())
}

fn native_result(root: &Path, name: &str) -> Result<u32> {
    let executable = root
        .join("target/release/examples")
        .join(format!("{name}{}", std::env::consts::EXE_SUFFIX));
    let output = Command::new(executable).output()?;
    if !output.status.success() {
        return Err(format!(
            "{name}: {}\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    Ok(u32::from_str_radix(
        std::str::from_utf8(&output.stdout)?.trim(),
        16,
    )?)
}

fn wasm_result(root: &Path, name: &str, engine: &Engine) -> Result<u32> {
    let path = root
        .join("target/wasm32-unknown-unknown/release/examples")
        .join(format!("{name}.wasm"));
    let module = Module::from_file(engine, path)?;
    let mut store = Store::new(engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;
    let function = instance.get_typed_func::<(), i32>(&mut store, "run")?;
    Ok(function.call(&mut store, ())? as u32)
}
