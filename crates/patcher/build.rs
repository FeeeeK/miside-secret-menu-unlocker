use std::env;
use std::process::Command;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_dir = std::path::Path::new(manifest_dir.as_str())
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_str()
        .unwrap();
    std::env::set_current_dir(workspace_dir).unwrap();

    println!("cargo:rerun-if-changed=crates/loader/src/main.rs");
    println!("cargo:rerun-if-changed=crates/patcher/src/lib.rs");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Plugin");

    let build_type = env::var("PROFILE").unwrap_or_else(|_| "release".to_string());
    let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    let build_dir = format!("{}/{}", target_dir, build_type);

    Command::new("dotnet")
        .args(&[
            "publish",
            "-c",
            &build_type,
            "Plugin/SecretMenu.csproj",
            "--output",
            build_dir.as_str(),
        ])
        .status()
        .unwrap();
}
