extern crate pkg_config;

use std::process::Command;
use std::path::{ Path, PathBuf} ;
use std::fs::{ File, read_dir };
use std::ffi::OsString;
use std::io::Write;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let opencv = pkg_config::Config::new().find("opencv").unwrap();
    let search_opencv = opencv.include_paths.iter().map( |p| {
        let mut path = PathBuf::from(p);
        path.push("opencv2");
        path
    }).find( { |path| read_dir(path).is_ok() });
    let actual_opencv = search_opencv
        .expect("Could not find opencv2 dir in pkg-config includes");

    println!("OpenCV lives in {:?}", actual_opencv);
    println!("Generating code in {:?}", out_dir);

    let modules = vec![
        ("core", vec!["core/core.hpp" ]), // utility, base
        ("imgproc", vec![   "imgproc/imgproc_c.h", "imgproc/types_c.h",
                            "imgproc/imgproc.hpp" ]),
        ("highgui", vec![   "highgui/cap_ios.h", "highgui/highgui.hpp",
                            "highgui/highgui_c.h", "highgui/ios.h" ]),
        ("features2d", vec![ "features2d/features2d.hpp" ]),
        ("photo", vec!["photo/photo_c.h", "photo/photo.hpp" ]),
        ("video", vec![ "video/tracking.hpp", "video/video.hpp",
                        "video/background_segm.hpp"]),
        ("objdetect", vec![ "objdetect/objdetect.hpp" ]),
        ("calib3d", vec![ "calib3d/calib3d.hpp"])
    ];

    let mut objects:Vec<PathBuf> = Vec::new();

    for ref module in modules.iter() {
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

        if !Command::new("c++")
                    .args(&[cpp.to_str().unwrap(), "-c", "-o", object.to_str().unwrap(), "-I", "."])
                    .status().unwrap().success() {
            panic!();
        }
        objects.push(object);
    }

    let mut object = PathBuf::from(&out_dir);
    object.push("cv.o");
    if !Command::new("c++")
                .args(&["src/cv.cpp", "-c", "-o", object.to_str().unwrap(), "-I", "."])
                .status().unwrap().success() {
        panic!();
    }
    objects.push(object);

    if !Command::new("ar")
                        .args(&["crus", "libocvrs.a"])
                        .args(&(objects.iter().map(|p| p.clone().into_os_string()).collect::<Vec<OsString>>()))
                      .current_dir(Path::new(&out_dir))
                      .status().unwrap().success() {
        panic!();
    }

    let mut hub_filename = PathBuf::from(&out_dir);
    hub_filename.push("hub.rs");
    {
        let mut hub = File::create(hub_filename).unwrap();
        for ref module in modules {
            write!(&mut hub, r#"include!(concat!(env!("OUT_DIR"), "/{}.rs"));"#, module.0).unwrap();
            write!(&mut hub, "\n").unwrap();
        }
    }

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=ocvrs");
    println!("cargo:rustc-link-lib=dylib=stdc++");

//    pkg_config::Config::new().find("opencv").unwrap();
}
