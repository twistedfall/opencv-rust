use std::{
	borrow::Cow,
	cmp,
	collections::HashSet,
	fmt,
	hash,
	iter,
};

use clang::Entity;
use maplit::hashmap;
use once_cell::sync::Lazy;

use crate::{
	CompiledInterpolation,
	Const,
	Constness,
	DefaultElement,
	DefinitionLocation,
	DependentTypeMode,
	Element,
	EntityElement,
	EntityExt,
	Field,
	Func,
	FunctionTypeHint,
	GeneratedElement,
	GeneratorEnv,
	get_debug,
	IteratorExt,
	settings,
	StrExt,
	TypeRef,
};

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq)]
pub enum Kind {
	Simple,
	Boxed,
	System,
	Excluded,
}

#[derive(Clone)]
pub struct Class<'tu, 'g> {
	entity: Entity<'tu>,
	custom_fullname: Option<String>,
	gen_env: & 'g GeneratorEnv < 'tu >,
}

impl<'tu, 'g> Class<'tu, 'g> {
	pub fn new(entity: Entity<'tu>, gen_env: &'g GeneratorEnv<'tu>) -> Self {
		Self { entity, custom_fullname: None, gen_env }
	}

	pub fn new_ext(entity: Entity<'tu>, custom_fullname: String, gen_env: &'g GeneratorEnv<'tu>) -> Self {
		Self { entity, custom_fullname: Some(custom_fullname), gen_env }
	}

	fn kind(&self) -> Kind {
		if settings::ELEMENT_EXCLUDE.is_match(self.cpp_fullname().as_ref()) {
			return Kind::Excluded
		}
		match self.gen_env.get_export_config(self.entity).map(|c| c.simple) {
			Some(true) => Kind::Simple,
			Some(false) => Kind::Boxed,
			None => {
				if self.is_system() {
					Kind::System
				} else if let Some(kind) = self.gen_env.get_class_kind(self.entity) {
					kind
				} else {
					Kind::Excluded
				}
			}
		}
	}

	pub fn type_ref(&self) -> TypeRef<'tu, 'g> {
		TypeRef::new(self.entity.get_type().expect("Can't get class type"), self.gen_env)
	}

	pub fn detect_class_simplicity(&self) -> bool {
		!self.has_bases()
			&& !self.has_descendants()
			&& self.fields().into_iter()
			.map(|f| f.type_ref())
			.all(|t| t.is_copy())
	}

	pub fn as_template(&self) -> Option<Class<'tu, 'g>> {
		self.entity.get_template()
			.map(|t| Class::new(t, self.gen_env))
	}

	pub fn is_simple(&self) -> bool {
		self.kind() == Kind::Simple
	}

	pub fn is_boxed(&self) -> bool {
		self.kind() == Kind::Boxed
	}

	pub fn is_template(&self) -> bool {
		self.entity.get_template_kind().is_some()
	}

	pub fn is_template_specialization(&self) -> bool {
		self.entity.get_template().is_some()
	}

	pub fn is_abstract(&self) -> bool {
		self.entity.is_abstract_record()
		// is_abstract_record() also check parent classes for presence of pure virtual methods, we don't want that
//		self.entity.walk_methods_while(|child| !Func::new(child, self.gen_env).is_abstract())
	}

	pub fn is_trait(&self) -> bool {
		self.is_boxed()
//		self.is_abstract() || self.has_descendants() || settings::FORCE_CLASS_TRAIT.contains(self.cpp_fullname().as_ref())
	}

	pub fn is_by_ptr(&self) -> bool {
		match self.kind() {
			Kind::Boxed => true,
			Kind::Simple | Kind::System | Kind::Excluded => false,
		}
	}

	pub fn rust_trait_name(&self, full: bool) -> Cow<str> {
		let mut out = self.rust_name(full);
		if self.is_trait() && !self.is_abstract() {
			out.to_mut().push_str("Trait");
		}
		out
	}

	pub fn rust_trait_localname(&self) -> Cow<str> {
		self.rust_trait_name(false)
	}

	pub fn rust_trait_fullname(&self) -> Cow<str> {
		self.rust_trait_name(true)
	}

	pub fn has_bases(&self) -> bool {
		self.entity.walk_bases_while(|_| false)
	}

