use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        File::create(out.join("device.x"))
            .unwrap()
            .write_all(include_bytes!("../stm32f767/device.x"))
            .unwrap();
        println!("cargo:rustc-link-search={}", out.display());
        println!("cargo:rerun-if-changed=../stm32f767/device.x");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
