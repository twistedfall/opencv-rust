use std::{
    collections::HashSet,
    env,
    error::Error,
    ffi::OsString,
    fs::{self, File, OpenOptions},
    io::{self, Write},
    iter::FromIterator,
    path::PathBuf,
    process::Command,
};

use once_cell::sync::OnceCell;
use rayon::prelude::*;
use semver::{Version, VersionReq};

use glob::glob;

static MODULES: OnceCell<Vec<(String, Vec<String>)>> = OnceCell::new();

fn get_versioned_hub_dir() -> PathBuf {
    let mut hub_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("Can't read CARGO_MANIFEST_DIR env var"));
    hub_dir.push("src");
    if cfg!(feature = "opencv-32") {
        hub_dir.push("opencv_32");
    } else if cfg!(feature = "opencv-34") {
        hub_dir.push("opencv_34");
    } else if cfg!(feature = "opencv-41") {
        hub_dir.push("opencv_41");
    }
    hub_dir
}

fn get_modules(opencv_dir_as_string: &str) -> Result<&'static Vec<(String, Vec<String>)>, Box<dyn Error>> {
    if let Some(modules) = MODULES.get() {
        return Ok(modules);
    }
    let ignore_modules: HashSet<&'static str> = HashSet::from_iter([
        "aruco",
        "bgsegm",
        "core_detect",
        "face",
        "flann",
        "gapi",
        "hal",
        "hdf", // includes platform-specific headers like /usr/include/x86_64-pc-linux-gnu/opencv2/hdf/hdf5.hpp
        "hfs",
        "ippicv",
        "line_descriptor",
        "opencv",
        "opencv_modules",
        "optflow",
        "quality",
        "rgbd",
        "saliency",
        "stereo",
        "structured_light",
        "surface_matching",
        "text",
        "tracking",
        "xfeatures2d", // only appears in some builds, maybe platform or opencv compile flag specific
        "ximgproc",
        "xobjdetect",
        "xphoto",
    ].iter().map(|x| *x));
    let ignore_header_suffix = [
        ".details.hpp",
        ".inl.hpp",
        "hal.hpp",
        "_c.h",
        "core/cv_cpu_dispatch.h", // ?
        "core/ocl_genbase.hpp", // ?
        "core/opengl.hpp", // ?
        "core/cvstd.hpp", // contains functions with Rust native counterparts and c++ specific classes
        "core/cvstd_wrapper.hpp",
        "core/eigen.hpp",
        "core/fast_math.hpp", // contains functions with Rust native counterparts
        "core/private.hpp",
        "core/utils/filesystem.hpp", // contains functions with Rust native counterparts
        "dnn/blob.hpp",
        "ios.h",
        "ippasync.hpp",
        "ocl.hpp",
        "operations.hpp",
        "persistence.hpp",
        "utils/trace.hpp",
        "viz/widget_accessor.hpp",  // want to include vtk header
    ];
    let ignore_header_substring = [
        "/detail/",
        "/superres/",
        "core/hal/intrin",
        "core/opencl/",
        "cuda",
    ];

    let mut modules = glob(&format!("{}/*.hpp", opencv_dir_as_string))?
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let module = entry.file_stem().unwrap().to_string_lossy();
            if ignore_modules.contains(module.as_ref()) {
                None
            } else {
                let mut files = vec![entry.to_string_lossy().into_owned()];
                files.extend(
                    glob(&format!("{}/{}/**/*.h*", opencv_dir_as_string, module)).unwrap()
                        .filter_map(|file| {
                            let f = file.unwrap();
                            let f = f.to_string_lossy();
                            if !ignore_header_suffix.iter().any(|&x| f.ends_with(x)) && !ignore_header_substring.iter().any(|&x| f.contains(x)) {
                                Some(f.into_owned())
                            } else {
                                None
                            }
                        })
                );
                Some((module.into_owned(), files))
            }
        })
        .collect::<Vec<_>>();

    if let Some(core_idx) = modules.iter().position(|x| x.0 == "core") {
        if core_idx != 0 {
            modules.swap(0, core_idx);
        }
    }
    MODULES.set(modules).expect("Cannot assign module cache");
    Ok(MODULES.get().unwrap())
}

fn build_pkg_config_args((opencv, pkg_name): (&pkg_config::Library, &str)) -> Vec<String> {
    let mut out = Vec::with_capacity(60);
    opencv.link_paths.iter().for_each(|x| out.push(format!("-L{}", x.to_string_lossy())));
    if let Ok(prefix) = pkg_config::get_variable(pkg_name, "prefix") {
        out.push(format!("-L{}/share/OpenCV/3rdparty/lib", prefix))
    }
    if let Ok(libdir) = pkg_config::get_variable(pkg_name, "libdir") {
        out.push(format!("-L{}/{}/3rdparty", libdir, pkg_name));
    }
    opencv.framework_paths.iter().for_each(|x| out.push(format!("-F{}", x.to_string_lossy())));
    opencv.include_paths.iter().for_each(|x| out.push(format!("-I{}", x.to_string_lossy())));
    opencv.libs.iter().for_each(|x| out.push(format!("-l{}", x)));
    out
}