	pub fn bases(&self) -> Vec<Class<'tu, 'g>> {
		let mut out = vec![];
		let entity = if let Some(entity) = self.entity.get_template() {
			entity
		} else {
			self.entity
		};
		entity.walk_bases_while(|child| {
			out.push(Class::new(child.get_definition().expect("Can't get base class definition"), self.gen_env));
			true
		});
		out
	}

	pub fn all_bases(&self) -> HashSet<Class<'tu, 'g>> {
		self.bases().into_iter()
			.map(|b| {
				let mut out = b.all_bases();
				out.insert(b);
				out
			})
			.flatten()
			.collect()
	}

	pub fn has_descendants(&self) -> bool {
		self.gen_env.has_descendants(self.entity)
	}

	pub fn has_methods(&self) -> bool {
		self.entity.walk_methods_while(|_| false)
	}

	pub fn methods(&self) -> Vec<Func<'tu, 'g>> {
		let mut out = Vec::with_capacity(64);
		self.entity.walk_methods_while(|child| {
			let func = Func::new(child, self.gen_env);
			if func.is_generic() {
				if let Some(specs) = settings::FUNC_SPECIALIZE.get(func.identifier().as_ref()) {
					for spec in specs {
						out.push(Func::new_ext(child, FunctionTypeHint::Specialized(spec), None, self.gen_env));
					}
					return true;
				}
			}
			out.push(func);
			true
		});
		out
	}

	pub fn has_fields(&self) -> bool {
		self.entity.walk_fields_while(|_| false)
	}

	pub fn fields(&self) -> Vec<Field<'tu, 'g>> {
		let mut out = Vec::with_capacity(32);
		self.entity.walk_fields_while(|child| {
			out.push(Field::new(child, self.gen_env));
			true
		});
		out
	}

