use cmake::Config;

use std::{env, path::PathBuf};

fn main() {
    let target = env::var("TARGET").unwrap();
    let (_arch, _vendor, sys, _abi) = {
        let mut target_s = target.split('-');
        (
            target_s.next().unwrap(),
            target_s.next().unwrap(),
            target_s.next().unwrap(),
            target_s.next().unwrap_or(""),
        )
    };
    let dst = Config::new(".").build();
    println!("cargo:rustc-link-search=native={}/lib", dst.display(),);
    println!("cargo:rustc-link-lib=static=blend2d");
    match sys {
        "windows" => {
            println!("cargo:rustc-link-lib=user32");
            println!("cargo:rustc-link-lib=uuid");
            println!("cargo:rustc-link-lib=shell32");
        },
        "linux" => {
            println!("cargo:rustc-link-lib=c");
            println!("cargo:rustc-link-lib=m");
            println!("cargo:rustc-link-lib=pthread");
            println!("cargo:rustc-link-lib=rt");
        },
        "darwin" => {
            println!("cargo:rustc-link-lib=c");
            println!("cargo:rustc-link-lib=m");
            println!("cargo:rustc-link-lib=pthread");
        },
        _ => (),
    }

    let whitelist_regex = "[Bb][Ll].*";
    let bindings = bindgen::Builder::default()
        .header("blend2d/src/blend2d.h")
        .layout_tests(false)
        .generate_comments(false)
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
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
