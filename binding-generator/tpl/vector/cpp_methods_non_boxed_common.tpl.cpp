void cv_{{rust_local}}_push({{cpp_full}}* instance, {{inner_cpp_full}} val) {
	instance->push_back(val);
}

void cv_{{rust_local}}_insert({{cpp_full}}* instance, size_t index, {{inner_cpp_full}} val) {
	instance->insert(instance->begin() + index, val);
}

{{cpp_return_wrapper_type}} cv_{{rust_local}}_get(const {{cpp_full}}* instance, size_t index) {
	try {
		return Ok<{{inner_cpp_extern}}>(instance->at(index));
	} VEC_CATCH({{cpp_return_wrapper_type}})
}

Result_void cv_{{rust_local}}_set({{cpp_full}}* instance, size_t index, {{inner_cpp_full}} val) {
	try {
		instance->at(index) = val;
		return Ok();
	} VEC_CATCH(Result_void)
}

void cv_{{rust_local}}_set_unchecked({{cpp_full}}* instance, size_t index, {{inner_cpp_full}} val) {
	(*instance)[index] = val;
}