	pub fn consts(&self) -> Vec<Const<'tu>> {
		let mut out = Vec::with_capacity(8);
		self.entity.walk_consts_while(|child| {
			out.push(Const::new(child));
			true
		});
		out
	}

	pub fn is_definition(&self) -> bool {
		let class_loc = self.entity.get_location();
		let def_loc = self.entity.get_definition().and_then(|d| d.get_location());
		match (class_loc, def_loc) {
			(Some(class_loc), Some(def_loc)) => {
				class_loc == def_loc
			}
			(_, None) => {
				false
			}
			_ => {
				true
			}
		}
	}

	pub fn dependent_types(&self) -> Vec<Box<dyn GeneratedElement + 'g>> {
		self.fields().into_iter()
			.filter(|f| !f.is_excluded())
			.map(|f| f.type_ref().dependent_types_with_mode(DependentTypeMode::ForReturn(DefinitionLocation::Module)))
			.flatten()
			.chain(self.methods().into_iter()
				.filter(|m| !m.is_excluded())
				.map(|m| m.dependent_types())
				.flatten()
			)
			.collect()
	}

	pub fn cpp_method_call_name(&self, method_name: &str) -> String {
		if self.is_by_ptr() {
			format!("instance->{name}", name=method_name)
		} else {
			format!("instance.{name}", name=method_name)
		}
	}

	pub fn field_methods(&self, fields: impl Iterator<Item=&'g Field<'tu, 'g>>) -> Vec<Func<'tu, 'g>> {
		let mut out = Vec::with_capacity(fields.size_hint().1.map_or(8, |x| x * 2));
		out.extend(fields
			.map(|fld| {
				iter::from_fn({
					let fld_type_ref = fld.type_ref();
					let mut need_to_yield_read = true;
					let mut need_to_yield_write = !fld_type_ref.is_const() && !fld_type_ref.as_fixed_array().is_some();
					move || {
						if need_to_yield_read {
							need_to_yield_read = false;
							Some(Func::new(fld.entity(), self.gen_env))
						} else if need_to_yield_write {
							need_to_yield_write = false;
							Some(Func::new_ext(fld.entity(), FunctionTypeHint::FieldSetter, None, self.gen_env))
						} else {
							None
						}
					}
				})
			})
			.flatten()
		);
		out
	}

	fn gen_rust_class(&self, opencv_version: &str) -> String {
		static BOXED_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/class/boxed.tpl.rs").compile_interpolation()
		);

		static IMPL_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/class/impl.tpl.rs").compile_interpolation()
		);

		static SIMPLE_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/class/simple.tpl.rs").compile_interpolation()
		);

		static SIMPLE_FIELD_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/class/simple_field.tpl.rs").compile_interpolation()
		);

		static BASE_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/class/base.tpl.rs").compile_interpolation()
		);

		static SIMPLE_BASE_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/class/simple_base.tpl.rs").compile_interpolation()
		);

		static TRAIT_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/class/trait.tpl.rs").compile_interpolation()
		);

		static TRAIT_DYN_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/class/trait_dyn.tpl.rs").compile_interpolation()
		);

		let type_ref = self.type_ref();
		let is_trait = self.is_trait();
		let is_abstract = self.is_abstract();
		let is_simple = self.is_simple();

		let mut out = String::new();

		let consts = self.consts();
		let fields = self.fields();
		let mut methods = if is_simple {
			vec![]
		} else {
			self.field_methods(fields.iter())
		};
		methods.extend_from_slice(self.methods().as_slice());
		if is_trait {
			let mut bases = self.bases().into_iter()
				.filter(|b| !b.is_excluded() && !b.is_simple()) // todo, allow extension of simple classes for e.g. Elliptic_KeyPoint
				.map(|x| x.rust_trait_fullname().into_owned())
				.collect::<Vec<_>>();
			bases.sort_unstable();
			let mut trait_bases: String = bases.join(" + ");
			if !trait_bases.is_empty() {
				trait_bases.insert_str(0, ": ");
			};
			let trait_methods = Func::rust_generate_funcs(
				methods.iter().filter(|m| m.as_instance_method().is_some()),
				opencv_version
			);
			let dyn_impl = if is_abstract {
				let consts = consts.iter()
					.map(|c| c.gen_rust(opencv_version))
					.join("");

				let methods = Func::rust_generate_funcs(
					methods.iter().filter(|m| m.as_static_method().is_some()),
					opencv_version
				);
				if !methods.is_empty() || !consts.is_empty() {
					TRAIT_DYN_TPL.interpolate(&hashmap! {
						"rust_local" => self.rust_trait_localname(),
						"consts" => consts.into(),
						"methods" => methods.into(),
					})
				} else {
					String::new()
				}
			} else {
				String::new()
			};
			out = TRAIT_TPL.interpolate(&hashmap! {
				"doc_comment" => Cow::Owned(self.rendered_doc_comment(opencv_version)),
				"debug" => get_debug(self).into(),
				"rust_trait_local" => self.rust_trait_localname(),
				"rust_local" => type_ref.rust_local(),
				"rust_extern_const" => type_ref.rust_extern_with_const(Constness::Const),
				"rust_extern_mut" => type_ref.rust_extern_with_const(Constness::Mut),
				"bases" => trait_bases.into(),
				"trait_methods" => trait_methods.into(),
				"dyn_impl" => dyn_impl.into(),
			});
		}

		if !is_abstract {
			let mut bases = self.all_bases().into_iter()
				.filter(|b| !b.is_excluded() && !b.is_simple()) // todo, allow extension of simple classes for e.g. Elliptic_KeyPoint
				.collect::<Vec<_>>();
			if is_trait {
				bases.push(self.clone());
			}
			bases.sort_unstable_by(|a, b| a.cpp_localname().cmp(&b.cpp_localname()));
			let bases = bases.into_iter()
				.map(|base| {
					let base_type_ref = base.type_ref();
					let tpl = if is_simple {
						&SIMPLE_BASE_TPL
					} else {
						&BASE_TPL
					};
					tpl.interpolate(&hashmap! {
						"base_rust_full" => base.rust_trait_fullname(),
						"rust_local" => type_ref.rust_local(),
						"base_rust_local" => base_type_ref.rust_local(),
						"base_rust_extern_const" => base_type_ref.rust_extern_with_const(Constness::Const),
						"base_rust_extern_mut" => base_type_ref.rust_extern_with_const(Constness::Mut),
					})
				})
				.collect::<Vec<_>>();

			let fields = if is_simple {
				let mut out: Vec<_> = fields.iter()
					.map(|f| {
						let type_ref = f.type_ref();
						let mut typ = type_ref.rust_full();
						// hack for converting the references to array types in struct definitions
						if type_ref.as_fixed_array().is_some() {
							if let Some(new_typ) = typ.strip_str_prefix("&mut ") {
								typ = new_typ.to_string().into()
							}
						}
						SIMPLE_FIELD_TPL.interpolate(&hashmap! {
							"doc_comment" => Cow::Owned(f.rendered_doc_comment(opencv_version)),
							"visibility" => "pub ".into(),
							"name" => f.rust_leafname(),
							"type" => typ,
						})
					})
					.collect();
				if out.is_empty() {
					out.push(
						SIMPLE_FIELD_TPL.interpolate(&hashmap! {
							"doc_comment" => "",
							"visibility" => "",
							"name" => "__rust_private",
							"type" => "[u8; 0]",
						})
					)
				}
				out
			} else {
				vec![]
			};

			let mut inherent_methods = String::with_capacity(512 * methods.len());
			let is_trait = self.is_trait();
			inherent_methods.push_str(&if is_trait {
				Func::rust_generate_funcs(
					methods.iter()
						.filter(|m| m.as_static_method().is_some() || m.as_constructor().is_some()),
					opencv_version
				)
			} else {
				Func::rust_generate_funcs(&methods, opencv_version)
			});

			let tpl = if is_simple {
				&SIMPLE_TPL
			} else {
				&BOXED_TPL
			};

			let consts = consts.iter()
				.map(|c| c.gen_rust(opencv_version))
				.join("");

			out += &tpl.interpolate(&hashmap! {
				"doc_comment" => Cow::Owned(self.rendered_doc_comment(opencv_version)),
				"debug" => get_debug(self).into(),
				"rust_local" => type_ref.rust_local(),
				"rust_extern_const" => type_ref.rust_extern_with_const(Constness::Const),
				"rust_extern_mut" => type_ref.rust_extern_with_const(Constness::Mut),
				"fields" => fields.join("").into(),
				"bases" => bases.join("").into(),
				"impl" => IMPL_TPL.interpolate(&hashmap! {
					"rust_local" => self.rust_localname(),
					"consts" => consts.into(),
					"methods" => inherent_methods.into(),
				}).into()
			});
		}
		out
	}

	fn gen_rust_exports_boxed(&self) -> String {
		let fields = self.fields();
		let mut out = String::with_capacity((fields.len() + 1) * 128);
		for func in self.field_methods(fields.iter().filter(|f| !f.is_excluded())) {
			if !func.is_excluded() {
				out += &func.gen_rust_exports();
			}
		}
		out
	}

	fn gen_cpp_boxed(&self) -> String {
		static BOXED_CPP_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/class/boxed_dtor.tpl.cpp").compile_interpolation()
		);

		let fields = self.fields();
		let mut out = String::with_capacity(fields.len() * 512);
		for func in self.field_methods(fields.iter().filter(|f| !f.is_excluded())) {
			if !func.is_excluded() {
				out += &func.gen_cpp();
			}
		}

		if !self.is_abstract() {
			let type_ref = self.type_ref();
			out += &BOXED_CPP_TPL.interpolate(&hashmap! {
				"rust_local" => type_ref.rust_local(),
				"cpp_full" => type_ref.cpp_full(),
				"cpp_extern" => type_ref.cpp_extern(),
			})
		}
		out
	}
}

