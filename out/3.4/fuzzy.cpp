#include "ocvrs_common.hpp"
#include <opencv2/fuzzy.hpp>
#include "fuzzy_types.hpp"

extern "C" {
	// FT02D_FL_process(InputArray, const int, OutputArray)(InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F0_math.hpp:106
	// ("cv::ft::FT02D_FL_process", vec![(pred!(mut, ["matrix", "radius", "output"], ["const cv::_InputArray*", "const int", "const cv::_OutputArray*"]), _)]),
	void cv_ft_FT02D_FL_process_const__InputArrayR_const_int_const__OutputArrayR(const cv::_InputArray* matrix, const int radius, const cv::_OutputArray* output, ResultVoid* ocvrs_return) {
		try {
			cv::ft::FT02D_FL_process(*matrix, radius, *output);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// FT02D_FL_process_float(InputArray, const int, OutputArray)(InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F0_math.hpp:115
	// ("cv::ft::FT02D_FL_process_float", vec![(pred!(mut, ["matrix", "radius", "output"], ["const cv::_InputArray*", "const int", "const cv::_OutputArray*"]), _)]),
	void cv_ft_FT02D_FL_process_float_const__InputArrayR_const_int_const__OutputArrayR(const cv::_InputArray* matrix, const int radius, const cv::_OutputArray* output, ResultVoid* ocvrs_return) {
		try {
			cv::ft::FT02D_FL_process_float(*matrix, radius, *output);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ft::FT02D_components(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F0_math.hpp:64
	// ("cv::ft::FT02D_components", vec![(pred!(mut, ["matrix", "kernel", "components"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ft_FT02D_components_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* matrix, const cv::_InputArray* kernel, const cv::_OutputArray* components, ResultVoid* ocvrs_return) {
		try {
			cv::ft::FT02D_components(*matrix, *kernel, *components);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// FT02D_components(InputArray, InputArray, OutputArray, InputArray)(InputArray, InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F0_math.hpp:64
	// ("cv::ft::FT02D_components", vec![(pred!(mut, ["matrix", "kernel", "components", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_ft_FT02D_components_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* matrix, const cv::_InputArray* kernel, const cv::_OutputArray* components, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			cv::ft::FT02D_components(*matrix, *kernel, *components, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// FT02D_inverseFT(InputArray, InputArray, OutputArray, int, int)(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F0_math.hpp:75
	// ("cv::ft::FT02D_inverseFT", vec![(pred!(mut, ["components", "kernel", "output", "width", "height"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_ft_FT02D_inverseFT_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* components, const cv::_InputArray* kernel, const cv::_OutputArray* output, int width, int height, ResultVoid* ocvrs_return) {
		try {
			cv::ft::FT02D_inverseFT(*components, *kernel, *output, width, height);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// FT02D_iteration(InputArray, InputArray, OutputArray, InputArray, OutputArray, bool)(InputArray, InputArray, OutputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F0_math.hpp:97
	// ("cv::ft::FT02D_iteration", vec![(pred!(mut, ["matrix", "kernel", "output", "mask", "maskOutput", "firstStop"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "bool"]), _)]),
	void cv_ft_FT02D_iteration_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__OutputArrayR_bool(const cv::_InputArray* matrix, const cv::_InputArray* kernel, const cv::_OutputArray* output, const cv::_InputArray* mask, const cv::_OutputArray* maskOutput, bool firstStop, Result<int>* ocvrs_return) {
		try {
			int ret = cv::ft::FT02D_iteration(*matrix, *kernel, *output, *mask, *maskOutput, firstStop);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ft::FT02D_process(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F0_math.hpp:85
	// ("cv::ft::FT02D_process", vec![(pred!(mut, ["matrix", "kernel", "output"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ft_FT02D_process_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* matrix, const cv::_InputArray* kernel, const cv::_OutputArray* output, ResultVoid* ocvrs_return) {
		try {
			cv::ft::FT02D_process(*matrix, *kernel, *output);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// FT02D_process(InputArray, InputArray, OutputArray, InputArray)(InputArray, InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F0_math.hpp:85
	// ("cv::ft::FT02D_process", vec![(pred!(mut, ["matrix", "kernel", "output", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_ft_FT02D_process_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* matrix, const cv::_InputArray* kernel, const cv::_OutputArray* output, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			cv::ft::FT02D_process(*matrix, *kernel, *output, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// FT12D_components(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F1_math.hpp:63
	// ("cv::ft::FT12D_components", vec![(pred!(mut, ["matrix", "kernel", "components"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ft_FT12D_components_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* matrix, const cv::_InputArray* kernel, const cv::_OutputArray* components, ResultVoid* ocvrs_return) {
		try {
			cv::ft::FT12D_components(*matrix, *kernel, *components);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// FT12D_createPolynomMatrixHorizontal(int, OutputArray, const int)(Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F1_math.hpp:94
	// ("cv::ft::FT12D_createPolynomMatrixHorizontal", vec![(pred!(mut, ["radius", "matrix", "chn"], ["int", "const cv::_OutputArray*", "const int"]), _)]),
	void cv_ft_FT12D_createPolynomMatrixHorizontal_int_const__OutputArrayR_const_int(int radius, const cv::_OutputArray* matrix, const int chn, ResultVoid* ocvrs_return) {
		try {
			cv::ft::FT12D_createPolynomMatrixHorizontal(radius, *matrix, chn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// FT12D_createPolynomMatrixVertical(int, OutputArray, const int)(Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F1_math.hpp:85
	// ("cv::ft::FT12D_createPolynomMatrixVertical", vec![(pred!(mut, ["radius", "matrix", "chn"], ["int", "const cv::_OutputArray*", "const int"]), _)]),
	void cv_ft_FT12D_createPolynomMatrixVertical_int_const__OutputArrayR_const_int(int radius, const cv::_OutputArray* matrix, const int chn, ResultVoid* ocvrs_return) {
		try {
			cv::ft::FT12D_createPolynomMatrixVertical(radius, *matrix, chn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// FT12D_inverseFT(InputArray, InputArray, OutputArray, int, int)(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F1_math.hpp:118
	// ("cv::ft::FT12D_inverseFT", vec![(pred!(mut, ["components", "kernel", "output", "width", "height"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_ft_FT12D_inverseFT_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* components, const cv::_InputArray* kernel, const cv::_OutputArray* output, int width, int height, ResultVoid* ocvrs_return) {
		try {
			cv::ft::FT12D_inverseFT(*components, *kernel, *output, width, height);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ft::FT12D_polynomial(InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F1_math.hpp:76
	// ("cv::ft::FT12D_polynomial", vec![(pred!(mut, ["matrix", "kernel", "c00", "c10", "c01", "components"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ft_FT12D_polynomial_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* matrix, const cv::_InputArray* kernel, const cv::_OutputArray* c00, const cv::_OutputArray* c10, const cv::_OutputArray* c01, const cv::_OutputArray* components, ResultVoid* ocvrs_return) {
		try {
			cv::ft::FT12D_polynomial(*matrix, *kernel, *c00, *c10, *c01, *components);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// FT12D_polynomial(InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, InputArray)(InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F1_math.hpp:76
	// ("cv::ft::FT12D_polynomial", vec![(pred!(mut, ["matrix", "kernel", "c00", "c10", "c01", "components", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_ft_FT12D_polynomial_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* matrix, const cv::_InputArray* kernel, const cv::_OutputArray* c00, const cv::_OutputArray* c10, const cv::_OutputArray* c01, const cv::_OutputArray* components, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			cv::ft::FT12D_polynomial(*matrix, *kernel, *c00, *c10, *c01, *components, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ft::FT12D_process(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F1_math.hpp:107
	// ("cv::ft::FT12D_process", vec![(pred!(mut, ["matrix", "kernel", "output"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ft_FT12D_process_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* matrix, const cv::_InputArray* kernel, const cv::_OutputArray* output, ResultVoid* ocvrs_return) {
		try {
			cv::ft::FT12D_process(*matrix, *kernel, *output);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// FT12D_process(InputArray, InputArray, OutputArray, InputArray)(InputArray, InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F1_math.hpp:107
	// ("cv::ft::FT12D_process", vec![(pred!(mut, ["matrix", "kernel", "output", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_ft_FT12D_process_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* matrix, const cv::_InputArray* kernel, const cv::_OutputArray* output, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			cv::ft::FT12D_process(*matrix, *kernel, *output, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createKernel(InputArray, InputArray, OutputArray, const int)(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_image.hpp:64
	// ("cv::ft::createKernel", vec![(pred!(mut, ["A", "B", "kernel", "chn"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const int"]), _)]),
	void cv_ft_createKernel_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_int(const cv::_InputArray* A, const cv::_InputArray* B, const cv::_OutputArray* kernel, const int chn, ResultVoid* ocvrs_return) {
		try {
			cv::ft::createKernel(*A, *B, *kernel, chn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createKernel(int, int, OutputArray, const int)(Primitive, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_image.hpp:75
	// ("cv::ft::createKernel", vec![(pred!(mut, ["function", "radius", "kernel", "chn"], ["int", "int", "const cv::_OutputArray*", "const int"]), _)]),
	void cv_ft_createKernel_int_int_const__OutputArrayR_const_int(int function, int radius, const cv::_OutputArray* kernel, const int chn, ResultVoid* ocvrs_return) {
		try {
			cv::ft::createKernel(function, radius, *kernel, chn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// filter(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_image.hpp:103
	// ("cv::ft::filter", vec![(pred!(mut, ["image", "kernel", "output"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ft_filter_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* image, const cv::_InputArray* kernel, const cv::_OutputArray* output, ResultVoid* ocvrs_return) {
		try {
			cv::ft::filter(*image, *kernel, *output);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// inpaint(InputArray, InputArray, OutputArray, int, int, int)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_image.hpp:94
	// ("cv::ft::inpaint", vec![(pred!(mut, ["image", "mask", "output", "radius", "function", "algorithm"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
	void cv_ft_inpaint_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* image, const cv::_InputArray* mask, const cv::_OutputArray* output, int radius, int function, int algorithm, ResultVoid* ocvrs_return) {
		try {
			cv::ft::inpaint(*image, *mask, *output, radius, function, algorithm);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}
