use std::{
    borrow::Cow,
    collections::HashSet,
    env,
    ffi::OsString,
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    iter::FromIterator,
    path::{self, PathBuf},
    process::Command,
};

use cfg_if::cfg_if;
use glob_crate::glob;
use once_cell::sync::OnceCell;
use rayon::prelude::*;
use regex::Regex;
use semver::{Version, VersionReq};
use which_crate::which;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

static CORE_MODULES: OnceCell<HashSet<&'static str>> = OnceCell::new();
static MODULES: OnceCell<Vec<(String, Vec<PathBuf>)>> = OnceCell::new();

#[derive(Debug)]
struct Library {
    pub pkg_name: String,
    pub libs: Vec<String>,
    pub link_paths: Vec<PathBuf>,
    pub framework_paths: Vec<PathBuf>,
    pub include_paths: Vec<PathBuf>,
    pub header_path: Option<PathBuf>,
    pub version: String,
    pub prefix: PathBuf,
    pub libdir: PathBuf,
}

impl Library {
    fn probe_from_paths(pkg_name: &str, link_libs: &str, link_paths: &str, include_paths: &str, header_path: Option<&str>) -> Result<Self> {
        let libs: Vec<_> = link_libs.split(',')
            .map(|x| {
                let mut path = PathBuf::from(x.trim());
                if cfg!(target_env = "msvc") {
                    path.set_extension("lib");
                } else if path
                    .extension()
                    .map(|e| e == "lib" || e == "so" || e == "dylib")
                    .unwrap_or(false)
                {
                    path.set_extension("");
                }
                let out = path.file_name().and_then(|f| f.to_str()).expect("Invalid library name").to_owned();
                if cfg!(target_env = "msvc") {
                    println!("cargo:rustc-link-lib=static={}", out);
                } else {
                    println!("cargo:rustc-link-lib={}", out);
                }
                out
            })
            .collect();

        let link_paths: Vec<_> = link_paths
            .split(',')
            .map(|x| {
                let out = PathBuf::from(x.trim());
                println!("cargo:rustc-link-search=native={}", out.to_str().expect("Invalid link path"));
                out
            })
            .collect();

        let libdir = link_paths
            .first()
            .map(|x| x.clone())
            .unwrap_or_else(|| PathBuf::from(""));

        let include_paths: Vec<_> = include_paths
            .split(',')
            .map(|x| PathBuf::from(x.trim()))
            .collect();

        let version = include_paths.iter()
            .filter_map(get_version_from_headers)
            .next();

        let out = Self {
            pkg_name: pkg_name.to_owned(),
            libs,
            link_paths,
            framework_paths: vec![],
            include_paths,
            header_path: header_path.map(|p| PathBuf::from(p.trim())),
            version: version.unwrap_or("0.0.0".to_owned()),
            prefix: PathBuf::from(""),
            libdir
        };
        Ok(out)
    }

    #[cfg(not(target_env = "msvc"))]
    fn probe_pkg_config(pkg_name: &str) -> Result<Self> {
        let opencv = pkg_config::probe_library(pkg_name)?;
        let header_path = opencv.include_paths.first().map(|x| x.clone());

        Ok(Self {
            pkg_name: pkg_name.to_owned(),
            libs: opencv.libs,
            link_paths: opencv.link_paths,
            framework_paths: opencv.framework_paths,
            include_paths: opencv.include_paths,
            header_path,
            version: opencv.version,
            prefix: PathBuf::from(pkg_config::get_variable(pkg_name, "prefix")?),
            libdir: PathBuf::from(pkg_config::get_variable(pkg_name, "libdir")?),
        })
    }

