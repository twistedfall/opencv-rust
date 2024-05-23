#include "ocvrs_common.hpp"
#include <opencv2/ximgproc.hpp>
#include "ximgproc_types.hpp"

extern "C" {
	// cv::ximgproc::BrightEdges(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/brightedges.hpp:48
	// ("cv::ximgproc::BrightEdges", vec![(pred!(mut, ["_original", "_edgeview"], ["cv::Mat*", "cv::Mat*"]), _)]),
	void cv_ximgproc_BrightEdges_MatR_MatR(cv::Mat* _original, cv::Mat* _edgeview, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::BrightEdges(*_original, *_edgeview);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// BrightEdges(Mat &, Mat &, int, int, int)(TraitClass, TraitClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/brightedges.hpp:48
	// ("cv::ximgproc::BrightEdges", vec![(pred!(mut, ["_original", "_edgeview", "contrast", "shortrange", "longrange"], ["cv::Mat*", "cv::Mat*", "int", "int", "int"]), _)]),
	void cv_ximgproc_BrightEdges_MatR_MatR_int_int_int(cv::Mat* _original, cv::Mat* _edgeview, int contrast, int shortrange, int longrange, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::BrightEdges(*_original, *_edgeview, contrast, shortrange, longrange);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::FastHoughTransform(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/fast_hough_transform.hpp:132
	// ("cv::ximgproc::FastHoughTransform", vec![(pred!(mut, ["src", "dst", "dstMatDepth"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_ximgproc_FastHoughTransform_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int dstMatDepth, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::FastHoughTransform(*src, *dst, dstMatDepth);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// FastHoughTransform(InputArray, OutputArray, int, int, int, int)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/fast_hough_transform.hpp:132
	// ("cv::ximgproc::FastHoughTransform", vec![(pred!(mut, ["src", "dst", "dstMatDepth", "angleRange", "op", "makeSkew"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int", "int"]), _)]),
	void cv_ximgproc_FastHoughTransform_const__InputArrayR_const__OutputArrayR_int_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int dstMatDepth, int angleRange, int op, int makeSkew, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::FastHoughTransform(*src, *dst, dstMatDepth, angleRange, op, makeSkew);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GradientDericheX(InputArray, OutputArray, double, double)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/deriche_filter.hpp:72
	// ("cv::ximgproc::GradientDericheX", vec![(pred!(mut, ["op", "dst", "alpha", "omega"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
	void cv_ximgproc_GradientDericheX_const__InputArrayR_const__OutputArrayR_double_double(const cv::_InputArray* op, const cv::_OutputArray* dst, double alpha, double omega, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::GradientDericheX(*op, *dst, alpha, omega);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GradientDericheY(InputArray, OutputArray, double, double)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/deriche_filter.hpp:60
	// ("cv::ximgproc::GradientDericheY", vec![(pred!(mut, ["op", "dst", "alpha", "omega"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
	void cv_ximgproc_GradientDericheY_const__InputArrayR_const__OutputArrayR_double_double(const cv::_InputArray* op, const cv::_OutputArray* dst, double alpha, double omega, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::GradientDericheY(*op, *dst, alpha, omega);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GradientPaillouX(InputArray, OutputArray, double, double)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/paillou_filter.hpp:62
	// ("cv::ximgproc::GradientPaillouX", vec![(pred!(mut, ["op", "_dst", "alpha", "omega"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
	void cv_ximgproc_GradientPaillouX_const__InputArrayR_const__OutputArrayR_double_double(const cv::_InputArray* op, const cv::_OutputArray* _dst, double alpha, double omega, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::GradientPaillouX(*op, *_dst, alpha, omega);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GradientPaillouY(InputArray, OutputArray, double, double)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/paillou_filter.hpp:61
	// ("cv::ximgproc::GradientPaillouY", vec![(pred!(mut, ["op", "_dst", "alpha", "omega"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
	void cv_ximgproc_GradientPaillouY_const__InputArrayR_const__OutputArrayR_double_double(const cv::_InputArray* op, const cv::_OutputArray* _dst, double alpha, double omega, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::GradientPaillouY(*op, *_dst, alpha, omega);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::HoughPoint2Line(SimpleClass, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/fast_hough_transform.hpp:154
	// ("cv::ximgproc::HoughPoint2Line", vec![(pred!(mut, ["houghPoint", "srcImgInfo"], ["const cv::Point*", "const cv::_InputArray*"]), _)]),
	void cv_ximgproc_HoughPoint2Line_const_PointR_const__InputArrayR(const cv::Point* houghPoint, const cv::_InputArray* srcImgInfo, Result<cv::Vec4i>* ocvrs_return) {
		try {
			cv::Vec4i ret = cv::ximgproc::HoughPoint2Line(*houghPoint, *srcImgInfo);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// HoughPoint2Line(const Point &, InputArray, int, int, int)(SimpleClass, InputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/fast_hough_transform.hpp:154
	// ("cv::ximgproc::HoughPoint2Line", vec![(pred!(mut, ["houghPoint", "srcImgInfo", "angleRange", "makeSkew", "rules"], ["const cv::Point*", "const cv::_InputArray*", "int", "int", "int"]), _)]),
	void cv_ximgproc_HoughPoint2Line_const_PointR_const__InputArrayR_int_int_int(const cv::Point* houghPoint, const cv::_InputArray* srcImgInfo, int angleRange, int makeSkew, int rules, Result<cv::Vec4i>* ocvrs_return) {
		try {
			cv::Vec4i ret = cv::ximgproc::HoughPoint2Line(*houghPoint, *srcImgInfo, angleRange, makeSkew, rules);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// PeiLinNormalization(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/peilin.hpp:26
	// ("cv::ximgproc::PeiLinNormalization", vec![(pred!(mut, ["I"], ["const cv::_InputArray*"]), _)]),
	void cv_ximgproc_PeiLinNormalization_const__InputArrayR(const cv::_InputArray* I, Result<cv::Matx23d>* ocvrs_return) {
		try {
			cv::Matx23d ret = cv::ximgproc::PeiLinNormalization(*I);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// PeiLinNormalization(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/peilin.hpp:28
	// ("cv::ximgproc::PeiLinNormalization", vec![(pred!(mut, ["I", "T"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_PeiLinNormalization_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* I, const cv::_OutputArray* T, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::PeiLinNormalization(*I, *T);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::RadonTransform(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/radon_transform.hpp:31
	// ("cv::ximgproc::RadonTransform", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_RadonTransform_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::RadonTransform(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// RadonTransform(InputArray, OutputArray, double, double, double, bool, bool)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/radon_transform.hpp:31
	// ("cv::ximgproc::RadonTransform", vec![(pred!(mut, ["src", "dst", "theta", "start_angle", "end_angle", "crop", "norm"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "double", "bool", "bool"]), _)]),
	void cv_ximgproc_RadonTransform_const__InputArrayR_const__OutputArrayR_double_double_double_bool_bool(const cv::_InputArray* src, const cv::_OutputArray* dst, double theta, double start_angle, double end_angle, bool crop, bool norm, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::RadonTransform(*src, *dst, theta, start_angle, end_angle, crop, norm);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::amFilter(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:290
	// ("cv::ximgproc::amFilter", vec![(pred!(mut, ["joint", "src", "dst", "sigma_s", "sigma_r"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
	void cv_ximgproc_amFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double(const cv::_InputArray* joint, const cv::_InputArray* src, const cv::_OutputArray* dst, double sigma_s, double sigma_r, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::amFilter(*joint, *src, *dst, sigma_s, sigma_r);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// amFilter(InputArray, InputArray, OutputArray, double, double, bool)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:290
	// ("cv::ximgproc::amFilter", vec![(pred!(mut, ["joint", "src", "dst", "sigma_s", "sigma_r", "adjust_outliers"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "bool"]), _)]),
	void cv_ximgproc_amFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double_bool(const cv::_InputArray* joint, const cv::_InputArray* src, const cv::_OutputArray* dst, double sigma_s, double sigma_r, bool adjust_outliers, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::amFilter(*joint, *src, *dst, sigma_s, sigma_r, adjust_outliers);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// anisotropicDiffusion(InputArray, OutputArray, float, float, int)(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc.hpp:212
	// ("cv::ximgproc::anisotropicDiffusion", vec![(pred!(mut, ["src", "dst", "alpha", "K", "niters"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float", "float", "int"]), _)]),
	void cv_ximgproc_anisotropicDiffusion_const__InputArrayR_const__OutputArrayR_float_float_int(const cv::_InputArray* src, const cv::_OutputArray* dst, float alpha, float K, int niters, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::anisotropicDiffusion(*src, *dst, alpha, K, niters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::bilateralTextureFilter(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:345
	// ("cv::ximgproc::bilateralTextureFilter", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_bilateralTextureFilter_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::bilateralTextureFilter(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bilateralTextureFilter(InputArray, OutputArray, int, int, double, double)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:345
	// ("cv::ximgproc::bilateralTextureFilter", vec![(pred!(mut, ["src", "dst", "fr", "numIter", "sigmaAlpha", "sigmaAvg"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "double", "double"]), _)]),
	void cv_ximgproc_bilateralTextureFilter_const__InputArrayR_const__OutputArrayR_int_int_double_double(const cv::_InputArray* src, const cv::_OutputArray* dst, int fr, int numIter, double sigmaAlpha, double sigmaAvg, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::bilateralTextureFilter(*src, *dst, fr, numIter, sigmaAlpha, sigmaAvg);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// colorMatchTemplate(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/color_match.hpp:62
	// ("cv::ximgproc::colorMatchTemplate", vec![(pred!(mut, ["img", "templ", "result"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_colorMatchTemplate_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* img, const cv::_InputArray* templ, const cv::_OutputArray* result, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::colorMatchTemplate(*img, *templ, *result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::computeBadPixelPercent(InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/disparity_filter.hpp:193
	// ("cv::ximgproc::computeBadPixelPercent", vec![(pred!(mut, ["GT", "src", "ROI"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Rect"]), _)]),
	void cv_ximgproc_computeBadPixelPercent_const__InputArrayR_const__InputArrayR_Rect(const cv::_InputArray* GT, const cv::_InputArray* src, cv::Rect* ROI, Result<double>* ocvrs_return) {
		try {
			double ret = cv::ximgproc::computeBadPixelPercent(*GT, *src, *ROI);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeBadPixelPercent(InputArray, InputArray, Rect, int)(InputArray, InputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/disparity_filter.hpp:193
	// ("cv::ximgproc::computeBadPixelPercent", vec![(pred!(mut, ["GT", "src", "ROI", "thresh"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Rect", "int"]), _)]),
	void cv_ximgproc_computeBadPixelPercent_const__InputArrayR_const__InputArrayR_Rect_int(const cv::_InputArray* GT, const cv::_InputArray* src, cv::Rect* ROI, int thresh, Result<double>* ocvrs_return) {
		try {
			double ret = cv::ximgproc::computeBadPixelPercent(*GT, *src, *ROI, thresh);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeMSE(InputArray, InputArray, Rect)(InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/disparity_filter.hpp:177
	// ("cv::ximgproc::computeMSE", vec![(pred!(mut, ["GT", "src", "ROI"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Rect"]), _)]),
	void cv_ximgproc_computeMSE_const__InputArrayR_const__InputArrayR_Rect(const cv::_InputArray* GT, const cv::_InputArray* src, cv::Rect* ROI, Result<double>* ocvrs_return) {
		try {
			double ret = cv::ximgproc::computeMSE(*GT, *src, *ROI);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// contourSampling(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/fourier_descriptors.hpp:106
	// ("cv::ximgproc::contourSampling", vec![(pred!(mut, ["src", "out", "nbElt"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_ximgproc_contourSampling_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* out, int nbElt, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::contourSampling(*src, *out, nbElt);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// covarianceEstimation(InputArray, OutputArray, int, int)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/estimated_covariance.hpp:77
	// ("cv::ximgproc::covarianceEstimation", vec![(pred!(mut, ["src", "dst", "windowRows", "windowCols"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_ximgproc_covarianceEstimation_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int windowRows, int windowCols, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::covarianceEstimation(*src, *dst, windowRows, windowCols);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::createAMFilter(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:268
	// ("cv::ximgproc::createAMFilter", vec![(pred!(mut, ["sigma_s", "sigma_r"], ["double", "double"]), _)]),
	void cv_ximgproc_createAMFilter_double_double(double sigma_s, double sigma_r, Result<cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter> ret = cv::ximgproc::createAMFilter(sigma_s, sigma_r);
			Ok(new cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createAMFilter(double, double, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:268
	// ("cv::ximgproc::createAMFilter", vec![(pred!(mut, ["sigma_s", "sigma_r", "adjust_outliers"], ["double", "double", "bool"]), _)]),
	void cv_ximgproc_createAMFilter_double_double_bool(double sigma_s, double sigma_r, bool adjust_outliers, Result<cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter> ret = cv::ximgproc::createAMFilter(sigma_s, sigma_r, adjust_outliers);
			Ok(new cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::createContourFitting() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/fourier_descriptors.hpp:114
	// ("cv::ximgproc::createContourFitting", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_createContourFitting(Result<cv::Ptr<cv::ximgproc::ContourFitting>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::ContourFitting> ret = cv::ximgproc::createContourFitting();
			Ok(new cv::Ptr<cv::ximgproc::ContourFitting>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createContourFitting(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/fourier_descriptors.hpp:114
	// ("cv::ximgproc::createContourFitting", vec![(pred!(mut, ["ctr", "fd"], ["int", "int"]), _)]),
	void cv_ximgproc_createContourFitting_int_int(int ctr, int fd, Result<cv::Ptr<cv::ximgproc::ContourFitting>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::ContourFitting> ret = cv::ximgproc::createContourFitting(ctr, fd);
			Ok(new cv::Ptr<cv::ximgproc::ContourFitting>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::createDTFilter(InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:102
	// ("cv::ximgproc::createDTFilter", vec![(pred!(mut, ["guide", "sigmaSpatial", "sigmaColor"], ["const cv::_InputArray*", "double", "double"]), _)]),
	void cv_ximgproc_createDTFilter_const__InputArrayR_double_double(const cv::_InputArray* guide, double sigmaSpatial, double sigmaColor, Result<cv::Ptr<cv::ximgproc::DTFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::DTFilter> ret = cv::ximgproc::createDTFilter(*guide, sigmaSpatial, sigmaColor);
			Ok(new cv::Ptr<cv::ximgproc::DTFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createDTFilter(InputArray, double, double, int, int)(InputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:102
	// ("cv::ximgproc::createDTFilter", vec![(pred!(mut, ["guide", "sigmaSpatial", "sigmaColor", "mode", "numIters"], ["const cv::_InputArray*", "double", "double", "int", "int"]), _)]),
	void cv_ximgproc_createDTFilter_const__InputArrayR_double_double_int_int(const cv::_InputArray* guide, double sigmaSpatial, double sigmaColor, int mode, int numIters, Result<cv::Ptr<cv::ximgproc::DTFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::DTFilter> ret = cv::ximgproc::createDTFilter(*guide, sigmaSpatial, sigmaColor, mode, numIters);
			Ok(new cv::Ptr<cv::ximgproc::DTFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createDisparityWLSFilterGeneric(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/disparity_filter.hpp:149
	// ("cv::ximgproc::createDisparityWLSFilterGeneric", vec![(pred!(mut, ["use_confidence"], ["bool"]), _)]),
	void cv_ximgproc_createDisparityWLSFilterGeneric_bool(bool use_confidence, Result<cv::Ptr<cv::ximgproc::DisparityWLSFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::DisparityWLSFilter> ret = cv::ximgproc::createDisparityWLSFilterGeneric(use_confidence);
			Ok(new cv::Ptr<cv::ximgproc::DisparityWLSFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createDisparityWLSFilter(Ptr<StereoMatcher>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/disparity_filter.hpp:131
	// ("cv::ximgproc::createDisparityWLSFilter", vec![(pred!(mut, ["matcher_left"], ["cv::Ptr<cv::StereoMatcher>"]), _)]),
	void cv_ximgproc_createDisparityWLSFilter_PtrLStereoMatcherG(cv::Ptr<cv::StereoMatcher>* matcher_left, Result<cv::Ptr<cv::ximgproc::DisparityWLSFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::DisparityWLSFilter> ret = cv::ximgproc::createDisparityWLSFilter(*matcher_left);
			Ok(new cv::Ptr<cv::ximgproc::DisparityWLSFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createEdgeAwareInterpolator()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:138
	// ("cv::ximgproc::createEdgeAwareInterpolator", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_createEdgeAwareInterpolator(Result<cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::EdgeAwareInterpolator> ret = cv::ximgproc::createEdgeAwareInterpolator();
			Ok(new cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::createEdgeBoxes() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edgeboxes.hpp:183
	// ("cv::ximgproc::createEdgeBoxes", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_createEdgeBoxes(Result<cv::Ptr<cv::ximgproc::EdgeBoxes>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::EdgeBoxes> ret = cv::ximgproc::createEdgeBoxes();
			Ok(new cv::Ptr<cv::ximgproc::EdgeBoxes>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createEdgeBoxes(float, float, float, float, int, float, float, float, float, float, float, float)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edgeboxes.hpp:183
	// ("cv::ximgproc::createEdgeBoxes", vec![(pred!(mut, ["alpha", "beta", "eta", "minScore", "maxBoxes", "edgeMinMag", "edgeMergeThr", "clusterMinMag", "maxAspectRatio", "minBoxArea", "gamma", "kappa"], ["float", "float", "float", "float", "int", "float", "float", "float", "float", "float", "float", "float"]), _)]),
	void cv_ximgproc_createEdgeBoxes_float_float_float_float_int_float_float_float_float_float_float_float(float alpha, float beta, float eta, float minScore, int maxBoxes, float edgeMinMag, float edgeMergeThr, float clusterMinMag, float maxAspectRatio, float minBoxArea, float gamma, float kappa, Result<cv::Ptr<cv::ximgproc::EdgeBoxes>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::EdgeBoxes> ret = cv::ximgproc::createEdgeBoxes(alpha, beta, eta, minScore, maxBoxes, edgeMinMag, edgeMergeThr, clusterMinMag, maxAspectRatio, minBoxArea, gamma, kappa);
			Ok(new cv::Ptr<cv::ximgproc::EdgeBoxes>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createEdgeDrawing()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_drawing.hpp:126
	// ("cv::ximgproc::createEdgeDrawing", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_createEdgeDrawing(Result<cv::Ptr<cv::ximgproc::EdgeDrawing>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::EdgeDrawing> ret = cv::ximgproc::createEdgeDrawing();
			Ok(new cv::Ptr<cv::ximgproc::EdgeDrawing>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::createFastBilateralSolverFilter(InputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:423
	// ("cv::ximgproc::createFastBilateralSolverFilter", vec![(pred!(mut, ["guide", "sigma_spatial", "sigma_luma", "sigma_chroma"], ["const cv::_InputArray*", "double", "double", "double"]), _)]),
	void cv_ximgproc_createFastBilateralSolverFilter_const__InputArrayR_double_double_double(const cv::_InputArray* guide, double sigma_spatial, double sigma_luma, double sigma_chroma, Result<cv::Ptr<cv::ximgproc::FastBilateralSolverFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::FastBilateralSolverFilter> ret = cv::ximgproc::createFastBilateralSolverFilter(*guide, sigma_spatial, sigma_luma, sigma_chroma);
			Ok(new cv::Ptr<cv::ximgproc::FastBilateralSolverFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createFastBilateralSolverFilter(InputArray, double, double, double, double, int, double)(InputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:423
	// ("cv::ximgproc::createFastBilateralSolverFilter", vec![(pred!(mut, ["guide", "sigma_spatial", "sigma_luma", "sigma_chroma", "lambda", "num_iter", "max_tol"], ["const cv::_InputArray*", "double", "double", "double", "double", "int", "double"]), _)]),
	void cv_ximgproc_createFastBilateralSolverFilter_const__InputArrayR_double_double_double_double_int_double(const cv::_InputArray* guide, double sigma_spatial, double sigma_luma, double sigma_chroma, double lambda, int num_iter, double max_tol, Result<cv::Ptr<cv::ximgproc::FastBilateralSolverFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::FastBilateralSolverFilter> ret = cv::ximgproc::createFastBilateralSolverFilter(*guide, sigma_spatial, sigma_luma, sigma_chroma, lambda, num_iter, max_tol);
			Ok(new cv::Ptr<cv::ximgproc::FastBilateralSolverFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::createFastGlobalSmootherFilter(InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:495
	// ("cv::ximgproc::createFastGlobalSmootherFilter", vec![(pred!(mut, ["guide", "lambda", "sigma_color"], ["const cv::_InputArray*", "double", "double"]), _)]),
	void cv_ximgproc_createFastGlobalSmootherFilter_const__InputArrayR_double_double(const cv::_InputArray* guide, double lambda, double sigma_color, Result<cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter> ret = cv::ximgproc::createFastGlobalSmootherFilter(*guide, lambda, sigma_color);
			Ok(new cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createFastGlobalSmootherFilter(InputArray, double, double, double, int)(InputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:495
	// ("cv::ximgproc::createFastGlobalSmootherFilter", vec![(pred!(mut, ["guide", "lambda", "sigma_color", "lambda_attenuation", "num_iter"], ["const cv::_InputArray*", "double", "double", "double", "int"]), _)]),
	void cv_ximgproc_createFastGlobalSmootherFilter_const__InputArrayR_double_double_double_int(const cv::_InputArray* guide, double lambda, double sigma_color, double lambda_attenuation, int num_iter, Result<cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter> ret = cv::ximgproc::createFastGlobalSmootherFilter(*guide, lambda, sigma_color, lambda_attenuation, num_iter);
			Ok(new cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::createFastLineDetector() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/fast_line_detector.hpp:71
	// ("cv::ximgproc::createFastLineDetector", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_createFastLineDetector(Result<cv::Ptr<cv::ximgproc::FastLineDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::FastLineDetector> ret = cv::ximgproc::createFastLineDetector();
			Ok(new cv::Ptr<cv::ximgproc::FastLineDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createFastLineDetector(int, float, double, double, int, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/fast_line_detector.hpp:71
	// ("cv::ximgproc::createFastLineDetector", vec![(pred!(mut, ["length_threshold", "distance_threshold", "canny_th1", "canny_th2", "canny_aperture_size", "do_merge"], ["int", "float", "double", "double", "int", "bool"]), _)]),
	void cv_ximgproc_createFastLineDetector_int_float_double_double_int_bool(int length_threshold, float distance_threshold, double canny_th1, double canny_th2, int canny_aperture_size, bool do_merge, Result<cv::Ptr<cv::ximgproc::FastLineDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::FastLineDetector> ret = cv::ximgproc::createFastLineDetector(length_threshold, distance_threshold, canny_th1, canny_th2, canny_aperture_size, do_merge);
			Ok(new cv::Ptr<cv::ximgproc::FastLineDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::createGuidedFilter(InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:161
	// ("cv::ximgproc::createGuidedFilter", vec![(pred!(mut, ["guide", "radius", "eps"], ["const cv::_InputArray*", "int", "double"]), _)]),
	void cv_ximgproc_createGuidedFilter_const__InputArrayR_int_double(const cv::_InputArray* guide, int radius, double eps, Result<cv::Ptr<cv::ximgproc::GuidedFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::GuidedFilter> ret = cv::ximgproc::createGuidedFilter(*guide, radius, eps);
			Ok(new cv::Ptr<cv::ximgproc::GuidedFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createGuidedFilter(InputArray, int, double, double)(InputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:161
	// ("cv::ximgproc::createGuidedFilter", vec![(pred!(mut, ["guide", "radius", "eps", "scale"], ["const cv::_InputArray*", "int", "double", "double"]), _)]),
	void cv_ximgproc_createGuidedFilter_const__InputArrayR_int_double_double(const cv::_InputArray* guide, int radius, double eps, double scale, Result<cv::Ptr<cv::ximgproc::GuidedFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::GuidedFilter> ret = cv::ximgproc::createGuidedFilter(*guide, radius, eps, scale);
			Ok(new cv::Ptr<cv::ximgproc::GuidedFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createQuaternionImage(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/color_match.hpp:22
	// ("cv::ximgproc::createQuaternionImage", vec![(pred!(mut, ["img", "qimg"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_createQuaternionImage_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* img, const cv::_OutputArray* qimg, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::createQuaternionImage(*img, *qimg);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createRFFeatureGetter()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/structured_edge_detection.hpp:91
	// ("cv::ximgproc::createRFFeatureGetter", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_createRFFeatureGetter(Result<cv::Ptr<cv::ximgproc::RFFeatureGetter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::RFFeatureGetter> ret = cv::ximgproc::createRFFeatureGetter();
			Ok(new cv::Ptr<cv::ximgproc::RFFeatureGetter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createRICInterpolator()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:265
	// ("cv::ximgproc::createRICInterpolator", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_createRICInterpolator(Result<cv::Ptr<cv::ximgproc::RICInterpolator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::RICInterpolator> ret = cv::ximgproc::createRICInterpolator();
			Ok(new cv::Ptr<cv::ximgproc::RICInterpolator>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createRightMatcher(Ptr<StereoMatcher>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/disparity_filter.hpp:139
	// ("cv::ximgproc::createRightMatcher", vec![(pred!(mut, ["matcher_left"], ["cv::Ptr<cv::StereoMatcher>"]), _)]),
	void cv_ximgproc_createRightMatcher_PtrLStereoMatcherG(cv::Ptr<cv::StereoMatcher>* matcher_left, Result<cv::Ptr<cv::StereoMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::StereoMatcher> ret = cv::ximgproc::createRightMatcher(*matcher_left);
			Ok(new cv::Ptr<cv::StereoMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::createScanSegment(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/scansegment.hpp:80
	// ("cv::ximgproc::createScanSegment", vec![(pred!(mut, ["image_width", "image_height", "num_superpixels"], ["int", "int", "int"]), _)]),
	void cv_ximgproc_createScanSegment_int_int_int(int image_width, int image_height, int num_superpixels, Result<cv::Ptr<cv::ximgproc::ScanSegment>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::ScanSegment> ret = cv::ximgproc::createScanSegment(image_width, image_height, num_superpixels);
			Ok(new cv::Ptr<cv::ximgproc::ScanSegment>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createScanSegment(int, int, int, int, bool)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/scansegment.hpp:80
	// ("cv::ximgproc::createScanSegment", vec![(pred!(mut, ["image_width", "image_height", "num_superpixels", "slices", "merge_small"], ["int", "int", "int", "int", "bool"]), _)]),
	void cv_ximgproc_createScanSegment_int_int_int_int_bool(int image_width, int image_height, int num_superpixels, int slices, bool merge_small, Result<cv::Ptr<cv::ximgproc::ScanSegment>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::ScanSegment> ret = cv::ximgproc::createScanSegment(image_width, image_height, num_superpixels, slices, merge_small);
			Ok(new cv::Ptr<cv::ximgproc::ScanSegment>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::createStructuredEdgeDetection(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/structured_edge_detection.hpp:140
	// ("cv::ximgproc::createStructuredEdgeDetection", vec![(pred!(mut, ["model"], ["const cv::String*"]), _)]),
	void cv_ximgproc_createStructuredEdgeDetection_const_StringR(const char* model, Result<cv::Ptr<cv::ximgproc::StructuredEdgeDetection>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::StructuredEdgeDetection> ret = cv::ximgproc::createStructuredEdgeDetection(std::string(model));
			Ok(new cv::Ptr<cv::ximgproc::StructuredEdgeDetection>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createStructuredEdgeDetection(const String &, Ptr<const RFFeatureGetter>)(InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/structured_edge_detection.hpp:140
	// ("cv::ximgproc::createStructuredEdgeDetection", vec![(pred!(mut, ["model", "howToGetFeatures"], ["const cv::String*", "cv::Ptr<const cv::ximgproc::RFFeatureGetter>"]), _)]),
	void cv_ximgproc_createStructuredEdgeDetection_const_StringR_PtrLconst_RFFeatureGetterG(const char* model, const cv::Ptr<const cv::ximgproc::RFFeatureGetter>* howToGetFeatures, Result<cv::Ptr<cv::ximgproc::StructuredEdgeDetection>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::StructuredEdgeDetection> ret = cv::ximgproc::createStructuredEdgeDetection(std::string(model), *howToGetFeatures);
			Ok(new cv::Ptr<cv::ximgproc::StructuredEdgeDetection>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::createSuperpixelLSC(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/lsc.hpp:150
	// ("cv::ximgproc::createSuperpixelLSC", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
	void cv_ximgproc_createSuperpixelLSC_const__InputArrayR(const cv::_InputArray* image, Result<cv::Ptr<cv::ximgproc::SuperpixelLSC>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::SuperpixelLSC> ret = cv::ximgproc::createSuperpixelLSC(*image);
			Ok(new cv::Ptr<cv::ximgproc::SuperpixelLSC>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createSuperpixelLSC(InputArray, int, float)(InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/lsc.hpp:150
	// ("cv::ximgproc::createSuperpixelLSC", vec![(pred!(mut, ["image", "region_size", "ratio"], ["const cv::_InputArray*", "int", "float"]), _)]),
	void cv_ximgproc_createSuperpixelLSC_const__InputArrayR_int_float(const cv::_InputArray* image, int region_size, float ratio, Result<cv::Ptr<cv::ximgproc::SuperpixelLSC>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::SuperpixelLSC> ret = cv::ximgproc::createSuperpixelLSC(*image, region_size, ratio);
			Ok(new cv::Ptr<cv::ximgproc::SuperpixelLSC>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::createSuperpixelSEEDS(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/seeds.hpp:173
	// ("cv::ximgproc::createSuperpixelSEEDS", vec![(pred!(mut, ["image_width", "image_height", "image_channels", "num_superpixels", "num_levels"], ["int", "int", "int", "int", "int"]), _)]),
	void cv_ximgproc_createSuperpixelSEEDS_int_int_int_int_int(int image_width, int image_height, int image_channels, int num_superpixels, int num_levels, Result<cv::Ptr<cv::ximgproc::SuperpixelSEEDS>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::SuperpixelSEEDS> ret = cv::ximgproc::createSuperpixelSEEDS(image_width, image_height, image_channels, num_superpixels, num_levels);
			Ok(new cv::Ptr<cv::ximgproc::SuperpixelSEEDS>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createSuperpixelSEEDS(int, int, int, int, int, int, int, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/seeds.hpp:173
	// ("cv::ximgproc::createSuperpixelSEEDS", vec![(pred!(mut, ["image_width", "image_height", "image_channels", "num_superpixels", "num_levels", "prior", "histogram_bins", "double_step"], ["int", "int", "int", "int", "int", "int", "int", "bool"]), _)]),
	void cv_ximgproc_createSuperpixelSEEDS_int_int_int_int_int_int_int_bool(int image_width, int image_height, int image_channels, int num_superpixels, int num_levels, int prior, int histogram_bins, bool double_step, Result<cv::Ptr<cv::ximgproc::SuperpixelSEEDS>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::SuperpixelSEEDS> ret = cv::ximgproc::createSuperpixelSEEDS(image_width, image_height, image_channels, num_superpixels, num_levels, prior, histogram_bins, double_step);
			Ok(new cv::Ptr<cv::ximgproc::SuperpixelSEEDS>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::createSuperpixelSLIC(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/slic.hpp:160
	// ("cv::ximgproc::createSuperpixelSLIC", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
	void cv_ximgproc_createSuperpixelSLIC_const__InputArrayR(const cv::_InputArray* image, Result<cv::Ptr<cv::ximgproc::SuperpixelSLIC>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::SuperpixelSLIC> ret = cv::ximgproc::createSuperpixelSLIC(*image);
			Ok(new cv::Ptr<cv::ximgproc::SuperpixelSLIC>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createSuperpixelSLIC(InputArray, int, int, float)(InputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/slic.hpp:160
	// ("cv::ximgproc::createSuperpixelSLIC", vec![(pred!(mut, ["image", "algorithm", "region_size", "ruler"], ["const cv::_InputArray*", "int", "int", "float"]), _)]),
	void cv_ximgproc_createSuperpixelSLIC_const__InputArrayR_int_int_float(const cv::_InputArray* image, int algorithm, int region_size, float ruler, Result<cv::Ptr<cv::ximgproc::SuperpixelSLIC>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::SuperpixelSLIC> ret = cv::ximgproc::createSuperpixelSLIC(*image, algorithm, region_size, ruler);
			Ok(new cv::Ptr<cv::ximgproc::SuperpixelSLIC>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::dtFilter(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:121
	// ("cv::ximgproc::dtFilter", vec![(pred!(mut, ["guide", "src", "dst", "sigmaSpatial", "sigmaColor"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
	void cv_ximgproc_dtFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double(const cv::_InputArray* guide, const cv::_InputArray* src, const cv::_OutputArray* dst, double sigmaSpatial, double sigmaColor, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::dtFilter(*guide, *src, *dst, sigmaSpatial, sigmaColor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dtFilter(InputArray, InputArray, OutputArray, double, double, int, int)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:121
	// ("cv::ximgproc::dtFilter", vec![(pred!(mut, ["guide", "src", "dst", "sigmaSpatial", "sigmaColor", "mode", "numIters"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "int", "int"]), _)]),
	void cv_ximgproc_dtFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double_int_int(const cv::_InputArray* guide, const cv::_InputArray* src, const cv::_OutputArray* dst, double sigmaSpatial, double sigmaColor, int mode, int numIters, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::dtFilter(*guide, *src, *dst, sigmaSpatial, sigmaColor, mode, numIters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// edgePreservingFilter(InputArray, OutputArray, int, double)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edgepreserving_filter.hpp:27
	// ("cv::ximgproc::edgePreservingFilter", vec![(pred!(mut, ["src", "dst", "d", "threshold"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "double"]), _)]),
	void cv_ximgproc_edgePreservingFilter_const__InputArrayR_const__OutputArrayR_int_double(const cv::_InputArray* src, const cv::_OutputArray* dst, int d, double threshold, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::edgePreservingFilter(*src, *dst, d, threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::fastBilateralSolverFilter(InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:454
	// ("cv::ximgproc::fastBilateralSolverFilter", vec![(pred!(mut, ["guide", "src", "confidence", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_fastBilateralSolverFilter_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* guide, const cv::_InputArray* src, const cv::_InputArray* confidence, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::fastBilateralSolverFilter(*guide, *src, *confidence, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fastBilateralSolverFilter(InputArray, InputArray, InputArray, OutputArray, double, double, double, double, int, double)(InputArray, InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:454
	// ("cv::ximgproc::fastBilateralSolverFilter", vec![(pred!(mut, ["guide", "src", "confidence", "dst", "sigma_spatial", "sigma_luma", "sigma_chroma", "lambda", "num_iter", "max_tol"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "double", "double", "int", "double"]), _)]),
	void cv_ximgproc_fastBilateralSolverFilter_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double_double_double_int_double(const cv::_InputArray* guide, const cv::_InputArray* src, const cv::_InputArray* confidence, const cv::_OutputArray* dst, double sigma_spatial, double sigma_luma, double sigma_chroma, double lambda, int num_iter, double max_tol, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::fastBilateralSolverFilter(*guide, *src, *confidence, *dst, sigma_spatial, sigma_luma, sigma_chroma, lambda, num_iter, max_tol);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::fastGlobalSmootherFilter(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:515
	// ("cv::ximgproc::fastGlobalSmootherFilter", vec![(pred!(mut, ["guide", "src", "dst", "lambda", "sigma_color"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
	void cv_ximgproc_fastGlobalSmootherFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double(const cv::_InputArray* guide, const cv::_InputArray* src, const cv::_OutputArray* dst, double lambda, double sigma_color, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::fastGlobalSmootherFilter(*guide, *src, *dst, lambda, sigma_color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fastGlobalSmootherFilter(InputArray, InputArray, OutputArray, double, double, double, int)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:515
	// ("cv::ximgproc::fastGlobalSmootherFilter", vec![(pred!(mut, ["guide", "src", "dst", "lambda", "sigma_color", "lambda_attenuation", "num_iter"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "double", "int"]), _)]),
	void cv_ximgproc_fastGlobalSmootherFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double_double_int(const cv::_InputArray* guide, const cv::_InputArray* src, const cv::_OutputArray* dst, double lambda, double sigma_color, double lambda_attenuation, int num_iter, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::fastGlobalSmootherFilter(*guide, *src, *dst, lambda, sigma_color, lambda_attenuation, num_iter);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::findEllipses(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/find_ellipses.hpp:30
	// ("cv::ximgproc::findEllipses", vec![(pred!(mut, ["image", "ellipses"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_findEllipses_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* image, const cv::_OutputArray* ellipses, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::findEllipses(*image, *ellipses);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findEllipses(InputArray, OutputArray, float, float, float)(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/find_ellipses.hpp:30
	// ("cv::ximgproc::findEllipses", vec![(pred!(mut, ["image", "ellipses", "scoreThreshold", "reliabilityThreshold", "centerDistanceThreshold"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float", "float", "float"]), _)]),
	void cv_ximgproc_findEllipses_const__InputArrayR_const__OutputArrayR_float_float_float(const cv::_InputArray* image, const cv::_OutputArray* ellipses, float scoreThreshold, float reliabilityThreshold, float centerDistanceThreshold, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::findEllipses(*image, *ellipses, scoreThreshold, reliabilityThreshold, centerDistanceThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::fourierDescriptor(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/fourier_descriptors.hpp:87
	// ("cv::ximgproc::fourierDescriptor", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_fourierDescriptor_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::fourierDescriptor(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fourierDescriptor(InputArray, OutputArray, int, int)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/fourier_descriptors.hpp:87
	// ("cv::ximgproc::fourierDescriptor", vec![(pred!(mut, ["src", "dst", "nbElt", "nbFD"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_ximgproc_fourierDescriptor_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int nbElt, int nbFD, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::fourierDescriptor(*src, *dst, nbElt, nbFD);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::getDisparityVis(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/disparity_filter.hpp:204
	// ("cv::ximgproc::getDisparityVis", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_getDisparityVis_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::getDisparityVis(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDisparityVis(InputArray, OutputArray, double)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/disparity_filter.hpp:204
	// ("cv::ximgproc::getDisparityVis", vec![(pred!(mut, ["src", "dst", "scale"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double"]), _)]),
	void cv_ximgproc_getDisparityVis_const__InputArrayR_const__OutputArrayR_double(const cv::_InputArray* src, const cv::_OutputArray* dst, double scale, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::getDisparityVis(*src, *dst, scale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::guidedFilter(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:186
	// ("cv::ximgproc::guidedFilter", vec![(pred!(mut, ["guide", "src", "dst", "radius", "eps"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "double"]), _)]),
	void cv_ximgproc_guidedFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double(const cv::_InputArray* guide, const cv::_InputArray* src, const cv::_OutputArray* dst, int radius, double eps, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::guidedFilter(*guide, *src, *dst, radius, eps);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// guidedFilter(InputArray, InputArray, OutputArray, int, double, int, double)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:186
	// ("cv::ximgproc::guidedFilter", vec![(pred!(mut, ["guide", "src", "dst", "radius", "eps", "dDepth", "scale"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "int", "double"]), _)]),
	void cv_ximgproc_guidedFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_int_double(const cv::_InputArray* guide, const cv::_InputArray* src, const cv::_OutputArray* dst, int radius, double eps, int dDepth, double scale, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::guidedFilter(*guide, *src, *dst, radius, eps, dDepth, scale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::jointBilateralFilter(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:323
	// ("cv::ximgproc::jointBilateralFilter", vec![(pred!(mut, ["joint", "src", "dst", "d", "sigmaColor", "sigmaSpace"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double"]), _)]),
	void cv_ximgproc_jointBilateralFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_double(const cv::_InputArray* joint, const cv::_InputArray* src, const cv::_OutputArray* dst, int d, double sigmaColor, double sigmaSpace, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::jointBilateralFilter(*joint, *src, *dst, d, sigmaColor, sigmaSpace);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// jointBilateralFilter(InputArray, InputArray, OutputArray, int, double, double, int)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:323
	// ("cv::ximgproc::jointBilateralFilter", vec![(pred!(mut, ["joint", "src", "dst", "d", "sigmaColor", "sigmaSpace", "borderType"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double", "int"]), _)]),
	void cv_ximgproc_jointBilateralFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_double_int(const cv::_InputArray* joint, const cv::_InputArray* src, const cv::_OutputArray* dst, int d, double sigmaColor, double sigmaSpace, int borderType, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::jointBilateralFilter(*joint, *src, *dst, d, sigmaColor, sigmaSpace, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::l0Smooth(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:529
	// ("cv::ximgproc::l0Smooth", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_l0Smooth_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::l0Smooth(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// l0Smooth(InputArray, OutputArray, double, double)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:529
	// ("cv::ximgproc::l0Smooth", vec![(pred!(mut, ["src", "dst", "lambda", "kappa"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
	void cv_ximgproc_l0Smooth_const__InputArrayR_const__OutputArrayR_double_double(const cv::_InputArray* src, const cv::_OutputArray* dst, double lambda, double kappa, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::l0Smooth(*src, *dst, lambda, kappa);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::niBlackThreshold(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc.hpp:177
	// ("cv::ximgproc::niBlackThreshold", vec![(pred!(mut, ["_src", "_dst", "maxValue", "type", "blockSize", "k"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "int", "int", "double"]), _)]),
	void cv_ximgproc_niBlackThreshold_const__InputArrayR_const__OutputArrayR_double_int_int_double(const cv::_InputArray* _src, const cv::_OutputArray* _dst, double maxValue, int type, int blockSize, double k, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::niBlackThreshold(*_src, *_dst, maxValue, type, blockSize, k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// niBlackThreshold(InputArray, OutputArray, double, int, int, double, int, double)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc.hpp:177
	// ("cv::ximgproc::niBlackThreshold", vec![(pred!(mut, ["_src", "_dst", "maxValue", "type", "blockSize", "k", "binarizationMethod", "r"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "int", "int", "double", "int", "double"]), _)]),
	void cv_ximgproc_niBlackThreshold_const__InputArrayR_const__OutputArrayR_double_int_int_double_int_double(const cv::_InputArray* _src, const cv::_OutputArray* _dst, double maxValue, int type, int blockSize, double k, int binarizationMethod, double r, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::niBlackThreshold(*_src, *_dst, maxValue, type, blockSize, k, binarizationMethod, r);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// qconj(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/color_match.hpp:30
	// ("cv::ximgproc::qconj", vec![(pred!(mut, ["qimg", "qcimg"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_qconj_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* qimg, const cv::_OutputArray* qcimg, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::qconj(*qimg, *qcimg);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// qdft(InputArray, OutputArray, int, bool)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/color_match.hpp:54
	// ("cv::ximgproc::qdft", vec![(pred!(mut, ["img", "qimg", "flags", "sideLeft"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "bool"]), _)]),
	void cv_ximgproc_qdft_const__InputArrayR_const__OutputArrayR_int_bool(const cv::_InputArray* img, const cv::_OutputArray* qimg, int flags, bool sideLeft, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::qdft(*img, *qimg, flags, sideLeft);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// qmultiply(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/color_match.hpp:45
	// ("cv::ximgproc::qmultiply", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_qmultiply_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::qmultiply(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// qunitary(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/color_match.hpp:37
	// ("cv::ximgproc::qunitary", vec![(pred!(mut, ["qimg", "qnimg"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_qunitary_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* qimg, const cv::_OutputArray* qnimg, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::qunitary(*qimg, *qnimg);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readGT(String, OutputArray)(InString, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/disparity_filter.hpp:164
	// ("cv::ximgproc::readGT", vec![(pred!(mut, ["src_path", "dst"], ["cv::String", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_readGT_String_const__OutputArrayR(const char* src_path, const cv::_OutputArray* dst, Result<int>* ocvrs_return) {
		try {
			int ret = cv::ximgproc::readGT(std::string(src_path), *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::rl::createRLEImage(CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/run_length_morphology.hpp:97
	// ("cv::ximgproc::rl::createRLEImage", vec![(pred!(mut, ["runs", "res"], ["const std::vector<cv::Point3i>*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_rl_createRLEImage_const_vectorLPoint3iGR_const__OutputArrayR(const std::vector<cv::Point3i>* runs, const cv::_OutputArray* res, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::rl::createRLEImage(*runs, *res);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createRLEImage(const std::vector<cv::Point3i> &, OutputArray, Size)(CppPassByVoidPtr, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/run_length_morphology.hpp:97
	// ("cv::ximgproc::rl::createRLEImage", vec![(pred!(mut, ["runs", "res", "size"], ["const std::vector<cv::Point3i>*", "const cv::_OutputArray*", "cv::Size"]), _)]),
	void cv_ximgproc_rl_createRLEImage_const_vectorLPoint3iGR_const__OutputArrayR_Size(const std::vector<cv::Point3i>* runs, const cv::_OutputArray* res, cv::Size* size, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::rl::createRLEImage(*runs, *res, *size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::rl::dilate(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/run_length_morphology.hpp:42
	// ("cv::ximgproc::rl::dilate", vec![(pred!(mut, ["rlSrc", "rlDest", "rlKernel"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_ximgproc_rl_dilate_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* rlSrc, const cv::_OutputArray* rlDest, const cv::_InputArray* rlKernel, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::rl::dilate(*rlSrc, *rlDest, *rlKernel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dilate(InputArray, OutputArray, InputArray, Point)(InputArray, OutputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/run_length_morphology.hpp:42
	// ("cv::ximgproc::rl::dilate", vec![(pred!(mut, ["rlSrc", "rlDest", "rlKernel", "anchor"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::Point"]), _)]),
	void cv_ximgproc_rl_dilate_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Point(const cv::_InputArray* rlSrc, const cv::_OutputArray* rlDest, const cv::_InputArray* rlKernel, cv::Point* anchor, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::rl::dilate(*rlSrc, *rlDest, *rlKernel, *anchor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::rl::erode(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/run_length_morphology.hpp:57
	// ("cv::ximgproc::rl::erode", vec![(pred!(mut, ["rlSrc", "rlDest", "rlKernel"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_ximgproc_rl_erode_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* rlSrc, const cv::_OutputArray* rlDest, const cv::_InputArray* rlKernel, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::rl::erode(*rlSrc, *rlDest, *rlKernel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// erode(InputArray, OutputArray, InputArray, bool, Point)(InputArray, OutputArray, InputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/run_length_morphology.hpp:57
	// ("cv::ximgproc::rl::erode", vec![(pred!(mut, ["rlSrc", "rlDest", "rlKernel", "bBoundaryOn", "anchor"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "bool", "cv::Point"]), _)]),
	void cv_ximgproc_rl_erode_const__InputArrayR_const__OutputArrayR_const__InputArrayR_bool_Point(const cv::_InputArray* rlSrc, const cv::_OutputArray* rlDest, const cv::_InputArray* rlKernel, bool bBoundaryOn, cv::Point* anchor, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::rl::erode(*rlSrc, *rlDest, *rlKernel, bBoundaryOn, *anchor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getStructuringElement(int, Size)(Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/run_length_morphology.hpp:68
	// ("cv::ximgproc::rl::getStructuringElement", vec![(pred!(mut, ["shape", "ksize"], ["int", "cv::Size"]), _)]),
	void cv_ximgproc_rl_getStructuringElement_int_Size(int shape, cv::Size* ksize, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::ximgproc::rl::getStructuringElement(shape, *ksize);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isRLMorphologyPossible(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/run_length_morphology.hpp:87
	// ("cv::ximgproc::rl::isRLMorphologyPossible", vec![(pred!(mut, ["rlStructuringElement"], ["const cv::_InputArray*"]), _)]),
	void cv_ximgproc_rl_isRLMorphologyPossible_const__InputArrayR(const cv::_InputArray* rlStructuringElement, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::ximgproc::rl::isRLMorphologyPossible(*rlStructuringElement);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::rl::morphologyEx(InputArray, OutputArray, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/run_length_morphology.hpp:113
	// ("cv::ximgproc::rl::morphologyEx", vec![(pred!(mut, ["rlSrc", "rlDest", "op", "rlKernel"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "const cv::_InputArray*"]), _)]),
	void cv_ximgproc_rl_morphologyEx_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR(const cv::_InputArray* rlSrc, const cv::_OutputArray* rlDest, int op, const cv::_InputArray* rlKernel, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::rl::morphologyEx(*rlSrc, *rlDest, op, *rlKernel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// morphologyEx(InputArray, OutputArray, int, InputArray, bool, Point)(InputArray, OutputArray, Primitive, InputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/run_length_morphology.hpp:113
	// ("cv::ximgproc::rl::morphologyEx", vec![(pred!(mut, ["rlSrc", "rlDest", "op", "rlKernel", "bBoundaryOnForErosion", "anchor"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "const cv::_InputArray*", "bool", "cv::Point"]), _)]),
	void cv_ximgproc_rl_morphologyEx_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR_bool_Point(const cv::_InputArray* rlSrc, const cv::_OutputArray* rlDest, int op, const cv::_InputArray* rlKernel, bool bBoundaryOnForErosion, cv::Point* anchor, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::rl::morphologyEx(*rlSrc, *rlDest, op, *rlKernel, bBoundaryOnForErosion, *anchor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// paint(InputOutputArray, InputArray, const cv::Scalar &)(InputOutputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/run_length_morphology.hpp:79
	// ("cv::ximgproc::rl::paint", vec![(pred!(mut, ["image", "rlSrc", "value"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::Scalar*"]), _)]),
	void cv_ximgproc_rl_paint_const__InputOutputArrayR_const__InputArrayR_const_ScalarR(const cv::_InputOutputArray* image, const cv::_InputArray* rlSrc, const cv::Scalar* value, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::rl::paint(*image, *rlSrc, *value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// threshold(InputArray, OutputArray, double, int)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/run_length_morphology.hpp:28
	// ("cv::ximgproc::rl::threshold", vec![(pred!(mut, ["src", "rlDest", "thresh", "type"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "int"]), _)]),
	void cv_ximgproc_rl_threshold_const__InputArrayR_const__OutputArrayR_double_int(const cv::_InputArray* src, const cv::_OutputArray* rlDest, double thresh, int type, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::rl::threshold(*src, *rlDest, thresh, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::rollingGuidanceFilter(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:379
	// ("cv::ximgproc::rollingGuidanceFilter", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_rollingGuidanceFilter_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::rollingGuidanceFilter(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// rollingGuidanceFilter(InputArray, OutputArray, int, double, double, int, int)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:379
	// ("cv::ximgproc::rollingGuidanceFilter", vec![(pred!(mut, ["src", "dst", "d", "sigmaColor", "sigmaSpace", "numOfIter", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double", "int", "int"]), _)]),
	void cv_ximgproc_rollingGuidanceFilter_const__InputArrayR_const__OutputArrayR_int_double_double_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int d, double sigmaColor, double sigmaSpace, int numOfIter, int borderType, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::rollingGuidanceFilter(*src, *dst, d, sigmaColor, sigmaSpace, numOfIter, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::segmentation::createGraphSegmentation() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:69
	// ("cv::ximgproc::segmentation::createGraphSegmentation", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_segmentation_createGraphSegmentation(Result<cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation> ret = cv::ximgproc::segmentation::createGraphSegmentation();
			Ok(new cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createGraphSegmentation(double, float, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:69
	// ("cv::ximgproc::segmentation::createGraphSegmentation", vec![(pred!(mut, ["sigma", "k", "min_size"], ["double", "float", "int"]), _)]),
	void cv_ximgproc_segmentation_createGraphSegmentation_double_float_int(double sigma, float k, int min_size, Result<cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation> ret = cv::ximgproc::segmentation::createGraphSegmentation(sigma, k, min_size);
			Ok(new cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createSelectiveSearchSegmentation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:244
	// ("cv::ximgproc::segmentation::createSelectiveSearchSegmentation", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_segmentation_createSelectiveSearchSegmentation(Result<cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentation>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentation> ret = cv::ximgproc::segmentation::createSelectiveSearchSegmentation();
			Ok(new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentation>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createSelectiveSearchSegmentationStrategyColor()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:104
	// ("cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyColor", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyColor(Result<cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor> ret = cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyColor();
			Ok(new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createSelectiveSearchSegmentationStrategyFill()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:131
	// ("cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyFill", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyFill(Result<cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill> ret = cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyFill();
			Ok(new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createSelectiveSearchSegmentationStrategyMultiple()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:149
	// ("cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyMultiple", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyMultiple(Result<cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple> ret = cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyMultiple();
			Ok(new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createSelectiveSearchSegmentationStrategyMultiple(Ptr<SelectiveSearchSegmentationStrategy>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:154
	// ("cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyMultiple", vec![(pred!(mut, ["s1"], ["cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>"]), _)]),
	void cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyMultiple_PtrLSelectiveSearchSegmentationStrategyG(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* s1, Result<cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple> ret = cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyMultiple(*s1);
			Ok(new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createSelectiveSearchSegmentationStrategyMultiple(Ptr<SelectiveSearchSegmentationStrategy>, Ptr<SelectiveSearchSegmentationStrategy>)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:160
	// ("cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyMultiple", vec![(pred!(mut, ["s1", "s2"], ["cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>", "cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>"]), _)]),
	void cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyMultiple_PtrLSelectiveSearchSegmentationStrategyG_PtrLSelectiveSearchSegmentationStrategyG(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* s1, cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* s2, Result<cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple> ret = cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyMultiple(*s1, *s2);
			Ok(new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createSelectiveSearchSegmentationStrategyMultiple(Ptr<SelectiveSearchSegmentationStrategy>, Ptr<SelectiveSearchSegmentationStrategy>, Ptr<SelectiveSearchSegmentationStrategy>)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:168
	// ("cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyMultiple", vec![(pred!(mut, ["s1", "s2", "s3"], ["cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>", "cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>", "cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>"]), _)]),
	void cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyMultiple_PtrLSelectiveSearchSegmentationStrategyG_PtrLSelectiveSearchSegmentationStrategyG_PtrLSelectiveSearchSegmentationStrategyG(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* s1, cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* s2, cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* s3, Result<cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple> ret = cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyMultiple(*s1, *s2, *s3);
			Ok(new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createSelectiveSearchSegmentationStrategyMultiple(Ptr<SelectiveSearchSegmentationStrategy>, Ptr<SelectiveSearchSegmentationStrategy>, Ptr<SelectiveSearchSegmentationStrategy>, Ptr<SelectiveSearchSegmentationStrategy>)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:176
	// ("cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyMultiple", vec![(pred!(mut, ["s1", "s2", "s3", "s4"], ["cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>", "cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>", "cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>", "cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>"]), _)]),
	void cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyMultiple_PtrLSelectiveSearchSegmentationStrategyG_PtrLSelectiveSearchSegmentationStrategyG_PtrLSelectiveSearchSegmentationStrategyG_PtrLSelectiveSearchSegmentationStrategyG(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* s1, cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* s2, cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* s3, cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* s4, Result<cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple> ret = cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyMultiple(*s1, *s2, *s3, *s4);
			Ok(new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createSelectiveSearchSegmentationStrategySize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:113
	// ("cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategySize", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategySize(Result<cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize> ret = cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategySize();
			Ok(new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createSelectiveSearchSegmentationStrategyTexture()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:122
	// ("cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyTexture", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyTexture(Result<cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture> ret = cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyTexture();
			Ok(new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::thinning(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc.hpp:190
	// ("cv::ximgproc::thinning", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_thinning_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::thinning(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// thinning(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc.hpp:190
	// ("cv::ximgproc::thinning", vec![(pred!(mut, ["src", "dst", "thinningType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_ximgproc_thinning_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int thinningType, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::thinning(*src, *dst, thinningType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::transformFD(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/fourier_descriptors.hpp:97
	// ("cv::ximgproc::transformFD", vec![(pred!(mut, ["src", "t", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_transformFD_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_InputArray* t, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::transformFD(*src, *t, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// transformFD(InputArray, InputArray, OutputArray, bool)(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/fourier_descriptors.hpp:97
	// ("cv::ximgproc::transformFD", vec![(pred!(mut, ["src", "t", "dst", "fdContour"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "bool"]), _)]),
	void cv_ximgproc_transformFD_const__InputArrayR_const__InputArrayR_const__OutputArrayR_bool(const cv::_InputArray* src, const cv::_InputArray* t, const cv::_OutputArray* dst, bool fdContour, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::transformFD(*src, *t, *dst, fdContour);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::weightedMedianFilter(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/weighted_median_filter.hpp:90
	// ("cv::ximgproc::weightedMedianFilter", vec![(pred!(mut, ["joint", "src", "dst", "r"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_ximgproc_weightedMedianFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* joint, const cv::_InputArray* src, const cv::_OutputArray* dst, int r, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::weightedMedianFilter(*joint, *src, *dst, r);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// weightedMedianFilter(InputArray, InputArray, OutputArray, int, double, int, InputArray)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/weighted_median_filter.hpp:90
	// ("cv::ximgproc::weightedMedianFilter", vec![(pred!(mut, ["joint", "src", "dst", "r", "sigma", "weightType", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "int", "const cv::_InputArray*"]), _)]),
	void cv_ximgproc_weightedMedianFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_int_const__InputArrayR(const cv::_InputArray* joint, const cv::_InputArray* src, const cv::_OutputArray* dst, int r, double sigma, int weightType, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			cv::ximgproc::weightedMedianFilter(*joint, *src, *dst, r, sigma, weightType, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// filter(InputArray, OutputArray, InputArray)(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:220
	// ("cv::ximgproc::AdaptiveManifoldFilter::filter", vec![(pred!(mut, ["src", "dst", "joint"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_ximgproc_AdaptiveManifoldFilter_filter_const__InputArrayR_const__OutputArrayR_const__InputArrayR(cv::ximgproc::AdaptiveManifoldFilter* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* joint, ResultVoid* ocvrs_return) {
		try {
			instance->filter(*src, *dst, *joint);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::AdaptiveManifoldFilter::filter(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:220
	// ("cv::ximgproc::AdaptiveManifoldFilter::filter", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_AdaptiveManifoldFilter_filter_const__InputArrayR_const__OutputArrayR(cv::ximgproc::AdaptiveManifoldFilter* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->filter(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// collectGarbage()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:222
	// ("cv::ximgproc::AdaptiveManifoldFilter::collectGarbage", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_AdaptiveManifoldFilter_collectGarbage(cv::ximgproc::AdaptiveManifoldFilter* instance, ResultVoid* ocvrs_return) {
		try {
			instance->collectGarbage();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:224
	// ("cv::ximgproc::AdaptiveManifoldFilter::create", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_AdaptiveManifoldFilter_create(Result<cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter> ret = cv::ximgproc::AdaptiveManifoldFilter::create();
			Ok(new cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSigmaS()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:227
	// ("cv::ximgproc::AdaptiveManifoldFilter::getSigmaS", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_AdaptiveManifoldFilter_getSigmaS_const(const cv::ximgproc::AdaptiveManifoldFilter* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSigmaS();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSigmaS(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:229
	// ("cv::ximgproc::AdaptiveManifoldFilter::setSigmaS", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_ximgproc_AdaptiveManifoldFilter_setSigmaS_double(cv::ximgproc::AdaptiveManifoldFilter* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setSigmaS(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSigmaR()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:231
	// ("cv::ximgproc::AdaptiveManifoldFilter::getSigmaR", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_AdaptiveManifoldFilter_getSigmaR_const(const cv::ximgproc::AdaptiveManifoldFilter* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSigmaR();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSigmaR(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:233
	// ("cv::ximgproc::AdaptiveManifoldFilter::setSigmaR", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_ximgproc_AdaptiveManifoldFilter_setSigmaR_double(cv::ximgproc::AdaptiveManifoldFilter* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setSigmaR(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTreeHeight()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:235
	// ("cv::ximgproc::AdaptiveManifoldFilter::getTreeHeight", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_AdaptiveManifoldFilter_getTreeHeight_const(const cv::ximgproc::AdaptiveManifoldFilter* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getTreeHeight();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTreeHeight(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:237
	// ("cv::ximgproc::AdaptiveManifoldFilter::setTreeHeight", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_ximgproc_AdaptiveManifoldFilter_setTreeHeight_int(cv::ximgproc::AdaptiveManifoldFilter* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setTreeHeight(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPCAIterations()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:239
	// ("cv::ximgproc::AdaptiveManifoldFilter::getPCAIterations", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_AdaptiveManifoldFilter_getPCAIterations_const(const cv::ximgproc::AdaptiveManifoldFilter* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPCAIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPCAIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:241
	// ("cv::ximgproc::AdaptiveManifoldFilter::setPCAIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_ximgproc_AdaptiveManifoldFilter_setPCAIterations_int(cv::ximgproc::AdaptiveManifoldFilter* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setPCAIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAdjustOutliers()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:243
	// ("cv::ximgproc::AdaptiveManifoldFilter::getAdjustOutliers", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_AdaptiveManifoldFilter_getAdjustOutliers_const(const cv::ximgproc::AdaptiveManifoldFilter* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getAdjustOutliers();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setAdjustOutliers(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:245
	// ("cv::ximgproc::AdaptiveManifoldFilter::setAdjustOutliers", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	void cv_ximgproc_AdaptiveManifoldFilter_setAdjustOutliers_bool(cv::ximgproc::AdaptiveManifoldFilter* instance, bool val, ResultVoid* ocvrs_return) {
		try {
			instance->setAdjustOutliers(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUseRNG()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:247
	// ("cv::ximgproc::AdaptiveManifoldFilter::getUseRNG", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_AdaptiveManifoldFilter_getUseRNG_const(const cv::ximgproc::AdaptiveManifoldFilter* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseRNG();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseRNG(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:249
	// ("cv::ximgproc::AdaptiveManifoldFilter::setUseRNG", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	void cv_ximgproc_AdaptiveManifoldFilter_setUseRNG_bool(cv::ximgproc::AdaptiveManifoldFilter* instance, bool val, ResultVoid* ocvrs_return) {
		try {
			instance->setUseRNG(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::AdaptiveManifoldFilter::to_Algorithm() generated
	// ("cv::ximgproc::AdaptiveManifoldFilter::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ximgproc_AdaptiveManifoldFilter_to_Algorithm(cv::ximgproc::AdaptiveManifoldFilter* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ximgproc::AdaptiveManifoldFilter::delete() generated
	// ("cv::ximgproc::AdaptiveManifoldFilter::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_AdaptiveManifoldFilter_delete(cv::ximgproc::AdaptiveManifoldFilter* instance) {
			delete instance;
	}

	// ContourFitting(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/fourier_descriptors.hpp:38
	// ("cv::ximgproc::ContourFitting::ContourFitting", vec![(pred!(mut, ["ctr", "fd"], ["int", "int"]), _)]),
	void cv_ximgproc_ContourFitting_ContourFitting_int_int(int ctr, int fd, Result<cv::ximgproc::ContourFitting*>* ocvrs_return) {
		try {
			cv::ximgproc::ContourFitting* ret = new cv::ximgproc::ContourFitting(ctr, fd);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::ContourFitting::ContourFitting() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/fourier_descriptors.hpp:38
	// ("cv::ximgproc::ContourFitting::ContourFitting", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_ContourFitting_ContourFitting(Result<cv::ximgproc::ContourFitting*>* ocvrs_return) {
		try {
			cv::ximgproc::ContourFitting* ret = new cv::ximgproc::ContourFitting();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimateTransformation(InputArray, InputArray, OutputArray, double *, bool)(InputArray, InputArray, OutputArray, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/fourier_descriptors.hpp:47
	// ("cv::ximgproc::ContourFitting::estimateTransformation", vec![(pred!(mut, ["src", "dst", "alphaPhiST", "dist", "fdContour"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double*", "bool"]), _)]),
	void cv_ximgproc_ContourFitting_estimateTransformation_const__InputArrayR_const__InputArrayR_const__OutputArrayR_doubleX_bool(cv::ximgproc::ContourFitting* instance, const cv::_InputArray* src, const cv::_InputArray* dst, const cv::_OutputArray* alphaPhiST, double* dist, bool fdContour, ResultVoid* ocvrs_return) {
		try {
			instance->estimateTransformation(*src, *dst, *alphaPhiST, dist, fdContour);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::ContourFitting::estimateTransformation(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/fourier_descriptors.hpp:47
	// ("cv::ximgproc::ContourFitting::estimateTransformation", vec![(pred!(mut, ["src", "dst", "alphaPhiST"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_ContourFitting_estimateTransformation_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::ximgproc::ContourFitting* instance, const cv::_InputArray* src, const cv::_InputArray* dst, const cv::_OutputArray* alphaPhiST, ResultVoid* ocvrs_return) {
		try {
			instance->estimateTransformation(*src, *dst, *alphaPhiST);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimateTransformation(InputArray, InputArray, OutputArray, double &, bool)(InputArray, InputArray, OutputArray, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/fourier_descriptors.hpp:56
	// ("cv::ximgproc::ContourFitting::estimateTransformation", vec![(pred!(mut, ["src", "dst", "alphaPhiST", "dist", "fdContour"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double*", "bool"]), _)]),
	void cv_ximgproc_ContourFitting_estimateTransformation_const__InputArrayR_const__InputArrayR_const__OutputArrayR_doubleR_bool(cv::ximgproc::ContourFitting* instance, const cv::_InputArray* src, const cv::_InputArray* dst, const cv::_OutputArray* alphaPhiST, double* dist, bool fdContour, ResultVoid* ocvrs_return) {
		try {
			instance->estimateTransformation(*src, *dst, *alphaPhiST, *dist, fdContour);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::ContourFitting::estimateTransformation(InputArray, InputArray, OutputArray, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/fourier_descriptors.hpp:56
	// ("cv::ximgproc::ContourFitting::estimateTransformation", vec![(pred!(mut, ["src", "dst", "alphaPhiST", "dist"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double*"]), _)]),
	void cv_ximgproc_ContourFitting_estimateTransformation_const__InputArrayR_const__InputArrayR_const__OutputArrayR_doubleR(cv::ximgproc::ContourFitting* instance, const cv::_InputArray* src, const cv::_InputArray* dst, const cv::_OutputArray* alphaPhiST, double* dist, ResultVoid* ocvrs_return) {
		try {
			instance->estimateTransformation(*src, *dst, *alphaPhiST, *dist);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCtrSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/fourier_descriptors.hpp:61
	// ("cv::ximgproc::ContourFitting::setCtrSize", vec![(pred!(mut, ["n"], ["int"]), _)]),
	void cv_ximgproc_ContourFitting_setCtrSize_int(cv::ximgproc::ContourFitting* instance, int n, ResultVoid* ocvrs_return) {
		try {
			instance->setCtrSize(n);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFDSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/fourier_descriptors.hpp:66
	// ("cv::ximgproc::ContourFitting::setFDSize", vec![(pred!(mut, ["n"], ["int"]), _)]),
	void cv_ximgproc_ContourFitting_setFDSize_int(cv::ximgproc::ContourFitting* instance, int n, ResultVoid* ocvrs_return) {
		try {
			instance->setFDSize(n);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCtrSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/fourier_descriptors.hpp:70
	// ("cv::ximgproc::ContourFitting::getCtrSize", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_ContourFitting_getCtrSize(cv::ximgproc::ContourFitting* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getCtrSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFDSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/fourier_descriptors.hpp:74
	// ("cv::ximgproc::ContourFitting::getFDSize", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_ContourFitting_getFDSize(cv::ximgproc::ContourFitting* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFDSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::ContourFitting::to_Algorithm() generated
	// ("cv::ximgproc::ContourFitting::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ximgproc_ContourFitting_to_Algorithm(cv::ximgproc::ContourFitting* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ximgproc::ContourFitting::delete() generated
	// ("cv::ximgproc::ContourFitting::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_ContourFitting_delete(cv::ximgproc::ContourFitting* instance) {
			delete instance;
	}

	// filter(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:79
	// ("cv::ximgproc::DTFilter::filter", vec![(pred!(mut, ["src", "dst", "dDepth"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_ximgproc_DTFilter_filter_const__InputArrayR_const__OutputArrayR_int(cv::ximgproc::DTFilter* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, int dDepth, ResultVoid* ocvrs_return) {
		try {
			instance->filter(*src, *dst, dDepth);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::DTFilter::filter(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:79
	// ("cv::ximgproc::DTFilter::filter", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_DTFilter_filter_const__InputArrayR_const__OutputArrayR(cv::ximgproc::DTFilter* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->filter(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::DTFilter::to_Algorithm() generated
	// ("cv::ximgproc::DTFilter::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ximgproc_DTFilter_to_Algorithm(cv::ximgproc::DTFilter* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ximgproc::DTFilter::delete() generated
	// ("cv::ximgproc::DTFilter::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_DTFilter_delete(cv::ximgproc::DTFilter* instance) {
			delete instance;
	}

	// filter(InputArray, InputArray, OutputArray, InputArray, Rect, InputArray)(InputArray, InputArray, OutputArray, InputArray, SimpleClass, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/disparity_filter.hpp:75
	// ("cv::ximgproc::DisparityFilter::filter", vec![(pred!(mut, ["disparity_map_left", "left_view", "filtered_disparity_map", "disparity_map_right", "ROI", "right_view"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::Rect", "const cv::_InputArray*"]), _)]),
	void cv_ximgproc_DisparityFilter_filter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Rect_const__InputArrayR(cv::ximgproc::DisparityFilter* instance, const cv::_InputArray* disparity_map_left, const cv::_InputArray* left_view, const cv::_OutputArray* filtered_disparity_map, const cv::_InputArray* disparity_map_right, cv::Rect* ROI, const cv::_InputArray* right_view, ResultVoid* ocvrs_return) {
		try {
			instance->filter(*disparity_map_left, *left_view, *filtered_disparity_map, *disparity_map_right, *ROI, *right_view);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::DisparityFilter::filter(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/disparity_filter.hpp:75
	// ("cv::ximgproc::DisparityFilter::filter", vec![(pred!(mut, ["disparity_map_left", "left_view", "filtered_disparity_map"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_DisparityFilter_filter_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::ximgproc::DisparityFilter* instance, const cv::_InputArray* disparity_map_left, const cv::_InputArray* left_view, const cv::_OutputArray* filtered_disparity_map, ResultVoid* ocvrs_return) {
		try {
			instance->filter(*disparity_map_left, *left_view, *filtered_disparity_map);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::DisparityFilter::to_DisparityWLSFilter() generated
	// ("cv::ximgproc::DisparityFilter::to_DisparityWLSFilter", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::DisparityWLSFilter* cv_ximgproc_DisparityFilter_to_DisparityWLSFilter(cv::ximgproc::DisparityFilter* instance) {
			return dynamic_cast<cv::ximgproc::DisparityWLSFilter*>(instance);
	}

	// cv::ximgproc::DisparityFilter::to_Algorithm() generated
	// ("cv::ximgproc::DisparityFilter::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ximgproc_DisparityFilter_to_Algorithm(cv::ximgproc::DisparityFilter* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ximgproc::DisparityFilter::delete() generated
	// ("cv::ximgproc::DisparityFilter::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_DisparityFilter_delete(cv::ximgproc::DisparityFilter* instance) {
			delete instance;
	}

	// getLambda()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/disparity_filter.hpp:90
	// ("cv::ximgproc::DisparityWLSFilter::getLambda", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_DisparityWLSFilter_getLambda(cv::ximgproc::DisparityWLSFilter* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getLambda();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setLambda(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/disparity_filter.hpp:92
	// ("cv::ximgproc::DisparityWLSFilter::setLambda", vec![(pred!(mut, ["_lambda"], ["double"]), _)]),
	void cv_ximgproc_DisparityWLSFilter_setLambda_double(cv::ximgproc::DisparityWLSFilter* instance, double _lambda, ResultVoid* ocvrs_return) {
		try {
			instance->setLambda(_lambda);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSigmaColor()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/disparity_filter.hpp:97
	// ("cv::ximgproc::DisparityWLSFilter::getSigmaColor", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_DisparityWLSFilter_getSigmaColor(cv::ximgproc::DisparityWLSFilter* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSigmaColor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSigmaColor(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/disparity_filter.hpp:99
	// ("cv::ximgproc::DisparityWLSFilter::setSigmaColor", vec![(pred!(mut, ["_sigma_color"], ["double"]), _)]),
	void cv_ximgproc_DisparityWLSFilter_setSigmaColor_double(cv::ximgproc::DisparityWLSFilter* instance, double _sigma_color, ResultVoid* ocvrs_return) {
		try {
			instance->setSigmaColor(_sigma_color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLRCthresh()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/disparity_filter.hpp:106
	// ("cv::ximgproc::DisparityWLSFilter::getLRCthresh", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_DisparityWLSFilter_getLRCthresh(cv::ximgproc::DisparityWLSFilter* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getLRCthresh();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setLRCthresh(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/disparity_filter.hpp:108
	// ("cv::ximgproc::DisparityWLSFilter::setLRCthresh", vec![(pred!(mut, ["_LRC_thresh"], ["int"]), _)]),
	void cv_ximgproc_DisparityWLSFilter_setLRCthresh_int(cv::ximgproc::DisparityWLSFilter* instance, int _LRC_thresh, ResultVoid* ocvrs_return) {
		try {
			instance->setLRCthresh(_LRC_thresh);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDepthDiscontinuityRadius()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/disparity_filter.hpp:112
	// ("cv::ximgproc::DisparityWLSFilter::getDepthDiscontinuityRadius", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_DisparityWLSFilter_getDepthDiscontinuityRadius(cv::ximgproc::DisparityWLSFilter* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDepthDiscontinuityRadius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDepthDiscontinuityRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/disparity_filter.hpp:114
	// ("cv::ximgproc::DisparityWLSFilter::setDepthDiscontinuityRadius", vec![(pred!(mut, ["_disc_radius"], ["int"]), _)]),
	void cv_ximgproc_DisparityWLSFilter_setDepthDiscontinuityRadius_int(cv::ximgproc::DisparityWLSFilter* instance, int _disc_radius, ResultVoid* ocvrs_return) {
		try {
			instance->setDepthDiscontinuityRadius(_disc_radius);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getConfidenceMap()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/disparity_filter.hpp:119
	// ("cv::ximgproc::DisparityWLSFilter::getConfidenceMap", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_DisparityWLSFilter_getConfidenceMap(cv::ximgproc::DisparityWLSFilter* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getConfidenceMap();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getROI()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/disparity_filter.hpp:122
	// ("cv::ximgproc::DisparityWLSFilter::getROI", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_DisparityWLSFilter_getROI(cv::ximgproc::DisparityWLSFilter* instance, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->getROI();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::DisparityWLSFilter::to_Algorithm() generated
	// ("cv::ximgproc::DisparityWLSFilter::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ximgproc_DisparityWLSFilter_to_Algorithm(cv::ximgproc::DisparityWLSFilter* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ximgproc::DisparityWLSFilter::to_DisparityFilter() generated
	// ("cv::ximgproc::DisparityWLSFilter::to_DisparityFilter", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::DisparityFilter* cv_ximgproc_DisparityWLSFilter_to_DisparityFilter(cv::ximgproc::DisparityWLSFilter* instance) {
			return dynamic_cast<cv::ximgproc::DisparityFilter*>(instance);
	}

	// cv::ximgproc::DisparityWLSFilter::delete() generated
	// ("cv::ximgproc::DisparityWLSFilter::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_DisparityWLSFilter_delete(cv::ximgproc::DisparityWLSFilter* instance) {
			delete instance;
	}

	// setCostMap(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:88
	// ("cv::ximgproc::EdgeAwareInterpolator::setCostMap", vec![(pred!(mut, ["_costMap"], ["const cv::Mat*"]), _)]),
	void cv_ximgproc_EdgeAwareInterpolator_setCostMap_const_MatR(cv::ximgproc::EdgeAwareInterpolator* instance, const cv::Mat* _costMap, ResultVoid* ocvrs_return) {
		try {
			instance->setCostMap(*_costMap);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setK(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:96
	// ("cv::ximgproc::EdgeAwareInterpolator::setK", vec![(pred!(mut, ["_k"], ["int"]), _)]),
	void cv_ximgproc_EdgeAwareInterpolator_setK_int(cv::ximgproc::EdgeAwareInterpolator* instance, int _k, ResultVoid* ocvrs_return) {
		try {
			instance->setK(_k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getK()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:98
	// ("cv::ximgproc::EdgeAwareInterpolator::getK", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_EdgeAwareInterpolator_getK(cv::ximgproc::EdgeAwareInterpolator* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getK();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSigma(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:104
	// ("cv::ximgproc::EdgeAwareInterpolator::setSigma", vec![(pred!(mut, ["_sigma"], ["float"]), _)]),
	void cv_ximgproc_EdgeAwareInterpolator_setSigma_float(cv::ximgproc::EdgeAwareInterpolator* instance, float _sigma, ResultVoid* ocvrs_return) {
		try {
			instance->setSigma(_sigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSigma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:106
	// ("cv::ximgproc::EdgeAwareInterpolator::getSigma", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_EdgeAwareInterpolator_getSigma(cv::ximgproc::EdgeAwareInterpolator* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setLambda(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:111
	// ("cv::ximgproc::EdgeAwareInterpolator::setLambda", vec![(pred!(mut, ["_lambda"], ["float"]), _)]),
	void cv_ximgproc_EdgeAwareInterpolator_setLambda_float(cv::ximgproc::EdgeAwareInterpolator* instance, float _lambda, ResultVoid* ocvrs_return) {
		try {
			instance->setLambda(_lambda);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLambda()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:113
	// ("cv::ximgproc::EdgeAwareInterpolator::getLambda", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_EdgeAwareInterpolator_getLambda(cv::ximgproc::EdgeAwareInterpolator* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getLambda();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUsePostProcessing(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:118
	// ("cv::ximgproc::EdgeAwareInterpolator::setUsePostProcessing", vec![(pred!(mut, ["_use_post_proc"], ["bool"]), _)]),
	void cv_ximgproc_EdgeAwareInterpolator_setUsePostProcessing_bool(cv::ximgproc::EdgeAwareInterpolator* instance, bool _use_post_proc, ResultVoid* ocvrs_return) {
		try {
			instance->setUsePostProcessing(_use_post_proc);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUsePostProcessing()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:120
	// ("cv::ximgproc::EdgeAwareInterpolator::getUsePostProcessing", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_EdgeAwareInterpolator_getUsePostProcessing(cv::ximgproc::EdgeAwareInterpolator* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUsePostProcessing();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFGSLambda(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:124
	// ("cv::ximgproc::EdgeAwareInterpolator::setFGSLambda", vec![(pred!(mut, ["_lambda"], ["float"]), _)]),
	void cv_ximgproc_EdgeAwareInterpolator_setFGSLambda_float(cv::ximgproc::EdgeAwareInterpolator* instance, float _lambda, ResultVoid* ocvrs_return) {
		try {
			instance->setFGSLambda(_lambda);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFGSLambda()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:126
	// ("cv::ximgproc::EdgeAwareInterpolator::getFGSLambda", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_EdgeAwareInterpolator_getFGSLambda(cv::ximgproc::EdgeAwareInterpolator* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getFGSLambda();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFGSSigma(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:129
	// ("cv::ximgproc::EdgeAwareInterpolator::setFGSSigma", vec![(pred!(mut, ["_sigma"], ["float"]), _)]),
	void cv_ximgproc_EdgeAwareInterpolator_setFGSSigma_float(cv::ximgproc::EdgeAwareInterpolator* instance, float _sigma, ResultVoid* ocvrs_return) {
		try {
			instance->setFGSSigma(_sigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFGSSigma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:131
	// ("cv::ximgproc::EdgeAwareInterpolator::getFGSSigma", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_EdgeAwareInterpolator_getFGSSigma(cv::ximgproc::EdgeAwareInterpolator* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getFGSSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::EdgeAwareInterpolator::to_Algorithm() generated
	// ("cv::ximgproc::EdgeAwareInterpolator::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ximgproc_EdgeAwareInterpolator_to_Algorithm(cv::ximgproc::EdgeAwareInterpolator* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ximgproc::EdgeAwareInterpolator::to_SparseMatchInterpolator() generated
	// ("cv::ximgproc::EdgeAwareInterpolator::to_SparseMatchInterpolator", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::SparseMatchInterpolator* cv_ximgproc_EdgeAwareInterpolator_to_SparseMatchInterpolator(cv::ximgproc::EdgeAwareInterpolator* instance) {
			return dynamic_cast<cv::ximgproc::SparseMatchInterpolator*>(instance);
	}

	// cv::ximgproc::EdgeAwareInterpolator::delete() generated
	// ("cv::ximgproc::EdgeAwareInterpolator::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_EdgeAwareInterpolator_delete(cv::ximgproc::EdgeAwareInterpolator* instance) {
			delete instance;
	}

	// getBoundingBoxes(InputArray, InputArray, std::vector<Rect> &, OutputArray)(InputArray, InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edgeboxes.hpp:79
	// ("cv::ximgproc::EdgeBoxes::getBoundingBoxes", vec![(pred!(mut, ["edge_map", "orientation_map", "boxes", "scores"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::Rect>*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_EdgeBoxes_getBoundingBoxes_const__InputArrayR_const__InputArrayR_vectorLRectGR_const__OutputArrayR(cv::ximgproc::EdgeBoxes* instance, const cv::_InputArray* edge_map, const cv::_InputArray* orientation_map, std::vector<cv::Rect>* boxes, const cv::_OutputArray* scores, ResultVoid* ocvrs_return) {
		try {
			instance->getBoundingBoxes(*edge_map, *orientation_map, *boxes, *scores);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::EdgeBoxes::getBoundingBoxes(InputArray, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edgeboxes.hpp:79
	// ("cv::ximgproc::EdgeBoxes::getBoundingBoxes", vec![(pred!(mut, ["edge_map", "orientation_map", "boxes"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::Rect>*"]), _)]),
	void cv_ximgproc_EdgeBoxes_getBoundingBoxes_const__InputArrayR_const__InputArrayR_vectorLRectGR(cv::ximgproc::EdgeBoxes* instance, const cv::_InputArray* edge_map, const cv::_InputArray* orientation_map, std::vector<cv::Rect>* boxes, ResultVoid* ocvrs_return) {
		try {
			instance->getBoundingBoxes(*edge_map, *orientation_map, *boxes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAlpha()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edgeboxes.hpp:83
	// ("cv::ximgproc::EdgeBoxes::getAlpha", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_EdgeBoxes_getAlpha_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getAlpha();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setAlpha(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edgeboxes.hpp:86
	// ("cv::ximgproc::EdgeBoxes::setAlpha", vec![(pred!(mut, ["value"], ["float"]), _)]),
	void cv_ximgproc_EdgeBoxes_setAlpha_float(cv::ximgproc::EdgeBoxes* instance, float value, ResultVoid* ocvrs_return) {
		try {
			instance->setAlpha(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBeta()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edgeboxes.hpp:90
	// ("cv::ximgproc::EdgeBoxes::getBeta", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_EdgeBoxes_getBeta_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getBeta();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBeta(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edgeboxes.hpp:93
	// ("cv::ximgproc::EdgeBoxes::setBeta", vec![(pred!(mut, ["value"], ["float"]), _)]),
	void cv_ximgproc_EdgeBoxes_setBeta_float(cv::ximgproc::EdgeBoxes* instance, float value, ResultVoid* ocvrs_return) {
		try {
			instance->setBeta(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEta()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edgeboxes.hpp:97
	// ("cv::ximgproc::EdgeBoxes::getEta", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_EdgeBoxes_getEta_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getEta();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEta(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edgeboxes.hpp:100
	// ("cv::ximgproc::EdgeBoxes::setEta", vec![(pred!(mut, ["value"], ["float"]), _)]),
	void cv_ximgproc_EdgeBoxes_setEta_float(cv::ximgproc::EdgeBoxes* instance, float value, ResultVoid* ocvrs_return) {
		try {
			instance->setEta(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinScore()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edgeboxes.hpp:104
	// ("cv::ximgproc::EdgeBoxes::getMinScore", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_EdgeBoxes_getMinScore_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMinScore();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinScore(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edgeboxes.hpp:107
	// ("cv::ximgproc::EdgeBoxes::setMinScore", vec![(pred!(mut, ["value"], ["float"]), _)]),
	void cv_ximgproc_EdgeBoxes_setMinScore_float(cv::ximgproc::EdgeBoxes* instance, float value, ResultVoid* ocvrs_return) {
		try {
			instance->setMinScore(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxBoxes()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edgeboxes.hpp:111
	// ("cv::ximgproc::EdgeBoxes::getMaxBoxes", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_EdgeBoxes_getMaxBoxes_const(const cv::ximgproc::EdgeBoxes* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxBoxes();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxBoxes(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edgeboxes.hpp:114
	// ("cv::ximgproc::EdgeBoxes::setMaxBoxes", vec![(pred!(mut, ["value"], ["int"]), _)]),
	void cv_ximgproc_EdgeBoxes_setMaxBoxes_int(cv::ximgproc::EdgeBoxes* instance, int value, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxBoxes(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEdgeMinMag()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edgeboxes.hpp:118
	// ("cv::ximgproc::EdgeBoxes::getEdgeMinMag", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_EdgeBoxes_getEdgeMinMag_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getEdgeMinMag();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEdgeMinMag(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edgeboxes.hpp:121
	// ("cv::ximgproc::EdgeBoxes::setEdgeMinMag", vec![(pred!(mut, ["value"], ["float"]), _)]),
	void cv_ximgproc_EdgeBoxes_setEdgeMinMag_float(cv::ximgproc::EdgeBoxes* instance, float value, ResultVoid* ocvrs_return) {
		try {
			instance->setEdgeMinMag(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEdgeMergeThr()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edgeboxes.hpp:125
	// ("cv::ximgproc::EdgeBoxes::getEdgeMergeThr", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_EdgeBoxes_getEdgeMergeThr_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getEdgeMergeThr();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEdgeMergeThr(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edgeboxes.hpp:128
	// ("cv::ximgproc::EdgeBoxes::setEdgeMergeThr", vec![(pred!(mut, ["value"], ["float"]), _)]),
	void cv_ximgproc_EdgeBoxes_setEdgeMergeThr_float(cv::ximgproc::EdgeBoxes* instance, float value, ResultVoid* ocvrs_return) {
		try {
			instance->setEdgeMergeThr(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getClusterMinMag()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edgeboxes.hpp:132
	// ("cv::ximgproc::EdgeBoxes::getClusterMinMag", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_EdgeBoxes_getClusterMinMag_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getClusterMinMag();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setClusterMinMag(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edgeboxes.hpp:135
	// ("cv::ximgproc::EdgeBoxes::setClusterMinMag", vec![(pred!(mut, ["value"], ["float"]), _)]),
	void cv_ximgproc_EdgeBoxes_setClusterMinMag_float(cv::ximgproc::EdgeBoxes* instance, float value, ResultVoid* ocvrs_return) {
		try {
			instance->setClusterMinMag(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxAspectRatio()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edgeboxes.hpp:139
	// ("cv::ximgproc::EdgeBoxes::getMaxAspectRatio", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_EdgeBoxes_getMaxAspectRatio_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMaxAspectRatio();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxAspectRatio(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edgeboxes.hpp:142
	// ("cv::ximgproc::EdgeBoxes::setMaxAspectRatio", vec![(pred!(mut, ["value"], ["float"]), _)]),
	void cv_ximgproc_EdgeBoxes_setMaxAspectRatio_float(cv::ximgproc::EdgeBoxes* instance, float value, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxAspectRatio(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinBoxArea()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edgeboxes.hpp:146
	// ("cv::ximgproc::EdgeBoxes::getMinBoxArea", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_EdgeBoxes_getMinBoxArea_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMinBoxArea();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinBoxArea(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edgeboxes.hpp:149
	// ("cv::ximgproc::EdgeBoxes::setMinBoxArea", vec![(pred!(mut, ["value"], ["float"]), _)]),
	void cv_ximgproc_EdgeBoxes_setMinBoxArea_float(cv::ximgproc::EdgeBoxes* instance, float value, ResultVoid* ocvrs_return) {
		try {
			instance->setMinBoxArea(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGamma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edgeboxes.hpp:153
	// ("cv::ximgproc::EdgeBoxes::getGamma", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_EdgeBoxes_getGamma_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getGamma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setGamma(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edgeboxes.hpp:156
	// ("cv::ximgproc::EdgeBoxes::setGamma", vec![(pred!(mut, ["value"], ["float"]), _)]),
	void cv_ximgproc_EdgeBoxes_setGamma_float(cv::ximgproc::EdgeBoxes* instance, float value, ResultVoid* ocvrs_return) {
		try {
			instance->setGamma(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getKappa()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edgeboxes.hpp:160
	// ("cv::ximgproc::EdgeBoxes::getKappa", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_EdgeBoxes_getKappa_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getKappa();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setKappa(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edgeboxes.hpp:163
	// ("cv::ximgproc::EdgeBoxes::setKappa", vec![(pred!(mut, ["value"], ["float"]), _)]),
	void cv_ximgproc_EdgeBoxes_setKappa_float(cv::ximgproc::EdgeBoxes* instance, float value, ResultVoid* ocvrs_return) {
		try {
			instance->setKappa(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::EdgeBoxes::to_Algorithm() generated
	// ("cv::ximgproc::EdgeBoxes::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ximgproc_EdgeBoxes_to_Algorithm(cv::ximgproc::EdgeBoxes* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ximgproc::EdgeBoxes::delete() generated
	// ("cv::ximgproc::EdgeBoxes::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_EdgeBoxes_delete(cv::ximgproc::EdgeBoxes* instance) {
			delete instance;
	}

	// detectEdges(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_drawing.hpp:77
	// ("cv::ximgproc::EdgeDrawing::detectEdges", vec![(pred!(mut, ["src"], ["const cv::_InputArray*"]), _)]),
	void cv_ximgproc_EdgeDrawing_detectEdges_const__InputArrayR(cv::ximgproc::EdgeDrawing* instance, const cv::_InputArray* src, ResultVoid* ocvrs_return) {
		try {
			instance->detectEdges(*src);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEdgeImage(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_drawing.hpp:83
	// ("cv::ximgproc::EdgeDrawing::getEdgeImage", vec![(pred!(mut, ["dst"], ["const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_EdgeDrawing_getEdgeImage_const__OutputArrayR(cv::ximgproc::EdgeDrawing* instance, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->getEdgeImage(*dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGradientImage(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_drawing.hpp:89
	// ("cv::ximgproc::EdgeDrawing::getGradientImage", vec![(pred!(mut, ["dst"], ["const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_EdgeDrawing_getGradientImage_const__OutputArrayR(cv::ximgproc::EdgeDrawing* instance, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->getGradientImage(*dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSegments()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_drawing.hpp:93
	// ("cv::ximgproc::EdgeDrawing::getSegments", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_EdgeDrawing_getSegments(cv::ximgproc::EdgeDrawing* instance, Result<std::vector<std::vector<cv::Point>>*>* ocvrs_return) {
		try {
			std::vector<std::vector<cv::Point>> ret = instance->getSegments();
			Ok(new std::vector<std::vector<cv::Point>>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSegmentIndicesOfLines()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_drawing.hpp:97
	// ("cv::ximgproc::EdgeDrawing::getSegmentIndicesOfLines", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_EdgeDrawing_getSegmentIndicesOfLines_const(const cv::ximgproc::EdgeDrawing* instance, Result<std::vector<int>*>* ocvrs_return) {
		try {
			std::vector<int> ret = instance->getSegmentIndicesOfLines();
			Ok(new std::vector<int>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectLines(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_drawing.hpp:104
	// ("cv::ximgproc::EdgeDrawing::detectLines", vec![(pred!(mut, ["lines"], ["const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_EdgeDrawing_detectLines_const__OutputArrayR(cv::ximgproc::EdgeDrawing* instance, const cv::_OutputArray* lines, ResultVoid* ocvrs_return) {
		try {
			instance->detectLines(*lines);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectEllipses(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_drawing.hpp:111
	// ("cv::ximgproc::EdgeDrawing::detectEllipses", vec![(pred!(mut, ["ellipses"], ["const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_EdgeDrawing_detectEllipses_const__OutputArrayR(cv::ximgproc::EdgeDrawing* instance, const cv::_OutputArray* ellipses, ResultVoid* ocvrs_return) {
		try {
			instance->detectEllipses(*ellipses);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setParams(const EdgeDrawing::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_drawing.hpp:120
	// ("cv::ximgproc::EdgeDrawing::setParams", vec![(pred!(mut, ["parameters"], ["const cv::ximgproc::EdgeDrawing::Params*"]), _)]),
	void cv_ximgproc_EdgeDrawing_setParams_const_ParamsR(cv::ximgproc::EdgeDrawing* instance, const cv::ximgproc::EdgeDrawing::Params* parameters, ResultVoid* ocvrs_return) {
		try {
			instance->setParams(*parameters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::EdgeDrawing::params() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_drawing.hpp:113
	// ("cv::ximgproc::EdgeDrawing::params", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_EdgeDrawing_propParams_const(const cv::ximgproc::EdgeDrawing* instance, cv::ximgproc::EdgeDrawing::Params* ocvrs_return) {
			cv::ximgproc::EdgeDrawing::Params ret = instance->params;
			*ocvrs_return = ret;
	}

	// cv::ximgproc::EdgeDrawing::setParams(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_drawing.hpp:113
	// ("cv::ximgproc::EdgeDrawing::setParams", vec![(pred!(mut, ["val"], ["const cv::ximgproc::EdgeDrawing::Params"]), _)]),
	void cv_ximgproc_EdgeDrawing_propParams_const_Params(cv::ximgproc::EdgeDrawing* instance, const cv::ximgproc::EdgeDrawing::Params* val) {
			instance->params = *val;
	}

	// cv::ximgproc::EdgeDrawing::to_Algorithm() generated
	// ("cv::ximgproc::EdgeDrawing::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ximgproc_EdgeDrawing_to_Algorithm(cv::ximgproc::EdgeDrawing* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ximgproc::EdgeDrawing::delete() generated
	// ("cv::ximgproc::EdgeDrawing::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_EdgeDrawing_delete(cv::ximgproc::EdgeDrawing* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_drawing.hpp:35
	// ("cv::ximgproc::EdgeDrawing::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_EdgeDrawing_Params_Params(Result<cv::ximgproc::EdgeDrawing::Params>* ocvrs_return) {
		try {
			cv::ximgproc::EdgeDrawing::Params ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_drawing.hpp:69
	// ("cv::ximgproc::EdgeDrawing::Params::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_ximgproc_EdgeDrawing_Params_read_const_FileNodeR(cv::ximgproc::EdgeDrawing::Params* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_drawing.hpp:70
	// ("cv::ximgproc::EdgeDrawing::Params::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_ximgproc_EdgeDrawing_Params_write_const_FileStorageR(const cv::ximgproc::EdgeDrawing::Params* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// filter(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:401
	// ("cv::ximgproc::FastBilateralSolverFilter::filter", vec![(pred!(mut, ["src", "confidence", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_FastBilateralSolverFilter_filter_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::ximgproc::FastBilateralSolverFilter* instance, const cv::_InputArray* src, const cv::_InputArray* confidence, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->filter(*src, *confidence, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::FastBilateralSolverFilter::to_Algorithm() generated
	// ("cv::ximgproc::FastBilateralSolverFilter::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ximgproc_FastBilateralSolverFilter_to_Algorithm(cv::ximgproc::FastBilateralSolverFilter* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ximgproc::FastBilateralSolverFilter::delete() generated
	// ("cv::ximgproc::FastBilateralSolverFilter::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_FastBilateralSolverFilter_delete(cv::ximgproc::FastBilateralSolverFilter* instance) {
			delete instance;
	}

	// filter(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:472
	// ("cv::ximgproc::FastGlobalSmootherFilter::filter", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_FastGlobalSmootherFilter_filter_const__InputArrayR_const__OutputArrayR(cv::ximgproc::FastGlobalSmootherFilter* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->filter(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::FastGlobalSmootherFilter::to_Algorithm() generated
	// ("cv::ximgproc::FastGlobalSmootherFilter::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ximgproc_FastGlobalSmootherFilter_to_Algorithm(cv::ximgproc::FastGlobalSmootherFilter* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ximgproc::FastGlobalSmootherFilter::delete() generated
	// ("cv::ximgproc::FastGlobalSmootherFilter::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_FastGlobalSmootherFilter_delete(cv::ximgproc::FastGlobalSmootherFilter* instance) {
			delete instance;
	}

	// detect(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/fast_line_detector.hpp:44
	// ("cv::ximgproc::FastLineDetector::detect", vec![(pred!(mut, ["image", "lines"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_FastLineDetector_detect_const__InputArrayR_const__OutputArrayR(cv::ximgproc::FastLineDetector* instance, const cv::_InputArray* image, const cv::_OutputArray* lines, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*image, *lines);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawSegments(InputOutputArray, InputArray, bool, Scalar, int)(InputOutputArray, InputArray, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/fast_line_detector.hpp:54
	// ("cv::ximgproc::FastLineDetector::drawSegments", vec![(pred!(mut, ["image", "lines", "draw_arrow", "linecolor", "linethickness"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "bool", "cv::Scalar", "int"]), _)]),
	void cv_ximgproc_FastLineDetector_drawSegments_const__InputOutputArrayR_const__InputArrayR_bool_Scalar_int(cv::ximgproc::FastLineDetector* instance, const cv::_InputOutputArray* image, const cv::_InputArray* lines, bool draw_arrow, cv::Scalar* linecolor, int linethickness, ResultVoid* ocvrs_return) {
		try {
			instance->drawSegments(*image, *lines, draw_arrow, *linecolor, linethickness);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::FastLineDetector::drawSegments(InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/fast_line_detector.hpp:54
	// ("cv::ximgproc::FastLineDetector::drawSegments", vec![(pred!(mut, ["image", "lines"], ["const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_ximgproc_FastLineDetector_drawSegments_const__InputOutputArrayR_const__InputArrayR(cv::ximgproc::FastLineDetector* instance, const cv::_InputOutputArray* image, const cv::_InputArray* lines, ResultVoid* ocvrs_return) {
		try {
			instance->drawSegments(*image, *lines);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::FastLineDetector::to_Algorithm() generated
	// ("cv::ximgproc::FastLineDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ximgproc_FastLineDetector_to_Algorithm(cv::ximgproc::FastLineDetector* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ximgproc::FastLineDetector::delete() generated
	// ("cv::ximgproc::FastLineDetector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_FastLineDetector_delete(cv::ximgproc::FastLineDetector* instance) {
			delete instance;
	}

	// filter(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:143
	// ("cv::ximgproc::GuidedFilter::filter", vec![(pred!(mut, ["src", "dst", "dDepth"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_ximgproc_GuidedFilter_filter_const__InputArrayR_const__OutputArrayR_int(cv::ximgproc::GuidedFilter* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, int dDepth, ResultVoid* ocvrs_return) {
		try {
			instance->filter(*src, *dst, dDepth);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::GuidedFilter::filter(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/edge_filter.hpp:143
	// ("cv::ximgproc::GuidedFilter::filter", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_GuidedFilter_filter_const__InputArrayR_const__OutputArrayR(cv::ximgproc::GuidedFilter* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->filter(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::GuidedFilter::to_Algorithm() generated
	// ("cv::ximgproc::GuidedFilter::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ximgproc_GuidedFilter_to_Algorithm(cv::ximgproc::GuidedFilter* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ximgproc::GuidedFilter::delete() generated
	// ("cv::ximgproc::GuidedFilter::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_GuidedFilter_delete(cv::ximgproc::GuidedFilter* instance) {
			delete instance;
	}

	// getFeatures(const Mat &, Mat &, const int, const int, const int, const int, const int)(TraitClass, TraitClass, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/structured_edge_detection.hpp:83
	// ("cv::ximgproc::RFFeatureGetter::getFeatures", vec![(pred!(const, ["src", "features", "gnrmRad", "gsmthRad", "shrink", "outNum", "gradNum"], ["const cv::Mat*", "cv::Mat*", "const int", "const int", "const int", "const int", "const int"]), _)]),
	void cv_ximgproc_RFFeatureGetter_getFeatures_const_const_MatR_MatR_const_int_const_int_const_int_const_int_const_int(const cv::ximgproc::RFFeatureGetter* instance, const cv::Mat* src, cv::Mat* features, const int gnrmRad, const int gsmthRad, const int shrink, const int outNum, const int gradNum, ResultVoid* ocvrs_return) {
		try {
			instance->getFeatures(*src, *features, gnrmRad, gsmthRad, shrink, outNum, gradNum);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::RFFeatureGetter::to_Algorithm() generated
	// ("cv::ximgproc::RFFeatureGetter::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ximgproc_RFFeatureGetter_to_Algorithm(cv::ximgproc::RFFeatureGetter* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ximgproc::RFFeatureGetter::delete() generated
	// ("cv::ximgproc::RFFeatureGetter::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_RFFeatureGetter_delete(cv::ximgproc::RFFeatureGetter* instance) {
			delete instance;
	}

	// setK(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:153
	// ("cv::ximgproc::RICInterpolator::setK", vec![(pred!(mut, ["k"], ["int"]), _)]),
	void cv_ximgproc_RICInterpolator_setK_int(cv::ximgproc::RICInterpolator* instance, int k, ResultVoid* ocvrs_return) {
		try {
			instance->setK(k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::RICInterpolator::setK() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:153
	// ("cv::ximgproc::RICInterpolator::setK", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_RICInterpolator_setK(cv::ximgproc::RICInterpolator* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setK();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getK()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:157
	// ("cv::ximgproc::RICInterpolator::getK", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_RICInterpolator_getK_const(const cv::ximgproc::RICInterpolator* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getK();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCostMap(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:166
	// ("cv::ximgproc::RICInterpolator::setCostMap", vec![(pred!(mut, ["costMap"], ["const cv::Mat*"]), _)]),
	void cv_ximgproc_RICInterpolator_setCostMap_const_MatR(cv::ximgproc::RICInterpolator* instance, const cv::Mat* costMap, ResultVoid* ocvrs_return) {
		try {
			instance->setCostMap(*costMap);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSuperpixelSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:170
	// ("cv::ximgproc::RICInterpolator::setSuperpixelSize", vec![(pred!(mut, ["spSize"], ["int"]), _)]),
	void cv_ximgproc_RICInterpolator_setSuperpixelSize_int(cv::ximgproc::RICInterpolator* instance, int spSize, ResultVoid* ocvrs_return) {
		try {
			instance->setSuperpixelSize(spSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::RICInterpolator::setSuperpixelSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:170
	// ("cv::ximgproc::RICInterpolator::setSuperpixelSize", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_RICInterpolator_setSuperpixelSize(cv::ximgproc::RICInterpolator* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setSuperpixelSize();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSuperpixelSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:174
	// ("cv::ximgproc::RICInterpolator::getSuperpixelSize", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_RICInterpolator_getSuperpixelSize_const(const cv::ximgproc::RICInterpolator* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSuperpixelSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSuperpixelNNCnt(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:178
	// ("cv::ximgproc::RICInterpolator::setSuperpixelNNCnt", vec![(pred!(mut, ["spNN"], ["int"]), _)]),
	void cv_ximgproc_RICInterpolator_setSuperpixelNNCnt_int(cv::ximgproc::RICInterpolator* instance, int spNN, ResultVoid* ocvrs_return) {
		try {
			instance->setSuperpixelNNCnt(spNN);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::RICInterpolator::setSuperpixelNNCnt() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:178
	// ("cv::ximgproc::RICInterpolator::setSuperpixelNNCnt", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_RICInterpolator_setSuperpixelNNCnt(cv::ximgproc::RICInterpolator* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setSuperpixelNNCnt();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSuperpixelNNCnt()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:182
	// ("cv::ximgproc::RICInterpolator::getSuperpixelNNCnt", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_RICInterpolator_getSuperpixelNNCnt_const(const cv::ximgproc::RICInterpolator* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSuperpixelNNCnt();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSuperpixelRuler(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:186
	// ("cv::ximgproc::RICInterpolator::setSuperpixelRuler", vec![(pred!(mut, ["ruler"], ["float"]), _)]),
	void cv_ximgproc_RICInterpolator_setSuperpixelRuler_float(cv::ximgproc::RICInterpolator* instance, float ruler, ResultVoid* ocvrs_return) {
		try {
			instance->setSuperpixelRuler(ruler);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::RICInterpolator::setSuperpixelRuler() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:186
	// ("cv::ximgproc::RICInterpolator::setSuperpixelRuler", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_RICInterpolator_setSuperpixelRuler(cv::ximgproc::RICInterpolator* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setSuperpixelRuler();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSuperpixelRuler()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:190
	// ("cv::ximgproc::RICInterpolator::getSuperpixelRuler", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_RICInterpolator_getSuperpixelRuler_const(const cv::ximgproc::RICInterpolator* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSuperpixelRuler();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSuperpixelMode(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:197
	// ("cv::ximgproc::RICInterpolator::setSuperpixelMode", vec![(pred!(mut, ["mode"], ["int"]), _)]),
	void cv_ximgproc_RICInterpolator_setSuperpixelMode_int(cv::ximgproc::RICInterpolator* instance, int mode, ResultVoid* ocvrs_return) {
		try {
			instance->setSuperpixelMode(mode);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::RICInterpolator::setSuperpixelMode() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:197
	// ("cv::ximgproc::RICInterpolator::setSuperpixelMode", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_RICInterpolator_setSuperpixelMode(cv::ximgproc::RICInterpolator* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setSuperpixelMode();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSuperpixelMode()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:201
	// ("cv::ximgproc::RICInterpolator::getSuperpixelMode", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_RICInterpolator_getSuperpixelMode_const(const cv::ximgproc::RICInterpolator* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSuperpixelMode();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setAlpha(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:204
	// ("cv::ximgproc::RICInterpolator::setAlpha", vec![(pred!(mut, ["alpha"], ["float"]), _)]),
	void cv_ximgproc_RICInterpolator_setAlpha_float(cv::ximgproc::RICInterpolator* instance, float alpha, ResultVoid* ocvrs_return) {
		try {
			instance->setAlpha(alpha);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::RICInterpolator::setAlpha() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:204
	// ("cv::ximgproc::RICInterpolator::setAlpha", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_RICInterpolator_setAlpha(cv::ximgproc::RICInterpolator* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setAlpha();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAlpha()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:208
	// ("cv::ximgproc::RICInterpolator::getAlpha", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_RICInterpolator_getAlpha_const(const cv::ximgproc::RICInterpolator* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getAlpha();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setModelIter(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:211
	// ("cv::ximgproc::RICInterpolator::setModelIter", vec![(pred!(mut, ["modelIter"], ["int"]), _)]),
	void cv_ximgproc_RICInterpolator_setModelIter_int(cv::ximgproc::RICInterpolator* instance, int modelIter, ResultVoid* ocvrs_return) {
		try {
			instance->setModelIter(modelIter);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::RICInterpolator::setModelIter() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:211
	// ("cv::ximgproc::RICInterpolator::setModelIter", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_RICInterpolator_setModelIter(cv::ximgproc::RICInterpolator* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setModelIter();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getModelIter()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:215
	// ("cv::ximgproc::RICInterpolator::getModelIter", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_RICInterpolator_getModelIter_const(const cv::ximgproc::RICInterpolator* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getModelIter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRefineModels(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:218
	// ("cv::ximgproc::RICInterpolator::setRefineModels", vec![(pred!(mut, ["refineModles"], ["bool"]), _)]),
	void cv_ximgproc_RICInterpolator_setRefineModels_bool(cv::ximgproc::RICInterpolator* instance, bool refineModles, ResultVoid* ocvrs_return) {
		try {
			instance->setRefineModels(refineModles);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::RICInterpolator::setRefineModels() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:218
	// ("cv::ximgproc::RICInterpolator::setRefineModels", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_RICInterpolator_setRefineModels(cv::ximgproc::RICInterpolator* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setRefineModels();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRefineModels()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:222
	// ("cv::ximgproc::RICInterpolator::getRefineModels", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_RICInterpolator_getRefineModels_const(const cv::ximgproc::RICInterpolator* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getRefineModels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxFlow(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:226
	// ("cv::ximgproc::RICInterpolator::setMaxFlow", vec![(pred!(mut, ["maxFlow"], ["float"]), _)]),
	void cv_ximgproc_RICInterpolator_setMaxFlow_float(cv::ximgproc::RICInterpolator* instance, float maxFlow, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxFlow(maxFlow);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::RICInterpolator::setMaxFlow() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:226
	// ("cv::ximgproc::RICInterpolator::setMaxFlow", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_RICInterpolator_setMaxFlow(cv::ximgproc::RICInterpolator* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxFlow();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxFlow()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:230
	// ("cv::ximgproc::RICInterpolator::getMaxFlow", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_RICInterpolator_getMaxFlow_const(const cv::ximgproc::RICInterpolator* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMaxFlow();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseVariationalRefinement(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:233
	// ("cv::ximgproc::RICInterpolator::setUseVariationalRefinement", vec![(pred!(mut, ["use_variational_refinement"], ["bool"]), _)]),
	void cv_ximgproc_RICInterpolator_setUseVariationalRefinement_bool(cv::ximgproc::RICInterpolator* instance, bool use_variational_refinement, ResultVoid* ocvrs_return) {
		try {
			instance->setUseVariationalRefinement(use_variational_refinement);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::RICInterpolator::setUseVariationalRefinement() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:233
	// ("cv::ximgproc::RICInterpolator::setUseVariationalRefinement", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_RICInterpolator_setUseVariationalRefinement(cv::ximgproc::RICInterpolator* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setUseVariationalRefinement();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUseVariationalRefinement()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:237
	// ("cv::ximgproc::RICInterpolator::getUseVariationalRefinement", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_RICInterpolator_getUseVariationalRefinement_const(const cv::ximgproc::RICInterpolator* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseVariationalRefinement();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseGlobalSmootherFilter(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:240
	// ("cv::ximgproc::RICInterpolator::setUseGlobalSmootherFilter", vec![(pred!(mut, ["use_FGS"], ["bool"]), _)]),
	void cv_ximgproc_RICInterpolator_setUseGlobalSmootherFilter_bool(cv::ximgproc::RICInterpolator* instance, bool use_FGS, ResultVoid* ocvrs_return) {
		try {
			instance->setUseGlobalSmootherFilter(use_FGS);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::RICInterpolator::setUseGlobalSmootherFilter() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:240
	// ("cv::ximgproc::RICInterpolator::setUseGlobalSmootherFilter", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_RICInterpolator_setUseGlobalSmootherFilter(cv::ximgproc::RICInterpolator* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setUseGlobalSmootherFilter();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUseGlobalSmootherFilter()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:244
	// ("cv::ximgproc::RICInterpolator::getUseGlobalSmootherFilter", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_RICInterpolator_getUseGlobalSmootherFilter_const(const cv::ximgproc::RICInterpolator* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseGlobalSmootherFilter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFGSLambda(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:247
	// ("cv::ximgproc::RICInterpolator::setFGSLambda", vec![(pred!(mut, ["lambda"], ["float"]), _)]),
	void cv_ximgproc_RICInterpolator_setFGSLambda_float(cv::ximgproc::RICInterpolator* instance, float lambda, ResultVoid* ocvrs_return) {
		try {
			instance->setFGSLambda(lambda);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::RICInterpolator::setFGSLambda() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:247
	// ("cv::ximgproc::RICInterpolator::setFGSLambda", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_RICInterpolator_setFGSLambda(cv::ximgproc::RICInterpolator* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setFGSLambda();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFGSLambda()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:251
	// ("cv::ximgproc::RICInterpolator::getFGSLambda", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_RICInterpolator_getFGSLambda_const(const cv::ximgproc::RICInterpolator* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getFGSLambda();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFGSSigma(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:254
	// ("cv::ximgproc::RICInterpolator::setFGSSigma", vec![(pred!(mut, ["sigma"], ["float"]), _)]),
	void cv_ximgproc_RICInterpolator_setFGSSigma_float(cv::ximgproc::RICInterpolator* instance, float sigma, ResultVoid* ocvrs_return) {
		try {
			instance->setFGSSigma(sigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::RICInterpolator::setFGSSigma() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:254
	// ("cv::ximgproc::RICInterpolator::setFGSSigma", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_RICInterpolator_setFGSSigma(cv::ximgproc::RICInterpolator* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setFGSSigma();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFGSSigma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:258
	// ("cv::ximgproc::RICInterpolator::getFGSSigma", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_RICInterpolator_getFGSSigma_const(const cv::ximgproc::RICInterpolator* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getFGSSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::RICInterpolator::to_Algorithm() generated
	// ("cv::ximgproc::RICInterpolator::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ximgproc_RICInterpolator_to_Algorithm(cv::ximgproc::RICInterpolator* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ximgproc::RICInterpolator::to_SparseMatchInterpolator() generated
	// ("cv::ximgproc::RICInterpolator::to_SparseMatchInterpolator", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::SparseMatchInterpolator* cv_ximgproc_RICInterpolator_to_SparseMatchInterpolator(cv::ximgproc::RICInterpolator* instance) {
			return dynamic_cast<cv::ximgproc::SparseMatchInterpolator*>(instance);
	}

	// cv::ximgproc::RICInterpolator::delete() generated
	// ("cv::ximgproc::RICInterpolator::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_RICInterpolator_delete(cv::ximgproc::RICInterpolator* instance) {
			delete instance;
	}

	// create(int, int, int, int, int, double, double, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/ridgefilter.hpp:42
	// ("cv::ximgproc::RidgeDetectionFilter::create", vec![(pred!(mut, ["ddepth", "dx", "dy", "ksize", "out_dtype", "scale", "delta", "borderType"], ["int", "int", "int", "int", "int", "double", "double", "int"]), _)]),
	void cv_ximgproc_RidgeDetectionFilter_create_int_int_int_int_int_double_double_int(int ddepth, int dx, int dy, int ksize, int out_dtype, double scale, double delta, int borderType, Result<cv::Ptr<cv::ximgproc::RidgeDetectionFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::RidgeDetectionFilter> ret = cv::ximgproc::RidgeDetectionFilter::create(ddepth, dx, dy, ksize, out_dtype, scale, delta, borderType);
			Ok(new cv::Ptr<cv::ximgproc::RidgeDetectionFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::RidgeDetectionFilter::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/ridgefilter.hpp:42
	// ("cv::ximgproc::RidgeDetectionFilter::create", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_RidgeDetectionFilter_create(Result<cv::Ptr<cv::ximgproc::RidgeDetectionFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::RidgeDetectionFilter> ret = cv::ximgproc::RidgeDetectionFilter::create();
			Ok(new cv::Ptr<cv::ximgproc::RidgeDetectionFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRidgeFilteredImage(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/ridgefilter.hpp:48
	// ("cv::ximgproc::RidgeDetectionFilter::getRidgeFilteredImage", vec![(pred!(mut, ["_img", "out"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_RidgeDetectionFilter_getRidgeFilteredImage_const__InputArrayR_const__OutputArrayR(cv::ximgproc::RidgeDetectionFilter* instance, const cv::_InputArray* _img, const cv::_OutputArray* out, ResultVoid* ocvrs_return) {
		try {
			instance->getRidgeFilteredImage(*_img, *out);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::RidgeDetectionFilter::to_Algorithm() generated
	// ("cv::ximgproc::RidgeDetectionFilter::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ximgproc_RidgeDetectionFilter_to_Algorithm(cv::ximgproc::RidgeDetectionFilter* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ximgproc::RidgeDetectionFilter::delete() generated
	// ("cv::ximgproc::RidgeDetectionFilter::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_RidgeDetectionFilter_delete(cv::ximgproc::RidgeDetectionFilter* instance) {
			delete instance;
	}

	// getNumberOfSuperpixels()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/scansegment.hpp:32
	// ("cv::ximgproc::ScanSegment::getNumberOfSuperpixels", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_ScanSegment_getNumberOfSuperpixels(cv::ximgproc::ScanSegment* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumberOfSuperpixels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// iterate(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/scansegment.hpp:43
	// ("cv::ximgproc::ScanSegment::iterate", vec![(pred!(mut, ["img"], ["const cv::_InputArray*"]), _)]),
	void cv_ximgproc_ScanSegment_iterate_const__InputArrayR(cv::ximgproc::ScanSegment* instance, const cv::_InputArray* img, ResultVoid* ocvrs_return) {
		try {
			instance->iterate(*img);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLabels(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/scansegment.hpp:52
	// ("cv::ximgproc::ScanSegment::getLabels", vec![(pred!(mut, ["labels_out"], ["const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_ScanSegment_getLabels_const__OutputArrayR(cv::ximgproc::ScanSegment* instance, const cv::_OutputArray* labels_out, ResultVoid* ocvrs_return) {
		try {
			instance->getLabels(*labels_out);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLabelContourMask(OutputArray, bool)(OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/scansegment.hpp:61
	// ("cv::ximgproc::ScanSegment::getLabelContourMask", vec![(pred!(mut, ["image", "thick_line"], ["const cv::_OutputArray*", "bool"]), _)]),
	void cv_ximgproc_ScanSegment_getLabelContourMask_const__OutputArrayR_bool(cv::ximgproc::ScanSegment* instance, const cv::_OutputArray* image, bool thick_line, ResultVoid* ocvrs_return) {
		try {
			instance->getLabelContourMask(*image, thick_line);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::ScanSegment::getLabelContourMask(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/scansegment.hpp:61
	// ("cv::ximgproc::ScanSegment::getLabelContourMask", vec![(pred!(mut, ["image"], ["const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_ScanSegment_getLabelContourMask_const__OutputArrayR(cv::ximgproc::ScanSegment* instance, const cv::_OutputArray* image, ResultVoid* ocvrs_return) {
		try {
			instance->getLabelContourMask(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::ScanSegment::to_Algorithm() generated
	// ("cv::ximgproc::ScanSegment::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ximgproc_ScanSegment_to_Algorithm(cv::ximgproc::ScanSegment* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ximgproc::ScanSegment::delete() generated
	// ("cv::ximgproc::ScanSegment::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_ScanSegment_delete(cv::ximgproc::ScanSegment* instance) {
			delete instance;
	}

	// interpolate(InputArray, InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/sparse_match_interpolator.hpp:69
	// ("cv::ximgproc::SparseMatchInterpolator::interpolate", vec![(pred!(mut, ["from_image", "from_points", "to_image", "to_points", "dense_flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_SparseMatchInterpolator_interpolate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::ximgproc::SparseMatchInterpolator* instance, const cv::_InputArray* from_image, const cv::_InputArray* from_points, const cv::_InputArray* to_image, const cv::_InputArray* to_points, const cv::_OutputArray* dense_flow, ResultVoid* ocvrs_return) {
		try {
			instance->interpolate(*from_image, *from_points, *to_image, *to_points, *dense_flow);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::SparseMatchInterpolator::to_EdgeAwareInterpolator() generated
	// ("cv::ximgproc::SparseMatchInterpolator::to_EdgeAwareInterpolator", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::EdgeAwareInterpolator* cv_ximgproc_SparseMatchInterpolator_to_EdgeAwareInterpolator(cv::ximgproc::SparseMatchInterpolator* instance) {
			return dynamic_cast<cv::ximgproc::EdgeAwareInterpolator*>(instance);
	}

	// cv::ximgproc::SparseMatchInterpolator::to_RICInterpolator() generated
	// ("cv::ximgproc::SparseMatchInterpolator::to_RICInterpolator", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::RICInterpolator* cv_ximgproc_SparseMatchInterpolator_to_RICInterpolator(cv::ximgproc::SparseMatchInterpolator* instance) {
			return dynamic_cast<cv::ximgproc::RICInterpolator*>(instance);
	}

	// cv::ximgproc::SparseMatchInterpolator::to_Algorithm() generated
	// ("cv::ximgproc::SparseMatchInterpolator::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ximgproc_SparseMatchInterpolator_to_Algorithm(cv::ximgproc::SparseMatchInterpolator* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ximgproc::SparseMatchInterpolator::delete() generated
	// ("cv::ximgproc::SparseMatchInterpolator::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_SparseMatchInterpolator_delete(cv::ximgproc::SparseMatchInterpolator* instance) {
			delete instance;
	}

	// detectEdges(cv::InputArray, cv::OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/structured_edge_detection.hpp:109
	// ("cv::ximgproc::StructuredEdgeDetection::detectEdges", vec![(pred!(const, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_StructuredEdgeDetection_detectEdges_const_const__InputArrayR_const__OutputArrayR(const cv::ximgproc::StructuredEdgeDetection* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->detectEdges(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeOrientation(cv::InputArray, cv::OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/structured_edge_detection.hpp:116
	// ("cv::ximgproc::StructuredEdgeDetection::computeOrientation", vec![(pred!(const, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_StructuredEdgeDetection_computeOrientation_const_const__InputArrayR_const__OutputArrayR(const cv::ximgproc::StructuredEdgeDetection* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->computeOrientation(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// edgesNms(cv::InputArray, cv::InputArray, cv::OutputArray, int, int, float, bool)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/structured_edge_detection.hpp:129
	// ("cv::ximgproc::StructuredEdgeDetection::edgesNms", vec![(pred!(const, ["edge_image", "orientation_image", "dst", "r", "s", "m", "isParallel"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "float", "bool"]), _)]),
	void cv_ximgproc_StructuredEdgeDetection_edgesNms_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_float_bool(const cv::ximgproc::StructuredEdgeDetection* instance, const cv::_InputArray* edge_image, const cv::_InputArray* orientation_image, const cv::_OutputArray* dst, int r, int s, float m, bool isParallel, ResultVoid* ocvrs_return) {
		try {
			instance->edgesNms(*edge_image, *orientation_image, *dst, r, s, m, isParallel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::StructuredEdgeDetection::edgesNms(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/structured_edge_detection.hpp:129
	// ("cv::ximgproc::StructuredEdgeDetection::edgesNms", vec![(pred!(const, ["edge_image", "orientation_image", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_StructuredEdgeDetection_edgesNms_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::ximgproc::StructuredEdgeDetection* instance, const cv::_InputArray* edge_image, const cv::_InputArray* orientation_image, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->edgesNms(*edge_image, *orientation_image, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::StructuredEdgeDetection::to_Algorithm() generated
	// ("cv::ximgproc::StructuredEdgeDetection::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ximgproc_StructuredEdgeDetection_to_Algorithm(cv::ximgproc::StructuredEdgeDetection* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ximgproc::StructuredEdgeDetection::delete() generated
	// ("cv::ximgproc::StructuredEdgeDetection::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_StructuredEdgeDetection_delete(cv::ximgproc::StructuredEdgeDetection* instance) {
			delete instance;
	}

	// getNumberOfSuperpixels()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/lsc.hpp:78
	// ("cv::ximgproc::SuperpixelLSC::getNumberOfSuperpixels", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_SuperpixelLSC_getNumberOfSuperpixels_const(const cv::ximgproc::SuperpixelLSC* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumberOfSuperpixels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// iterate(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/lsc.hpp:94
	// ("cv::ximgproc::SuperpixelLSC::iterate", vec![(pred!(mut, ["num_iterations"], ["int"]), _)]),
	void cv_ximgproc_SuperpixelLSC_iterate_int(cv::ximgproc::SuperpixelLSC* instance, int num_iterations, ResultVoid* ocvrs_return) {
		try {
			instance->iterate(num_iterations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::SuperpixelLSC::iterate() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/lsc.hpp:94
	// ("cv::ximgproc::SuperpixelLSC::iterate", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_SuperpixelLSC_iterate(cv::ximgproc::SuperpixelLSC* instance, ResultVoid* ocvrs_return) {
		try {
			instance->iterate();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLabels(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/lsc.hpp:106
	// ("cv::ximgproc::SuperpixelLSC::getLabels", vec![(pred!(const, ["labels_out"], ["const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_SuperpixelLSC_getLabels_const_const__OutputArrayR(const cv::ximgproc::SuperpixelLSC* instance, const cv::_OutputArray* labels_out, ResultVoid* ocvrs_return) {
		try {
			instance->getLabels(*labels_out);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLabelContourMask(OutputArray, bool)(OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/lsc.hpp:118
	// ("cv::ximgproc::SuperpixelLSC::getLabelContourMask", vec![(pred!(const, ["image", "thick_line"], ["const cv::_OutputArray*", "bool"]), _)]),
	void cv_ximgproc_SuperpixelLSC_getLabelContourMask_const_const__OutputArrayR_bool(const cv::ximgproc::SuperpixelLSC* instance, const cv::_OutputArray* image, bool thick_line, ResultVoid* ocvrs_return) {
		try {
			instance->getLabelContourMask(*image, thick_line);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::SuperpixelLSC::getLabelContourMask(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/lsc.hpp:118
	// ("cv::ximgproc::SuperpixelLSC::getLabelContourMask", vec![(pred!(const, ["image"], ["const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_SuperpixelLSC_getLabelContourMask_const_const__OutputArrayR(const cv::ximgproc::SuperpixelLSC* instance, const cv::_OutputArray* image, ResultVoid* ocvrs_return) {
		try {
			instance->getLabelContourMask(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// enforceLabelConnectivity(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/lsc.hpp:129
	// ("cv::ximgproc::SuperpixelLSC::enforceLabelConnectivity", vec![(pred!(mut, ["min_element_size"], ["int"]), _)]),
	void cv_ximgproc_SuperpixelLSC_enforceLabelConnectivity_int(cv::ximgproc::SuperpixelLSC* instance, int min_element_size, ResultVoid* ocvrs_return) {
		try {
			instance->enforceLabelConnectivity(min_element_size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::SuperpixelLSC::enforceLabelConnectivity() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/lsc.hpp:129
	// ("cv::ximgproc::SuperpixelLSC::enforceLabelConnectivity", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_SuperpixelLSC_enforceLabelConnectivity(cv::ximgproc::SuperpixelLSC* instance, ResultVoid* ocvrs_return) {
		try {
			instance->enforceLabelConnectivity();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::SuperpixelLSC::to_Algorithm() generated
	// ("cv::ximgproc::SuperpixelLSC::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ximgproc_SuperpixelLSC_to_Algorithm(cv::ximgproc::SuperpixelLSC* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ximgproc::SuperpixelLSC::delete() generated
	// ("cv::ximgproc::SuperpixelLSC::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_SuperpixelLSC_delete(cv::ximgproc::SuperpixelLSC* instance) {
			delete instance;
	}

	// getNumberOfSuperpixels()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/seeds.hpp:75
	// ("cv::ximgproc::SuperpixelSEEDS::getNumberOfSuperpixels", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_SuperpixelSEEDS_getNumberOfSuperpixels(cv::ximgproc::SuperpixelSEEDS* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumberOfSuperpixels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// iterate(InputArray, int)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/seeds.hpp:99
	// ("cv::ximgproc::SuperpixelSEEDS::iterate", vec![(pred!(mut, ["img", "num_iterations"], ["const cv::_InputArray*", "int"]), _)]),
	void cv_ximgproc_SuperpixelSEEDS_iterate_const__InputArrayR_int(cv::ximgproc::SuperpixelSEEDS* instance, const cv::_InputArray* img, int num_iterations, ResultVoid* ocvrs_return) {
		try {
			instance->iterate(*img, num_iterations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::SuperpixelSEEDS::iterate(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/seeds.hpp:99
	// ("cv::ximgproc::SuperpixelSEEDS::iterate", vec![(pred!(mut, ["img"], ["const cv::_InputArray*"]), _)]),
	void cv_ximgproc_SuperpixelSEEDS_iterate_const__InputArrayR(cv::ximgproc::SuperpixelSEEDS* instance, const cv::_InputArray* img, ResultVoid* ocvrs_return) {
		try {
			instance->iterate(*img);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLabels(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/seeds.hpp:111
	// ("cv::ximgproc::SuperpixelSEEDS::getLabels", vec![(pred!(mut, ["labels_out"], ["const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_SuperpixelSEEDS_getLabels_const__OutputArrayR(cv::ximgproc::SuperpixelSEEDS* instance, const cv::_OutputArray* labels_out, ResultVoid* ocvrs_return) {
		try {
			instance->getLabels(*labels_out);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLabelContourMask(OutputArray, bool)(OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/seeds.hpp:139
	// ("cv::ximgproc::SuperpixelSEEDS::getLabelContourMask", vec![(pred!(mut, ["image", "thick_line"], ["const cv::_OutputArray*", "bool"]), _)]),
	void cv_ximgproc_SuperpixelSEEDS_getLabelContourMask_const__OutputArrayR_bool(cv::ximgproc::SuperpixelSEEDS* instance, const cv::_OutputArray* image, bool thick_line, ResultVoid* ocvrs_return) {
		try {
			instance->getLabelContourMask(*image, thick_line);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::SuperpixelSEEDS::getLabelContourMask(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/seeds.hpp:139
	// ("cv::ximgproc::SuperpixelSEEDS::getLabelContourMask", vec![(pred!(mut, ["image"], ["const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_SuperpixelSEEDS_getLabelContourMask_const__OutputArrayR(cv::ximgproc::SuperpixelSEEDS* instance, const cv::_OutputArray* image, ResultVoid* ocvrs_return) {
		try {
			instance->getLabelContourMask(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::SuperpixelSEEDS::to_Algorithm() generated
	// ("cv::ximgproc::SuperpixelSEEDS::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ximgproc_SuperpixelSEEDS_to_Algorithm(cv::ximgproc::SuperpixelSEEDS* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ximgproc::SuperpixelSEEDS::delete() generated
	// ("cv::ximgproc::SuperpixelSEEDS::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_SuperpixelSEEDS_delete(cv::ximgproc::SuperpixelSEEDS* instance) {
			delete instance;
	}

	// getNumberOfSuperpixels()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/slic.hpp:85
	// ("cv::ximgproc::SuperpixelSLIC::getNumberOfSuperpixels", vec![(pred!(const, [], []), _)]),
	void cv_ximgproc_SuperpixelSLIC_getNumberOfSuperpixels_const(const cv::ximgproc::SuperpixelSLIC* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumberOfSuperpixels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// iterate(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/slic.hpp:101
	// ("cv::ximgproc::SuperpixelSLIC::iterate", vec![(pred!(mut, ["num_iterations"], ["int"]), _)]),
	void cv_ximgproc_SuperpixelSLIC_iterate_int(cv::ximgproc::SuperpixelSLIC* instance, int num_iterations, ResultVoid* ocvrs_return) {
		try {
			instance->iterate(num_iterations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::SuperpixelSLIC::iterate() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/slic.hpp:101
	// ("cv::ximgproc::SuperpixelSLIC::iterate", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_SuperpixelSLIC_iterate(cv::ximgproc::SuperpixelSLIC* instance, ResultVoid* ocvrs_return) {
		try {
			instance->iterate();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLabels(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/slic.hpp:113
	// ("cv::ximgproc::SuperpixelSLIC::getLabels", vec![(pred!(const, ["labels_out"], ["const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_SuperpixelSLIC_getLabels_const_const__OutputArrayR(const cv::ximgproc::SuperpixelSLIC* instance, const cv::_OutputArray* labels_out, ResultVoid* ocvrs_return) {
		try {
			instance->getLabels(*labels_out);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLabelContourMask(OutputArray, bool)(OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/slic.hpp:125
	// ("cv::ximgproc::SuperpixelSLIC::getLabelContourMask", vec![(pred!(const, ["image", "thick_line"], ["const cv::_OutputArray*", "bool"]), _)]),
	void cv_ximgproc_SuperpixelSLIC_getLabelContourMask_const_const__OutputArrayR_bool(const cv::ximgproc::SuperpixelSLIC* instance, const cv::_OutputArray* image, bool thick_line, ResultVoid* ocvrs_return) {
		try {
			instance->getLabelContourMask(*image, thick_line);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::SuperpixelSLIC::getLabelContourMask(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/slic.hpp:125
	// ("cv::ximgproc::SuperpixelSLIC::getLabelContourMask", vec![(pred!(const, ["image"], ["const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_SuperpixelSLIC_getLabelContourMask_const_const__OutputArrayR(const cv::ximgproc::SuperpixelSLIC* instance, const cv::_OutputArray* image, ResultVoid* ocvrs_return) {
		try {
			instance->getLabelContourMask(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// enforceLabelConnectivity(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/slic.hpp:136
	// ("cv::ximgproc::SuperpixelSLIC::enforceLabelConnectivity", vec![(pred!(mut, ["min_element_size"], ["int"]), _)]),
	void cv_ximgproc_SuperpixelSLIC_enforceLabelConnectivity_int(cv::ximgproc::SuperpixelSLIC* instance, int min_element_size, ResultVoid* ocvrs_return) {
		try {
			instance->enforceLabelConnectivity(min_element_size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::SuperpixelSLIC::enforceLabelConnectivity() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/slic.hpp:136
	// ("cv::ximgproc::SuperpixelSLIC::enforceLabelConnectivity", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_SuperpixelSLIC_enforceLabelConnectivity(cv::ximgproc::SuperpixelSLIC* instance, ResultVoid* ocvrs_return) {
		try {
			instance->enforceLabelConnectivity();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::SuperpixelSLIC::to_Algorithm() generated
	// ("cv::ximgproc::SuperpixelSLIC::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ximgproc_SuperpixelSLIC_to_Algorithm(cv::ximgproc::SuperpixelSLIC* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ximgproc::SuperpixelSLIC::delete() generated
	// ("cv::ximgproc::SuperpixelSLIC::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_SuperpixelSLIC_delete(cv::ximgproc::SuperpixelSLIC* instance) {
			delete instance;
	}

	// processImage(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:52
	// ("cv::ximgproc::segmentation::GraphSegmentation::processImage", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ximgproc_segmentation_GraphSegmentation_processImage_const__InputArrayR_const__OutputArrayR(cv::ximgproc::segmentation::GraphSegmentation* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->processImage(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSigma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:54
	// ("cv::ximgproc::segmentation::GraphSegmentation::setSigma", vec![(pred!(mut, ["sigma"], ["double"]), _)]),
	void cv_ximgproc_segmentation_GraphSegmentation_setSigma_double(cv::ximgproc::segmentation::GraphSegmentation* instance, double sigma, ResultVoid* ocvrs_return) {
		try {
			instance->setSigma(sigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSigma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:55
	// ("cv::ximgproc::segmentation::GraphSegmentation::getSigma", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_segmentation_GraphSegmentation_getSigma(cv::ximgproc::segmentation::GraphSegmentation* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setK(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:57
	// ("cv::ximgproc::segmentation::GraphSegmentation::setK", vec![(pred!(mut, ["k"], ["float"]), _)]),
	void cv_ximgproc_segmentation_GraphSegmentation_setK_float(cv::ximgproc::segmentation::GraphSegmentation* instance, float k, ResultVoid* ocvrs_return) {
		try {
			instance->setK(k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getK()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:58
	// ("cv::ximgproc::segmentation::GraphSegmentation::getK", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_segmentation_GraphSegmentation_getK(cv::ximgproc::segmentation::GraphSegmentation* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getK();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:60
	// ("cv::ximgproc::segmentation::GraphSegmentation::setMinSize", vec![(pred!(mut, ["min_size"], ["int"]), _)]),
	void cv_ximgproc_segmentation_GraphSegmentation_setMinSize_int(cv::ximgproc::segmentation::GraphSegmentation* instance, int min_size, ResultVoid* ocvrs_return) {
		try {
			instance->setMinSize(min_size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:61
	// ("cv::ximgproc::segmentation::GraphSegmentation::getMinSize", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_segmentation_GraphSegmentation_getMinSize(cv::ximgproc::segmentation::GraphSegmentation* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMinSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::segmentation::GraphSegmentation::to_Algorithm() generated
	// ("cv::ximgproc::segmentation::GraphSegmentation::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ximgproc_segmentation_GraphSegmentation_to_Algorithm(cv::ximgproc::segmentation::GraphSegmentation* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ximgproc::segmentation::GraphSegmentation::delete() generated
	// ("cv::ximgproc::segmentation::GraphSegmentation::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_segmentation_GraphSegmentation_delete(cv::ximgproc::segmentation::GraphSegmentation* instance) {
			delete instance;
	}

	// setBaseImage(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:187
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::setBaseImage", vec![(pred!(mut, ["img"], ["const cv::_InputArray*"]), _)]),
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_setBaseImage_const__InputArrayR(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, const cv::_InputArray* img, ResultVoid* ocvrs_return) {
		try {
			instance->setBaseImage(*img);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// switchToSingleStrategy(int, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:193
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::switchToSingleStrategy", vec![(pred!(mut, ["k", "sigma"], ["int", "float"]), _)]),
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_switchToSingleStrategy_int_float(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, int k, float sigma, ResultVoid* ocvrs_return) {
		try {
			instance->switchToSingleStrategy(k, sigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::segmentation::SelectiveSearchSegmentation::switchToSingleStrategy() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:193
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::switchToSingleStrategy", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_switchToSingleStrategy(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, ResultVoid* ocvrs_return) {
		try {
			instance->switchToSingleStrategy();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// switchToSelectiveSearchFast(int, int, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:200
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::switchToSelectiveSearchFast", vec![(pred!(mut, ["base_k", "inc_k", "sigma"], ["int", "int", "float"]), _)]),
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_switchToSelectiveSearchFast_int_int_float(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, int base_k, int inc_k, float sigma, ResultVoid* ocvrs_return) {
		try {
			instance->switchToSelectiveSearchFast(base_k, inc_k, sigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::segmentation::SelectiveSearchSegmentation::switchToSelectiveSearchFast() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:200
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::switchToSelectiveSearchFast", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_switchToSelectiveSearchFast(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, ResultVoid* ocvrs_return) {
		try {
			instance->switchToSelectiveSearchFast();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// switchToSelectiveSearchQuality(int, int, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:207
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::switchToSelectiveSearchQuality", vec![(pred!(mut, ["base_k", "inc_k", "sigma"], ["int", "int", "float"]), _)]),
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_switchToSelectiveSearchQuality_int_int_float(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, int base_k, int inc_k, float sigma, ResultVoid* ocvrs_return) {
		try {
			instance->switchToSelectiveSearchQuality(base_k, inc_k, sigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::segmentation::SelectiveSearchSegmentation::switchToSelectiveSearchQuality() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:207
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::switchToSelectiveSearchQuality", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_switchToSelectiveSearchQuality(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, ResultVoid* ocvrs_return) {
		try {
			instance->switchToSelectiveSearchQuality();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addImage(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:212
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::addImage", vec![(pred!(mut, ["img"], ["const cv::_InputArray*"]), _)]),
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_addImage_const__InputArrayR(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, const cv::_InputArray* img, ResultVoid* ocvrs_return) {
		try {
			instance->addImage(*img);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clearImages()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:216
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::clearImages", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_clearImages(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, ResultVoid* ocvrs_return) {
		try {
			instance->clearImages();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addGraphSegmentation(Ptr<GraphSegmentation>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:221
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::addGraphSegmentation", vec![(pred!(mut, ["g"], ["cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>"]), _)]),
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_addGraphSegmentation_PtrLGraphSegmentationG(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>* g, ResultVoid* ocvrs_return) {
		try {
			instance->addGraphSegmentation(*g);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clearGraphSegmentations()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:225
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::clearGraphSegmentations", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_clearGraphSegmentations(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, ResultVoid* ocvrs_return) {
		try {
			instance->clearGraphSegmentations();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addStrategy(Ptr<SelectiveSearchSegmentationStrategy>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:230
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::addStrategy", vec![(pred!(mut, ["s"], ["cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>"]), _)]),
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_addStrategy_PtrLSelectiveSearchSegmentationStrategyG(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* s, ResultVoid* ocvrs_return) {
		try {
			instance->addStrategy(*s);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clearStrategies()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:234
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::clearStrategies", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_clearStrategies(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, ResultVoid* ocvrs_return) {
		try {
			instance->clearStrategies();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// process(std::vector<Rect> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:239
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::process", vec![(pred!(mut, ["rects"], ["std::vector<cv::Rect>*"]), _)]),
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_process_vectorLRectGR(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, std::vector<cv::Rect>* rects, ResultVoid* ocvrs_return) {
		try {
			instance->process(*rects);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::segmentation::SelectiveSearchSegmentation::to_Algorithm() generated
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ximgproc_segmentation_SelectiveSearchSegmentation_to_Algorithm(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ximgproc::segmentation::SelectiveSearchSegmentation::delete() generated
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentation::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_delete(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance) {
			delete instance;
	}

	// setImage(InputArray, InputArray, InputArray, int)(InputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:82
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::setImage", vec![(pred!(mut, ["img", "regions", "sizes", "image_id"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
	void cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_setImage_const__InputArrayR_const__InputArrayR_const__InputArrayR_int(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* instance, const cv::_InputArray* img, const cv::_InputArray* regions, const cv::_InputArray* sizes, int image_id, ResultVoid* ocvrs_return) {
		try {
			instance->setImage(*img, *regions, *sizes, image_id);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::setImage(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:82
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::setImage", vec![(pred!(mut, ["img", "regions", "sizes"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_setImage_const__InputArrayR_const__InputArrayR_const__InputArrayR(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* instance, const cv::_InputArray* img, const cv::_InputArray* regions, const cv::_InputArray* sizes, ResultVoid* ocvrs_return) {
		try {
			instance->setImage(*img, *regions, *sizes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// get(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:88
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::get", vec![(pred!(mut, ["r1", "r2"], ["int", "int"]), _)]),
	void cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_get_int_int(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* instance, int r1, int r2, Result<float>* ocvrs_return) {
		try {
			float ret = instance->get(r1, r2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// merge(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:94
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::merge", vec![(pred!(mut, ["r1", "r2"], ["int", "int"]), _)]),
	void cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_merge_int_int(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* instance, int r1, int r2, ResultVoid* ocvrs_return) {
		try {
			instance->merge(r1, r2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::to_SelectiveSearchSegmentationStrategyColor() generated
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::to_SelectiveSearchSegmentationStrategyColor", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_to_SelectiveSearchSegmentationStrategyColor(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* instance) {
			return dynamic_cast<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor*>(instance);
	}

	// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::to_SelectiveSearchSegmentationStrategyFill() generated
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::to_SelectiveSearchSegmentationStrategyFill", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_to_SelectiveSearchSegmentationStrategyFill(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* instance) {
			return dynamic_cast<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill*>(instance);
	}

	// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::to_SelectiveSearchSegmentationStrategyMultiple() generated
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::to_SelectiveSearchSegmentationStrategyMultiple", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_to_SelectiveSearchSegmentationStrategyMultiple(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* instance) {
			return dynamic_cast<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple*>(instance);
	}

	// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::to_SelectiveSearchSegmentationStrategySize() generated
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::to_SelectiveSearchSegmentationStrategySize", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_to_SelectiveSearchSegmentationStrategySize(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* instance) {
			return dynamic_cast<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize*>(instance);
	}

	// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::to_SelectiveSearchSegmentationStrategyTexture() generated
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::to_SelectiveSearchSegmentationStrategyTexture", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_to_SelectiveSearchSegmentationStrategyTexture(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* instance) {
			return dynamic_cast<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture*>(instance);
	}

	// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::to_Algorithm() generated
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_to_Algorithm(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::delete() generated
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_delete(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* instance) {
			delete instance;
	}

	// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor::to_Algorithm() generated
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyColor_to_Algorithm(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor::to_SelectiveSearchSegmentationStrategy() generated
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor::to_SelectiveSearchSegmentationStrategy", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyColor_to_SelectiveSearchSegmentationStrategy(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor* instance) {
			return dynamic_cast<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy*>(instance);
	}

	// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor::delete() generated
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyColor_delete(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor* instance) {
			delete instance;
	}

	// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill::to_Algorithm() generated
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyFill_to_Algorithm(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill::to_SelectiveSearchSegmentationStrategy() generated
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill::to_SelectiveSearchSegmentationStrategy", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyFill_to_SelectiveSearchSegmentationStrategy(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill* instance) {
			return dynamic_cast<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy*>(instance);
	}

	// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill::delete() generated
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyFill_delete(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill* instance) {
			delete instance;
	}

	// addStrategy(Ptr<SelectiveSearchSegmentationStrategy>, float)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:142
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple::addStrategy", vec![(pred!(mut, ["g", "weight"], ["cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>", "float"]), _)]),
	void cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultiple_addStrategy_PtrLSelectiveSearchSegmentationStrategyG_float(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple* instance, cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* g, float weight, ResultVoid* ocvrs_return) {
		try {
			instance->addStrategy(*g, weight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clearStrategies()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ximgproc/segmentation.hpp:145
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple::clearStrategies", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultiple_clearStrategies(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple* instance, ResultVoid* ocvrs_return) {
		try {
			instance->clearStrategies();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple::to_Algorithm() generated
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultiple_to_Algorithm(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple::to_SelectiveSearchSegmentationStrategy() generated
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple::to_SelectiveSearchSegmentationStrategy", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultiple_to_SelectiveSearchSegmentationStrategy(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple* instance) {
			return dynamic_cast<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy*>(instance);
	}

	// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple::delete() generated
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultiple_delete(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple* instance) {
			delete instance;
	}

	// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize::to_Algorithm() generated
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategySize_to_Algorithm(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize::to_SelectiveSearchSegmentationStrategy() generated
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize::to_SelectiveSearchSegmentationStrategy", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategySize_to_SelectiveSearchSegmentationStrategy(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize* instance) {
			return dynamic_cast<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy*>(instance);
	}

	// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize::delete() generated
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategySize_delete(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize* instance) {
			delete instance;
	}

	// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture::to_Algorithm() generated
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyTexture_to_Algorithm(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture::to_SelectiveSearchSegmentationStrategy() generated
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture::to_SelectiveSearchSegmentationStrategy", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyTexture_to_SelectiveSearchSegmentationStrategy(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture* instance) {
			return dynamic_cast<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy*>(instance);
	}

	// cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture::delete() generated
	// ("cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyTexture_delete(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture* instance) {
			delete instance;
	}

}
