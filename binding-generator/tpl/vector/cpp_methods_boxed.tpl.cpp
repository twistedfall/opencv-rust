void cv_{{rust_local}}_push({{cpp_full}}* instance, {{inner_cpp_full}}* val) {
	instance->push_back(*val);
}

void cv_{{rust_local}}_insert({{cpp_full}}* instance, size_t index, {{inner_cpp_full}}* val) {
	instance->insert(instance->begin() + index, *val);
}

{{cpp_return_wrapper_type}} cv_{{rust_local}}_get(const {{cpp_full}}* instance, size_t index) {
	try {
		return Ok(new {{inner_cpp_full}}(instance->at(index)));
	} VEC_CATCH({{cpp_return_wrapper_type}})
}

{{inner_cpp_extern_return}} cv_{{rust_local}}_get_unchecked(const {{cpp_full}}* instance, size_t index) {
	return new {{inner_cpp_full}}((*instance)[index]);
}

Result_void cv_{{rust_local}}_set({{cpp_full}}* instance, size_t index, {{inner_cpp_full}}* val) {
	try {
		instance->at(index) = *val;
		return Ok();
	} VEC_CATCH(Result_void)
}

void cv_{{rust_local}}_set_unchecked({{cpp_full}}* instance, size_t index, {{inner_cpp_full}}* val) {
	(*instance)[index] = *val;
}


