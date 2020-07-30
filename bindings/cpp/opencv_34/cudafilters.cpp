#include "common.hpp"
#include <opencv2/cudafilters.hpp>
#include "cudafilters_types.hpp"

extern "C" {
	Result<cv::Ptr<cv::cuda::Filter>*> cv_cuda_createBoxFilter_int_int_Size_Point_int_Scalar(int srcType, int dstType, cv::Size* ksize, cv::Point* anchor, int borderMode, cv::Scalar* borderVal) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createBoxFilter(srcType, dstType, *ksize, *anchor, borderMode, *borderVal);
			return Ok(new cv::Ptr<cv::cuda::Filter>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::Filter>*>))
	}
	
	Result<cv::Ptr<cv::cuda::Filter>*> cv_cuda_createBoxMaxFilter_int_Size_Point_int_Scalar(int srcType, cv::Size* ksize, cv::Point* anchor, int borderMode, cv::Scalar* borderVal) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createBoxMaxFilter(srcType, *ksize, *anchor, borderMode, *borderVal);
			return Ok(new cv::Ptr<cv::cuda::Filter>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::Filter>*>))
	}
	
	Result<cv::Ptr<cv::cuda::Filter>*> cv_cuda_createBoxMinFilter_int_Size_Point_int_Scalar(int srcType, cv::Size* ksize, cv::Point* anchor, int borderMode, cv::Scalar* borderVal) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createBoxMinFilter(srcType, *ksize, *anchor, borderMode, *borderVal);
			return Ok(new cv::Ptr<cv::cuda::Filter>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::Filter>*>))
	}
	
	Result<cv::Ptr<cv::cuda::Filter>*> cv_cuda_createColumnSumFilter_int_int_int_int_int_Scalar(int srcType, int dstType, int ksize, int anchor, int borderMode, cv::Scalar* borderVal) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createColumnSumFilter(srcType, dstType, ksize, anchor, borderMode, *borderVal);
			return Ok(new cv::Ptr<cv::cuda::Filter>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::Filter>*>))
	}
	
	Result<cv::Ptr<cv::cuda::Filter>*> cv_cuda_createDerivFilter_int_int_int_int_int_bool_double_int_int(int srcType, int dstType, int dx, int dy, int ksize, bool normalize, double scale, int rowBorderMode, int columnBorderMode) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createDerivFilter(srcType, dstType, dx, dy, ksize, normalize, scale, rowBorderMode, columnBorderMode);
			return Ok(new cv::Ptr<cv::cuda::Filter>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::Filter>*>))
	}
	
	Result<cv::Ptr<cv::cuda::Filter>*> cv_cuda_createGaussianFilter_int_int_Size_double_double_int_int(int srcType, int dstType, cv::Size* ksize, double sigma1, double sigma2, int rowBorderMode, int columnBorderMode) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createGaussianFilter(srcType, dstType, *ksize, sigma1, sigma2, rowBorderMode, columnBorderMode);
			return Ok(new cv::Ptr<cv::cuda::Filter>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::Filter>*>))
	}
	
	Result<cv::Ptr<cv::cuda::Filter>*> cv_cuda_createLaplacianFilter_int_int_int_double_int_Scalar(int srcType, int dstType, int ksize, double scale, int borderMode, cv::Scalar* borderVal) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createLaplacianFilter(srcType, dstType, ksize, scale, borderMode, *borderVal);
			return Ok(new cv::Ptr<cv::cuda::Filter>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::Filter>*>))
	}
	
	Result<cv::Ptr<cv::cuda::Filter>*> cv_cuda_createLinearFilter_int_int_const__InputArrayR_Point_int_Scalar(int srcType, int dstType, const cv::_InputArray* kernel, cv::Point* anchor, int borderMode, cv::Scalar* borderVal) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createLinearFilter(srcType, dstType, *kernel, *anchor, borderMode, *borderVal);
			return Ok(new cv::Ptr<cv::cuda::Filter>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::Filter>*>))
	}
	
	Result<cv::Ptr<cv::cuda::Filter>*> cv_cuda_createMedianFilter_int_int_int(int srcType, int windowSize, int partition) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createMedianFilter(srcType, windowSize, partition);
			return Ok(new cv::Ptr<cv::cuda::Filter>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::Filter>*>))
	}
	
	Result<cv::Ptr<cv::cuda::Filter>*> cv_cuda_createMorphologyFilter_int_int_const__InputArrayR_Point_int(int op, int srcType, const cv::_InputArray* kernel, cv::Point* anchor, int iterations) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createMorphologyFilter(op, srcType, *kernel, *anchor, iterations);
			return Ok(new cv::Ptr<cv::cuda::Filter>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::Filter>*>))
	}
	
	Result<cv::Ptr<cv::cuda::Filter>*> cv_cuda_createRowSumFilter_int_int_int_int_int_Scalar(int srcType, int dstType, int ksize, int anchor, int borderMode, cv::Scalar* borderVal) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createRowSumFilter(srcType, dstType, ksize, anchor, borderMode, *borderVal);
			return Ok(new cv::Ptr<cv::cuda::Filter>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::Filter>*>))
	}
	
	Result<cv::Ptr<cv::cuda::Filter>*> cv_cuda_createScharrFilter_int_int_int_int_double_int_int(int srcType, int dstType, int dx, int dy, double scale, int rowBorderMode, int columnBorderMode) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createScharrFilter(srcType, dstType, dx, dy, scale, rowBorderMode, columnBorderMode);
			return Ok(new cv::Ptr<cv::cuda::Filter>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::Filter>*>))
	}
	
	Result<cv::Ptr<cv::cuda::Filter>*> cv_cuda_createSeparableLinearFilter_int_int_const__InputArrayR_const__InputArrayR_Point_int_int(int srcType, int dstType, const cv::_InputArray* rowKernel, const cv::_InputArray* columnKernel, cv::Point* anchor, int rowBorderMode, int columnBorderMode) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createSeparableLinearFilter(srcType, dstType, *rowKernel, *columnKernel, *anchor, rowBorderMode, columnBorderMode);
			return Ok(new cv::Ptr<cv::cuda::Filter>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::Filter>*>))
	}
	
	Result<cv::Ptr<cv::cuda::Filter>*> cv_cuda_createSobelFilter_int_int_int_int_int_double_int_int(int srcType, int dstType, int dx, int dy, int ksize, double scale, int rowBorderMode, int columnBorderMode) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createSobelFilter(srcType, dstType, dx, dy, ksize, scale, rowBorderMode, columnBorderMode);
			return Ok(new cv::Ptr<cv::cuda::Filter>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::Filter>*>))
	}
	
	Result_void cv_cuda_Filter_apply_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::Filter* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, cv::cuda::Stream* stream) {
		try {
			instance->apply(*src, *dst, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
