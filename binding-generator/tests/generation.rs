use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use clang::diagnostic::Severity;
use clang::{Clang, Entity, Index};
use opencv_binding_generator::writer::rust_native::element::RustNativeGeneratedElement;
use opencv_binding_generator::{EntityWalkerExt, Func, GeneratorEnv, GeneratorVisitor, OpenCvWalker, SupportedModule};
use tempfile::TempDir;

fn clang_parse(code: &str, op: impl FnOnce(Entity)) {
	const CODE_TPL: &str = include_str!("code_template.cpp");
	const CODE_PAT: &str = "{{code}}";

	let temp_dir = TempDir::new().expect("Can't create temp dir");
	let temp_file_path = temp_dir.path().join("temp.cpp");
	if let Some(start) = CODE_TPL.find(CODE_PAT) {
		let mut temp_cpp = BufWriter::new(File::create(&temp_file_path).expect("Can't create temp file"));
		temp_cpp
			.write_all(&CODE_TPL.as_bytes()[..start])
			.expect("Can't write to temp file");
		temp_cpp.write_all(code.as_bytes()).expect("Can't write to temp file");
		temp_cpp
			.write_all(&CODE_TPL.as_bytes()[start + CODE_PAT.len()..])
			.expect("Can't write to temp file");
	}
	let clang = Clang::new().expect("Can't init clang");
	let index = Index::new(&clang, false, false);
	let root_tu = index
		.parser(&temp_file_path)
		.skip_function_bodies(true)
		.detailed_preprocessing_record(true)
		.parse()
		.expect("Can't parse");
	let diags = root_tu.get_diagnostics();
	if !diags.is_empty() {
		let mut has_error = false;
		eprintln!("WARNING: {} diagnostic messages", diags.len());
		for diag in diags {
			if !has_error && matches!(diag.get_severity(), Severity::Error | Severity::Fatal) {
				has_error = true;
			}
			eprintln!("   {diag}");
		}
		if has_error {
			panic!("Errors during header parsing");
		}
	}
	op(root_tu.get_entity());
}

fn extract_functions(code: &str, cb: impl FnMut(Func)) {
	struct FunctionExtractor<F> {
		cb: F,
	}

	impl<F: FnMut(Func)> GeneratorVisitor<'_> for FunctionExtractor<F> {
		fn visit_func(&mut self, func: Func) {
			(self.cb)(func);
		}
	}

	clang_parse(code, |root_tu| {
		let gen_env = GeneratorEnv::empty();
		let visitor = FunctionExtractor { cb };
		let opencv_walker = OpenCvWalker::new(SupportedModule::Core, Path::new(""), visitor, gen_env);

		root_tu.walk_opencv_entities(opencv_walker);
	});
}

#[test]
fn char_ptr_slice() {
	extract_functions("CV_EXPORTS int startLoop(int argc, char* argv[]);", |f| {
		assert_eq!(f.gen_rust("0.0.0").trim(), "#[inline]\npub fn start_loop(argv: &mut [&str]) -> Result<i32> {\n\tstring_array_arg_mut!(argv);\n\treturn_send!(via ocvrs_return);\n\tunsafe { sys::cv_startLoop_int_charXX(argv.len().try_into()?, argv.as_mut_ptr(), ocvrs_return.as_mut_ptr()) };\n\treturn_receive!(ocvrs_return => ret);\n\tlet ret = ret.into_result()?;\n\tOk(ret)\n}");
		assert_eq!(f.gen_cpp().trim(), "void cv_startLoop_int_charXX(int argc, char** argv, Result<int>* ocvrs_return) {\n\ttry {\n\t\tint ret = cv::startLoop(argc, argv);\n\t\tOk(ret, ocvrs_return);\n\t} OCVRS_CATCH(ocvrs_return);\n}");
		assert_eq!(
			f.gen_rust_externs().trim(),
			r#"pub fn cv_startLoop_int_charXX(argc: i32, argv: *mut *mut c_char, ocvrs_return: *mut Result<i32>);"#,
		);
	});
}