fn build_compiler(opencv_header_dir: &PathBuf) -> cc::Build {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("Can't get OUT_DIR env var"));
    let mut out = cc::Build::new();
    out.cpp(true)
        .flag("-std=c++11")
        .flag_if_supported("-Wno-deprecated-declarations")
        .flag_if_supported("-Wno-class-memaccess")
        .include(opencv_header_dir)
        .include(&out_dir)
        .include(".");
    out
}

fn link_wrapper() -> Result<(pkg_config::Library, &'static str), Box<dyn Error>> {
    let (opencv, pkg_name) = if cfg!(feature = "opencv-32") || cfg!(feature = "opencv-34") {
        let pkg_name = "opencv";
        let opencv = pkg_config::probe_library(pkg_name).expect("package opencv is not found by pkg-config");
        if cfg!(feature = "opencv-32") && !VersionReq::parse("~3.2")?.matches(&Version::parse(&opencv.version)?) {
            panic!("OpenCV version from pkg-config: {} must be from 3.2 branch because of the feature: opencv-32", opencv.version);
        }
        if cfg!(feature = "opencv-34") && !VersionReq::parse("~3.4")?.matches(&Version::parse(&opencv.version)?) {
            panic!("OpenCV version from pkg-config: {} must be from 3.4 branch because of the feature: opencv-34", opencv.version);
        }
        (opencv, pkg_name)
    } else if cfg!(feature = "opencv-41") {
        let pkg_name = "opencv4";
        let opencv = pkg_config::probe_library(pkg_name).expect("package opencv4 is not found by pkg-config");
        if !VersionReq::parse("~4.1")?.matches(&Version::parse(&opencv.version)?) {
            panic!("OpenCV version from pkg-config: {} must be from 4.1 branch because of the feature: opencv-41", opencv.version);
        }
        (opencv, pkg_name)
    } else {
        unreachable!("Cannot be until we allow custom headers");
    };

    eprintln!("=== Using OpenCV library version: {} from: {}", opencv.version, pkg_config::get_variable(pkg_name, "libdir")?);

    // add 3rdparty lib dit. pkgconfig forgets it somehow.
    let third_party_dir1 = format!("{}/share/OpenCV/3rdparty/lib", pkg_config::get_variable(pkg_name, "prefix")?);
    println!("cargo:rustc-link-search=native={}", third_party_dir1);
    let third_party_dir2 = format!("{}/{}/3rdparty", pkg_config::get_variable(pkg_name, "libdir")?, pkg_name);
    println!("cargo:rustc-link-search=native={}", third_party_dir2);
    let third_party_dirs: [&str; 2] = [&third_party_dir1, &third_party_dir2];

    // now, this is a nightmare.
    // opencv will embark these as .a when they are not available, or
    // use the one from the system
    // and some may appear in one or more variant (-llibtiff or -ltiff, depending on the system)
    fn lookup_lib(third_party_dirs: &[&str], search: &str) {
        for prefix in &["lib", "liblib"] {
            for &path in third_party_dirs.iter().chain(&["/usr/lib", "/usr/lib64", "/usr/local/lib", "/usr/local/lib64", "/usr/lib/x86_64-linux-gnu/"]) {
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
        "ippiw",
        "ippicv",
        "ittnotify",
        "jpeg",
        "jpeg-turbo",
        "png",
        "jasper",
        "tbb",
        "webp",
        "z",
        "zlib",
    ];
    third_party_deps.iter().for_each(|&x| lookup_lib(&third_party_dirs, x));

    Ok((opencv, pkg_name))
}

fn build_wrapper(opencv_header_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=hdr_parser.py");
    println!("cargo:rerun-if-changed=gen_rust.py");

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let out_dir_as_str = out_dir.to_str().unwrap();

    let opencv_dir = opencv_header_dir.join("opencv2");

    eprintln!("=== Using OpenCV headers from: {}", opencv_dir.display());
    eprintln!("=== Generating code in: {}", out_dir_as_str);

    for entry in glob(&format!("{}/*", out_dir_as_str))? {
        let _ = fs::remove_file(entry?);
    }

    let modules = get_modules(&opencv_dir.to_string_lossy())?;

    {
        let mut types = File::create(out_dir.join("common_opencv.h"))?;
        for m in modules {
            write!(&mut types, "#include <opencv2/{}.hpp>\n", m.0)?;
            if m.0 == "dnn" {
                // include it manually, otherwise it's not included
                if cfg!(feature = "opencv-41") {
                    write!(&mut types, "#include <opencv2/{}/version.hpp>\n", m.0)?;
                }
                write!(&mut types, "#include <opencv2/{}/all_layers.hpp>\n", m.0)?;
            }
        }
    }

    {
        let mut types = File::create(out_dir.join("types.h"))?;
        write!(&mut types, "#include <cstddef>\n")?;
    }

    let version = if cfg!(feature = "opencv-32") {
        "3.2.0"
    } else if cfg!(feature = "opencv-34") {
        "3.4.6"
    } else if cfg!(feature = "opencv-41") {
        "4.1.0"
    } else {
        unreachable!();
    };
    modules.par_iter().for_each(|module| {
        if !Command::new("python3")
            .args(&["-B", "gen_rust.py", "hdr_parser.py", out_dir_as_str, out_dir_as_str, &module.0, &version])
            .args(
                module
                    .1
                    .iter()
                    .map(|p| {
                        let mut path = opencv_dir.clone();
                        path.push(p);
                        path.into_os_string()
                    })
                    .collect::<Vec<OsString>>().as_slice(),
            )
            .status()
            .unwrap()
            .success()
        {
            panic!();
        }
    });

    let mut cc = build_compiler(opencv_header_dir);

    {
        let mut types_file = File::create(out_dir.join("types.h"))?;
        for module in modules {
            cc.file(out_dir.join(format!("{}.cpp", module.0)));
            let src = out_dir.join(format!("{}.types.h", module.0));
            io::copy(&mut File::open(&src)?, &mut types_file)?;
            let _ = fs::remove_file(src);
        }
    }

    {
        let mut hub_return_types = File::create(out_dir.join("return_types.h"))?;
        for entry in glob(&format!("{}/cv_return_value_*.type.h", out_dir_as_str))? {
            writeln!(
                &mut hub_return_types,
                r#"#include "{}""#,
                entry.unwrap().file_name().unwrap().to_str().unwrap()
            )?;
        }
    }

    for entry in glob(&format!("{}/*.type.cpp", out_dir_as_str))? {
        cc.file(entry?);
    }

    cc.compile("ocvrs");
    Ok(())
}

fn install_wrapper() -> Result<(), Box<dyn Error>> {
    let src_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("src");
    let hub_dir = get_versioned_hub_dir();
    let target_hub_dir = src_dir.join("opencv");
    let target_module_dir = target_hub_dir.join("hub");
    for entry in glob(&format!("{}/*.rs", target_module_dir.to_str().unwrap()))? {
        let _ = fs::remove_file(entry?);
    }
    for entry in glob(&format!("{}/**/*.rs", hub_dir.to_str().unwrap())).unwrap() {
        let entry: PathBuf = entry?;
        let target_file = target_hub_dir.join(entry.strip_prefix(&hub_dir)?);
        if let Some(target_dir) = target_file.parent() {
            if !target_dir.exists() {
                fs::create_dir_all(target_dir)?;
            }
        }
        fs::copy(&entry, target_file)?;
    }
    Ok(())
}

fn gen_wrapper((opencv, pkg_name): (&pkg_config::Library, &str), opencv_header_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let out_dir_as_str = out_dir.to_str().unwrap();
    let src_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("src");
    let hub_dir = get_versioned_hub_dir();
    let module_dir = hub_dir.join("hub");
    let manual_dir = src_dir.join("manual");
    let opencv_dir = opencv_header_dir.join("opencv2");

    eprintln!("=== Placing generated bindings into: {}", hub_dir.display());

    let compiler = build_compiler(opencv_header_dir).get_compiler();
    let pkg_config_args = build_pkg_config_args((opencv, pkg_name));
    let modules = get_modules(&opencv_dir.to_string_lossy())?;
    modules.par_iter().for_each(|module| {
        let e = compiler.to_command()
            .current_dir(&out_dir)
            .args(&[
                format!("{}.consts.cpp", module.0),
                "-o".into(),
                format!("{}.consts", module.0)
            ])
            .args(&pkg_config_args)
            .status()
            .unwrap();
        assert!(e.success());
        let output = Command::new(format!("./{}.consts", module.0))
            .current_dir(&out_dir)
            .output()
            .unwrap();
        assert!(output.status.success());
        {
            let mut module_file = OpenOptions::new().append(true).open(out_dir.join(format!("{}.rs", module.0))).expect("Cannot open module file for append");
            io::copy(&mut output.stdout.as_slice(), &mut module_file).expect("Cannot write constant data to module file");
        }
    });

    if !module_dir.exists() {
        fs::create_dir(&module_dir)?;
    }

    for entry in glob(&format!("{}/*.rs", module_dir.to_str().unwrap()))? {
        let _ = fs::remove_file(entry?);
    }

    let add_manual = |file: &mut File, mod_name: &str| -> Result<bool, Box<dyn Error>> {
        if manual_dir.join(format!("{}.rs", mod_name)).exists() {
            writeln!(file, "pub use crate::manual::{}::*;", mod_name)?;
            Ok(true)
        } else {
            Ok(false)
        }
    };

    {
        let mut hub = File::create(hub_dir.join("hub.rs"))?;
        for ref module in modules {
            writeln!(&mut hub, "pub mod {};", module.0)?;
            let module_filename = format!("{}.rs", module.0);
            let target_file = module_dir.join(&module_filename);
            fs::rename(out_dir.join(&module_filename), &target_file)?;
            let mut f = OpenOptions::new().append(true).open(&target_file)?;
            add_manual(&mut f, &module.0)?;
        }
        writeln!(&mut hub, "pub mod types;")?;
        writeln!(&mut hub, "#[doc(hidden)] pub mod sys;")?;
    }

    {
        let mut types = File::create(module_dir.join("types.rs"))?;
        writeln!(&mut types, "use std::os::raw::{{c_char, c_void}};")?;
        writeln!(&mut types, "use libc::size_t;")?;
        writeln!(&mut types, "use crate::{{core, types}};")?;
        writeln!(&mut types, "")?;
        for entry in glob(&format!("{}/*.type.rs", out_dir_as_str))? {
            let entry = entry?;
            io::copy(&mut File::open(&entry)?, &mut types)?;
        }
        add_manual(&mut types, "types")?;
    }

    {
        let mut sys = File::create(module_dir.join("sys.rs"))?;
        writeln!(&mut sys, "use std::os::raw::{{c_char, c_void}};")?;
        writeln!(&mut sys, "use libc::size_t;")?;
        writeln!(&mut sys, "use crate::core;")?;
        writeln!(&mut sys, "")?;
        for entry in glob(&format!("{}/*.rv.rs", out_dir_as_str))? {
            let entry = entry?;
            io::copy(&mut File::open(&entry)?, &mut sys)?;
        }
        for module in modules {
            let path = out_dir.join(format!("{}.externs.rs", module.0));
            io::copy(&mut File::open(&path)?, &mut sys)?;
        }
        add_manual(&mut sys, "sys")?;
    }

    Ok(())
}

fn cleanup(opencv_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let modules = get_modules(&opencv_dir.to_string_lossy())?;
    modules.par_iter().for_each(|module| {
        let _ = fs::remove_file(out_dir.join(format!("{}.consts", module.0)));
        let _ = fs::remove_file(out_dir.join(format!("{}.consts.cpp", module.0)));
    });
    for entry in glob(&format!("{}/*.rs", out_dir.to_string_lossy()))? {
        let _ = fs::remove_file(entry?);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let features = [cfg!(feature = "opencv-32"), cfg!(feature = "opencv-34"), cfg!(feature = "opencv-41")].iter().map(|&x| if x { 1 } else { 0 }).sum::<i32>();
    if features != 1 {
        // todo: allow building with custom headers
        panic!("Please select exactly one of the features: opencv-32, opencv-34, opencv-41");
    }
    let opencv_header_dir = env::var("OPENCV_HEADER_DIR").map(PathBuf::from).unwrap_or_else(|_| {
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set"));
        if cfg!(feature = "opencv-32") {
            manifest_dir.join("headers/3.2")
        } else if cfg!(feature = "opencv-34") {
            manifest_dir.join("headers/3.4")
        } else if cfg!(feature = "opencv-41") {
            manifest_dir.join("headers/4.1")
        } else {
            panic!("Please select one OpenCV major version using one of the opencv-* features or specify OpenCV header path manually via OPENCV_HEADER_DIR environment var");
        }
    });

    build_wrapper(&opencv_header_dir)?;
    if !cfg!(feature = "docs-only") {
        let opencv = link_wrapper()?;
        if cfg!(feature = "buildtime-bindgen") {
            gen_wrapper((&opencv.0, opencv.1), &opencv_header_dir)?;
        }
    }
    install_wrapper()?;
    cleanup(&opencv_header_dir)?;

    let mut config = cpp_build::Config::new();

    config.flag_if_supported("-Wno-class-memaccess");
    config.flag_if_supported("-Wno-ignored-qualifiers");
    config.include(opencv_header_dir);
    config.build("src/lib.rs");
    Ok(())
}
