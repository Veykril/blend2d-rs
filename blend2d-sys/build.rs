use std::ffi::OsStr;
use std::path::Path;
use std::{env, fs, io, path::PathBuf};

fn add_source<P: AsRef<Path>>(cfg: &mut cc::Build, path: P) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let path = entry?.path();
        if path.is_dir() {
            add_source(cfg, path)?;
        } else if path.extension().and_then(OsStr::to_str) == Some("cpp") {
            cfg.file(path);
        }
    }
    Ok(())
}

static ASMJIT_SOURCE_PATH: &str = "./asmjit/src";
static BLEND2D_SOURCE_PATH: &str = "./blend2d/src";

fn main() -> io::Result<()> {
    let target = env::var("TARGET").unwrap();
    let (arch, _vendor, sys, _abi) = {
        let mut target_s = target.split('-');
        (
            target_s.next().unwrap(),
            target_s.next().unwrap(),
            target_s.next().unwrap(),
            target_s.next().unwrap_or(""),
        )
    };
    let (msvc, gnu, clang) = {
        let tool = cc::Build::new().get_compiler();
        (
            tool.is_like_msvc(),
            tool.is_like_gnu(),
            tool.is_like_clang(),
        )
    };
    let x64 = arch == "x86_64";

    let mut cfg = cc::Build::new();
    add_source(&mut cfg, BLEND2D_SOURCE_PATH)?;
    add_source(&mut cfg, ASMJIT_SOURCE_PATH)?;
    cfg.cpp(true)
        .warnings(false)
        .extra_warnings(false)
        .flag_if_supported("-static")
        .flag_if_supported("-std=c++17")
        .flag_if_supported("/std:c++latest")
        .include(ASMJIT_SOURCE_PATH)
        .include(BLEND2D_SOURCE_PATH)
        .define("ASMJIT_STATIC", None);

    if let Some(s) = env::var_os("PROFILE") {
        if &*s == "release" || &*s == "bench" {
            cfg.define("NDEBUG", None);
        }
    }

    if cfg!(feature = "sse2") {
        cfg.define("BL_BUILD_OPT_SSE2", None);
        if msvc {
            cfg.define("__SSE2__", None);
            if !x64 {
                cfg.flag("-arch:SSE2");
            }
        } else {
            cfg.flag("-msse2");
        }
    }
    if cfg!(feature = "sse3") {
        cfg.define("BL_BUILD_OPT_SSE3", None);
        if msvc {
            cfg.define("__SSE3__", None);
            if !x64 {
                cfg.flag("-arch:SSE2");
            }
        } else {
            cfg.flag("-msse3");
        }
    }
    if cfg!(feature = "ssse3") {
        cfg.define("BL_BUILD_OPT_SSSE3", None);
        if msvc {
            cfg.define("__SSSE3__", None);
            if !x64 {
                cfg.flag("-arch:SSE2");
            }
        } else {
            cfg.flag("-mssse3");
        }
    }
    if cfg!(feature = "sse4_1") {
        cfg.define("BL_BUILD_OPT_SSE4_1", None);
        if msvc {
            cfg.define("__SSE4_1__", None);
            if !x64 {
                cfg.flag("-arch:SSE2");
            }
        } else {
            cfg.flag("-msse4.1");
        }
    }
    if cfg!(feature = "sse4_2") {
        cfg.define("BL_BUILD_OPT_SSE4_2", None);
        if msvc {
            cfg.define("__SSE4_2__", None);
            if !x64 {
                cfg.flag("-arch:SSE2");
            }
        } else {
            cfg.flag("-msse4.2");
        }
    }
    if cfg!(feature = "avx") {
        cfg.define("BL_BUILD_OPT_AVX", None);
        if msvc {
            cfg.flag("-arch:AVX");
        } else {
            cfg.flag("-mavx");
        }
    }
    if cfg!(feature = "avx2") {
        cfg.define("BL_BUILD_OPT_AVX2", None);
        if msvc {
            cfg.flag("-arch:AVX2");
        } else {
            cfg.flag("-mavx2");
        }
    }

    if clang || gnu {
        cfg.flag("-fvisibility=hidden")
            .flag("-fno-exceptions")
            .flag("-fno-rtti")
            .flag("-fno-math-errno")
            .flag("-fmerge-all-constants")
            .flag("-ftree-vectorize");
    }

    cfg.compile("blend2d");
    match sys {
        "windows" => {
            println!("cargo:rustc-link-lib=user32");
            println!("cargo:rustc-link-lib=uuid");
            println!("cargo:rustc-link-lib=shell32");
        }
        "linux" => {
            println!("cargo:rustc-link-lib=c");
            println!("cargo:rustc-link-lib=m");
            println!("cargo:rustc-link-lib=pthread");
            println!("cargo:rustc-link-lib=rt");
        }
        "darwin" => {
            println!("cargo:rustc-link-lib=c");
            println!("cargo:rustc-link-lib=m");
            println!("cargo:rustc-link-lib=pthread");
        }
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
    Ok(())
}
