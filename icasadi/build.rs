use bindgen;
use cc;

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Sanity checks to get better error messages
    assert!(
        Path::new("extern/auto_casadi_cost.c").exists(),
        "extern/auto_casadi_cost.c is missing"
    );
    assert!(
        Path::new("extern/auto_casadi_grad.c").exists(),
        "extern/auto_casadi_grad.c is missing"
    );
    assert!(
        Path::new("extern/icasadi.c").exists(),
        "extern/icasadi.c is missing"
    );
    assert!(
        Path::new("extern/icasadi.h").exists(),
        "extern/icasadi.h is missing"
    );
    assert!(
        Path::new("extern/icasadi_config.h").exists(),
        "extern/icasadi_config.h is missing"
    );

    cc::Build::new()
        .flag_if_supported("-Wall")
        .flag_if_supported("-Wpedantic")
        .flag_if_supported("-Wno-long-long")
        .flag("-Wno-unused-parameter")
        .pic(true)
        .include("src")
        .file("extern/auto_casadi_cost.c")
        .file("extern/auto_casadi_grad.c")
        .file("extern/auto_casadi_constraints_type_penalty.c")
        .file("extern/icasadi.c")
        .compile("icasadi");

    // Extract the problem size parameter size constants from the
    // icasadi_config.h file
    bindgen::Builder::default()
        .header("extern/icasadi_config.h")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Rerun if these autogenerated files change
    println!("cargo:rerun-if-changed=extern/auto_casadi_cost.c");
    println!("cargo:rerun-if-changed=extern/auto_casadi_grad.c");
    println!("cargo:rerun-if-changed=extern/auto_casadi_constraints_type_penalty.c");
    println!("cargo:rerun-if-changed=extern/icasadi.c");
    println!("cargo:rerun-if-changed=extern/icasadi.h");
    println!("cargo:rerun-if-changed=extern/icasadi_config.h");
}