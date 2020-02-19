Result<void*> cv_{{rust_local}}_input_array({{cpp_full}}* instance) {
	try {
		return Ok<void*>(new cv::_InputArray(*instance));
	} OCVRS_CATCH(Result<void*>)
}

Result<void*> cv_{{rust_local}}_output_array({{cpp_full}}* instance) {
	try {
		return Ok<void*>(new cv::_OutputArray(*instance));
	} OCVRS_CATCH(Result<void*>)
}

Result<void*> cv_{{rust_local}}_input_output_array({{cpp_full}}* instance) {
	try {
		return Ok<void*>(new cv::_InputOutputArray(*instance));
	} OCVRS_CATCH(Result<void*>)
}


