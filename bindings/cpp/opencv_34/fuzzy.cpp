#include "common.hpp"
#include <opencv2/fuzzy.hpp>
#include "fuzzy_types.hpp"

extern "C" {
	Result_void cv_ft_FT02D_FL_process_const__InputArrayX_int_const__OutputArrayX(const cv::_InputArray* matrix, int radius, const cv::_OutputArray* output) {
		try {
			cv::ft::FT02D_FL_process(*matrix, radius, *output);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ft_FT02D_FL_process_float_const__InputArrayX_int_const__OutputArrayX(const cv::_InputArray* matrix, int radius, const cv::_OutputArray* output) {
		try {
			cv::ft::FT02D_FL_process_float(*matrix, radius, *output);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ft_FT02D_components_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__InputArrayX(const cv::_InputArray* matrix, const cv::_InputArray* kernel, const cv::_OutputArray* components, const cv::_InputArray* mask) {
		try {
			cv::ft::FT02D_components(*matrix, *kernel, *components, *mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ft_FT02D_inverseFT_const__InputArrayX_const__InputArrayX_const__OutputArrayX_int_int(const cv::_InputArray* components, const cv::_InputArray* kernel, const cv::_OutputArray* output, int width, int height) {
		try {
			cv::ft::FT02D_inverseFT(*components, *kernel, *output, width, height);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ft_FT02D_iteration_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__InputArrayX_const__OutputArrayX_bool(const cv::_InputArray* matrix, const cv::_InputArray* kernel, const cv::_OutputArray* output, const cv::_InputArray* mask, const cv::_OutputArray* maskOutput, bool firstStop) {
		try {
			int ret = cv::ft::FT02D_iteration(*matrix, *kernel, *output, *mask, *maskOutput, firstStop);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ft_FT02D_process_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__InputArrayX(const cv::_InputArray* matrix, const cv::_InputArray* kernel, const cv::_OutputArray* output, const cv::_InputArray* mask) {
		try {
			cv::ft::FT02D_process(*matrix, *kernel, *output, *mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ft_FT12D_components_const__InputArrayX_const__InputArrayX_const__OutputArrayX(const cv::_InputArray* matrix, const cv::_InputArray* kernel, const cv::_OutputArray* components) {
		try {
			cv::ft::FT12D_components(*matrix, *kernel, *components);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ft_FT12D_createPolynomMatrixHorizontal_int_const__OutputArrayX_int(int radius, const cv::_OutputArray* matrix, int chn) {
		try {
			cv::ft::FT12D_createPolynomMatrixHorizontal(radius, *matrix, chn);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ft_FT12D_createPolynomMatrixVertical_int_const__OutputArrayX_int(int radius, const cv::_OutputArray* matrix, int chn) {
		try {
			cv::ft::FT12D_createPolynomMatrixVertical(radius, *matrix, chn);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ft_FT12D_inverseFT_const__InputArrayX_const__InputArrayX_const__OutputArrayX_int_int(const cv::_InputArray* components, const cv::_InputArray* kernel, const cv::_OutputArray* output, int width, int height) {
		try {
			cv::ft::FT12D_inverseFT(*components, *kernel, *output, width, height);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ft_FT12D_polynomial_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__InputArrayX(const cv::_InputArray* matrix, const cv::_InputArray* kernel, const cv::_OutputArray* c00, const cv::_OutputArray* c10, const cv::_OutputArray* c01, const cv::_OutputArray* components, const cv::_InputArray* mask) {
		try {
			cv::ft::FT12D_polynomial(*matrix, *kernel, *c00, *c10, *c01, *components, *mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ft_FT12D_process_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__InputArrayX(const cv::_InputArray* matrix, const cv::_InputArray* kernel, const cv::_OutputArray* output, const cv::_InputArray* mask) {
		try {
			cv::ft::FT12D_process(*matrix, *kernel, *output, *mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ft_createKernel_const__InputArrayX_const__InputArrayX_const__OutputArrayX_int(const cv::_InputArray* A, const cv::_InputArray* B, const cv::_OutputArray* kernel, int chn) {
		try {
			cv::ft::createKernel(*A, *B, *kernel, chn);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ft_createKernel_int_int_const__OutputArrayX_int(int function, int radius, const cv::_OutputArray* kernel, int chn) {
		try {
			cv::ft::createKernel(function, radius, *kernel, chn);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ft_filter_const__InputArrayX_const__InputArrayX_const__OutputArrayX(const cv::_InputArray* image, const cv::_InputArray* kernel, const cv::_OutputArray* output) {
		try {
			cv::ft::filter(*image, *kernel, *output);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ft_inpaint_const__InputArrayX_const__InputArrayX_const__OutputArrayX_int_int_int(const cv::_InputArray* image, const cv::_InputArray* mask, const cv::_OutputArray* output, int radius, int function, int algorithm) {
		try {
			cv::ft::inpaint(*image, *mask, *output, radius, function, algorithm);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
}
