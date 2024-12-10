use std::{env, path::PathBuf, process::Command};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let cargo_home = PathBuf::from(env::var("CARGO_HOME").unwrap());
    let target = env::var("TARGET").unwrap();

    println!("cargo:rerun-if-changed=build.rs");

    Command::new("git")
        .args([
            "clone",
            "https://github.com/fltk/fltk",
            "--depth=1",
        ])
        .current_dir(&out_dir)
        .status()
        .ok();

    let mut dst = cmake::Config::new(out_dir.join("fltk"));
    dst.profile("Release")
        .define("FLTK_BUILD_EXAMPLES", "OFF")
        .define("FLTK_BUILD_TEST", "OFF")
        .define("FLTK_BUILD_GL", "OFF")
        .define("FLTK_BUILD_HTML_DOCS", "OFF")
        .define("FLTK_BUILD_PDF_DOCS", "OFF");
    if !target.contains("windows") && !target.contains("apple") {
        dst.define("FLTK_GRAPHICS_CAIRO", "ON");
    }
    dst.build();

    let mut inp = out_dir.join("bin/fluid");
    let mut out = cargo_home.join("bin/fluid");

    if target.contains("windows") {
        inp.set_extension("exe");
        out.set_extension("exe");
    }

    if !target.contains("msvc") {
        Command::new("strip").arg(&inp).status().unwrap();
    }

    std::fs::copy(&inp, &out).unwrap();
}