    #[cfg(target_env = "msvc")]
    fn probe_vcpkg(pkg_name: &str) -> Result<Self> {
        let opencv = vcpkg::Config::new().find_package(pkg_name)?;
        let libs: Vec<String> = opencv
            .found_libs
            .iter()
            .map(|lib| lib.file_name().unwrap().to_string_lossy().to_string())
            .collect();
        let version = opencv
            .include_paths
            .iter()
            .filter_map(get_version_from_headers)
            .next();
        let libdir = opencv
            .link_paths
            .first()
            .map(|x| x.clone())
            .unwrap_or_else(|| PathBuf::from(""));
        let header_path = opencv
            .include_paths
            .first()
            .map(|x| x.clone())
            .unwrap_or_else(|| PathBuf::from(""));

        Ok(Self {
            pkg_name: pkg_name.to_owned(),
            libs,
            link_paths: opencv.link_paths,
            framework_paths: vec![],
            include_paths: opencv.include_paths,
            header_path: Some(header_path),
            version: version.unwrap_or("0.0.0".to_owned()),
            prefix: PathBuf::from(""),
            libdir
        })
    }

    pub fn probe(pkg_name: &str) -> Result<Self> {
        if let (Ok(link_libs), Ok(link_paths), Ok(include_paths), header_path) = (
            env::var("OPENCV_LINK_LIBS"),
            env::var("OPENCV_LINK_PATHS"),
            env::var("OPENCV_INCLUDE_PATHS"),
            env::var("OPENCV_HEADER_PATH"),
        ) {
            Self::probe_from_paths(
                pkg_name,
                &link_libs,
                &link_paths,
                &include_paths,
                header_path.ok().as_ref().map(String::as_ref),
            )
        } else {
            cfg_if! {
                if #[cfg(target_env = "msvc")] {
                    Self::probe_vcpkg(pkg_name)
                } else {
                    Self::probe_pkg_config(pkg_name)
                }
            }
        }
    }

    pub fn update_compiler(&self, cc: &mut cc::Build) -> Result<Vec<OsString>> {
        for p in &self.include_paths {
            cc.include(p);
        }
        let mut extra_args: Vec<OsString> = Vec::with_capacity(60);
        let third_party_dirs = if cfg!(feature = "force-3rd-party-libs-discovery") {
            // add 3rdparty lib dir. pkgconfig forgets it somehow.
            self.get_3rdparty_lib_dirs()
        } else {
            vec![]
        };
        let link_paths = self.link_paths.iter().chain(third_party_dirs.iter());
        if cfg!(target_env = "msvc") {
            for l in &self.libs {
                 extra_args.push(l.into());
            }
            extra_args.push("-link".into());
            for p in link_paths {
                extra_args.push(format!("-LIBPATH:{}", p.to_string_lossy()).into());
            }
        } else {
            for p in link_paths {
                extra_args.push("-L".into());
                extra_args.push(p.into());
            }
            for f in &self.framework_paths {
                extra_args.push("-F".into());
                extra_args.push(f.into());
            }
            for l in &self.libs {
                extra_args.push("-l".into());
                extra_args.push(l.into());
            }
        }
        Ok(extra_args)
    }

    pub fn get_3rdparty_lib_dirs(&self) -> Vec<PathBuf> {
        vec![self.prefix.join("share/OpenCV/3rdparty/lib"), self.libdir.join(&format!("{}/3rdparty", self.pkg_name))]
    }
}

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
        .flag_if_supported("-Wno-deprecated-copy")
        .flag_if_supported("-Wno-ignored-qualifiers");
    if cfg!(target_env = "msvc") {
        cc.flag_if_supported("-wd4996")
            .flag_if_supported("-EHsc");
    }
}

fn get_version_from_headers(header_dir: &PathBuf) -> Option<String> {
    let version_hpp = header_dir.join("opencv2/core/version.hpp");
    if !version_hpp.is_file() {
        return None;
    }
    let version_header = fs::read_to_string(version_hpp).ok()?;
    let get_version_part = |def| {
        Regex::new(&format!(r"#define\s+CV_VERSION_{}\s+(\d+)", def)).ok()
            .map(|r| {
                r.captures(&version_header)
                    .and_then(|c| c.get(1))
                    .map(|m| m.as_str())
                    .unwrap_or("0")
            })
    };
    let major = get_version_part("MAJOR")?;
    let minor = get_version_part("MINOR")?;
    let revision = get_version_part("REVISION")?;
    Some(format!("{}.{}.{}", major, minor, revision))
}

