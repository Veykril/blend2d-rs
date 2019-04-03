use cmake::Config;

use std::{env, path::PathBuf};

fn main() {
    //let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let dst = Config::new("blend2d")
        //.env("APP_DIR", &format!("{}/blend2d", manifest_dir))
        //.env("BLEND2D_DIR", &format!("{}/blend2d", manifest_dir))
        .define("BLEND2D_BUILD_STATIC:BOOL", "TRUE")
        .build();
    // fixme for release build
    println!(
        "cargo:rustc-link-search=native={}/build/Debug",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=blend2d");

    let whitelist_regex = "[Bb][Ll].*";
    let bindings = bindgen::Builder::default()
        .header("blend2d/src/blend2d.h")
        .layout_tests(false)
        .generate_comments(false)
        .default_enum_style(bindgen::EnumVariation::Rust)
        .whitelist_function(whitelist_regex)
        .whitelist_type(whitelist_regex)
        .whitelist_var(whitelist_regex)
        .derive_debug(false)
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
