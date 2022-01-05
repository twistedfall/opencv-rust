{{doc_comment}}
{{debug}}
#[inline]
{{attributes}}
{{visibility}}{{unsafety_decl}}fn {{name}}{{generic_decl}}({{decl_args}}){{rv_rust_full}} {
	{{pre_call_args}}
	{{ret_receive}}{{unsafety_call}}{ sys::{{identifier}}({{call_args}}) };
	{{ret_convert}}
	{{post_call_args}}
}


