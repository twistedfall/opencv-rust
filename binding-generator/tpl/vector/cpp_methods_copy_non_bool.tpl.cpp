const {{cpp_extern}}* cv_{{rust_local}}_data({{cpp_full}}* instance) {
	return reinterpret_cast<const {{cpp_extern}}*>(instance->data());
}


