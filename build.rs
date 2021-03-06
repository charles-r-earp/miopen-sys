

fn main() {
    // link to hipblas
    /*let hipblas_lib = "opt/rocm/hipblas/lib";
    println!("cargo:rustc-link-search=native={}", hipblas_lib);
    println!("cargo:rustc-link-lib=dylib=hipblas");*/
    // link to miopen
    let miopen_lib = "/opt/rocm/miopen/lib";
    println!("cargo:rustc-link-search=native={}", miopen_lib);
    println!("cargo:rustc-link-lib=dylib=MIOpen");
    
    #[cfg(feature = "bindgen")] {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
        println!("cargo:rerun-if-changed=wrapper.h");
        let bindings = bindgen::Builder::default()
            .raw_line("#![allow(warnings)]")
            .raw_line("use std::fmt::Debug;")
            // The input header we would like to generate
            // bindings for.
            .header("wrapper.h")
            //.clang_arg("-I/opt/rocm/hipblas/include")
            .clang_arg("-I/opt/rocm/miopen/include")
            .clang_arg("-I/opt/rocm/hip/include")
            .rustified_non_exhaustive_enum("hip.*")
            .rustified_non_exhaustive_enum("miopen.*")
            .rustified_non_exhaustive_enum("MIOPEN.*")
            .generate_block(false)
            .size_t_is_usize(true)
            .ctypes_prefix("::libc")
            // Tell cargo to invalidate the built crate whenever any of the
            // included header files changed.
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            // Finish the builder and generate the bindings.
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings");
        bindings
            .write_to_file("src/lib.rs")
            .expect("Couldn't write bindings!");
    }
}