impl<'tu> EntityElement<'tu> for Class<'tu, '_> {
	fn entity(&self) -> Entity<'tu> {
		self.entity
	}
}

impl Element for Class<'_, '_> {
	fn is_excluded(&self) -> bool {
		DefaultElement::is_excluded(self)
			|| match self.kind() { Kind::Excluded => true, _ => false }
			|| self.cpp_namespace() == "" // we don't process out of namespace (legacy C) items, so mark them as excluded
	}

	fn is_ignored(&self) -> bool {
		DefaultElement::is_ignored(self)
			|| self.is_template()
			|| self.is_template_specialization() && !settings::IMPLEMENTED_GENERICS.contains(self.cpp_fullname().as_ref())
			|| !self.is_template_specialization() && !self.is_definition()
	}

	fn is_system(&self) -> bool {
		DefaultElement::is_system(self)
	}

	fn is_public(&self) -> bool {
		DefaultElement::is_public(self)
	}

	fn usr(&self) -> Cow<str> {
		DefaultElement::usr(self)
	}

	fn rendered_doc_comment_with_prefix(&self, prefix: &str, opencv_version: &str) -> String {
		DefaultElement::rendered_doc_comment_with_prefix(self, prefix, opencv_version)
	}

	fn cpp_namespace(&self) -> Cow<str> {
		if self.custom_fullname.is_some() {
			self.cpp_fullname().namespace().to_string().into()
		} else {
			DefaultElement::cpp_namespace(self)
		}
	}

