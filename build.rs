use std::{
    collections::HashSet,
    error::Error,
    ffi::OsString,
    fs::{self, File, OpenOptions, read_dir},
    io::{self, Write},
    iter::FromIterator,
    path::PathBuf,
    process::Command,
};

use rayon::prelude::*;

use glob::glob;

fn link_wrapper() -> Result<pkg_config::Library, Box<dyn Error>> {
    let opencv = pkg_config::Config::new().probe("opencv")?;

    // add 3rdparty lib dit. pkgconfig forgets it somehow.
    let third_party_dir = format!("{}/share/OpenCV/3rdparty/lib", pkg_config::Config::get_variable("opencv", "prefix")?);
    println!("cargo:rustc-link-search=native={}", third_party_dir);

    for path in opencv.link_paths.iter().filter_map(|path| path.to_str()) {
        println!("cargo:rustc-link-search=native={}", path);
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

    Ok(opencv)
}

fn build_wrapper(opencv: pkg_config::Library) -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=hdr_parser.py");
    println!("cargo:rerun-if-changed=gen_rust.py");

    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);
    let out_dir_as_str = out_dir.to_str().unwrap();
    let hub_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?).join("src");
    let module_dir = hub_dir.join("hub");
    let manual_dir = module_dir.join("manual");

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
        .flag("-std=c++11")
        .flag_if_supported("-Wno-deprecated-declarations")
        .flag_if_supported("-Wno-class-memaccess")
    ;
    for path in &opencv.include_paths {
        gcc.include(path);
    }

    for entry in glob(&format!("{}/*", out_dir_as_str))? {
        let _ = fs::remove_file(entry?);
    }

    let opencv_dir_as_string = opencv_dir.to_string_lossy();
    let ignore_modules: HashSet<&'static str> = HashSet::from_iter([
        "aruco",
        "bgsegm",
        "core_detect",
        "face",
        "flann",
        "hal",
        "hdf", // includes platform-specific headers like /usr/include/x86_64-pc-linux-gnu/opencv2/hdf/hdf5.hpp
        "hfs",
        "ippicv",
        "line_descriptor",
        "opencv",
        "opencv_modules",
        "optflow",
        "rgbd",
        "saliency",
        "stereo",
        "structured_light",
        "surface_matching",
        "text",
        "tracking",
        "viz", // includes platform-specific headers like /usr/include/x86_64-pc-linux-gnu/opencv2/viz.hpp
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
        "core/eigen.hpp",
        "core/fast_math.hpp", // contains functions with Rust native counterparts
        "core/utils/filesystem.hpp", // contains functions with Rust native counterparts
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
        let mut types = File::create(out_dir.join("common_opencv.h"))?;
        for m in &modules {
            write!(&mut types, "#include <opencv2/{}.hpp>\n", m.0)?;
            if m.0 == "dnn" {
                // include it manually, otherwise it's not included
                write!(&mut types, "#include <opencv2/{}/all_layers.hpp>\n", m.0)?;
            }
        }
    }

    {
        let mut types = File::create(out_dir.join("types.h"))?;
        write!(&mut types, "#include <cstddef>\n")?;
    }

    modules.par_iter_mut().for_each(|module| {
        if !Command::new("python3")
            .args(&["-B", "gen_rust.py", "hdr_parser.py", out_dir_as_str, out_dir_as_str, &module.0])
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

        if cfg!(feature = "buildtime_bindgen") {
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
        }
        let _ = fs::remove_file(out_dir.join(format!("{}.consts.cpp", module.0)));
    });

    {
        let mut types_file = File::create(out_dir.join("types.h"))?;
        for module in &modules {
            gcc.file(out_dir.join(format!("{}.cpp", module.0)));
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

    for entry in glob("native/*.cpp")? {
        gcc.file(entry?);
    }
    for entry in glob(&format!("{}/*.type.cpp", out_dir_as_str))? {
        gcc.file(entry?);
    }

    gcc.include(".")
        .include(&out_dir);

    gcc.compile("ocvrs");

    if cfg!(feature = "buildtime_bindgen") {
        if !module_dir.exists() {
            fs::create_dir(&module_dir)?;
        }

        for entry in glob(&format!("{}/*.rs", module_dir.to_str().unwrap()))? {
            let _ = fs::remove_file(entry?);
        }

        let add_manual = |file: &mut File, mod_name: &str| -> Result<bool, Box<dyn Error>> {
            if manual_dir.join(format!("{}.rs", mod_name)).exists() {
                writeln!(file, "pub use crate::hub::manual::{}::*;", mod_name)?;
                let mut m = OpenOptions::new().create(true).append(true).open(&module_dir.join("manual.rs"))?;
                writeln!(&mut m, "pub mod {};", mod_name)?;
                Ok(true)
            } else {
                Ok(false)
            }
        };

        {
            let mut hub = File::create(hub_dir.join("hub.rs"))?;
            let mut manual_writen = false;
            for ref module in &modules {
                writeln!(&mut hub, r#"pub mod {};"#, module.0)?;
                let module_filename = format!("{}.rs", module.0);
                let target_file = module_dir.join(&module_filename);
                fs::rename(out_dir.join(&module_filename), &target_file)?;
                let mut f = OpenOptions::new().append(true).open(&target_file)?;
                if let Ok(true) = add_manual(&mut f, &module.0) {
                    if !manual_writen {
                        writeln!(&mut hub, "mod manual;")?;
                        manual_writen = true;
                    }
                }
            }
            writeln!(&mut hub, "pub mod types;")?;
            writeln!(&mut hub, "#[doc(hidden)] pub mod sys;")?;
        }

        {
            let mut types = File::create(module_dir.join("types.rs"))?;
            writeln!(&mut types, "use libc::{{c_void, c_char, size_t}};")?;
            writeln!(&mut types, "use crate::{{core, types}};")?;
            writeln!(&mut types, "cpp!{{{{")?;
            writeln!(&mut types, "    #include \"../common_opencv.h\"")?;
            writeln!(&mut types, "    using namespace cv;")?;
            writeln!(&mut types, "    #include \"common.h\"")?;
            writeln!(&mut types, "    #include \"../types.h\"")?;
            writeln!(&mut types, "    #include \"../return_types.h\"")?;
            writeln!(&mut types, "}}}}")?;
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
            writeln!(&mut sys, "use crate::{{core, Error, Result}};")?;
            writeln!(&mut sys, "")?;
            for entry in glob(&format!("{}/*.rv.rs", out_dir_as_str))? {
                let entry = entry?;
                io::copy(&mut File::open(&entry)?, &mut sys)?;
            }
            for module in &modules {
                let path = out_dir.join(format!("{}.externs.rs", module.0));
                io::copy(&mut File::open(&path)?, &mut sys)?;
            }
            add_manual(&mut sys, "sys")?;
        }
    }
    for entry in glob(&format!("{}/*.rs", out_dir_as_str))? {
        let _ = fs::remove_file(entry?);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    if !cfg!(feature = "docs-only") {
        build_wrapper(link_wrapper()?)?;
        let mut config = cpp_build::Config::new();
        config.flag_if_supported("-Wno-class-memaccess");
        config.build("src/lib.rs");
    }
    Ok(())
}
