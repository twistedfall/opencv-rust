#include "ocvrs_common.hpp"
#include <opencv2/imgproc.hpp>
#include "imgproc_types.hpp"

extern "C" {
	// cv::Canny(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1906
	// ("cv::Canny", vec![(pred!(mut, ["dx", "dy", "edges", "threshold1", "threshold2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
	void cv_Canny_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double(const cv::_InputArray* dx, const cv::_InputArray* dy, const cv::_OutputArray* edges, double threshold1, double threshold2, ResultVoid* ocvrs_return) {
		try {
			cv::Canny(*dx, *dy, *edges, threshold1, threshold2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Canny(InputArray, InputArray, OutputArray, double, double, bool)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1906
	// ("cv::Canny", vec![(pred!(mut, ["dx", "dy", "edges", "threshold1", "threshold2", "L2gradient"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "bool"]), _)]),
	void cv_Canny_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double_bool(const cv::_InputArray* dx, const cv::_InputArray* dy, const cv::_OutputArray* edges, double threshold1, double threshold2, bool L2gradient, ResultVoid* ocvrs_return) {
		try {
			cv::Canny(*dx, *dy, *edges, threshold1, threshold2, L2gradient);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Canny(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1888
	// ("cv::Canny", vec![(pred!(mut, ["image", "edges", "threshold1", "threshold2"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
	void cv_Canny_const__InputArrayR_const__OutputArrayR_double_double(const cv::_InputArray* image, const cv::_OutputArray* edges, double threshold1, double threshold2, ResultVoid* ocvrs_return) {
		try {
			cv::Canny(*image, *edges, threshold1, threshold2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Canny(InputArray, OutputArray, double, double, int, bool)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1888
	// ("cv::Canny", vec![(pred!(mut, ["image", "edges", "threshold1", "threshold2", "apertureSize", "L2gradient"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "int", "bool"]), _)]),
	void cv_Canny_const__InputArrayR_const__OutputArrayR_double_double_int_bool(const cv::_InputArray* image, const cv::_OutputArray* edges, double threshold1, double threshold2, int apertureSize, bool L2gradient, ResultVoid* ocvrs_return) {
		try {
			cv::Canny(*image, *edges, threshold1, threshold2, apertureSize, L2gradient);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::EMD(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3389
	// ("cv::EMD", vec![(pred!(mut, ["signature1", "signature2", "distType"], ["const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
	void cv_EMD_const__InputArrayR_const__InputArrayR_int(const cv::_InputArray* signature1, const cv::_InputArray* signature2, int distType, Result<float>* ocvrs_return) {
		try {
			float ret = cv::EMD(*signature1, *signature2, distType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// EMD(InputArray, InputArray, int, InputArray, float *, OutputArray)(InputArray, InputArray, Primitive, InputArray, Indirect, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3389
	// ("cv::EMD", vec![(pred!(mut, ["signature1", "signature2", "distType", "cost", "lowerBound", "flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "const cv::_InputArray*", "float*", "const cv::_OutputArray*"]), _)]),
	void cv_EMD_const__InputArrayR_const__InputArrayR_int_const__InputArrayR_floatX_const__OutputArrayR(const cv::_InputArray* signature1, const cv::_InputArray* signature2, int distType, const cv::_InputArray* cost, float* lowerBound, const cv::_OutputArray* flow, Result<float>* ocvrs_return) {
		try {
			float ret = cv::EMD(*signature1, *signature2, distType, *cost, lowerBound, *flow);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GaussianBlur(InputArray, OutputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1543
	// ("cv::GaussianBlur", vec![(pred!(mut, ["src", "dst", "ksize", "sigmaX"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size", "double"]), _)]),
	void cv_GaussianBlur_const__InputArrayR_const__OutputArrayR_Size_double(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Size* ksize, double sigmaX, ResultVoid* ocvrs_return) {
		try {
			cv::GaussianBlur(*src, *dst, *ksize, sigmaX);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GaussianBlur(InputArray, OutputArray, Size, double, double, int, AlgorithmHint)(InputArray, OutputArray, SimpleClass, Primitive, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1543
	// ("cv::GaussianBlur", vec![(pred!(mut, ["src", "dst", "ksize", "sigmaX", "sigmaY", "borderType", "hint"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size", "double", "double", "int", "cv::AlgorithmHint"]), _)]),
	void cv_GaussianBlur_const__InputArrayR_const__OutputArrayR_Size_double_double_int_AlgorithmHint(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Size* ksize, double sigmaX, double sigmaY, int borderType, cv::AlgorithmHint hint, ResultVoid* ocvrs_return) {
		try {
			cv::GaussianBlur(*src, *dst, *ksize, sigmaX, sigmaY, borderType, hint);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HoughCircles(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2277
	// ("cv::HoughCircles", vec![(pred!(mut, ["image", "circles", "method", "dp", "minDist"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double"]), _)]),
	void cv_HoughCircles_const__InputArrayR_const__OutputArrayR_int_double_double(const cv::_InputArray* image, const cv::_OutputArray* circles, int method, double dp, double minDist, ResultVoid* ocvrs_return) {
		try {
			cv::HoughCircles(*image, *circles, method, dp, minDist);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// HoughCircles(InputArray, OutputArray, int, double, double, double, double, int, int)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2277
	// ("cv::HoughCircles", vec![(pred!(mut, ["image", "circles", "method", "dp", "minDist", "param1", "param2", "minRadius", "maxRadius"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double", "double", "double", "int", "int"]), _)]),
	void cv_HoughCircles_const__InputArrayR_const__OutputArrayR_int_double_double_double_double_int_int(const cv::_InputArray* image, const cv::_OutputArray* circles, int method, double dp, double minDist, double param1, double param2, int minRadius, int maxRadius, ResultVoid* ocvrs_return) {
		try {
			cv::HoughCircles(*image, *circles, method, dp, minDist, param1, param2, minRadius, maxRadius);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HoughLinesP(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2204
	// ("cv::HoughLinesP", vec![(pred!(mut, ["image", "lines", "rho", "theta", "threshold"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "int"]), _)]),
	void cv_HoughLinesP_const__InputArrayR_const__OutputArrayR_double_double_int(const cv::_InputArray* image, const cv::_OutputArray* lines, double rho, double theta, int threshold, ResultVoid* ocvrs_return) {
		try {
			cv::HoughLinesP(*image, *lines, rho, theta, threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// HoughLinesP(InputArray, OutputArray, double, double, int, double, double)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2204
	// ("cv::HoughLinesP", vec![(pred!(mut, ["image", "lines", "rho", "theta", "threshold", "minLineLength", "maxLineGap"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "int", "double", "double"]), _)]),
	void cv_HoughLinesP_const__InputArrayR_const__OutputArrayR_double_double_int_double_double(const cv::_InputArray* image, const cv::_OutputArray* lines, double rho, double theta, int threshold, double minLineLength, double maxLineGap, ResultVoid* ocvrs_return) {
		try {
			cv::HoughLinesP(*image, *lines, rho, theta, threshold, minLineLength, maxLineGap);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// HoughLinesPointSet(InputArray, OutputArray, int, int, double, double, double, double, double, double)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2226
	// ("cv::HoughLinesPointSet", vec![(pred!(mut, ["point", "lines", "lines_max", "threshold", "min_rho", "max_rho", "rho_step", "min_theta", "max_theta", "theta_step"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "double", "double", "double", "double", "double", "double"]), _)]),
	void cv_HoughLinesPointSet_const__InputArrayR_const__OutputArrayR_int_int_double_double_double_double_double_double(const cv::_InputArray* point, const cv::_OutputArray* lines, int lines_max, int threshold, double min_rho, double max_rho, double rho_step, double min_theta, double max_theta, double theta_step, ResultVoid* ocvrs_return) {
		try {
			cv::HoughLinesPointSet(*point, *lines, lines_max, threshold, min_rho, max_rho, rho_step, min_theta, max_theta, theta_step);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HoughLines(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2170
	// ("cv::HoughLines", vec![(pred!(mut, ["image", "lines", "rho", "theta", "threshold"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "int"]), _)]),
	void cv_HoughLines_const__InputArrayR_const__OutputArrayR_double_double_int(const cv::_InputArray* image, const cv::_OutputArray* lines, double rho, double theta, int threshold, ResultVoid* ocvrs_return) {
		try {
			cv::HoughLines(*image, *lines, rho, theta, threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// HoughLines(InputArray, OutputArray, double, double, int, double, double, double, double, bool)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2170
	// ("cv::HoughLines", vec![(pred!(mut, ["image", "lines", "rho", "theta", "threshold", "srn", "stn", "min_theta", "max_theta", "use_edgeval"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "int", "double", "double", "double", "double", "bool"]), _)]),
	void cv_HoughLines_const__InputArrayR_const__OutputArrayR_double_double_int_double_double_double_double_bool(const cv::_InputArray* image, const cv::_OutputArray* lines, double rho, double theta, int threshold, double srn, double stn, double min_theta, double max_theta, bool use_edgeval, ResultVoid* ocvrs_return) {
		try {
			cv::HoughLines(*image, *lines, rho, theta, threshold, srn, stn, min_theta, max_theta, use_edgeval);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// HuMoments(const Moments &, OutputArray)(SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3835
	// ("cv::HuMoments", vec![(pred!(mut, ["m", "hu"], ["const cv::Moments*", "const cv::_OutputArray*"]), _)]),
	void cv_HuMoments_const_MomentsR_const__OutputArrayR(const cv::Moments* m, const cv::_OutputArray* hu, ResultVoid* ocvrs_return) {
		try {
			cv::HuMoments(*m, *hu);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// HuMoments(const Moments &, double *)(SimpleClass, FixedArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3832
	// ("cv::HuMoments", vec![(pred!(mut, ["moments", "hu"], ["const cv::Moments*", "double**"]), _)]),
	void cv_HuMoments_const_MomentsR_doubleXX(const cv::Moments* moments, double(*hu)[7], ResultVoid* ocvrs_return) {
		try {
			cv::HuMoments(*moments, *hu);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Laplacian(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1856
	// ("cv::Laplacian", vec![(pred!(mut, ["src", "dst", "ddepth"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_Laplacian_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, ResultVoid* ocvrs_return) {
		try {
			cv::Laplacian(*src, *dst, ddepth);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Laplacian(InputArray, OutputArray, int, int, double, double, int)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1856
	// ("cv::Laplacian", vec![(pred!(mut, ["src", "dst", "ddepth", "ksize", "scale", "delta", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "double", "double", "int"]), _)]),
	void cv_Laplacian_const__InputArrayR_const__OutputArrayR_int_int_double_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, int ksize, double scale, double delta, int borderType, ResultVoid* ocvrs_return) {
		try {
			cv::Laplacian(*src, *dst, ddepth, ksize, scale, delta, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Scharr(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1825
	// ("cv::Scharr", vec![(pred!(mut, ["src", "dst", "ddepth", "dx", "dy"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
	void cv_Scharr_const__InputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, int dx, int dy, ResultVoid* ocvrs_return) {
		try {
			cv::Scharr(*src, *dst, ddepth, dx, dy);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Scharr(InputArray, OutputArray, int, int, int, double, double, int)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1825
	// ("cv::Scharr", vec![(pred!(mut, ["src", "dst", "ddepth", "dx", "dy", "scale", "delta", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int", "double", "double", "int"]), _)]),
	void cv_Scharr_const__InputArrayR_const__OutputArrayR_int_int_int_double_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, int dx, int dy, double scale, double delta, int borderType, ResultVoid* ocvrs_return) {
		try {
			cv::Scharr(*src, *dst, ddepth, dx, dy, scale, delta, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Sobel(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1775
	// ("cv::Sobel", vec![(pred!(mut, ["src", "dst", "ddepth", "dx", "dy"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
	void cv_Sobel_const__InputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, int dx, int dy, ResultVoid* ocvrs_return) {
		try {
			cv::Sobel(*src, *dst, ddepth, dx, dy);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Sobel(InputArray, OutputArray, int, int, int, int, double, double, int)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1775
	// ("cv::Sobel", vec![(pred!(mut, ["src", "dst", "ddepth", "dx", "dy", "ksize", "scale", "delta", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int", "int", "double", "double", "int"]), _)]),
	void cv_Sobel_const__InputArrayR_const__OutputArrayR_int_int_int_int_double_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, int dx, int dy, int ksize, double scale, double delta, int borderType, ResultVoid* ocvrs_return) {
		try {
			cv::Sobel(*src, *dst, ddepth, dx, dy, ksize, scale, delta, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::accumulateProduct(InputArray, InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2960
	// ("cv::accumulateProduct", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_accumulateProduct_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_InputOutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::accumulateProduct(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// accumulateProduct(InputArray, InputArray, InputOutputArray, InputArray)(InputArray, InputArray, InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2960
	// ("cv::accumulateProduct", vec![(pred!(mut, ["src1", "src2", "dst", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_accumulateProduct_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_InputOutputArray* dst, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			cv::accumulateProduct(*src1, *src2, *dst, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::accumulateSquare(InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2941
	// ("cv::accumulateSquare", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_accumulateSquare_const__InputArrayR_const__InputOutputArrayR(const cv::_InputArray* src, const cv::_InputOutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::accumulateSquare(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// accumulateSquare(InputArray, InputOutputArray, InputArray)(InputArray, InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2941
	// ("cv::accumulateSquare", vec![(pred!(mut, ["src", "dst", "mask"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_accumulateSquare_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_InputOutputArray* dst, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			cv::accumulateSquare(*src, *dst, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::accumulateWeighted(InputArray, InputOutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2981
	// ("cv::accumulateWeighted", vec![(pred!(mut, ["src", "dst", "alpha"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "double"]), _)]),
	void cv_accumulateWeighted_const__InputArrayR_const__InputOutputArrayR_double(const cv::_InputArray* src, const cv::_InputOutputArray* dst, double alpha, ResultVoid* ocvrs_return) {
		try {
			cv::accumulateWeighted(*src, *dst, alpha);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// accumulateWeighted(InputArray, InputOutputArray, double, InputArray)(InputArray, InputOutputArray, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2981
	// ("cv::accumulateWeighted", vec![(pred!(mut, ["src", "dst", "alpha", "mask"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "double", "const cv::_InputArray*"]), _)]),
	void cv_accumulateWeighted_const__InputArrayR_const__InputOutputArrayR_double_const__InputArrayR(const cv::_InputArray* src, const cv::_InputOutputArray* dst, double alpha, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			cv::accumulateWeighted(*src, *dst, alpha, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::accumulate(InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2922
	// ("cv::accumulate", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_accumulate_const__InputArrayR_const__InputOutputArrayR(const cv::_InputArray* src, const cv::_InputOutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::accumulate(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// accumulate(InputArray, InputOutputArray, InputArray)(InputArray, InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2922
	// ("cv::accumulate", vec![(pred!(mut, ["src", "dst", "mask"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_accumulate_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_InputOutputArray* dst, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			cv::accumulate(*src, *dst, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// adaptiveThreshold(InputArray, OutputArray, double, int, int, int, double)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3113
	// ("cv::adaptiveThreshold", vec![(pred!(mut, ["src", "dst", "maxValue", "adaptiveMethod", "thresholdType", "blockSize", "C"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "int", "int", "int", "double"]), _)]),
	void cv_adaptiveThreshold_const__InputArrayR_const__OutputArrayR_double_int_int_int_double(const cv::_InputArray* src, const cv::_OutputArray* dst, double maxValue, int adaptiveMethod, int thresholdType, int blockSize, double C, ResultVoid* ocvrs_return) {
		try {
			cv::adaptiveThreshold(*src, *dst, maxValue, adaptiveMethod, thresholdType, blockSize, C);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// applyColorMap(InputArray, OutputArray, InputArray)(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4517
	// ("cv::applyColorMap", vec![(pred!(mut, ["src", "dst", "userColor"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_applyColorMap_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* userColor, ResultVoid* ocvrs_return) {
		try {
			cv::applyColorMap(*src, *dst, *userColor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// applyColorMap(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4509
	// ("cv::applyColorMap", vec![(pred!(mut, ["src", "dst", "colormap"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_applyColorMap_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int colormap, ResultVoid* ocvrs_return) {
		try {
			cv::applyColorMap(*src, *dst, colormap);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// approxPolyDP(InputArray, OutputArray, double, bool)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4063
	// ("cv::approxPolyDP", vec![(pred!(mut, ["curve", "approxCurve", "epsilon", "closed"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "bool"]), _)]),
	void cv_approxPolyDP_const__InputArrayR_const__OutputArrayR_double_bool(const cv::_InputArray* curve, const cv::_OutputArray* approxCurve, double epsilon, bool closed, ResultVoid* ocvrs_return) {
		try {
			cv::approxPolyDP(*curve, *approxCurve, epsilon, closed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::approxPolyN(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4085
	// ("cv::approxPolyN", vec![(pred!(mut, ["curve", "approxCurve", "nsides"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_approxPolyN_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* curve, const cv::_OutputArray* approxCurve, int nsides, ResultVoid* ocvrs_return) {
		try {
			cv::approxPolyN(*curve, *approxCurve, nsides);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// approxPolyN(InputArray, OutputArray, int, float, bool)(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4085
	// ("cv::approxPolyN", vec![(pred!(mut, ["curve", "approxCurve", "nsides", "epsilon_percentage", "ensure_convex"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "float", "bool"]), _)]),
	void cv_approxPolyN_const__InputArrayR_const__OutputArrayR_int_float_bool(const cv::_InputArray* curve, const cv::_OutputArray* approxCurve, int nsides, float epsilon_percentage, bool ensure_convex, ResultVoid* ocvrs_return) {
		try {
			cv::approxPolyN(*curve, *approxCurve, nsides, epsilon_percentage, ensure_convex);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// arcLength(InputArray, bool)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4096
	// ("cv::arcLength", vec![(pred!(mut, ["curve", "closed"], ["const cv::_InputArray*", "bool"]), _)]),
	void cv_arcLength_const__InputArrayR_bool(const cv::_InputArray* curve, bool closed, Result<double>* ocvrs_return) {
		try {
			double ret = cv::arcLength(*curve, closed);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::arrowedLine(InputOutputArray, SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4559
	// ("cv::arrowedLine", vec![(pred!(mut, ["img", "pt1", "pt2", "color"], ["const cv::_InputOutputArray*", "cv::Point", "cv::Point", "const cv::Scalar*"]), _)]),
	void cv_arrowedLine_const__InputOutputArrayR_Point_Point_const_ScalarR(const cv::_InputOutputArray* img, cv::Point* pt1, cv::Point* pt2, const cv::Scalar* color, ResultVoid* ocvrs_return) {
		try {
			cv::arrowedLine(*img, *pt1, *pt2, *color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// arrowedLine(InputOutputArray, Point, Point, const Scalar &, int, int, int, double)(InputOutputArray, SimpleClass, SimpleClass, SimpleClass, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4559
	// ("cv::arrowedLine", vec![(pred!(mut, ["img", "pt1", "pt2", "color", "thickness", "line_type", "shift", "tipLength"], ["const cv::_InputOutputArray*", "cv::Point", "cv::Point", "const cv::Scalar*", "int", "int", "int", "double"]), _)]),
	void cv_arrowedLine_const__InputOutputArrayR_Point_Point_const_ScalarR_int_int_int_double(const cv::_InputOutputArray* img, cv::Point* pt1, cv::Point* pt2, const cv::Scalar* color, int thickness, int line_type, int shift, double tipLength, ResultVoid* ocvrs_return) {
		try {
			cv::arrowedLine(*img, *pt1, *pt2, *color, thickness, line_type, shift, tipLength);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bilateralFilter(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1576
	// ("cv::bilateralFilter", vec![(pred!(mut, ["src", "dst", "d", "sigmaColor", "sigmaSpace"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double"]), _)]),
	void cv_bilateralFilter_const__InputArrayR_const__OutputArrayR_int_double_double(const cv::_InputArray* src, const cv::_OutputArray* dst, int d, double sigmaColor, double sigmaSpace, ResultVoid* ocvrs_return) {
		try {
			cv::bilateralFilter(*src, *dst, d, sigmaColor, sigmaSpace);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bilateralFilter(InputArray, OutputArray, int, double, double, int)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1576
	// ("cv::bilateralFilter", vec![(pred!(mut, ["src", "dst", "d", "sigmaColor", "sigmaSpace", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double", "int"]), _)]),
	void cv_bilateralFilter_const__InputArrayR_const__OutputArrayR_int_double_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int d, double sigmaColor, double sigmaSpace, int borderType, ResultVoid* ocvrs_return) {
		try {
			cv::bilateralFilter(*src, *dst, d, sigmaColor, sigmaSpace, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// blendLinear(InputArray, InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3686
	// ("cv::blendLinear", vec![(pred!(mut, ["src1", "src2", "weights1", "weights2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_blendLinear_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_InputArray* weights1, const cv::_InputArray* weights2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::blendLinear(*src1, *src2, *weights1, *weights2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::blur(InputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1650
	// ("cv::blur", vec![(pred!(mut, ["src", "dst", "ksize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size"]), _)]),
	void cv_blur_const__InputArrayR_const__OutputArrayR_Size(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Size* ksize, ResultVoid* ocvrs_return) {
		try {
			cv::blur(*src, *dst, *ksize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// blur(InputArray, OutputArray, Size, Point, int)(InputArray, OutputArray, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1650
	// ("cv::blur", vec![(pred!(mut, ["src", "dst", "ksize", "anchor", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size", "cv::Point", "int"]), _)]),
	void cv_blur_const__InputArrayR_const__OutputArrayR_Size_Point_int(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Size* ksize, cv::Point* anchor, int borderType, ResultVoid* ocvrs_return) {
		try {
			cv::blur(*src, *dst, *ksize, *anchor, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// boundingRect(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4105
	// ("cv::boundingRect", vec![(pred!(mut, ["array"], ["const cv::_InputArray*"]), _)]),
	void cv_boundingRect_const__InputArrayR(const cv::_InputArray* array, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = cv::boundingRect(*array);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::boxFilter(InputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1604
	// ("cv::boxFilter", vec![(pred!(mut, ["src", "dst", "ddepth", "ksize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "cv::Size"]), _)]),
	void cv_boxFilter_const__InputArrayR_const__OutputArrayR_int_Size(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, cv::Size* ksize, ResultVoid* ocvrs_return) {
		try {
			cv::boxFilter(*src, *dst, ddepth, *ksize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// boxFilter(InputArray, OutputArray, int, Size, Point, bool, int)(InputArray, OutputArray, Primitive, SimpleClass, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1604
	// ("cv::boxFilter", vec![(pred!(mut, ["src", "dst", "ddepth", "ksize", "anchor", "normalize", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "cv::Size", "cv::Point", "bool", "int"]), _)]),
	void cv_boxFilter_const__InputArrayR_const__OutputArrayR_int_Size_Point_bool_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, cv::Size* ksize, cv::Point* anchor, bool normalize, int borderType, ResultVoid* ocvrs_return) {
		try {
			cv::boxFilter(*src, *dst, ddepth, *ksize, *anchor, normalize, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// boxPoints(RotatedRect, OutputArray)(SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4158
	// ("cv::boxPoints", vec![(pred!(mut, ["box", "points"], ["cv::RotatedRect", "const cv::_OutputArray*"]), _)]),
	void cv_boxPoints_RotatedRect_const__OutputArrayR(cv::RotatedRect* box, const cv::_OutputArray* points, ResultVoid* ocvrs_return) {
		try {
			cv::boxPoints(*box, *points);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::buildPyramid(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3179
	// ("cv::buildPyramid", vec![(pred!(mut, ["src", "dst", "maxlevel"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_buildPyramid_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int maxlevel, ResultVoid* ocvrs_return) {
		try {
			cv::buildPyramid(*src, *dst, maxlevel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// buildPyramid(InputArray, OutputArrayOfArrays, int, int)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3179
	// ("cv::buildPyramid", vec![(pred!(mut, ["src", "dst", "maxlevel", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_buildPyramid_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int maxlevel, int borderType, ResultVoid* ocvrs_return) {
		try {
			cv::buildPyramid(*src, *dst, maxlevel, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calcBackProject(InputArrayOfArrays, const std::vector<int> &, InputArray, OutputArray, const std::vector<float> &, double)(InputArray, CppPassByVoidPtr, InputArray, OutputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3305
	// ("cv::calcBackProject", vec![(pred!(mut, ["images", "channels", "hist", "dst", "ranges", "scale"], ["const cv::_InputArray*", "const std::vector<int>*", "const cv::_InputArray*", "const cv::_OutputArray*", "const std::vector<float>*", "double"]), _)]),
	void cv_calcBackProject_const__InputArrayR_const_vectorLintGR_const__InputArrayR_const__OutputArrayR_const_vectorLfloatGR_double(const cv::_InputArray* images, const std::vector<int>* channels, const cv::_InputArray* hist, const cv::_OutputArray* dst, const std::vector<float>* ranges, double scale, ResultVoid* ocvrs_return) {
		try {
			cv::calcBackProject(*images, *channels, *hist, *dst, *ranges, scale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::calcHist(InputArray, CppPassByVoidPtr, InputArray, OutputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3247
	// ("cv::calcHist", vec![(pred!(mut, ["images", "channels", "mask", "hist", "histSize", "ranges"], ["const cv::_InputArray*", "const std::vector<int>*", "const cv::_InputArray*", "const cv::_OutputArray*", "const std::vector<int>*", "const std::vector<float>*"]), _)]),
	void cv_calcHist_const__InputArrayR_const_vectorLintGR_const__InputArrayR_const__OutputArrayR_const_vectorLintGR_const_vectorLfloatGR(const cv::_InputArray* images, const std::vector<int>* channels, const cv::_InputArray* mask, const cv::_OutputArray* hist, const std::vector<int>* histSize, const std::vector<float>* ranges, ResultVoid* ocvrs_return) {
		try {
			cv::calcHist(*images, *channels, *mask, *hist, *histSize, *ranges);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calcHist(InputArrayOfArrays, const std::vector<int> &, InputArray, OutputArray, const std::vector<int> &, const std::vector<float> &, bool)(InputArray, CppPassByVoidPtr, InputArray, OutputArray, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3247
	// ("cv::calcHist", vec![(pred!(mut, ["images", "channels", "mask", "hist", "histSize", "ranges", "accumulate"], ["const cv::_InputArray*", "const std::vector<int>*", "const cv::_InputArray*", "const cv::_OutputArray*", "const std::vector<int>*", "const std::vector<float>*", "bool"]), _)]),
	void cv_calcHist_const__InputArrayR_const_vectorLintGR_const__InputArrayR_const__OutputArrayR_const_vectorLintGR_const_vectorLfloatGR_bool(const cv::_InputArray* images, const std::vector<int>* channels, const cv::_InputArray* mask, const cv::_OutputArray* hist, const std::vector<int>* histSize, const std::vector<float>* ranges, bool accumulate, ResultVoid* ocvrs_return) {
		try {
			cv::calcHist(*images, *channels, *mask, *hist, *histSize, *ranges, accumulate);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::circle(InputOutputArray, SimpleClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4605
	// ("cv::circle", vec![(pred!(mut, ["img", "center", "radius", "color"], ["const cv::_InputOutputArray*", "cv::Point", "int", "const cv::Scalar*"]), _)]),
	void cv_circle_const__InputOutputArrayR_Point_int_const_ScalarR(const cv::_InputOutputArray* img, cv::Point* center, int radius, const cv::Scalar* color, ResultVoid* ocvrs_return) {
		try {
			cv::circle(*img, *center, radius, *color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// circle(InputOutputArray, Point, int, const Scalar &, int, int, int)(InputOutputArray, SimpleClass, Primitive, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4605
	// ("cv::circle", vec![(pred!(mut, ["img", "center", "radius", "color", "thickness", "lineType", "shift"], ["const cv::_InputOutputArray*", "cv::Point", "int", "const cv::Scalar*", "int", "int", "int"]), _)]),
	void cv_circle_const__InputOutputArrayR_Point_int_const_ScalarR_int_int_int(const cv::_InputOutputArray* img, cv::Point* center, int radius, const cv::Scalar* color, int thickness, int lineType, int shift, ResultVoid* ocvrs_return) {
		try {
			cv::circle(*img, *center, radius, *color, thickness, lineType, shift);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clipLine(Rect, Point &, Point &)(SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4814
	// ("cv::clipLine", vec![(pred!(mut, ["imgRect", "pt1", "pt2"], ["cv::Rect", "cv::Point*", "cv::Point*"]), _)]),
	void cv_clipLine_Rect_PointR_PointR(cv::Rect* imgRect, cv::Point* pt1, cv::Point* pt2, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::clipLine(*imgRect, *pt1, *pt2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clipLine(Size2l, Point2l &, Point2l &)(SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4807
	// ("cv::clipLine", vec![(pred!(mut, ["imgSize", "pt1", "pt2"], ["cv::Size2l", "cv::Point2l*", "cv::Point2l*"]), _)]),
	void cv_clipLine_Size2l_Point2lR_Point2lR(cv::Size2l* imgSize, cv::Point2l* pt1, cv::Point2l* pt2, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::clipLine(*imgSize, *pt1, *pt2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clipLine(Size, Point &, Point &)(SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4800
	// ("cv::clipLine", vec![(pred!(mut, ["imgSize", "pt1", "pt2"], ["cv::Size", "cv::Point*", "cv::Point*"]), _)]),
	void cv_clipLine_Size_PointR_PointR(cv::Size* imgSize, cv::Point* pt1, cv::Point* pt2, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::clipLine(*imgSize, *pt1, *pt2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compareHist(const SparseMat &, const SparseMat &, int)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3328
	// ("cv::compareHist", vec![(pred!(mut, ["H1", "H2", "method"], ["const cv::SparseMat*", "const cv::SparseMat*", "int"]), _)]),
	void cv_compareHist_const_SparseMatR_const_SparseMatR_int(const cv::SparseMat* H1, const cv::SparseMat* H2, int method, Result<double>* ocvrs_return) {
		try {
			double ret = cv::compareHist(*H1, *H2, method);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compareHist(InputArray, InputArray, int)(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3325
	// ("cv::compareHist", vec![(pred!(mut, ["H1", "H2", "method"], ["const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
	void cv_compareHist_const__InputArrayR_const__InputArrayR_int(const cv::_InputArray* H1, const cv::_InputArray* H2, int method, Result<double>* ocvrs_return) {
		try {
			double ret = cv::compareHist(*H1, *H2, method);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::connectedComponentsWithStats(InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3998
	// ("cv::connectedComponentsWithStats", vec![(pred!(mut, ["image", "labels", "stats", "centroids"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_connectedComponentsWithStats_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* image, const cv::_OutputArray* labels, const cv::_OutputArray* stats, const cv::_OutputArray* centroids, Result<int>* ocvrs_return) {
		try {
			int ret = cv::connectedComponentsWithStats(*image, *labels, *stats, *centroids);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// connectedComponentsWithStats(InputArray, OutputArray, OutputArray, OutputArray, int, int)(InputArray, OutputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3998
	// ("cv::connectedComponentsWithStats", vec![(pred!(mut, ["image", "labels", "stats", "centroids", "connectivity", "ltype"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_connectedComponentsWithStats_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* image, const cv::_OutputArray* labels, const cv::_OutputArray* stats, const cv::_OutputArray* centroids, int connectivity, int ltype, Result<int>* ocvrs_return) {
		try {
			int ret = cv::connectedComponentsWithStats(*image, *labels, *stats, *centroids, connectivity, ltype);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// connectedComponentsWithStats(InputArray, OutputArray, OutputArray, OutputArray, int, int, int)(InputArray, OutputArray, OutputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3983
	// ("cv::connectedComponentsWithStats", vec![(pred!(mut, ["image", "labels", "stats", "centroids", "connectivity", "ltype", "ccltype"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
	void cv_connectedComponentsWithStats_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* image, const cv::_OutputArray* labels, const cv::_OutputArray* stats, const cv::_OutputArray* centroids, int connectivity, int ltype, int ccltype, Result<int>* ocvrs_return) {
		try {
			int ret = cv::connectedComponentsWithStats(*image, *labels, *stats, *centroids, connectivity, ltype, ccltype);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::connectedComponents(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3956
	// ("cv::connectedComponents", vec![(pred!(mut, ["image", "labels"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_connectedComponents_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* image, const cv::_OutputArray* labels, Result<int>* ocvrs_return) {
		try {
			int ret = cv::connectedComponents(*image, *labels);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// connectedComponents(InputArray, OutputArray, int, int)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3956
	// ("cv::connectedComponents", vec![(pred!(mut, ["image", "labels", "connectivity", "ltype"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_connectedComponents_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* image, const cv::_OutputArray* labels, int connectivity, int ltype, Result<int>* ocvrs_return) {
		try {
			int ret = cv::connectedComponents(*image, *labels, connectivity, ltype);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// connectedComponents(InputArray, OutputArray, int, int, int)(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3945
	// ("cv::connectedComponents", vec![(pred!(mut, ["image", "labels", "connectivity", "ltype", "ccltype"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
	void cv_connectedComponents_const__InputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* image, const cv::_OutputArray* labels, int connectivity, int ltype, int ccltype, Result<int>* ocvrs_return) {
		try {
			int ret = cv::connectedComponents(*image, *labels, connectivity, ltype, ccltype);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::contourArea(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4137
	// ("cv::contourArea", vec![(pred!(mut, ["contour"], ["const cv::_InputArray*"]), _)]),
	void cv_contourArea_const__InputArrayR(const cv::_InputArray* contour, Result<double>* ocvrs_return) {
		try {
			double ret = cv::contourArea(*contour);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// contourArea(InputArray, bool)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4137
	// ("cv::contourArea", vec![(pred!(mut, ["contour", "oriented"], ["const cv::_InputArray*", "bool"]), _)]),
	void cv_contourArea_const__InputArrayR_bool(const cv::_InputArray* contour, bool oriented, Result<double>* ocvrs_return) {
		try {
			double ret = cv::contourArea(*contour, oriented);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::convertMaps(InputArray, InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2560
	// ("cv::convertMaps", vec![(pred!(mut, ["map1", "map2", "dstmap1", "dstmap2", "dstmap1type"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_convertMaps_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int(const cv::_InputArray* map1, const cv::_InputArray* map2, const cv::_OutputArray* dstmap1, const cv::_OutputArray* dstmap2, int dstmap1type, ResultVoid* ocvrs_return) {
		try {
			cv::convertMaps(*map1, *map2, *dstmap1, *dstmap2, dstmap1type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertMaps(InputArray, InputArray, OutputArray, OutputArray, int, bool)(InputArray, InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2560
	// ("cv::convertMaps", vec![(pred!(mut, ["map1", "map2", "dstmap1", "dstmap2", "dstmap1type", "nninterpolation"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "bool"]), _)]),
	void cv_convertMaps_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_bool(const cv::_InputArray* map1, const cv::_InputArray* map2, const cv::_OutputArray* dstmap1, const cv::_OutputArray* dstmap2, int dstmap1type, bool nninterpolation, ResultVoid* ocvrs_return) {
		try {
			cv::convertMaps(*map1, *map2, *dstmap1, *dstmap2, dstmap1type, nninterpolation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::convexHull(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4238
	// ("cv::convexHull", vec![(pred!(mut, ["points", "hull"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_convexHull_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* points, const cv::_OutputArray* hull, ResultVoid* ocvrs_return) {
		try {
			cv::convexHull(*points, *hull);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convexHull(InputArray, OutputArray, bool, bool)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4238
	// ("cv::convexHull", vec![(pred!(mut, ["points", "hull", "clockwise", "returnPoints"], ["const cv::_InputArray*", "const cv::_OutputArray*", "bool", "bool"]), _)]),
	void cv_convexHull_const__InputArrayR_const__OutputArrayR_bool_bool(const cv::_InputArray* points, const cv::_OutputArray* hull, bool clockwise, bool returnPoints, ResultVoid* ocvrs_return) {
		try {
			cv::convexHull(*points, *hull, clockwise, returnPoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convexityDefects(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4258
	// ("cv::convexityDefects", vec![(pred!(mut, ["contour", "convexhull", "convexityDefects"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_convexityDefects_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* contour, const cv::_InputArray* convexhull, const cv::_OutputArray* convexityDefects, ResultVoid* ocvrs_return) {
		try {
			cv::convexityDefects(*contour, *convexhull, *convexityDefects);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cornerEigenValsAndVecs(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1977
	// ("cv::cornerEigenValsAndVecs", vec![(pred!(mut, ["src", "dst", "blockSize", "ksize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_cornerEigenValsAndVecs_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int blockSize, int ksize, ResultVoid* ocvrs_return) {
		try {
			cv::cornerEigenValsAndVecs(*src, *dst, blockSize, ksize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cornerEigenValsAndVecs(InputArray, OutputArray, int, int, int)(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1977
	// ("cv::cornerEigenValsAndVecs", vec![(pred!(mut, ["src", "dst", "blockSize", "ksize", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
	void cv_cornerEigenValsAndVecs_const__InputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int blockSize, int ksize, int borderType, ResultVoid* ocvrs_return) {
		try {
			cv::cornerEigenValsAndVecs(*src, *dst, blockSize, ksize, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cornerHarris(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1947
	// ("cv::cornerHarris", vec![(pred!(mut, ["src", "dst", "blockSize", "ksize", "k"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "double"]), _)]),
	void cv_cornerHarris_const__InputArrayR_const__OutputArrayR_int_int_double(const cv::_InputArray* src, const cv::_OutputArray* dst, int blockSize, int ksize, double k, ResultVoid* ocvrs_return) {
		try {
			cv::cornerHarris(*src, *dst, blockSize, ksize, k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cornerHarris(InputArray, OutputArray, int, int, double, int)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1947
	// ("cv::cornerHarris", vec![(pred!(mut, ["src", "dst", "blockSize", "ksize", "k", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "double", "int"]), _)]),
	void cv_cornerHarris_const__InputArrayR_const__OutputArrayR_int_int_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int blockSize, int ksize, double k, int borderType, ResultVoid* ocvrs_return) {
		try {
			cv::cornerHarris(*src, *dst, blockSize, ksize, k, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cornerMinEigenVal(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1924
	// ("cv::cornerMinEigenVal", vec![(pred!(mut, ["src", "dst", "blockSize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_cornerMinEigenVal_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int blockSize, ResultVoid* ocvrs_return) {
		try {
			cv::cornerMinEigenVal(*src, *dst, blockSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cornerMinEigenVal(InputArray, OutputArray, int, int, int)(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1924
	// ("cv::cornerMinEigenVal", vec![(pred!(mut, ["src", "dst", "blockSize", "ksize", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
	void cv_cornerMinEigenVal_const__InputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int blockSize, int ksize, int borderType, ResultVoid* ocvrs_return) {
		try {
			cv::cornerMinEigenVal(*src, *dst, blockSize, ksize, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cornerSubPix(InputArray, InputOutputArray, Size, Size, TermCriteria)(InputArray, InputOutputArray, SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2047
	// ("cv::cornerSubPix", vec![(pred!(mut, ["image", "corners", "winSize", "zeroZone", "criteria"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "cv::Size", "cv::Size", "cv::TermCriteria"]), _)]),
	void cv_cornerSubPix_const__InputArrayR_const__InputOutputArrayR_Size_Size_TermCriteria(const cv::_InputArray* image, const cv::_InputOutputArray* corners, cv::Size* winSize, cv::Size* zeroZone, cv::TermCriteria* criteria, ResultVoid* ocvrs_return) {
		try {
			cv::cornerSubPix(*image, *corners, *winSize, *zeroZone, *criteria);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::createCLAHE() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3353
	// ("cv::createCLAHE", vec![(pred!(mut, [], []), _)]),
	void cv_createCLAHE(Result<cv::Ptr<cv::CLAHE>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::CLAHE> ret = cv::createCLAHE();
			Ok(new cv::Ptr<cv::CLAHE>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createCLAHE(double, Size)(Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3353
	// ("cv::createCLAHE", vec![(pred!(mut, ["clipLimit", "tileGridSize"], ["double", "cv::Size"]), _)]),
	void cv_createCLAHE_double_Size(double clipLimit, cv::Size* tileGridSize, Result<cv::Ptr<cv::CLAHE>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::CLAHE> ret = cv::createCLAHE(clipLimit, *tileGridSize);
			Ok(new cv::Ptr<cv::CLAHE>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createGeneralizedHoughBallard()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4461
	// ("cv::createGeneralizedHoughBallard", vec![(pred!(mut, [], []), _)]),
	void cv_createGeneralizedHoughBallard(Result<cv::Ptr<cv::GeneralizedHoughBallard>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::GeneralizedHoughBallard> ret = cv::createGeneralizedHoughBallard();
			Ok(new cv::Ptr<cv::GeneralizedHoughBallard>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createGeneralizedHoughGuil()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4465
	// ("cv::createGeneralizedHoughGuil", vec![(pred!(mut, [], []), _)]),
	void cv_createGeneralizedHoughGuil(Result<cv::Ptr<cv::GeneralizedHoughGuil>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::GeneralizedHoughGuil> ret = cv::createGeneralizedHoughGuil();
			Ok(new cv::Ptr<cv::GeneralizedHoughGuil>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createHanningWindow(OutputArray, Size, int)(OutputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3037
	// ("cv::createHanningWindow", vec![(pred!(mut, ["dst", "winSize", "type"], ["const cv::_OutputArray*", "cv::Size", "int"]), _)]),
	void cv_createHanningWindow_const__OutputArrayR_Size_int(const cv::_OutputArray* dst, cv::Size* winSize, int type, ResultVoid* ocvrs_return) {
		try {
			cv::createHanningWindow(*dst, *winSize, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::createLineSegmentDetector() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1414
	// ("cv::createLineSegmentDetector", vec![(pred!(mut, [], []), _)]),
	void cv_createLineSegmentDetector(Result<cv::Ptr<cv::LineSegmentDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::LineSegmentDetector> ret = cv::createLineSegmentDetector();
			Ok(new cv::Ptr<cv::LineSegmentDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createLineSegmentDetector(int, double, double, double, double, double, double, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1414
	// ("cv::createLineSegmentDetector", vec![(pred!(mut, ["refine", "scale", "sigma_scale", "quant", "ang_th", "log_eps", "density_th", "n_bins"], ["int", "double", "double", "double", "double", "double", "double", "int"]), _)]),
	void cv_createLineSegmentDetector_int_double_double_double_double_double_double_int(int refine, double scale, double sigma_scale, double quant, double ang_th, double log_eps, double density_th, int n_bins, Result<cv::Ptr<cv::LineSegmentDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::LineSegmentDetector> ret = cv::createLineSegmentDetector(refine, scale, sigma_scale, quant, ang_th, log_eps, density_th, n_bins);
			Ok(new cv::Ptr<cv::LineSegmentDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cvtColorTwoPlane(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3756
	// ("cv::cvtColorTwoPlane", vec![(pred!(mut, ["src1", "src2", "dst", "code"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_cvtColorTwoPlane_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, int code, ResultVoid* ocvrs_return) {
		try {
			cv::cvtColorTwoPlane(*src1, *src2, *dst, code);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cvtColorTwoPlane(InputArray, InputArray, OutputArray, int, AlgorithmHint)(InputArray, InputArray, OutputArray, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3756
	// ("cv::cvtColorTwoPlane", vec![(pred!(mut, ["src1", "src2", "dst", "code", "hint"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "cv::AlgorithmHint"]), _)]),
	void cv_cvtColorTwoPlane_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_AlgorithmHint(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, int code, cv::AlgorithmHint hint, ResultVoid* ocvrs_return) {
		try {
			cv::cvtColorTwoPlane(*src1, *src2, *dst, code, hint);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cvtColor(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3735
	// ("cv::cvtColor", vec![(pred!(mut, ["src", "dst", "code"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_cvtColor_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int code, ResultVoid* ocvrs_return) {
		try {
			cv::cvtColor(*src, *dst, code);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cvtColor(InputArray, OutputArray, int, int, AlgorithmHint)(InputArray, OutputArray, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3735
	// ("cv::cvtColor", vec![(pred!(mut, ["src", "dst", "code", "dstCn", "hint"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "cv::AlgorithmHint"]), _)]),
	void cv_cvtColor_const__InputArrayR_const__OutputArrayR_int_int_AlgorithmHint(const cv::_InputArray* src, const cv::_OutputArray* dst, int code, int dstCn, cv::AlgorithmHint hint, ResultVoid* ocvrs_return) {
		try {
			cv::cvtColor(*src, *dst, code, dstCn, hint);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::demosaicing(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3788
	// ("cv::demosaicing", vec![(pred!(mut, ["src", "dst", "code"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_demosaicing_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int code, ResultVoid* ocvrs_return) {
		try {
			cv::demosaicing(*src, *dst, code);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// demosaicing(InputArray, OutputArray, int, int)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3788
	// ("cv::demosaicing", vec![(pred!(mut, ["src", "dst", "code", "dstCn"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_demosaicing_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int code, int dstCn, ResultVoid* ocvrs_return) {
		try {
			cv::demosaicing(*src, *dst, code, dstCn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dilate(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2347
	// ("cv::dilate", vec![(pred!(mut, ["src", "dst", "kernel"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_dilate_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* kernel, ResultVoid* ocvrs_return) {
		try {
			cv::dilate(*src, *dst, *kernel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dilate(InputArray, OutputArray, InputArray, Point, int, int, const Scalar &)(InputArray, OutputArray, InputArray, SimpleClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2347
	// ("cv::dilate", vec![(pred!(mut, ["src", "dst", "kernel", "anchor", "iterations", "borderType", "borderValue"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::Point", "int", "int", "const cv::Scalar*"]), _)]),
	void cv_dilate_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Point_int_int_const_ScalarR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* kernel, cv::Point* anchor, int iterations, int borderType, const cv::Scalar* borderValue, ResultVoid* ocvrs_return) {
		try {
			cv::dilate(*src, *dst, *kernel, *anchor, iterations, borderType, *borderValue);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::distanceTransform(InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3571
	// ("cv::distanceTransform", vec![(pred!(mut, ["src", "dst", "labels", "distanceType", "maskSize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_distanceTransform_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_OutputArray* labels, int distanceType, int maskSize, ResultVoid* ocvrs_return) {
		try {
			cv::distanceTransform(*src, *dst, *labels, distanceType, maskSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// distanceTransform(InputArray, OutputArray, OutputArray, int, int, int)(InputArray, OutputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3571
	// ("cv::distanceTransform", vec![(pred!(mut, ["src", "dst", "labels", "distanceType", "maskSize", "labelType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
	void cv_distanceTransform_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_OutputArray* labels, int distanceType, int maskSize, int labelType, ResultVoid* ocvrs_return) {
		try {
			cv::distanceTransform(*src, *dst, *labels, distanceType, maskSize, labelType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::distanceTransform(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3586
	// ("cv::distanceTransform", vec![(pred!(mut, ["src", "dst", "distanceType", "maskSize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_distanceTransform_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int distanceType, int maskSize, ResultVoid* ocvrs_return) {
		try {
			cv::distanceTransform(*src, *dst, distanceType, maskSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// distanceTransform(InputArray, OutputArray, int, int, int)(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3586
	// ("cv::distanceTransform", vec![(pred!(mut, ["src", "dst", "distanceType", "maskSize", "dstType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
	void cv_distanceTransform_const__InputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int distanceType, int maskSize, int dstType, ResultVoid* ocvrs_return) {
		try {
			cv::distanceTransform(*src, *dst, distanceType, maskSize, dstType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::divSpectrums(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3052
	// ("cv::divSpectrums", vec![(pred!(mut, ["a", "b", "c", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_divSpectrums_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* a, const cv::_InputArray* b, const cv::_OutputArray* c, int flags, ResultVoid* ocvrs_return) {
		try {
			cv::divSpectrums(*a, *b, *c, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// divSpectrums(InputArray, InputArray, OutputArray, int, bool)(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3052
	// ("cv::divSpectrums", vec![(pred!(mut, ["a", "b", "c", "flags", "conjB"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "bool"]), _)]),
	void cv_divSpectrums_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_bool(const cv::_InputArray* a, const cv::_InputArray* b, const cv::_OutputArray* c, int flags, bool conjB, ResultVoid* ocvrs_return) {
		try {
			cv::divSpectrums(*a, *b, *c, flags, conjB);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::drawContours(InputOutputArray, InputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4785
	// ("cv::drawContours", vec![(pred!(mut, ["image", "contours", "contourIdx", "color"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "int", "const cv::Scalar*"]), _)]),
	void cv_drawContours_const__InputOutputArrayR_const__InputArrayR_int_const_ScalarR(const cv::_InputOutputArray* image, const cv::_InputArray* contours, int contourIdx, const cv::Scalar* color, ResultVoid* ocvrs_return) {
		try {
			cv::drawContours(*image, *contours, contourIdx, *color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawContours(InputOutputArray, InputArrayOfArrays, int, const Scalar &, int, int, InputArray, int, Point)(InputOutputArray, InputArray, Primitive, SimpleClass, Primitive, Primitive, InputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4785
	// ("cv::drawContours", vec![(pred!(mut, ["image", "contours", "contourIdx", "color", "thickness", "lineType", "hierarchy", "maxLevel", "offset"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "int", "const cv::Scalar*", "int", "int", "const cv::_InputArray*", "int", "cv::Point"]), _)]),
	void cv_drawContours_const__InputOutputArrayR_const__InputArrayR_int_const_ScalarR_int_int_const__InputArrayR_int_Point(const cv::_InputOutputArray* image, const cv::_InputArray* contours, int contourIdx, const cv::Scalar* color, int thickness, int lineType, const cv::_InputArray* hierarchy, int maxLevel, cv::Point* offset, ResultVoid* ocvrs_return) {
		try {
			cv::drawContours(*image, *contours, contourIdx, *color, thickness, lineType, *hierarchy, maxLevel, *offset);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::drawMarker(InputOutputArray, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4668
	// ("cv::drawMarker", vec![(pred!(mut, ["img", "position", "color"], ["const cv::_InputOutputArray*", "cv::Point", "const cv::Scalar*"]), _)]),
	void cv_drawMarker_const__InputOutputArrayR_Point_const_ScalarR(const cv::_InputOutputArray* img, cv::Point* position, const cv::Scalar* color, ResultVoid* ocvrs_return) {
		try {
			cv::drawMarker(*img, *position, *color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawMarker(InputOutputArray, Point, const Scalar &, int, int, int, int)(InputOutputArray, SimpleClass, SimpleClass, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4668
	// ("cv::drawMarker", vec![(pred!(mut, ["img", "position", "color", "markerType", "markerSize", "thickness", "line_type"], ["const cv::_InputOutputArray*", "cv::Point", "const cv::Scalar*", "int", "int", "int", "int"]), _)]),
	void cv_drawMarker_const__InputOutputArrayR_Point_const_ScalarR_int_int_int_int(const cv::_InputOutputArray* img, cv::Point* position, const cv::Scalar* color, int markerType, int markerSize, int thickness, int line_type, ResultVoid* ocvrs_return) {
		try {
			cv::drawMarker(*img, *position, *color, markerType, markerSize, thickness, line_type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ellipse2Poly(Point2d, Size2d, int, int, int, int, std::vector<Point2d> &)(SimpleClass, SimpleClass, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4843
	// ("cv::ellipse2Poly", vec![(pred!(mut, ["center", "axes", "angle", "arcStart", "arcEnd", "delta", "pts"], ["cv::Point2d", "cv::Size2d", "int", "int", "int", "int", "std::vector<cv::Point2d>*"]), _)]),
	void cv_ellipse2Poly_Point2d_Size2d_int_int_int_int_vectorLPoint2dGR(cv::Point2d* center, cv::Size2d* axes, int angle, int arcStart, int arcEnd, int delta, std::vector<cv::Point2d>* pts, ResultVoid* ocvrs_return) {
		try {
			cv::ellipse2Poly(*center, *axes, angle, arcStart, arcEnd, delta, *pts);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ellipse2Poly(Point, Size, int, int, int, int, std::vector<Point> &)(SimpleClass, SimpleClass, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4830
	// ("cv::ellipse2Poly", vec![(pred!(mut, ["center", "axes", "angle", "arcStart", "arcEnd", "delta", "pts"], ["cv::Point", "cv::Size", "int", "int", "int", "int", "std::vector<cv::Point>*"]), _)]),
	void cv_ellipse2Poly_Point_Size_int_int_int_int_vectorLPointGR(cv::Point* center, cv::Size* axes, int angle, int arcStart, int arcEnd, int delta, std::vector<cv::Point>* pts, ResultVoid* ocvrs_return) {
		try {
			cv::ellipse2Poly(*center, *axes, angle, arcStart, arcEnd, delta, *pts);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ellipse(InputOutputArray, SimpleClass, SimpleClass, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4634
	// ("cv::ellipse", vec![(pred!(mut, ["img", "center", "axes", "angle", "startAngle", "endAngle", "color"], ["const cv::_InputOutputArray*", "cv::Point", "cv::Size", "double", "double", "double", "const cv::Scalar*"]), _)]),
	void cv_ellipse_const__InputOutputArrayR_Point_Size_double_double_double_const_ScalarR(const cv::_InputOutputArray* img, cv::Point* center, cv::Size* axes, double angle, double startAngle, double endAngle, const cv::Scalar* color, ResultVoid* ocvrs_return) {
		try {
			cv::ellipse(*img, *center, *axes, angle, startAngle, endAngle, *color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ellipse(InputOutputArray, Point, Size, double, double, double, const Scalar &, int, int, int)(InputOutputArray, SimpleClass, SimpleClass, Primitive, Primitive, Primitive, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4634
	// ("cv::ellipse", vec![(pred!(mut, ["img", "center", "axes", "angle", "startAngle", "endAngle", "color", "thickness", "lineType", "shift"], ["const cv::_InputOutputArray*", "cv::Point", "cv::Size", "double", "double", "double", "const cv::Scalar*", "int", "int", "int"]), _)]),
	void cv_ellipse_const__InputOutputArrayR_Point_Size_double_double_double_const_ScalarR_int_int_int(const cv::_InputOutputArray* img, cv::Point* center, cv::Size* axes, double angle, double startAngle, double endAngle, const cv::Scalar* color, int thickness, int lineType, int shift, ResultVoid* ocvrs_return) {
		try {
			cv::ellipse(*img, *center, *axes, angle, startAngle, endAngle, *color, thickness, lineType, shift);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ellipse(InputOutputArray, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4648
	// ("cv::ellipse", vec![(pred!(mut, ["img", "box", "color"], ["const cv::_InputOutputArray*", "const cv::RotatedRect*", "const cv::Scalar*"]), _)]),
	void cv_ellipse_const__InputOutputArrayR_const_RotatedRectR_const_ScalarR(const cv::_InputOutputArray* img, const cv::RotatedRect* box, const cv::Scalar* color, ResultVoid* ocvrs_return) {
		try {
			cv::ellipse(*img, *box, *color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ellipse(InputOutputArray, const RotatedRect &, const Scalar &, int, int)(InputOutputArray, SimpleClass, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4648
	// ("cv::ellipse", vec![(pred!(mut, ["img", "box", "color", "thickness", "lineType"], ["const cv::_InputOutputArray*", "const cv::RotatedRect*", "const cv::Scalar*", "int", "int"]), _)]),
	void cv_ellipse_const__InputOutputArrayR_const_RotatedRectR_const_ScalarR_int_int(const cv::_InputOutputArray* img, const cv::RotatedRect* box, const cv::Scalar* color, int thickness, int lineType, ResultVoid* ocvrs_return) {
		try {
			cv::ellipse(*img, *box, *color, thickness, lineType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// equalizeHist(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3345
	// ("cv::equalizeHist", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_equalizeHist_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::equalizeHist(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::erode(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2315
	// ("cv::erode", vec![(pred!(mut, ["src", "dst", "kernel"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_erode_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* kernel, ResultVoid* ocvrs_return) {
		try {
			cv::erode(*src, *dst, *kernel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// erode(InputArray, OutputArray, InputArray, Point, int, int, const Scalar &)(InputArray, OutputArray, InputArray, SimpleClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2315
	// ("cv::erode", vec![(pred!(mut, ["src", "dst", "kernel", "anchor", "iterations", "borderType", "borderValue"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::Point", "int", "int", "const cv::Scalar*"]), _)]),
	void cv_erode_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Point_int_int_const_ScalarR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* kernel, cv::Point* anchor, int iterations, int borderType, const cv::Scalar* borderValue, ResultVoid* ocvrs_return) {
		try {
			cv::erode(*src, *dst, *kernel, *anchor, iterations, borderType, *borderValue);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::fillConvexPoly(InputOutputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4689
	// ("cv::fillConvexPoly", vec![(pred!(mut, ["img", "points", "color"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::Scalar*"]), _)]),
	void cv_fillConvexPoly_const__InputOutputArrayR_const__InputArrayR_const_ScalarR(const cv::_InputOutputArray* img, const cv::_InputArray* points, const cv::Scalar* color, ResultVoid* ocvrs_return) {
		try {
			cv::fillConvexPoly(*img, *points, *color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fillConvexPoly(InputOutputArray, InputArray, const Scalar &, int, int)(InputOutputArray, InputArray, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4689
	// ("cv::fillConvexPoly", vec![(pred!(mut, ["img", "points", "color", "lineType", "shift"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::Scalar*", "int", "int"]), _)]),
	void cv_fillConvexPoly_const__InputOutputArrayR_const__InputArrayR_const_ScalarR_int_int(const cv::_InputOutputArray* img, const cv::_InputArray* points, const cv::Scalar* color, int lineType, int shift, ResultVoid* ocvrs_return) {
		try {
			cv::fillConvexPoly(*img, *points, *color, lineType, shift);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::fillPoly(InputOutputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4716
	// ("cv::fillPoly", vec![(pred!(mut, ["img", "pts", "color"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::Scalar*"]), _)]),
	void cv_fillPoly_const__InputOutputArrayR_const__InputArrayR_const_ScalarR(const cv::_InputOutputArray* img, const cv::_InputArray* pts, const cv::Scalar* color, ResultVoid* ocvrs_return) {
		try {
			cv::fillPoly(*img, *pts, *color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fillPoly(InputOutputArray, InputArrayOfArrays, const Scalar &, int, int, Point)(InputOutputArray, InputArray, SimpleClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4716
	// ("cv::fillPoly", vec![(pred!(mut, ["img", "pts", "color", "lineType", "shift", "offset"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::Scalar*", "int", "int", "cv::Point"]), _)]),
	void cv_fillPoly_const__InputOutputArrayR_const__InputArrayR_const_ScalarR_int_int_Point(const cv::_InputOutputArray* img, const cv::_InputArray* pts, const cv::Scalar* color, int lineType, int shift, cv::Point* offset, ResultVoid* ocvrs_return) {
		try {
			cv::fillPoly(*img, *pts, *color, lineType, shift, *offset);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::filter2D(InputArray, OutputArray, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1701
	// ("cv::filter2D", vec![(pred!(mut, ["src", "dst", "ddepth", "kernel"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "const cv::_InputArray*"]), _)]),
	void cv_filter2D_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, const cv::_InputArray* kernel, ResultVoid* ocvrs_return) {
		try {
			cv::filter2D(*src, *dst, ddepth, *kernel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// filter2D(InputArray, OutputArray, int, InputArray, Point, double, int)(InputArray, OutputArray, Primitive, InputArray, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1701
	// ("cv::filter2D", vec![(pred!(mut, ["src", "dst", "ddepth", "kernel", "anchor", "delta", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "const cv::_InputArray*", "cv::Point", "double", "int"]), _)]),
	void cv_filter2D_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR_Point_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, const cv::_InputArray* kernel, cv::Point* anchor, double delta, int borderType, ResultVoid* ocvrs_return) {
		try {
			cv::filter2D(*src, *dst, ddepth, *kernel, *anchor, delta, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findContoursLinkRuns(InputArray, OutputArrayOfArrays)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4048
	// ("cv::findContoursLinkRuns", vec![(pred!(mut, ["image", "contours"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_findContoursLinkRuns_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* image, const cv::_OutputArray* contours, ResultVoid* ocvrs_return) {
		try {
			cv::findContoursLinkRuns(*image, *contours);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findContoursLinkRuns(InputArray, OutputArrayOfArrays, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4045
	// ("cv::findContoursLinkRuns", vec![(pred!(mut, ["image", "contours", "hierarchy"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_findContoursLinkRuns_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* image, const cv::_OutputArray* contours, const cv::_OutputArray* hierarchy, ResultVoid* ocvrs_return) {
		try {
			cv::findContoursLinkRuns(*image, *contours, *hierarchy);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::findContours(InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4029
	// ("cv::findContours", vec![(pred!(mut, ["image", "contours", "hierarchy", "mode", "method"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_findContours_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* image, const cv::_OutputArray* contours, const cv::_OutputArray* hierarchy, int mode, int method, ResultVoid* ocvrs_return) {
		try {
			cv::findContours(*image, *contours, *hierarchy, mode, method);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findContours(InputArray, OutputArrayOfArrays, OutputArray, int, int, Point)(InputArray, OutputArray, OutputArray, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4029
	// ("cv::findContours", vec![(pred!(mut, ["image", "contours", "hierarchy", "mode", "method", "offset"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "int", "cv::Point"]), _)]),
	void cv_findContours_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int_Point(const cv::_InputArray* image, const cv::_OutputArray* contours, const cv::_OutputArray* hierarchy, int mode, int method, cv::Point* offset, ResultVoid* ocvrs_return) {
		try {
			cv::findContours(*image, *contours, *hierarchy, mode, method, *offset);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::findContours(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4034
	// ("cv::findContours", vec![(pred!(mut, ["image", "contours", "mode", "method"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_findContours_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* image, const cv::_OutputArray* contours, int mode, int method, ResultVoid* ocvrs_return) {
		try {
			cv::findContours(*image, *contours, mode, method);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findContours(InputArray, OutputArrayOfArrays, int, int, Point)(InputArray, OutputArray, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4034
	// ("cv::findContours", vec![(pred!(mut, ["image", "contours", "mode", "method", "offset"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "cv::Point"]), _)]),
	void cv_findContours_const__InputArrayR_const__OutputArrayR_int_int_Point(const cv::_InputArray* image, const cv::_OutputArray* contours, int mode, int method, cv::Point* offset, ResultVoid* ocvrs_return) {
		try {
			cv::findContours(*image, *contours, mode, method, *offset);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fitEllipseAMS(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4340
	// ("cv::fitEllipseAMS", vec![(pred!(mut, ["points"], ["const cv::_InputArray*"]), _)]),
	void cv_fitEllipseAMS_const__InputArrayR(const cv::_InputArray* points, Result<cv::RotatedRect>* ocvrs_return) {
		try {
			cv::RotatedRect ret = cv::fitEllipseAMS(*points);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fitEllipseDirect(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4385
	// ("cv::fitEllipseDirect", vec![(pred!(mut, ["points"], ["const cv::_InputArray*"]), _)]),
	void cv_fitEllipseDirect_const__InputArrayR(const cv::_InputArray* points, Result<cv::RotatedRect>* ocvrs_return) {
		try {
			cv::RotatedRect ret = cv::fitEllipseDirect(*points);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fitEllipse(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4303
	// ("cv::fitEllipse", vec![(pred!(mut, ["points"], ["const cv::_InputArray*"]), _)]),
	void cv_fitEllipse_const__InputArrayR(const cv::_InputArray* points, Result<cv::RotatedRect>* ocvrs_return) {
		try {
			cv::RotatedRect ret = cv::fitEllipse(*points);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fitLine(InputArray, OutputArray, int, double, double, double)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4421
	// ("cv::fitLine", vec![(pred!(mut, ["points", "line", "distType", "param", "reps", "aeps"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double", "double"]), _)]),
	void cv_fitLine_const__InputArrayR_const__OutputArrayR_int_double_double_double(const cv::_InputArray* points, const cv::_OutputArray* line, int distType, double param, double reps, double aeps, ResultVoid* ocvrs_return) {
		try {
			cv::fitLine(*points, *line, distType, param, reps, aeps);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::floodFill(InputOutputArray, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3674
	// ("cv::floodFill", vec![(pred!(mut, ["image", "seedPoint", "newVal"], ["const cv::_InputOutputArray*", "cv::Point", "cv::Scalar"]), _)]),
	void cv_floodFill_const__InputOutputArrayR_Point_Scalar(const cv::_InputOutputArray* image, cv::Point* seedPoint, cv::Scalar* newVal, Result<int>* ocvrs_return) {
		try {
			int ret = cv::floodFill(*image, *seedPoint, *newVal);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// floodFill(InputOutputArray, Point, Scalar, Rect *, Scalar, Scalar, int)(InputOutputArray, SimpleClass, SimpleClass, SimpleClass, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3674
	// ("cv::floodFill", vec![(pred!(mut, ["image", "seedPoint", "newVal", "rect", "loDiff", "upDiff", "flags"], ["const cv::_InputOutputArray*", "cv::Point", "cv::Scalar", "cv::Rect*", "cv::Scalar", "cv::Scalar", "int"]), _)]),
	void cv_floodFill_const__InputOutputArrayR_Point_Scalar_RectX_Scalar_Scalar_int(const cv::_InputOutputArray* image, cv::Point* seedPoint, cv::Scalar* newVal, cv::Rect* rect, cv::Scalar* loDiff, cv::Scalar* upDiff, int flags, Result<int>* ocvrs_return) {
		try {
			int ret = cv::floodFill(*image, *seedPoint, *newVal, rect, *loDiff, *upDiff, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::floodFill(InputOutputArray, InputOutputArray, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3661
	// ("cv::floodFill", vec![(pred!(mut, ["image", "mask", "seedPoint", "newVal"], ["const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::Point", "cv::Scalar"]), _)]),
	void cv_floodFill_const__InputOutputArrayR_const__InputOutputArrayR_Point_Scalar(const cv::_InputOutputArray* image, const cv::_InputOutputArray* mask, cv::Point* seedPoint, cv::Scalar* newVal, Result<int>* ocvrs_return) {
		try {
			int ret = cv::floodFill(*image, *mask, *seedPoint, *newVal);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// floodFill(InputOutputArray, InputOutputArray, Point, Scalar, Rect *, Scalar, Scalar, int)(InputOutputArray, InputOutputArray, SimpleClass, SimpleClass, SimpleClass, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3661
	// ("cv::floodFill", vec![(pred!(mut, ["image", "mask", "seedPoint", "newVal", "rect", "loDiff", "upDiff", "flags"], ["const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "cv::Point", "cv::Scalar", "cv::Rect*", "cv::Scalar", "cv::Scalar", "int"]), _)]),
	void cv_floodFill_const__InputOutputArrayR_const__InputOutputArrayR_Point_Scalar_RectX_Scalar_Scalar_int(const cv::_InputOutputArray* image, const cv::_InputOutputArray* mask, cv::Point* seedPoint, cv::Scalar* newVal, cv::Rect* rect, cv::Scalar* loDiff, cv::Scalar* upDiff, int flags, Result<int>* ocvrs_return) {
		try {
			int ret = cv::floodFill(*image, *mask, *seedPoint, *newVal, rect, *loDiff, *upDiff, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAffineTransform(const Point2f *, const Point2f *)(FixedArray, FixedArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2609
	// ("cv::getAffineTransform", vec![(pred!(mut, ["src", "dst"], ["const cv::Point2f*", "const cv::Point2f*"]), _)]),
	void cv_getAffineTransform_const_Point2fXX_const_Point2fXX(const cv::Point2f(*src)[3], const cv::Point2f(*dst)[3], Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::getAffineTransform(*src, *dst);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAffineTransform(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2646
	// ("cv::getAffineTransform", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_getAffineTransform_const__InputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_InputArray* dst, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::getAffineTransform(*src, *dst);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::getDerivKernels(OutputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1462
	// ("cv::getDerivKernels", vec![(pred!(mut, ["kx", "ky", "dx", "dy", "ksize"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
	void cv_getDerivKernels_const__OutputArrayR_const__OutputArrayR_int_int_int(const cv::_OutputArray* kx, const cv::_OutputArray* ky, int dx, int dy, int ksize, ResultVoid* ocvrs_return) {
		try {
			cv::getDerivKernels(*kx, *ky, dx, dy, ksize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDerivKernels(OutputArray, OutputArray, int, int, int, bool, int)(OutputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1462
	// ("cv::getDerivKernels", vec![(pred!(mut, ["kx", "ky", "dx", "dy", "ksize", "normalize", "ktype"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "int", "int", "int", "bool", "int"]), _)]),
	void cv_getDerivKernels_const__OutputArrayR_const__OutputArrayR_int_int_int_bool_int(const cv::_OutputArray* kx, const cv::_OutputArray* ky, int dx, int dy, int ksize, bool normalize, int ktype, ResultVoid* ocvrs_return) {
		try {
			cv::getDerivKernels(*kx, *ky, dx, dy, ksize, normalize, ktype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::getFontScaleFromHeight(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4928
	// ("cv::getFontScaleFromHeight", vec![(pred!(mut, ["fontFace", "pixelHeight"], ["const int", "const int"]), _)]),
	void cv_getFontScaleFromHeight_const_int_const_int(const int fontFace, const int pixelHeight, Result<double>* ocvrs_return) {
		try {
			double ret = cv::getFontScaleFromHeight(fontFace, pixelHeight);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFontScaleFromHeight(const int, const int, const int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4928
	// ("cv::getFontScaleFromHeight", vec![(pred!(mut, ["fontFace", "pixelHeight", "thickness"], ["const int", "const int", "const int"]), _)]),
	void cv_getFontScaleFromHeight_const_int_const_int_const_int(const int fontFace, const int pixelHeight, const int thickness, Result<double>* ocvrs_return) {
		try {
			double ret = cv::getFontScaleFromHeight(fontFace, pixelHeight, thickness);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::getGaborKernel(SimpleClass, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1479
	// ("cv::getGaborKernel", vec![(pred!(mut, ["ksize", "sigma", "theta", "lambd", "gamma"], ["cv::Size", "double", "double", "double", "double"]), _)]),
	void cv_getGaborKernel_Size_double_double_double_double(cv::Size* ksize, double sigma, double theta, double lambd, double gamma, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::getGaborKernel(*ksize, sigma, theta, lambd, gamma);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGaborKernel(Size, double, double, double, double, double, int)(SimpleClass, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1479
	// ("cv::getGaborKernel", vec![(pred!(mut, ["ksize", "sigma", "theta", "lambd", "gamma", "psi", "ktype"], ["cv::Size", "double", "double", "double", "double", "double", "int"]), _)]),
	void cv_getGaborKernel_Size_double_double_double_double_double_int(cv::Size* ksize, double sigma, double theta, double lambd, double gamma, double psi, int ktype, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::getGaborKernel(*ksize, sigma, theta, lambd, gamma, psi, ktype);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::getGaussianKernel(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1442
	// ("cv::getGaussianKernel", vec![(pred!(mut, ["ksize", "sigma"], ["int", "double"]), _)]),
	void cv_getGaussianKernel_int_double(int ksize, double sigma, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::getGaussianKernel(ksize, sigma);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGaussianKernel(int, double, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1442
	// ("cv::getGaussianKernel", vec![(pred!(mut, ["ksize", "sigma", "ktype"], ["int", "double", "int"]), _)]),
	void cv_getGaussianKernel_int_double_int(int ksize, double sigma, int ktype, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::getGaussianKernel(ksize, sigma, ktype);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::getPerspectiveTransform(FixedArray, FixedArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2643
	// ("cv::getPerspectiveTransform", vec![(pred!(mut, ["src", "dst"], ["const cv::Point2f**", "const cv::Point2f**"]), _)]),
	void cv_getPerspectiveTransform_const_Point2fXX_const_Point2fXX(const cv::Point2f(*src)[4], const cv::Point2f(*dst)[4], Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::getPerspectiveTransform(*src, *dst);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPerspectiveTransform(const Point2f *, const Point2f *, int)(FixedArray, FixedArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2643
	// ("cv::getPerspectiveTransform", vec![(pred!(mut, ["src", "dst", "solveMethod"], ["const cv::Point2f*", "const cv::Point2f*", "int"]), _)]),
	void cv_getPerspectiveTransform_const_Point2fXX_const_Point2fXX_int(const cv::Point2f(*src)[4], const cv::Point2f(*dst)[4], int solveMethod, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::getPerspectiveTransform(*src, *dst, solveMethod);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::getPerspectiveTransform(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2640
	// ("cv::getPerspectiveTransform", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_getPerspectiveTransform_const__InputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_InputArray* dst, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::getPerspectiveTransform(*src, *dst);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPerspectiveTransform(InputArray, InputArray, int)(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2640
	// ("cv::getPerspectiveTransform", vec![(pred!(mut, ["src", "dst", "solveMethod"], ["const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
	void cv_getPerspectiveTransform_const__InputArrayR_const__InputArrayR_int(const cv::_InputArray* src, const cv::_InputArray* dst, int solveMethod, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::getPerspectiveTransform(*src, *dst, solveMethod);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::getRectSubPix(InputArray, SimpleClass, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2668
	// ("cv::getRectSubPix", vec![(pred!(mut, ["image", "patchSize", "center", "patch"], ["const cv::_InputArray*", "cv::Size", "cv::Point2f", "const cv::_OutputArray*"]), _)]),
	void cv_getRectSubPix_const__InputArrayR_Size_Point2f_const__OutputArrayR(const cv::_InputArray* image, cv::Size* patchSize, cv::Point2f* center, const cv::_OutputArray* patch, ResultVoid* ocvrs_return) {
		try {
			cv::getRectSubPix(*image, *patchSize, *center, *patch);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRectSubPix(InputArray, Size, Point2f, OutputArray, int)(InputArray, SimpleClass, SimpleClass, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2668
	// ("cv::getRectSubPix", vec![(pred!(mut, ["image", "patchSize", "center", "patch", "patchType"], ["const cv::_InputArray*", "cv::Size", "cv::Point2f", "const cv::_OutputArray*", "int"]), _)]),
	void cv_getRectSubPix_const__InputArrayR_Size_Point2f_const__OutputArrayR_int(const cv::_InputArray* image, cv::Size* patchSize, cv::Point2f* center, const cv::_OutputArray* patch, int patchType, ResultVoid* ocvrs_return) {
		try {
			cv::getRectSubPix(*image, *patchSize, *center, *patch, patchType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRotationMatrix2D(Point2f, double, double)(SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2583
	// ("cv::getRotationMatrix2D", vec![(pred!(mut, ["center", "angle", "scale"], ["cv::Point2f", "double", "double"]), _)]),
	void cv_getRotationMatrix2D_Point2f_double_double(cv::Point2f* center, double angle, double scale, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::getRotationMatrix2D(*center, angle, scale);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	#if !defined(OCVRS_TARGET_OS_WINDOWS)
	// getRotationMatrix2D_(Point2f, double, double)(SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2586
	// ("cv::getRotationMatrix2D_", vec![(pred!(mut, ["center", "angle", "scale"], ["cv::Point2f", "double", "double"]), _)]),
	void cv_getRotationMatrix2D__Point2f_double_double(cv::Point2f* center, double angle, double scale, Result<cv::Matx23d>* ocvrs_return) {
		try {
			cv::Matx23d ret = cv::getRotationMatrix2D_(*center, angle, scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	#endif

	// cv::getStructuringElement(Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1498
	// ("cv::getStructuringElement", vec![(pred!(mut, ["shape", "ksize"], ["int", "cv::Size"]), _)]),
	void cv_getStructuringElement_int_Size(int shape, cv::Size* ksize, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::getStructuringElement(shape, *ksize);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getStructuringElement(int, Size, Point)(Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1498
	// ("cv::getStructuringElement", vec![(pred!(mut, ["shape", "ksize", "anchor"], ["int", "cv::Size", "cv::Point"]), _)]),
	void cv_getStructuringElement_int_Size_Point(int shape, cv::Size* ksize, cv::Point* anchor, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::getStructuringElement(shape, *ksize, *anchor);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTextSize(const String &, int, double, int, int *)(InString, Primitive, Primitive, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4914
	// ("cv::getTextSize", vec![(pred!(mut, ["text", "fontFace", "fontScale", "thickness", "baseLine"], ["const cv::String*", "int", "double", "int", "int*"]), _)]),
	void cv_getTextSize_const_StringR_int_double_int_intX(const char* text, int fontFace, double fontScale, int thickness, int* baseLine, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = cv::getTextSize(std::string(text), fontFace, fontScale, thickness, baseLine);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::goodFeaturesToTrack(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2095
	// ("cv::goodFeaturesToTrack", vec![(pred!(mut, ["image", "corners", "maxCorners", "qualityLevel", "minDistance"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double"]), _)]),
	void cv_goodFeaturesToTrack_const__InputArrayR_const__OutputArrayR_int_double_double(const cv::_InputArray* image, const cv::_OutputArray* corners, int maxCorners, double qualityLevel, double minDistance, ResultVoid* ocvrs_return) {
		try {
			cv::goodFeaturesToTrack(*image, *corners, maxCorners, qualityLevel, minDistance);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::goodFeaturesToTrack(InputArray, OutputArray, Primitive, Primitive, Primitive, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2131
	// ("cv::goodFeaturesToTrack", vec![(pred!(mut, ["image", "corners", "maxCorners", "qualityLevel", "minDistance", "mask", "cornersQuality"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_goodFeaturesToTrack_const__InputArrayR_const__OutputArrayR_int_double_double_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* image, const cv::_OutputArray* corners, int maxCorners, double qualityLevel, double minDistance, const cv::_InputArray* mask, const cv::_OutputArray* cornersQuality, ResultVoid* ocvrs_return) {
		try {
			cv::goodFeaturesToTrack(*image, *corners, maxCorners, qualityLevel, minDistance, *mask, *cornersQuality);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// goodFeaturesToTrack(InputArray, OutputArray, int, double, double, InputArray, OutputArray, int, int, bool, double)(InputArray, OutputArray, Primitive, Primitive, Primitive, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2131
	// ("cv::goodFeaturesToTrack", vec![(pred!(mut, ["image", "corners", "maxCorners", "qualityLevel", "minDistance", "mask", "cornersQuality", "blockSize", "gradientSize", "useHarrisDetector", "k"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "bool", "double"]), _)]),
	void cv_goodFeaturesToTrack_const__InputArrayR_const__OutputArrayR_int_double_double_const__InputArrayR_const__OutputArrayR_int_int_bool_double(const cv::_InputArray* image, const cv::_OutputArray* corners, int maxCorners, double qualityLevel, double minDistance, const cv::_InputArray* mask, const cv::_OutputArray* cornersQuality, int blockSize, int gradientSize, bool useHarrisDetector, double k, ResultVoid* ocvrs_return) {
		try {
			cv::goodFeaturesToTrack(*image, *corners, maxCorners, qualityLevel, minDistance, *mask, *cornersQuality, blockSize, gradientSize, useHarrisDetector, k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// goodFeaturesToTrack(InputArray, OutputArray, int, double, double, InputArray, int, bool, double)(InputArray, OutputArray, Primitive, Primitive, Primitive, InputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2095
	// ("cv::goodFeaturesToTrack", vec![(pred!(mut, ["image", "corners", "maxCorners", "qualityLevel", "minDistance", "mask", "blockSize", "useHarrisDetector", "k"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double", "const cv::_InputArray*", "int", "bool", "double"]), _)]),
	void cv_goodFeaturesToTrack_const__InputArrayR_const__OutputArrayR_int_double_double_const__InputArrayR_int_bool_double(const cv::_InputArray* image, const cv::_OutputArray* corners, int maxCorners, double qualityLevel, double minDistance, const cv::_InputArray* mask, int blockSize, bool useHarrisDetector, double k, ResultVoid* ocvrs_return) {
		try {
			cv::goodFeaturesToTrack(*image, *corners, maxCorners, qualityLevel, minDistance, *mask, blockSize, useHarrisDetector, k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::goodFeaturesToTrack(InputArray, OutputArray, Primitive, Primitive, Primitive, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2100
	// ("cv::goodFeaturesToTrack", vec![(pred!(mut, ["image", "corners", "maxCorners", "qualityLevel", "minDistance", "mask", "blockSize", "gradientSize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double", "const cv::_InputArray*", "int", "int"]), _)]),
	void cv_goodFeaturesToTrack_const__InputArrayR_const__OutputArrayR_int_double_double_const__InputArrayR_int_int(const cv::_InputArray* image, const cv::_OutputArray* corners, int maxCorners, double qualityLevel, double minDistance, const cv::_InputArray* mask, int blockSize, int gradientSize, ResultVoid* ocvrs_return) {
		try {
			cv::goodFeaturesToTrack(*image, *corners, maxCorners, qualityLevel, minDistance, *mask, blockSize, gradientSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// goodFeaturesToTrack(InputArray, OutputArray, int, double, double, InputArray, int, int, bool, double)(InputArray, OutputArray, Primitive, Primitive, Primitive, InputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2100
	// ("cv::goodFeaturesToTrack", vec![(pred!(mut, ["image", "corners", "maxCorners", "qualityLevel", "minDistance", "mask", "blockSize", "gradientSize", "useHarrisDetector", "k"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "double", "double", "const cv::_InputArray*", "int", "int", "bool", "double"]), _)]),
	void cv_goodFeaturesToTrack_const__InputArrayR_const__OutputArrayR_int_double_double_const__InputArrayR_int_int_bool_double(const cv::_InputArray* image, const cv::_OutputArray* corners, int maxCorners, double qualityLevel, double minDistance, const cv::_InputArray* mask, int blockSize, int gradientSize, bool useHarrisDetector, double k, ResultVoid* ocvrs_return) {
		try {
			cv::goodFeaturesToTrack(*image, *corners, maxCorners, qualityLevel, minDistance, *mask, blockSize, gradientSize, useHarrisDetector, k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::grabCut(InputArray, InputOutputArray, SimpleClass, InputOutputArray, InputOutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3505
	// ("cv::grabCut", vec![(pred!(mut, ["img", "mask", "rect", "bgdModel", "fgdModel", "iterCount"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "cv::Rect", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "int"]), _)]),
	void cv_grabCut_const__InputArrayR_const__InputOutputArrayR_Rect_const__InputOutputArrayR_const__InputOutputArrayR_int(const cv::_InputArray* img, const cv::_InputOutputArray* mask, cv::Rect* rect, const cv::_InputOutputArray* bgdModel, const cv::_InputOutputArray* fgdModel, int iterCount, ResultVoid* ocvrs_return) {
		try {
			cv::grabCut(*img, *mask, *rect, *bgdModel, *fgdModel, iterCount);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// grabCut(InputArray, InputOutputArray, Rect, InputOutputArray, InputOutputArray, int, int)(InputArray, InputOutputArray, SimpleClass, InputOutputArray, InputOutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3505
	// ("cv::grabCut", vec![(pred!(mut, ["img", "mask", "rect", "bgdModel", "fgdModel", "iterCount", "mode"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "cv::Rect", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "int", "int"]), _)]),
	void cv_grabCut_const__InputArrayR_const__InputOutputArrayR_Rect_const__InputOutputArrayR_const__InputOutputArrayR_int_int(const cv::_InputArray* img, const cv::_InputOutputArray* mask, cv::Rect* rect, const cv::_InputOutputArray* bgdModel, const cv::_InputOutputArray* fgdModel, int iterCount, int mode, ResultVoid* ocvrs_return) {
		try {
			cv::grabCut(*img, *mask, *rect, *bgdModel, *fgdModel, iterCount, mode);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::integral(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2894
	// ("cv::integral", vec![(pred!(mut, ["src", "sum"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_integral_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* sum, ResultVoid* ocvrs_return) {
		try {
			cv::integral(*src, *sum);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::integral(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2897
	// ("cv::integral", vec![(pred!(mut, ["src", "sum", "sqsum"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_integral_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* sum, const cv::_OutputArray* sqsum, ResultVoid* ocvrs_return) {
		try {
			cv::integral(*src, *sum, *sqsum);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::integral(InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2889
	// ("cv::integral", vec![(pred!(mut, ["src", "sum", "sqsum", "tilted"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_integral_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* sum, const cv::_OutputArray* sqsum, const cv::_OutputArray* tilted, ResultVoid* ocvrs_return) {
		try {
			cv::integral(*src, *sum, *sqsum, *tilted);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// integral(InputArray, OutputArray, OutputArray, OutputArray, int, int)(InputArray, OutputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2889
	// ("cv::integral", vec![(pred!(mut, ["src", "sum", "sqsum", "tilted", "sdepth", "sqdepth"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_integral_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* sum, const cv::_OutputArray* sqsum, const cv::_OutputArray* tilted, int sdepth, int sqdepth, ResultVoid* ocvrs_return) {
		try {
			cv::integral(*src, *sum, *sqsum, *tilted, sdepth, sqdepth);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// integral(InputArray, OutputArray, OutputArray, int, int)(InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2897
	// ("cv::integral", vec![(pred!(mut, ["src", "sum", "sqsum", "sdepth", "sqdepth"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_integral_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* sum, const cv::_OutputArray* sqsum, int sdepth, int sqdepth, ResultVoid* ocvrs_return) {
		try {
			cv::integral(*src, *sum, *sqsum, sdepth, sqdepth);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// integral(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2894
	// ("cv::integral", vec![(pred!(mut, ["src", "sum", "sdepth"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_integral_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* sum, int sdepth, ResultVoid* ocvrs_return) {
		try {
			cv::integral(*src, *sum, sdepth);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::intersectConvexConvex(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4286
	// ("cv::intersectConvexConvex", vec![(pred!(mut, ["p1", "p2", "p12"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_intersectConvexConvex_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* p1, const cv::_InputArray* p2, const cv::_OutputArray* p12, Result<float>* ocvrs_return) {
		try {
			float ret = cv::intersectConvexConvex(*p1, *p2, *p12);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// intersectConvexConvex(InputArray, InputArray, OutputArray, bool)(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4286
	// ("cv::intersectConvexConvex", vec![(pred!(mut, ["p1", "p2", "p12", "handleNested"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "bool"]), _)]),
	void cv_intersectConvexConvex_const__InputArrayR_const__InputArrayR_const__OutputArrayR_bool(const cv::_InputArray* p1, const cv::_InputArray* p2, const cv::_OutputArray* p12, bool handleNested, Result<float>* ocvrs_return) {
		try {
			float ret = cv::intersectConvexConvex(*p1, *p2, *p12, handleNested);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// invertAffineTransform(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2622
	// ("cv::invertAffineTransform", vec![(pred!(mut, ["M", "iM"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_invertAffineTransform_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* M, const cv::_OutputArray* iM, ResultVoid* ocvrs_return) {
		try {
			cv::invertAffineTransform(*M, *iM);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isContourConvex(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4267
	// ("cv::isContourConvex", vec![(pred!(mut, ["contour"], ["const cv::_InputArray*"]), _)]),
	void cv_isContourConvex_const__InputArrayR(const cv::_InputArray* contour, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::isContourConvex(*contour);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::line(InputOutputArray, SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4543
	// ("cv::line", vec![(pred!(mut, ["img", "pt1", "pt2", "color"], ["const cv::_InputOutputArray*", "cv::Point", "cv::Point", "const cv::Scalar*"]), _)]),
	void cv_line_const__InputOutputArrayR_Point_Point_const_ScalarR(const cv::_InputOutputArray* img, cv::Point* pt1, cv::Point* pt2, const cv::Scalar* color, ResultVoid* ocvrs_return) {
		try {
			cv::line(*img, *pt1, *pt2, *color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// line(InputOutputArray, Point, Point, const Scalar &, int, int, int)(InputOutputArray, SimpleClass, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4543
	// ("cv::line", vec![(pred!(mut, ["img", "pt1", "pt2", "color", "thickness", "lineType", "shift"], ["const cv::_InputOutputArray*", "cv::Point", "cv::Point", "const cv::Scalar*", "int", "int", "int"]), _)]),
	void cv_line_const__InputOutputArrayR_Point_Point_const_ScalarR_int_int_int(const cv::_InputOutputArray* img, cv::Point* pt1, cv::Point* pt2, const cv::Scalar* color, int thickness, int lineType, int shift, ResultVoid* ocvrs_return) {
		try {
			cv::line(*img, *pt1, *pt2, *color, thickness, lineType, shift);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// linearPolar(InputArray, OutputArray, Point2f, double, int)(InputArray, OutputArray, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2755
	// ("cv::linearPolar", vec![(pred!(mut, ["src", "dst", "center", "maxRadius", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Point2f", "double", "int"]), _)]),
	void cv_linearPolar_const__InputArrayR_const__OutputArrayR_Point2f_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Point2f* center, double maxRadius, int flags, ResultVoid* ocvrs_return) {
		try {
			cv::linearPolar(*src, *dst, *center, maxRadius, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// logPolar(InputArray, OutputArray, Point2f, double, int)(InputArray, OutputArray, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2714
	// ("cv::logPolar", vec![(pred!(mut, ["src", "dst", "center", "M", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Point2f", "double", "int"]), _)]),
	void cv_logPolar_const__InputArrayR_const__OutputArrayR_Point2f_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Point2f* center, double M, int flags, ResultVoid* ocvrs_return) {
		try {
			cv::logPolar(*src, *dst, *center, M, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// matchShapes(InputArray, InputArray, int, double)(InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4204
	// ("cv::matchShapes", vec![(pred!(mut, ["contour1", "contour2", "method", "parameter"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "double"]), _)]),
	void cv_matchShapes_const__InputArrayR_const__InputArrayR_int_double(const cv::_InputArray* contour1, const cv::_InputArray* contour2, int method, double parameter, Result<double>* ocvrs_return) {
		try {
			double ret = cv::matchShapes(*contour1, *contour2, method, parameter);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::matchTemplate(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3915
	// ("cv::matchTemplate", vec![(pred!(mut, ["image", "templ", "result", "method"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_matchTemplate_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* image, const cv::_InputArray* templ, const cv::_OutputArray* result, int method, ResultVoid* ocvrs_return) {
		try {
			cv::matchTemplate(*image, *templ, *result, method);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// matchTemplate(InputArray, InputArray, OutputArray, int, InputArray)(InputArray, InputArray, OutputArray, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3915
	// ("cv::matchTemplate", vec![(pred!(mut, ["image", "templ", "result", "method", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "const cv::_InputArray*"]), _)]),
	void cv_matchTemplate_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR(const cv::_InputArray* image, const cv::_InputArray* templ, const cv::_OutputArray* result, int method, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			cv::matchTemplate(*image, *templ, *result, method, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// medianBlur(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1520
	// ("cv::medianBlur", vec![(pred!(mut, ["src", "dst", "ksize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_medianBlur_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ksize, ResultVoid* ocvrs_return) {
		try {
			cv::medianBlur(*src, *dst, ksize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// minAreaRect(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4147
	// ("cv::minAreaRect", vec![(pred!(mut, ["points"], ["const cv::_InputArray*"]), _)]),
	void cv_minAreaRect_const__InputArrayR(const cv::_InputArray* points, Result<cv::RotatedRect>* ocvrs_return) {
		try {
			cv::RotatedRect ret = cv::minAreaRect(*points);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// minEnclosingCircle(InputArray, Point2f &, float &)(InputArray, SimpleClass, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4168
	// ("cv::minEnclosingCircle", vec![(pred!(mut, ["points", "center", "radius"], ["const cv::_InputArray*", "cv::Point2f*", "float*"]), _)]),
	void cv_minEnclosingCircle_const__InputArrayR_Point2fR_floatR(const cv::_InputArray* points, cv::Point2f* center, float* radius, ResultVoid* ocvrs_return) {
		try {
			cv::minEnclosingCircle(*points, *center, *radius);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// minEnclosingTriangle(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4193
	// ("cv::minEnclosingTriangle", vec![(pred!(mut, ["points", "triangle"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_minEnclosingTriangle_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* points, const cv::_OutputArray* triangle, Result<double>* ocvrs_return) {
		try {
			double ret = cv::minEnclosingTriangle(*points, *triangle);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::moments(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3811
	// ("cv::moments", vec![(pred!(mut, ["array"], ["const cv::_InputArray*"]), _)]),
	void cv_moments_const__InputArrayR(const cv::_InputArray* array, Result<cv::Moments>* ocvrs_return) {
		try {
			cv::Moments ret = cv::moments(*array);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// moments(InputArray, bool)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3811
	// ("cv::moments", vec![(pred!(mut, ["array", "binaryImage"], ["const cv::_InputArray*", "bool"]), _)]),
	void cv_moments_const__InputArrayR_bool(const cv::_InputArray* array, bool binaryImage, Result<cv::Moments>* ocvrs_return) {
		try {
			cv::Moments ret = cv::moments(*array, binaryImage);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// morphologyDefaultBorderValue()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1483
	// ("cv::morphologyDefaultBorderValue", vec![(pred!(mut, [], []), _)]),
	void cv_morphologyDefaultBorderValue(Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = cv::morphologyDefaultBorderValue();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::morphologyEx(InputArray, OutputArray, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2376
	// ("cv::morphologyEx", vec![(pred!(mut, ["src", "dst", "op", "kernel"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "const cv::_InputArray*"]), _)]),
	void cv_morphologyEx_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, int op, const cv::_InputArray* kernel, ResultVoid* ocvrs_return) {
		try {
			cv::morphologyEx(*src, *dst, op, *kernel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// morphologyEx(InputArray, OutputArray, int, InputArray, Point, int, int, const Scalar &)(InputArray, OutputArray, Primitive, InputArray, SimpleClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2376
	// ("cv::morphologyEx", vec![(pred!(mut, ["src", "dst", "op", "kernel", "anchor", "iterations", "borderType", "borderValue"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "const cv::_InputArray*", "cv::Point", "int", "int", "const cv::Scalar*"]), _)]),
	void cv_morphologyEx_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR_Point_int_int_const_ScalarR(const cv::_InputArray* src, const cv::_OutputArray* dst, int op, const cv::_InputArray* kernel, cv::Point* anchor, int iterations, int borderType, const cv::Scalar* borderValue, ResultVoid* ocvrs_return) {
		try {
			cv::morphologyEx(*src, *dst, op, *kernel, *anchor, iterations, borderType, *borderValue);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::phaseCorrelate(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3019
	// ("cv::phaseCorrelate", vec![(pred!(mut, ["src1", "src2"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_phaseCorrelate_const__InputArrayR_const__InputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, Result<cv::Point2d>* ocvrs_return) {
		try {
			cv::Point2d ret = cv::phaseCorrelate(*src1, *src2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// phaseCorrelate(InputArray, InputArray, InputArray, double *)(InputArray, InputArray, InputArray, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3019
	// ("cv::phaseCorrelate", vec![(pred!(mut, ["src1", "src2", "window", "response"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "double*"]), _)]),
	void cv_phaseCorrelate_const__InputArrayR_const__InputArrayR_const__InputArrayR_doubleX(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_InputArray* window, double* response, Result<cv::Point2d>* ocvrs_return) {
		try {
			cv::Point2d ret = cv::phaseCorrelate(*src1, *src2, *window, response);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// pointPolygonTest(InputArray, Point2f, bool)(InputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4440
	// ("cv::pointPolygonTest", vec![(pred!(mut, ["contour", "pt", "measureDist"], ["const cv::_InputArray*", "cv::Point2f", "bool"]), _)]),
	void cv_pointPolygonTest_const__InputArrayR_Point2f_bool(const cv::_InputArray* contour, cv::Point2f* pt, bool measureDist, Result<double>* ocvrs_return) {
		try {
			double ret = cv::pointPolygonTest(*contour, *pt, measureDist);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::polylines(InputOutputArray, InputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4739
	// ("cv::polylines", vec![(pred!(mut, ["img", "pts", "isClosed", "color"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "bool", "const cv::Scalar*"]), _)]),
	void cv_polylines_const__InputOutputArrayR_const__InputArrayR_bool_const_ScalarR(const cv::_InputOutputArray* img, const cv::_InputArray* pts, bool isClosed, const cv::Scalar* color, ResultVoid* ocvrs_return) {
		try {
			cv::polylines(*img, *pts, isClosed, *color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// polylines(InputOutputArray, InputArrayOfArrays, bool, const Scalar &, int, int, int)(InputOutputArray, InputArray, Primitive, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4739
	// ("cv::polylines", vec![(pred!(mut, ["img", "pts", "isClosed", "color", "thickness", "lineType", "shift"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "bool", "const cv::Scalar*", "int", "int", "int"]), _)]),
	void cv_polylines_const__InputOutputArrayR_const__InputArrayR_bool_const_ScalarR_int_int_int(const cv::_InputOutputArray* img, const cv::_InputArray* pts, bool isClosed, const cv::Scalar* color, int thickness, int lineType, int shift, ResultVoid* ocvrs_return) {
		try {
			cv::polylines(*img, *pts, isClosed, *color, thickness, lineType, shift);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::preCornerDetect(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2004
	// ("cv::preCornerDetect", vec![(pred!(mut, ["src", "dst", "ksize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_preCornerDetect_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ksize, ResultVoid* ocvrs_return) {
		try {
			cv::preCornerDetect(*src, *dst, ksize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// preCornerDetect(InputArray, OutputArray, int, int)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2004
	// ("cv::preCornerDetect", vec![(pred!(mut, ["src", "dst", "ksize", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_preCornerDetect_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ksize, int borderType, ResultVoid* ocvrs_return) {
		try {
			cv::preCornerDetect(*src, *dst, ksize, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::putText(InputOutputArray, InString, SimpleClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4864
	// ("cv::putText", vec![(pred!(mut, ["img", "text", "org", "fontFace", "fontScale", "color"], ["const cv::_InputOutputArray*", "const cv::String*", "cv::Point", "int", "double", "cv::Scalar"]), _)]),
	void cv_putText_const__InputOutputArrayR_const_StringR_Point_int_double_Scalar(const cv::_InputOutputArray* img, const char* text, cv::Point* org, int fontFace, double fontScale, cv::Scalar* color, ResultVoid* ocvrs_return) {
		try {
			cv::putText(*img, std::string(text), *org, fontFace, fontScale, *color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// putText(InputOutputArray, const String &, Point, int, double, Scalar, int, int, bool)(InputOutputArray, InString, SimpleClass, Primitive, Primitive, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4864
	// ("cv::putText", vec![(pred!(mut, ["img", "text", "org", "fontFace", "fontScale", "color", "thickness", "lineType", "bottomLeftOrigin"], ["const cv::_InputOutputArray*", "const cv::String*", "cv::Point", "int", "double", "cv::Scalar", "int", "int", "bool"]), _)]),
	void cv_putText_const__InputOutputArrayR_const_StringR_Point_int_double_Scalar_int_int_bool(const cv::_InputOutputArray* img, const char* text, cv::Point* org, int fontFace, double fontScale, cv::Scalar* color, int thickness, int lineType, bool bottomLeftOrigin, ResultVoid* ocvrs_return) {
		try {
			cv::putText(*img, std::string(text), *org, fontFace, fontScale, *color, thickness, lineType, bottomLeftOrigin);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::pyrDown(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3145
	// ("cv::pyrDown", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_pyrDown_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::pyrDown(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// pyrDown(InputArray, OutputArray, const Size &, int)(InputArray, OutputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3145
	// ("cv::pyrDown", vec![(pred!(mut, ["src", "dst", "dstsize", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::Size*", "int"]), _)]),
	void cv_pyrDown_const__InputArrayR_const__OutputArrayR_const_SizeR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::Size* dstsize, int borderType, ResultVoid* ocvrs_return) {
		try {
			cv::pyrDown(*src, *dst, *dstsize, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::pyrMeanShiftFiltering(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3473
	// ("cv::pyrMeanShiftFiltering", vec![(pred!(mut, ["src", "dst", "sp", "sr"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
	void cv_pyrMeanShiftFiltering_const__InputArrayR_const__OutputArrayR_double_double(const cv::_InputArray* src, const cv::_OutputArray* dst, double sp, double sr, ResultVoid* ocvrs_return) {
		try {
			cv::pyrMeanShiftFiltering(*src, *dst, sp, sr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// pyrMeanShiftFiltering(InputArray, OutputArray, double, double, int, TermCriteria)(InputArray, OutputArray, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3473
	// ("cv::pyrMeanShiftFiltering", vec![(pred!(mut, ["src", "dst", "sp", "sr", "maxLevel", "termcrit"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "int", "cv::TermCriteria"]), _)]),
	void cv_pyrMeanShiftFiltering_const__InputArrayR_const__OutputArrayR_double_double_int_TermCriteria(const cv::_InputArray* src, const cv::_OutputArray* dst, double sp, double sr, int maxLevel, cv::TermCriteria* termcrit, ResultVoid* ocvrs_return) {
		try {
			cv::pyrMeanShiftFiltering(*src, *dst, sp, sr, maxLevel, *termcrit);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::pyrUp(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3165
	// ("cv::pyrUp", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_pyrUp_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::pyrUp(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// pyrUp(InputArray, OutputArray, const Size &, int)(InputArray, OutputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3165
	// ("cv::pyrUp", vec![(pred!(mut, ["src", "dst", "dstsize", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::Size*", "int"]), _)]),
	void cv_pyrUp_const__InputArrayR_const__OutputArrayR_const_SizeR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::Size* dstsize, int borderType, ResultVoid* ocvrs_return) {
		try {
			cv::pyrUp(*src, *dst, *dstsize, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rectangle(InputOutputArray, SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4576
	// ("cv::rectangle", vec![(pred!(mut, ["img", "pt1", "pt2", "color"], ["const cv::_InputOutputArray*", "cv::Point", "cv::Point", "const cv::Scalar*"]), _)]),
	void cv_rectangle_const__InputOutputArrayR_Point_Point_const_ScalarR(const cv::_InputOutputArray* img, cv::Point* pt1, cv::Point* pt2, const cv::Scalar* color, ResultVoid* ocvrs_return) {
		try {
			cv::rectangle(*img, *pt1, *pt2, *color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// rectangle(InputOutputArray, Point, Point, const Scalar &, int, int, int)(InputOutputArray, SimpleClass, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4576
	// ("cv::rectangle", vec![(pred!(mut, ["img", "pt1", "pt2", "color", "thickness", "lineType", "shift"], ["const cv::_InputOutputArray*", "cv::Point", "cv::Point", "const cv::Scalar*", "int", "int", "int"]), _)]),
	void cv_rectangle_const__InputOutputArrayR_Point_Point_const_ScalarR_int_int_int(const cv::_InputOutputArray* img, cv::Point* pt1, cv::Point* pt2, const cv::Scalar* color, int thickness, int lineType, int shift, ResultVoid* ocvrs_return) {
		try {
			cv::rectangle(*img, *pt1, *pt2, *color, thickness, lineType, shift);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rectangle(InputOutputArray, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4585
	// ("cv::rectangle", vec![(pred!(mut, ["img", "rec", "color"], ["const cv::_InputOutputArray*", "cv::Rect", "const cv::Scalar*"]), _)]),
	void cv_rectangle_const__InputOutputArrayR_Rect_const_ScalarR(const cv::_InputOutputArray* img, cv::Rect* rec, const cv::Scalar* color, ResultVoid* ocvrs_return) {
		try {
			cv::rectangle(*img, *rec, *color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// rectangle(InputOutputArray, Rect, const Scalar &, int, int, int)(InputOutputArray, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4585
	// ("cv::rectangle", vec![(pred!(mut, ["img", "rec", "color", "thickness", "lineType", "shift"], ["const cv::_InputOutputArray*", "cv::Rect", "const cv::Scalar*", "int", "int", "int"]), _)]),
	void cv_rectangle_const__InputOutputArrayR_Rect_const_ScalarR_int_int_int(const cv::_InputOutputArray* img, cv::Rect* rec, const cv::Scalar* color, int thickness, int lineType, int shift, ResultVoid* ocvrs_return) {
		try {
			cv::rectangle(*img, *rec, *color, thickness, lineType, shift);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::remap(InputArray, OutputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2525
	// ("cv::remap", vec![(pred!(mut, ["src", "dst", "map1", "map2", "interpolation"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
	void cv_remap_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* map1, const cv::_InputArray* map2, int interpolation, ResultVoid* ocvrs_return) {
		try {
			cv::remap(*src, *dst, *map1, *map2, interpolation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// remap(InputArray, OutputArray, InputArray, InputArray, int, int, const Scalar &)(InputArray, OutputArray, InputArray, InputArray, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2525
	// ("cv::remap", vec![(pred!(mut, ["src", "dst", "map1", "map2", "interpolation", "borderMode", "borderValue"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::Scalar*"]), _)]),
	void cv_remap_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_int_int_const_ScalarR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* map1, const cv::_InputArray* map2, int interpolation, int borderMode, const cv::Scalar* borderValue, ResultVoid* ocvrs_return) {
		try {
			cv::remap(*src, *dst, *map1, *map2, interpolation, borderMode, *borderValue);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::resize(InputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2421
	// ("cv::resize", vec![(pred!(mut, ["src", "dst", "dsize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size"]), _)]),
	void cv_resize_const__InputArrayR_const__OutputArrayR_Size(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Size* dsize, ResultVoid* ocvrs_return) {
		try {
			cv::resize(*src, *dst, *dsize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// resize(InputArray, OutputArray, Size, double, double, int)(InputArray, OutputArray, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2421
	// ("cv::resize", vec![(pred!(mut, ["src", "dst", "dsize", "fx", "fy", "interpolation"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size", "double", "double", "int"]), _)]),
	void cv_resize_const__InputArrayR_const__OutputArrayR_Size_double_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Size* dsize, double fx, double fy, int interpolation, ResultVoid* ocvrs_return) {
		try {
			cv::resize(*src, *dst, *dsize, fx, fy, interpolation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// rotatedRectangleIntersection(const RotatedRect &, const RotatedRect &, OutputArray)(SimpleClass, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4457
	// ("cv::rotatedRectangleIntersection", vec![(pred!(mut, ["rect1", "rect2", "intersectingRegion"], ["const cv::RotatedRect*", "const cv::RotatedRect*", "const cv::_OutputArray*"]), _)]),
	void cv_rotatedRectangleIntersection_const_RotatedRectR_const_RotatedRectR_const__OutputArrayR(const cv::RotatedRect* rect1, const cv::RotatedRect* rect2, const cv::_OutputArray* intersectingRegion, Result<int>* ocvrs_return) {
		try {
			int ret = cv::rotatedRectangleIntersection(*rect1, *rect2, *intersectingRegion);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::sepFilter2D(InputArray, OutputArray, Primitive, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1722
	// ("cv::sepFilter2D", vec![(pred!(mut, ["src", "dst", "ddepth", "kernelX", "kernelY"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_sepFilter2D_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, const cv::_InputArray* kernelX, const cv::_InputArray* kernelY, ResultVoid* ocvrs_return) {
		try {
			cv::sepFilter2D(*src, *dst, ddepth, *kernelX, *kernelY);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// sepFilter2D(InputArray, OutputArray, int, InputArray, InputArray, Point, double, int)(InputArray, OutputArray, Primitive, InputArray, InputArray, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1722
	// ("cv::sepFilter2D", vec![(pred!(mut, ["src", "dst", "ddepth", "kernelX", "kernelY", "anchor", "delta", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Point", "double", "int"]), _)]),
	void cv_sepFilter2D_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR_const__InputArrayR_Point_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, const cv::_InputArray* kernelX, const cv::_InputArray* kernelY, cv::Point* anchor, double delta, int borderType, ResultVoid* ocvrs_return) {
		try {
			cv::sepFilter2D(*src, *dst, ddepth, *kernelX, *kernelY, *anchor, delta, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::spatialGradient(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1799
	// ("cv::spatialGradient", vec![(pred!(mut, ["src", "dx", "dy"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_spatialGradient_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dx, const cv::_OutputArray* dy, ResultVoid* ocvrs_return) {
		try {
			cv::spatialGradient(*src, *dx, *dy);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// spatialGradient(InputArray, OutputArray, OutputArray, int, int)(InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1799
	// ("cv::spatialGradient", vec![(pred!(mut, ["src", "dx", "dy", "ksize", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_spatialGradient_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dx, const cv::_OutputArray* dy, int ksize, int borderType, ResultVoid* ocvrs_return) {
		try {
			cv::spatialGradient(*src, *dx, *dy, ksize, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::sqrBoxFilter(InputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1627
	// ("cv::sqrBoxFilter", vec![(pred!(mut, ["src", "dst", "ddepth", "ksize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "cv::Size"]), _)]),
	void cv_sqrBoxFilter_const__InputArrayR_const__OutputArrayR_int_Size(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, cv::Size* ksize, ResultVoid* ocvrs_return) {
		try {
			cv::sqrBoxFilter(*src, *dst, ddepth, *ksize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// sqrBoxFilter(InputArray, OutputArray, int, Size, Point, bool, int)(InputArray, OutputArray, Primitive, SimpleClass, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1627
	// ("cv::sqrBoxFilter", vec![(pred!(mut, ["src", "dst", "ddepth", "ksize", "anchor", "normalize", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "cv::Size", "cv::Point", "bool", "int"]), _)]),
	void cv_sqrBoxFilter_const__InputArrayR_const__OutputArrayR_int_Size_Point_bool_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, cv::Size* ksize, cv::Point* anchor, bool normalize, int borderType, ResultVoid* ocvrs_return) {
		try {
			cv::sqrBoxFilter(*src, *dst, ddepth, *ksize, *anchor, normalize, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stackBlur(InputArray, OutputArray, Size)(InputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1669
	// ("cv::stackBlur", vec![(pred!(mut, ["src", "dst", "ksize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size"]), _)]),
	void cv_stackBlur_const__InputArrayR_const__OutputArrayR_Size(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Size* ksize, ResultVoid* ocvrs_return) {
		try {
			cv::stackBlur(*src, *dst, *ksize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// threshold(InputArray, OutputArray, double, double, int)(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3084
	// ("cv::threshold", vec![(pred!(mut, ["src", "dst", "thresh", "maxval", "type"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "int"]), _)]),
	void cv_threshold_const__InputArrayR_const__OutputArrayR_double_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, double thresh, double maxval, int type, Result<double>* ocvrs_return) {
		try {
			double ret = cv::threshold(*src, *dst, thresh, maxval, type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::warpAffine(InputArray, OutputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2449
	// ("cv::warpAffine", vec![(pred!(mut, ["src", "dst", "M", "dsize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::Size"]), _)]),
	void cv_warpAffine_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Size(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* M, cv::Size* dsize, ResultVoid* ocvrs_return) {
		try {
			cv::warpAffine(*src, *dst, *M, *dsize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpAffine(InputArray, OutputArray, InputArray, Size, int, int, const Scalar &)(InputArray, OutputArray, InputArray, SimpleClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2449
	// ("cv::warpAffine", vec![(pred!(mut, ["src", "dst", "M", "dsize", "flags", "borderMode", "borderValue"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::Size", "int", "int", "const cv::Scalar*"]), _)]),
	void cv_warpAffine_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Size_int_int_const_ScalarR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* M, cv::Size* dsize, int flags, int borderMode, const cv::Scalar* borderValue, ResultVoid* ocvrs_return) {
		try {
			cv::warpAffine(*src, *dst, *M, *dsize, flags, borderMode, *borderValue);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::warpPerspective(InputArray, OutputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2481
	// ("cv::warpPerspective", vec![(pred!(mut, ["src", "dst", "M", "dsize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::Size"]), _)]),
	void cv_warpPerspective_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Size(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* M, cv::Size* dsize, ResultVoid* ocvrs_return) {
		try {
			cv::warpPerspective(*src, *dst, *M, *dsize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpPerspective(InputArray, OutputArray, InputArray, Size, int, int, const Scalar &)(InputArray, OutputArray, InputArray, SimpleClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2481
	// ("cv::warpPerspective", vec![(pred!(mut, ["src", "dst", "M", "dsize", "flags", "borderMode", "borderValue"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::Size", "int", "int", "const cv::Scalar*"]), _)]),
	void cv_warpPerspective_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Size_int_int_const_ScalarR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* M, cv::Size* dsize, int flags, int borderMode, const cv::Scalar* borderValue, ResultVoid* ocvrs_return) {
		try {
			cv::warpPerspective(*src, *dst, *M, *dsize, flags, borderMode, *borderValue);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpPolar(InputArray, OutputArray, Size, Point2f, double, int)(InputArray, OutputArray, SimpleClass, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:2846
	// ("cv::warpPolar", vec![(pred!(mut, ["src", "dst", "dsize", "center", "maxRadius", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size", "cv::Point2f", "double", "int"]), _)]),
	void cv_warpPolar_const__InputArrayR_const__OutputArrayR_Size_Point2f_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Size* dsize, cv::Point2f* center, double maxRadius, int flags, ResultVoid* ocvrs_return) {
		try {
			cv::warpPolar(*src, *dst, *dsize, *center, maxRadius, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// watershed(InputArray, InputOutputArray)(InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3430
	// ("cv::watershed", vec![(pred!(mut, ["image", "markers"], ["const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_watershed_const__InputArrayR_const__InputOutputArrayR(const cv::_InputArray* image, const cv::_InputOutputArray* markers, ResultVoid* ocvrs_return) {
		try {
			cv::watershed(*image, *markers);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::wrapperEMD(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3393
	// ("cv::wrapperEMD", vec![(pred!(mut, ["signature1", "signature2", "distType"], ["const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
	void cv_wrapperEMD_const__InputArrayR_const__InputArrayR_int(const cv::_InputArray* signature1, const cv::_InputArray* signature2, int distType, Result<float>* ocvrs_return) {
		try {
			float ret = cv::wrapperEMD(*signature1, *signature2, distType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// wrapperEMD(InputArray, InputArray, int, InputArray, Ptr<float>, OutputArray)(InputArray, InputArray, Primitive, InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:3393
	// ("cv::wrapperEMD", vec![(pred!(mut, ["signature1", "signature2", "distType", "cost", "lowerBound", "flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "const cv::_InputArray*", "cv::Ptr<float>", "const cv::_OutputArray*"]), _)]),
	void cv_wrapperEMD_const__InputArrayR_const__InputArrayR_int_const__InputArrayR_PtrLfloatG_const__OutputArrayR(const cv::_InputArray* signature1, const cv::_InputArray* signature2, int distType, const cv::_InputArray* cost, cv::Ptr<float>* lowerBound, const cv::_OutputArray* flow, Result<float>* ocvrs_return) {
		try {
			float ret = cv::wrapperEMD(*signature1, *signature2, distType, *cost, *lowerBound, *flow);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// apply(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1047
	// ("cv::CLAHE::apply", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_CLAHE_apply_const__InputArrayR_const__OutputArrayR(cv::CLAHE* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setClipLimit(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1053
	// ("cv::CLAHE::setClipLimit", vec![(pred!(mut, ["clipLimit"], ["double"]), _)]),
	void cv_CLAHE_setClipLimit_double(cv::CLAHE* instance, double clipLimit, ResultVoid* ocvrs_return) {
		try {
			instance->setClipLimit(clipLimit);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getClipLimit()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1056
	// ("cv::CLAHE::getClipLimit", vec![(pred!(const, [], []), _)]),
	void cv_CLAHE_getClipLimit_const(const cv::CLAHE* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getClipLimit();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTilesGridSize(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1063
	// ("cv::CLAHE::setTilesGridSize", vec![(pred!(mut, ["tileGridSize"], ["cv::Size"]), _)]),
	void cv_CLAHE_setTilesGridSize_Size(cv::CLAHE* instance, cv::Size* tileGridSize, ResultVoid* ocvrs_return) {
		try {
			instance->setTilesGridSize(*tileGridSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTilesGridSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1066
	// ("cv::CLAHE::getTilesGridSize", vec![(pred!(const, [], []), _)]),
	void cv_CLAHE_getTilesGridSize_const(const cv::CLAHE* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getTilesGridSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// collectGarbage()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1068
	// ("cv::CLAHE::collectGarbage", vec![(pred!(mut, [], []), _)]),
	void cv_CLAHE_collectGarbage(cv::CLAHE* instance, ResultVoid* ocvrs_return) {
		try {
			instance->collectGarbage();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CLAHE::to_Algorithm() generated
	// ("cv::CLAHE::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_CLAHE_to_Algorithm(cv::CLAHE* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::CLAHE::delete() generated
	// ("cv::CLAHE::delete", vec![(pred!(mut, [], []), _)]),
	void cv_CLAHE_delete(cv::CLAHE* instance) {
			delete instance;
	}

	// setTemplate(InputArray, Point)(InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:932
	// ("cv::GeneralizedHough::setTemplate", vec![(pred!(mut, ["templ", "templCenter"], ["const cv::_InputArray*", "cv::Point"]), _)]),
	void cv_GeneralizedHough_setTemplate_const__InputArrayR_Point(cv::GeneralizedHough* instance, const cv::_InputArray* templ, cv::Point* templCenter, ResultVoid* ocvrs_return) {
		try {
			instance->setTemplate(*templ, *templCenter);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GeneralizedHough::setTemplate(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:932
	// ("cv::GeneralizedHough::setTemplate", vec![(pred!(mut, ["templ"], ["const cv::_InputArray*"]), _)]),
	void cv_GeneralizedHough_setTemplate_const__InputArrayR(cv::GeneralizedHough* instance, const cv::_InputArray* templ, ResultVoid* ocvrs_return) {
		try {
			instance->setTemplate(*templ);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTemplate(InputArray, InputArray, InputArray, Point)(InputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:933
	// ("cv::GeneralizedHough::setTemplate", vec![(pred!(mut, ["edges", "dx", "dy", "templCenter"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Point"]), _)]),
	void cv_GeneralizedHough_setTemplate_const__InputArrayR_const__InputArrayR_const__InputArrayR_Point(cv::GeneralizedHough* instance, const cv::_InputArray* edges, const cv::_InputArray* dx, const cv::_InputArray* dy, cv::Point* templCenter, ResultVoid* ocvrs_return) {
		try {
			instance->setTemplate(*edges, *dx, *dy, *templCenter);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GeneralizedHough::setTemplate(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:933
	// ("cv::GeneralizedHough::setTemplate", vec![(pred!(mut, ["edges", "dx", "dy"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_GeneralizedHough_setTemplate_const__InputArrayR_const__InputArrayR_const__InputArrayR(cv::GeneralizedHough* instance, const cv::_InputArray* edges, const cv::_InputArray* dx, const cv::_InputArray* dy, ResultVoid* ocvrs_return) {
		try {
			instance->setTemplate(*edges, *dx, *dy);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detect(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:936
	// ("cv::GeneralizedHough::detect", vec![(pred!(mut, ["image", "positions", "votes"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_GeneralizedHough_detect_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::GeneralizedHough* instance, const cv::_InputArray* image, const cv::_OutputArray* positions, const cv::_OutputArray* votes, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*image, *positions, *votes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GeneralizedHough::detect(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:936
	// ("cv::GeneralizedHough::detect", vec![(pred!(mut, ["image", "positions"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_GeneralizedHough_detect_const__InputArrayR_const__OutputArrayR(cv::GeneralizedHough* instance, const cv::_InputArray* image, const cv::_OutputArray* positions, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*image, *positions);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detect(InputArray, InputArray, InputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:937
	// ("cv::GeneralizedHough::detect", vec![(pred!(mut, ["edges", "dx", "dy", "positions", "votes"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_GeneralizedHough_detect_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::GeneralizedHough* instance, const cv::_InputArray* edges, const cv::_InputArray* dx, const cv::_InputArray* dy, const cv::_OutputArray* positions, const cv::_OutputArray* votes, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*edges, *dx, *dy, *positions, *votes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GeneralizedHough::detect(InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:937
	// ("cv::GeneralizedHough::detect", vec![(pred!(mut, ["edges", "dx", "dy", "positions"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_GeneralizedHough_detect_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::GeneralizedHough* instance, const cv::_InputArray* edges, const cv::_InputArray* dx, const cv::_InputArray* dy, const cv::_OutputArray* positions, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*edges, *dx, *dy, *positions);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCannyLowThresh(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:940
	// ("cv::GeneralizedHough::setCannyLowThresh", vec![(pred!(mut, ["cannyLowThresh"], ["int"]), _)]),
	void cv_GeneralizedHough_setCannyLowThresh_int(cv::GeneralizedHough* instance, int cannyLowThresh, ResultVoid* ocvrs_return) {
		try {
			instance->setCannyLowThresh(cannyLowThresh);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCannyLowThresh()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:941
	// ("cv::GeneralizedHough::getCannyLowThresh", vec![(pred!(const, [], []), _)]),
	void cv_GeneralizedHough_getCannyLowThresh_const(const cv::GeneralizedHough* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getCannyLowThresh();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCannyHighThresh(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:944
	// ("cv::GeneralizedHough::setCannyHighThresh", vec![(pred!(mut, ["cannyHighThresh"], ["int"]), _)]),
	void cv_GeneralizedHough_setCannyHighThresh_int(cv::GeneralizedHough* instance, int cannyHighThresh, ResultVoid* ocvrs_return) {
		try {
			instance->setCannyHighThresh(cannyHighThresh);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCannyHighThresh()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:945
	// ("cv::GeneralizedHough::getCannyHighThresh", vec![(pred!(const, [], []), _)]),
	void cv_GeneralizedHough_getCannyHighThresh_const(const cv::GeneralizedHough* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getCannyHighThresh();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinDist(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:948
	// ("cv::GeneralizedHough::setMinDist", vec![(pred!(mut, ["minDist"], ["double"]), _)]),
	void cv_GeneralizedHough_setMinDist_double(cv::GeneralizedHough* instance, double minDist, ResultVoid* ocvrs_return) {
		try {
			instance->setMinDist(minDist);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinDist()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:949
	// ("cv::GeneralizedHough::getMinDist", vec![(pred!(const, [], []), _)]),
	void cv_GeneralizedHough_getMinDist_const(const cv::GeneralizedHough* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMinDist();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDp(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:952
	// ("cv::GeneralizedHough::setDp", vec![(pred!(mut, ["dp"], ["double"]), _)]),
	void cv_GeneralizedHough_setDp_double(cv::GeneralizedHough* instance, double dp, ResultVoid* ocvrs_return) {
		try {
			instance->setDp(dp);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDp()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:953
	// ("cv::GeneralizedHough::getDp", vec![(pred!(const, [], []), _)]),
	void cv_GeneralizedHough_getDp_const(const cv::GeneralizedHough* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getDp();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxBufferSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:956
	// ("cv::GeneralizedHough::setMaxBufferSize", vec![(pred!(mut, ["maxBufferSize"], ["int"]), _)]),
	void cv_GeneralizedHough_setMaxBufferSize_int(cv::GeneralizedHough* instance, int maxBufferSize, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxBufferSize(maxBufferSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxBufferSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:957
	// ("cv::GeneralizedHough::getMaxBufferSize", vec![(pred!(const, [], []), _)]),
	void cv_GeneralizedHough_getMaxBufferSize_const(const cv::GeneralizedHough* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxBufferSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GeneralizedHough::to_GeneralizedHoughBallard() generated
	// ("cv::GeneralizedHough::to_GeneralizedHoughBallard", vec![(pred!(mut, [], []), _)]),
	cv::GeneralizedHoughBallard* cv_GeneralizedHough_to_GeneralizedHoughBallard(cv::GeneralizedHough* instance) {
			return dynamic_cast<cv::GeneralizedHoughBallard*>(instance);
	}

	// cv::GeneralizedHough::to_GeneralizedHoughGuil() generated
	// ("cv::GeneralizedHough::to_GeneralizedHoughGuil", vec![(pred!(mut, [], []), _)]),
	cv::GeneralizedHoughGuil* cv_GeneralizedHough_to_GeneralizedHoughGuil(cv::GeneralizedHough* instance) {
			return dynamic_cast<cv::GeneralizedHoughGuil*>(instance);
	}

	// cv::GeneralizedHough::to_Algorithm() generated
	// ("cv::GeneralizedHough::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_GeneralizedHough_to_Algorithm(cv::GeneralizedHough* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::GeneralizedHough::delete() generated
	// ("cv::GeneralizedHough::delete", vec![(pred!(mut, [], []), _)]),
	void cv_GeneralizedHough_delete(cv::GeneralizedHough* instance) {
			delete instance;
	}

	// setLevels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:968
	// ("cv::GeneralizedHoughBallard::setLevels", vec![(pred!(mut, ["levels"], ["int"]), _)]),
	void cv_GeneralizedHoughBallard_setLevels_int(cv::GeneralizedHoughBallard* instance, int levels, ResultVoid* ocvrs_return) {
		try {
			instance->setLevels(levels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLevels()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:969
	// ("cv::GeneralizedHoughBallard::getLevels", vec![(pred!(const, [], []), _)]),
	void cv_GeneralizedHoughBallard_getLevels_const(const cv::GeneralizedHoughBallard* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getLevels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setVotesThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:972
	// ("cv::GeneralizedHoughBallard::setVotesThreshold", vec![(pred!(mut, ["votesThreshold"], ["int"]), _)]),
	void cv_GeneralizedHoughBallard_setVotesThreshold_int(cv::GeneralizedHoughBallard* instance, int votesThreshold, ResultVoid* ocvrs_return) {
		try {
			instance->setVotesThreshold(votesThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVotesThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:973
	// ("cv::GeneralizedHoughBallard::getVotesThreshold", vec![(pred!(const, [], []), _)]),
	void cv_GeneralizedHoughBallard_getVotesThreshold_const(const cv::GeneralizedHoughBallard* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getVotesThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GeneralizedHoughBallard::to_Algorithm() generated
	// ("cv::GeneralizedHoughBallard::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_GeneralizedHoughBallard_to_Algorithm(cv::GeneralizedHoughBallard* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::GeneralizedHoughBallard::to_GeneralizedHough() generated
	// ("cv::GeneralizedHoughBallard::to_GeneralizedHough", vec![(pred!(mut, [], []), _)]),
	cv::GeneralizedHough* cv_GeneralizedHoughBallard_to_GeneralizedHough(cv::GeneralizedHoughBallard* instance) {
			return dynamic_cast<cv::GeneralizedHough*>(instance);
	}

	// cv::GeneralizedHoughBallard::delete() generated
	// ("cv::GeneralizedHoughBallard::delete", vec![(pred!(mut, [], []), _)]),
	void cv_GeneralizedHoughBallard_delete(cv::GeneralizedHoughBallard* instance) {
			delete instance;
	}

	// setXi(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:984
	// ("cv::GeneralizedHoughGuil::setXi", vec![(pred!(mut, ["xi"], ["double"]), _)]),
	void cv_GeneralizedHoughGuil_setXi_double(cv::GeneralizedHoughGuil* instance, double xi, ResultVoid* ocvrs_return) {
		try {
			instance->setXi(xi);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getXi()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:985
	// ("cv::GeneralizedHoughGuil::getXi", vec![(pred!(const, [], []), _)]),
	void cv_GeneralizedHoughGuil_getXi_const(const cv::GeneralizedHoughGuil* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getXi();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setLevels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:988
	// ("cv::GeneralizedHoughGuil::setLevels", vec![(pred!(mut, ["levels"], ["int"]), _)]),
	void cv_GeneralizedHoughGuil_setLevels_int(cv::GeneralizedHoughGuil* instance, int levels, ResultVoid* ocvrs_return) {
		try {
			instance->setLevels(levels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLevels()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:989
	// ("cv::GeneralizedHoughGuil::getLevels", vec![(pred!(const, [], []), _)]),
	void cv_GeneralizedHoughGuil_getLevels_const(const cv::GeneralizedHoughGuil* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getLevels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setAngleEpsilon(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:992
	// ("cv::GeneralizedHoughGuil::setAngleEpsilon", vec![(pred!(mut, ["angleEpsilon"], ["double"]), _)]),
	void cv_GeneralizedHoughGuil_setAngleEpsilon_double(cv::GeneralizedHoughGuil* instance, double angleEpsilon, ResultVoid* ocvrs_return) {
		try {
			instance->setAngleEpsilon(angleEpsilon);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAngleEpsilon()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:993
	// ("cv::GeneralizedHoughGuil::getAngleEpsilon", vec![(pred!(const, [], []), _)]),
	void cv_GeneralizedHoughGuil_getAngleEpsilon_const(const cv::GeneralizedHoughGuil* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getAngleEpsilon();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinAngle(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:996
	// ("cv::GeneralizedHoughGuil::setMinAngle", vec![(pred!(mut, ["minAngle"], ["double"]), _)]),
	void cv_GeneralizedHoughGuil_setMinAngle_double(cv::GeneralizedHoughGuil* instance, double minAngle, ResultVoid* ocvrs_return) {
		try {
			instance->setMinAngle(minAngle);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinAngle()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:997
	// ("cv::GeneralizedHoughGuil::getMinAngle", vec![(pred!(const, [], []), _)]),
	void cv_GeneralizedHoughGuil_getMinAngle_const(const cv::GeneralizedHoughGuil* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMinAngle();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxAngle(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1000
	// ("cv::GeneralizedHoughGuil::setMaxAngle", vec![(pred!(mut, ["maxAngle"], ["double"]), _)]),
	void cv_GeneralizedHoughGuil_setMaxAngle_double(cv::GeneralizedHoughGuil* instance, double maxAngle, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxAngle(maxAngle);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxAngle()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1001
	// ("cv::GeneralizedHoughGuil::getMaxAngle", vec![(pred!(const, [], []), _)]),
	void cv_GeneralizedHoughGuil_getMaxAngle_const(const cv::GeneralizedHoughGuil* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxAngle();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setAngleStep(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1004
	// ("cv::GeneralizedHoughGuil::setAngleStep", vec![(pred!(mut, ["angleStep"], ["double"]), _)]),
	void cv_GeneralizedHoughGuil_setAngleStep_double(cv::GeneralizedHoughGuil* instance, double angleStep, ResultVoid* ocvrs_return) {
		try {
			instance->setAngleStep(angleStep);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAngleStep()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1005
	// ("cv::GeneralizedHoughGuil::getAngleStep", vec![(pred!(const, [], []), _)]),
	void cv_GeneralizedHoughGuil_getAngleStep_const(const cv::GeneralizedHoughGuil* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getAngleStep();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setAngleThresh(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1008
	// ("cv::GeneralizedHoughGuil::setAngleThresh", vec![(pred!(mut, ["angleThresh"], ["int"]), _)]),
	void cv_GeneralizedHoughGuil_setAngleThresh_int(cv::GeneralizedHoughGuil* instance, int angleThresh, ResultVoid* ocvrs_return) {
		try {
			instance->setAngleThresh(angleThresh);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAngleThresh()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1009
	// ("cv::GeneralizedHoughGuil::getAngleThresh", vec![(pred!(const, [], []), _)]),
	void cv_GeneralizedHoughGuil_getAngleThresh_const(const cv::GeneralizedHoughGuil* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getAngleThresh();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinScale(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1012
	// ("cv::GeneralizedHoughGuil::setMinScale", vec![(pred!(mut, ["minScale"], ["double"]), _)]),
	void cv_GeneralizedHoughGuil_setMinScale_double(cv::GeneralizedHoughGuil* instance, double minScale, ResultVoid* ocvrs_return) {
		try {
			instance->setMinScale(minScale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinScale()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1013
	// ("cv::GeneralizedHoughGuil::getMinScale", vec![(pred!(const, [], []), _)]),
	void cv_GeneralizedHoughGuil_getMinScale_const(const cv::GeneralizedHoughGuil* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMinScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxScale(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1016
	// ("cv::GeneralizedHoughGuil::setMaxScale", vec![(pred!(mut, ["maxScale"], ["double"]), _)]),
	void cv_GeneralizedHoughGuil_setMaxScale_double(cv::GeneralizedHoughGuil* instance, double maxScale, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxScale(maxScale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxScale()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1017
	// ("cv::GeneralizedHoughGuil::getMaxScale", vec![(pred!(const, [], []), _)]),
	void cv_GeneralizedHoughGuil_getMaxScale_const(const cv::GeneralizedHoughGuil* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScaleStep(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1020
	// ("cv::GeneralizedHoughGuil::setScaleStep", vec![(pred!(mut, ["scaleStep"], ["double"]), _)]),
	void cv_GeneralizedHoughGuil_setScaleStep_double(cv::GeneralizedHoughGuil* instance, double scaleStep, ResultVoid* ocvrs_return) {
		try {
			instance->setScaleStep(scaleStep);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScaleStep()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1021
	// ("cv::GeneralizedHoughGuil::getScaleStep", vec![(pred!(const, [], []), _)]),
	void cv_GeneralizedHoughGuil_getScaleStep_const(const cv::GeneralizedHoughGuil* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getScaleStep();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScaleThresh(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1024
	// ("cv::GeneralizedHoughGuil::setScaleThresh", vec![(pred!(mut, ["scaleThresh"], ["int"]), _)]),
	void cv_GeneralizedHoughGuil_setScaleThresh_int(cv::GeneralizedHoughGuil* instance, int scaleThresh, ResultVoid* ocvrs_return) {
		try {
			instance->setScaleThresh(scaleThresh);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScaleThresh()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1025
	// ("cv::GeneralizedHoughGuil::getScaleThresh", vec![(pred!(const, [], []), _)]),
	void cv_GeneralizedHoughGuil_getScaleThresh_const(const cv::GeneralizedHoughGuil* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getScaleThresh();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPosThresh(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1028
	// ("cv::GeneralizedHoughGuil::setPosThresh", vec![(pred!(mut, ["posThresh"], ["int"]), _)]),
	void cv_GeneralizedHoughGuil_setPosThresh_int(cv::GeneralizedHoughGuil* instance, int posThresh, ResultVoid* ocvrs_return) {
		try {
			instance->setPosThresh(posThresh);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPosThresh()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1029
	// ("cv::GeneralizedHoughGuil::getPosThresh", vec![(pred!(const, [], []), _)]),
	void cv_GeneralizedHoughGuil_getPosThresh_const(const cv::GeneralizedHoughGuil* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPosThresh();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::GeneralizedHoughGuil::to_Algorithm() generated
	// ("cv::GeneralizedHoughGuil::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_GeneralizedHoughGuil_to_Algorithm(cv::GeneralizedHoughGuil* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::GeneralizedHoughGuil::to_GeneralizedHough() generated
	// ("cv::GeneralizedHoughGuil::to_GeneralizedHough", vec![(pred!(mut, [], []), _)]),
	cv::GeneralizedHough* cv_GeneralizedHoughGuil_to_GeneralizedHough(cv::GeneralizedHoughGuil* instance) {
			return dynamic_cast<cv::GeneralizedHough*>(instance);
	}

	// cv::GeneralizedHoughGuil::delete() generated
	// ("cv::GeneralizedHoughGuil::delete", vec![(pred!(mut, [], []), _)]),
	void cv_GeneralizedHoughGuil_delete(cv::GeneralizedHoughGuil* instance) {
			delete instance;
	}

	// LineIterator(const Mat &, Point, Point, int, bool)(TraitClass, SimpleClass, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4979
	// ("cv::LineIterator::LineIterator", vec![(pred!(mut, ["img", "pt1", "pt2", "connectivity", "leftToRight"], ["const cv::Mat*", "cv::Point", "cv::Point", "int", "bool"]), _)]),
	void cv_LineIterator_LineIterator_const_MatR_Point_Point_int_bool(const cv::Mat* img, cv::Point* pt1, cv::Point* pt2, int connectivity, bool leftToRight, Result<cv::LineIterator*>* ocvrs_return) {
		try {
			cv::LineIterator* ret = new cv::LineIterator(*img, *pt1, *pt2, connectivity, leftToRight);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::LineIterator::LineIterator(TraitClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4979
	// ("cv::LineIterator::LineIterator", vec![(pred!(mut, ["img", "pt1", "pt2"], ["const cv::Mat*", "cv::Point", "cv::Point"]), _)]),
	void cv_LineIterator_LineIterator_const_MatR_Point_Point(const cv::Mat* img, cv::Point* pt1, cv::Point* pt2, Result<cv::LineIterator*>* ocvrs_return) {
		try {
			cv::LineIterator* ret = new cv::LineIterator(*img, *pt1, *pt2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// LineIterator(Point, Point, int, bool)(SimpleClass, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4985
	// ("cv::LineIterator::LineIterator", vec![(pred!(mut, ["pt1", "pt2", "connectivity", "leftToRight"], ["cv::Point", "cv::Point", "int", "bool"]), _)]),
	void cv_LineIterator_LineIterator_Point_Point_int_bool(cv::Point* pt1, cv::Point* pt2, int connectivity, bool leftToRight, Result<cv::LineIterator*>* ocvrs_return) {
		try {
			cv::LineIterator* ret = new cv::LineIterator(*pt1, *pt2, connectivity, leftToRight);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::LineIterator::LineIterator(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4985
	// ("cv::LineIterator::LineIterator", vec![(pred!(mut, ["pt1", "pt2"], ["cv::Point", "cv::Point"]), _)]),
	void cv_LineIterator_LineIterator_Point_Point(cv::Point* pt1, cv::Point* pt2, Result<cv::LineIterator*>* ocvrs_return) {
		try {
			cv::LineIterator* ret = new cv::LineIterator(*pt1, *pt2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// LineIterator(Size, Point, Point, int, bool)(SimpleClass, SimpleClass, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4995
	// ("cv::LineIterator::LineIterator", vec![(pred!(mut, ["boundingAreaSize", "pt1", "pt2", "connectivity", "leftToRight"], ["cv::Size", "cv::Point", "cv::Point", "int", "bool"]), _)]),
	void cv_LineIterator_LineIterator_Size_Point_Point_int_bool(cv::Size* boundingAreaSize, cv::Point* pt1, cv::Point* pt2, int connectivity, bool leftToRight, Result<cv::LineIterator*>* ocvrs_return) {
		try {
			cv::LineIterator* ret = new cv::LineIterator(*boundingAreaSize, *pt1, *pt2, connectivity, leftToRight);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::LineIterator::LineIterator(SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:4995
	// ("cv::LineIterator::LineIterator", vec![(pred!(mut, ["boundingAreaSize", "pt1", "pt2"], ["cv::Size", "cv::Point", "cv::Point"]), _)]),
	void cv_LineIterator_LineIterator_Size_Point_Point(cv::Size* boundingAreaSize, cv::Point* pt1, cv::Point* pt2, Result<cv::LineIterator*>* ocvrs_return) {
		try {
			cv::LineIterator* ret = new cv::LineIterator(*boundingAreaSize, *pt1, *pt2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// LineIterator(Rect, Point, Point, int, bool)(SimpleClass, SimpleClass, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5002
	// ("cv::LineIterator::LineIterator", vec![(pred!(mut, ["boundingAreaRect", "pt1", "pt2", "connectivity", "leftToRight"], ["cv::Rect", "cv::Point", "cv::Point", "int", "bool"]), _)]),
	void cv_LineIterator_LineIterator_Rect_Point_Point_int_bool(cv::Rect* boundingAreaRect, cv::Point* pt1, cv::Point* pt2, int connectivity, bool leftToRight, Result<cv::LineIterator*>* ocvrs_return) {
		try {
			cv::LineIterator* ret = new cv::LineIterator(*boundingAreaRect, *pt1, *pt2, connectivity, leftToRight);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::LineIterator::LineIterator(SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5002
	// ("cv::LineIterator::LineIterator", vec![(pred!(mut, ["boundingAreaRect", "pt1", "pt2"], ["cv::Rect", "cv::Point", "cv::Point"]), _)]),
	void cv_LineIterator_LineIterator_Rect_Point_Point(cv::Rect* boundingAreaRect, cv::Point* pt1, cv::Point* pt2, Result<cv::LineIterator*>* ocvrs_return) {
		try {
			cv::LineIterator* ret = new cv::LineIterator(*boundingAreaRect, *pt1, *pt2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// init(const Mat *, Rect, Point, Point, int, bool)(TraitClass, SimpleClass, SimpleClass, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5008
	// ("cv::LineIterator::init", vec![(pred!(mut, ["img", "boundingAreaRect", "pt1", "pt2", "connectivity", "leftToRight"], ["const cv::Mat*", "cv::Rect", "cv::Point", "cv::Point", "int", "bool"]), _)]),
	void cv_LineIterator_init_const_MatX_Rect_Point_Point_int_bool(cv::LineIterator* instance, const cv::Mat* img, cv::Rect* boundingAreaRect, cv::Point* pt1, cv::Point* pt2, int connectivity, bool leftToRight, ResultVoid* ocvrs_return) {
		try {
			instance->init(img, *boundingAreaRect, *pt1, *pt2, connectivity, leftToRight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator*()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5012
	// ("cv::LineIterator::operator*", vec![(pred!(mut, [], []), _)]),
	void cv_LineIterator_operatorX(cv::LineIterator* instance, Result<unsigned char*>* ocvrs_return) {
		try {
			unsigned char* ret = instance->operator*();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator++()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5018
	// ("cv::LineIterator::operator++", vec![(pred!(mut, [], []), _)]),
	void cv_LineIterator_operatorAA(cv::LineIterator* instance, Result<cv::LineIterator*>* ocvrs_return) {
		try {
			cv::LineIterator ret = instance->operator++();
			Ok(new cv::LineIterator(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// pos()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5028
	// ("cv::LineIterator::pos", vec![(pred!(const, [], []), _)]),
	void cv_LineIterator_pos_const(const cv::LineIterator* instance, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->pos();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::LineIterator::ptr() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5030
	// ("cv::LineIterator::ptr", vec![(pred!(const, [], []), _)]),
	unsigned char* cv_LineIterator_propPtr_const(const cv::LineIterator* instance) {
			unsigned char* const ret = instance->ptr;
			return ret;
	}

	// cv::LineIterator::ptrMut() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5030
	// ("cv::LineIterator::ptrMut", vec![(pred!(mut, [], []), _)]),
	unsigned char* cv_LineIterator_propPtr(cv::LineIterator* instance) {
			unsigned char* ret = instance->ptr;
			return ret;
	}

	// cv::LineIterator::setPtr(Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5030
	// ("cv::LineIterator::setPtr", vec![(pred!(mut, ["val"], ["unsigned char*"]), _)]),
	void cv_LineIterator_propPtr_unsigned_charX(cv::LineIterator* instance, unsigned char* const val) {
			instance->ptr = val;
	}

	// cv::LineIterator::ptr0() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5031
	// ("cv::LineIterator::ptr0", vec![(pred!(const, [], []), _)]),
	const unsigned char* cv_LineIterator_propPtr0_const(const cv::LineIterator* instance) {
			const unsigned char* ret = instance->ptr0;
			return ret;
	}

	// cv::LineIterator::step() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5032
	// ("cv::LineIterator::step", vec![(pred!(const, [], []), _)]),
	int cv_LineIterator_propStep_const(const cv::LineIterator* instance) {
			int ret = instance->step;
			return ret;
	}

	// cv::LineIterator::setStep(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5032
	// ("cv::LineIterator::setStep", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_LineIterator_propStep_const_int(cv::LineIterator* instance, const int val) {
			instance->step = val;
	}

	// cv::LineIterator::elemSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5032
	// ("cv::LineIterator::elemSize", vec![(pred!(const, [], []), _)]),
	int cv_LineIterator_propElemSize_const(const cv::LineIterator* instance) {
			int ret = instance->elemSize;
			return ret;
	}

	// cv::LineIterator::setElemSize(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5032
	// ("cv::LineIterator::setElemSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_LineIterator_propElemSize_const_int(cv::LineIterator* instance, const int val) {
			instance->elemSize = val;
	}

	// cv::LineIterator::err() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5033
	// ("cv::LineIterator::err", vec![(pred!(const, [], []), _)]),
	int cv_LineIterator_propErr_const(const cv::LineIterator* instance) {
			int ret = instance->err;
			return ret;
	}

	// cv::LineIterator::setErr(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5033
	// ("cv::LineIterator::setErr", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_LineIterator_propErr_const_int(cv::LineIterator* instance, const int val) {
			instance->err = val;
	}

	// cv::LineIterator::count() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5033
	// ("cv::LineIterator::count", vec![(pred!(const, [], []), _)]),
	int cv_LineIterator_propCount_const(const cv::LineIterator* instance) {
			int ret = instance->count;
			return ret;
	}

	// cv::LineIterator::setCount(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5033
	// ("cv::LineIterator::setCount", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_LineIterator_propCount_const_int(cv::LineIterator* instance, const int val) {
			instance->count = val;
	}

	// cv::LineIterator::minusDelta() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5034
	// ("cv::LineIterator::minusDelta", vec![(pred!(const, [], []), _)]),
	int cv_LineIterator_propMinusDelta_const(const cv::LineIterator* instance) {
			int ret = instance->minusDelta;
			return ret;
	}

	// cv::LineIterator::setMinusDelta(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5034
	// ("cv::LineIterator::setMinusDelta", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_LineIterator_propMinusDelta_const_int(cv::LineIterator* instance, const int val) {
			instance->minusDelta = val;
	}

	// cv::LineIterator::plusDelta() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5034
	// ("cv::LineIterator::plusDelta", vec![(pred!(const, [], []), _)]),
	int cv_LineIterator_propPlusDelta_const(const cv::LineIterator* instance) {
			int ret = instance->plusDelta;
			return ret;
	}

	// cv::LineIterator::setPlusDelta(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5034
	// ("cv::LineIterator::setPlusDelta", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_LineIterator_propPlusDelta_const_int(cv::LineIterator* instance, const int val) {
			instance->plusDelta = val;
	}

	// cv::LineIterator::minusStep() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5035
	// ("cv::LineIterator::minusStep", vec![(pred!(const, [], []), _)]),
	int cv_LineIterator_propMinusStep_const(const cv::LineIterator* instance) {
			int ret = instance->minusStep;
			return ret;
	}

	// cv::LineIterator::setMinusStep(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5035
	// ("cv::LineIterator::setMinusStep", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_LineIterator_propMinusStep_const_int(cv::LineIterator* instance, const int val) {
			instance->minusStep = val;
	}

	// cv::LineIterator::plusStep() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5035
	// ("cv::LineIterator::plusStep", vec![(pred!(const, [], []), _)]),
	int cv_LineIterator_propPlusStep_const(const cv::LineIterator* instance) {
			int ret = instance->plusStep;
			return ret;
	}

	// cv::LineIterator::setPlusStep(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5035
	// ("cv::LineIterator::setPlusStep", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_LineIterator_propPlusStep_const_int(cv::LineIterator* instance, const int val) {
			instance->plusStep = val;
	}

	// cv::LineIterator::minusShift() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5036
	// ("cv::LineIterator::minusShift", vec![(pred!(const, [], []), _)]),
	int cv_LineIterator_propMinusShift_const(const cv::LineIterator* instance) {
			int ret = instance->minusShift;
			return ret;
	}

	// cv::LineIterator::setMinusShift(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5036
	// ("cv::LineIterator::setMinusShift", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_LineIterator_propMinusShift_const_int(cv::LineIterator* instance, const int val) {
			instance->minusShift = val;
	}

	// cv::LineIterator::plusShift() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5036
	// ("cv::LineIterator::plusShift", vec![(pred!(const, [], []), _)]),
	int cv_LineIterator_propPlusShift_const(const cv::LineIterator* instance) {
			int ret = instance->plusShift;
			return ret;
	}

	// cv::LineIterator::setPlusShift(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5036
	// ("cv::LineIterator::setPlusShift", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_LineIterator_propPlusShift_const_int(cv::LineIterator* instance, const int val) {
			instance->plusShift = val;
	}

	// cv::LineIterator::p() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5037
	// ("cv::LineIterator::p", vec![(pred!(const, [], []), _)]),
	void cv_LineIterator_propP_const(const cv::LineIterator* instance, cv::Point* ocvrs_return) {
			cv::Point ret = instance->p;
			*ocvrs_return = ret;
	}

	// cv::LineIterator::setP(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5037
	// ("cv::LineIterator::setP", vec![(pred!(mut, ["val"], ["const cv::Point"]), _)]),
	void cv_LineIterator_propP_const_Point(cv::LineIterator* instance, const cv::Point* val) {
			instance->p = *val;
	}

	// cv::LineIterator::ptmode() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5038
	// ("cv::LineIterator::ptmode", vec![(pred!(const, [], []), _)]),
	bool cv_LineIterator_propPtmode_const(const cv::LineIterator* instance) {
			bool ret = instance->ptmode;
			return ret;
	}

	// cv::LineIterator::setPtmode(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:5038
	// ("cv::LineIterator::setPtmode", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_LineIterator_propPtmode_const_bool(cv::LineIterator* instance, const bool val) {
			instance->ptmode = val;
	}

	// cv::LineIterator::delete() generated
	// ("cv::LineIterator::delete", vec![(pred!(mut, [], []), _)]),
	void cv_LineIterator_delete(cv::LineIterator* instance) {
			delete instance;
	}

	// detect(InputArray, OutputArray, OutputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1376
	// ("cv::LineSegmentDetector::detect", vec![(pred!(mut, ["image", "lines", "width", "prec", "nfa"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_LineSegmentDetector_detect_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(cv::LineSegmentDetector* instance, const cv::_InputArray* image, const cv::_OutputArray* lines, const cv::_OutputArray* width, const cv::_OutputArray* prec, const cv::_OutputArray* nfa, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*image, *lines, *width, *prec, *nfa);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::LineSegmentDetector::detect(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1376
	// ("cv::LineSegmentDetector::detect", vec![(pred!(mut, ["image", "lines"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_LineSegmentDetector_detect_const__InputArrayR_const__OutputArrayR(cv::LineSegmentDetector* instance, const cv::_InputArray* image, const cv::_OutputArray* lines, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*image, *lines);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawSegments(InputOutputArray, InputArray)(InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1385
	// ("cv::LineSegmentDetector::drawSegments", vec![(pred!(mut, ["image", "lines"], ["const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_LineSegmentDetector_drawSegments_const__InputOutputArrayR_const__InputArrayR(cv::LineSegmentDetector* instance, const cv::_InputOutputArray* image, const cv::_InputArray* lines, ResultVoid* ocvrs_return) {
		try {
			instance->drawSegments(*image, *lines);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compareSegments(const Size &, InputArray, InputArray, InputOutputArray)(SimpleClass, InputArray, InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1395
	// ("cv::LineSegmentDetector::compareSegments", vec![(pred!(mut, ["size", "lines1", "lines2", "image"], ["const cv::Size*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_LineSegmentDetector_compareSegments_const_SizeR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(cv::LineSegmentDetector* instance, const cv::Size* size, const cv::_InputArray* lines1, const cv::_InputArray* lines2, const cv::_InputOutputArray* image, Result<int>* ocvrs_return) {
		try {
			int ret = instance->compareSegments(*size, *lines1, *lines2, *image);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::LineSegmentDetector::compareSegments(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1395
	// ("cv::LineSegmentDetector::compareSegments", vec![(pred!(mut, ["size", "lines1", "lines2"], ["const cv::Size*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_LineSegmentDetector_compareSegments_const_SizeR_const__InputArrayR_const__InputArrayR(cv::LineSegmentDetector* instance, const cv::Size* size, const cv::_InputArray* lines1, const cv::_InputArray* lines2, Result<int>* ocvrs_return) {
		try {
			int ret = instance->compareSegments(*size, *lines1, *lines2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::LineSegmentDetector::to_Algorithm() generated
	// ("cv::LineSegmentDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_LineSegmentDetector_to_Algorithm(cv::LineSegmentDetector* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::LineSegmentDetector::delete() generated
	// ("cv::LineSegmentDetector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_LineSegmentDetector_delete(cv::LineSegmentDetector* instance) {
			delete instance;
	}

	// Subdiv2D()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1101
	// ("cv::Subdiv2D::Subdiv2D", vec![(pred!(mut, [], []), _)]),
	void cv_Subdiv2D_Subdiv2D(Result<cv::Subdiv2D*>* ocvrs_return) {
		try {
			cv::Subdiv2D* ret = new cv::Subdiv2D();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Subdiv2D(Rect)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1111
	// ("cv::Subdiv2D::Subdiv2D", vec![(pred!(mut, ["rect"], ["cv::Rect"]), _)]),
	void cv_Subdiv2D_Subdiv2D_Rect(cv::Rect* rect, Result<cv::Subdiv2D*>* ocvrs_return) {
		try {
			cv::Subdiv2D* ret = new cv::Subdiv2D(*rect);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initDelaunay(Rect)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1118
	// ("cv::Subdiv2D::initDelaunay", vec![(pred!(mut, ["rect"], ["cv::Rect"]), _)]),
	void cv_Subdiv2D_initDelaunay_Rect(cv::Subdiv2D* instance, cv::Rect* rect, ResultVoid* ocvrs_return) {
		try {
			instance->initDelaunay(*rect);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// insert(Point2f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1130
	// ("cv::Subdiv2D::insert", vec![(pred!(mut, ["pt"], ["cv::Point2f"]), _)]),
	void cv_Subdiv2D_insert_Point2f(cv::Subdiv2D* instance, cv::Point2f* pt, Result<int>* ocvrs_return) {
		try {
			int ret = instance->insert(*pt);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// insert(const std::vector<Point2f> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1139
	// ("cv::Subdiv2D::insert", vec![(pred!(mut, ["ptvec"], ["const std::vector<cv::Point2f>*"]), _)]),
	void cv_Subdiv2D_insert_const_vectorLPoint2fGR(cv::Subdiv2D* instance, const std::vector<cv::Point2f>* ptvec, ResultVoid* ocvrs_return) {
		try {
			instance->insert(*ptvec);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// locate(Point2f, int &, int &)(SimpleClass, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1161
	// ("cv::Subdiv2D::locate", vec![(pred!(mut, ["pt", "edge", "vertex"], ["cv::Point2f", "int*", "int*"]), _)]),
	void cv_Subdiv2D_locate_Point2f_intR_intR(cv::Subdiv2D* instance, cv::Point2f* pt, int* edge, int* vertex, Result<int>* ocvrs_return) {
		try {
			int ret = instance->locate(*pt, *edge, *vertex);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findNearest(Point2f, Point2f *)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1175
	// ("cv::Subdiv2D::findNearest", vec![(pred!(mut, ["pt", "nearestPt"], ["cv::Point2f", "cv::Point2f*"]), _)]),
	void cv_Subdiv2D_findNearest_Point2f_Point2fX(cv::Subdiv2D* instance, cv::Point2f* pt, cv::Point2f* nearestPt, Result<int>* ocvrs_return) {
		try {
			int ret = instance->findNearest(*pt, nearestPt);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Subdiv2D::findNearest(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1175
	// ("cv::Subdiv2D::findNearest", vec![(pred!(mut, ["pt"], ["cv::Point2f"]), _)]),
	void cv_Subdiv2D_findNearest_Point2f(cv::Subdiv2D* instance, cv::Point2f* pt, Result<int>* ocvrs_return) {
		try {
			int ret = instance->findNearest(*pt);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEdgeList(std::vector<Vec4f> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1184
	// ("cv::Subdiv2D::getEdgeList", vec![(pred!(const, ["edgeList"], ["std::vector<cv::Vec4f>*"]), _)]),
	void cv_Subdiv2D_getEdgeList_const_vectorLVec4fGR(const cv::Subdiv2D* instance, std::vector<cv::Vec4f>* edgeList, ResultVoid* ocvrs_return) {
		try {
			instance->getEdgeList(*edgeList);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLeadingEdgeList(std::vector<int> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1192
	// ("cv::Subdiv2D::getLeadingEdgeList", vec![(pred!(const, ["leadingEdgeList"], ["std::vector<int>*"]), _)]),
	void cv_Subdiv2D_getLeadingEdgeList_const_vectorLintGR(const cv::Subdiv2D* instance, std::vector<int>* leadingEdgeList, ResultVoid* ocvrs_return) {
		try {
			instance->getLeadingEdgeList(*leadingEdgeList);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTriangleList(std::vector<Vec6f> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1201
	// ("cv::Subdiv2D::getTriangleList", vec![(pred!(const, ["triangleList"], ["std::vector<cv::Vec6f>*"]), _)]),
	void cv_Subdiv2D_getTriangleList_const_vectorLVec6fGR(const cv::Subdiv2D* instance, std::vector<cv::Vec6f>* triangleList, ResultVoid* ocvrs_return) {
		try {
			instance->getTriangleList(*triangleList);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVoronoiFacetList(const std::vector<int> &, std::vector<std::vector<Point2f>> &, std::vector<Point2f> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1210
	// ("cv::Subdiv2D::getVoronoiFacetList", vec![(pred!(mut, ["idx", "facetList", "facetCenters"], ["const std::vector<int>*", "std::vector<std::vector<cv::Point2f>>*", "std::vector<cv::Point2f>*"]), _)]),
	void cv_Subdiv2D_getVoronoiFacetList_const_vectorLintGR_vectorLvectorLPoint2fGGR_vectorLPoint2fGR(cv::Subdiv2D* instance, const std::vector<int>* idx, std::vector<std::vector<cv::Point2f>>* facetList, std::vector<cv::Point2f>* facetCenters, ResultVoid* ocvrs_return) {
		try {
			instance->getVoronoiFacetList(*idx, *facetList, *facetCenters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVertex(int, int *)(Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1220
	// ("cv::Subdiv2D::getVertex", vec![(pred!(const, ["vertex", "firstEdge"], ["int", "int*"]), _)]),
	void cv_Subdiv2D_getVertex_const_int_intX(const cv::Subdiv2D* instance, int vertex, int* firstEdge, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->getVertex(vertex, firstEdge);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Subdiv2D::getVertex(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1220
	// ("cv::Subdiv2D::getVertex", vec![(pred!(const, ["vertex"], ["int"]), _)]),
	void cv_Subdiv2D_getVertex_const_int(const cv::Subdiv2D* instance, int vertex, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->getVertex(vertex);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEdge(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1240
	// ("cv::Subdiv2D::getEdge", vec![(pred!(const, ["edge", "nextEdgeType"], ["int", "int"]), _)]),
	void cv_Subdiv2D_getEdge_const_int_int(const cv::Subdiv2D* instance, int edge, int nextEdgeType, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getEdge(edge, nextEdgeType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// nextEdge(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1249
	// ("cv::Subdiv2D::nextEdge", vec![(pred!(const, ["edge"], ["int"]), _)]),
	void cv_Subdiv2D_nextEdge_const_int(const cv::Subdiv2D* instance, int edge, Result<int>* ocvrs_return) {
		try {
			int ret = instance->nextEdge(edge);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// rotateEdge(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1263
	// ("cv::Subdiv2D::rotateEdge", vec![(pred!(const, ["edge", "rotate"], ["int", "int"]), _)]),
	void cv_Subdiv2D_rotateEdge_const_int_int(const cv::Subdiv2D* instance, int edge, int rotate, Result<int>* ocvrs_return) {
		try {
			int ret = instance->rotateEdge(edge, rotate);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// symEdge(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1264
	// ("cv::Subdiv2D::symEdge", vec![(pred!(const, ["edge"], ["int"]), _)]),
	void cv_Subdiv2D_symEdge_const_int(const cv::Subdiv2D* instance, int edge, Result<int>* ocvrs_return) {
		try {
			int ret = instance->symEdge(edge);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// edgeOrg(int, Point2f *)(Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1273
	// ("cv::Subdiv2D::edgeOrg", vec![(pred!(const, ["edge", "orgpt"], ["int", "cv::Point2f*"]), _)]),
	void cv_Subdiv2D_edgeOrg_const_int_Point2fX(const cv::Subdiv2D* instance, int edge, cv::Point2f* orgpt, Result<int>* ocvrs_return) {
		try {
			int ret = instance->edgeOrg(edge, orgpt);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Subdiv2D::edgeOrg(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1273
	// ("cv::Subdiv2D::edgeOrg", vec![(pred!(const, ["edge"], ["int"]), _)]),
	void cv_Subdiv2D_edgeOrg_const_int(const cv::Subdiv2D* instance, int edge, Result<int>* ocvrs_return) {
		try {
			int ret = instance->edgeOrg(edge);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// edgeDst(int, Point2f *)(Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1282
	// ("cv::Subdiv2D::edgeDst", vec![(pred!(const, ["edge", "dstpt"], ["int", "cv::Point2f*"]), _)]),
	void cv_Subdiv2D_edgeDst_const_int_Point2fX(const cv::Subdiv2D* instance, int edge, cv::Point2f* dstpt, Result<int>* ocvrs_return) {
		try {
			int ret = instance->edgeDst(edge, dstpt);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Subdiv2D::edgeDst(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc.hpp:1282
	// ("cv::Subdiv2D::edgeDst", vec![(pred!(const, ["edge"], ["int"]), _)]),
	void cv_Subdiv2D_edgeDst_const_int(const cv::Subdiv2D* instance, int edge, Result<int>* ocvrs_return) {
		try {
			int ret = instance->edgeDst(edge);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Subdiv2D::delete() generated
	// ("cv::Subdiv2D::delete", vec![(pred!(mut, [], []), _)]),
	void cv_Subdiv2D_delete(cv::Subdiv2D* instance) {
			delete instance;
	}

	// IntelligentScissorsMB()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc/segmentation.hpp:34
	// ("cv::segmentation::IntelligentScissorsMB::IntelligentScissorsMB", vec![(pred!(mut, [], []), _)]),
	void cv_segmentation_IntelligentScissorsMB_IntelligentScissorsMB(Result<cv::segmentation::IntelligentScissorsMB*>* ocvrs_return) {
		try {
			cv::segmentation::IntelligentScissorsMB* ret = new cv::segmentation::IntelligentScissorsMB();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWeights(float, float, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc/segmentation.hpp:46
	// ("cv::segmentation::IntelligentScissorsMB::setWeights", vec![(pred!(mut, ["weight_non_edge", "weight_gradient_direction", "weight_gradient_magnitude"], ["float", "float", "float"]), _)]),
	void cv_segmentation_IntelligentScissorsMB_setWeights_float_float_float(cv::segmentation::IntelligentScissorsMB* instance, float weight_non_edge, float weight_gradient_direction, float weight_gradient_magnitude, Result<cv::segmentation::IntelligentScissorsMB*>* ocvrs_return) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->setWeights(weight_non_edge, weight_gradient_direction, weight_gradient_magnitude);
			Ok(new cv::segmentation::IntelligentScissorsMB(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setGradientMagnitudeMaxLimit(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc/segmentation.hpp:58
	// ("cv::segmentation::IntelligentScissorsMB::setGradientMagnitudeMaxLimit", vec![(pred!(mut, ["gradient_magnitude_threshold_max"], ["float"]), _)]),
	void cv_segmentation_IntelligentScissorsMB_setGradientMagnitudeMaxLimit_float(cv::segmentation::IntelligentScissorsMB* instance, float gradient_magnitude_threshold_max, Result<cv::segmentation::IntelligentScissorsMB*>* ocvrs_return) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->setGradientMagnitudeMaxLimit(gradient_magnitude_threshold_max);
			Ok(new cv::segmentation::IntelligentScissorsMB(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::segmentation::IntelligentScissorsMB::setGradientMagnitudeMaxLimit() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc/segmentation.hpp:58
	// ("cv::segmentation::IntelligentScissorsMB::setGradientMagnitudeMaxLimit", vec![(pred!(mut, [], []), _)]),
	void cv_segmentation_IntelligentScissorsMB_setGradientMagnitudeMaxLimit(cv::segmentation::IntelligentScissorsMB* instance, Result<cv::segmentation::IntelligentScissorsMB*>* ocvrs_return) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->setGradientMagnitudeMaxLimit();
			Ok(new cv::segmentation::IntelligentScissorsMB(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEdgeFeatureZeroCrossingParameters(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc/segmentation.hpp:74
	// ("cv::segmentation::IntelligentScissorsMB::setEdgeFeatureZeroCrossingParameters", vec![(pred!(mut, ["gradient_magnitude_min_value"], ["float"]), _)]),
	void cv_segmentation_IntelligentScissorsMB_setEdgeFeatureZeroCrossingParameters_float(cv::segmentation::IntelligentScissorsMB* instance, float gradient_magnitude_min_value, Result<cv::segmentation::IntelligentScissorsMB*>* ocvrs_return) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->setEdgeFeatureZeroCrossingParameters(gradient_magnitude_min_value);
			Ok(new cv::segmentation::IntelligentScissorsMB(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::segmentation::IntelligentScissorsMB::setEdgeFeatureZeroCrossingParameters() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc/segmentation.hpp:74
	// ("cv::segmentation::IntelligentScissorsMB::setEdgeFeatureZeroCrossingParameters", vec![(pred!(mut, [], []), _)]),
	void cv_segmentation_IntelligentScissorsMB_setEdgeFeatureZeroCrossingParameters(cv::segmentation::IntelligentScissorsMB* instance, Result<cv::segmentation::IntelligentScissorsMB*>* ocvrs_return) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->setEdgeFeatureZeroCrossingParameters();
			Ok(new cv::segmentation::IntelligentScissorsMB(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEdgeFeatureCannyParameters(double, double, int, bool)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc/segmentation.hpp:83
	// ("cv::segmentation::IntelligentScissorsMB::setEdgeFeatureCannyParameters", vec![(pred!(mut, ["threshold1", "threshold2", "apertureSize", "L2gradient"], ["double", "double", "int", "bool"]), _)]),
	void cv_segmentation_IntelligentScissorsMB_setEdgeFeatureCannyParameters_double_double_int_bool(cv::segmentation::IntelligentScissorsMB* instance, double threshold1, double threshold2, int apertureSize, bool L2gradient, Result<cv::segmentation::IntelligentScissorsMB*>* ocvrs_return) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->setEdgeFeatureCannyParameters(threshold1, threshold2, apertureSize, L2gradient);
			Ok(new cv::segmentation::IntelligentScissorsMB(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::segmentation::IntelligentScissorsMB::setEdgeFeatureCannyParameters(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc/segmentation.hpp:83
	// ("cv::segmentation::IntelligentScissorsMB::setEdgeFeatureCannyParameters", vec![(pred!(mut, ["threshold1", "threshold2"], ["double", "double"]), _)]),
	void cv_segmentation_IntelligentScissorsMB_setEdgeFeatureCannyParameters_double_double(cv::segmentation::IntelligentScissorsMB* instance, double threshold1, double threshold2, Result<cv::segmentation::IntelligentScissorsMB*>* ocvrs_return) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->setEdgeFeatureCannyParameters(threshold1, threshold2);
			Ok(new cv::segmentation::IntelligentScissorsMB(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// applyImage(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc/segmentation.hpp:93
	// ("cv::segmentation::IntelligentScissorsMB::applyImage", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
	void cv_segmentation_IntelligentScissorsMB_applyImage_const__InputArrayR(cv::segmentation::IntelligentScissorsMB* instance, const cv::_InputArray* image, Result<cv::segmentation::IntelligentScissorsMB*>* ocvrs_return) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->applyImage(*image);
			Ok(new cv::segmentation::IntelligentScissorsMB(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// applyImageFeatures(InputArray, InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc/segmentation.hpp:105
	// ("cv::segmentation::IntelligentScissorsMB::applyImageFeatures", vec![(pred!(mut, ["non_edge", "gradient_direction", "gradient_magnitude", "image"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_segmentation_IntelligentScissorsMB_applyImageFeatures_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(cv::segmentation::IntelligentScissorsMB* instance, const cv::_InputArray* non_edge, const cv::_InputArray* gradient_direction, const cv::_InputArray* gradient_magnitude, const cv::_InputArray* image, Result<cv::segmentation::IntelligentScissorsMB*>* ocvrs_return) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->applyImageFeatures(*non_edge, *gradient_direction, *gradient_magnitude, *image);
			Ok(new cv::segmentation::IntelligentScissorsMB(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::segmentation::IntelligentScissorsMB::applyImageFeatures(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc/segmentation.hpp:105
	// ("cv::segmentation::IntelligentScissorsMB::applyImageFeatures", vec![(pred!(mut, ["non_edge", "gradient_direction", "gradient_magnitude"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_segmentation_IntelligentScissorsMB_applyImageFeatures_const__InputArrayR_const__InputArrayR_const__InputArrayR(cv::segmentation::IntelligentScissorsMB* instance, const cv::_InputArray* non_edge, const cv::_InputArray* gradient_direction, const cv::_InputArray* gradient_magnitude, Result<cv::segmentation::IntelligentScissorsMB*>* ocvrs_return) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->applyImageFeatures(*non_edge, *gradient_direction, *gradient_magnitude);
			Ok(new cv::segmentation::IntelligentScissorsMB(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// buildMap(const Point &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc/segmentation.hpp:116
	// ("cv::segmentation::IntelligentScissorsMB::buildMap", vec![(pred!(mut, ["sourcePt"], ["const cv::Point*"]), _)]),
	void cv_segmentation_IntelligentScissorsMB_buildMap_const_PointR(cv::segmentation::IntelligentScissorsMB* instance, const cv::Point* sourcePt, ResultVoid* ocvrs_return) {
		try {
			instance->buildMap(*sourcePt);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getContour(const Point &, OutputArray, bool)(SimpleClass, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc/segmentation.hpp:126
	// ("cv::segmentation::IntelligentScissorsMB::getContour", vec![(pred!(const, ["targetPt", "contour", "backward"], ["const cv::Point*", "const cv::_OutputArray*", "bool"]), _)]),
	void cv_segmentation_IntelligentScissorsMB_getContour_const_const_PointR_const__OutputArrayR_bool(const cv::segmentation::IntelligentScissorsMB* instance, const cv::Point* targetPt, const cv::_OutputArray* contour, bool backward, ResultVoid* ocvrs_return) {
		try {
			instance->getContour(*targetPt, *contour, backward);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::segmentation::IntelligentScissorsMB::getContour(SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgproc/segmentation.hpp:126
	// ("cv::segmentation::IntelligentScissorsMB::getContour", vec![(pred!(const, ["targetPt", "contour"], ["const cv::Point*", "const cv::_OutputArray*"]), _)]),
	void cv_segmentation_IntelligentScissorsMB_getContour_const_const_PointR_const__OutputArrayR(const cv::segmentation::IntelligentScissorsMB* instance, const cv::Point* targetPt, const cv::_OutputArray* contour, ResultVoid* ocvrs_return) {
		try {
			instance->getContour(*targetPt, *contour);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::segmentation::IntelligentScissorsMB::implicitClone() generated
	// ("cv::segmentation::IntelligentScissorsMB::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::segmentation::IntelligentScissorsMB* cv_segmentation_IntelligentScissorsMB_implicitClone_const(const cv::segmentation::IntelligentScissorsMB* instance) {
			return new cv::segmentation::IntelligentScissorsMB(*instance);
	}

	// cv::segmentation::IntelligentScissorsMB::delete() generated
	// ("cv::segmentation::IntelligentScissorsMB::delete", vec![(pred!(mut, [], []), _)]),
	void cv_segmentation_IntelligentScissorsMB_delete(cv::segmentation::IntelligentScissorsMB* instance) {
			delete instance;
	}

}
