extern "C" {
	{{ctor}}
	void cv_{{rust_localalias}}_delete({{cpp_full}}* instance) {
		delete instance;
	}

	{{inner_cpp_extern}} cv_{{rust_localalias}}_get_inner_ptr({{cpp_full}}* instance) {
		return instance->get();
	}
}


