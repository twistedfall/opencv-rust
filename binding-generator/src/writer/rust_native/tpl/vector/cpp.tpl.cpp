extern "C" {
	void cv_{{rust_localalias}}_delete({{cpp_full}}* instance) {
		delete instance;
	}

	{{cpp_extern_return}} cv_{{rust_localalias}}_new() {
		return new {{cpp_full}}();
	}

	size_t cv_{{rust_localalias}}_len(const {{cpp_full}}* instance) {
		return instance->size();
	}

	bool cv_{{rust_localalias}}_is_empty(const {{cpp_full}}* instance) {
		return instance->empty();
	}

	size_t cv_{{rust_localalias}}_capacity(const {{cpp_full}}* instance) {
		return instance->capacity();
	}

	void cv_{{rust_localalias}}_shrink_to_fit({{cpp_full}}* instance) {
		instance->shrink_to_fit();
	}

	void cv_{{rust_localalias}}_reserve({{cpp_full}}* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_{{rust_localalias}}_remove({{cpp_full}}* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_{{rust_localalias}}_swap({{cpp_full}}* instance, size_t index1, size_t index2) {
		{{swap_func}}((*instance)[index1], (*instance)[index2]);
	}

	void cv_{{rust_localalias}}_clear({{cpp_full}}* instance) {
		instance->clear();
	}

	void cv_{{rust_localalias}}_push({{cpp_full}}* instance, {{inner_cpp_func_decl}}) {
		instance->push_back({{inner_cpp_func_call}});
	}

	void cv_{{rust_localalias}}_insert({{cpp_full}}* instance, size_t index, {{inner_cpp_func_decl}}) {
		instance->insert(instance->begin() + index, {{inner_cpp_func_call}});
	}

	void cv_{{rust_localalias}}_get(const {{cpp_full}}* instance, size_t index, {{inner_cpp_extern_return}}* ocvrs_return) {
		*ocvrs_return = {{prefix}}(*instance)[index]{{suffix}};
	}

	void cv_{{rust_localalias}}_set({{cpp_full}}* instance, size_t index, {{inner_cpp_func_decl}}) {
		(*instance)[index] = {{inner_cpp_func_call}};
	}

	{{exports}}
}



