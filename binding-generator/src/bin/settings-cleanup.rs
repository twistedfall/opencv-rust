use std::cell::RefCell;
use std::collections::HashSet;
use std::ops::ControlFlow;
use std::path::{Path, PathBuf};
use std::{env, fmt};

use clang::{Entity, EntityKind};
use opencv_binding_generator::{
	opencv_module_from_path, settings, Class, EntityExt, EntityWalkerExt, EntityWalkerVisitor, Func, FuncId, Generator,
	GeneratorEnv,
};

struct FunctionFinder<'tu, 'f> {
	pub gen_env: GeneratorEnv<'tu>,
	pub func_rename_unused: RefCell<&'f mut HashSet<&'static str>>,
	pub func_exclude_unused: RefCell<&'f mut HashSet<&'static str>>,
	pub func_cfg_attr_unused: RefCell<&'f mut HashSet<&'static str>>,
	pub func_unsafe_unused: RefCell<&'f mut HashSet<FuncId<'static>>>,
	pub func_replace_unused: RefCell<&'f mut HashSet<FuncId<'static>>>,
	pub func_specialize_unused: RefCell<&'f mut HashSet<FuncId<'static>>>,
	pub argument_override_unused: RefCell<&'f mut HashSet<FuncId<'static>>>,
}

impl<'tu, 'f> FunctionFinder<'tu, 'f> {
	pub fn update_used_func(&self, f: &Func) {
		let identifier = f.identifier();
		let func_id = f.func_id().make_static();

		self.func_rename_unused.borrow_mut().remove(identifier.as_str());
		self.func_exclude_unused.borrow_mut().remove(identifier.as_str());
		self.func_cfg_attr_unused.borrow_mut().remove(identifier.as_str());
		self.func_unsafe_unused.borrow_mut().remove(&func_id);
		self.func_replace_unused.borrow_mut().remove(&func_id);
		self.func_specialize_unused.borrow_mut().remove(&func_id);
		self.argument_override_unused.borrow_mut().remove(&func_id);
	}
}

impl<'tu> EntityWalkerVisitor<'tu> for FunctionFinder<'tu, '_> {
	fn wants_file(&mut self, path: &Path) -> bool {
		opencv_module_from_path(path).map_or(false, |m| m == self.gen_env.module())
	}

	fn visit_entity(&mut self, entity: Entity<'tu>) -> ControlFlow<()> {
		match entity.get_kind() {
			EntityKind::ClassDecl
			| EntityKind::ClassTemplate
			| EntityKind::ClassTemplatePartialSpecialization
			| EntityKind::StructDecl => {
				let c = Class::new(entity, &self.gen_env);
				if !c.template_kind().is_template() {
					c.methods().into_iter().for_each(|f| self.update_used_func(&f));
					let fields = c.fields();
					c.field_methods(fields.iter(), None)
						.into_iter()
						.for_each(|f| self.update_used_func(&f));
					entity.walk_methods_while(|child| {
						let func = Func::new(child, &self.gen_env);
						if func.is_generic() {
							self.update_used_func(&func);
						}
						ControlFlow::Continue(())
					});
					entity.walk_classes_while(|child| self.visit_entity(child));
				}
			}
			EntityKind::FunctionDecl => {
				let f = Func::new(entity, &self.gen_env);
				self.update_used_func(&f);
			}
			_ => {}
		}
		ControlFlow::Continue(())
	}
}

fn show<S: fmt::Display>(c: impl IntoIterator<Item = S>) {
	let v = c.into_iter().collect::<Vec<_>>();
	let mut sorted = v.iter().map(|s| s.to_string()).collect::<Vec<_>>();
	sorted.sort_unstable();
	for f in sorted {
		println!("{f}");
	}
}

fn main() {
	let mut args = env::args_os().skip(1);
	let src_cpp_dir = PathBuf::from(args.next().expect("2nd argument must be dir with custom cpp"));
	let opencv_header_dirs = args.map(PathBuf::from);
	let mut func_rename_unused = settings::FUNC_RENAME.keys().copied().collect::<HashSet<_>>();
	let mut func_exclude_unused = settings::FUNC_EXCLUDE.clone();
	let mut func_cfg_attr_unused = settings::FUNC_CFG_ATTR.keys().copied().collect::<HashSet<_>>();
	let mut func_unsafe_unused = settings::FUNC_UNSAFE.clone();
	let mut func_replace_unused = settings::FUNC_REPLACE.keys().cloned().collect::<HashSet<_>>();
	let mut func_specialize_unused = settings::FUNC_SPECIALIZE.keys().cloned().collect::<HashSet<_>>();
	let mut argument_override_unused = settings::ARGUMENT_OVERRIDE.keys().cloned().collect::<HashSet<_>>();
	for opencv_header_dir in opencv_header_dirs {
		println!("Processing header dir: {}", opencv_header_dir.display());
		let modules = opencv_header_dir
			.join("opencv2")
			.read_dir()
			.expect("Can't read dir")
			.map(|p| p.expect("Bad path").path())
			.filter(|p| p.is_file() && p.extension().map_or(false, |e| e == "hpp"))
			.filter_map(|mut p| {
				p.set_extension("");
				p.file_name().and_then(|f| f.to_str()).map(|f| f.to_string())
			});
		let gen = Generator::new(&opencv_header_dir, &[], &src_cpp_dir);
		for module in modules {
			println!("  {module}");
			gen.pre_process(&module, false, |root_entity| {
				let gen_env = GeneratorEnv::new(root_entity, &module);
				root_entity.walk_opencv_entities(FunctionFinder {
					gen_env,
					func_rename_unused: RefCell::new(&mut func_rename_unused),
					func_exclude_unused: RefCell::new(&mut func_exclude_unused),
					func_cfg_attr_unused: RefCell::new(&mut func_cfg_attr_unused),
					func_unsafe_unused: RefCell::new(&mut func_unsafe_unused),
					func_replace_unused: RefCell::new(&mut func_replace_unused),
					func_specialize_unused: RefCell::new(&mut func_specialize_unused),
					argument_override_unused: RefCell::new(&mut argument_override_unused),
				});
			});
		}
	}
	println!("Unused entries in settings::FUNC_RENAME ({}):", func_rename_unused.len());
	show(func_rename_unused);
	println!("Unused entries in settings::FUNC_EXCLUDE ({}):", func_exclude_unused.len());
	show(func_exclude_unused);
	println!("Unused entries in settings::FUNC_CFG_ATTR ({}):", func_cfg_attr_unused.len());
	show(func_cfg_attr_unused);
	println!("Unused entries in settings::FUNC_UNSAFE ({}):", func_unsafe_unused.len());
	show(func_unsafe_unused);
	println!("Unused entries in settings::FUNC_REPLACE ({}):", func_replace_unused.len());
	show(func_replace_unused);
	println!(
		"Unused entries in settings::FUNC_SPECIALIZE ({}):",
		func_specialize_unused.len()
	);
	show(func_specialize_unused);
	println!(
		"Unused entries in settings::ARGUMENT_OVERRIDE ({}):",
		argument_override_unused.len()
	);
	show(argument_override_unused);
}
