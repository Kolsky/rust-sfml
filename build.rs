use cmake::Config;
use std::env;

fn main() {
    // Builds the project in the directory located in `libfoo`, installing it
    // into $OUT_DIR
    let dst = Config::new("CSFML")
        .define("BUILD_SHARED_LIBS", "FALSE")
        .profile("Release")
        .build();
    println!("cargo:warning=CMake output is in {}", dst.display());

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-flags=-lstdc++");
    println!("cargo:rustc-link-lib=static=csfml-system");
    println!("cargo:rustc-link-lib=dylib=sfml-system");
    if env::var("CARGO_FEATURE_AUDIO").is_ok() {
        println!("cargo:rustc-link-lib=static=csfml-audio");
        println!("cargo:rustc-link-lib=dylib=sfml-audio");
    }
    if env::var("CARGO_FEATURE_WINDOW").is_ok() {
        println!("cargo:rustc-link-lib=static=csfml-window");
        println!("cargo:rustc-link-lib=dylib=sfml-window");
    }
    if env::var("CARGO_FEATURE_GRAPHICS").is_ok() {
        println!("cargo:rustc-link-lib=static=csfml-graphics");
        println!("cargo:rustc-link-lib=dylib=sfml-graphics");
    }
}