use std::borrow::Cow;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NameStyle {
	Declaration,
	Reference(FishStyle),
}

impl NameStyle {
	pub fn decl() -> Self {
		Self::Declaration
	}

	pub fn ref_() -> Self {
		Self::Reference(FishStyle::No)
	}

	pub fn ref_fish() -> Self {
		Self::Reference(FishStyle::Turbo)
	}

	pub fn is_declaration(&self) -> bool {
		match self {
			Self::Declaration => true,
			Self::Reference(..) => false,
		}
	}

	pub fn is_reference(&self) -> bool {
		!self.is_declaration()
	}

	pub fn turbo_fish_style(&self) -> FishStyle {
		match self {
			Self::Reference(fish_style) => *fish_style,
			Self::Declaration => FishStyle::No,
		}
	}

	pub fn rust_turbo_fish_qual(&self) -> &'static str {
		match self {
			Self::Declaration => "",
			Self::Reference(fish) => fish.rust_qual(),
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FishStyle {
	No,
	Turbo,
}

impl FishStyle {
	pub fn from_is_turbo(is_turbo: bool) -> Self {
		if is_turbo {
			Self::Turbo
		} else {
			Self::No
		}
	}

	pub fn is_turbo(&self) -> bool {
		match self {
			FishStyle::No => false,
			FishStyle::Turbo => true,
		}
	}

	pub fn rust_qual(&self) -> &'static str {
		match self {
			FishStyle::No => "",
			FishStyle::Turbo => "::",
		}
	}

	pub fn apply<'s>(&self, rust_name: &'s str) -> Cow<'s, str> {
		match self {
			FishStyle::No => {
				if let Some(fish_idx) = rust_name.find("::<") {
					let mut rust_name = rust_name.to_string();
					rust_name.drain(fish_idx..(fish_idx + FishStyle::Turbo.rust_qual().len()));
					rust_name.into()
				} else {
					rust_name.into()
				}
			}
			FishStyle::Turbo => {
				if let Some((before_bracket, _)) = rust_name.split_once('<') {
					if before_bracket.ends_with("::") {
						rust_name.into()
					} else {
						let mut rust_name = rust_name.to_string();
						rust_name.insert_str(before_bracket.len(), "::");
						rust_name.into()
					}
				} else {
					rust_name.into()
				}
			}
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CppNameStyle {
	Declaration,
	Reference,
}

impl CppNameStyle {
	pub fn is_declaration(&self) -> bool {
		match self {
			Self::Declaration => true,
			Self::Reference => false,
		}
	}

	pub fn is_reference(&self) -> bool {
		!self.is_declaration()
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StrType {
	StdString(StrEnc),
	CvString(StrEnc),
	CharPtr,
}

impl StrType {
	pub fn set_encoding(&mut self, enc: StrEnc) {
		match self {
			StrType::StdString(old_enc) | StrType::CvString(old_enc) => *old_enc = enc,
			StrType::CharPtr => {}
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StrEnc {
	Text,
	/// string with binary data, e.g. can contain 0 byte
	Binary,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir<T> {
	In(T),
	Out(T),
}

impl<T> Dir<T> {
	pub fn with_out_dir(self, is_out_dir: bool) -> Self {
		let inner = match self {
			Self::In(inner) | Self::Out(inner) => inner,
		};
		if is_out_dir {
			Self::Out(inner)
		} else {
			Self::In(inner)
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExternDir {
	/// used for inner type (e.g. for Point*) and for callbacks
	Pure,
	ToCpp,
	FromCpp,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExternPassKind {
	AsIs,
	ByPtr,
	/// Value of the type needs to be passed by pointer to a heap-allocated object to and from the C++ side
	ByVoidPtr,
}

impl ExternPassKind {
	pub fn is_by_ptr(&self) -> bool {
		match self {
			ExternPassKind::AsIs => false,
			ExternPassKind::ByPtr | ExternPassKind::ByVoidPtr => true,
		}
	}

	pub fn is_by_void_ptr(&self) -> bool {
		match self {
			ExternPassKind::AsIs | ExternPassKind::ByPtr => false,
			ExternPassKind::ByVoidPtr => true,
		}
	}
}
