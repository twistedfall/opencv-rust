use std::{
	collections::HashSet,
	env,
	path::{Path, PathBuf},
};

use clang::{Clang, Entity, EntityKind, Type};

use opencv_binding_generator::{
	Class,
	Element,
	EntityExt,
	EntityWalker,
	EntityWalkerVisitor,
	Func,
	Generator,
	GeneratorEnv,
	main_module_from_path,
	module_from_path,
	settings,
};

struct FunctionFinder<'tu, 'f> {
	pub gen_env: GeneratorEnv<'tu>,
	pub func_rename_unused: &'f mut HashSet<&'static str>,
	pub func_cfg_attr_unused: &'f mut HashSet<&'static str>,
	pub func_unsafe_unused: &'f mut HashSet<&'static str>,
	pub func_manual_unused: &'f mut HashSet<&'static str>,
	pub func_specialize_unused: &'f mut HashSet<&'static str>,
	pub slice_argument_unused: &'f mut HashSet<&'static str>, // fixme, doesn't seem to work perfectly (shows cv::mixChannels, but it's def used)
}

impl<'tu, 'f> FunctionFinder<'tu, 'f> {
	pub fn update_used_func_identifier(&mut self, identifier: &str) {
		self.func_rename_unused.remove(identifier);
		self.func_cfg_attr_unused.remove(identifier);
		self.func_unsafe_unused.remove(identifier);
		self.func_manual_unused.remove(identifier);
		self.func_specialize_unused.remove(identifier);
	}

	pub fn update_used_func_cpp_fullname(&mut self, cpp_fullname: &str) {
		self.slice_argument_unused.remove(cpp_fullname);
	}
}

impl<'tu> EntityWalkerVisitor<'tu> for FunctionFinder<'tu, '_> {
	fn wants_file(&mut self, path: &Path) -> bool {
		module_from_path(path).map_or(false, |m| m == self.gen_env.module())
			|| main_module_from_path(path).map_or(false, |m| m == self.gen_env.module())
	}

	fn visit_resolve_type(&mut self, _typ: Type<'tu>) -> bool {
		false
	}

	fn visit_entity(&mut self, entity: Entity<'tu>) -> bool {
		match entity.get_kind() {
			EntityKind::ClassDecl | EntityKind::ClassTemplate | EntityKind::ClassTemplatePartialSpecialization
			| EntityKind::StructDecl => {
				let c = Class::new(entity, &self.gen_env);
				if !c.is_template() {
					let fields = c.fields();
					let mut methods = c.methods().into_iter()
						.chain(c.field_methods(fields.iter()))
						.collect::<Vec<_>>();
					entity.walk_methods_while(|child| {
						let func = Func::new(child, &self.gen_env);
						if func.is_generic() {
							methods.push(func);
						}
						true
					});
					let methods = methods.into_iter()
						.map(|m| (m.identifier().into_owned(), m.cpp_fullname().into_owned()))
						.collect::<Vec<_>>();
					for (identifier, cpp_fullname) in methods {
						self.update_used_func_identifier(&identifier);
						self.update_used_func_cpp_fullname(&cpp_fullname);
					}
					entity.walk_classes_while(|child| {
						self.visit_entity(child)
					});
				}
				true
			}
			EntityKind::FunctionDecl => {
				let f = Func::new(entity, &self.gen_env);
				let identifier = f.identifier().into_owned();
				self.update_used_func_identifier(&identifier);
				true
			}
			_ => true
		}
	}
}

fn sorted<'a>(c: impl IntoIterator<Item=&'a str>) -> Vec<&'a str> {
	let mut out = c.into_iter().collect::<Vec<_>>();
	out.sort_unstable();
	out
}

fn show<'a>(c: impl IntoIterator<Item=&'a str>) {
	for f in sorted(c) {
		println!("{}", f);
	}
}

fn main() {
	let mut args = env::args_os().skip(1);
	let opencv_header_base_dir = PathBuf::from(args.next().expect("1st argument must be OpenCV header dir"));
	let src_cpp_dir = PathBuf::from(args.next().expect("2nd argument must be dir with custom cpp"));
	let opencv_header_dirs = opencv_header_base_dir.read_dir().expect("Can't read opencv_header_base_dir")
		.map(|p| p.expect("Bad path").path())
		.filter(|p| p.is_dir());
	let mut func_rename_unused = settings::FUNC_RENAME.keys().copied().collect::<HashSet<_>>();
	let mut func_cfg_attr_unused = settings::FUNC_CFG_ATTR.keys().copied().collect::<HashSet<_>>();
	let mut func_unsafe_unused = settings::FUNC_UNSAFE.clone();
	let mut func_manual_unused = settings::FUNC_MANUAL.keys().copied().collect::<HashSet<_>>();
	let mut func_specialize_unused = settings::FUNC_SPECIALIZE.keys().copied().collect::<HashSet<_>>();
	let mut slice_argument_unused = settings::SLICE_ARGUMENT.keys().copied()
		.map(|(cpp_fullname, _)| cpp_fullname)
		.collect::<HashSet<_>>();
	for opencv_header_dir in opencv_header_dirs {
		println!("Processing header dir: {}", opencv_header_dir.display());
		let modules = opencv_header_dir.join("opencv2").read_dir().expect("Can't read dir")
			.map(|p| p.expect("Bad path").path())
			.filter(|p| p.is_file()
				&& p.extension()
				.map_or(false, |e| e == "hpp")
			)
			.filter_map(|mut p| {
				p.set_extension("");
				p.file_name()
					.and_then(|f| f.to_str())
					.map(|f| f.to_string())
			});
		let clang = Clang::new().expect("Cannot initialize clang");
		let gen = Generator::new(None, &opencv_header_dir, &src_cpp_dir, clang);
		for module in modules {
			println!("  {}", module);
			gen.process_module(&module, false, |root_entity| {
				let gen_env = GeneratorEnv::new(root_entity, &module);
				let walker = EntityWalker::new(root_entity);
				walker.walk_opencv_entities(FunctionFinder {
					gen_env,
					func_rename_unused: &mut func_rename_unused,
					func_cfg_attr_unused: &mut func_cfg_attr_unused,
					func_unsafe_unused: &mut func_unsafe_unused,
					func_manual_unused: &mut func_manual_unused,
					func_specialize_unused: &mut func_specialize_unused,
					slice_argument_unused: &mut slice_argument_unused,
					});
				});
		}
	}
	println!("Unused entries in settings::FUNC_RENAME ({}):", func_rename_unused.len());
	show(func_rename_unused);
	println!("Unused entries in settings::FUNC_CFG_ATTR ({}):", func_cfg_attr_unused.len());
	show(func_cfg_attr_unused);
	println!("Unused entries in settings::FUNC_UNSAFE ({}):", func_unsafe_unused.len());
	show(func_unsafe_unused);
	println!("Unused entries in settings::FUNC_UNSAFE ({}):", func_manual_unused.len());
	show(func_manual_unused);
	println!("Unused entries in settings::FUNC_SPECIALIZE ({}):", func_specialize_unused.len());
	show(func_specialize_unused);
	println!("Unused entries in settings::SLICE_ARGUMENT ({}):", slice_argument_unused.len());
	show(slice_argument_unused);
}
