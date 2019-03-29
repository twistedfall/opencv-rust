use std::{ffi::OsString, fs::{self, File, read_dir}, io, io::Write, path::PathBuf, process::Command};
use std::collections::HashSet;
use std::iter::FromIterator;

use glob::glob;

fn link_wrapper() -> pkg_config::Library {
    let opencv = pkg_config::Config::new().probe("opencv").unwrap();

    // add 3rdparty lib dit. pkgconfig forgets it somehow.
    let third_party_dir = format!("{}/share/OpenCV/3rdparty/lib", pkg_config::Config::get_variable("opencv", "prefix").unwrap());
    println!("cargo:rustc-link-search=native={}", third_party_dir);

    for path in &opencv.link_paths {
        println!("cargo:rustc-link-search=native={}", path.to_str().unwrap());
    }

    // now, this is a nightmare.
    // opencv will embark these as .a when they are not available, or
    // use the one from the system
    // and some may appear in one or more variant (-llibtiff or -ltiff, depending on the system)
    fn lookup_lib(third_party_dir: &str, search: &str) {
        for prefix in &["lib", "liblib"] {
            for &path in vec![third_party_dir].iter().chain(&["/usr/lib", "/usr/local/lib", "/usr/lib/x86_64-linux-gnu/"]) {
                for ext in &[".a", ".dylib", ".so"] {
                    let name = format!("{}{}", prefix, search);
                    let filename = PathBuf::from(format!("{}/{}{}", path, name, ext));
                    if filename.exists() {
                        println!("cargo:rustc-link-lib={}", &name[3..]);
                        return;
                    }
                }
            }
        }
    }

    let third_party_deps = [
        "IlmImf",
        "tiff",
        "ippicv",
        "jpeg",
        "png",
        "jasper",
        "webp",
        "z",
    ];
    third_party_deps.iter().for_each(|&x| lookup_lib(&third_party_dir, x));

    opencv
}

fn build_wrapper(opencv: pkg_config::Library) {
    println!("cargo:rerun-if-changed=hdr_parser.py");
    println!("cargo:rerun-if-changed=gen_rust.py");

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let out_dir_as_str = out_dir.to_str().unwrap();
    let hub_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("src");
    let module_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("src/hub");

    let search_opencv = opencv.include_paths
        .iter()
        .chain(&[PathBuf::from("/usr/include")])
        .map(|p| {
            let mut path = PathBuf::from(p);
            path.push("opencv2");
            path
        })
        .find(|path| read_dir(path).is_ok());
    let opencv_dir = search_opencv.expect("Could not find opencv2 dir in pkg-config includes");

    println!("OpenCV lives in {}", opencv_dir.display());
    println!("Generating code in {}", out_dir_as_str);

    let mut gcc = cc::Build::new();
    gcc.cpp(true)
        .flag("-std=c++0x")
        .flag("-Wno-deprecated-declarations")
        .flag("-fno-strict-aliasing");
//        .flag("-Wno-c++11-extensions");
    for path in &opencv.include_paths {
        gcc.include(path);
    }

    for entry in glob(&format!("{}/*", out_dir_as_str)).unwrap() {
        fs::remove_file(entry.unwrap()).unwrap()
    }

    let opencv_dir_as_string = opencv_dir.to_string_lossy();
    let ignore_modules: HashSet<&'static str> = HashSet::from_iter([
        "aruco",
        "bgsegm",
        "hal",
        "hfs",
        "flann",
        "ippicv",
        "opencv",
        "opencv_modules",
        "optflow",
        "rgbd",
        "stereo",
        "surface_matching",
        "tracking",
        "ximgproc",
        "xobjdetect",
        "xphoto",
    ].iter().map(|x| *x));
    let ignore_header_suffix = [
        ".details.hpp",
        ".inl.hpp",
        "_inl.hpp",
        "_winrt.hpp",
        "core/cv_cpu_dispatch.h",
        "core/hal/intrin.hpp",
        "core/hal/intrin_avx.hpp",
        "core/hal/intrin_cpp.hpp",
        "core/hal/intrin_forward.hpp",
        "core/hal/intrin_neon.hpp",
        "core/hal/intrin_sse.hpp",
        "core/hal/intrin_sse_em.hpp",
        "core/hal/intrin_vsx.hpp",
        "core/ocl_genbase.hpp",
        "core/opengl.hpp",
        "cvstd.hpp",
        "ios.h",
        "ippasync.hpp",
        "ocl.hpp",
        "operations.hpp",
        "persistence.hpp",
        "utils/trace.hpp",
    ];
    let ignore_header_substring = [
        "/detail/",
        "/superres/",
        "core/opencl/",
        "cuda",
        "eigen",
        "private",
    ];
    let mut modules = glob(&format!("{}/*.hpp", opencv_dir_as_string)).unwrap()
        .map(|entry| {
             let entry = entry.unwrap();
             let mut files = vec!(entry.to_str().unwrap().to_string());

             let module = entry.file_stem().unwrap().to_string_lossy();

             files.extend(
                 glob(&format!("{}/{}/**/*.h*", opencv_dir_as_string, module)).unwrap()
                 .map(|file| file.unwrap().to_string_lossy().into_owned())
                 .filter(|f| !ignore_header_suffix.iter().any(|&x| f.ends_with(x)) && !ignore_header_substring.iter().any(|&x| f.contains(x)))
             );
             (module.into_owned(), files)
         })
        .filter(|module| !ignore_modules.contains(&module.0.as_ref()))
        .collect::<Vec<(String,Vec<String>)>>();

    if let Some(core_idx) = modules.iter().position(|x| x.0 == "core") {
        if core_idx != 0 {
            modules.swap(0, core_idx);
        }
    }

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
                        let mut path = opencv_dir.clone();
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
}

fn main() {
    build_wrapper(link_wrapper());
}