fn check_matching_version(version: &str) -> Result<()> {
    if cfg!(feature = "opencv-32") && !VersionReq::parse("~3.2")?.matches(&Version::parse(version)?) {
        Err(format!("OpenCV version: {} must be from 3.2 branch because of the feature: opencv-32", version).into())
    } else if cfg!(feature = "opencv-34") && !VersionReq::parse("~3.4")?.matches(&Version::parse(version)?) {
        Err(format!("OpenCV version: {} must be from 3.4 branch because of the feature: opencv-34", version).into())
    } else if cfg!(feature = "opencv-41") && !VersionReq::parse("~4.1")?.matches(&Version::parse(version)?) {
        Err(format!("OpenCV version: {} must be from 4.1 branch because of the feature: opencv-41", version).into())
    } else {
        Ok(())
    }
}

fn get_versioned_hub_dir() -> PathBuf {
    let mut hub_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("Can't read CARGO_MANIFEST_DIR env var"));
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

fn is_core_module(module: &str) -> bool {
    let core_modules = match CORE_MODULES.get() {
        None => {
            CORE_MODULES.set(HashSet::from_iter([
                "calib3d",
                "core",
                #[cfg(not(feature = "opencv-32"))]
                "dnn",
                #[cfg(feature = "opencv-41")]
                "dnn_superres",
                "features2d",
                "flann",
                #[cfg(feature = "opencv-41")]
                "gapi",
                "highgui",
                "imgcodecs",
                "imgproc",
                "ml",
                "objdetect",
                "photo",
                #[cfg(any(feature = "opencv-32", feature = "opencv-34"))]
                "shape",
                "stitching",
                #[cfg(any(feature = "opencv-32", feature = "opencv-34"))]
                "superres",
                "video",
                "videoio",
                #[cfg(any(feature = "opencv-32", feature = "opencv-34"))]
                "videostab",
                "viz",
            ].iter().map(|x| *x))).expect("Cannot set CORE_MODULES cache");
            CORE_MODULES.get().unwrap()
        },
        Some(modules) => modules,
    };
    core_modules.contains(module)
}

fn get_modules(opencv_dir_as_string: &str) -> Result<&'static Vec<(String, Vec<PathBuf>)>> {
    if let Some(modules) = MODULES.get() {
        return Ok(modules);
    }
    let ignore_modules: HashSet<&'static str> = HashSet::from_iter([
        "core_detect",
        "cudalegacy",
        "cudev",
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
        PathBuf::from("viz/types.hpp"),
        PathBuf::from("viz/widgets.hpp"),
    ];

    modules.sort_by_key(|(mod_name, ..)| module_order.iter().position(|&order_module| order_module == mod_name).unwrap_or_else(|| module_order.len()));
    for (.., file_list) in &mut modules {
        file_list.sort_by_key(|header| header_file_order.iter().position(|order_header| header.ends_with(order_header)).unwrap_or_else(|| header_file_order.len()));
    }

    MODULES.set(modules).expect("Cannot set MODULES cache");
    Ok(MODULES.get().unwrap())
}

fn copy_indent(mut read: impl BufRead, mut write: impl Write, level: usize, indent: &str) -> Result<()> {
    let full_indent = indent.repeat(level);
    let mut line = Vec::with_capacity(100);
    while read.read_until(b'\n', &mut line)? != 0 {
        write.write(full_indent.as_bytes())?;
        write.write(&line)?;
        line.clear();
    }
    Ok(())
}

fn build_compiler(opencv_header_dir: &PathBuf) -> cc::Build {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("Can't read OUT_DIR env var"));
    let mut out = cc::Build::new();
    set_compiler_flags(&mut out);
    out.cpp(true)
        .include(opencv_header_dir)
        .include(&out_dir)
        .include(".");
    if !cfg!(target_env = "msvc") {
        out.flag("-std=c++11");
    }
    out
}

