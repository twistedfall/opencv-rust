{{doc_comment}}
{{debug}}
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct {{rust_local}} {
	{{fields}}
}

opencv_type_simple! { {{rust_full}} }

{{impl}}
{{impls}}

