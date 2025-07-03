{{doc_comment}}
{{debug}}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct {{rust_decl}}(i32);

impl {{rust_decl}} {
	{{enum_consts}}
}

opencv_type_bitfield_enum! { {{rust_ref}} { {{consts_list}} } }


