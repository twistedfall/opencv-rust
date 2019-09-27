use std::{
    borrow::Cow,
    collections::HashSet,
    env,
    ffi::OsString,
    fs::{self, File, OpenOptions},
    io::{self, Write},
    iter::FromIterator,
    path::{self, PathBuf},
    process::Command,
};

use once_cell::sync::OnceCell;
use rayon::prelude::*;
use semver::{Version, VersionReq};

use glob_crate::glob;
use which_crate::which;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

static MODULES: OnceCell<Vec<(String, Vec<PathBuf>)>> = OnceCell::new();

trait CompilerFlagSetter {
    fn flag_if_supported(&mut self, flag: &str) -> &mut Self;
}

impl CompilerFlagSetter for cc::Build {
    #[inline(always)]
    fn flag_if_supported(&mut self, flag: &str) -> &mut Self {
        self.flag_if_supported(flag)
    }
}

impl CompilerFlagSetter for cpp_build::Config {
    #[inline(always)]
    fn flag_if_supported(&mut self, flag: &str) -> &mut Self {
        self.flag_if_supported(flag)
    }
}

fn set_compiler_flags<T: CompilerFlagSetter>(cc: &mut T) {
    cc.flag_if_supported("-fno-strict-aliasing")
        .flag_if_supported("-Wno-class-memaccess")
        .flag_if_supported("-Wno-deprecated-declarations")
        .flag_if_supported("-Wno-ignored-qualifiers");
}

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

fn get_modules(opencv_dir_as_string: &str) -> Result<&'static Vec<(String, Vec<PathBuf>)>> {
    if let Some(modules) = MODULES.get() {
        return Ok(modules);
    }
    let ignore_modules: HashSet<&'static str> = HashSet::from_iter([
        "core_detect",
        "cudalegacy",
        "cudev",
        "face",
        "flann",
        "gapi",
        "hal",
        "hfs",
        "ippicv",
        "opencv",
        "opencv_modules",
        "optflow",
        "quality",
        "rgbd",
        "saliency",
        "stereo",
        "surface_matching",
        "text",
        "tracking",
        "ximgproc",
    ].iter().map(|x| *x));
    let ignore_header_suffix = [
        ".inl.hpp",
        "_c.h",
    ];
    let ignore_header_substring = [
        "stitching/detail/",
        "/superres/",
        "core/hal/intrin",
        "core/opencl/",
        "cuda",
    ];
    let ignore_header_files = [
        PathBuf::from("core/cv_cpu_dispatch.h"), // ?
        PathBuf::from("core/cvstd.hpp"), // contains functions with Rust native counterparts and c++ specific classes
        PathBuf::from("core/cvstd_wrapper.hpp"),
        PathBuf::from("core/eigen.hpp"),
        PathBuf::from("core/fast_math.hpp"), // contains functions with Rust native counterparts
        PathBuf::from("core/ippasync.hpp"),
        PathBuf::from("core/opengl.hpp"), // ?
        PathBuf::from("core/operations.hpp"),
        PathBuf::from("core/private.hpp"),
        PathBuf::from("core/utils/allocator_stats.impl.hpp"),
        PathBuf::from("core/utils/filesystem.hpp"), // contains functions with Rust native counterparts
        PathBuf::from("core/utils/trace.hpp"),
        PathBuf::from("dnn/layer.details.hpp"),
        PathBuf::from("hal.hpp"), // ?
        PathBuf::from("imgcodecs/ios.h"),
        PathBuf::from("videoio/cap_ios.h"),
        PathBuf::from("viz/widget_accessor.hpp"), // wants to include vtk header
    ];

    let mut modules: Vec<(String, Vec<PathBuf>)> = glob(&format!("{}/*.hpp", opencv_dir_as_string))?
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let module: String = entry.file_stem().unwrap().to_string_lossy().into_owned();
            if ignore_modules.contains(module.as_str()) {
                None
            } else {
                let mut files = vec![entry];
                files.extend(
                    glob(&format!("{}/{}/**/*.h*", opencv_dir_as_string, module)).unwrap()
                        .filter_map(|file| {
                            let path = file.unwrap();
                            let path_str = path.to_string_lossy();
                            if !ignore_header_files.iter().any(|x| path.ends_with(x))
                                && !ignore_header_suffix.iter().any(|&x| path_str.ends_with(x))
                                && !ignore_header_substring.iter().any(|&x| path_str.contains(&x.replace('/', &path::MAIN_SEPARATOR.to_string()))) {
                                Some(path)
                            } else {
                                None
                            }
                        })
                );
                Some((module, files))
            }
        })
        .collect();

    let module_order = ["core"];
    let header_file_order = [
        PathBuf::from("core/cvdef.h"),
        PathBuf::from("core/version.hpp"),
        PathBuf::from("core/base.hpp"),
        PathBuf::from("core/cvstd.hpp"),
        PathBuf::from("core/traits.hpp"),
        PathBuf::from("core/matx.hpp"),
        PathBuf::from("core/types.hpp"),
        PathBuf::from("core/mat.hpp"),
        PathBuf::from("core/persistence.hpp"),
        PathBuf::from("aruco/dictionary.hpp"),
        PathBuf::from("dnn/blob.hpp"),
    ];

    modules.sort_by_key(|(mod_name, ..)| module_order.iter().position(|&order_module| order_module == mod_name).unwrap_or_else(|| module_order.len()));
    for (.., file_list) in &mut modules {
        file_list.sort_by_key(|header| header_file_order.iter().position(|order_header| header.ends_with(order_header)).unwrap_or_else(|| header_file_order.len()));
    }

    MODULES.set(modules).expect("Cannot set MODULES cache");
    Ok(MODULES.get().unwrap())
}

