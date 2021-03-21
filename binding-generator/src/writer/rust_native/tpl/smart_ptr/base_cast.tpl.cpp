

cv::Ptr<{{base_cpp_full}}>* cv_{{rust_localalias}}_to_PtrOf{{base_rust_local}}({{cpp_decl}}) {
	return new cv::Ptr<{{base_cpp_full}}>(instance->dynamicCast<{{base_cpp_full}}>());
}

