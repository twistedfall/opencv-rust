{{attributes_begin}}
{{debug}}
{{return_wrapper_type}} {{identifier}}({{decl_args}}) {
	try {
		{{pre_call_args}}
		{{call}}
		{{post_call_args}}
		{{return}}
	} OCVRS_CATCH({{return_wrapper_type}})
}
{{attributes_end}}


