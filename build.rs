#![allow(unstable)]
use std::old_io::Command;
use std::os;

fn main() {
    let out_dir = os::getenv("OUT_DIR").unwrap();

    Command::new("gcc").args(&["c_test_lib/c_test_lib.c", "-std=c99", "-fPIC", "-c", "-o"])
                       .arg(format!("{}/c_test_lib.o", out_dir))
                       .status().unwrap();
    Command::new("ar").args(&["crus", "libc_test_lib.a", "c_test_lib.o"])
                      .cwd(&Path::new(&out_dir))
                      .status().unwrap();

    println!("cargo:rustc-flags=-l c_test_lib -L {}", out_dir);
}