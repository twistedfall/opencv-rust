#![feature(path_ext,convert)]

extern crate pkg_config;

use std::process::Command;
use std::path::{ Path, PathBuf} ;
use std::fs::PathExt;
use std::ffi::OsString;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let opencv = pkg_config::Config::new().find("opencv").unwrap();
    let search_opencv = opencv.include_paths.iter().map( |p| {
        let mut path = PathBuf::from(p);
        path.push("opencv2");
        path
    }).find( { |path| (*path).is_dir() });
    let actual_opencv = search_opencv
        .expect("Could not find opencv2 dir in pkg-config includes");

    println!("OpenCV lives in {:?}", actual_opencv);
    println!("Generating code in {:?}", out_dir);

    let modules = vec![
        ("core", vec!["core/core.hpp"])
    ];

    let mut objects:Vec<PathBuf> = Vec::new();

    for ref module in modules {
        let mut cpp = PathBuf::from(&out_dir);
        cpp.push(module.0);
        cpp.set_extension("cpp");

        if !Command::new("python")
                            .args(&["gen_rust.py", "hdr_parser.py", &*out_dir, module.0])
                            .args(&(module.1.iter().map(|p| {
                                let mut path = actual_opencv.clone();
                                path.push(p);
                                path.into_os_string()
                            }).collect::<Vec<OsString>>()[..]))
                           .status().unwrap().success() {
            panic!();
        }

        let mut object = cpp.clone();
        object.set_extension("o");
        println!("{:?}",cpp);
        println!("{:?}",object);

        if !Command::new("c++")
                    .args(&[cpp.to_str().unwrap(), "-c", "-o", object.to_str().unwrap(), "-I", "."])
                    .status().unwrap().success() {
            panic!();
        }
        objects.push(object);
    }


    if !Command::new("ar")
                        .args(&["crus", "libocvrs.a"])
                        .args(&(objects.iter().map(|p| p.clone().into_os_string()).collect::<Vec<OsString>>()))
                      .current_dir(Path::new(&out_dir))
                      .status().unwrap().success() {
        panic!();
    }

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=ocvrs");
    println!("cargo:rustc-link-lib=dylib=stdc++");

//    pkg_config::Config::new().find("opencv").unwrap();
}
