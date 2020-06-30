const {{inner_cpp_extern_return}}* cv_{{rust_localalias}}_data(const {{cpp_full}}* instance) {
	return instance->data();
}

{{inner_cpp_extern_return}}* cv_{{rust_localalias}}_data_mut({{cpp_full}}* instance) {
	return instance->data();
}

	{{cpp_extern_return}} cv_{{rust_localalias}}_clone(const {{cpp_full}}* instance) {
		return new {{cpp_full}}(*instance);
	}


