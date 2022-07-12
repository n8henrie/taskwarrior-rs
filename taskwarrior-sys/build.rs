use std::env;
use std::path::PathBuf;

use cmake::Config;

struct FakeDst();
impl FakeDst {
    fn display(&self) -> &'static str {
        return "/Users/n8henrie/git/taskwarrior-rs/target/debug/build/taskwarrior-sys-c885456ffaf97d6f/out";
    }
}

fn main() {
    // println!("cargo:rerun-if-changed=wrapper.h");
    // println!("cargo:rerun-if-changed=wrapper.cpp");

    let dst = Config::new("vendor/taskwarrior")
        .profile("release")
        .define("PROJECT_VERSION", "2.6.2")
        .build();
    // let dst = FakeDst();

    // let src = ["wrapper.cpp"];
    //     let src = ["vendor/taskwarrior/src/Context.cpp"];
    //     let includes = [
    //         "vendor/taskwarrior",
    //         "vendor/taskwarrior/src",
    //         "vendor/taskwarrior/src/commands",
    //         "vendor/taskwarrior/src/columns",
    //         "vendor/taskwarrior/src/libshared/src",
    //     ];
    //     cc::Build::new()
    //         .cpp(true)
    //         .files(src.iter())
    //         .includes(includes.iter())
    //         .flag("-std=c++17")
    //         .warnings(false)
    //         .compile("twrs");

    println!("cargo:rustc-link-search=native={}/build/src", dst.display());
    println!(
        "cargo:rustc-link-search=native={}/build/src/columns",
        dst.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/build/src/commands",
        dst.display()
    );
    println!("cargo:rustc-link-search=native=/opt/homebrew/opt/gnutls/lib");

    println!("cargo:rustc-link-lib=c++");
    println!("cargo:rustc-link-lib=static=columns");
    println!("cargo:rustc-link-lib=static=commands");
    println!("cargo:rustc-link-lib=static=task");
    println!("cargo:rustc-link-lib=static=libshared");
    println!("cargo:rustc-link-lib=dylib=gnutls");

    let bindings = bindgen::Builder::default()
        .header("vendor/taskwarrior/src/Context.h")
        .header("vendor/taskwarrior/src/Task.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_args([
            "-xc++",
            "-std=c++17",
            // "-I/Library/Developer/CommandLineTools/usr/include/c++/v1",
            "-Ivendor/taskwarrior/src",
            "-Ivendor/taskwarrior/src/commands",
            "-Ivendor/taskwarrior/src/columns",
            "-Ivendor/taskwarrior/src/libshared/src",
        ])
        .opaque_type("size_type")
        .allowlist_type("Context")
        .opaque_type("std::.*")
        // .derive_default(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
