use std::{path::Path, process::Command};

// TODO(Dorian): recompile on source change
fn compile_asm(input: impl AsRef<Path>, output: impl AsRef<Path>) {
    // TODO(Dorian): Better error handling
    Command::new("nasm")
        .arg("-felf32")
        .arg(input.as_ref())
        .arg("-o")
        .arg(output.as_ref())
        .spawn()
        .expect("Could not run nasm")
        .wait()
        .expect("Could not wait for nasm");
}

const SOURCE_PATH: &str = "asm/i686-elf/boot.asm";
const OBJECT_PATH: &str = "boot.o";
const STATIC_LIB_NAME: &str = "boot";

fn main() {
    compile_asm(SOURCE_PATH, OBJECT_PATH);

    // Build static library
    Command::new("ar")
        .arg("rcsu") // TODO: MAYBE remove
        .arg(format!("lib{STATIC_LIB_NAME}.a"))
        .arg(OBJECT_PATH)
        .spawn()
        .expect("Could not run ar")
        .wait()
        .expect("Could not wait for nasm");

    // Search current directory for library
    println!(
        "cargo:rustc-link-search=native={}",
        env!("CARGO_MANIFEST_DIR")
    );
    // Use boot library
    println!("cargo:rustc-link-lib=static={STATIC_LIB_NAME}");
}