fn build_pkg_config_args((opencv, pkg_name): (&pkg_config::Library, &str)) -> Vec<String> {
    let mut out = Vec::with_capacity(60);
    out.extend(opencv.link_paths.iter().map(|x| format!("-L{}", x.to_string_lossy())));
    if let Ok(prefix) = pkg_config::get_variable(pkg_name, "prefix") {
        out.push(format!("-L{}/share/OpenCV/3rdparty/lib", prefix))
    }
    if let Ok(libdir) = pkg_config::get_variable(pkg_name, "libdir") {
        out.push(format!("-L{}/{}/3rdparty", libdir, pkg_name));
    }
    out.extend(opencv.framework_paths.iter().map(|x| format!("-F{}", x.to_string_lossy())));
    out.extend(opencv.include_paths.iter().map(|x| format!("-I{}", x.to_string_lossy())));
    out.extend(opencv.libs.iter().map(|x| format!("-l{}", x)));
    out
}

fn build_compiler(opencv_header_dir: &PathBuf) -> cc::Build {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("Can't get OUT_DIR env var"));
    let mut out = cc::Build::new();
    set_compiler_flags(&mut out);
    out.cpp(true)
        .flag("-std=c++11")
        .include(opencv_header_dir)
        .include(&out_dir)
        .include(".");
    out
}

