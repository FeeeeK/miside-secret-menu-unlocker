use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=SecretMenu.cs");
    println!("cargo:rerun-if-changed=SecretMenu.csproj");

    Command::new("dotnet")
        .args(&["build", "-c", "Release"])
        .status()
        .unwrap();
}