	fn cpp_localname(&self) -> Cow<str> {
		if self.custom_fullname.is_some() {
			self.cpp_fullname().localname().to_string().into()
		} else {
			DefaultElement::cpp_localname(self)
		}
	}

	fn cpp_fullname(&self) -> Cow<str> {
		if let Some(custom_fullname) = &self.custom_fullname {
			custom_fullname.into()
		} else {
			DefaultElement::cpp_fullname(self)
		}
	}

	fn rust_module(&self) -> Cow<str> {
		DefaultElement::rust_module(self)
	}

	fn rust_leafname(&self) -> Cow<str> {
		if self.type_ref().is_string() {
			"String".into()
		} else {
			self.cpp_localname()
		}
	}

	fn rust_localname(&self) -> Cow<str> {
		DefaultElement::rust_localname(self)
	}
}

impl GeneratedElement for Class<'_, '_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_module(), self.rust_localname())
	}

	fn gen_rust(&self, opencv_version: &str) -> String {
		match self.kind() {
			Kind::Simple => {
				self.gen_rust_class(opencv_version)
			}
			Kind::Boxed => {
				self.gen_rust_class(opencv_version)
			},
			Kind::System | Kind::Excluded => {
				"".to_string()
			},
		}
	}

	fn gen_rust_exports(&self) -> String {
		let out = match self.kind() {
			Kind::Simple => {
				"".to_string()
			}
			Kind::Boxed => {
				self.gen_rust_exports_boxed()
			},
			Kind::System | Kind::Excluded => {
				"".to_string()
			},
		};

		let mut methods = self.methods().into_iter()
			.filter(|m| !m.is_excluded())
			.map(|m| m.gen_rust_exports());

		out + &methods.join("")
	}

	fn gen_cpp(&self) -> String {
		let out = match self.kind() {
			Kind::Simple => {
				"".to_string()
			}
			Kind::Boxed => {
				self.gen_cpp_boxed()
			},
			Kind::System | Kind::Excluded => {
				"".to_string()
			},
		};

		let methods: Vec<_> = self.methods().into_iter()
			.filter(|m| !m.is_excluded())
			.map(|m| m.gen_cpp())
			.collect();

		out + &methods.join("")
	}
}

impl hash::Hash for Class<'_, '_> {
	fn hash<H: hash::Hasher>(&self, state: &mut H) {
		self.entity.hash(state)
	}
}

impl cmp::PartialEq for Class<'_, '_> {
	fn eq(&self, other: &Self) -> bool {
		self.entity.eq(&other.entity)
	}
}

impl cmp::Eq for Class<'_, '_> {}

impl fmt::Display for Class<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.entity.get_display_name().expect("Can't get display name"))
	}
}

impl fmt::Debug for Class<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut props = vec![];
		if self.is_template() {
			props.push("template");
		}
		if self.is_template_specialization() {
			props.push("template_specialization");
		}
		if self.is_abstract() {
			props.push("abstract");
		}
		if self.is_trait() {
			props.push("trait");
		}
		if self.is_by_ptr() {
			props.push("by_ptr");
		}
		if self.is_simple() {
			props.push("simple");
		}
		if self.has_descendants() {
			props.push("has_descendants");
		}
		if self.has_methods() {
			props.push("has_methods");
		}
		if self.has_fields() {
			props.push("has_fields");
		}
		if self.has_bases() {
			props.push("has_bases");
		}
		let mut debug_struct = f.debug_struct("Class");
		self.update_debug_struct(&mut debug_struct)
			.field("export_config", &self.gen_env.get_export_config(self.entity))
			.field("kind", &self.kind())
			.field("props", &props.join(", "))
			.field("as_template", &self.as_template())
//			.field("type_ref", &self.type_ref())
//			.field("bases", &self.bases())
//			.field("methods", &self.methods())
//			.field("fields", &self.fields())
			.finish()
	}
}
