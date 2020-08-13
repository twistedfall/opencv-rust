extern "C" {
	{{ctor}}
	void cv_{{rust_localalias}}_delete({{cpp_full}}* instance) {
		delete instance;
	}

	{{inner_cpp_extern_const}} cv_{{rust_localalias}}_get_inner_ptr({{cpp_full_const}}* instance) {
		return instance->get();
	}

	{{inner_cpp_extern}} cv_{{rust_localalias}}_get_inner_ptr_mut({{cpp_full}}* instance) {
		return instance->get();
	}
}