fn link_wrapper() -> Result<(pkg_config::Library, String)> {
    let (opencv, pkg_name) = if cfg!(feature = "opencv-32") || cfg!(feature = "opencv-34") {
        let pkg_name = env::var("OPENCV_PKGCONFIG_NAME").map(|x| Cow::Owned(x)).unwrap_or_else(|_| Cow::Borrowed("opencv"));
        let opencv = pkg_config::probe_library(&pkg_name).expect(&format!("package {} is not found by pkg-config", pkg_name));
        if cfg!(feature = "opencv-32") && !VersionReq::parse("~3.2")?.matches(&Version::parse(&opencv.version)?) {
            panic!("OpenCV version from pkg-config: {} must be from 3.2 branch because of the feature: opencv-32", opencv.version);
        }
        if cfg!(feature = "opencv-34") && !VersionReq::parse("~3.4")?.matches(&Version::parse(&opencv.version)?) {
            panic!("OpenCV version from pkg-config: {} must be from 3.4 branch because of the feature: opencv-34", opencv.version);
        }
        (opencv, pkg_name)
    } else if cfg!(feature = "opencv-41") {
        let pkg_name = env::var("OPENCV_PKGCONFIG_NAME").map(|x| Cow::Owned(x)).unwrap_or_else(|_| Cow::Borrowed("opencv4"));
        let opencv = pkg_config::probe_library(&pkg_name).expect(&format!("package {} is not found by pkg-config", pkg_name));
        if !VersionReq::parse("~4.1")?.matches(&Version::parse(&opencv.version)?) {
            panic!("OpenCV version from pkg-config: {} must be from 4.1 branch because of the feature: opencv-41", opencv.version);
        }
        (opencv, pkg_name)
    } else {
        unreachable!("Feature flags should have been checked in main()");
    };

    eprintln!("=== Using OpenCV library version: {} from: {}", opencv.version, pkg_config::get_variable(&pkg_name, "libdir")?);

    // fixme: I wonder whether that kind of forced discovery is needed at all now
    // It sure messes with cross-building when a lib is present in host, but not in target platform
    // So for now let's hide it behind a non-default feature and check the breakage reports
    if cfg!(feature = "force-3rd-party-libs-discovery") {
        // add 3rdparty lib dir. pkgconfig forgets it somehow.
        let third_party_dir1 = format!("{}/share/OpenCV/3rdparty/lib", pkg_config::get_variable(&pkg_name, "prefix")?);
        println!("cargo:rustc-link-search=native={}", third_party_dir1);
        let third_party_dir2 = format!("{}/{}/3rdparty", pkg_config::get_variable(&pkg_name, "libdir")?, pkg_name);
        println!("cargo:rustc-link-search=native={}", third_party_dir2);
        let third_party_dirs: [&str; 2] = [&third_party_dir1, &third_party_dir2];

        // now, this is a nightmare.
        // opencv will embark these as .a when they are not available, or
        // use the one from the system
        // and some may appear in one or more variant (-llibtiff or -ltiff, depending on the system)
        fn lookup_lib(third_party_dirs: &[&str], search: &str) {
            for &prefix in &["lib", "liblib"] {
                for &path in third_party_dirs.iter().chain(&["/usr/lib", "/usr/lib64", "/usr/local/lib", "/usr/local/lib64", "/usr/lib/x86_64-linux-gnu/"]) {
                    for &ext in &[".a", ".dylib", ".so"] {
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
    }

    Ok((opencv, pkg_name.into_owned()))
}

fn build_wrapper(opencv_header_dir: &PathBuf) -> Result<()> {
    println!("cargo:rerun-if-changed=hdr_parser.py");
    println!("cargo:rerun-if-changed=gen_rust.py");
    println!("cargo:rerun-if-env-changed=OPENCV_PKGCONFIG_NAME");
    println!("cargo:rerun-if-env-changed=OPENCV_HEADER_DIR");

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
        writeln!(&mut types, "#define CERES_FOUND true")?; // for sfm module
        if cfg!(feature = "opencv-32") { // for opencl support
            writeln!(&mut types, "#define HAVE_OPENCV_OCL true")?;
        }
        for m in modules {
            writeln!(&mut types, "#include <opencv2/{}.hpp>", m.0)?;
            match m.0.as_str() {
                "dnn" => {
                    // include it manually, otherwise it's not included
                    if cfg!(feature = "opencv-41") {
                        writeln!(&mut types, "#include <opencv2/{}/version.hpp>", m.0)?;
                    }
                    writeln!(&mut types, "#include <opencv2/{}/all_layers.hpp>", m.0)?;
                }
                "aruco" => {
                    writeln!(&mut types, "#include <opencv2/{}/charuco.hpp>", m.0)?;
                }
                "core" => {
                    writeln!(&mut types, "#include <opencv2/{}/ocl.hpp>", m.0)?;
                }
                _ => ()
            }
        }
        if !cfg!(feature = "opencv-32") {
            writeln!(&mut types, "#include <opencv2/core/utils/logger.hpp>")?;
        }
    }

    {
        let mut types = File::create(out_dir.join("types.h"))?;
        writeln!(&mut types, "#include <cstddef>")?;
    }

    let version = if cfg!(feature = "opencv-32") {
        "3.2.0"
    } else if cfg!(feature = "opencv-34") {
        "3.4.7"
    } else if cfg!(feature = "opencv-41") {
        "4.1.1"
    } else {
        unreachable!();
    };
    modules.par_iter().for_each(|(module, files): &(String, Vec<PathBuf>)| {
        let python3 = env::var_os("OPENCV_PYTHON3_BIN")
            .map(PathBuf::from)
            .or_else(|| which("python3").ok())
            .or_else(|| which("python").ok())
            .unwrap_or_else(|| PathBuf::from("python3"));
        // todo ensure that this is actually python3 (e.g. run --version)
        if !Command::new(python3)
            .args(&["-B", "gen_rust.py", "hdr_parser.py", out_dir_as_str, out_dir_as_str, module, &version])
            .args(
                files.iter()
                    .map(|p| opencv_dir.join(p).into_os_string())
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
        for (module, ..) in modules {
            cc.file(out_dir.join(format!("{}.cpp", module)));
            let src = out_dir.join(format!("{}.types.h", module));
            io::copy(&mut File::open(&src)?, &mut types_file)?;
            let _ = fs::remove_file(src);
        }
    }

    {
        let mut hub_return_types = File::create(out_dir.join("return_types.h"))?;
        for entry in glob(&format!("{}/cv_return_value_*.type.h", out_dir_as_str))? {
            let entry = entry?;
            writeln!(
                &mut hub_return_types,
                r#"#include "{}""#,
                entry.file_name().unwrap().to_str().unwrap()
            )?;
        }
    }

    for entry in glob(&format!("{}/*.type.cpp", out_dir_as_str))? {
        cc.file(entry?);
    }

    cc.compile("ocvrs");
    Ok(())
}

fn install_wrapper() -> Result<()> {
    let src_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("src");
    let hub_dir = get_versioned_hub_dir();
    let target_hub_dir = src_dir.join("opencv");
    let target_module_dir = target_hub_dir.join("hub");
    for entry in glob(&format!("{}/*.rs", target_module_dir.to_str().unwrap()))? {
        let _ = fs::remove_file(entry?);
    }
    for entry in glob(&format!("{}/**/*.rs", hub_dir.to_str().unwrap())).unwrap() {
        let entry = entry?;
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

fn gen_wrapper((opencv, pkg_name): (&pkg_config::Library, &str), opencv_header_dir: &PathBuf) -> Result<()> {
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
    modules.par_iter().for_each(|(module, ..)| {
        let consts_cpp = out_dir.join(format!("{}.consts.cpp", module));
        if consts_cpp.is_file() {
            let e = compiler.to_command()
                .current_dir(&out_dir)
                .args(&[
                    format!("{}.consts.cpp", module),
                    "-o".into(),
                    format!("{}.consts", module)
                ])
                .args(&pkg_config_args)
                .status()
                .unwrap();
            assert!(e.success());
            let output = Command::new(format!("./{}.consts", module))
                .current_dir(&out_dir)
                .output()
                .unwrap();
            assert!(output.status.success());
            {
                let mut module_file = OpenOptions::new().append(true).open(out_dir.join(format!("{}.rs", module))).expect("Cannot open module file for append");
                io::copy(&mut output.stdout.as_slice(), &mut module_file).expect("Cannot write constant data to module file");
            }
        }
    });

    if !module_dir.exists() {
        fs::create_dir(&module_dir)?;
    }

    for entry in glob(&format!("{}/*.rs", module_dir.to_str().unwrap()))? {
        let _ = fs::remove_file(entry?);
    }

    let add_manual = |file: &mut File, mod_name: &str| -> Result<bool> {
        if manual_dir.join(format!("{}.rs", mod_name)).exists() {
            writeln!(file, "pub use crate::manual::{}::*;", mod_name)?;
            Ok(true)
        } else {
            Ok(false)
        }
    };

    {
        let mut hub = File::create(hub_dir.join("hub.rs"))?;
        for (module, ..) in modules {
            writeln!(&mut hub, "pub mod {};", module)?;
            let module_filename = format!("{}.rs", module);
            let target_file = module_dir.join(&module_filename);
            fs::rename(out_dir.join(&module_filename), &target_file)?;
            let mut f = OpenOptions::new().append(true).open(&target_file)?;
            add_manual(&mut f, &module)?;
        }
        writeln!(&mut hub, "pub mod types;")?;
        writeln!(&mut hub, "#[doc(hidden)]")?;
        writeln!(&mut hub, "pub mod sys;")?;
    }

    {
        let mut types = File::create(module_dir.join("types.rs"))?;
        writeln!(&mut types, "use std::os::raw::{{c_char, c_void}};")?;
        writeln!(&mut types, "use libc::size_t;")?;
        writeln!(&mut types, "use crate::{{core, types, sys, Result}};")?;
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
        writeln!(&mut sys, "use libc::{{ptrdiff_t, size_t}};")?;
        writeln!(&mut sys, "use crate::core;")?;
        writeln!(&mut sys, "")?;
        for entry in glob(&format!("{}/*.rv.rs", out_dir_as_str))? {
            let entry = entry?;
            io::copy(&mut File::open(&entry)?, &mut sys)?;
        }
        for (module, ..) in modules {
            let path = out_dir.join(format!("{}.externs.rs", module));
            io::copy(&mut File::open(&path)?, &mut sys)?;
        }
        add_manual(&mut sys, "sys")?;
    }

    Ok(())
}

fn cleanup(opencv_dir: &PathBuf) -> Result<()> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let modules = get_modules(&opencv_dir.to_string_lossy())?;
    modules.par_iter().for_each(|(module, ..)| {
        let _ = fs::remove_file(out_dir.join(format!("{}.consts", module)));
        let _ = fs::remove_file(out_dir.join(format!("{}.consts.cpp", module)));
    });
    for entry in glob(&format!("{}/*.rs", out_dir.to_string_lossy()))? {
        let _ = fs::remove_file(entry?);
    }

    Ok(())
}

fn main() -> Result<()> {
    let features = [cfg!(feature = "opencv-32"), cfg!(feature = "opencv-34"), cfg!(feature = "opencv-41")].iter().map(|&x| i32::from(x)).sum::<i32>();
    if features != 1 {
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
            gen_wrapper((&opencv.0, &opencv.1), &opencv_header_dir)?;
        }
    }
    install_wrapper()?;
    cleanup(&opencv_header_dir)?;

    let mut config = cpp_build::Config::new();
    set_compiler_flags(&mut config);
    config.include(opencv_header_dir)
        .build("src/lib.rs");
    Ok(())
}
