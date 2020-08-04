#include "ocvrs_common.hpp"
#include <opencv2/cudaarithm.hpp>
#include "cudaarithm_types.hpp"

extern "C" {
	Result<cv::Scalar> cv_cuda_absSum_const__InputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_InputArray* mask) {
		try {
			cv::Scalar ret = cv::cuda::absSum(*src, *mask);
			return Ok<cv::Scalar>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	Result_void cv_cuda_abs_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::cuda::Stream* stream) {
		try {
			cv::cuda::abs(*src, *dst, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_absdiff_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, cv::cuda::Stream* stream) {
		try {
			cv::cuda::absdiff(*src1, *src2, *dst, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_addWeighted_const__InputArrayR_double_const__InputArrayR_double_double_const__OutputArrayR_int_StreamR(const cv::_InputArray* src1, double alpha, const cv::_InputArray* src2, double beta, double gamma, const cv::_OutputArray* dst, int dtype, cv::cuda::Stream* stream) {
		try {
			cv::cuda::addWeighted(*src1, alpha, *src2, beta, gamma, *dst, dtype, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_add_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_int_StreamR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, const cv::_InputArray* mask, int dtype, cv::cuda::Stream* stream) {
		try {
			cv::cuda::add(*src1, *src2, *dst, *mask, dtype, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_bitwise_and_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, const cv::_InputArray* mask, cv::cuda::Stream* stream) {
		try {
			cv::cuda::bitwise_and(*src1, *src2, *dst, *mask, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_bitwise_not_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* mask, cv::cuda::Stream* stream) {
		try {
			cv::cuda::bitwise_not(*src, *dst, *mask, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_bitwise_or_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, const cv::_InputArray* mask, cv::cuda::Stream* stream) {
		try {
			cv::cuda::bitwise_or(*src1, *src2, *dst, *mask, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_bitwise_xor_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, const cv::_InputArray* mask, cv::cuda::Stream* stream) {
		try {
			cv::cuda::bitwise_xor(*src1, *src2, *dst, *mask, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_calcAbsSum_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* mask, cv::cuda::Stream* stream) {
		try {
			cv::cuda::calcAbsSum(*src, *dst, *mask, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_calcNormDiff_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_StreamR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, int normType, cv::cuda::Stream* stream) {
		try {
			cv::cuda::calcNormDiff(*src1, *src2, *dst, normType, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_calcNorm_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, int normType, const cv::_InputArray* mask, cv::cuda::Stream* stream) {
		try {
			cv::cuda::calcNorm(*src, *dst, normType, *mask, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_calcSqrSum_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* mask, cv::cuda::Stream* stream) {
		try {
			cv::cuda::calcSqrSum(*src, *dst, *mask, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_calcSum_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* mask, cv::cuda::Stream* stream) {
		try {
			cv::cuda::calcSum(*src, *dst, *mask, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_cartToPolar_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_StreamR(const cv::_InputArray* x, const cv::_InputArray* y, const cv::_OutputArray* magnitude, const cv::_OutputArray* angle, bool angleInDegrees, cv::cuda::Stream* stream) {
		try {
			cv::cuda::cartToPolar(*x, *y, *magnitude, *angle, angleInDegrees, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_compare_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_StreamR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, int cmpop, cv::cuda::Stream* stream) {
		try {
			cv::cuda::compare(*src1, *src2, *dst, cmpop, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_copyMakeBorder_const__InputArrayR_const__OutputArrayR_int_int_int_int_int_Scalar_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, int top, int bottom, int left, int right, int borderType, cv::Scalar* value, cv::cuda::Stream* stream) {
		try {
			cv::cuda::copyMakeBorder(*src, *dst, top, bottom, left, right, borderType, *value, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_countNonZero_const__InputArrayR(const cv::_InputArray* src) {
		try {
			int ret = cv::cuda::countNonZero(*src);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_countNonZero_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::cuda::Stream* stream) {
		try {
			cv::cuda::countNonZero(*src, *dst, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::cuda::Convolution>*> cv_cuda_createConvolution_Size(cv::Size* user_block_size) {
		try {
			cv::Ptr<cv::cuda::Convolution> ret = cv::cuda::createConvolution(*user_block_size);
			return Ok(new cv::Ptr<cv::cuda::Convolution>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::Convolution>*>))
	}
	
	Result<cv::Ptr<cv::cuda::DFT>*> cv_cuda_createDFT_Size_int(cv::Size* dft_size, int flags) {
		try {
			cv::Ptr<cv::cuda::DFT> ret = cv::cuda::createDFT(*dft_size, flags);
			return Ok(new cv::Ptr<cv::cuda::DFT>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::DFT>*>))
	}
	
	Result<cv::Ptr<cv::cuda::LookUpTable>*> cv_cuda_createLookUpTable_const__InputArrayR(const cv::_InputArray* lut) {
		try {
			cv::Ptr<cv::cuda::LookUpTable> ret = cv::cuda::createLookUpTable(*lut);
			return Ok(new cv::Ptr<cv::cuda::LookUpTable>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::LookUpTable>*>))
	}
	
	Result_void cv_cuda_dft_const__InputArrayR_const__OutputArrayR_Size_int_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Size* dft_size, int flags, cv::cuda::Stream* stream) {
		try {
			cv::cuda::dft(*src, *dst, *dft_size, flags, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_divide_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_int_StreamR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, double scale, int dtype, cv::cuda::Stream* stream) {
		try {
			cv::cuda::divide(*src1, *src2, *dst, scale, dtype, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_exp_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::cuda::Stream* stream) {
		try {
			cv::cuda::exp(*src, *dst, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_findMinMaxLoc_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* minMaxVals, const cv::_OutputArray* loc, const cv::_InputArray* mask, cv::cuda::Stream* stream) {
		try {
			cv::cuda::findMinMaxLoc(*src, *minMaxVals, *loc, *mask, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_findMinMax_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* mask, cv::cuda::Stream* stream) {
		try {
			cv::cuda::findMinMax(*src, *dst, *mask, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_flip_const__InputArrayR_const__OutputArrayR_int_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, int flipCode, cv::cuda::Stream* stream) {
		try {
			cv::cuda::flip(*src, *dst, flipCode, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_gemm_const__InputArrayR_const__InputArrayR_double_const__InputArrayR_double_const__OutputArrayR_int_StreamR(const cv::_InputArray* src1, const cv::_InputArray* src2, double alpha, const cv::_InputArray* src3, double beta, const cv::_OutputArray* dst, int flags, cv::cuda::Stream* stream) {
		try {
			cv::cuda::gemm(*src1, *src2, alpha, *src3, beta, *dst, flags, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_integral_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* sum, cv::cuda::Stream* stream) {
		try {
			cv::cuda::integral(*src, *sum, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_log_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::cuda::Stream* stream) {
		try {
			cv::cuda::log(*src, *dst, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_lshift_const__InputArrayR_Scalar__int__const__OutputArrayR_StreamR(const cv::_InputArray* src, cv::Scalar_<int>* val, const cv::_OutputArray* dst, cv::cuda::Stream* stream) {
		try {
			cv::cuda::lshift(*src, *val, *dst, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_magnitudeSqr_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* x, const cv::_InputArray* y, const cv::_OutputArray* magnitude, cv::cuda::Stream* stream) {
		try {
			cv::cuda::magnitudeSqr(*x, *y, *magnitude, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_magnitudeSqr_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* xy, const cv::_OutputArray* magnitude, cv::cuda::Stream* stream) {
		try {
			cv::cuda::magnitudeSqr(*xy, *magnitude, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_magnitude_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* x, const cv::_InputArray* y, const cv::_OutputArray* magnitude, cv::cuda::Stream* stream) {
		try {
			cv::cuda::magnitude(*x, *y, *magnitude, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_magnitude_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* xy, const cv::_OutputArray* magnitude, cv::cuda::Stream* stream) {
		try {
			cv::cuda::magnitude(*xy, *magnitude, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_max_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, cv::cuda::Stream* stream) {
		try {
			cv::cuda::max(*src1, *src2, *dst, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_meanStdDev_const__InputArrayR_ScalarR_ScalarR(const cv::_InputArray* mtx, cv::Scalar* mean, cv::Scalar* stddev) {
		try {
			cv::cuda::meanStdDev(*mtx, *mean, *stddev);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_meanStdDev_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* mtx, const cv::_OutputArray* dst, cv::cuda::Stream* stream) {
		try {
			cv::cuda::meanStdDev(*mtx, *dst, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_merge_const_GpuMatX_size_t_const__OutputArrayR_StreamR(const cv::cuda::GpuMat* src, size_t n, const cv::_OutputArray* dst, cv::cuda::Stream* stream) {
		try {
			cv::cuda::merge(src, n, *dst, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_merge_const_vector_GpuMat_R_const__OutputArrayR_StreamR(const std::vector<cv::cuda::GpuMat>* src, const cv::_OutputArray* dst, cv::cuda::Stream* stream) {
		try {
			cv::cuda::merge(*src, *dst, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_minMaxLoc_const__InputArrayR_doubleX_doubleX_PointX_PointX_const__InputArrayR(const cv::_InputArray* src, double* minVal, double* maxVal, cv::Point* minLoc, cv::Point* maxLoc, const cv::_InputArray* mask) {
		try {
			cv::cuda::minMaxLoc(*src, minVal, maxVal, minLoc, maxLoc, *mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_minMax_const__InputArrayR_doubleX_doubleX_const__InputArrayR(const cv::_InputArray* src, double* minVal, double* maxVal, const cv::_InputArray* mask) {
		try {
			cv::cuda::minMax(*src, minVal, maxVal, *mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_min_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, cv::cuda::Stream* stream) {
		try {
			cv::cuda::min(*src1, *src2, *dst, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_mulAndScaleSpectrums_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_float_bool_StreamR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, int flags, float scale, bool conjB, cv::cuda::Stream* stream) {
		try {
			cv::cuda::mulAndScaleSpectrums(*src1, *src2, *dst, flags, scale, conjB, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_mulSpectrums_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_bool_StreamR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, int flags, bool conjB, cv::cuda::Stream* stream) {
		try {
			cv::cuda::mulSpectrums(*src1, *src2, *dst, flags, conjB, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_multiply_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_int_StreamR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, double scale, int dtype, cv::cuda::Stream* stream) {
		try {
			cv::cuda::multiply(*src1, *src2, *dst, scale, dtype, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_cuda_norm_const__InputArrayR_const__InputArrayR_int(const cv::_InputArray* src1, const cv::_InputArray* src2, int normType) {
		try {
			double ret = cv::cuda::norm(*src1, *src2, normType);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<double> cv_cuda_norm_const__InputArrayR_int_const__InputArrayR(const cv::_InputArray* src1, int normType, const cv::_InputArray* mask) {
		try {
			double ret = cv::cuda::norm(*src1, normType, *mask);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_cuda_normalize_const__InputArrayR_const__OutputArrayR_double_double_int_int_const__InputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, double alpha, double beta, int norm_type, int dtype, const cv::_InputArray* mask, cv::cuda::Stream* stream) {
		try {
			cv::cuda::normalize(*src, *dst, alpha, beta, norm_type, dtype, *mask, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_phase_const__InputArrayR_const__InputArrayR_const__OutputArrayR_bool_StreamR(const cv::_InputArray* x, const cv::_InputArray* y, const cv::_OutputArray* angle, bool angleInDegrees, cv::cuda::Stream* stream) {
		try {
			cv::cuda::phase(*x, *y, *angle, angleInDegrees, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_polarToCart_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_StreamR(const cv::_InputArray* magnitude, const cv::_InputArray* angle, const cv::_OutputArray* x, const cv::_OutputArray* y, bool angleInDegrees, cv::cuda::Stream* stream) {
		try {
			cv::cuda::polarToCart(*magnitude, *angle, *x, *y, angleInDegrees, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_pow_const__InputArrayR_double_const__OutputArrayR_StreamR(const cv::_InputArray* src, double power, const cv::_OutputArray* dst, cv::cuda::Stream* stream) {
		try {
			cv::cuda::pow(*src, power, *dst, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_rectStdDev_const__InputArrayR_const__InputArrayR_const__OutputArrayR_Rect_StreamR(const cv::_InputArray* src, const cv::_InputArray* sqr, const cv::_OutputArray* dst, cv::Rect* rect, cv::cuda::Stream* stream) {
		try {
			cv::cuda::rectStdDev(*src, *sqr, *dst, *rect, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_reduce_const__InputArrayR_const__OutputArrayR_int_int_int_StreamR(const cv::_InputArray* mtx, const cv::_OutputArray* vec, int dim, int reduceOp, int dtype, cv::cuda::Stream* stream) {
		try {
			cv::cuda::reduce(*mtx, *vec, dim, reduceOp, dtype, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_rshift_const__InputArrayR_Scalar__int__const__OutputArrayR_StreamR(const cv::_InputArray* src, cv::Scalar_<int>* val, const cv::_OutputArray* dst, cv::cuda::Stream* stream) {
		try {
			cv::cuda::rshift(*src, *val, *dst, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_split_const__InputArrayR_GpuMatX_StreamR(const cv::_InputArray* src, cv::cuda::GpuMat* dst, cv::cuda::Stream* stream) {
		try {
			cv::cuda::split(*src, dst, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_split_const__InputArrayR_vector_GpuMat_R_StreamR(const cv::_InputArray* src, std::vector<cv::cuda::GpuMat>* dst, cv::cuda::Stream* stream) {
		try {
			cv::cuda::split(*src, *dst, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_sqrIntegral_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* sqsum, cv::cuda::Stream* stream) {
		try {
			cv::cuda::sqrIntegral(*src, *sqsum, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Scalar> cv_cuda_sqrSum_const__InputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_InputArray* mask) {
		try {
			cv::Scalar ret = cv::cuda::sqrSum(*src, *mask);
			return Ok<cv::Scalar>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	Result_void cv_cuda_sqr_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::cuda::Stream* stream) {
		try {
			cv::cuda::sqr(*src, *dst, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_sqrt_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::cuda::Stream* stream) {
		try {
			cv::cuda::sqrt(*src, *dst, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_subtract_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_int_StreamR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, const cv::_InputArray* mask, int dtype, cv::cuda::Stream* stream) {
		try {
			cv::cuda::subtract(*src1, *src2, *dst, *mask, dtype, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Scalar> cv_cuda_sum_const__InputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_InputArray* mask) {
		try {
			cv::Scalar ret = cv::cuda::sum(*src, *mask);
			return Ok<cv::Scalar>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	Result<double> cv_cuda_threshold_const__InputArrayR_const__OutputArrayR_double_double_int_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, double thresh, double maxval, int type, cv::cuda::Stream* stream) {
		try {
			double ret = cv::cuda::threshold(*src, *dst, thresh, maxval, type, *stream);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_cuda_transpose_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src1, const cv::_OutputArray* dst, cv::cuda::Stream* stream) {
		try {
			cv::cuda::transpose(*src1, *dst, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_Convolution_convolve_const__InputArrayR_const__InputArrayR_const__OutputArrayR_bool_StreamR(cv::cuda::Convolution* instance, const cv::_InputArray* image, const cv::_InputArray* templ, const cv::_OutputArray* result, bool ccorr, cv::cuda::Stream* stream) {
		try {
			instance->convolve(*image, *templ, *result, ccorr, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_DFT_compute_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::DFT* instance, const cv::_InputArray* image, const cv::_OutputArray* result, cv::cuda::Stream* stream) {
		try {
			instance->compute(*image, *result, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_LookUpTable_transform_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::LookUpTable* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, cv::cuda::Stream* stream) {
		try {
			instance->transform(*src, *dst, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
