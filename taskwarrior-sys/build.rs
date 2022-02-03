use std::env;
use std::path::PathBuf;

use cmake::Config;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    let dst = Config::new("vendor/taskwarrior")
        .profile("release")
        .define("PROJECT_VERSION", "2.6.1")
        .build();

    let src = ["wrapper.cpp"];
    let includes = [
        "vendor/taskwarrior/src",
        "vendor/taskwarrior/src/commands",
        "vendor/taskwarrior/src/columns",
        "vendor/taskwarrior/src/libshared/src",
    ];
    cc::Build::new()
        .cpp(true)
        .files(src.iter())
        .includes(includes.iter())
        .flag("-std=c++17")
        .warnings(false)
        .compile("twrs");

    println!("cargo:rustc-link-search=native={}/build/src", dst.display());
    println!(
        "cargo:rustc-link-search=native={}/build/src/columns",
        dst.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/build/src/commands",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=task");
    println!("cargo:rustc-link-lib=static=libshared");
    println!("cargo:rustc-link-lib=static=columns");
    println!("cargo:rustc-link-lib=static=commands");
    println!("cargo:rustc-link-lib=c++");

    println!("cargo:rustc-link-search=native=/opt/homebrew/opt/gnutls/lib");
    println!("cargo:rustc-link-lib=dylib=gnutls");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .opaque_type("size_type")
        .opaque_type("std::.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
