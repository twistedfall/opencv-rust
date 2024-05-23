#include "photo.hpp"
#include "photo_types.hpp"

extern "C" {
	// cv::colorChange(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:795
	// ("cv::colorChange", vec![(pred!(mut, ["src", "mask", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_colorChange_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_InputArray* mask, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::colorChange(*src, *mask, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// colorChange(InputArray, InputArray, OutputArray, float, float, float)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:795
	// ("cv::colorChange", vec![(pred!(mut, ["src", "mask", "dst", "red_mul", "green_mul", "blue_mul"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "float", "float", "float"]), _)]),
	void cv_colorChange_const__InputArrayR_const__InputArrayR_const__OutputArrayR_float_float_float(const cv::_InputArray* src, const cv::_InputArray* mask, const cv::_OutputArray* dst, float red_mul, float green_mul, float blue_mul, ResultVoid* ocvrs_return) {
		try {
			cv::colorChange(*src, *mask, *dst, red_mul, green_mul, blue_mul);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::createAlignMTB() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:526
	// ("cv::createAlignMTB", vec![(pred!(mut, [], []), _)]),
	void cv_createAlignMTB(Result<cv::Ptr<cv::AlignMTB>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::AlignMTB> ret = cv::createAlignMTB();
			Ok(new cv::Ptr<cv::AlignMTB>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createAlignMTB(int, int, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:526
	// ("cv::createAlignMTB", vec![(pred!(mut, ["max_bits", "exclude_range", "cut"], ["int", "int", "bool"]), _)]),
	void cv_createAlignMTB_int_int_bool(int max_bits, int exclude_range, bool cut, Result<cv::Ptr<cv::AlignMTB>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::AlignMTB> ret = cv::createAlignMTB(max_bits, exclude_range, cut);
			Ok(new cv::Ptr<cv::AlignMTB>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::createCalibrateDebevec() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:569
	// ("cv::createCalibrateDebevec", vec![(pred!(mut, [], []), _)]),
	void cv_createCalibrateDebevec(Result<cv::Ptr<cv::CalibrateDebevec>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::CalibrateDebevec> ret = cv::createCalibrateDebevec();
			Ok(new cv::Ptr<cv::CalibrateDebevec>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createCalibrateDebevec(int, float, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:569
	// ("cv::createCalibrateDebevec", vec![(pred!(mut, ["samples", "lambda", "random"], ["int", "float", "bool"]), _)]),
	void cv_createCalibrateDebevec_int_float_bool(int samples, float lambda, bool random, Result<cv::Ptr<cv::CalibrateDebevec>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::CalibrateDebevec> ret = cv::createCalibrateDebevec(samples, lambda, random);
			Ok(new cv::Ptr<cv::CalibrateDebevec>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::createCalibrateRobertson() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:593
	// ("cv::createCalibrateRobertson", vec![(pred!(mut, [], []), _)]),
	void cv_createCalibrateRobertson(Result<cv::Ptr<cv::CalibrateRobertson>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::CalibrateRobertson> ret = cv::createCalibrateRobertson();
			Ok(new cv::Ptr<cv::CalibrateRobertson>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createCalibrateRobertson(int, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:593
	// ("cv::createCalibrateRobertson", vec![(pred!(mut, ["max_iter", "threshold"], ["int", "float"]), _)]),
	void cv_createCalibrateRobertson_int_float(int max_iter, float threshold, Result<cv::Ptr<cv::CalibrateRobertson>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::CalibrateRobertson> ret = cv::createCalibrateRobertson(max_iter, threshold);
			Ok(new cv::Ptr<cv::CalibrateRobertson>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createMergeDebevec()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:627
	// ("cv::createMergeDebevec", vec![(pred!(mut, [], []), _)]),
	void cv_createMergeDebevec(Result<cv::Ptr<cv::MergeDebevec>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::MergeDebevec> ret = cv::createMergeDebevec();
			Ok(new cv::Ptr<cv::MergeDebevec>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::createMergeMertens() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:669
	// ("cv::createMergeMertens", vec![(pred!(mut, [], []), _)]),
	void cv_createMergeMertens(Result<cv::Ptr<cv::MergeMertens>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::MergeMertens> ret = cv::createMergeMertens();
			Ok(new cv::Ptr<cv::MergeMertens>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createMergeMertens(float, float, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:669
	// ("cv::createMergeMertens", vec![(pred!(mut, ["contrast_weight", "saturation_weight", "exposure_weight"], ["float", "float", "float"]), _)]),
	void cv_createMergeMertens_float_float_float(float contrast_weight, float saturation_weight, float exposure_weight, Result<cv::Ptr<cv::MergeMertens>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::MergeMertens> ret = cv::createMergeMertens(contrast_weight, saturation_weight, exposure_weight);
			Ok(new cv::Ptr<cv::MergeMertens>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createMergeRobertson()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:686
	// ("cv::createMergeRobertson", vec![(pred!(mut, [], []), _)]),
	void cv_createMergeRobertson(Result<cv::Ptr<cv::MergeRobertson>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::MergeRobertson> ret = cv::createMergeRobertson();
			Ok(new cv::Ptr<cv::MergeRobertson>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::createTonemap() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:355
	// ("cv::createTonemap", vec![(pred!(mut, [], []), _)]),
	void cv_createTonemap(Result<cv::Ptr<cv::Tonemap>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::Tonemap> ret = cv::createTonemap();
			Ok(new cv::Ptr<cv::Tonemap>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::createTonemapDrago() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:386
	// ("cv::createTonemapDrago", vec![(pred!(mut, [], []), _)]),
	void cv_createTonemapDrago(Result<cv::Ptr<cv::TonemapDrago>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TonemapDrago> ret = cv::createTonemapDrago();
			Ok(new cv::Ptr<cv::TonemapDrago>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createTonemapDrago(float, float, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:386
	// ("cv::createTonemapDrago", vec![(pred!(mut, ["gamma", "saturation", "bias"], ["float", "float", "float"]), _)]),
	void cv_createTonemapDrago_float_float_float(float gamma, float saturation, float bias, Result<cv::Ptr<cv::TonemapDrago>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TonemapDrago> ret = cv::createTonemapDrago(gamma, saturation, bias);
			Ok(new cv::Ptr<cv::TonemapDrago>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::createTonemapMantiuk() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:445
	// ("cv::createTonemapMantiuk", vec![(pred!(mut, [], []), _)]),
	void cv_createTonemapMantiuk(Result<cv::Ptr<cv::TonemapMantiuk>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TonemapMantiuk> ret = cv::createTonemapMantiuk();
			Ok(new cv::Ptr<cv::TonemapMantiuk>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createTonemapMantiuk(float, float, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:445
	// ("cv::createTonemapMantiuk", vec![(pred!(mut, ["gamma", "scale", "saturation"], ["float", "float", "float"]), _)]),
	void cv_createTonemapMantiuk_float_float_float(float gamma, float scale, float saturation, Result<cv::Ptr<cv::TonemapMantiuk>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TonemapMantiuk> ret = cv::createTonemapMantiuk(gamma, scale, saturation);
			Ok(new cv::Ptr<cv::TonemapMantiuk>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::createTonemapReinhard() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:419
	// ("cv::createTonemapReinhard", vec![(pred!(mut, [], []), _)]),
	void cv_createTonemapReinhard(Result<cv::Ptr<cv::TonemapReinhard>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TonemapReinhard> ret = cv::createTonemapReinhard();
			Ok(new cv::Ptr<cv::TonemapReinhard>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createTonemapReinhard(float, float, float, float)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:419
	// ("cv::createTonemapReinhard", vec![(pred!(mut, ["gamma", "intensity", "light_adapt", "color_adapt"], ["float", "float", "float", "float"]), _)]),
	void cv_createTonemapReinhard_float_float_float_float(float gamma, float intensity, float light_adapt, float color_adapt, Result<cv::Ptr<cv::TonemapReinhard>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TonemapReinhard> ret = cv::createTonemapReinhard(gamma, intensity, light_adapt, color_adapt);
			Ok(new cv::Ptr<cv::TonemapReinhard>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createTonemap(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:355
	// ("cv::createTonemap", vec![(pred!(mut, ["gamma"], ["float"]), _)]),
	void cv_createTonemap_float(float gamma, Result<cv::Ptr<cv::Tonemap>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::Tonemap> ret = cv::createTonemap(gamma);
			Ok(new cv::Ptr<cv::Tonemap>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::fastNlMeansDenoisingColored(TraitClass, TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo/cuda.hpp:144
	// ("cv::cuda::fastNlMeansDenoisingColored", vec![(pred!(mut, ["src", "dst", "h_luminance", "photo_render"], ["const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "float", "float"]), _)]),
	void cv_cuda_fastNlMeansDenoisingColored_const_GpuMatR_GpuMatR_float_float(const cv::cuda::GpuMat* src, cv::cuda::GpuMat* dst, float h_luminance, float photo_render, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::fastNlMeansDenoisingColored(*src, *dst, h_luminance, photo_render);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fastNlMeansDenoisingColored(const GpuMat &, GpuMat &, float, float, int, int, Stream &)(TraitClass, TraitClass, Primitive, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo/cuda.hpp:144
	// ("cv::cuda::fastNlMeansDenoisingColored", vec![(pred!(mut, ["src", "dst", "h_luminance", "photo_render", "search_window", "block_size", "stream"], ["const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "float", "float", "int", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_fastNlMeansDenoisingColored_const_GpuMatR_GpuMatR_float_float_int_int_StreamR(const cv::cuda::GpuMat* src, cv::cuda::GpuMat* dst, float h_luminance, float photo_render, int search_window, int block_size, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::fastNlMeansDenoisingColored(*src, *dst, h_luminance, photo_render, search_window, block_size, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::fastNlMeansDenoisingColored(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo/cuda.hpp:139
	// ("cv::cuda::fastNlMeansDenoisingColored", vec![(pred!(mut, ["src", "dst", "h_luminance", "photo_render"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float", "float"]), _)]),
	void cv_cuda_fastNlMeansDenoisingColored_const__InputArrayR_const__OutputArrayR_float_float(const cv::_InputArray* src, const cv::_OutputArray* dst, float h_luminance, float photo_render, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::fastNlMeansDenoisingColored(*src, *dst, h_luminance, photo_render);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fastNlMeansDenoisingColored(InputArray, OutputArray, float, float, int, int, Stream &)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo/cuda.hpp:139
	// ("cv::cuda::fastNlMeansDenoisingColored", vec![(pred!(mut, ["src", "dst", "h_luminance", "photo_render", "search_window", "block_size", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float", "float", "int", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_fastNlMeansDenoisingColored_const__InputArrayR_const__OutputArrayR_float_float_int_int_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, float h_luminance, float photo_render, int search_window, int block_size, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::fastNlMeansDenoisingColored(*src, *dst, h_luminance, photo_render, search_window, block_size, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::fastNlMeansDenoising(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo/cuda.hpp:109
	// ("cv::cuda::fastNlMeansDenoising", vec![(pred!(mut, ["src", "dst", "h"], ["const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "float"]), _)]),
	void cv_cuda_fastNlMeansDenoising_const_GpuMatR_GpuMatR_float(const cv::cuda::GpuMat* src, cv::cuda::GpuMat* dst, float h, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::fastNlMeansDenoising(*src, *dst, h);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fastNlMeansDenoising(const GpuMat &, GpuMat &, float, int, int, Stream &)(TraitClass, TraitClass, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo/cuda.hpp:109
	// ("cv::cuda::fastNlMeansDenoising", vec![(pred!(mut, ["src", "dst", "h", "search_window", "block_size", "stream"], ["const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "float", "int", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_fastNlMeansDenoising_const_GpuMatR_GpuMatR_float_int_int_StreamR(const cv::cuda::GpuMat* src, cv::cuda::GpuMat* dst, float h, int search_window, int block_size, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::fastNlMeansDenoising(*src, *dst, h, search_window, block_size, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::fastNlMeansDenoising(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo/cuda.hpp:104
	// ("cv::cuda::fastNlMeansDenoising", vec![(pred!(mut, ["src", "dst", "h"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float"]), _)]),
	void cv_cuda_fastNlMeansDenoising_const__InputArrayR_const__OutputArrayR_float(const cv::_InputArray* src, const cv::_OutputArray* dst, float h, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::fastNlMeansDenoising(*src, *dst, h);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fastNlMeansDenoising(InputArray, OutputArray, float, int, int, Stream &)(InputArray, OutputArray, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo/cuda.hpp:104
	// ("cv::cuda::fastNlMeansDenoising", vec![(pred!(mut, ["src", "dst", "h", "search_window", "block_size", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float", "int", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_fastNlMeansDenoising_const__InputArrayR_const__OutputArrayR_float_int_int_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, float h, int search_window, int block_size, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::fastNlMeansDenoising(*src, *dst, h, search_window, block_size, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::nonLocalMeans(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo/cuda.hpp:73
	// ("cv::cuda::nonLocalMeans", vec![(pred!(mut, ["src", "dst", "h"], ["const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "float"]), _)]),
	void cv_cuda_nonLocalMeans_const_GpuMatR_GpuMatR_float(const cv::cuda::GpuMat* src, cv::cuda::GpuMat* dst, float h, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::nonLocalMeans(*src, *dst, h);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// nonLocalMeans(const GpuMat &, GpuMat &, float, int, int, int, Stream &)(TraitClass, TraitClass, Primitive, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo/cuda.hpp:73
	// ("cv::cuda::nonLocalMeans", vec![(pred!(mut, ["src", "dst", "h", "search_window", "block_size", "borderMode", "stream"], ["const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "float", "int", "int", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_nonLocalMeans_const_GpuMatR_GpuMatR_float_int_int_int_StreamR(const cv::cuda::GpuMat* src, cv::cuda::GpuMat* dst, float h, int search_window, int block_size, int borderMode, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::nonLocalMeans(*src, *dst, h, search_window, block_size, borderMode, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::nonLocalMeans(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo/cuda.hpp:67
	// ("cv::cuda::nonLocalMeans", vec![(pred!(mut, ["src", "dst", "h"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float"]), _)]),
	void cv_cuda_nonLocalMeans_const__InputArrayR_const__OutputArrayR_float(const cv::_InputArray* src, const cv::_OutputArray* dst, float h, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::nonLocalMeans(*src, *dst, h);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// nonLocalMeans(InputArray, OutputArray, float, int, int, int, Stream &)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo/cuda.hpp:67
	// ("cv::cuda::nonLocalMeans", vec![(pred!(mut, ["src", "dst", "h", "search_window", "block_size", "borderMode", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float", "int", "int", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_nonLocalMeans_const__InputArrayR_const__OutputArrayR_float_int_int_int_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, float h, int search_window, int block_size, int borderMode, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::nonLocalMeans(*src, *dst, h, search_window, block_size, borderMode, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// decolor(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:703
	// ("cv::decolor", vec![(pred!(mut, ["src", "grayscale", "color_boost"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_decolor_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* grayscale, const cv::_OutputArray* color_boost, ResultVoid* ocvrs_return) {
		try {
			cv::decolor(*src, *grayscale, *color_boost);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::denoise_TVL1(CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:324
	// ("cv::denoise_TVL1", vec![(pred!(mut, ["observations", "result"], ["const std::vector<cv::Mat>*", "cv::Mat*"]), _)]),
	void cv_denoise_TVL1_const_vectorLMatGR_MatR(const std::vector<cv::Mat>* observations, cv::Mat* result, ResultVoid* ocvrs_return) {
		try {
			cv::denoise_TVL1(*observations, *result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// denoise_TVL1(const std::vector<Mat> &, Mat &, double, int)(CppPassByVoidPtr, TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:324
	// ("cv::denoise_TVL1", vec![(pred!(mut, ["observations", "result", "lambda", "niters"], ["const std::vector<cv::Mat>*", "cv::Mat*", "double", "int"]), _)]),
	void cv_denoise_TVL1_const_vectorLMatGR_MatR_double_int(const std::vector<cv::Mat>* observations, cv::Mat* result, double lambda, int niters, ResultVoid* ocvrs_return) {
		try {
			cv::denoise_TVL1(*observations, *result, lambda, niters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::detailEnhance(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:862
	// ("cv::detailEnhance", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_detailEnhance_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::detailEnhance(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detailEnhance(InputArray, OutputArray, float, float)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:862
	// ("cv::detailEnhance", vec![(pred!(mut, ["src", "dst", "sigma_s", "sigma_r"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float", "float"]), _)]),
	void cv_detailEnhance_const__InputArrayR_const__OutputArrayR_float_float(const cv::_InputArray* src, const cv::_OutputArray* dst, float sigma_s, float sigma_r, ResultVoid* ocvrs_return) {
		try {
			cv::detailEnhance(*src, *dst, sigma_s, sigma_r);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::edgePreservingFilter(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:852
	// ("cv::edgePreservingFilter", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_edgePreservingFilter_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::edgePreservingFilter(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// edgePreservingFilter(InputArray, OutputArray, int, float, float)(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:852
	// ("cv::edgePreservingFilter", vec![(pred!(mut, ["src", "dst", "flags", "sigma_s", "sigma_r"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "float", "float"]), _)]),
	void cv_edgePreservingFilter_const__InputArrayR_const__OutputArrayR_int_float_float(const cv::_InputArray* src, const cv::_OutputArray* dst, int flags, float sigma_s, float sigma_r, ResultVoid* ocvrs_return) {
		try {
			cv::edgePreservingFilter(*src, *dst, flags, sigma_s, sigma_r);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::fastNlMeansDenoisingColoredMulti(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:282
	// ("cv::fastNlMeansDenoisingColoredMulti", vec![(pred!(mut, ["srcImgs", "dst", "imgToDenoiseIndex", "temporalWindowSize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_fastNlMeansDenoisingColoredMulti_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* srcImgs, const cv::_OutputArray* dst, int imgToDenoiseIndex, int temporalWindowSize, ResultVoid* ocvrs_return) {
		try {
			cv::fastNlMeansDenoisingColoredMulti(*srcImgs, *dst, imgToDenoiseIndex, temporalWindowSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fastNlMeansDenoisingColoredMulti(InputArrayOfArrays, OutputArray, int, int, float, float, int, int)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:282
	// ("cv::fastNlMeansDenoisingColoredMulti", vec![(pred!(mut, ["srcImgs", "dst", "imgToDenoiseIndex", "temporalWindowSize", "h", "hColor", "templateWindowSize", "searchWindowSize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "float", "float", "int", "int"]), _)]),
	void cv_fastNlMeansDenoisingColoredMulti_const__InputArrayR_const__OutputArrayR_int_int_float_float_int_int(const cv::_InputArray* srcImgs, const cv::_OutputArray* dst, int imgToDenoiseIndex, int temporalWindowSize, float h, float hColor, int templateWindowSize, int searchWindowSize, ResultVoid* ocvrs_return) {
		try {
			cv::fastNlMeansDenoisingColoredMulti(*srcImgs, *dst, imgToDenoiseIndex, temporalWindowSize, h, hColor, templateWindowSize, searchWindowSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::fastNlMeansDenoisingColored(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:197
	// ("cv::fastNlMeansDenoisingColored", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_fastNlMeansDenoisingColored_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::fastNlMeansDenoisingColored(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fastNlMeansDenoisingColored(InputArray, OutputArray, float, float, int, int)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:197
	// ("cv::fastNlMeansDenoisingColored", vec![(pred!(mut, ["src", "dst", "h", "hColor", "templateWindowSize", "searchWindowSize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float", "float", "int", "int"]), _)]),
	void cv_fastNlMeansDenoisingColored_const__InputArrayR_const__OutputArrayR_float_float_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, float h, float hColor, int templateWindowSize, int searchWindowSize, ResultVoid* ocvrs_return) {
		try {
			cv::fastNlMeansDenoisingColored(*src, *dst, h, hColor, templateWindowSize, searchWindowSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::fastNlMeansDenoisingMulti(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:224
	// ("cv::fastNlMeansDenoisingMulti", vec![(pred!(mut, ["srcImgs", "dst", "imgToDenoiseIndex", "temporalWindowSize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_fastNlMeansDenoisingMulti_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* srcImgs, const cv::_OutputArray* dst, int imgToDenoiseIndex, int temporalWindowSize, ResultVoid* ocvrs_return) {
		try {
			cv::fastNlMeansDenoisingMulti(*srcImgs, *dst, imgToDenoiseIndex, temporalWindowSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::fastNlMeansDenoisingMulti(InputArray, OutputArray, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:253
	// ("cv::fastNlMeansDenoisingMulti", vec![(pred!(mut, ["srcImgs", "dst", "imgToDenoiseIndex", "temporalWindowSize", "h"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "const std::vector<float>*"]), _)]),
	void cv_fastNlMeansDenoisingMulti_const__InputArrayR_const__OutputArrayR_int_int_const_vectorLfloatGR(const cv::_InputArray* srcImgs, const cv::_OutputArray* dst, int imgToDenoiseIndex, int temporalWindowSize, const std::vector<float>* h, ResultVoid* ocvrs_return) {
		try {
			cv::fastNlMeansDenoisingMulti(*srcImgs, *dst, imgToDenoiseIndex, temporalWindowSize, *h);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fastNlMeansDenoisingMulti(InputArrayOfArrays, OutputArray, int, int, const std::vector<float> &, int, int, int)(InputArray, OutputArray, Primitive, Primitive, CppPassByVoidPtr, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:253
	// ("cv::fastNlMeansDenoisingMulti", vec![(pred!(mut, ["srcImgs", "dst", "imgToDenoiseIndex", "temporalWindowSize", "h", "templateWindowSize", "searchWindowSize", "normType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "const std::vector<float>*", "int", "int", "int"]), _)]),
	void cv_fastNlMeansDenoisingMulti_const__InputArrayR_const__OutputArrayR_int_int_const_vectorLfloatGR_int_int_int(const cv::_InputArray* srcImgs, const cv::_OutputArray* dst, int imgToDenoiseIndex, int temporalWindowSize, const std::vector<float>* h, int templateWindowSize, int searchWindowSize, int normType, ResultVoid* ocvrs_return) {
		try {
			cv::fastNlMeansDenoisingMulti(*srcImgs, *dst, imgToDenoiseIndex, temporalWindowSize, *h, templateWindowSize, searchWindowSize, normType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fastNlMeansDenoisingMulti(InputArrayOfArrays, OutputArray, int, int, float, int, int)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:224
	// ("cv::fastNlMeansDenoisingMulti", vec![(pred!(mut, ["srcImgs", "dst", "imgToDenoiseIndex", "temporalWindowSize", "h", "templateWindowSize", "searchWindowSize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "float", "int", "int"]), _)]),
	void cv_fastNlMeansDenoisingMulti_const__InputArrayR_const__OutputArrayR_int_int_float_int_int(const cv::_InputArray* srcImgs, const cv::_OutputArray* dst, int imgToDenoiseIndex, int temporalWindowSize, float h, int templateWindowSize, int searchWindowSize, ResultVoid* ocvrs_return) {
		try {
			cv::fastNlMeansDenoisingMulti(*srcImgs, *dst, imgToDenoiseIndex, temporalWindowSize, h, templateWindowSize, searchWindowSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::fastNlMeansDenoising(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:147
	// ("cv::fastNlMeansDenoising", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_fastNlMeansDenoising_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::fastNlMeansDenoising(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::fastNlMeansDenoising(InputArray, OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:174
	// ("cv::fastNlMeansDenoising", vec![(pred!(mut, ["src", "dst", "h"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const std::vector<float>*"]), _)]),
	void cv_fastNlMeansDenoising_const__InputArrayR_const__OutputArrayR_const_vectorLfloatGR(const cv::_InputArray* src, const cv::_OutputArray* dst, const std::vector<float>* h, ResultVoid* ocvrs_return) {
		try {
			cv::fastNlMeansDenoising(*src, *dst, *h);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fastNlMeansDenoising(InputArray, OutputArray, const std::vector<float> &, int, int, int)(InputArray, OutputArray, CppPassByVoidPtr, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:174
	// ("cv::fastNlMeansDenoising", vec![(pred!(mut, ["src", "dst", "h", "templateWindowSize", "searchWindowSize", "normType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const std::vector<float>*", "int", "int", "int"]), _)]),
	void cv_fastNlMeansDenoising_const__InputArrayR_const__OutputArrayR_const_vectorLfloatGR_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, const std::vector<float>* h, int templateWindowSize, int searchWindowSize, int normType, ResultVoid* ocvrs_return) {
		try {
			cv::fastNlMeansDenoising(*src, *dst, *h, templateWindowSize, searchWindowSize, normType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fastNlMeansDenoising(InputArray, OutputArray, float, int, int)(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:147
	// ("cv::fastNlMeansDenoising", vec![(pred!(mut, ["src", "dst", "h", "templateWindowSize", "searchWindowSize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float", "int", "int"]), _)]),
	void cv_fastNlMeansDenoising_const__InputArrayR_const__OutputArrayR_float_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, float h, int templateWindowSize, int searchWindowSize, ResultVoid* ocvrs_return) {
		try {
			cv::fastNlMeansDenoising(*src, *dst, h, templateWindowSize, searchWindowSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::illuminationChange(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:809
	// ("cv::illuminationChange", vec![(pred!(mut, ["src", "mask", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_illuminationChange_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_InputArray* mask, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::illuminationChange(*src, *mask, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// illuminationChange(InputArray, InputArray, OutputArray, float, float)(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:809
	// ("cv::illuminationChange", vec![(pred!(mut, ["src", "mask", "dst", "alpha", "beta"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "float", "float"]), _)]),
	void cv_illuminationChange_const__InputArrayR_const__InputArrayR_const__OutputArrayR_float_float(const cv::_InputArray* src, const cv::_InputArray* mask, const cv::_OutputArray* dst, float alpha, float beta, ResultVoid* ocvrs_return) {
		try {
			cv::illuminationChange(*src, *mask, *dst, alpha, beta);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// inpaint(InputArray, InputArray, OutputArray, double, int)(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:119
	// ("cv::inpaint", vec![(pred!(mut, ["src", "inpaintMask", "dst", "inpaintRadius", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "int"]), _)]),
	void cv_inpaint_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_int(const cv::_InputArray* src, const cv::_InputArray* inpaintMask, const cv::_OutputArray* dst, double inpaintRadius, int flags, ResultVoid* ocvrs_return) {
		try {
			cv::inpaint(*src, *inpaintMask, *dst, inpaintRadius, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::pencilSketch(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:877
	// ("cv::pencilSketch", vec![(pred!(mut, ["src", "dst1", "dst2"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_pencilSketch_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst1, const cv::_OutputArray* dst2, ResultVoid* ocvrs_return) {
		try {
			cv::pencilSketch(*src, *dst1, *dst2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// pencilSketch(InputArray, OutputArray, OutputArray, float, float, float)(InputArray, OutputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:877
	// ("cv::pencilSketch", vec![(pred!(mut, ["src", "dst1", "dst2", "sigma_s", "sigma_r", "shade_factor"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "float", "float", "float"]), _)]),
	void cv_pencilSketch_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_float_float_float(const cv::_InputArray* src, const cv::_OutputArray* dst1, const cv::_OutputArray* dst2, float sigma_s, float sigma_r, float shade_factor, ResultVoid* ocvrs_return) {
		try {
			cv::pencilSketch(*src, *dst1, *dst2, sigma_s, sigma_r, shade_factor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// seamlessClone(InputArray, InputArray, InputArray, Point, OutputArray, int)(InputArray, InputArray, InputArray, SimpleClass, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:780
	// ("cv::seamlessClone", vec![(pred!(mut, ["src", "dst", "mask", "p", "blend", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Point", "const cv::_OutputArray*", "int"]), _)]),
	void cv_seamlessClone_const__InputArrayR_const__InputArrayR_const__InputArrayR_Point_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_InputArray* dst, const cv::_InputArray* mask, cv::Point* p, const cv::_OutputArray* blend, int flags, ResultVoid* ocvrs_return) {
		try {
			cv::seamlessClone(*src, *dst, *mask, *p, *blend, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::stylization(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:889
	// ("cv::stylization", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_stylization_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::stylization(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stylization(InputArray, OutputArray, float, float)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:889
	// ("cv::stylization", vec![(pred!(mut, ["src", "dst", "sigma_s", "sigma_r"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float", "float"]), _)]),
	void cv_stylization_const__InputArrayR_const__OutputArrayR_float_float(const cv::_InputArray* src, const cv::_OutputArray* dst, float sigma_s, float sigma_r, ResultVoid* ocvrs_return) {
		try {
			cv::stylization(*src, *dst, sigma_s, sigma_r);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::textureFlattening(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:827
	// ("cv::textureFlattening", vec![(pred!(mut, ["src", "mask", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_textureFlattening_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_InputArray* mask, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::textureFlattening(*src, *mask, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// textureFlattening(InputArray, InputArray, OutputArray, float, float, int)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:827
	// ("cv::textureFlattening", vec![(pred!(mut, ["src", "mask", "dst", "low_threshold", "high_threshold", "kernel_size"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "float", "float", "int"]), _)]),
	void cv_textureFlattening_const__InputArrayR_const__InputArrayR_const__OutputArrayR_float_float_int(const cv::_InputArray* src, const cv::_InputArray* mask, const cv::_OutputArray* dst, float low_threshold, float high_threshold, int kernel_size, ResultVoid* ocvrs_return) {
		try {
			cv::textureFlattening(*src, *mask, *dst, low_threshold, high_threshold, kernel_size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// process(InputArrayOfArrays, std::vector<Mat> &, InputArray, InputArray)(InputArray, CppPassByVoidPtr, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:460
	// ("cv::AlignExposures::process", vec![(pred!(mut, ["src", "dst", "times", "response"], ["const cv::_InputArray*", "std::vector<cv::Mat>*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_AlignExposures_process_const__InputArrayR_vectorLMatGR_const__InputArrayR_const__InputArrayR(cv::AlignExposures* instance, const cv::_InputArray* src, std::vector<cv::Mat>* dst, const cv::_InputArray* times, const cv::_InputArray* response, ResultVoid* ocvrs_return) {
		try {
			instance->process(*src, *dst, *times, *response);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::AlignExposures::to_AlignMTB() generated
	// ("cv::AlignExposures::to_AlignMTB", vec![(pred!(mut, [], []), _)]),
	cv::AlignMTB* cv_AlignExposures_to_AlignMTB(cv::AlignExposures* instance) {
			return dynamic_cast<cv::AlignMTB*>(instance);
	}

	// cv::AlignExposures::to_Algorithm() generated
	// ("cv::AlignExposures::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_AlignExposures_to_Algorithm(cv::AlignExposures* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::AlignExposures::delete() generated
	// ("cv::AlignExposures::delete", vec![(pred!(mut, [], []), _)]),
	void cv_AlignExposures_delete(cv::AlignExposures* instance) {
			delete instance;
	}

	// process(InputArrayOfArrays, std::vector<Mat> &, InputArray, InputArray)(InputArray, CppPassByVoidPtr, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:476
	// ("cv::AlignMTB::process", vec![(pred!(mut, ["src", "dst", "times", "response"], ["const cv::_InputArray*", "std::vector<cv::Mat>*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_AlignMTB_process_const__InputArrayR_vectorLMatGR_const__InputArrayR_const__InputArrayR(cv::AlignMTB* instance, const cv::_InputArray* src, std::vector<cv::Mat>* dst, const cv::_InputArray* times, const cv::_InputArray* response, ResultVoid* ocvrs_return) {
		try {
			instance->process(*src, *dst, *times, *response);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// process(InputArrayOfArrays, std::vector<Mat> &)(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:484
	// ("cv::AlignMTB::process", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "std::vector<cv::Mat>*"]), _)]),
	void cv_AlignMTB_process_const__InputArrayR_vectorLMatGR(cv::AlignMTB* instance, const cv::_InputArray* src, std::vector<cv::Mat>* dst, ResultVoid* ocvrs_return) {
		try {
			instance->process(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calculateShift(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:492
	// ("cv::AlignMTB::calculateShift", vec![(pred!(mut, ["img0", "img1"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_AlignMTB_calculateShift_const__InputArrayR_const__InputArrayR(cv::AlignMTB* instance, const cv::_InputArray* img0, const cv::_InputArray* img1, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->calculateShift(*img0, *img1);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// shiftMat(InputArray, OutputArray, const Point)(InputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:499
	// ("cv::AlignMTB::shiftMat", vec![(pred!(mut, ["src", "dst", "shift"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::Point"]), _)]),
	void cv_AlignMTB_shiftMat_const__InputArrayR_const__OutputArrayR_const_Point(cv::AlignMTB* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::Point* shift, ResultVoid* ocvrs_return) {
		try {
			instance->shiftMat(*src, *dst, *shift);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeBitmaps(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:506
	// ("cv::AlignMTB::computeBitmaps", vec![(pred!(mut, ["img", "tb", "eb"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_AlignMTB_computeBitmaps_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::AlignMTB* instance, const cv::_InputArray* img, const cv::_OutputArray* tb, const cv::_OutputArray* eb, ResultVoid* ocvrs_return) {
		try {
			instance->computeBitmaps(*img, *tb, *eb);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxBits()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:508
	// ("cv::AlignMTB::getMaxBits", vec![(pred!(const, [], []), _)]),
	void cv_AlignMTB_getMaxBits_const(const cv::AlignMTB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxBits();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxBits(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:509
	// ("cv::AlignMTB::setMaxBits", vec![(pred!(mut, ["max_bits"], ["int"]), _)]),
	void cv_AlignMTB_setMaxBits_int(cv::AlignMTB* instance, int max_bits, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxBits(max_bits);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getExcludeRange()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:511
	// ("cv::AlignMTB::getExcludeRange", vec![(pred!(const, [], []), _)]),
	void cv_AlignMTB_getExcludeRange_const(const cv::AlignMTB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getExcludeRange();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setExcludeRange(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:512
	// ("cv::AlignMTB::setExcludeRange", vec![(pred!(mut, ["exclude_range"], ["int"]), _)]),
	void cv_AlignMTB_setExcludeRange_int(cv::AlignMTB* instance, int exclude_range, ResultVoid* ocvrs_return) {
		try {
			instance->setExcludeRange(exclude_range);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCut()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:514
	// ("cv::AlignMTB::getCut", vec![(pred!(const, [], []), _)]),
	void cv_AlignMTB_getCut_const(const cv::AlignMTB* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getCut();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCut(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:515
	// ("cv::AlignMTB::setCut", vec![(pred!(mut, ["value"], ["bool"]), _)]),
	void cv_AlignMTB_setCut_bool(cv::AlignMTB* instance, bool value, ResultVoid* ocvrs_return) {
		try {
			instance->setCut(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::AlignMTB::to_Algorithm() generated
	// ("cv::AlignMTB::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_AlignMTB_to_Algorithm(cv::AlignMTB* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::AlignMTB::to_AlignExposures() generated
	// ("cv::AlignMTB::to_AlignExposures", vec![(pred!(mut, [], []), _)]),
	cv::AlignExposures* cv_AlignMTB_to_AlignExposures(cv::AlignMTB* instance) {
			return dynamic_cast<cv::AlignExposures*>(instance);
	}

	// cv::AlignMTB::delete() generated
	// ("cv::AlignMTB::delete", vec![(pred!(mut, [], []), _)]),
	void cv_AlignMTB_delete(cv::AlignMTB* instance) {
			delete instance;
	}

	// process(InputArrayOfArrays, OutputArray, InputArray)(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:539
	// ("cv::CalibrateCRF::process", vec![(pred!(mut, ["src", "dst", "times"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_CalibrateCRF_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR(cv::CalibrateCRF* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* times, ResultVoid* ocvrs_return) {
		try {
			instance->process(*src, *dst, *times);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CalibrateCRF::to_CalibrateDebevec() generated
	// ("cv::CalibrateCRF::to_CalibrateDebevec", vec![(pred!(mut, [], []), _)]),
	cv::CalibrateDebevec* cv_CalibrateCRF_to_CalibrateDebevec(cv::CalibrateCRF* instance) {
			return dynamic_cast<cv::CalibrateDebevec*>(instance);
	}

	// cv::CalibrateCRF::to_CalibrateRobertson() generated
	// ("cv::CalibrateCRF::to_CalibrateRobertson", vec![(pred!(mut, [], []), _)]),
	cv::CalibrateRobertson* cv_CalibrateCRF_to_CalibrateRobertson(cv::CalibrateCRF* instance) {
			return dynamic_cast<cv::CalibrateRobertson*>(instance);
	}

	// cv::CalibrateCRF::to_Algorithm() generated
	// ("cv::CalibrateCRF::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_CalibrateCRF_to_Algorithm(cv::CalibrateCRF* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::CalibrateCRF::delete() generated
	// ("cv::CalibrateCRF::delete", vec![(pred!(mut, [], []), _)]),
	void cv_CalibrateCRF_delete(cv::CalibrateCRF* instance) {
			delete instance;
	}

	// getLambda()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:551
	// ("cv::CalibrateDebevec::getLambda", vec![(pred!(const, [], []), _)]),
	void cv_CalibrateDebevec_getLambda_const(const cv::CalibrateDebevec* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getLambda();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setLambda(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:552
	// ("cv::CalibrateDebevec::setLambda", vec![(pred!(mut, ["lambda"], ["float"]), _)]),
	void cv_CalibrateDebevec_setLambda_float(cv::CalibrateDebevec* instance, float lambda, ResultVoid* ocvrs_return) {
		try {
			instance->setLambda(lambda);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSamples()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:554
	// ("cv::CalibrateDebevec::getSamples", vec![(pred!(const, [], []), _)]),
	void cv_CalibrateDebevec_getSamples_const(const cv::CalibrateDebevec* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSamples();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSamples(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:555
	// ("cv::CalibrateDebevec::setSamples", vec![(pred!(mut, ["samples"], ["int"]), _)]),
	void cv_CalibrateDebevec_setSamples_int(cv::CalibrateDebevec* instance, int samples, ResultVoid* ocvrs_return) {
		try {
			instance->setSamples(samples);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRandom()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:557
	// ("cv::CalibrateDebevec::getRandom", vec![(pred!(const, [], []), _)]),
	void cv_CalibrateDebevec_getRandom_const(const cv::CalibrateDebevec* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getRandom();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRandom(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:558
	// ("cv::CalibrateDebevec::setRandom", vec![(pred!(mut, ["random"], ["bool"]), _)]),
	void cv_CalibrateDebevec_setRandom_bool(cv::CalibrateDebevec* instance, bool random, ResultVoid* ocvrs_return) {
		try {
			instance->setRandom(random);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CalibrateDebevec::to_Algorithm() generated
	// ("cv::CalibrateDebevec::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_CalibrateDebevec_to_Algorithm(cv::CalibrateDebevec* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::CalibrateDebevec::to_CalibrateCRF() generated
	// ("cv::CalibrateDebevec::to_CalibrateCRF", vec![(pred!(mut, [], []), _)]),
	cv::CalibrateCRF* cv_CalibrateDebevec_to_CalibrateCRF(cv::CalibrateDebevec* instance) {
			return dynamic_cast<cv::CalibrateCRF*>(instance);
	}

	// cv::CalibrateDebevec::delete() generated
	// ("cv::CalibrateDebevec::delete", vec![(pred!(mut, [], []), _)]),
	void cv_CalibrateDebevec_delete(cv::CalibrateDebevec* instance) {
			delete instance;
	}

	// getMaxIter()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:579
	// ("cv::CalibrateRobertson::getMaxIter", vec![(pred!(const, [], []), _)]),
	void cv_CalibrateRobertson_getMaxIter_const(const cv::CalibrateRobertson* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxIter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxIter(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:580
	// ("cv::CalibrateRobertson::setMaxIter", vec![(pred!(mut, ["max_iter"], ["int"]), _)]),
	void cv_CalibrateRobertson_setMaxIter_int(cv::CalibrateRobertson* instance, int max_iter, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxIter(max_iter);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:582
	// ("cv::CalibrateRobertson::getThreshold", vec![(pred!(const, [], []), _)]),
	void cv_CalibrateRobertson_getThreshold_const(const cv::CalibrateRobertson* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:583
	// ("cv::CalibrateRobertson::setThreshold", vec![(pred!(mut, ["threshold"], ["float"]), _)]),
	void cv_CalibrateRobertson_setThreshold_float(cv::CalibrateRobertson* instance, float threshold, ResultVoid* ocvrs_return) {
		try {
			instance->setThreshold(threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRadiance()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:585
	// ("cv::CalibrateRobertson::getRadiance", vec![(pred!(const, [], []), _)]),
	void cv_CalibrateRobertson_getRadiance_const(const cv::CalibrateRobertson* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getRadiance();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CalibrateRobertson::to_Algorithm() generated
	// ("cv::CalibrateRobertson::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_CalibrateRobertson_to_Algorithm(cv::CalibrateRobertson* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::CalibrateRobertson::to_CalibrateCRF() generated
	// ("cv::CalibrateRobertson::to_CalibrateCRF", vec![(pred!(mut, [], []), _)]),
	cv::CalibrateCRF* cv_CalibrateRobertson_to_CalibrateCRF(cv::CalibrateRobertson* instance) {
			return dynamic_cast<cv::CalibrateCRF*>(instance);
	}

	// cv::CalibrateRobertson::delete() generated
	// ("cv::CalibrateRobertson::delete", vec![(pred!(mut, [], []), _)]),
	void cv_CalibrateRobertson_delete(cv::CalibrateRobertson* instance) {
			delete instance;
	}

	// process(InputArrayOfArrays, OutputArray, InputArray, InputArray)(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:620
	// ("cv::MergeDebevec::process", vec![(pred!(mut, ["src", "dst", "times", "response"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_MergeDebevec_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(cv::MergeDebevec* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* times, const cv::_InputArray* response, ResultVoid* ocvrs_return) {
		try {
			instance->process(*src, *dst, *times, *response);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// process(InputArrayOfArrays, OutputArray, InputArray)(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:622
	// ("cv::MergeDebevec::process", vec![(pred!(mut, ["src", "dst", "times"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_MergeDebevec_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR(cv::MergeDebevec* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* times, ResultVoid* ocvrs_return) {
		try {
			instance->process(*src, *dst, *times);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::MergeDebevec::to_Algorithm() generated
	// ("cv::MergeDebevec::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_MergeDebevec_to_Algorithm(cv::MergeDebevec* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::MergeDebevec::to_MergeExposures() generated
	// ("cv::MergeDebevec::to_MergeExposures", vec![(pred!(mut, [], []), _)]),
	cv::MergeExposures* cv_MergeDebevec_to_MergeExposures(cv::MergeDebevec* instance) {
			return dynamic_cast<cv::MergeExposures*>(instance);
	}

	// cv::MergeDebevec::delete() generated
	// ("cv::MergeDebevec::delete", vec![(pred!(mut, [], []), _)]),
	void cv_MergeDebevec_delete(cv::MergeDebevec* instance) {
			delete instance;
	}

	// process(InputArrayOfArrays, OutputArray, InputArray, InputArray)(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:608
	// ("cv::MergeExposures::process", vec![(pred!(mut, ["src", "dst", "times", "response"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_MergeExposures_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(cv::MergeExposures* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* times, const cv::_InputArray* response, ResultVoid* ocvrs_return) {
		try {
			instance->process(*src, *dst, *times, *response);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::MergeExposures::to_MergeDebevec() generated
	// ("cv::MergeExposures::to_MergeDebevec", vec![(pred!(mut, [], []), _)]),
	cv::MergeDebevec* cv_MergeExposures_to_MergeDebevec(cv::MergeExposures* instance) {
			return dynamic_cast<cv::MergeDebevec*>(instance);
	}

	// cv::MergeExposures::to_MergeMertens() generated
	// ("cv::MergeExposures::to_MergeMertens", vec![(pred!(mut, [], []), _)]),
	cv::MergeMertens* cv_MergeExposures_to_MergeMertens(cv::MergeExposures* instance) {
			return dynamic_cast<cv::MergeMertens*>(instance);
	}

	// cv::MergeExposures::to_MergeRobertson() generated
	// ("cv::MergeExposures::to_MergeRobertson", vec![(pred!(mut, [], []), _)]),
	cv::MergeRobertson* cv_MergeExposures_to_MergeRobertson(cv::MergeExposures* instance) {
			return dynamic_cast<cv::MergeRobertson*>(instance);
	}

	// cv::MergeExposures::to_Algorithm() generated
	// ("cv::MergeExposures::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_MergeExposures_to_Algorithm(cv::MergeExposures* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::MergeExposures::delete() generated
	// ("cv::MergeExposures::delete", vec![(pred!(mut, [], []), _)]),
	void cv_MergeExposures_delete(cv::MergeExposures* instance) {
			delete instance;
	}

	// process(InputArrayOfArrays, OutputArray, InputArray, InputArray)(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:643
	// ("cv::MergeMertens::process", vec![(pred!(mut, ["src", "dst", "times", "response"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_MergeMertens_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(cv::MergeMertens* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* times, const cv::_InputArray* response, ResultVoid* ocvrs_return) {
		try {
			instance->process(*src, *dst, *times, *response);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// process(InputArrayOfArrays, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:650
	// ("cv::MergeMertens::process", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_MergeMertens_process_const__InputArrayR_const__OutputArrayR(cv::MergeMertens* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->process(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getContrastWeight()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:652
	// ("cv::MergeMertens::getContrastWeight", vec![(pred!(const, [], []), _)]),
	void cv_MergeMertens_getContrastWeight_const(const cv::MergeMertens* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getContrastWeight();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setContrastWeight(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:653
	// ("cv::MergeMertens::setContrastWeight", vec![(pred!(mut, ["contrast_weiht"], ["float"]), _)]),
	void cv_MergeMertens_setContrastWeight_float(cv::MergeMertens* instance, float contrast_weiht, ResultVoid* ocvrs_return) {
		try {
			instance->setContrastWeight(contrast_weiht);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSaturationWeight()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:655
	// ("cv::MergeMertens::getSaturationWeight", vec![(pred!(const, [], []), _)]),
	void cv_MergeMertens_getSaturationWeight_const(const cv::MergeMertens* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSaturationWeight();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSaturationWeight(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:656
	// ("cv::MergeMertens::setSaturationWeight", vec![(pred!(mut, ["saturation_weight"], ["float"]), _)]),
	void cv_MergeMertens_setSaturationWeight_float(cv::MergeMertens* instance, float saturation_weight, ResultVoid* ocvrs_return) {
		try {
			instance->setSaturationWeight(saturation_weight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getExposureWeight()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:658
	// ("cv::MergeMertens::getExposureWeight", vec![(pred!(const, [], []), _)]),
	void cv_MergeMertens_getExposureWeight_const(const cv::MergeMertens* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getExposureWeight();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setExposureWeight(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:659
	// ("cv::MergeMertens::setExposureWeight", vec![(pred!(mut, ["exposure_weight"], ["float"]), _)]),
	void cv_MergeMertens_setExposureWeight_float(cv::MergeMertens* instance, float exposure_weight, ResultVoid* ocvrs_return) {
		try {
			instance->setExposureWeight(exposure_weight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::MergeMertens::to_Algorithm() generated
	// ("cv::MergeMertens::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_MergeMertens_to_Algorithm(cv::MergeMertens* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::MergeMertens::to_MergeExposures() generated
	// ("cv::MergeMertens::to_MergeExposures", vec![(pred!(mut, [], []), _)]),
	cv::MergeExposures* cv_MergeMertens_to_MergeExposures(cv::MergeMertens* instance) {
			return dynamic_cast<cv::MergeExposures*>(instance);
	}

	// cv::MergeMertens::delete() generated
	// ("cv::MergeMertens::delete", vec![(pred!(mut, [], []), _)]),
	void cv_MergeMertens_delete(cv::MergeMertens* instance) {
			delete instance;
	}

	// process(InputArrayOfArrays, OutputArray, InputArray, InputArray)(InputArray, OutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:679
	// ("cv::MergeRobertson::process", vec![(pred!(mut, ["src", "dst", "times", "response"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_MergeRobertson_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(cv::MergeRobertson* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* times, const cv::_InputArray* response, ResultVoid* ocvrs_return) {
		try {
			instance->process(*src, *dst, *times, *response);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// process(InputArrayOfArrays, OutputArray, InputArray)(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:681
	// ("cv::MergeRobertson::process", vec![(pred!(mut, ["src", "dst", "times"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_MergeRobertson_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR(cv::MergeRobertson* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* times, ResultVoid* ocvrs_return) {
		try {
			instance->process(*src, *dst, *times);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::MergeRobertson::to_Algorithm() generated
	// ("cv::MergeRobertson::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_MergeRobertson_to_Algorithm(cv::MergeRobertson* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::MergeRobertson::to_MergeExposures() generated
	// ("cv::MergeRobertson::to_MergeExposures", vec![(pred!(mut, [], []), _)]),
	cv::MergeExposures* cv_MergeRobertson_to_MergeExposures(cv::MergeRobertson* instance) {
			return dynamic_cast<cv::MergeExposures*>(instance);
	}

	// cv::MergeRobertson::delete() generated
	// ("cv::MergeRobertson::delete", vec![(pred!(mut, [], []), _)]),
	void cv_MergeRobertson_delete(cv::MergeRobertson* instance) {
			delete instance;
	}

	// process(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:343
	// ("cv::Tonemap::process", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_Tonemap_process_const__InputArrayR_const__OutputArrayR(cv::Tonemap* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->process(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGamma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:345
	// ("cv::Tonemap::getGamma", vec![(pred!(const, [], []), _)]),
	void cv_Tonemap_getGamma_const(const cv::Tonemap* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getGamma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setGamma(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:346
	// ("cv::Tonemap::setGamma", vec![(pred!(mut, ["gamma"], ["float"]), _)]),
	void cv_Tonemap_setGamma_float(cv::Tonemap* instance, float gamma, ResultVoid* ocvrs_return) {
		try {
			instance->setGamma(gamma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Tonemap::to_TonemapDrago() generated
	// ("cv::Tonemap::to_TonemapDrago", vec![(pred!(mut, [], []), _)]),
	cv::TonemapDrago* cv_Tonemap_to_TonemapDrago(cv::Tonemap* instance) {
			return dynamic_cast<cv::TonemapDrago*>(instance);
	}

	// cv::Tonemap::to_TonemapMantiuk() generated
	// ("cv::Tonemap::to_TonemapMantiuk", vec![(pred!(mut, [], []), _)]),
	cv::TonemapMantiuk* cv_Tonemap_to_TonemapMantiuk(cv::Tonemap* instance) {
			return dynamic_cast<cv::TonemapMantiuk*>(instance);
	}

	// cv::Tonemap::to_TonemapReinhard() generated
	// ("cv::Tonemap::to_TonemapReinhard", vec![(pred!(mut, [], []), _)]),
	cv::TonemapReinhard* cv_Tonemap_to_TonemapReinhard(cv::Tonemap* instance) {
			return dynamic_cast<cv::TonemapReinhard*>(instance);
	}

	// cv::Tonemap::to_Algorithm() generated
	// ("cv::Tonemap::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_Tonemap_to_Algorithm(cv::Tonemap* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::Tonemap::delete() generated
	// ("cv::Tonemap::delete", vec![(pred!(mut, [], []), _)]),
	void cv_Tonemap_delete(cv::Tonemap* instance) {
			delete instance;
	}

	// getSaturation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:371
	// ("cv::TonemapDrago::getSaturation", vec![(pred!(const, [], []), _)]),
	void cv_TonemapDrago_getSaturation_const(const cv::TonemapDrago* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSaturation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSaturation(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:372
	// ("cv::TonemapDrago::setSaturation", vec![(pred!(mut, ["saturation"], ["float"]), _)]),
	void cv_TonemapDrago_setSaturation_float(cv::TonemapDrago* instance, float saturation, ResultVoid* ocvrs_return) {
		try {
			instance->setSaturation(saturation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBias()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:374
	// ("cv::TonemapDrago::getBias", vec![(pred!(const, [], []), _)]),
	void cv_TonemapDrago_getBias_const(const cv::TonemapDrago* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getBias();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBias(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:375
	// ("cv::TonemapDrago::setBias", vec![(pred!(mut, ["bias"], ["float"]), _)]),
	void cv_TonemapDrago_setBias_float(cv::TonemapDrago* instance, float bias, ResultVoid* ocvrs_return) {
		try {
			instance->setBias(bias);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TonemapDrago::to_Algorithm() generated
	// ("cv::TonemapDrago::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_TonemapDrago_to_Algorithm(cv::TonemapDrago* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::TonemapDrago::to_Tonemap() generated
	// ("cv::TonemapDrago::to_Tonemap", vec![(pred!(mut, [], []), _)]),
	cv::Tonemap* cv_TonemapDrago_to_Tonemap(cv::TonemapDrago* instance) {
			return dynamic_cast<cv::Tonemap*>(instance);
	}

	// cv::TonemapDrago::delete() generated
	// ("cv::TonemapDrago::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TonemapDrago_delete(cv::TonemapDrago* instance) {
			delete instance;
	}

	// getScale()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:430
	// ("cv::TonemapMantiuk::getScale", vec![(pred!(const, [], []), _)]),
	void cv_TonemapMantiuk_getScale_const(const cv::TonemapMantiuk* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScale(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:431
	// ("cv::TonemapMantiuk::setScale", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	void cv_TonemapMantiuk_setScale_float(cv::TonemapMantiuk* instance, float scale, ResultVoid* ocvrs_return) {
		try {
			instance->setScale(scale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSaturation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:433
	// ("cv::TonemapMantiuk::getSaturation", vec![(pred!(const, [], []), _)]),
	void cv_TonemapMantiuk_getSaturation_const(const cv::TonemapMantiuk* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSaturation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSaturation(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:434
	// ("cv::TonemapMantiuk::setSaturation", vec![(pred!(mut, ["saturation"], ["float"]), _)]),
	void cv_TonemapMantiuk_setSaturation_float(cv::TonemapMantiuk* instance, float saturation, ResultVoid* ocvrs_return) {
		try {
			instance->setSaturation(saturation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TonemapMantiuk::to_Algorithm() generated
	// ("cv::TonemapMantiuk::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_TonemapMantiuk_to_Algorithm(cv::TonemapMantiuk* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::TonemapMantiuk::to_Tonemap() generated
	// ("cv::TonemapMantiuk::to_Tonemap", vec![(pred!(mut, [], []), _)]),
	cv::Tonemap* cv_TonemapMantiuk_to_Tonemap(cv::TonemapMantiuk* instance) {
			return dynamic_cast<cv::Tonemap*>(instance);
	}

	// cv::TonemapMantiuk::delete() generated
	// ("cv::TonemapMantiuk::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TonemapMantiuk_delete(cv::TonemapMantiuk* instance) {
			delete instance;
	}

	// getIntensity()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:399
	// ("cv::TonemapReinhard::getIntensity", vec![(pred!(const, [], []), _)]),
	void cv_TonemapReinhard_getIntensity_const(const cv::TonemapReinhard* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getIntensity();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setIntensity(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:400
	// ("cv::TonemapReinhard::setIntensity", vec![(pred!(mut, ["intensity"], ["float"]), _)]),
	void cv_TonemapReinhard_setIntensity_float(cv::TonemapReinhard* instance, float intensity, ResultVoid* ocvrs_return) {
		try {
			instance->setIntensity(intensity);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLightAdaptation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:402
	// ("cv::TonemapReinhard::getLightAdaptation", vec![(pred!(const, [], []), _)]),
	void cv_TonemapReinhard_getLightAdaptation_const(const cv::TonemapReinhard* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getLightAdaptation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setLightAdaptation(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:403
	// ("cv::TonemapReinhard::setLightAdaptation", vec![(pred!(mut, ["light_adapt"], ["float"]), _)]),
	void cv_TonemapReinhard_setLightAdaptation_float(cv::TonemapReinhard* instance, float light_adapt, ResultVoid* ocvrs_return) {
		try {
			instance->setLightAdaptation(light_adapt);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getColorAdaptation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:405
	// ("cv::TonemapReinhard::getColorAdaptation", vec![(pred!(const, [], []), _)]),
	void cv_TonemapReinhard_getColorAdaptation_const(const cv::TonemapReinhard* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getColorAdaptation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setColorAdaptation(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/photo.hpp:406
	// ("cv::TonemapReinhard::setColorAdaptation", vec![(pred!(mut, ["color_adapt"], ["float"]), _)]),
	void cv_TonemapReinhard_setColorAdaptation_float(cv::TonemapReinhard* instance, float color_adapt, ResultVoid* ocvrs_return) {
		try {
			instance->setColorAdaptation(color_adapt);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TonemapReinhard::to_Algorithm() generated
	// ("cv::TonemapReinhard::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_TonemapReinhard_to_Algorithm(cv::TonemapReinhard* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::TonemapReinhard::to_Tonemap() generated
	// ("cv::TonemapReinhard::to_Tonemap", vec![(pred!(mut, [], []), _)]),
	cv::Tonemap* cv_TonemapReinhard_to_Tonemap(cv::TonemapReinhard* instance) {
			return dynamic_cast<cv::Tonemap*>(instance);
	}

	// cv::TonemapReinhard::delete() generated
	// ("cv::TonemapReinhard::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TonemapReinhard_delete(cv::TonemapReinhard* instance) {
			delete instance;
	}

}
