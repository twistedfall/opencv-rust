#include "common.hpp"
#include <opencv2/cudawarping.hpp>
#include "cudawarping_types.hpp"

extern "C" {
	Result_void cv_cuda_buildWarpAffineMaps_const__InputArrayR_bool_Size_const__OutputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* M, bool inverse, const cv::Size* dsize, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap, cv::cuda::Stream* stream) {
		try {
			cv::cuda::buildWarpAffineMaps(*M, inverse, *dsize, *xmap, *ymap, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_buildWarpPerspectiveMaps_const__InputArrayR_bool_Size_const__OutputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* M, bool inverse, const cv::Size* dsize, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap, cv::cuda::Stream* stream) {
		try {
			cv::cuda::buildWarpPerspectiveMaps(*M, inverse, *dsize, *xmap, *ymap, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_pyrDown_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::cuda::Stream* stream) {
		try {
			cv::cuda::pyrDown(*src, *dst, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_pyrUp_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::cuda::Stream* stream) {
		try {
			cv::cuda::pyrUp(*src, *dst, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_remap_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_int_int_Scalar_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* xmap, const cv::_InputArray* ymap, int interpolation, int borderMode, const cv::Scalar* borderValue, cv::cuda::Stream* stream) {
		try {
			cv::cuda::remap(*src, *dst, *xmap, *ymap, interpolation, borderMode, *borderValue, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_resize_const__InputArrayR_const__OutputArrayR_Size_double_double_int_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::Size* dsize, double fx, double fy, int interpolation, cv::cuda::Stream* stream) {
		try {
			cv::cuda::resize(*src, *dst, *dsize, fx, fy, interpolation, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_rotate_const__InputArrayR_const__OutputArrayR_Size_double_double_double_int_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::Size* dsize, double angle, double xShift, double yShift, int interpolation, cv::cuda::Stream* stream) {
		try {
			cv::cuda::rotate(*src, *dst, *dsize, angle, xShift, yShift, interpolation, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_warpAffine_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Size_int_int_Scalar_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* M, const cv::Size* dsize, int flags, int borderMode, const cv::Scalar* borderValue, cv::cuda::Stream* stream) {
		try {
			cv::cuda::warpAffine(*src, *dst, *M, *dsize, flags, borderMode, *borderValue, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_warpPerspective_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Size_int_int_Scalar_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* M, const cv::Size* dsize, int flags, int borderMode, const cv::Scalar* borderValue, cv::cuda::Stream* stream) {
		try {
			cv::cuda::warpPerspective(*src, *dst, *M, *dsize, flags, borderMode, *borderValue, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
