use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::env;
use std::ops::ControlFlow;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use clang::{Entity, EntityKind};
use opencv_binding_generator::{
	opencv_module_from_path, Class, Constness, EntityExt, EntityWalkerExt, EntityWalkerVisitor, Func, Generator, GeneratorEnv,
	Pred,
};

struct FunctionFinder<'tu> {
	pub module: &'tu str,
	pub gen_env: GeneratorEnv<'tu>,
}

impl<'tu> FunctionFinder<'tu> {
	pub fn update_used_func(&self, f: &Func) {
		let mut matcher = f.matcher();
		self.gen_env.settings.arg_override.get(&mut matcher);
		self.gen_env.settings.return_override.get(&mut matcher);
		self.gen_env.settings.force_infallible.get(&mut matcher);
		self.gen_env.settings.func_cfg_attr.get(&mut matcher);
		self.gen_env.settings.func_companion_tweak.get(&mut matcher);
		self.gen_env.settings.func_replace.get(&mut matcher);
		self.gen_env.settings.func_specialize.get(&mut matcher);
		self.gen_env.settings.func_unsafe.get(&mut matcher);
	}
}

impl<'tu> EntityWalkerVisitor<'tu> for &mut FunctionFinder<'tu> {
	fn wants_file(&mut self, path: &Path) -> bool {
		opencv_module_from_path(path).map_or(false, |m| m == self.module)
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

fn main() {
	let mut args = env::args_os().skip(1);
	let src_cpp_dir = PathBuf::from(args.next().expect("2nd argument must be dir with custom cpp"));
	let opencv_header_dirs = args.map(PathBuf::from);
	// module -> usage_section -> (name, preds)
	let global_usage_tracking = Rc::new(RefCell::new(HashMap::<
		String,
		HashMap<&'static str, HashSet<UsageTrackerOwned>>,
	>::new()));
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
			gen.pre_process(&module, false, {
				let global_usage_tracking = Rc::clone(&global_usage_tracking);
				|root_entity| {
					let global_usage_tracking = global_usage_tracking; // force move
					let mut gen_env = GeneratorEnv::global(&module, root_entity);
					gen_env.settings.start_usage_tracking();
					let mut function_finder = FunctionFinder {
						module: &module,
						gen_env,
					};
					root_entity.walk_opencv_entities(&mut function_finder);

					let usage_tracking = function_finder.gen_env.settings.finish_usage_tracking();
					let mut global_usage_tracking = global_usage_tracking.borrow_mut();
					let module_usage_tracking = global_usage_tracking.entry(module.to_string()).or_default();
					for (usage_section, new_usage_tracking) in usage_tracking {
						let new_usage_tracking: HashSet<UsageTrackerOwned> = new_usage_tracking
							.into_iter()
							.map(|(name, preds)| (name.to_string(), preds.iter().map(PredOwned::from_pred).collect()))
							.collect();
						if let Some(prev_usage_tracking) = module_usage_tracking.get_mut(usage_section) {
							*prev_usage_tracking = new_usage_tracking.intersection(prev_usage_tracking).cloned().collect();
						} else {
							module_usage_tracking.insert(usage_section, new_usage_tracking);
						}
					}
				}
			});
		}
	}

	let global_usage_tracking = Rc::try_unwrap(global_usage_tracking).expect("Not owned").into_inner();
	let mut usage_per_section = HashMap::new();
	for (_module, module_usage) in global_usage_tracking {
		for (section, preds) in module_usage {
			let section_usage = usage_per_section.entry(section).or_insert_with(Vec::new);
			for (name, preds) in preds {
				section_usage.push((name, preds));
			}
		}
	}
	for (section, mut usage_tracking) in usage_per_section {
		if usage_tracking.is_empty() {
			println!("No unused entries in {section}");
		} else {
			println!("Unused entries in {section} ({}):", usage_tracking.len());
			usage_tracking.sort_unstable();
			for (name, mut preds) in usage_tracking {
				preds.sort_unstable();
				println!("  {name}: {preds:?}");
			}
		}
	}
}

type UsageTrackerOwned = (String, Vec<PredOwned>);

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum PredOwned {
	Constness(Constness),
	Return(String),
	ArgNames(Vec<String>),
	ArgTypes(Vec<String>),
}

impl PredOwned {
	fn from_pred(pred: &Pred) -> Self {
		match pred {
			Pred::Constness(c) => Self::Constness(*c),
			Pred::Return(r) => Self::Return(r.to_string()),
			Pred::ArgNames(a) => Self::ArgNames(a.iter().map(|s| s.to_string()).collect()),
			Pred::ArgTypes(a) => Self::ArgTypes(a.iter().map(|s| s.to_string()).collect()),
		}
	}
}
