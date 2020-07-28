{{attributes_begin}}
{{debug}}
{{return_wrapper_type}} {{identifier}}({{decl_args}}) {
	try {
		{{pre_call_args}}
		{{call}}
		{{post_call_args}}
		{{cleanup_args}}
		{{return}}
	} OCVRS_CATCH(OCVRS_TYPE({{return_wrapper_type}}))
}
{{attributes_end}}


