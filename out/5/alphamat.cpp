#include "alphamat.hpp"
#include "alphamat_types.hpp"

extern "C" {
	// infoFlow(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/alphamat.hpp:38
	// ("cv::alphamat::infoFlow", vec![(pred!(mut, ["image", "tmap", "result"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_alphamat_infoFlow_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* image, const cv::_InputArray* tmap, const cv::_OutputArray* result, ResultVoid* ocvrs_return) {
		try {
			cv::alphamat::infoFlow(*image, *tmap, *result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}
