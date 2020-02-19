void cv_{{rust_local}}_push({{cpp_full}}* instance, {{inner_cpp_extern}} val) {
	instance->push_back({{call_arg}});
}

void cv_{{rust_local}}_insert({{cpp_full}}* instance, size_t index, {{inner_cpp_extern}} val) {
	instance->insert(instance->begin() + index, {{call_arg}});
}

Result<const char*> cv_{{rust_local}}_get(const {{cpp_full}}* instance, size_t index) {
	try {
		return Ok<const char*>(instance->at(index).c_str());
	} VEC_CATCH(Result<const char*>)
}

const char* cv_{{rust_local}}_get_unchecked(const {{cpp_full}}* instance, size_t index) {
	return (*instance)[index].c_str();
}

Result_void cv_{{rust_local}}_set({{cpp_full}}* instance, size_t index, {{inner_cpp_extern}} val) {
	try {
		instance->at(index) = {{call_arg}};
		return Ok();
	} VEC_CATCH(Result_void)
}

void cv_{{rust_local}}_set_unchecked({{cpp_full}}* instance, size_t index, {{inner_cpp_extern}} val) {
	(*instance)[index] = {{call_arg}};
}


