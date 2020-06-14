extern "C" void cv_{{rust_localalias}}_delete({{cpp_full}}* instance) {
	delete instance;
}

extern "C" {{inner_cpp_extern}} cv_{{rust_localalias}}_get_inner_ptr({{cpp_full}}* instance) {
	return instance->get();
}


