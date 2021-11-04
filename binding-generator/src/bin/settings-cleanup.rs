use std::{
	cell::RefCell,
	collections::HashSet,
	env,
	fmt,
	mem,
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
	FuncId,
	Generator,
	GeneratorEnv,
	opencv_module_from_path,
	settings,
};

struct FunctionFinder<'tu, 'f> {
	pub gen_env: GeneratorEnv<'tu>,
	pub func_rename_unused: RefCell<&'f mut HashSet<&'static str>>,
	pub func_cfg_attr_unused: RefCell<&'f mut HashSet<&'static str>>,
	pub func_unsafe_unused: RefCell<&'f mut HashSet<FuncId<'static>>>,
	pub func_manual_unused: RefCell<&'f mut HashSet<&'static str>>,
	pub func_specialize_unused: RefCell<&'f mut HashSet<&'static str>>,
	pub argument_override_unused: RefCell<&'f mut HashSet<String>>, // fixme, doesn't seem to work perfectly (shows cv::mixChannels, but it's def used)
}

impl<'tu, 'f> FunctionFinder<'tu, 'f> {
	pub fn update_used_func(&self, f: &Func) {
		let identifier = f.identifier();
		let func_id = f.func_id();
		let cpp_fullname = f.cpp_fullname();

		self.func_rename_unused.borrow_mut().remove(identifier.as_ref());
		self.func_cfg_attr_unused.borrow_mut().remove(identifier.as_ref());
		{
			let static_func_id: FuncId<'static> = unsafe { mem::transmute(func_id) };
			self.func_unsafe_unused.borrow_mut().remove(&static_func_id);
		}
		self.func_manual_unused.borrow_mut().remove(identifier.as_ref());
		self.func_specialize_unused.borrow_mut().remove(identifier.as_ref());
		self.argument_override_unused.borrow_mut().remove(cpp_fullname.as_ref());
	}
}

impl<'tu> EntityWalkerVisitor<'tu> for FunctionFinder<'tu, '_> {
	fn wants_file(&mut self, path: &Path) -> bool {
		opencv_module_from_path(path).map_or(false, |m| m == self.gen_env.module())
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
					let mut methods = c.methods(None).into_iter()
						.chain(c.field_methods(fields.iter(), None))
						.collect::<Vec<_>>();
					entity.walk_methods_while(|child| {
						let func = Func::new(child, &self.gen_env);
						if func.is_generic() {
							methods.push(func);
						}
						true
					});
					for f in methods {
						self.update_used_func(&f);
					}
					entity.walk_classes_while(|child| {
						self.visit_entity(child)
					});
				}
				true
			}
			EntityKind::FunctionDecl => {
				let f = Func::new(entity, &self.gen_env);
				self.update_used_func(&f);
				true
			}
			_ => true
		}
	}
}

fn show<S: fmt::Display>(c: impl IntoIterator<Item=S>) {
	let v = c.into_iter().collect::<Vec<_>>();
	let mut sorted = v.iter()
		.map(|s| s.to_string())
		.collect::<Vec<_>>();
	sorted.sort_unstable();
	for f in sorted {
		println!("{}", f);
	}
}

fn main() {
	let mut args = env::args_os().skip(1);
	let src_cpp_dir = PathBuf::from(args.next().expect("2nd argument must be dir with custom cpp"));
	let opencv_header_dirs = args.into_iter().map(PathBuf::from);
	let mut func_rename_unused = settings::FUNC_RENAME.keys().copied().collect::<HashSet<_>>();
	let mut func_cfg_attr_unused = settings::FUNC_CFG_ATTR.keys().copied().collect::<HashSet<_>>();
	let mut func_unsafe_unused = settings::FUNC_UNSAFE.clone();
	let mut func_manual_unused = settings::FUNC_MANUAL.keys().copied().collect::<HashSet<_>>();
	let mut func_specialize_unused = settings::FUNC_SPECIALIZE.keys().copied().collect::<HashSet<_>>();
	let mut argument_override_unused = settings::ARGUMENT_OVERRIDE.keys().cloned()
		.map(|func_id| func_id.name().to_string())
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
		let gen = Generator::new(&opencv_header_dir, &[], &src_cpp_dir, clang);
		for module in modules {
			println!("  {}", module);
			gen.process_module(&module, false, |root_tu, _hdr| {
				let root_entity = root_tu.get_entity();
				let gen_env = GeneratorEnv::new(root_entity, &module);
				let walker = EntityWalker::new(root_entity);
				walker.walk_opencv_entities(FunctionFinder {
					gen_env,
					func_rename_unused: RefCell::new(&mut func_rename_unused),
					func_cfg_attr_unused: RefCell::new(&mut func_cfg_attr_unused),
					func_unsafe_unused: RefCell::new(&mut func_unsafe_unused),
					func_manual_unused: RefCell::new(&mut func_manual_unused),
					func_specialize_unused: RefCell::new(&mut func_specialize_unused),
					argument_override_unused: RefCell::new(&mut argument_override_unused),
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
	println!("Unused entries in settings::ARGUMENT_OVERRIDE ({}):", argument_override_unused.len());
	show(argument_override_unused);
}
