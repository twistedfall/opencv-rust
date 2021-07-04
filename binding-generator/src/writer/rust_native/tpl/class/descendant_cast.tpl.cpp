 Result<{{descendant_cpp_full}}*> cv_{{rust_local}}_to_{{descendant_rust_local}}({{cpp_decl}}) {
	try {
		return Ok(dynamic_cast<{{descendant_cpp_full}}*>(instance));
	} OCVRS_CATCH(Result<{{descendant_cpp_full}}*>)
}


