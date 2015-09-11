extern crate pkg_config;

use std::env;
use std::process::Command;
use std::path::PathBuf;

fn main() {
    if env::var_os("ICU_SYS_NO_PKG_CONFIG").is_none() {
        if pkg_config::find_library("icu-uc").is_ok() {
            return
        }
    }

    assert!(Command::new("make")
        .args(&["-f", "makefile.cargo", "-j", &env::var("NUM_JOBS").unwrap()])
        .status()
        .unwrap()
        .success());

    let install_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("prefix");
    println!("cargo:install_dir={}", install_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=icuuc");
    println!("cargo:rustc-link-search=native={}", install_dir.join("lib").to_str().unwrap());
}
