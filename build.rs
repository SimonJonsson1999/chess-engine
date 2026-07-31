use std::{env, fs, path::Path};

fn main() {
    let manifest = env::var("CARGO_MANIFEST_DIR").unwrap();

    let libraries = [
        ("SDL3-3.4.12", "SDL3"),
        ("SDL3_image-3.4.4", "SDL3_image"),
        ("SDL3_ttf-3.2.2", "SDL3_ttf"),
    ];

    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir).ancestors().nth(3).unwrap();

    for (folder, lib_name) in libraries {
        let root = Path::new(&manifest).join(folder);

        // Tell Rust where to find the import library (.lib)
        println!(
            "cargo:rustc-link-search=native={}",
            root.join("lib").join("x64").display()
        );
        println!("cargo:rustc-link-lib={}", lib_name);

        // Copy the runtime DLL next to the executable
        fs::copy(
            root.join("lib").join("x64").join(format!("{lib_name}.dll")),
            target_dir.join(format!("{lib_name}.dll")),
        )
        .unwrap_or_else(|e| panic!("Failed to copy {lib_name}.dll: {e}"));
    }
}