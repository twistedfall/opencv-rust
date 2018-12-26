extern crate cc;
extern crate glob;
extern crate pkg_config;

use glob::glob;
use std::ffi::OsString;
use std::fs;
use std::fs::{read_dir, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let opencv = pkg_config::Config::new().find("opencv").unwrap();
    let mut search_paths = opencv.include_paths.clone();
    search_paths.push(PathBuf::from("/usr/include"));
    let search_opencv = search_paths
        .iter()
        .map(|p| {
            let mut path = PathBuf::from(p);
            path.push("opencv2");
            path
        })
        .find({ |path| read_dir(path).is_ok() });
    let actual_opencv = search_opencv.expect("Could not find opencv2 dir in pkg-config includes");

    println!("OpenCV lives in {:?}", actual_opencv);
    println!("Generating code in {:?}", out_dir);
    println!("cargo:rerun-if-changed=gen_rust.py");
    println!("cargo:rerun-if-changed=hdr_parser.py");

    let mut gcc = cc::Build::new();
    gcc.cpp(true);
    gcc.flag("-std=c++0x");
    for path in opencv.include_paths {
        gcc.include(path);
    }

    for entry in glob(&(out_dir.clone() + "/*")).unwrap() {
        fs::remove_file(entry.unwrap()).unwrap()
    }

    let modules = vec![
        ("core", vec!["core/types_c.h", "core/core.hpp"]), // utility, base
        (
            "imgproc",
            vec![
                "imgproc/types_c.h",
                "imgproc/imgproc_c.h",
                "imgproc/imgproc.hpp",
            ],
        ),
        (
            "highgui",
            vec![
                "highgui/cap_ios.h",
                "highgui/highgui.hpp",
                "highgui/highgui_c.h",
                //"highgui/ios.h"
            ],
        ),
        ("features2d", vec!["features2d/features2d.hpp"]),
        ("photo", vec!["photo/photo_c.h", "photo/photo.hpp"]),
        (
            "video",
            vec![
                "video/tracking.hpp",
                "video/video.hpp",
                "video/background_segm.hpp",
            ],
        ),
        ("objdetect", vec!["objdetect/objdetect.hpp"]),
        ("calib3d", vec!["calib3d/calib3d.hpp"]),
    ];

    let mut types = PathBuf::from(&out_dir);
    types.push("common_opencv.h");
    {
        let mut types = File::create(types).unwrap();
        for ref m in modules.iter() {
            write!(&mut types, "#include <opencv2/{}/{}.hpp>\n", m.0, m.0).unwrap();
        }
    }

    let mut types = PathBuf::from(&out_dir);
    types.push("types.h");
    {
        let mut types = File::create(types).unwrap();
        write!(&mut types, "#include <cstddef>\n").unwrap();
    }

    for ref module in modules.iter() {
        let mut cpp = PathBuf::from(&out_dir);
        cpp.push(module.0);
        cpp.set_extension("cpp");

        if !Command::new("python2.7")
            .args(&["gen_rust.py", "hdr_parser.py", &*out_dir, module.0])
            .args(
                &(module
                    .1
                    .iter()
                    .map(|p| {
                        let mut path = actual_opencv.clone();
                        path.push(p);
                        path.into_os_string()
                    })
                    .collect::<Vec<OsString>>()[..]),
            )
            .status()
            .unwrap()
            .success()
        {
            panic!();
        }

        gcc.file(cpp);
    }

    let mut return_types = PathBuf::from(&out_dir);
    return_types.push("return_types.h");
    let mut hub_return_types = File::create(return_types).unwrap();
    for entry in glob(&(out_dir.clone() + "/cv_return_value_*.type.h")).unwrap() {
        writeln!(
            &mut hub_return_types,
            r#"#include "{}""#,
            entry.unwrap().file_name().unwrap().to_str().unwrap()
        )
        .unwrap();
    }

    for entry in glob("native/*.cpp").unwrap() {
        gcc.file(entry.unwrap());
    }
    for entry in glob(&(out_dir.clone() + "/*.type.cpp")).unwrap() {
        gcc.file(entry.unwrap());
    }

    gcc.include(".")
        .include(&out_dir)
        .flag("-Wno-c++11-extensions");

    gcc.compile("libocvrs.a");

    for ref module in &modules {
        let e = Command::new("sh")
            .current_dir(&out_dir)
            .arg("-c")
            .arg(format!(
                "g++ {}.consts.cpp -o {}.consts `pkg-config --cflags --libs opencv`",
                module.0, module.0
            ))
            .status()
            .unwrap();
        assert!(e.success());
        let e = Command::new("sh")
            .current_dir(&out_dir)
            .arg("-c")
            .arg(format!("./{}.consts > {}.consts.rs", module.0, module.0))
            .status()
            .unwrap();
        assert!(e.success());
    }

    let mut hub_filename = PathBuf::from(&out_dir);
    hub_filename.push("hub.rs");
    {
        let mut hub = File::create(hub_filename).unwrap();
        for ref module in &modules {
            writeln!(&mut hub, r#"pub mod {};"#, module.0).unwrap();
        }
        writeln!(&mut hub, r#"pub mod types {{"#).unwrap();
        writeln!(&mut hub, "  use libc::{{ c_void, c_char, size_t }};").unwrap();
        for entry in glob(&(out_dir.clone() + "/*.type.rs")).unwrap() {
            writeln!(
                &mut hub,
                r#"  include!(concat!(env!("OUT_DIR"), "/{}"));"#,
                entry.unwrap().file_name().unwrap().to_str().unwrap()
            )
            .unwrap();
        }
        writeln!(&mut hub, r#"}}"#).unwrap();
        writeln!(&mut hub, "#[doc(hidden)] pub mod sys {{").unwrap();
        writeln!(&mut hub, "  use libc::{{ c_void, c_char, size_t }};").unwrap();
        for entry in glob(&(out_dir.clone() + "/*.rv.rs")).unwrap() {
            writeln!(
                &mut hub,
                r#"  include!(concat!(env!("OUT_DIR"), "/{}"));"#,
                entry.unwrap().file_name().unwrap().to_str().unwrap()
            )
            .unwrap();
        }
        for ref module in &modules {
            writeln!(
                &mut hub,
                r#"  include!(concat!(env!("OUT_DIR"), "/{}.externs.rs"));"#,
                module.0
            )
            .unwrap();
        }
        writeln!(&mut hub, "}}\n").unwrap();
    }
    println!("cargo:rustc-link-lib=ocvrs");
}
