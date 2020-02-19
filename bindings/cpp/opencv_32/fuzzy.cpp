#include "common.hpp"
#include <opencv2/fuzzy.hpp>
#include "fuzzy_types.hpp"

extern "C" {
	Result_void cv_ft_FT02D_components_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* matrix, void* kernel, void* components) {
		try {
			cv::ft::FT02D_components(*reinterpret_cast<const cv::_InputArray*>(matrix), *reinterpret_cast<const cv::_InputArray*>(kernel), *reinterpret_cast<const cv::_OutputArray*>(components));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ft_FT02D_components_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__InputArrayX(void* matrix, void* kernel, void* components, void* mask) {
		try {
			cv::ft::FT02D_components(*reinterpret_cast<const cv::_InputArray*>(matrix), *reinterpret_cast<const cv::_InputArray*>(kernel), *reinterpret_cast<const cv::_OutputArray*>(components), *reinterpret_cast<const cv::_InputArray*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ft_FT02D_inverseFT_const__InputArrayX_const__InputArrayX_const__OutputArrayX_int_int(void* components, void* kernel, void* output, int width, int height) {
		try {
			cv::ft::FT02D_inverseFT(*reinterpret_cast<const cv::_InputArray*>(components), *reinterpret_cast<const cv::_InputArray*>(kernel), *reinterpret_cast<const cv::_OutputArray*>(output), width, height);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ft_FT02D_iteration_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__InputArrayX_const__OutputArrayX_bool(void* matrix, void* kernel, void* output, void* mask, void* maskOutput, bool firstStop) {
		try {
			int ret = cv::ft::FT02D_iteration(*reinterpret_cast<const cv::_InputArray*>(matrix), *reinterpret_cast<const cv::_InputArray*>(kernel), *reinterpret_cast<const cv::_OutputArray*>(output), *reinterpret_cast<const cv::_InputArray*>(mask), *reinterpret_cast<const cv::_OutputArray*>(maskOutput), firstStop);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ft_FT02D_process_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* matrix, void* kernel, void* output) {
		try {
			cv::ft::FT02D_process(*reinterpret_cast<const cv::_InputArray*>(matrix), *reinterpret_cast<const cv::_InputArray*>(kernel), *reinterpret_cast<const cv::_OutputArray*>(output));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ft_FT02D_process_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__InputArrayX(void* matrix, void* kernel, void* output, void* mask) {
		try {
			cv::ft::FT02D_process(*reinterpret_cast<const cv::_InputArray*>(matrix), *reinterpret_cast<const cv::_InputArray*>(kernel), *reinterpret_cast<const cv::_OutputArray*>(output), *reinterpret_cast<const cv::_InputArray*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ft_createKernel_const__InputArrayX_const__InputArrayX_const__OutputArrayX_int(void* A, void* B, void* kernel, int chn) {
		try {
			cv::ft::createKernel(*reinterpret_cast<const cv::_InputArray*>(A), *reinterpret_cast<const cv::_InputArray*>(B), *reinterpret_cast<const cv::_OutputArray*>(kernel), chn);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ft_createKernel_int_int_const__OutputArrayX_int(int function, int radius, void* kernel, int chn) {
		try {
			cv::ft::createKernel(function, radius, *reinterpret_cast<const cv::_OutputArray*>(kernel), chn);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ft_filter_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* image, void* kernel, void* output) {
		try {
			cv::ft::filter(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_InputArray*>(kernel), *reinterpret_cast<const cv::_OutputArray*>(output));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ft_inpaint_const__InputArrayX_const__InputArrayX_const__OutputArrayX_int_int_int(void* image, void* mask, void* output, int radius, int function, int algorithm) {
		try {
			cv::ft::inpaint(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_InputArray*>(mask), *reinterpret_cast<const cv::_OutputArray*>(output), radius, function, algorithm);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
}
