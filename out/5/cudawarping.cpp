#include "ocvrs_common.hpp"
#include <opencv2/cudawarping.hpp>
#include "cudawarping_types.hpp"

extern "C" {
	// cv::cuda::buildWarpAffineMaps(TraitClass, Primitive, SimpleClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:162
	// ("cv::cuda::buildWarpAffineMaps", vec![(pred!(mut, ["M", "inverse", "dsize", "xmap", "ymap"], ["cv::Mat", "bool", "cv::Size", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	void cv_cuda_buildWarpAffineMaps_Mat_bool_Size_GpuMatR_GpuMatR(cv::Mat* M, bool inverse, cv::Size* dsize, cv::cuda::GpuMat* xmap, cv::cuda::GpuMat* ymap, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::buildWarpAffineMaps(*M, inverse, *dsize, *xmap, *ymap);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// buildWarpAffineMaps(Mat, bool, Size, GpuMat &, GpuMat &, Stream &)(TraitClass, Primitive, SimpleClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:162
	// ("cv::cuda::buildWarpAffineMaps", vec![(pred!(mut, ["M", "inverse", "dsize", "xmap", "ymap", "stream"], ["cv::Mat", "bool", "cv::Size", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_buildWarpAffineMaps_Mat_bool_Size_GpuMatR_GpuMatR_StreamR(cv::Mat* M, bool inverse, cv::Size* dsize, cv::cuda::GpuMat* xmap, cv::cuda::GpuMat* ymap, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::buildWarpAffineMaps(*M, inverse, *dsize, *xmap, *ymap, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::buildWarpAffineMaps(TraitClass, Primitive, SimpleClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:158
	// ("cv::cuda::buildWarpAffineMaps", vec![(pred!(mut, ["M", "inverse", "dsize", "xmap", "ymap"], ["cv::UMat", "bool", "cv::Size", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	void cv_cuda_buildWarpAffineMaps_UMat_bool_Size_GpuMatR_GpuMatR(cv::UMat* M, bool inverse, cv::Size* dsize, cv::cuda::GpuMat* xmap, cv::cuda::GpuMat* ymap, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::buildWarpAffineMaps(*M, inverse, *dsize, *xmap, *ymap);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// buildWarpAffineMaps(UMat, bool, Size, GpuMat &, GpuMat &, Stream &)(TraitClass, Primitive, SimpleClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:158
	// ("cv::cuda::buildWarpAffineMaps", vec![(pred!(mut, ["M", "inverse", "dsize", "xmap", "ymap", "stream"], ["cv::UMat", "bool", "cv::Size", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_buildWarpAffineMaps_UMat_bool_Size_GpuMatR_GpuMatR_StreamR(cv::UMat* M, bool inverse, cv::Size* dsize, cv::cuda::GpuMat* xmap, cv::cuda::GpuMat* ymap, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::buildWarpAffineMaps(*M, inverse, *dsize, *xmap, *ymap, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::buildWarpAffineMaps(InputArray, Primitive, SimpleClass, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:156
	// ("cv::cuda::buildWarpAffineMaps", vec![(pred!(mut, ["M", "inverse", "dsize", "xmap", "ymap"], ["const cv::_InputArray*", "bool", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_buildWarpAffineMaps_const__InputArrayR_bool_Size_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* M, bool inverse, cv::Size* dsize, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::buildWarpAffineMaps(*M, inverse, *dsize, *xmap, *ymap);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// buildWarpAffineMaps(InputArray, bool, Size, OutputArray, OutputArray, Stream &)(InputArray, Primitive, SimpleClass, OutputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:156
	// ("cv::cuda::buildWarpAffineMaps", vec![(pred!(mut, ["M", "inverse", "dsize", "xmap", "ymap", "stream"], ["const cv::_InputArray*", "bool", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_buildWarpAffineMaps_const__InputArrayR_bool_Size_const__OutputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* M, bool inverse, cv::Size* dsize, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::buildWarpAffineMaps(*M, inverse, *dsize, *xmap, *ymap, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::buildWarpPerspectiveMaps(TraitClass, Primitive, SimpleClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:212
	// ("cv::cuda::buildWarpPerspectiveMaps", vec![(pred!(mut, ["M", "inverse", "dsize", "xmap", "ymap"], ["cv::Mat", "bool", "cv::Size", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	void cv_cuda_buildWarpPerspectiveMaps_Mat_bool_Size_GpuMatR_GpuMatR(cv::Mat* M, bool inverse, cv::Size* dsize, cv::cuda::GpuMat* xmap, cv::cuda::GpuMat* ymap, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::buildWarpPerspectiveMaps(*M, inverse, *dsize, *xmap, *ymap);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// buildWarpPerspectiveMaps(Mat, bool, Size, GpuMat &, GpuMat &, Stream &)(TraitClass, Primitive, SimpleClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:212
	// ("cv::cuda::buildWarpPerspectiveMaps", vec![(pred!(mut, ["M", "inverse", "dsize", "xmap", "ymap", "stream"], ["cv::Mat", "bool", "cv::Size", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_buildWarpPerspectiveMaps_Mat_bool_Size_GpuMatR_GpuMatR_StreamR(cv::Mat* M, bool inverse, cv::Size* dsize, cv::cuda::GpuMat* xmap, cv::cuda::GpuMat* ymap, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::buildWarpPerspectiveMaps(*M, inverse, *dsize, *xmap, *ymap, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::buildWarpPerspectiveMaps(TraitClass, Primitive, SimpleClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:208
	// ("cv::cuda::buildWarpPerspectiveMaps", vec![(pred!(mut, ["M", "inverse", "dsize", "xmap", "ymap"], ["cv::UMat", "bool", "cv::Size", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	void cv_cuda_buildWarpPerspectiveMaps_UMat_bool_Size_GpuMatR_GpuMatR(cv::UMat* M, bool inverse, cv::Size* dsize, cv::cuda::GpuMat* xmap, cv::cuda::GpuMat* ymap, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::buildWarpPerspectiveMaps(*M, inverse, *dsize, *xmap, *ymap);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// buildWarpPerspectiveMaps(UMat, bool, Size, GpuMat &, GpuMat &, Stream &)(TraitClass, Primitive, SimpleClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:208
	// ("cv::cuda::buildWarpPerspectiveMaps", vec![(pred!(mut, ["M", "inverse", "dsize", "xmap", "ymap", "stream"], ["cv::UMat", "bool", "cv::Size", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_buildWarpPerspectiveMaps_UMat_bool_Size_GpuMatR_GpuMatR_StreamR(cv::UMat* M, bool inverse, cv::Size* dsize, cv::cuda::GpuMat* xmap, cv::cuda::GpuMat* ymap, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::buildWarpPerspectiveMaps(*M, inverse, *dsize, *xmap, *ymap, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::buildWarpPerspectiveMaps(InputArray, Primitive, SimpleClass, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:206
	// ("cv::cuda::buildWarpPerspectiveMaps", vec![(pred!(mut, ["M", "inverse", "dsize", "xmap", "ymap"], ["const cv::_InputArray*", "bool", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_buildWarpPerspectiveMaps_const__InputArrayR_bool_Size_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* M, bool inverse, cv::Size* dsize, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::buildWarpPerspectiveMaps(*M, inverse, *dsize, *xmap, *ymap);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// buildWarpPerspectiveMaps(InputArray, bool, Size, OutputArray, OutputArray, Stream &)(InputArray, Primitive, SimpleClass, OutputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:206
	// ("cv::cuda::buildWarpPerspectiveMaps", vec![(pred!(mut, ["M", "inverse", "dsize", "xmap", "ymap", "stream"], ["const cv::_InputArray*", "bool", "cv::Size", "const cv::_OutputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_buildWarpPerspectiveMaps_const__InputArrayR_bool_Size_const__OutputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* M, bool inverse, cv::Size* dsize, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::buildWarpPerspectiveMaps(*M, inverse, *dsize, *xmap, *ymap, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::pyrDown(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:243
	// ("cv::cuda::pyrDown", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_pyrDown_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::pyrDown(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// pyrDown(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:243
	// ("cv::cuda::pyrDown", vec![(pred!(mut, ["src", "dst", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_pyrDown_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::pyrDown(*src, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::pyrUp(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:252
	// ("cv::cuda::pyrUp", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_pyrUp_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::pyrUp(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// pyrUp(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:252
	// ("cv::cuda::pyrUp", vec![(pred!(mut, ["src", "dst", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_pyrUp_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::pyrUp(*src, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::remap(InputArray, OutputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:92
	// ("cv::cuda::remap", vec![(pred!(mut, ["src", "dst", "xmap", "ymap", "interpolation"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
	void cv_cuda_remap_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* xmap, const cv::_InputArray* ymap, int interpolation, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::remap(*src, *dst, *xmap, *ymap, interpolation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// remap(InputArray, OutputArray, InputArray, InputArray, int, int, Scalar, Stream &)(InputArray, OutputArray, InputArray, InputArray, Primitive, Primitive, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:92
	// ("cv::cuda::remap", vec![(pred!(mut, ["src", "dst", "xmap", "ymap", "interpolation", "borderMode", "borderValue", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "cv::Scalar", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_remap_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_int_int_Scalar_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* xmap, const cv::_InputArray* ymap, int interpolation, int borderMode, cv::Scalar* borderValue, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::remap(*src, *dst, *xmap, *ymap, interpolation, borderMode, *borderValue, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::resize(InputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:114
	// ("cv::cuda::resize", vec![(pred!(mut, ["src", "dst", "dsize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size"]), _)]),
	void cv_cuda_resize_const__InputArrayR_const__OutputArrayR_Size(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Size* dsize, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::resize(*src, *dst, *dsize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// resize(InputArray, OutputArray, Size, double, double, int, Stream &)(InputArray, OutputArray, SimpleClass, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:114
	// ("cv::cuda::resize", vec![(pred!(mut, ["src", "dst", "dsize", "fx", "fy", "interpolation", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size", "double", "double", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_resize_const__InputArrayR_const__OutputArrayR_Size_double_double_int_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Size* dsize, double fx, double fy, int interpolation, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::resize(*src, *dst, *dsize, fx, fy, interpolation, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::rotate(InputArray, OutputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:231
	// ("cv::cuda::rotate", vec![(pred!(mut, ["src", "dst", "dsize", "angle"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size", "double"]), _)]),
	void cv_cuda_rotate_const__InputArrayR_const__OutputArrayR_Size_double(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Size* dsize, double angle, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::rotate(*src, *dst, *dsize, angle);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// rotate(InputArray, OutputArray, Size, double, double, double, int, Stream &)(InputArray, OutputArray, SimpleClass, Primitive, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:231
	// ("cv::cuda::rotate", vec![(pred!(mut, ["src", "dst", "dsize", "angle", "xShift", "yShift", "interpolation", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size", "double", "double", "double", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_rotate_const__InputArrayR_const__OutputArrayR_Size_double_double_double_int_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Size* dsize, double angle, double xShift, double yShift, int interpolation, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::rotate(*src, *dst, *dsize, angle, xShift, yShift, interpolation, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::warpAffine(InputArray, OutputArray, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:140
	// ("cv::cuda::warpAffine", vec![(pred!(mut, ["src", "dst", "M", "dsize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Mat", "cv::Size"]), _)]),
	void cv_cuda_warpAffine_const__InputArrayR_const__OutputArrayR_Mat_Size(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Mat* M, cv::Size* dsize, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::warpAffine(*src, *dst, *M, *dsize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpAffine(InputArray, OutputArray, Mat, Size, int, int, Scalar, Stream &)(InputArray, OutputArray, TraitClass, SimpleClass, Primitive, Primitive, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:140
	// ("cv::cuda::warpAffine", vec![(pred!(mut, ["src", "dst", "M", "dsize", "flags", "borderMode", "borderValue", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Mat", "cv::Size", "int", "int", "cv::Scalar", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_warpAffine_const__InputArrayR_const__OutputArrayR_Mat_Size_int_int_Scalar_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Mat* M, cv::Size* dsize, int flags, int borderMode, cv::Scalar* borderValue, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::warpAffine(*src, *dst, *M, *dsize, flags, borderMode, *borderValue, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::warpAffine(InputArray, OutputArray, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:135
	// ("cv::cuda::warpAffine", vec![(pred!(mut, ["src", "dst", "M", "dsize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::UMat", "cv::Size"]), _)]),
	void cv_cuda_warpAffine_const__InputArrayR_const__OutputArrayR_UMat_Size(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::UMat* M, cv::Size* dsize, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::warpAffine(*src, *dst, *M, *dsize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpAffine(InputArray, OutputArray, UMat, Size, int, int, Scalar, Stream &)(InputArray, OutputArray, TraitClass, SimpleClass, Primitive, Primitive, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:135
	// ("cv::cuda::warpAffine", vec![(pred!(mut, ["src", "dst", "M", "dsize", "flags", "borderMode", "borderValue", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::UMat", "cv::Size", "int", "int", "cv::Scalar", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_warpAffine_const__InputArrayR_const__OutputArrayR_UMat_Size_int_int_Scalar_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::UMat* M, cv::Size* dsize, int flags, int borderMode, cv::Scalar* borderValue, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::warpAffine(*src, *dst, *M, *dsize, flags, borderMode, *borderValue, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::warpAffine(InputArray, OutputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:132
	// ("cv::cuda::warpAffine", vec![(pred!(mut, ["src", "dst", "M", "dsize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::Size"]), _)]),
	void cv_cuda_warpAffine_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Size(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* M, cv::Size* dsize, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::warpAffine(*src, *dst, *M, *dsize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpAffine(InputArray, OutputArray, InputArray, Size, int, int, Scalar, Stream &)(InputArray, OutputArray, InputArray, SimpleClass, Primitive, Primitive, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:132
	// ("cv::cuda::warpAffine", vec![(pred!(mut, ["src", "dst", "M", "dsize", "flags", "borderMode", "borderValue", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::Size", "int", "int", "cv::Scalar", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_warpAffine_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Size_int_int_Scalar_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* M, cv::Size* dsize, int flags, int borderMode, cv::Scalar* borderValue, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::warpAffine(*src, *dst, *M, *dsize, flags, borderMode, *borderValue, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::warpPerspective(InputArray, OutputArray, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:190
	// ("cv::cuda::warpPerspective", vec![(pred!(mut, ["src", "dst", "M", "dsize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Mat", "cv::Size"]), _)]),
	void cv_cuda_warpPerspective_const__InputArrayR_const__OutputArrayR_Mat_Size(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Mat* M, cv::Size* dsize, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::warpPerspective(*src, *dst, *M, *dsize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpPerspective(InputArray, OutputArray, Mat, Size, int, int, Scalar, Stream &)(InputArray, OutputArray, TraitClass, SimpleClass, Primitive, Primitive, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:190
	// ("cv::cuda::warpPerspective", vec![(pred!(mut, ["src", "dst", "M", "dsize", "flags", "borderMode", "borderValue", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Mat", "cv::Size", "int", "int", "cv::Scalar", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_warpPerspective_const__InputArrayR_const__OutputArrayR_Mat_Size_int_int_Scalar_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Mat* M, cv::Size* dsize, int flags, int borderMode, cv::Scalar* borderValue, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::warpPerspective(*src, *dst, *M, *dsize, flags, borderMode, *borderValue, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::warpPerspective(InputArray, OutputArray, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:185
	// ("cv::cuda::warpPerspective", vec![(pred!(mut, ["src", "dst", "M", "dsize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::UMat", "cv::Size"]), _)]),
	void cv_cuda_warpPerspective_const__InputArrayR_const__OutputArrayR_UMat_Size(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::UMat* M, cv::Size* dsize, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::warpPerspective(*src, *dst, *M, *dsize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpPerspective(InputArray, OutputArray, UMat, Size, int, int, Scalar, Stream &)(InputArray, OutputArray, TraitClass, SimpleClass, Primitive, Primitive, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:185
	// ("cv::cuda::warpPerspective", vec![(pred!(mut, ["src", "dst", "M", "dsize", "flags", "borderMode", "borderValue", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::UMat", "cv::Size", "int", "int", "cv::Scalar", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_warpPerspective_const__InputArrayR_const__OutputArrayR_UMat_Size_int_int_Scalar_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::UMat* M, cv::Size* dsize, int flags, int borderMode, cv::Scalar* borderValue, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::warpPerspective(*src, *dst, *M, *dsize, flags, borderMode, *borderValue, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::warpPerspective(InputArray, OutputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:182
	// ("cv::cuda::warpPerspective", vec![(pred!(mut, ["src", "dst", "M", "dsize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::Size"]), _)]),
	void cv_cuda_warpPerspective_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Size(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* M, cv::Size* dsize, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::warpPerspective(*src, *dst, *M, *dsize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpPerspective(InputArray, OutputArray, InputArray, Size, int, int, Scalar, Stream &)(InputArray, OutputArray, InputArray, SimpleClass, Primitive, Primitive, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudawarping.hpp:182
	// ("cv::cuda::warpPerspective", vec![(pred!(mut, ["src", "dst", "M", "dsize", "flags", "borderMode", "borderValue", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::Size", "int", "int", "cv::Scalar", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_warpPerspective_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Size_int_int_Scalar_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* M, cv::Size* dsize, int flags, int borderMode, cv::Scalar* borderValue, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::warpPerspective(*src, *dst, *M, *dsize, flags, borderMode, *borderValue, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}
