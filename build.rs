use std::ffi::OsString;
use std::{fs, io};
use std::fs::{File, read_dir};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use glob::glob;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=gen_rust.py");
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let out_dir_as_str = out_dir.to_str().unwrap();
    let hub_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("src");
    let module_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("src/hub");

    let opencv = pkg_config::Config::new().cargo_metadata(false).probe("opencv").unwrap();
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

    // add 3rdparty lib dit. pkgconfig forgets it somehow.
    let third_party_dir = format!("{}/share/OpenCV/3rdparty/lib", pkg_config::Config::get_variable("opencv", "prefix").unwrap());
    println!("cargo:rustc-link-search=native={}", third_party_dir);

    println!("OpenCV lives in {}", opencv_dir.display());
    println!("Generating code in {}", out_dir_as_str);
    println!("cargo:rerun-if-changed=gen_rust.py");
    println!("cargo:rerun-if-changed=hdr_parser.py");

    for path in opencv.link_paths {
        println!("cargo:rustc-link-search=native={}", path.to_str().unwrap());
    }

    for lib in opencv.libs {
        if lib != "stdc++" {
            println!("cargo:rustc-link-lib={}", lib);
        }
    }

    let mut gcc = cc::Build::new();
    gcc.cpp(true);
    gcc.flag("-std=c++0x");
    for path in opencv.include_paths {
        gcc.include(path);
    }

    for entry in glob(&format!("{}/*", out_dir_as_str)).unwrap() {
        fs::remove_file(entry.unwrap()).unwrap()
    }

