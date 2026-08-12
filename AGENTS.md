# AGENTS.md

Guidance for AI agents working on `opencv-rust` — Rust bindings for OpenCV.

## What this project is

The crate does **not** contain hand-written bindings. `libclang` parses the OpenCV C++ headers, a code generator emits a C shim
(`*.cpp`) plus Rust wrappers, and those are compiled into the `opencv` crate at build time. Almost every user-visible API change
is made by changing the **generator**, not by editing generated Rust.

## Repository layout

| Path                                              | Purpose                                                                                                                                                                                                    |
|---------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `binding-generator/`                              | Workspace member `opencv-binding-generator`: clang parsing, IR (`class`, `func`, `type_ref`, …) and code writers (`writer/rust_native/`, with `.tpl.rs`/`.tpl.cpp` templates in `writer/rust_native/tpl/`) |
| `binding-generator/src/settings/`                 | Per-module manual tweaks: renames, excludes, argument/return overrides, injected funcs, unsafety, cfg attrs.                                                                                               |
| `build.rs` + `build/`                             | Build script: OpenCV discovery (`library.rs`, `cmake_probe.rs`), running the generator (`generator.rs`), emitting `cfg` flags, compiling the C++ shim                                                      |
| `src/`                                            | Thin hand-written crate surface: `lib.rs`, `error.rs`, `boxed_ref.rs`, `traits/`, `templ.rs` (macros)                                                                                                      |
| `src/manual/`                                     | Hand-written replacements/extensions for generated code (`Mat`, `Vector`, `Ptr`, `Matx`, `Point`, …). Files here extend the generated equivalents                                                          |
| `src_cpp/`                                        | Hand-written C++ headers/impl fed to the generator (`ocvrs_common.hpp`, per-module `*.hpp`)                                                                                                                |
| `docs/`                                           | Pre-generated bindings committed for docs.rs builds (regenerated on release)                                                                                                                               |
| `tests/`, `examples/`, `binding-generator/tests/` | Integration tests and examples                                                                                                                                                                             |
| `ci/`, `tools/`                                   | Install/test scripts for CI, and maintainer scripts for local regeneration                                                                                                                                 |
| `out/`                                            | Scratch output of `tools/generate-bindings.sh`, useful to get the overview of how the codebase changes affect the generation (do a diff of before and after)                                               |

## Build & test

Requires a local OpenCV (4.x or 5.x) and `libclang`.

```bash
cargo test -vv --features rgb,f16    # full crate tests, needs OpenCV, -vv shows generator output; essential when debugging generation
tools/generate-bindings.sh           # useful during the development to see how the code changes affect the binding generation (requires `tools/config.sh` set up)
ci/script.sh                         # what CI runs (env-driven; see the OPENCV_* vars below)
```

Discovery is controlled by env vars, all documented in `README.md`: `OPENCV_LINK_LIBS`,
`OPENCV_LINK_PATHS`, `OPENCV_INCLUDE_PATHS`, `OPENCV_DISABLE_PROBES`, `OpenCV_DIR`, `VCPKG_ROOT`, …
`tools/env-4.sh` / `tools/env-5.sh` set them from an untracked `tools/config.sh` (copy from
`tools/config.tpl.sh`).

Generator-internal env vars: `OCVRS_DOCS_GENERATE_DIR` (write bindings into `docs/`),
`OCVRS_PARSING_HEADERS`, `OCVRS_OVERRIDE`, `OCVRS_FFI_EXPORT_SUFFIX`.

Touch `build.rs` to force regeneration.

## Making changes

* **Fixing/renaming/hiding a generated API** → edit the matching file under
  `binding-generator/src/settings/` (`func_rename.rs`, `func_exclude.rs`, `argument_override.rs`,
  `func_unsafe.rs`, `class_tweaks.rs`, `property_tweaks.rs`, `element_exclude_kind.rs`, …). Functions are keyed by
  `Func::identifier()` (e.g. `cv_bioinspired_Retina_getMagnoRAW_const__OutputArrayR`); in a rename value `+` expands to the old
  name.
* **Changing emitted code shape** → `binding-generator/src/writer/rust_native/` and its `tpl/` templates.
* **Adding a new OpenCV module** → add a variant to `SupportedModule` in
  `binding-generator/src/supported_module.rs`, add it to `SUPPORTED_MODULES` in `build/enums.rs` (there is an explicit comment
  reminding you), and add the Cargo feature (with its module deps) in `Cargo.toml`.
* **Hand-written Rust API** → `src/manual/`; add the trait/type to the relevant `prelude` if users need it.
* **Extra C++ helpers** → `src_cpp/<module>.hpp`.
* Verify a generation change by inspecting the emitted code: `cargo build -vv`, or run
  `tools/generate-bindings.sh` which writes to `out/4/` and `out/5/`.
* Search for `todo` / `fixme` comments — they mark known work items.

## Conventions

* For code formatting run `cargo +nightly fmt`.
* Conditional compilation uses generated `cfg` flags — `ocvrs_has_module_<name>`,
  `ocvrs_has_inherent_feature_<name>`, `ocvrs_opencv_branch_4` / `_5` — plus the exported
  `opencv_branch_4!`, `opencv_has_module_<name>!` and `opencv_has_inherent_feature_<name>!` macros (templates in
  `build/cond_macros/`). Gate any version- or module-specific code with them.
* Fallible OpenCV calls return `Result` (C++ exceptions are translated); property getters/setters and
  `CV_NOEXCEPT` functions return values directly.
* Scope: the crate wraps the OpenCV API and adds only light ergonomics — avoid inventing new functionality.
* Tests live in `tests/*.rs`, use `opencv::prelude::*` and return `Result<()>`. Files named
  `*_only_latest_opencv.rs` are removed by CI for older OpenCV versions — put version-sensitive tests there.
* User-visible changes get an entry at the top of `CHANGES.md`.
* `docs/` is regenerated by maintainers (`tools/regen-docs.sh`, run automatically on release); when a generator change alters
  output, the regenerated docs will be committed automatically during the release process.