fn link_wrapper() -> Result<Library> {
    let pkg_name = if cfg!(feature = "opencv-32") || cfg!(feature = "opencv-34") {
        env::var("OPENCV_PKGCONFIG_NAME").map(|x| Cow::Owned(x)).unwrap_or_else(|_| Cow::Borrowed("opencv"))
    } else if cfg!(feature = "opencv-41") {
        env::var("OPENCV_PKGCONFIG_NAME").map(|x| Cow::Owned(x)).unwrap_or_else(|_| Cow::Borrowed("opencv4"))
    } else {
        unreachable!("Feature flags should have been checked in main()");
    };
    let opencv = Library::probe(&pkg_name).expect(&format!("Package {} is not found", pkg_name));
    check_matching_version(&opencv.version).map_err(|e| format!("{}, (version coming from pkg_config for package: {})", e, opencv.pkg_name))?;

    eprintln!("=== Using OpenCV library version: {} from: {}", opencv.version, opencv.libdir.display());

    // fixme: I wonder whether that kind of forced discovery is needed at all now
    // It sure messes with cross-building when a lib is present in host, but not in target platform
    // So for now let's hide it behind a non-default feature and check the breakage reports
    if cfg!(feature = "force-3rd-party-libs-discovery") {
        let mut third_party_dirs = opencv.get_3rdparty_lib_dirs();
        // add 3rdparty lib dir. pkgconfig forgets it somehow.
        third_party_dirs.iter().for_each(|p| println!("cargo:rustc-link-search=native={}", p.to_string_lossy()));
        third_party_dirs.extend(vec![
            PathBuf::from("/usr/lib"),
            PathBuf::from("/usr/lib64"),
            PathBuf::from("/usr/local/lib"),
            PathBuf::from("/usr/local/lib64"),
            PathBuf::from("/usr/lib/x86_64-linux-gnu/"),
        ]);

        // now, this is a nightmare.
        // opencv will embark these as .a when they are not available, or
        // use the one from the system
        // and some may appear in one or more variant (-llibtiff or -ltiff, depending on the system)
        fn lookup_lib(third_party_dirs: &[PathBuf], search: &str) {
            for &prefix in &["lib", "liblib"] {
                for path in third_party_dirs.iter() {
                    for &ext in &[".a", ".dylib", ".so"] {
                        let name = format!("{}{}", prefix, search);
                        let filename = path.join(format!("{}{}", name, ext));
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

    Ok(opencv)
}

fn build_wrapper(opencv_header_dir: &PathBuf) -> Result<()> {
    println!("cargo:rerun-if-changed=hdr_parser.py");
    println!("cargo:rerun-if-changed=gen_rust.py");
    println!("cargo:rerun-if-env-changed=OPENCV_HEADER_DIR");
    println!("cargo:rerun-if-env-changed=OPENCV_PKGCONFIG_NAME");
    println!("cargo:rerun-if-env-changed=OPENCV_LINK_LIBS");
    println!("cargo:rerun-if-env-changed=OPENCV_LINK_PATHS");
    println!("cargo:rerun-if-env-changed=OPENCV_INCLUDE_PATHS");

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").ok_or_else(|| "Can't read OUT_DIR env var")?);
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
                "core" => {
                    writeln!(&mut types, "#include <opencv2/{}/ocl.hpp>", m.0)?;
                }
                "aruco" => {
                    writeln!(&mut types, "#include <opencv2/{}/charuco.hpp>", m.0)?;
                }
                "dnn" => {
                    // include it manually, otherwise it's not included
                    if cfg!(feature = "opencv-41") {
                        writeln!(&mut types, "#include <opencv2/{}/version.hpp>", m.0)?;
                    }
                    writeln!(&mut types, "#include <opencv2/{}/all_layers.hpp>", m.0)?;
                }
                "face" => {
                    writeln!(&mut types, "#include <opencv2/{}/bif.hpp>", m.0)?;
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
        "3.4.8"
    } else if cfg!(feature = "opencv-41") {
        "4.1.2"
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
            .env("LC_CTYPE", "C.UTF-8") // makes python3 locale.getpreferredencoding() return utf8 encoding instead of ansi
            .args(&["-B", "gen_rust.py", "hdr_parser.py", out_dir_as_str, out_dir_as_str, module, &version])
            .args(files.iter().map(|p| opencv_dir.join(p).into_os_string()))
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
    let src_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").ok_or_else(|| "Can't read CARGO_MANIFEST_DIR env var")?).join("src");
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

fn gen_wrapper(opencv: &Library, opencv_header_dir: &PathBuf) -> Result<()> {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").ok_or_else(|| "Can't read OUT_DIR env var")?);
    let out_dir_as_str = out_dir.to_str().unwrap();
    let src_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").ok_or_else(|| "Can't read CARGO_MANIFEST_DIR env var")?).join("src");
    let hub_dir = get_versioned_hub_dir();
    let module_dir = hub_dir.join("hub");
    let manual_dir = src_dir.join("manual");
    let opencv_dir = opencv_header_dir.join("opencv2");

    eprintln!("=== Placing generated bindings into: {}", hub_dir.display());

    let mut compiler = build_compiler(opencv_header_dir);
    let extra_args = opencv.update_compiler(&mut compiler)?;
    let modules = get_modules(&opencv_dir.to_string_lossy())?;
    modules.par_iter().for_each(|(module, ..)| {
        let consts_cpp = out_dir.join(format!("{}.consts.cpp", module));
        if consts_cpp.is_file() {
            let consts_bin = out_dir.join(format!("{}.consts{}", module, if cfg!(target_env = "msvc") { ".exe" } else { "" }));
            let mut command = compiler.get_compiler().to_command();
            command
                .current_dir(&out_dir)
                .arg(consts_cpp)
                .args(&extra_args)
                .arg(format!("{}{}", if cfg!(target_env = "msvc") { "-OUT:" } else { "-o" }, consts_bin.to_string_lossy()));
            println!("running: {:?}", &command);
            let e = command
                .status()
                .unwrap();
            assert!(e.success());
            let output = Command::new(consts_bin)
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

        let mut types = File::create(module_dir.join("types.rs"))?;
        writeln!(&mut types, "use std::os::raw::{{c_char, c_void}};")?;
        writeln!(&mut types, "use libc::size_t;")?;
        writeln!(&mut types, "use crate::{{core, types, sys, Result}};")?;
        writeln!(&mut types, "")?;

        let mut sys = File::create(module_dir.join("sys.rs"))?;
        writeln!(&mut sys, "use std::os::raw::{{c_char, c_void}};")?;
        writeln!(&mut sys, "use libc::{{ptrdiff_t, size_t}};")?;
        writeln!(&mut sys, "use crate::core;")?;
        writeln!(&mut sys, "")?;
        for (module, ..) in modules {
            let is_contrib_module = !is_core_module(module);
            let write_if_contrib = |write: &mut File| -> Result<()> {
                if is_contrib_module {
                    writeln!(write, r#"#[cfg(feature = "contrib")]"#)?;
                }
                Ok(())
            };
            // hub
            write_if_contrib(&mut hub)?;
            writeln!(&mut hub, "pub mod {};", module)?;
            let module_filename = format!("{}.rs", module);
            let target_file = module_dir.join(&module_filename);
            let src_file = out_dir.join(&module_filename);
            if fs::rename(&src_file, &target_file).is_err() {
                fs::copy(&src_file, &target_file)?;
                fs::remove_file(src_file)?;
            }
            let mut f = OpenOptions::new().append(true).open(&target_file)?;
            add_manual(&mut f, module)?;

            // types
            let mut write_header = true;
            for entry in glob(&format!("{}/{}-*.type.rs", out_dir_as_str, module))? {
                let entry = entry?;
                if write_header {
                    write_if_contrib(&mut types)?;
                    writeln!(&mut types, "mod {}_types {{", module)?;
                    writeln!(&mut types, "    use super::*;")?;
                    writeln!(&mut types, "")?;
                    write_header = false;
                }
                copy_indent(BufReader::new(File::open(&entry)?), &mut types, 1, "    ")?;
            }
            if !write_header {
                writeln!(&mut types, "}}")?;
                write_if_contrib(&mut types)?;
                writeln!(&mut types, "pub use {}_types::*;", module)?;
                writeln!(&mut types, "")?;
            }

            // sys
            let path = out_dir.join(format!("{}.externs.rs", module));
            write_if_contrib(&mut sys)?;
            writeln!(&mut sys, "mod {}_sys {{", module)?;
            writeln!(&mut sys, "    use super::*;")?;
            writeln!(&mut sys, "")?;
            for entry in glob(&format!("{}/{}-*.rv.rs", out_dir_as_str, module))? {
                let entry: PathBuf = entry?;
                copy_indent(BufReader::new(File::open(entry)?), &mut sys, 1, "    ")?;
            }
            copy_indent(BufReader::new(File::open(&path)?), &mut sys, 1, "    ")?;
            writeln!(&mut sys, "}}")?;
            write_if_contrib(&mut sys)?;
            writeln!(&mut sys, "pub use {}_sys::*;", module)?;
            writeln!(&mut sys, "")?;
        }
        writeln!(&mut hub, "pub mod types;")?;
        writeln!(&mut hub, "#[doc(hidden)]")?;
        writeln!(&mut hub, "pub mod sys;")?;

        add_manual(&mut types, "types")?;

        add_manual(&mut sys, "sys")?;
    }

    Ok(())
}

fn cleanup(opencv_dir: &PathBuf) -> Result<()> {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").ok_or_else(|| "Can't read OUT_DIR env var")?);
    let modules = get_modules(&opencv_dir.to_string_lossy())?;
    modules.par_iter().for_each(|(module, ..)| {
        if cfg!(target_env = "msvc") {
            let _ = fs::remove_file(out_dir.join(format!("{}.consts.exe", module)));
        } else {
            let _ = fs::remove_file(out_dir.join(format!("{}.consts", module)));
        }
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

    // no idea if this breaks docs_only
    let opencv = link_wrapper()?;

    let opencv_header_dir = env::var_os("OPENCV_HEADER_DIR").map(PathBuf::from).unwrap_or_else(|| {
        let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("Can't read CARGO_MANIFEST_DIR env var"));
        if let Some(header_path) = &opencv.header_path {
            header_path.clone()
        } else if cfg!(feature = "opencv-32") {
            manifest_dir.join("headers/3.2")
        } else if cfg!(feature = "opencv-34") {
            manifest_dir.join("headers/3.4")
        } else if cfg!(feature = "opencv-41") {
            manifest_dir.join("headers/4.1")
        } else {
            panic!("Please select one OpenCV major version using one of the opencv-* features or specify OpenCV header path manually via OPENCV_HEADER_DIR environment var");
        }
    });
    if let Some(version) = get_version_from_headers(&opencv_header_dir) {
        check_matching_version(&version).map_err(|e| format!("{}, (version coming from headers at: {})", e, opencv_header_dir.display()))?;
    } else {
        panic!("Unable to find header version in: {}", opencv_header_dir.display())
    }

    build_wrapper(&opencv_header_dir)?;
    if !cfg!(feature = "docs-only") {
        if cfg!(feature = "buildtime-bindgen") {
            gen_wrapper(&opencv, &opencv_header_dir)?;
        }
        install_wrapper()?;
    }
    cleanup(&opencv_header_dir)?;

    let mut config = cpp_build::Config::new();
    set_compiler_flags(&mut config);
    config.include(opencv_header_dir)
        .build("src/lib.rs");
    Ok(())
}
