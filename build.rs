use std::{env, path::PathBuf, process::Command};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_triple = env::var("TARGET").unwrap();

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

    if !target_triple.contains("apple")
        && !target_triple.contains("windows")
        && !target_triple.contains("android")
    {
        dst.define("OPTION_USE_SYSTEM_LIBPNG", "ON");
    }

    dst.build();

    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.join("build").display()
    );

    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.join("build").join("Release").display()
    );

    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.join("build").join("lib").display()
    );

    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.join("build").join("lib").join("Release").display()
    );

    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.join("lib").display()
    );

    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.join("lib64").display()
    );

    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.join("lib").join("Release").display()
    );

    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.join("lib64").join("Release").display()
    );

    println!("cargo:rustc-link-lib=static=fltk");
    println!("cargo:rustc-link-lib=static=fltk_images");
    println!("cargo:rustc-link-lib=static=fltk_z");
    println!("cargo:rustc-link-lib=static=fltk_jpeg");
    println!("cargo:rustc-link-lib=static=fltk_forms");
    println!("cargo:rustc-link-lib=static=fluid");

    if !target_triple.contains("apple")
        && !target_triple.contains("windows")
        && !target_triple.contains("android")
    {
        println!("cargo:rustc-link-lib=dylib=png");
    } else {
        println!("cargo:rustc-link-lib=static=fltk_png");
    }

    match target_os.as_str() {
        "macos" => {
            println!("cargo:rustc-link-lib=framework=Carbon");
            println!("cargo:rustc-link-lib=framework=Cocoa");
            println!("cargo:rustc-link-lib=framework=ApplicationServices");
            println!("cargo:rustc-link-lib=dylib=c++");
        }
        "windows" => {
            println!("cargo:rustc-link-lib=dylib=gdiplus");
            println!("cargo:rustc-link-lib=dylib=ws2_32");
            println!("cargo:rustc-link-lib=dylib=comctl32");
            println!("cargo:rustc-link-lib=dylib=gdi32");
            println!("cargo:rustc-link-lib=dylib=oleaut32");
            println!("cargo:rustc-link-lib=dylib=ole32");
            println!("cargo:rustc-link-lib=dylib=uuid");
            println!("cargo:rustc-link-lib=dylib=shell32");
            println!("cargo:rustc-link-lib=dylib=advapi32");
            println!("cargo:rustc-link-lib=dylib=comdlg32");
            println!("cargo:rustc-link-lib=dylib=winspool");
            println!("cargo:rustc-link-lib=dylib=user32");
            println!("cargo:rustc-link-lib=dylib=kernel32");
            println!("cargo:rustc-link-lib=dylib=odbc32");
            if cfg!(target_env = "gnu") {
                println!("cargo:rustc-link-lib=dylib=stdc++");
            }
        }
        _ => {
            println!("cargo:rustc-link-lib=dylib=pthread");
            println!("cargo:rustc-link-lib=dylib=X11");
            println!("cargo:rustc-link-lib=dylib=Xext");
            println!("cargo:rustc-link-lib=dylib=Xinerama");
            println!("cargo:rustc-link-lib=dylib=Xcursor");
            println!("cargo:rustc-link-lib=dylib=Xrender");
            println!("cargo:rustc-link-lib=dylib=Xfixes");
            println!("cargo:rustc-link-lib=dylib=Xft");
            println!("cargo:rustc-link-lib=dylib=fontconfig");
            println!("cargo:rustc-link-lib=dylib=stdc++");
        }
    }
}
