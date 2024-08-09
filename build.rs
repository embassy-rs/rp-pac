use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        if env::var_os("CARGO_FEATURE_RP2040").is_some() {
            File::create(out.join("device.x"))
                .unwrap()
                .write_all(include_bytes!("device-rp2040.x"))
                .unwrap();
        }
        if env::var_os("CARGO_FEATURE_RP235X").is_some() {
            File::create(out.join("device.x"))
                .unwrap()
                .write_all(include_bytes!("device-rp235x.x"))
                .unwrap();
        }
        println!("cargo:rustc-link-search={}", out.display());
        println!("cargo:rerun-if-changed=device.x");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
