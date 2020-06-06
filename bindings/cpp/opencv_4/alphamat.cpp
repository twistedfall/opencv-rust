#include "common.hpp"
#include <opencv2/alphamat.hpp>
#include "alphamat_types.hpp"

extern "C" {
	Result_void cv_alphamat_infoFlow_const__InputArrayX_const__InputArrayX_const__OutputArrayX(const cv::_InputArray* image, const cv::_InputArray* tmap, const cv::_OutputArray* result) {
		try {
			cv::alphamat::infoFlow(*image, *tmap, *result);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
