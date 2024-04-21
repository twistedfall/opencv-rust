{{doc_comment}}
{{debug}}
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum {{rust_local}} {
	{{enum_consts}}
}

impl TryFrom<i32> for {{rust_local}} {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			{{from_consts}}
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: {{rust_full}}"))),
		}
	}
}

opencv_type_enum! { {{rust_full}} }


