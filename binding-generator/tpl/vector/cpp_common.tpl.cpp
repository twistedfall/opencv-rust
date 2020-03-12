extern "C" {
	void cv_{{rust_local}}_delete({{cpp_full}}* instance) {
		delete instance;
	}

	{{cpp_extern_return}} cv_{{rust_local}}_new() {
		return new {{cpp_full}}();
	}

	size_t cv_{{rust_local}}_len(const {{cpp_full}}* instance) {
		return instance->size();
	}

	bool cv_{{rust_local}}_is_empty(const {{cpp_full}}* instance) {
		return instance->empty();
	}

	size_t cv_{{rust_local}}_capacity(const {{cpp_full}}* instance) {
		return instance->capacity();
	}

	void cv_{{rust_local}}_shrink_to_fit({{cpp_full}}* instance) {
		instance->shrink_to_fit();
	}

	void cv_{{rust_local}}_reserve({{cpp_full}}* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_{{rust_local}}_remove({{cpp_full}}* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_{{rust_local}}_swap({{cpp_full}}* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_{{rust_local}}_clear({{cpp_full}}* instance) {
		instance->clear();
	}

	{{exports}}
}



