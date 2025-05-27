use crate::type_ref::{TypeRefKind, TypeRefTypeHint};

pub trait TypeRefKindExt {
	/// Returns what kind of borrow is possible from the current [TypeRefKind]
	///
	/// Used mainly to decide whether the return type of function should have explicit or elided lifetime.
	fn rust_borrow_kind(&self, type_hint: &TypeRefTypeHint) -> BorrowKind;
}

impl TypeRefKindExt for TypeRefKind<'_, '_> {
	fn rust_borrow_kind(&self, type_hint: &TypeRefTypeHint) -> BorrowKind {
		match self {
			TypeRefKind::Array(elem, _) => elem.kind().rust_borrow_kind(type_hint),
			TypeRefKind::StdVector(vec) => vec.element_type().kind().rust_borrow_kind(type_hint),
			TypeRefKind::StdTuple(tup) => tup.elements().iter().fold(BorrowKind::Impossible, |a, e| {
				a.more_complicated(e.kind().rust_borrow_kind(type_hint))
			}),
			TypeRefKind::RValueReference(inner) => inner.kind().rust_borrow_kind(type_hint),
			TypeRefKind::SmartPtr(ptr) => ptr.pointee().kind().rust_borrow_kind(type_hint),
			TypeRefKind::Typedef(tdef) => tdef.underlying_type_ref().kind().rust_borrow_kind(type_hint),
			TypeRefKind::Class(cls) => {
				let type_ref = cls.type_ref();
				if !type_ref.kind().is_copy(type_ref.type_hint()) {
					BorrowKind::FromLifetime
				} else {
					BorrowKind::Impossible
				}
			}
			TypeRefKind::Pointer(_) if self.is_rust_by_ptr(type_hint) => BorrowKind::FromPointer,
			TypeRefKind::Pointer(_) | TypeRefKind::Reference(_) => BorrowKind::FromLifetime,
			TypeRefKind::Primitive(_, _)
			| TypeRefKind::Enum(_)
			| TypeRefKind::Function(_)
			| TypeRefKind::Generic(_)
			| TypeRefKind::Ignored => BorrowKind::Impossible,
		}
	}
}

pub enum BorrowKind {
	/// A borrow from the data passed as a Rust pointer is possible
	FromPointer,
	/// A borrow from a reference or a struct with a lifetime is possible
	FromLifetime,
	/// No borrow possible
	Impossible,
}

impl BorrowKind {
	/// Returns a variante that requires a more complicated handling, i.e., in the following order from the higher complication to the lower:
	/// * [Self::FromPointer]
	/// * [Self::FromLifetime]
	/// * [Self::Impossible]
	pub fn more_complicated(self, other: Self) -> Self {
		match (self, other) {
			(BorrowKind::FromPointer, _) | (_, BorrowKind::FromPointer) => BorrowKind::FromPointer,
			(BorrowKind::FromLifetime, _) | (_, BorrowKind::FromLifetime) => BorrowKind::FromLifetime,
			(BorrowKind::Impossible, _) => BorrowKind::Impossible,
		}
	}
}
