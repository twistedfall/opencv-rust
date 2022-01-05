void cv_{{rust_localalias}}_input_array({{cpp_full}}* instance, Result<cv::_InputArray*>* ocvrs_return) {
	try {
		Ok(new cv::_InputArray(*instance), ocvrs_return);
	} OCVRS_CATCH(Result<cv::_InputArray*>)
}

void cv_{{rust_localalias}}_output_array({{cpp_full}}* instance, Result<cv::_OutputArray*>* ocvrs_return) {
	try {
		Ok(new cv::_OutputArray(*instance), ocvrs_return);
	} OCVRS_CATCH(Result<cv::_OutputArray*>)
}

void cv_{{rust_localalias}}_input_output_array({{cpp_full}}* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
	try {
		Ok(new cv::_InputOutputArray(*instance), ocvrs_return);
	} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
}


