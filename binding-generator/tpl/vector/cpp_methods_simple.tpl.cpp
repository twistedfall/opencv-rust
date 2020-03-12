Result<{{inner_cpp_extern}}> cv_{{rust_local}}_get_unchecked(const {{cpp_full}}* instance, size_t index) {
	return Ok((*instance)[index]);
}


