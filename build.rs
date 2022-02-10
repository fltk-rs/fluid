use std::{env, path::PathBuf, process::Command};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let cargo_home = PathBuf::from(env::var("CARGO_HOME").unwrap());
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    println!("cargo:rerun-if-changed=build.rs");

    Command::new("git")
        .args(&["submodule", "update", "--init", "--recursive"])
        .current_dir(manifest_dir.clone())
        .status()
        .expect("Git is needed to retrieve the fltk source files!");

    let mut dst = cmake::Config::new("fltk");
        dst.profile("Release")
        .define("CMAKE_EXPORT_COMPILE_COMMANDS", "ON")
        .define("FLTK_BUILD_EXAMPLES", "OFF")
        .define("FLTK_BUILD_TEST", "OFF")
        .define("OPTION_USE_SYSTEM_LIBPNG", "OFF")
        .define("OPTION_USE_SYSTEM_LIBJPEG", "OFF")
        .define("OPTION_USE_SYSTEM_ZLIB", "OFF")
        .define("OPTION_USE_THREADS", "ON")
        .define("OPTION_LARGE_FILE", "ON")
        .define("OPTION_BUILD_HTML_DOCUMENTATION", "OFF")
        .define("OPTION_BUILD_PDF_DOCUMENTATION", "OFF");
    dst.build();

    let mut inp = out_dir.join("bin/fluid");

    let mut out = cargo_home.join("bin/fltk-fluid");

    match target_os.as_str() {
        "windows" => {
            inp = inp.join(".exe");
            out = out.join(".exe");
        }
        _ => (),
    }

    std::fs::copy(&inp, &out).unwrap();
}
