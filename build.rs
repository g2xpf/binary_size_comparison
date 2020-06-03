use std::process::Command;

fn main() {
    Command::new("gcc")
        .args(&["src/main.c", "-o", "bin/hello"])
        .status()
        .unwrap();
    println!("cargo:rerun-if-changed=src/main.c");
}