/*
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
*/

    let opencv_path_as_string = actual_opencv.to_str().unwrap().to_string();
    let modules = glob(&(opencv_path_as_string.clone()+"/*.hpp")).unwrap().map(|entry| {
        let entry = entry.unwrap();
        let mut files = vec!(entry.to_str().unwrap().to_string());

        let module = entry.file_stem().unwrap().to_str().unwrap().to_string();

        files.extend(glob(&(opencv_path_as_string.clone()+"/"+&*module+"/**/*.h*")).unwrap()
            .map(|file| file.unwrap().to_str().unwrap().to_string())
            .filter(|f|
                    !f.ends_with("cvstd.hpp")
                &&  !f.ends_with("cvstd.inl.hpp")
                &&  !f.ends_with("ocl.hpp")
                &&  !f.ends_with("operations.hpp")
                &&  !f.ends_with("persistence.hpp")
                &&  !f.ends_with("ippasync.hpp")
                &&  !f.ends_with("core/ocl_genbase.hpp")
                &&  !f.ends_with("core/opengl.hpp")
                &&  !f.ends_with("core/mat.inl.hpp")
                &&  !f.ends_with("core/hal/intrin_cpp.hpp")
                &&  !f.ends_with("core/hal/intrin_sse.hpp")
                &&  !f.ends_with("core/hal/intrin_neon.hpp")
                &&  !f.ends_with("ios.h")
                &&  !f.contains("cuda")
                &&  !f.contains("eigen")
                &&  !f.contains("/detail/")
                &&  !f.contains("private")
                &&  !f.contains("/superres/")
            ));
        (module, files)
    })
    .filter(|module|    module.0 != "flann"
                    &&  module.0 != "opencv_modules"
                    &&  module.0 != "opencv"
                    &&  module.0 != "ippicv"
                    &&  module.0 != "hal"
    )
    .collect::<Vec<(String,Vec<String>)>>();

    {
        let mut types = File::create(out_dir.join("common_opencv.h")).unwrap();
        for m in &modules {
            write!(&mut types, "#include <opencv2/{}.hpp>\n", m.0).unwrap();
            if m.0 == "dnn" {
                // include it manually, otherwise it's not included
                write!(&mut types, "#include <opencv2/{}/all_layers.hpp>\n", m.0).unwrap();
            }
        }
    }

    {
        let mut types = File::create(out_dir.join("types.h")).unwrap();
        write!(&mut types, "#include <cstddef>\n").unwrap();
    }

    for module in &modules {
        let mut cpp = out_dir.clone();
        cpp.push(&*module.0);
        cpp.set_extension("cpp");

        if !Command::new("python2.7")
            .args(&["gen_rust.py", "hdr_parser.py", out_dir_as_str, out_dir_as_str, &module.0])
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
        let e = Command::new("sh")
            .current_dir(&out_dir)
            .arg("-c")
            .arg(format!(
                "g++ {}.consts.cpp -o {}.consts `pkg-config --cflags --libs opencv` -L`pkg-config --variable=prefix opencv`/share/OpenCV/3rdparty/lib",
                module.0, module.0
            ))
            .status()
            .unwrap();
        assert!(e.success());
        let e = Command::new("sh")
            .current_dir(&out_dir)
            .arg("-c")
            .arg(format!("./{}.consts >> {}.rs", module.0, module.0))
            .status()
            .unwrap();
        assert!(e.success());
        let _ = fs::remove_file(out_dir.join(format!("{}.consts", module.0)));
        let _ = fs::remove_file(out_dir.join(format!("{}.consts.cpp", module.0)));
    }
    let _ = fs::remove_file("gen_rust.pyc");
    let _ = fs::remove_file("hdr_parser.pyc");

    {
        let mut hub_return_types = File::create(out_dir.join("return_types.h")).unwrap();
        for entry in glob(&format!("{}/cv_return_value_*.type.h", out_dir_as_str)).unwrap() {
            writeln!(
                &mut hub_return_types,
                r#"#include "{}""#,
                entry.unwrap().file_name().unwrap().to_str().unwrap()
            )
               .unwrap();
        }
    }

    for entry in glob("native/*.cpp").unwrap() {
        gcc.file(entry.unwrap());
    }
    for entry in glob(&format!("{}/*.type.cpp", out_dir_as_str)).unwrap() {
        gcc.file(entry.unwrap());
    }

    gcc.include(".")
        .include(&out_dir);

    gcc.compile("ocvrs");

    if !module_dir.exists() {
        fs::create_dir(&module_dir).unwrap();
    }

    for entry in glob(&format!("{}/*", module_dir.to_str().unwrap())).unwrap() {
        let _ = fs::remove_file(entry.unwrap());
    }

    {
        let mut hub = File::create(hub_dir.join("hub.rs")).unwrap();
        for ref module in &modules {
            writeln!(&mut hub, r#"pub mod {};"#, module.0).unwrap();
            fs::rename(out_dir.join(format!("{}.rs", module.0)), module_dir.join(format!("{}.rs", module.0))).unwrap();
        }
        writeln!(&mut hub, "pub mod types;").unwrap();
        writeln!(&mut hub, "#[doc(hidden)] pub mod sys;").unwrap();
    }

    {
        let mut types = File::create(module_dir.join("types.rs")).unwrap();
        writeln!(&mut types, "use libc::{{c_void, c_char, size_t}};").unwrap();
        writeln!(&mut types, "use crate::{{core, types}};").unwrap();
        for entry in glob(&format!("{}/*.type.rs", out_dir_as_str)).unwrap() {
            let entry = entry.unwrap();
            io::copy(&mut File::open(&entry).unwrap(), &mut types).unwrap();
            let _ = fs::remove_file(entry);
        }
    }

    {
        let mut sys = File::create(module_dir.join("sys.rs")).unwrap();
        writeln!(&mut sys, "use libc::{{c_void, c_char, size_t}};").unwrap();
        writeln!(&mut sys, "use crate::{{core}};").unwrap();
        for entry in glob(&format!("{}/*.rv.rs", out_dir_as_str)).unwrap() {
            let entry = entry.unwrap();
            io::copy(&mut File::open(&entry).unwrap(), &mut sys).unwrap();
            let _ = fs::remove_file(entry);
        }
        for module in &modules {
            let path = out_dir.join(format!("{}.externs.rs", module.0));
            io::copy(&mut File::open(&path).unwrap(), &mut sys).unwrap();
            let _ = fs::remove_file(path);
        }
    }

    // now, this is a nightmare.
    // opencv will embark these as .a when they are not available, or
    // use the one from the system
    // and some may appear in one or more variant (-llibtiff or -ltiff, depending on the system)
    fn lookup_lib(third_party_dir:&str, search: &str) {
        for prefix in vec![ "lib", "liblib" ] {
            for path in vec![ third_party_dir, "/usr/lib", "/usr/local/lib", "/usr/lib/x86_64-linux-gnu/" ] {
                for ext in vec!(".a", ".dylib", ".so") {
                    let name = prefix.to_string() + search;
                    let filename = path.to_string() + "/" + &*name + ext;
                    if fs::metadata(filename.clone()).is_ok() {
                        println!("cargo:rustc-link-lib={}", &name[3..]);
                        return
                    }
                }
            }
        }
    }

    lookup_lib(&*third_party_dir, "IlmImf");
    lookup_lib(&*third_party_dir, "tiff");
    lookup_lib(&*third_party_dir, "ippicv");
    lookup_lib(&*third_party_dir, "jpeg");
    lookup_lib(&*third_party_dir, "png");
    lookup_lib(&*third_party_dir, "jasper");
    lookup_lib(&*third_party_dir, "webp");

    println!("cargo:rustc-link-lib=z");
}

