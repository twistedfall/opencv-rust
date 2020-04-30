Result<cv::_InputArray*> cv_{{rust_localalias}}_input_array({{cpp_full}}* instance) {
	try {
		return Ok(new cv::_InputArray(*instance));
	} OCVRS_CATCH(Result<cv::_InputArray*>)
}

Result<cv::_OutputArray*> cv_{{rust_localalias}}_output_array({{cpp_full}}* instance) {
	try {
		return Ok(new cv::_OutputArray(*instance));
	} OCVRS_CATCH(Result<cv::_OutputArray*>)
}

Result<cv::_InputOutputArray*> cv_{{rust_localalias}}_input_output_array({{cpp_full}}* instance) {
	try {
		return Ok(new cv::_InputOutputArray(*instance));
	} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
}


