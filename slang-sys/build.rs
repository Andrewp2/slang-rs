use std::fs;

use bindgen::Formatter;

fn generate_bindings() {
    fs::create_dir_all("gen").unwrap();

    bindgen::Builder::default()
        .header("vendor/slang/slang.h")
        .clang_arg("-I./vendor/slang")
        .clang_arg("-xc++")
        .clang_arg("-std=c++17")
        .allowlist_function("slang_.*")
        .allowlist_type("slang.*")
        .allowlist_type("FileSystemContentsCallBack")
        .allowlist_type("PathKind")
        .allowlist_var("SLANG_.*")
        .with_codegen_config(
            bindgen::CodegenConfig::FUNCTIONS
                | bindgen::CodegenConfig::TYPES
                | bindgen::CodegenConfig::VARS,
        )
        .layout_tests(false)
        .vtable_generation(true)
        .derive_copy(true)
        .formatter(Formatter::Rustfmt)
        .generate()
        .expect("Failed to generate bindings")
        .write_to_file("gen/bindings.rs")
        .expect("Failed to write bindings to file");
}

fn main() {
    generate_bindings();

    #[cfg(feature = "static")]
    {
        let path = cmake::Config::new("vendor/slang")
            .define("SLANG_LIB_TYPE", "STATIC")
            .build();

        // println!("cargo:rustc-link-lib=static={:?}", path.join("lib").join("slang.lib"));
        // Add the directory containing the library to the linker search path
        println!(
            "cargo:rustc-link-search=native={}",
            path.join("lib").display()
        );
        // Specify the name of the library to link against
        println!("cargo:rustc-link-lib=static=slang");
        let cpp_lib = if cfg!(target_os = "macos") {
            "c++"
        } else if cfg!(target_os = "windows") {
            "msvcrt"
        } else {
            "stdc++"
        };
        println!("cargo:rustc-link-lib={}", cpp_lib);
        // Link additional libraries if necessary (e.g., `dl` and `pthread` on Linux)
        #[cfg(target_os = "linux")]
        {
            println!("cargo:rustc-link-lib=dylib=dl");
            println!("cargo:rustc-link-lib=dylib=pthread");
        }
    }

    #[cfg(not(feature = "static"))]
    {
        #[cfg(any(target_os = "windows"))]
        {
            use std::{env, path::PathBuf};

            let slang_dir = env::var("SLANG_DIR").map(PathBuf::from).expect("Please provide an environment variable `SLANG_DIR` that points to your slang installation.");
            println!(
                "cargo:rustc-link-search=native={}",
                slang_dir.join("bin").display()
            );
            println!(
                "cargo:rustc-link-search=native={}",
                slang_dir.join("lib").display()
            );

            println!("cargo:rustc-link-lib=static=slang");
        }
        #[cfg(target_os = "linux")]
        println!("cargo:rustc-link-lib=slang");

        #[cfg(target_os = "macos")]
        println!("cargo:rustc-link-lib=dylib=slang");
    }
}
