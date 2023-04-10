use std::process::Command;

fn main() {
    cc::Build::new()
        .file("src/auxbinds.c")
        .compile("auxbinds");
    let ldflags = String::from_utf8(Command::new("python3-config").arg("--ldflags").output().expect("").stdout).unwrap();
    println!("cargo:rustc-link-arg-bins={}", ldflags);
    println!("cargo:rustc-link-lib=python3.12d");
    println!("cargo:rustc-link-lib=asan");
}
