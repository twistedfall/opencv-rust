{{doc_comment}}
{{debug}}
#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum {{rust_decl}} {
	{{enum_consts}}
}

opencv_type_enum! { {{rust_ref}} { {{consts_list}} } }


