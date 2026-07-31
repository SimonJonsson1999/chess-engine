use std::{env, fs, path::Path};

fn main() {
    let manifest = env::var("CARGO_MANIFEST_DIR").unwrap();

    let sdl = Path::new(&manifest).join("SDL3-3.4.12");

    println!(
        "cargo:rustc-link-search=native={}",
        sdl.join("lib").join("x64").display()
    );
    println!("cargo:rustc-link-lib=SDL3");

    let out_dir = env::var("OUT_DIR").unwrap();

    // From OUT_DIR navigate back to target/debug
    let target_dir = Path::new(&out_dir).ancestors().nth(3).unwrap();

    fs::copy(
        sdl.join("lib").join("x64").join("SDL3.dll"),
        target_dir.join("SDL3.dll"),
    )
    .unwrap();
}
