#include "core.hpp"
#include "core_types.hpp"

extern "C" {
	Result<bool> cv_Cholesky_doubleX_size_t_int_doubleX_size_t_int(double* A, size_t astep, int m, double* b, size_t bstep, int n) {
		try {
			bool ret = cv::Cholesky(A, astep, m, b, bstep, n);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_Cholesky_floatX_size_t_int_floatX_size_t_int(float* A, size_t astep, int m, float* b, size_t bstep, int n) {
		try {
			bool ret = cv::Cholesky(A, astep, m, b, bstep, n);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_LUT_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_InputArray* lut, const cv::_OutputArray* dst) {
		try {
			cv::LUT(*src, *lut, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_LU_doubleX_size_t_int_doubleX_size_t_int(double* A, size_t astep, int m, double* b, size_t bstep, int n) {
		try {
			int ret = cv::LU(A, astep, m, b, bstep, n);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_LU_floatX_size_t_int_floatX_size_t_int(float* A, size_t astep, int m, float* b, size_t bstep, int n) {
		try {
			int ret = cv::LU(A, astep, m, b, bstep, n);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<double> cv_Mahalanobis_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* v1, const cv::_InputArray* v2, const cv::_InputArray* icovar) {
		try {
			double ret = cv::Mahalanobis(*v1, *v2, *icovar);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_PCABackProject_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* data, const cv::_InputArray* mean, const cv::_InputArray* eigenvectors, const cv::_OutputArray* result) {
		try {
			cv::PCABackProject(*data, *mean, *eigenvectors, *result);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_double(const cv::_InputArray* data, const cv::_InputOutputArray* mean, const cv::_OutputArray* eigenvectors, double retainedVariance) {
		try {
			cv::PCACompute(*data, *mean, *eigenvectors, retainedVariance);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_int(const cv::_InputArray* data, const cv::_InputOutputArray* mean, const cv::_OutputArray* eigenvectors, int maxComponents) {
		try {
			cv::PCACompute(*data, *mean, *eigenvectors, maxComponents);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_PCAProject_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* data, const cv::_InputArray* mean, const cv::_InputArray* eigenvectors, const cv::_OutputArray* result) {
		try {
			cv::PCAProject(*data, *mean, *eigenvectors, *result);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_PSNR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2) {
		try {
			double ret = cv::PSNR(*src1, *src2);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_SVBackSubst_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* w, const cv::_InputArray* u, const cv::_InputArray* vt, const cv::_InputArray* rhs, const cv::_OutputArray* dst) {
		try {
			cv::SVBackSubst(*w, *u, *vt, *rhs, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_SVDecomp_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* w, const cv::_OutputArray* u, const cv::_OutputArray* vt, int flags) {
		try {
			cv::SVDecomp(*src, *w, *u, *vt, flags);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::MatExpr*> cv_abs_const_MatExprR(const cv::MatExpr* e) {
		try {
			cv::MatExpr ret = cv::abs(*e);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_abs_const_MatR(const cv::Mat* m) {
		try {
			cv::MatExpr ret = cv::abs(*m);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result_void cv_absdiff_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst) {
		try {
			cv::absdiff(*src1, *src2, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_addWeighted_const__InputArrayR_double_const__InputArrayR_double_double_const__OutputArrayR_int(const cv::_InputArray* src1, double alpha, const cv::_InputArray* src2, double beta, double gamma, const cv::_OutputArray* dst, int dtype) {
		try {
			cv::addWeighted(*src1, alpha, *src2, beta, gamma, *dst, dtype);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_add_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_int(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, const cv::_InputArray* mask, int dtype) {
		try {
			cv::add(*src1, *src2, *dst, *mask, dtype);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_batchDistance_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_const__OutputArrayR_int_int_const__InputArrayR_int_bool(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dist, int dtype, const cv::_OutputArray* nidx, int normType, int K, const cv::_InputArray* mask, int update, bool crosscheck) {
		try {
			cv::batchDistance(*src1, *src2, *dist, dtype, *nidx, normType, K, *mask, update, crosscheck);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_bitwise_and_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, const cv::_InputArray* mask) {
		try {
			cv::bitwise_and(*src1, *src2, *dst, *mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_bitwise_not_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* mask) {
		try {
			cv::bitwise_not(*src, *dst, *mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_bitwise_or_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, const cv::_InputArray* mask) {
		try {
			cv::bitwise_or(*src1, *src2, *dst, *mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_bitwise_xor_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, const cv::_InputArray* mask) {
		try {
			cv::bitwise_xor(*src1, *src2, *dst, *mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_borderInterpolate_int_int_int(int p, int len, int borderType) {
		try {
			int ret = cv::borderInterpolate(p, len, borderType);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_calcCovarMatrix_const__InputArrayR_const__OutputArrayR_const__InputOutputArrayR_int_int(const cv::_InputArray* samples, const cv::_OutputArray* covar, const cv::_InputOutputArray* mean, int flags, int ctype) {
		try {
			cv::calcCovarMatrix(*samples, *covar, *mean, flags, ctype);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cartToPolar_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool(const cv::_InputArray* x, const cv::_InputArray* y, const cv::_OutputArray* magnitude, const cv::_OutputArray* angle, bool angleInDegrees) {
		try {
			cv::cartToPolar(*x, *y, *magnitude, *angle, angleInDegrees);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_checkHardwareSupport_int(int feature) {
		try {
			bool ret = cv::checkHardwareSupport(feature);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_checkRange_const__InputArrayR_bool_PointX_double_double(const cv::_InputArray* a, bool quiet, cv::Point* pos, double minVal, double maxVal) {
		try {
			bool ret = cv::checkRange(*a, quiet, pos, minVal, maxVal);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_compare_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, int cmpop) {
		try {
			cv::compare(*src1, *src2, *dst, cmpop);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_completeSymm_const__InputOutputArrayR_bool(const cv::_InputOutputArray* mtx, bool lowerToUpper) {
		try {
			cv::completeSymm(*mtx, lowerToUpper);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_convertFp16_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst) {
		try {
			cv::convertFp16(*src, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_convertScaleAbs_const__InputArrayR_const__OutputArrayR_double_double(const cv::_InputArray* src, const cv::_OutputArray* dst, double alpha, double beta) {
		try {
			cv::convertScaleAbs(*src, *dst, alpha, beta);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_copyMakeBorder_const__InputArrayR_const__OutputArrayR_int_int_int_int_int_const_ScalarR(const cv::_InputArray* src, const cv::_OutputArray* dst, int top, int bottom, int left, int right, int borderType, const cv::Scalar* value) {
		try {
			cv::copyMakeBorder(*src, *dst, top, bottom, left, right, borderType, *value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_countNonZero_const__InputArrayR(const cv::_InputArray* src) {
		try {
			int ret = cv::countNonZero(*src);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<float> cv_cubeRoot_float(float val) {
		try {
			float ret = cv::cubeRoot(val);
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	#if !defined(OCVRS_TARGET_OS_WINDOWS)
	Result_void cv_cuda_convertFp16_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* _src, const cv::_OutputArray* _dst, cv::cuda::Stream* stream) {
		try {
			cv::cuda::convertFp16(*_src, *_dst, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	#endif
	
	Result_void cv_cuda_createContinuous_int_int_int_const__OutputArrayR(int rows, int cols, int type, const cv::_OutputArray* arr) {
		try {
			cv::cuda::createContinuous(rows, cols, type, *arr);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_cuda_deviceSupports_FeatureSet(cv::cuda::FeatureSet feature_set) {
		try {
			bool ret = cv::cuda::deviceSupports(feature_set);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_cuda_ensureSizeIsEnough_int_int_int_const__OutputArrayR(int rows, int cols, int type, const cv::_OutputArray* arr) {
		try {
			cv::cuda::ensureSizeIsEnough(rows, cols, type, *arr);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_getCudaEnabledDeviceCount() {
		try {
			int ret = cv::cuda::getCudaEnabledDeviceCount();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_cuda_getDevice() {
		try {
			int ret = cv::cuda::getDevice();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_printCudaDeviceInfo_int(int device) {
		try {
			cv::cuda::printCudaDeviceInfo(device);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_printShortCudaDeviceInfo_int(int device) {
		try {
			cv::cuda::printShortCudaDeviceInfo(device);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_registerPageLocked_MatR(cv::Mat* m) {
		try {
			cv::cuda::registerPageLocked(*m);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_resetDevice() {
		try {
			cv::cuda::resetDevice();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_setBufferPoolConfig_int_size_t_int(int deviceId, size_t stackSize, int stackCount) {
		try {
			cv::cuda::setBufferPoolConfig(deviceId, stackSize, stackCount);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_setBufferPoolUsage_bool(bool on) {
		try {
			cv::cuda::setBufferPoolUsage(on);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_setDevice_int(int device) {
		try {
			cv::cuda::setDevice(device);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_unregisterPageLocked_MatR(cv::Mat* m) {
		try {
			cv::cuda::unregisterPageLocked(*m);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dct_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int flags) {
		try {
			cv::dct(*src, *dst, flags);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_determinant_const__InputArrayR(const cv::_InputArray* mtx) {
		try {
			double ret = cv::determinant(*mtx);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_dft_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int flags, int nonzeroRows) {
		try {
			cv::dft(*src, *dst, flags, nonzeroRows);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_directx_getTypeFromD3DFORMAT_const_int(const int iD3DFORMAT) {
		try {
			int ret = cv::directx::getTypeFromD3DFORMAT(iD3DFORMAT);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_directx_getTypeFromDXGI_FORMAT_const_int(const int iDXGI_FORMAT) {
		try {
			int ret = cv::directx::getTypeFromDXGI_FORMAT(iDXGI_FORMAT);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_divide_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_int(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, double scale, int dtype) {
		try {
			cv::divide(*src1, *src2, *dst, scale, dtype);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_divide_double_const__InputArrayR_const__OutputArrayR_int(double scale, const cv::_InputArray* src2, const cv::_OutputArray* dst, int dtype) {
		try {
			cv::divide(scale, *src2, *dst, dtype);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_eigen_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* eigenvalues, const cv::_OutputArray* eigenvectors) {
		try {
			bool ret = cv::eigen(*src, *eigenvalues, *eigenvectors);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_errorNoReturn_int_const_StringR_const_charX_const_charX_int(int _code, const char* _err, const char* _func, const char* _file, int _line) {
		try {
			cv::errorNoReturn(_code, cv::String(_err), _func, _file, _line);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_error_const_ExceptionR(const cv::Exception* exc) {
		try {
			cv::error(*exc);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_error_int_const_StringR_const_charX_const_charX_int(int _code, const char* _err, const char* _func, const char* _file, int _line) {
		try {
			cv::error(_code, cv::String(_err), _func, _file, _line);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_exp_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst) {
		try {
			cv::exp(*src, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_extractChannel_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int coi) {
		try {
			cv::extractChannel(*src, *dst, coi);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_extractImageCOI_const_CvArrX_const__OutputArrayR_int(const CvArr* arr, const cv::_OutputArray* coiimg, int coi) {
		try {
			cv::extractImageCOI(arr, *coiimg, coi);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_fastAtan2_float_float(float y, float x) {
		try {
			float ret = cv::fastAtan2(y, x);
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_findNonZero_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* idx) {
		try {
			cv::findNonZero(*src, *idx);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_flip_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int flipCode) {
		try {
			cv::flip(*src, *dst, flipCode);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_gemm_const__InputArrayR_const__InputArrayR_double_const__InputArrayR_double_const__OutputArrayR_int(const cv::_InputArray* src1, const cv::_InputArray* src2, double alpha, const cv::_InputArray* src3, double beta, const cv::_OutputArray* dst, int flags) {
		try {
			cv::gemm(*src1, *src2, alpha, *src3, beta, *dst, flags);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_getBuildInformation() {
		try {
			const cv::String ret = cv::getBuildInformation();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<int64_t> cv_getCPUTickCount() {
		try {
			int64_t ret = cv::getCPUTickCount();
			return Ok<int64_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int64_t>))
	}
	
	Result<size_t> cv_getElemSize_int(int type) {
		try {
			size_t ret = cv::getElemSize(type);
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	#if !defined(OCVRS_TARGET_OS_WINDOWS)
	Result<int> cv_getImpl_vector_int_R_vector_String_R(std::vector<int>* impl, std::vector<cv::String>* funName) {
		try {
			int ret = cv::getImpl(*impl, *funName);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	#endif
	
	Result<int> cv_getNumThreads() {
		try {
			int ret = cv::getNumThreads();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_getNumberOfCPUs() {
		try {
			int ret = cv::getNumberOfCPUs();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_getOptimalDFTSize_int(int vecsize) {
		try {
			int ret = cv::getOptimalDFTSize(vecsize);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_getThreadNum() {
		try {
			int ret = cv::getThreadNum();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int64_t> cv_getTickCount() {
		try {
			int64_t ret = cv::getTickCount();
			return Ok<int64_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int64_t>))
	}
	
	Result<double> cv_getTickFrequency() {
		try {
			double ret = cv::getTickFrequency();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_glob_String_vector_String_R_bool(char* pattern, std::vector<cv::String>* result, bool recursive) {
		try {
			cv::glob(cv::String(pattern), *result, recursive);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_haveOpenVX() {
		try {
			bool ret = cv::haveOpenVX();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_hconcat_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst) {
		try {
			cv::hconcat(*src1, *src2, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_hconcat_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst) {
		try {
			cv::hconcat(*src, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_idct_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int flags) {
		try {
			cv::idct(*src, *dst, flags);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_idft_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int flags, int nonzeroRows) {
		try {
			cv::idft(*src, *dst, flags, nonzeroRows);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_inRange_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_InputArray* lowerb, const cv::_InputArray* upperb, const cv::_OutputArray* dst) {
		try {
			cv::inRange(*src, *lowerb, *upperb, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_insertChannel_const__InputArrayR_const__InputOutputArrayR_int(const cv::_InputArray* src, const cv::_InputOutputArray* dst, int coi) {
		try {
			cv::insertChannel(*src, *dst, coi);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_insertImageCOI_const__InputArrayR_CvArrX_int(const cv::_InputArray* coiimg, CvArr* arr, int coi) {
		try {
			cv::insertImageCOI(*coiimg, arr, coi);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::instr::FLAGS> cv_instr_getFlags() {
		try {
			cv::instr::FLAGS ret = cv::instr::getFlags();
			return Ok<cv::instr::FLAGS>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::instr::FLAGS>))
	}
	
	Result_void cv_instr_resetTrace() {
		try {
			cv::instr::resetTrace();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_instr_setFlags_FLAGS(cv::instr::FLAGS modeFlags) {
		try {
			cv::instr::setFlags(modeFlags);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_instr_setUseInstrumentation_bool(bool flag) {
		try {
			cv::instr::setUseInstrumentation(flag);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_instr_useInstrumentation() {
		try {
			bool ret = cv::instr::useInstrumentation();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<double> cv_invert_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int flags) {
		try {
			double ret = cv::invert(*src, *dst, flags);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<void*> cv_ipp_getIppErrorLocation() {
		try {
			cv::String ret = cv::ipp::getIppErrorLocation();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<int> cv_ipp_getIppFeatures() {
		try {
			int ret = cv::ipp::getIppFeatures();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_ipp_getIppStatus() {
		try {
			int ret = cv::ipp::getIppStatus();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_ipp_setIppStatus_int_const_charX_const_charX_int(int status, const char* funcname, const char* filename, int line) {
		try {
			cv::ipp::setIppStatus(status, funcname, filename, line);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ipp_setUseIPP_bool(bool flag) {
		try {
			cv::ipp::setUseIPP(flag);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_ipp_useIPP() {
		try {
			bool ret = cv::ipp::useIPP();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<double> cv_kmeans_const__InputArrayR_int_const__InputOutputArrayR_TermCriteria_int_int_const__OutputArrayR(const cv::_InputArray* data, int K, const cv::_InputOutputArray* bestLabels, cv::TermCriteria* criteria, int attempts, int flags, const cv::_OutputArray* centers) {
		try {
			double ret = cv::kmeans(*data, K, *bestLabels, *criteria, attempts, flags, *centers);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_log_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst) {
		try {
			cv::log(*src, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_magnitude_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* x, const cv::_InputArray* y, const cv::_OutputArray* magnitude) {
		try {
			cv::magnitude(*x, *y, *magnitude);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::MatExpr*> cv_max_const_MatR_const_MatR(const cv::Mat* a, const cv::Mat* b) {
		try {
			cv::MatExpr ret = cv::max(*a, *b);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result_void cv_max_const_MatR_const_MatR_MatR(const cv::Mat* src1, const cv::Mat* src2, cv::Mat* dst) {
		try {
			cv::max(*src1, *src2, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::MatExpr*> cv_max_const_MatR_double(const cv::Mat* a, double s) {
		try {
			cv::MatExpr ret = cv::max(*a, s);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result_void cv_max_const_UMatR_const_UMatR_UMatR(const cv::UMat* src1, const cv::UMat* src2, cv::UMat* dst) {
		try {
			cv::max(*src1, *src2, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_max_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst) {
		try {
			cv::max(*src1, *src2, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::MatExpr*> cv_max_double_const_MatR(double s, const cv::Mat* a) {
		try {
			cv::MatExpr ret = cv::max(s, *a);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result_void cv_meanStdDev_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_OutputArray* mean, const cv::_OutputArray* stddev, const cv::_InputArray* mask) {
		try {
			cv::meanStdDev(*src, *mean, *stddev, *mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Scalar> cv_mean_const__InputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_InputArray* mask) {
		try {
			cv::Scalar ret = cv::mean(*src, *mask);
			return Ok<cv::Scalar>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	Result_void cv_merge_const_MatX_size_t_const__OutputArrayR(const cv::Mat* mv, size_t count, const cv::_OutputArray* dst) {
		try {
			cv::merge(mv, count, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_merge_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* mv, const cv::_OutputArray* dst) {
		try {
			cv::merge(*mv, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_minMaxIdx_const__InputArrayR_doubleX_doubleX_intX_intX_const__InputArrayR(const cv::_InputArray* src, double* minVal, double* maxVal, int* minIdx, int* maxIdx, const cv::_InputArray* mask) {
		try {
			cv::minMaxIdx(*src, minVal, maxVal, minIdx, maxIdx, *mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_minMaxLoc_const_SparseMatR_doubleX_doubleX_intX_intX(const cv::SparseMat* a, double* minVal, double* maxVal, int* minIdx, int* maxIdx) {
		try {
			cv::minMaxLoc(*a, minVal, maxVal, minIdx, maxIdx);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_minMaxLoc_const__InputArrayR_doubleX_doubleX_PointX_PointX_const__InputArrayR(const cv::_InputArray* src, double* minVal, double* maxVal, cv::Point* minLoc, cv::Point* maxLoc, const cv::_InputArray* mask) {
		try {
			cv::minMaxLoc(*src, minVal, maxVal, minLoc, maxLoc, *mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::MatExpr*> cv_min_const_MatR_const_MatR(const cv::Mat* a, const cv::Mat* b) {
		try {
			cv::MatExpr ret = cv::min(*a, *b);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result_void cv_min_const_MatR_const_MatR_MatR(const cv::Mat* src1, const cv::Mat* src2, cv::Mat* dst) {
		try {
			cv::min(*src1, *src2, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::MatExpr*> cv_min_const_MatR_double(const cv::Mat* a, double s) {
		try {
			cv::MatExpr ret = cv::min(*a, s);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result_void cv_min_const_UMatR_const_UMatR_UMatR(const cv::UMat* src1, const cv::UMat* src2, cv::UMat* dst) {
		try {
			cv::min(*src1, *src2, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_min_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst) {
		try {
			cv::min(*src1, *src2, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::MatExpr*> cv_min_double_const_MatR(double s, const cv::Mat* a) {
		try {
			cv::MatExpr ret = cv::min(s, *a);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result_void cv_mixChannels_const__InputArrayR_const__InputOutputArrayR_const_intX_size_t(const cv::_InputArray* src, const cv::_InputOutputArray* dst, const int* fromTo, size_t npairs) {
		try {
			cv::mixChannels(*src, *dst, fromTo, npairs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_mixChannels_const__InputArrayR_const__InputOutputArrayR_const_vector_int_R(const cv::_InputArray* src, const cv::_InputOutputArray* dst, const std::vector<int>* fromTo) {
		try {
			cv::mixChannels(*src, *dst, *fromTo);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_mulSpectrums_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_bool(const cv::_InputArray* a, const cv::_InputArray* b, const cv::_OutputArray* c, int flags, bool conjB) {
		try {
			cv::mulSpectrums(*a, *b, *c, flags, conjB);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_mulTransposed_const__InputArrayR_const__OutputArrayR_bool_const__InputArrayR_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, bool aTa, const cv::_InputArray* delta, double scale, int dtype) {
		try {
			cv::mulTransposed(*src, *dst, aTa, *delta, scale, dtype);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_multiply_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_int(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, double scale, int dtype) {
		try {
			cv::multiply(*src1, *src2, *dst, scale, dtype);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<const cv::_InputOutputArray*> cv_noArray() {
		try {
			const cv::_InputOutputArray ret = cv::noArray();
			return Ok(new const cv::_InputOutputArray(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<const cv::_InputOutputArray*>))
	}
	
	Result<double> cv_norm_const_SparseMatR_int(const cv::SparseMat* src, int normType) {
		try {
			double ret = cv::norm(*src, normType);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<double> cv_norm_const__InputArrayR_const__InputArrayR_int_const__InputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, int normType, const cv::_InputArray* mask) {
		try {
			double ret = cv::norm(*src1, *src2, normType, *mask);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<double> cv_norm_const__InputArrayR_int_const__InputArrayR(const cv::_InputArray* src1, int normType, const cv::_InputArray* mask) {
		try {
			double ret = cv::norm(*src1, normType, *mask);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_normalize_const_SparseMatR_SparseMatR_double_int(const cv::SparseMat* src, cv::SparseMat* dst, double alpha, int normType) {
		try {
			cv::normalize(*src, *dst, alpha, normType);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_normalize_const__InputArrayR_const__InputOutputArrayR_double_double_int_int_const__InputArrayR(const cv::_InputArray* src, const cv::_InputOutputArray* dst, double alpha, double beta, int norm_type, int dtype, const cv::_InputArray* mask) {
		try {
			cv::normalize(*src, *dst, alpha, beta, norm_type, dtype, *mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ocl_attachContext_const_StringR_voidX_voidX_voidX(const char* platformName, void* platformID, void* context, void* deviceID) {
		try {
			cv::ocl::attachContext(cv::String(platformName), platformID, context, deviceID);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ocl_buildOptionsAddMatrixDescription_StringR_const_StringR_const__InputArrayR(void** buildOptions, const char* name, const cv::_InputArray* _m) {
		try {
			cv::String buildOptions_out;
			cv::ocl::buildOptionsAddMatrixDescription(buildOptions_out, cv::String(name), *_m);
			*buildOptions = ocvrs_create_string(buildOptions_out.c_str());
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_ocl_checkOptimalVectorWidth_const_intX_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_OclVectorStrategy(const int* vectorWidths, const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_InputArray* src3, const cv::_InputArray* src4, const cv::_InputArray* src5, const cv::_InputArray* src6, const cv::_InputArray* src7, const cv::_InputArray* src8, const cv::_InputArray* src9, cv::ocl::OclVectorStrategy strat) {
		try {
			int ret = cv::ocl::checkOptimalVectorWidth(vectorWidths, *src1, *src2, *src3, *src4, *src5, *src6, *src7, *src8, *src9, strat);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_ocl_convertFromBuffer_voidX_size_t_int_int_int_UMatR(void* cl_mem_buffer, size_t step, int rows, int cols, int type, cv::UMat* dst) {
		try {
			cv::ocl::convertFromBuffer(cl_mem_buffer, step, rows, cols, type, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ocl_convertFromImage_voidX_UMatR(void* cl_mem_image, cv::UMat* dst) {
		try {
			cv::ocl::convertFromImage(cl_mem_image, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_ocl_convertTypeStr_int_int_int_charX(int sdepth, int ddepth, int cn, void** buf) {
		try {
			char* buf_out = new char[1024]();
			const char* ret = cv::ocl::convertTypeStr(sdepth, ddepth, cn, buf_out);
			*buf = ocvrs_create_string(buf_out);
			Result<void*> f_ret = Ok(ocvrs_create_string(ret));
			delete[] buf_out;
			return f_ret;
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_ocl_finish() {
		try {
			cv::ocl::finish();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ocl_getPlatfomsInfo_vector_PlatformInfo_R(std::vector<cv::ocl::PlatformInfo>* platform_info) {
		try {
			cv::ocl::getPlatfomsInfo(*platform_info);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_ocl_haveAmdBlas() {
		try {
			bool ret = cv::ocl::haveAmdBlas();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_ocl_haveAmdFft() {
		try {
			bool ret = cv::ocl::haveAmdFft();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_ocl_haveOpenCL() {
		try {
			bool ret = cv::ocl::haveOpenCL();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_ocl_haveSVM() {
		try {
			bool ret = cv::ocl::haveSVM();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<void*> cv_ocl_kernelToStr_const__InputArrayR_int_const_charX(const cv::_InputArray* _kernel, int ddepth, const char* name) {
		try {
			cv::String ret = cv::ocl::kernelToStr(*_kernel, ddepth, name);
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<void*> cv_ocl_memopTypeToStr_int(int t) {
		try {
			const char* ret = cv::ocl::memopTypeToStr(t);
			return Ok(ocvrs_create_string(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<int> cv_ocl_predictOptimalVectorWidthMax_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_InputArray* src3, const cv::_InputArray* src4, const cv::_InputArray* src5, const cv::_InputArray* src6, const cv::_InputArray* src7, const cv::_InputArray* src8, const cv::_InputArray* src9) {
		try {
			int ret = cv::ocl::predictOptimalVectorWidthMax(*src1, *src2, *src3, *src4, *src5, *src6, *src7, *src8, *src9);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_ocl_predictOptimalVectorWidth_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_OclVectorStrategy(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_InputArray* src3, const cv::_InputArray* src4, const cv::_InputArray* src5, const cv::_InputArray* src6, const cv::_InputArray* src7, const cv::_InputArray* src8, const cv::_InputArray* src9, cv::ocl::OclVectorStrategy strat) {
		try {
			int ret = cv::ocl::predictOptimalVectorWidth(*src1, *src2, *src3, *src4, *src5, *src6, *src7, *src8, *src9, strat);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_ocl_setUseOpenCL_bool(bool flag) {
		try {
			cv::ocl::setUseOpenCL(flag);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_ocl_typeToStr_int(int t) {
		try {
			const char* ret = cv::ocl::typeToStr(t);
			return Ok(ocvrs_create_string(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<bool> cv_ocl_useOpenCL() {
		try {
			bool ret = cv::ocl::useOpenCL();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<void*> cv_ocl_vecopTypeToStr_int(int t) {
		try {
			const char* ret = cv::ocl::vecopTypeToStr(t);
			return Ok(ocvrs_create_string(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<cv::MatExpr*> cv_operatorA_const_MatExprR_const_MatExprR(const cv::MatExpr* e1, const cv::MatExpr* e2) {
		try {
			cv::MatExpr ret = cv::operator+(*e1, *e2);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorA_const_MatExprR_const_MatR(const cv::MatExpr* e, const cv::Mat* m) {
		try {
			cv::MatExpr ret = cv::operator+(*e, *m);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorA_const_MatExprR_const_ScalarR(const cv::MatExpr* e, const cv::Scalar* s) {
		try {
			cv::MatExpr ret = cv::operator+(*e, *s);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorA_const_MatR_const_MatExprR(const cv::Mat* m, const cv::MatExpr* e) {
		try {
			cv::MatExpr ret = cv::operator+(*m, *e);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorA_const_MatR_const_MatR(const cv::Mat* a, const cv::Mat* b) {
		try {
			cv::MatExpr ret = cv::operator+(*a, *b);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorA_const_MatR_const_ScalarR(const cv::Mat* a, const cv::Scalar* s) {
		try {
			cv::MatExpr ret = cv::operator+(*a, *s);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorA_const_ScalarR_const_MatExprR(const cv::Scalar* s, const cv::MatExpr* e) {
		try {
			cv::MatExpr ret = cv::operator+(*s, *e);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorA_const_ScalarR_const_MatR(const cv::Scalar* s, const cv::Mat* a) {
		try {
			cv::MatExpr ret = cv::operator+(*s, *a);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorD_const_MatExprR_const_MatExprR(const cv::MatExpr* e1, const cv::MatExpr* e2) {
		try {
			cv::MatExpr ret = cv::operator/(*e1, *e2);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorD_const_MatExprR_const_MatR(const cv::MatExpr* e, const cv::Mat* m) {
		try {
			cv::MatExpr ret = cv::operator/(*e, *m);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorD_const_MatExprR_double(const cv::MatExpr* e, double s) {
		try {
			cv::MatExpr ret = cv::operator/(*e, s);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorD_const_MatR_const_MatExprR(const cv::Mat* m, const cv::MatExpr* e) {
		try {
			cv::MatExpr ret = cv::operator/(*m, *e);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorD_const_MatR_const_MatR(const cv::Mat* a, const cv::Mat* b) {
		try {
			cv::MatExpr ret = cv::operator/(*a, *b);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorD_const_MatR_double(const cv::Mat* a, double s) {
		try {
			cv::MatExpr ret = cv::operator/(*a, s);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorD_double_const_MatExprR(double s, const cv::MatExpr* e) {
		try {
			cv::MatExpr ret = cv::operator/(s, *e);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorD_double_const_MatR(double s, const cv::Mat* a) {
		try {
			cv::MatExpr ret = cv::operator/(s, *a);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorS_const_MatExprR(const cv::MatExpr* e) {
		try {
			cv::MatExpr ret = cv::operator-(*e);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorS_const_MatExprR_const_MatExprR(const cv::MatExpr* e1, const cv::MatExpr* e2) {
		try {
			cv::MatExpr ret = cv::operator-(*e1, *e2);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorS_const_MatExprR_const_MatR(const cv::MatExpr* e, const cv::Mat* m) {
		try {
			cv::MatExpr ret = cv::operator-(*e, *m);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorS_const_MatExprR_const_ScalarR(const cv::MatExpr* e, const cv::Scalar* s) {
		try {
			cv::MatExpr ret = cv::operator-(*e, *s);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorS_const_MatR(const cv::Mat* m) {
		try {
			cv::MatExpr ret = cv::operator-(*m);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorS_const_MatR_const_MatExprR(const cv::Mat* m, const cv::MatExpr* e) {
		try {
			cv::MatExpr ret = cv::operator-(*m, *e);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorS_const_MatR_const_MatR(const cv::Mat* a, const cv::Mat* b) {
		try {
			cv::MatExpr ret = cv::operator-(*a, *b);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorS_const_MatR_const_ScalarR(const cv::Mat* a, const cv::Scalar* s) {
		try {
			cv::MatExpr ret = cv::operator-(*a, *s);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorS_const_ScalarR_const_MatExprR(const cv::Scalar* s, const cv::MatExpr* e) {
		try {
			cv::MatExpr ret = cv::operator-(*s, *e);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorS_const_ScalarR_const_MatR(const cv::Scalar* s, const cv::Mat* a) {
		try {
			cv::MatExpr ret = cv::operator-(*s, *a);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorX_const_MatExprR_const_MatExprR(const cv::MatExpr* e1, const cv::MatExpr* e2) {
		try {
			cv::MatExpr ret = cv::operator*(*e1, *e2);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorX_const_MatExprR_const_MatR(const cv::MatExpr* e, const cv::Mat* m) {
		try {
			cv::MatExpr ret = cv::operator*(*e, *m);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorX_const_MatExprR_double(const cv::MatExpr* e, double s) {
		try {
			cv::MatExpr ret = cv::operator*(*e, s);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorX_const_MatR_const_MatExprR(const cv::Mat* m, const cv::MatExpr* e) {
		try {
			cv::MatExpr ret = cv::operator*(*m, *e);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorX_const_MatR_const_MatR(const cv::Mat* a, const cv::Mat* b) {
		try {
			cv::MatExpr ret = cv::operator*(*a, *b);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorX_const_MatR_double(const cv::Mat* a, double s) {
		try {
			cv::MatExpr ret = cv::operator*(*a, s);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorX_double_const_MatExprR(double s, const cv::MatExpr* e) {
		try {
			cv::MatExpr ret = cv::operator*(s, *e);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_operatorX_double_const_MatR(double s, const cv::Mat* a) {
		try {
			cv::MatExpr ret = cv::operator*(s, *a);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result_void cv_parallel_for__const_RangeR_const_ParallelLoopBodyR_double(const cv::Range* range, const cv::ParallelLoopBody* body, double nstripes) {
		try {
			cv::parallel_for_(*range, *body, nstripes);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_patchNaNs_const__InputOutputArrayR_double(const cv::_InputOutputArray* a, double val) {
		try {
			cv::patchNaNs(*a, val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_perspectiveTransform_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* m) {
		try {
			cv::perspectiveTransform(*src, *dst, *m);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_phase_const__InputArrayR_const__InputArrayR_const__OutputArrayR_bool(const cv::_InputArray* x, const cv::_InputArray* y, const cv::_OutputArray* angle, bool angleInDegrees) {
		try {
			cv::phase(*x, *y, *angle, angleInDegrees);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_polarToCart_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool(const cv::_InputArray* magnitude, const cv::_InputArray* angle, const cv::_OutputArray* x, const cv::_OutputArray* y, bool angleInDegrees) {
		try {
			cv::polarToCart(*magnitude, *angle, *x, *y, angleInDegrees);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_pow_const__InputArrayR_double_const__OutputArrayR(const cv::_InputArray* src, double power, const cv::_OutputArray* dst) {
		try {
			cv::pow(*src, power, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_randShuffle_const__InputOutputArrayR_double_RNGX(const cv::_InputOutputArray* dst, double iterFactor, cv::RNG* rng) {
		try {
			cv::randShuffle(*dst, iterFactor, rng);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_randn_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputOutputArray* dst, const cv::_InputArray* mean, const cv::_InputArray* stddev) {
		try {
			cv::randn(*dst, *mean, *stddev);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_randu_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputOutputArray* dst, const cv::_InputArray* low, const cv::_InputArray* high) {
		try {
			cv::randu(*dst, *low, *high);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_read_const_FileNodeR_MatR_const_MatR(const cv::FileNode* node, cv::Mat* mat, const cv::Mat* default_mat) {
		try {
			cv::read(*node, *mat, *default_mat);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_read_const_FileNodeR_SparseMatR_const_SparseMatR(const cv::FileNode* node, cv::SparseMat* mat, const cv::SparseMat* default_mat) {
		try {
			cv::read(*node, *mat, *default_mat);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_read_const_FileNodeR_StringR_const_StringR(const cv::FileNode* node, void** value, const char* default_value) {
		try {
			cv::String value_out;
			cv::read(*node, value_out, cv::String(default_value));
			*value = ocvrs_create_string(value_out.c_str());
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_read_const_FileNodeR_doubleR_double(const cv::FileNode* node, double* value, double default_value) {
		try {
			cv::read(*node, *value, default_value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_read_const_FileNodeR_floatR_float(const cv::FileNode* node, float* value, float default_value) {
		try {
			cv::read(*node, *value, default_value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_read_const_FileNodeR_intR_int(const cv::FileNode* node, int* value, int default_value) {
		try {
			cv::read(*node, *value, default_value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_read_const_FileNodeR_vector_DMatch_R(const cv::FileNode* node, std::vector<cv::DMatch>* matches) {
		try {
			cv::read(*node, *matches);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_read_const_FileNodeR_vector_KeyPoint_R(const cv::FileNode* node, std::vector<cv::KeyPoint>* keypoints) {
		try {
			cv::read(*node, *keypoints);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_reduce_const__InputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int dim, int rtype, int dtype) {
		try {
			cv::reduce(*src, *dst, dim, rtype, dtype);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_repeat_const_MatR_int_int(const cv::Mat* src, int ny, int nx) {
		try {
			cv::Mat ret = cv::repeat(*src, ny, nx);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_repeat_const__InputArrayR_int_int_const__OutputArrayR(const cv::_InputArray* src, int ny, int nx, const cv::_OutputArray* dst) {
		try {
			cv::repeat(*src, ny, nx, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_rotate_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int rotateCode) {
		try {
			cv::rotate(*src, *dst, rotateCode);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_scaleAdd_const__InputArrayR_double_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, double alpha, const cv::_InputArray* src2, const cv::_OutputArray* dst) {
		try {
			cv::scaleAdd(*src1, alpha, *src2, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_setBreakOnError_bool(bool flag) {
		try {
			bool ret = cv::setBreakOnError(flag);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_setIdentity_const__InputOutputArrayR_const_ScalarR(const cv::_InputOutputArray* mtx, const cv::Scalar* s) {
		try {
			cv::setIdentity(*mtx, *s);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_setNumThreads_int(int nthreads) {
		try {
			cv::setNumThreads(nthreads);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_setRNGSeed_int(int seed) {
		try {
			cv::setRNGSeed(seed);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_setUseOpenVX_bool(bool flag) {
		try {
			cv::setUseOpenVX(flag);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_setUseOptimized_bool(bool onoff) {
		try {
			cv::setUseOptimized(onoff);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_solveCubic_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* coeffs, const cv::_OutputArray* roots) {
		try {
			int ret = cv::solveCubic(*coeffs, *roots);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_solveLP_const_MatR_const_MatR_MatR(const cv::Mat* Func, const cv::Mat* Constr, cv::Mat* z) {
		try {
			int ret = cv::solveLP(*Func, *Constr, *z);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<double> cv_solvePoly_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* coeffs, const cv::_OutputArray* roots, int maxIters) {
		try {
			double ret = cv::solvePoly(*coeffs, *roots, maxIters);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<bool> cv_solve_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, int flags) {
		try {
			bool ret = cv::solve(*src1, *src2, *dst, flags);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_sortIdx_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int flags) {
		try {
			cv::sortIdx(*src, *dst, flags);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_sort_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int flags) {
		try {
			cv::sort(*src, *dst, flags);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_split_const_MatR_MatX(const cv::Mat* src, cv::Mat* mvbegin) {
		try {
			cv::split(*src, mvbegin);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_split_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* m, const cv::_OutputArray* mv) {
		try {
			cv::split(*m, *mv);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_sqrt_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst) {
		try {
			cv::sqrt(*src, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_subtract_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_int(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, const cv::_InputArray* mask, int dtype) {
		try {
			cv::subtract(*src1, *src2, *dst, *mask, dtype);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Scalar> cv_sum_const__InputArrayR(const cv::_InputArray* src) {
		try {
			cv::Scalar ret = cv::sum(*src);
			return Ok<cv::Scalar>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	Result_void cv_swap_MatR_MatR(cv::Mat* a, cv::Mat* b) {
		try {
			cv::swap(*a, *b);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_swap_UMatR_UMatR(cv::UMat* a, cv::UMat* b) {
		try {
			cv::swap(*a, *b);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_tempfile_const_charX(const char* suffix) {
		try {
			cv::String ret = cv::tempfile(suffix);
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<cv::RNG*> cv_theRNG() {
		try {
			cv::RNG ret = cv::theRNG();
			return Ok(new cv::RNG(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::RNG*>))
	}
	
	Result<cv::Scalar> cv_trace_const__InputArrayR(const cv::_InputArray* mtx) {
		try {
			cv::Scalar ret = cv::trace(*mtx);
			return Ok<cv::Scalar>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	Result_void cv_transform_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* m) {
		try {
			cv::transform(*src, *dst, *m);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_transpose_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst) {
		try {
			cv::transpose(*src, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_useOpenVX() {
		try {
			bool ret = cv::useOpenVX();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_useOptimized() {
		try {
			bool ret = cv::useOptimized();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_va_intel_convertFromVASurface_VADisplay_VASurfaceID_Size_const__OutputArrayR(VADisplay display, VASurfaceID surface, cv::Size* size, const cv::_OutputArray* dst) {
		try {
			cv::va_intel::convertFromVASurface(display, surface, *size, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_va_intel_convertToVASurface_VADisplay_const__InputArrayR_VASurfaceID_Size(VADisplay display, const cv::_InputArray* src, VASurfaceID surface, cv::Size* size) {
		try {
			cv::va_intel::convertToVASurface(display, *src, surface, *size);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::ocl::Context*> cv_va_intel_ocl_initializeContextFromVA_VADisplay_bool(VADisplay display, bool tryInterop) {
		try {
			cv::ocl::Context ret = cv::va_intel::ocl::initializeContextFromVA(display, tryInterop);
			return Ok(new cv::ocl::Context(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Context*>))
	}
	
	Result_void cv_vconcat_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst) {
		try {
			cv::vconcat(*src1, *src2, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_vconcat_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst) {
		try {
			cv::vconcat(*src, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_writeScalar_FileStorageR_const_StringR(cv::FileStorage* fs, const char* value) {
		try {
			cv::writeScalar(*fs, cv::String(value));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_writeScalar_FileStorageR_double(cv::FileStorage* fs, double value) {
		try {
			cv::writeScalar(*fs, value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_writeScalar_FileStorageR_float(cv::FileStorage* fs, float value) {
		try {
			cv::writeScalar(*fs, value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_writeScalar_FileStorageR_int(cv::FileStorage* fs, int value) {
		try {
			cv::writeScalar(*fs, value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_write_FileStorageR_const_StringR_const_MatR(cv::FileStorage* fs, const char* name, const cv::Mat* value) {
		try {
			cv::write(*fs, cv::String(name), *value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_write_FileStorageR_const_StringR_const_SparseMatR(cv::FileStorage* fs, const char* name, const cv::SparseMat* value) {
		try {
			cv::write(*fs, cv::String(name), *value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_write_FileStorageR_const_StringR_const_StringR(cv::FileStorage* fs, const char* name, const char* value) {
		try {
			cv::write(*fs, cv::String(name), cv::String(value));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_write_FileStorageR_const_StringR_const_vector_DMatch_R(cv::FileStorage* fs, const char* name, const std::vector<cv::DMatch>* value) {
		try {
			cv::write(*fs, cv::String(name), *value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_write_FileStorageR_const_StringR_const_vector_KeyPoint_R(cv::FileStorage* fs, const char* name, const std::vector<cv::KeyPoint>* value) {
		try {
			cv::write(*fs, cv::String(name), *value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_write_FileStorageR_const_StringR_double(cv::FileStorage* fs, const char* name, double value) {
		try {
			cv::write(*fs, cv::String(name), value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_write_FileStorageR_const_StringR_float(cv::FileStorage* fs, const char* name, float value) {
		try {
			cv::write(*fs, cv::String(name), value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_write_FileStorageR_const_StringR_int(cv::FileStorage* fs, const char* name, int value) {
		try {
			cv::write(*fs, cv::String(name), value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Algorithm_delete(cv::Algorithm* instance) {
		delete instance;
	}
	Result<cv::Algorithm*> cv_Algorithm_Algorithm() {
		try {
			cv::Algorithm* ret = new cv::Algorithm();
			return Ok<cv::Algorithm*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Algorithm*>))
	}
	
	Result_void cv_Algorithm_clear(cv::Algorithm* instance) {
		try {
			instance->clear();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_Algorithm_write_const_FileStorageR(const cv::Algorithm* instance, cv::FileStorage* fs) {
		try {
			instance->write(*fs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_Algorithm_read_const_FileNodeR(cv::Algorithm* instance, const cv::FileNode* fn) {
		try {
			instance->read(*fn);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_Algorithm_empty_const(const cv::Algorithm* instance) {
		try {
			bool ret = instance->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_Algorithm_save_const_const_StringR(const cv::Algorithm* instance, const char* filename) {
		try {
			instance->save(cv::String(filename));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_Algorithm_getDefaultName_const(const cv::Algorithm* instance) {
		try {
			cv::String ret = instance->getDefaultName();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	void cv_AutoLock_delete(cv::AutoLock* instance) {
		delete instance;
	}
	Result<cv::AutoLock*> cv_AutoLock_AutoLock_MutexR(cv::Mutex* m) {
		try {
			cv::AutoLock* ret = new cv::AutoLock(*m);
			return Ok<cv::AutoLock*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::AutoLock*>))
	}
	
	void cv_CommandLineParser_delete(cv::CommandLineParser* instance) {
		delete instance;
	}
	Result<cv::CommandLineParser*> cv_CommandLineParser_CommandLineParser_int_const_charXX_const_StringR(int argc, const char** argv, const char* keys) {
		try {
			cv::CommandLineParser* ret = new cv::CommandLineParser(argc, argv, cv::String(keys));
			return Ok<cv::CommandLineParser*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::CommandLineParser*>))
	}
	
	Result<cv::CommandLineParser*> cv_CommandLineParser_CommandLineParser_const_CommandLineParserR(const cv::CommandLineParser* parser) {
		try {
			cv::CommandLineParser* ret = new cv::CommandLineParser(*parser);
			return Ok<cv::CommandLineParser*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::CommandLineParser*>))
	}
	
	Result<void*> cv_CommandLineParser_getPathToApplication_const(const cv::CommandLineParser* instance) {
		try {
			cv::String ret = instance->getPathToApplication();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<bool> cv_CommandLineParser_has_const_const_StringR(const cv::CommandLineParser* instance, const char* name) {
		try {
			bool ret = instance->has(cv::String(name));
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_CommandLineParser_check_const(const cv::CommandLineParser* instance) {
		try {
			bool ret = instance->check();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_CommandLineParser_about_const_StringR(cv::CommandLineParser* instance, const char* message) {
		try {
			instance->about(cv::String(message));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_CommandLineParser_printMessage_const(const cv::CommandLineParser* instance) {
		try {
			instance->printMessage();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_CommandLineParser_printErrors_const(const cv::CommandLineParser* instance) {
		try {
			instance->printErrors();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::ConjGradSolver>*> cv_ConjGradSolver_create_const_Ptr_Function_R_TermCriteria(const cv::Ptr<cv::MinProblemSolver::Function>* f, cv::TermCriteria* termcrit) {
		try {
			cv::Ptr<cv::ConjGradSolver> ret = cv::ConjGradSolver::create(*f, *termcrit);
			return Ok(new cv::Ptr<cv::ConjGradSolver>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ConjGradSolver>*>))
	}
	
	Result<cv::DMatch> cv_DMatch_DMatch() {
		try {
			cv::DMatch ret;
			return Ok<cv::DMatch>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::DMatch>))
	}
	
	Result<cv::DMatch> cv_DMatch_DMatch_int_int_float(int _queryIdx, int _trainIdx, float _distance) {
		try {
			cv::DMatch ret(_queryIdx, _trainIdx, _distance);
			return Ok<cv::DMatch>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::DMatch>))
	}
	
	Result<cv::DMatch> cv_DMatch_DMatch_int_int_int_float(int _queryIdx, int _trainIdx, int _imgIdx, float _distance) {
		try {
			cv::DMatch ret(_queryIdx, _trainIdx, _imgIdx, _distance);
			return Ok<cv::DMatch>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::DMatch>))
	}
	
	Result_void cv_DownhillSolver_getInitStep_const_const__OutputArrayR(const cv::DownhillSolver* instance, const cv::_OutputArray* step) {
		try {
			instance->getInitStep(*step);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_DownhillSolver_setInitStep_const__InputArrayR(cv::DownhillSolver* instance, const cv::_InputArray* step) {
		try {
			instance->setInitStep(*step);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::DownhillSolver>*> cv_DownhillSolver_create_const_Ptr_Function_R_const__InputArrayR_TermCriteria(const cv::Ptr<cv::MinProblemSolver::Function>* f, const cv::_InputArray* initStep, cv::TermCriteria* termcrit) {
		try {
			cv::Ptr<cv::DownhillSolver> ret = cv::DownhillSolver::create(*f, *initStep, *termcrit);
			return Ok(new cv::Ptr<cv::DownhillSolver>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::DownhillSolver>*>))
	}
	
	Result<void*> cv_Exception_getPropMsg_const(const cv::Exception* instance) {
		try {
			cv::String ret = instance->msg;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_Exception_setPropMsg_String(cv::Exception* instance, char* val) {
		try {
			instance->msg = cv::String(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_Exception_getPropCode_const(const cv::Exception* instance) {
		try {
			int ret = instance->code;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_Exception_setPropCode_int(cv::Exception* instance, int val) {
		try {
			instance->code = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_Exception_getPropErr_const(const cv::Exception* instance) {
		try {
			cv::String ret = instance->err;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_Exception_setPropErr_String(cv::Exception* instance, char* val) {
		try {
			instance->err = cv::String(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_Exception_getPropFunc_const(const cv::Exception* instance) {
		try {
			cv::String ret = instance->func;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_Exception_setPropFunc_String(cv::Exception* instance, char* val) {
		try {
			instance->func = cv::String(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_Exception_getPropFile_const(const cv::Exception* instance) {
		try {
			cv::String ret = instance->file;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_Exception_setPropFile_String(cv::Exception* instance, char* val) {
		try {
			instance->file = cv::String(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_Exception_getPropLine_const(const cv::Exception* instance) {
		try {
			int ret = instance->line;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_Exception_setPropLine_int(cv::Exception* instance, int val) {
		try {
			instance->line = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Exception_delete(cv::Exception* instance) {
		delete instance;
	}
	Result<cv::Exception*> cv_Exception_Exception() {
		try {
			cv::Exception* ret = new cv::Exception();
			return Ok<cv::Exception*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Exception*>))
	}
	
	Result<cv::Exception*> cv_Exception_Exception_int_const_StringR_const_StringR_const_StringR_int(int _code, const char* _err, const char* _func, const char* _file, int _line) {
		try {
			cv::Exception* ret = new cv::Exception(_code, cv::String(_err), cv::String(_func), cv::String(_file), _line);
			return Ok<cv::Exception*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Exception*>))
	}
	
	Result<void*> cv_Exception_what_const(const cv::Exception* instance) {
		try {
			const char* ret = instance->what();
			return Ok(ocvrs_create_string(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_Exception_formatMessage(cv::Exception* instance) {
		try {
			instance->formatMessage();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_FileNode_delete(cv::FileNode* instance) {
		delete instance;
	}
	Result<cv::FileNode*> cv_FileNode_FileNode() {
		try {
			cv::FileNode* ret = new cv::FileNode();
			return Ok<cv::FileNode*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNode*>))
	}
	
	Result<cv::FileNode*> cv_FileNode_FileNode_const_FileNodeR(const cv::FileNode* node) {
		try {
			cv::FileNode* ret = new cv::FileNode(*node);
			return Ok<cv::FileNode*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNode*>))
	}
	
	Result<cv::FileNode*> cv_FileNode_operator___const_const_StringR(const cv::FileNode* instance, const char* nodename) {
		try {
			cv::FileNode ret = instance->operator[](cv::String(nodename));
			return Ok(new cv::FileNode(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNode*>))
	}
	
	Result<cv::FileNode*> cv_FileNode_operator___const_const_charX(const cv::FileNode* instance, const char* nodename) {
		try {
			cv::FileNode ret = instance->operator[](nodename);
			return Ok(new cv::FileNode(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNode*>))
	}
	
	Result<cv::FileNode*> cv_FileNode_operator___const_int(const cv::FileNode* instance, int i) {
		try {
			cv::FileNode ret = instance->operator[](i);
			return Ok(new cv::FileNode(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNode*>))
	}
	
	Result<int> cv_FileNode_type_const(const cv::FileNode* instance) {
		try {
			int ret = instance->type();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<bool> cv_FileNode_empty_const(const cv::FileNode* instance) {
		try {
			bool ret = instance->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_FileNode_isNone_const(const cv::FileNode* instance) {
		try {
			bool ret = instance->isNone();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_FileNode_isSeq_const(const cv::FileNode* instance) {
		try {
			bool ret = instance->isSeq();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_FileNode_isMap_const(const cv::FileNode* instance) {
		try {
			bool ret = instance->isMap();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_FileNode_isInt_const(const cv::FileNode* instance) {
		try {
			bool ret = instance->isInt();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_FileNode_isReal_const(const cv::FileNode* instance) {
		try {
			bool ret = instance->isReal();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_FileNode_isString_const(const cv::FileNode* instance) {
		try {
			bool ret = instance->isString();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_FileNode_isNamed_const(const cv::FileNode* instance) {
		try {
			bool ret = instance->isNamed();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<void*> cv_FileNode_name_const(const cv::FileNode* instance) {
		try {
			cv::String ret = instance->name();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<size_t> cv_FileNode_size_const(const cv::FileNode* instance) {
		try {
			size_t ret = instance->size();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<int> cv_FileNode_operator_int_const(const cv::FileNode* instance) {
		try {
			int ret = instance->operator int();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<float> cv_FileNode_operator_float_const(const cv::FileNode* instance) {
		try {
			float ret = instance->operator float();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result<double> cv_FileNode_operator_double_const(const cv::FileNode* instance) {
		try {
			double ret = instance->operator double();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<void*> cv_FileNode_operator_cv_String_const(const cv::FileNode* instance) {
		try {
			cv::String ret = instance->operator cv::String();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<void*> cv_FileNode_operator_std_string_const(const cv::FileNode* instance) {
		try {
			std::string ret = instance->operator std::string();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<cv::FileNodeIterator*> cv_FileNode_begin_const(const cv::FileNode* instance) {
		try {
			cv::FileNodeIterator ret = instance->begin();
			return Ok(new cv::FileNodeIterator(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNodeIterator*>))
	}
	
	Result<cv::FileNodeIterator*> cv_FileNode_end_const(const cv::FileNode* instance) {
		try {
			cv::FileNodeIterator ret = instance->end();
			return Ok(new cv::FileNodeIterator(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNodeIterator*>))
	}
	
	Result_void cv_FileNode_readRaw_const_const_StringR_unsigned_charX_size_t(const cv::FileNode* instance, const char* fmt, unsigned char* vec, size_t len) {
		try {
			instance->readRaw(cv::String(fmt), vec, len);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_FileNode_readObj_const(const cv::FileNode* instance) {
		try {
			void* ret = instance->readObj();
			return Ok<void*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<double> cv_FileNode_real_const(const cv::FileNode* instance) {
		try {
			double ret = instance->real();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<void*> cv_FileNode_string_const(const cv::FileNode* instance) {
		try {
			cv::String ret = instance->string();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<cv::Mat*> cv_FileNode_mat_const(const cv::FileNode* instance) {
		try {
			cv::Mat ret = instance->mat();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::FileNodeIterator::SeqReader*> cv_FileNodeIterator_getPropReader(cv::FileNodeIterator* instance) {
		try {
			cv::FileNodeIterator::SeqReader ret = instance->reader;
			return Ok(new cv::FileNodeIterator::SeqReader(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNodeIterator::SeqReader*>))
	}
	
	Result_void cv_FileNodeIterator_setPropReader_SeqReader(cv::FileNodeIterator* instance, cv::FileNodeIterator::SeqReader* val) {
		try {
			instance->reader = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<size_t> cv_FileNodeIterator_getPropRemaining_const(const cv::FileNodeIterator* instance) {
		try {
			size_t ret = instance->remaining;
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result_void cv_FileNodeIterator_setPropRemaining_size_t(cv::FileNodeIterator* instance, size_t val) {
		try {
			instance->remaining = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_FileNodeIterator_delete(cv::FileNodeIterator* instance) {
		delete instance;
	}
	Result<cv::FileNodeIterator*> cv_FileNodeIterator_FileNodeIterator() {
		try {
			cv::FileNodeIterator* ret = new cv::FileNodeIterator();
			return Ok<cv::FileNodeIterator*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNodeIterator*>))
	}
	
	Result<cv::FileNodeIterator*> cv_FileNodeIterator_FileNodeIterator_const_FileNodeIteratorR(const cv::FileNodeIterator* it) {
		try {
			cv::FileNodeIterator* ret = new cv::FileNodeIterator(*it);
			return Ok<cv::FileNodeIterator*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNodeIterator*>))
	}
	
	Result<cv::FileNode*> cv_FileNodeIterator_operatorX_const(const cv::FileNodeIterator* instance) {
		try {
			cv::FileNode ret = instance->operator*();
			return Ok(new cv::FileNode(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNode*>))
	}
	
	Result<cv::FileNodeIterator*> cv_FileNodeIterator_readRaw_const_StringR_unsigned_charX_size_t(cv::FileNodeIterator* instance, const char* fmt, unsigned char* vec, size_t maxCount) {
		try {
			cv::FileNodeIterator ret = instance->readRaw(cv::String(fmt), vec, maxCount);
			return Ok(new cv::FileNodeIterator(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNodeIterator*>))
	}
	
	Result<int> cv_FileNodeIterator_SeqReader_getPropHeader_size_const(const cv::FileNodeIterator::SeqReader* instance) {
		try {
			int ret = instance->header_size;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_FileNodeIterator_SeqReader_setPropHeader_size_int(cv::FileNodeIterator::SeqReader* instance, int val) {
		try {
			instance->header_size = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_FileNodeIterator_SeqReader_getPropSeq(cv::FileNodeIterator::SeqReader* instance) {
		try {
			void* ret = instance->seq;
			return Ok<void*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_FileNodeIterator_SeqReader_setPropSeq_voidX(cv::FileNodeIterator::SeqReader* instance, void* val) {
		try {
			instance->seq = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_FileNodeIterator_SeqReader_getPropBlock(cv::FileNodeIterator::SeqReader* instance) {
		try {
			void* ret = instance->block;
			return Ok<void*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_FileNodeIterator_SeqReader_setPropBlock_voidX(cv::FileNodeIterator::SeqReader* instance, void* val) {
		try {
			instance->block = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<signed char*> cv_FileNodeIterator_SeqReader_getPropPtr(cv::FileNodeIterator::SeqReader* instance) {
		try {
			signed char* ret = instance->ptr;
			return Ok<signed char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<signed char*>))
	}
	
	Result_void cv_FileNodeIterator_SeqReader_setPropPtr_signed_charX(cv::FileNodeIterator::SeqReader* instance, signed char* val) {
		try {
			instance->ptr = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<signed char*> cv_FileNodeIterator_SeqReader_getPropBlock_min(cv::FileNodeIterator::SeqReader* instance) {
		try {
			signed char* ret = instance->block_min;
			return Ok<signed char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<signed char*>))
	}
	
	Result_void cv_FileNodeIterator_SeqReader_setPropBlock_min_signed_charX(cv::FileNodeIterator::SeqReader* instance, signed char* val) {
		try {
			instance->block_min = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<signed char*> cv_FileNodeIterator_SeqReader_getPropBlock_max(cv::FileNodeIterator::SeqReader* instance) {
		try {
			signed char* ret = instance->block_max;
			return Ok<signed char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<signed char*>))
	}
	
	Result_void cv_FileNodeIterator_SeqReader_setPropBlock_max_signed_charX(cv::FileNodeIterator::SeqReader* instance, signed char* val) {
		try {
			instance->block_max = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_FileNodeIterator_SeqReader_getPropDelta_index_const(const cv::FileNodeIterator::SeqReader* instance) {
		try {
			int ret = instance->delta_index;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_FileNodeIterator_SeqReader_setPropDelta_index_int(cv::FileNodeIterator::SeqReader* instance, int val) {
		try {
			instance->delta_index = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<signed char*> cv_FileNodeIterator_SeqReader_getPropPrev_elem(cv::FileNodeIterator::SeqReader* instance) {
		try {
			signed char* ret = instance->prev_elem;
			return Ok<signed char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<signed char*>))
	}
	
	Result_void cv_FileNodeIterator_SeqReader_setPropPrev_elem_signed_charX(cv::FileNodeIterator::SeqReader* instance, signed char* val) {
		try {
			instance->prev_elem = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_FileNodeIterator_SeqReader_delete(cv::FileNodeIterator::SeqReader* instance) {
		delete instance;
	}
	Result<void*> cv_FileStorage_getPropElname_const(const cv::FileStorage* instance) {
		try {
			cv::String ret = instance->elname;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_FileStorage_setPropElname_String(cv::FileStorage* instance, char* val) {
		try {
			instance->elname = cv::String(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<char>*> cv_FileStorage_getPropStructs(cv::FileStorage* instance) {
		try {
			std::vector<char> ret = instance->structs;
			return Ok(new std::vector<char>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<char>*>))
	}
	
	Result_void cv_FileStorage_setPropStructs_vector_char_(cv::FileStorage* instance, std::vector<char>* val) {
		try {
			instance->structs = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_FileStorage_getPropState_const(const cv::FileStorage* instance) {
		try {
			int ret = instance->state;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_FileStorage_setPropState_int(cv::FileStorage* instance, int val) {
		try {
			instance->state = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_FileStorage_delete(cv::FileStorage* instance) {
		delete instance;
	}
	Result<cv::FileStorage*> cv_FileStorage_FileStorage() {
		try {
			cv::FileStorage* ret = new cv::FileStorage();
			return Ok<cv::FileStorage*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileStorage*>))
	}
	
	Result<cv::FileStorage*> cv_FileStorage_FileStorage_const_StringR_int_const_StringR(const char* source, int flags, const char* encoding) {
		try {
			cv::FileStorage* ret = new cv::FileStorage(cv::String(source), flags, cv::String(encoding));
			return Ok<cv::FileStorage*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileStorage*>))
	}
	
	Result<bool> cv_FileStorage_open_const_StringR_int_const_StringR(cv::FileStorage* instance, const char* filename, int flags, const char* encoding) {
		try {
			bool ret = instance->open(cv::String(filename), flags, cv::String(encoding));
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_FileStorage_isOpened_const(const cv::FileStorage* instance) {
		try {
			bool ret = instance->isOpened();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_FileStorage_release(cv::FileStorage* instance) {
		try {
			instance->release();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_FileStorage_releaseAndGetString(cv::FileStorage* instance) {
		try {
			cv::String ret = instance->releaseAndGetString();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<cv::FileNode*> cv_FileStorage_getFirstTopLevelNode_const(const cv::FileStorage* instance) {
		try {
			cv::FileNode ret = instance->getFirstTopLevelNode();
			return Ok(new cv::FileNode(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNode*>))
	}
	
	Result<cv::FileNode*> cv_FileStorage_root_const_int(const cv::FileStorage* instance, int streamidx) {
		try {
			cv::FileNode ret = instance->root(streamidx);
			return Ok(new cv::FileNode(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNode*>))
	}
	
	Result<cv::FileNode*> cv_FileStorage_operator___const_const_StringR(const cv::FileStorage* instance, const char* nodename) {
		try {
			cv::FileNode ret = instance->operator[](cv::String(nodename));
			return Ok(new cv::FileNode(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNode*>))
	}
	
	Result<cv::FileNode*> cv_FileStorage_operator___const_const_charX(const cv::FileStorage* instance, const char* nodename) {
		try {
			cv::FileNode ret = instance->operator[](nodename);
			return Ok(new cv::FileNode(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNode*>))
	}
	
	Result_void cv_FileStorage_writeRaw_const_StringR_const_unsigned_charX_size_t(cv::FileStorage* instance, const char* fmt, const unsigned char* vec, size_t len) {
		try {
			instance->writeRaw(cv::String(fmt), vec, len);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_FileStorage_writeObj_const_StringR_const_voidX(cv::FileStorage* instance, const char* name, const void* obj) {
		try {
			instance->writeObj(cv::String(name), obj);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_FileStorage_write_const_StringR_double(cv::FileStorage* instance, const char* name, double val) {
		try {
			instance->write(cv::String(name), val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_FileStorage_write_const_StringR_const_StringR(cv::FileStorage* instance, const char* name, const char* val) {
		try {
			instance->write(cv::String(name), cv::String(val));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_FileStorage_write_const_StringR_const__InputArrayR(cv::FileStorage* instance, const char* name, const cv::_InputArray* val) {
		try {
			instance->write(cv::String(name), *val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_FileStorage_writeComment_const_StringR_bool(cv::FileStorage* instance, const char* comment, bool append) {
		try {
			instance->writeComment(cv::String(comment), append);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_FileStorage_getDefaultObjectName_const_StringR(const char* filename) {
		try {
			cv::String ret = cv::FileStorage::getDefaultObjectName(cv::String(filename));
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<void*> cv_Formatted_next(cv::Formatted* instance) {
		try {
			const char* ret = instance->next();
			return Ok(ocvrs_create_string(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_Formatted_reset(cv::Formatted* instance) {
		try {
			instance->reset();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::Formatted>*> cv_Formatter_format_const_const_MatR(const cv::Formatter* instance, const cv::Mat* mtx) {
		try {
			cv::Ptr<cv::Formatted> ret = instance->format(*mtx);
			return Ok(new cv::Ptr<cv::Formatted>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::Formatted>*>))
	}
	
	Result_void cv_Formatter_set32fPrecision_int(cv::Formatter* instance, int p) {
		try {
			instance->set32fPrecision(p);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_Formatter_set64fPrecision_int(cv::Formatter* instance, int p) {
		try {
			instance->set64fPrecision(p);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_Formatter_setMultiline_bool(cv::Formatter* instance, bool ml) {
		try {
			instance->setMultiline(ml);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::Formatter>*> cv_Formatter_get_int(int fmt) {
		try {
			cv::Ptr<cv::Formatter> ret = cv::Formatter::get(fmt);
			return Ok(new cv::Ptr<cv::Formatter>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::Formatter>*>))
	}
	
	void cv_Hamming_delete(cv::Hamming* instance) {
		delete instance;
	}
	Result<cv::KeyPoint> cv_KeyPoint_KeyPoint() {
		try {
			cv::KeyPoint ret;
			return Ok<cv::KeyPoint>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::KeyPoint>))
	}
	
	Result<cv::KeyPoint> cv_KeyPoint_KeyPoint_Point2f_float_float_float_int_int(cv::Point2f* _pt, float _size, float _angle, float _response, int _octave, int _class_id) {
		try {
			cv::KeyPoint ret(*_pt, _size, _angle, _response, _octave, _class_id);
			return Ok<cv::KeyPoint>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::KeyPoint>))
	}
	
	Result<cv::KeyPoint> cv_KeyPoint_KeyPoint_float_float_float_float_float_int_int(float x, float y, float _size, float _angle, float _response, int _octave, int _class_id) {
		try {
			cv::KeyPoint ret(x, y, _size, _angle, _response, _octave, _class_id);
			return Ok<cv::KeyPoint>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::KeyPoint>))
	}
	
	Result<size_t> cv_KeyPoint_hash_const(const cv::KeyPoint instance) {
		try {
			size_t ret = instance.hash();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result_void cv_KeyPoint_convert_const_vector_KeyPoint_R_vector_Point2f_R_const_vector_int_R(const std::vector<cv::KeyPoint>* keypoints, std::vector<cv::Point2f>* points2f, const std::vector<int>* keypointIndexes) {
		try {
			cv::KeyPoint::convert(*keypoints, *points2f, *keypointIndexes);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_KeyPoint_convert_const_vector_Point2f_R_vector_KeyPoint_R_float_float_int_int(const std::vector<cv::Point2f>* points2f, std::vector<cv::KeyPoint>* keypoints, float size, float response, int octave, int class_id) {
		try {
			cv::KeyPoint::convert(*points2f, *keypoints, size, response, octave, class_id);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_KeyPoint_overlap_const_KeyPointR_const_KeyPointR(const cv::KeyPoint* kp1, const cv::KeyPoint* kp2) {
		try {
			float ret = cv::KeyPoint::overlap(*kp1, *kp2);
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	void cv_LDA_delete(cv::LDA* instance) {
		delete instance;
	}
	Result<cv::LDA*> cv_LDA_LDA_int(int num_components) {
		try {
			cv::LDA* ret = new cv::LDA(num_components);
			return Ok<cv::LDA*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::LDA*>))
	}
	
	Result<cv::LDA*> cv_LDA_LDA_const__InputArrayR_const__InputArrayR_int(const cv::_InputArray* src, const cv::_InputArray* labels, int num_components) {
		try {
			cv::LDA* ret = new cv::LDA(*src, *labels, num_components);
			return Ok<cv::LDA*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::LDA*>))
	}
	
	Result_void cv_LDA_save_const_const_StringR(const cv::LDA* instance, const char* filename) {
		try {
			instance->save(cv::String(filename));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_LDA_load_const_StringR(cv::LDA* instance, const char* filename) {
		try {
			instance->load(cv::String(filename));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_LDA_save_const_FileStorageR(const cv::LDA* instance, cv::FileStorage* fs) {
		try {
			instance->save(*fs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_LDA_load_const_FileStorageR(cv::LDA* instance, const cv::FileStorage* node) {
		try {
			instance->load(*node);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_LDA_compute_const__InputArrayR_const__InputArrayR(cv::LDA* instance, const cv::_InputArray* src, const cv::_InputArray* labels) {
		try {
			instance->compute(*src, *labels);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_LDA_project_const__InputArrayR(cv::LDA* instance, const cv::_InputArray* src) {
		try {
			cv::Mat ret = instance->project(*src);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_LDA_reconstruct_const__InputArrayR(cv::LDA* instance, const cv::_InputArray* src) {
		try {
			cv::Mat ret = instance->reconstruct(*src);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_LDA_eigenvectors_const(const cv::LDA* instance) {
		try {
			cv::Mat ret = instance->eigenvectors();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_LDA_eigenvalues_const(const cv::LDA* instance) {
		try {
			cv::Mat ret = instance->eigenvalues();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_LDA_subspaceProject_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* W, const cv::_InputArray* mean, const cv::_InputArray* src) {
		try {
			cv::Mat ret = cv::LDA::subspaceProject(*W, *mean, *src);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_LDA_subspaceReconstruct_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* W, const cv::_InputArray* mean, const cv::_InputArray* src) {
		try {
			cv::Mat ret = cv::LDA::subspaceReconstruct(*W, *mean, *src);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<int> cv_Mat_getPropFlags_const(const cv::Mat* instance) {
		try {
			int ret = instance->flags;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_Mat_setPropFlags_int(cv::Mat* instance, int val) {
		try {
			instance->flags = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_Mat_getPropDims_const(const cv::Mat* instance) {
		try {
			int ret = instance->dims;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_Mat_setPropDims_int(cv::Mat* instance, int val) {
		try {
			instance->dims = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_Mat_getPropRows_const(const cv::Mat* instance) {
		try {
			int ret = instance->rows;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_Mat_setPropRows_int(cv::Mat* instance, int val) {
		try {
			instance->rows = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_Mat_getPropCols_const(const cv::Mat* instance) {
		try {
			int ret = instance->cols;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_Mat_setPropCols_int(cv::Mat* instance, int val) {
		try {
			instance->cols = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<unsigned char*> cv_Mat_getPropData(cv::Mat* instance) {
		try {
			unsigned char* ret = instance->data;
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	Result_void cv_Mat_setPropData_unsigned_charX(cv::Mat* instance, unsigned char* val) {
		try {
			instance->data = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<const unsigned char*> cv_Mat_getPropDatastart_const(const cv::Mat* instance) {
		try {
			const unsigned char* ret = instance->datastart;
			return Ok<const unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const unsigned char*>))
	}
	
	Result<const unsigned char*> cv_Mat_getPropDataend_const(const cv::Mat* instance) {
		try {
			const unsigned char* ret = instance->dataend;
			return Ok<const unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const unsigned char*>))
	}
	
	Result<const unsigned char*> cv_Mat_getPropDatalimit_const(const cv::Mat* instance) {
		try {
			const unsigned char* ret = instance->datalimit;
			return Ok<const unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const unsigned char*>))
	}
	
	Result<cv::UMatData**> cv_Mat_getPropU(cv::Mat* instance) {
		try {
			cv::UMatData* ret = instance->u;
			return Ok(new cv::UMatData*(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMatData**>))
	}
	
	Result_void cv_Mat_setPropU_UMatDataX(cv::Mat* instance, cv::UMatData* val) {
		try {
			instance->u = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::MatSize*> cv_Mat_getPropSize_const(const cv::Mat* instance) {
		try {
			cv::MatSize ret = instance->size;
			return Ok(new cv::MatSize(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatSize*>))
	}
	
	Result<cv::MatStep*> cv_Mat_getPropStep_const(const cv::Mat* instance) {
		try {
			cv::MatStep ret = instance->step;
			return Ok(new cv::MatStep(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatStep*>))
	}
	
	void cv_Mat_delete(cv::Mat* instance) {
		delete instance;
	}
	Result<cv::Mat*> cv_Mat_Mat() {
		try {
			cv::Mat* ret = new cv::Mat();
			return Ok<cv::Mat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_Mat_Mat_int_int_int(int rows, int cols, int type) {
		try {
			cv::Mat* ret = new cv::Mat(rows, cols, type);
			return Ok<cv::Mat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_Mat_Mat_Size_int(cv::Size* size, int type) {
		try {
			cv::Mat* ret = new cv::Mat(*size, type);
			return Ok<cv::Mat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_Mat_Mat_int_int_int_const_ScalarR(int rows, int cols, int type, const cv::Scalar* s) {
		try {
			cv::Mat* ret = new cv::Mat(rows, cols, type, *s);
			return Ok<cv::Mat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_Mat_Mat_Size_int_const_ScalarR(cv::Size* size, int type, const cv::Scalar* s) {
		try {
			cv::Mat* ret = new cv::Mat(*size, type, *s);
			return Ok<cv::Mat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_Mat_Mat_int_const_intX_int(int ndims, const int* sizes, int type) {
		try {
			cv::Mat* ret = new cv::Mat(ndims, sizes, type);
			return Ok<cv::Mat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_Mat_Mat_const_vector_int_R_int(const std::vector<int>* sizes, int type) {
		try {
			cv::Mat* ret = new cv::Mat(*sizes, type);
			return Ok<cv::Mat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_Mat_Mat_int_const_intX_int_const_ScalarR(int ndims, const int* sizes, int type, const cv::Scalar* s) {
		try {
			cv::Mat* ret = new cv::Mat(ndims, sizes, type, *s);
			return Ok<cv::Mat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_Mat_Mat_const_vector_int_R_int_const_ScalarR(const std::vector<int>* sizes, int type, const cv::Scalar* s) {
		try {
			cv::Mat* ret = new cv::Mat(*sizes, type, *s);
			return Ok<cv::Mat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_Mat_Mat_const_MatR(const cv::Mat* m) {
		try {
			cv::Mat* ret = new cv::Mat(*m);
			return Ok<cv::Mat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_Mat_Mat_int_int_int_voidX_size_t(int rows, int cols, int type, void* data, size_t step) {
		try {
			cv::Mat* ret = new cv::Mat(rows, cols, type, data, step);
			return Ok<cv::Mat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_Mat_Mat_Size_int_voidX_size_t(cv::Size* size, int type, void* data, size_t step) {
		try {
			cv::Mat* ret = new cv::Mat(*size, type, data, step);
			return Ok<cv::Mat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_Mat_Mat_int_const_intX_int_voidX_const_size_tX(int ndims, const int* sizes, int type, void* data, const size_t* steps) {
		try {
			cv::Mat* ret = new cv::Mat(ndims, sizes, type, data, steps);
			return Ok<cv::Mat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_Mat_Mat_const_vector_int_R_int_voidX_const_size_tX(const std::vector<int>* sizes, int type, void* data, const size_t* steps) {
		try {
			cv::Mat* ret = new cv::Mat(*sizes, type, data, steps);
			return Ok<cv::Mat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_Mat_Mat_const_MatR_const_RangeR_const_RangeR(const cv::Mat* m, const cv::Range* rowRange, const cv::Range* colRange) {
		try {
			cv::Mat* ret = new cv::Mat(*m, *rowRange, *colRange);
			return Ok<cv::Mat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_Mat_Mat_const_MatR_const_RectR(const cv::Mat* m, const cv::Rect* roi) {
		try {
			cv::Mat* ret = new cv::Mat(*m, *roi);
			return Ok<cv::Mat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_Mat_Mat_const_MatR_const_vector_Range_R(const cv::Mat* m, const std::vector<cv::Range>* ranges) {
		try {
			cv::Mat* ret = new cv::Mat(*m, *ranges);
			return Ok<cv::Mat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_Mat_Mat_const_GpuMatR(const cv::cuda::GpuMat* m) {
		try {
			cv::Mat* ret = new cv::Mat(*m);
			return Ok<cv::Mat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::UMat*> cv_Mat_getUMat_const_int_UMatUsageFlags(const cv::Mat* instance, int accessFlags, cv::UMatUsageFlags usageFlags) {
		try {
			cv::UMat ret = instance->getUMat(accessFlags, usageFlags);
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::Mat*> cv_Mat_row_const_int(const cv::Mat* instance, int y) {
		try {
			cv::Mat ret = instance->row(y);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_Mat_col_const_int(const cv::Mat* instance, int x) {
		try {
			cv::Mat ret = instance->col(x);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_Mat_rowRange_const_int_int(const cv::Mat* instance, int startrow, int endrow) {
		try {
			cv::Mat ret = instance->rowRange(startrow, endrow);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_Mat_rowRange_const_const_RangeR(const cv::Mat* instance, const cv::Range* r) {
		try {
			cv::Mat ret = instance->rowRange(*r);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_Mat_colRange_const_int_int(const cv::Mat* instance, int startcol, int endcol) {
		try {
			cv::Mat ret = instance->colRange(startcol, endcol);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_Mat_colRange_const_const_RangeR(const cv::Mat* instance, const cv::Range* r) {
		try {
			cv::Mat ret = instance->colRange(*r);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_Mat_diag_const_int(const cv::Mat* instance, int d) {
		try {
			cv::Mat ret = instance->diag(d);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_Mat_diag_const_MatR(const cv::Mat* d) {
		try {
			cv::Mat ret = cv::Mat::diag(*d);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_Mat_clone_const(const cv::Mat* instance) {
		try {
			cv::Mat ret = instance->clone();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_Mat_copyTo_const_const__OutputArrayR(const cv::Mat* instance, const cv::_OutputArray* m) {
		try {
			instance->copyTo(*m);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_Mat_copyTo_const_const__OutputArrayR_const__InputArrayR(const cv::Mat* instance, const cv::_OutputArray* m, const cv::_InputArray* mask) {
		try {
			instance->copyTo(*m, *mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_Mat_convertTo_const_const__OutputArrayR_int_double_double(const cv::Mat* instance, const cv::_OutputArray* m, int rtype, double alpha, double beta) {
		try {
			instance->convertTo(*m, rtype, alpha, beta);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_Mat_assignTo_const_MatR_int(const cv::Mat* instance, cv::Mat* m, int type) {
		try {
			instance->assignTo(*m, type);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_Mat_setTo_const__InputArrayR_const__InputArrayR(cv::Mat* instance, const cv::_InputArray* value, const cv::_InputArray* mask) {
		try {
			cv::Mat ret = instance->setTo(*value, *mask);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_Mat_reshape_const_int_int(const cv::Mat* instance, int cn, int rows) {
		try {
			cv::Mat ret = instance->reshape(cn, rows);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_Mat_reshape_const_int_int_const_intX(const cv::Mat* instance, int cn, int newndims, const int* newsz) {
		try {
			cv::Mat ret = instance->reshape(cn, newndims, newsz);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::MatExpr*> cv_Mat_t_const(const cv::Mat* instance) {
		try {
			cv::MatExpr ret = instance->t();
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_Mat_inv_const_int(const cv::Mat* instance, int method) {
		try {
			cv::MatExpr ret = instance->inv(method);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_Mat_mul_const_const__InputArrayR_double(const cv::Mat* instance, const cv::_InputArray* m, double scale) {
		try {
			cv::MatExpr ret = instance->mul(*m, scale);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::Mat*> cv_Mat_cross_const_const__InputArrayR(const cv::Mat* instance, const cv::_InputArray* m) {
		try {
			cv::Mat ret = instance->cross(*m);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<double> cv_Mat_dot_const_const__InputArrayR(const cv::Mat* instance, const cv::_InputArray* m) {
		try {
			double ret = instance->dot(*m);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<cv::MatExpr*> cv_Mat_zeros_int_int_int(int rows, int cols, int type) {
		try {
			cv::MatExpr ret = cv::Mat::zeros(rows, cols, type);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_Mat_zeros_Size_int(cv::Size* size, int type) {
		try {
			cv::MatExpr ret = cv::Mat::zeros(*size, type);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_Mat_zeros_int_const_intX_int(int ndims, const int* sz, int type) {
		try {
			cv::MatExpr ret = cv::Mat::zeros(ndims, sz, type);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_Mat_ones_int_int_int(int rows, int cols, int type) {
		try {
			cv::MatExpr ret = cv::Mat::ones(rows, cols, type);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_Mat_ones_Size_int(cv::Size* size, int type) {
		try {
			cv::MatExpr ret = cv::Mat::ones(*size, type);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_Mat_ones_int_const_intX_int(int ndims, const int* sz, int type) {
		try {
			cv::MatExpr ret = cv::Mat::ones(ndims, sz, type);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_Mat_eye_int_int_int(int rows, int cols, int type) {
		try {
			cv::MatExpr ret = cv::Mat::eye(rows, cols, type);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_Mat_eye_Size_int(cv::Size* size, int type) {
		try {
			cv::MatExpr ret = cv::Mat::eye(*size, type);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result_void cv_Mat_create_int_int_int(cv::Mat* instance, int rows, int cols, int type) {
		try {
			instance->create(rows, cols, type);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_Mat_create_Size_int(cv::Mat* instance, cv::Size* size, int type) {
		try {
			instance->create(*size, type);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_Mat_create_int_const_intX_int(cv::Mat* instance, int ndims, const int* sizes, int type) {
		try {
			instance->create(ndims, sizes, type);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_Mat_create_const_vector_int_R_int(cv::Mat* instance, const std::vector<int>* sizes, int type) {
		try {
			instance->create(*sizes, type);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_Mat_addref(cv::Mat* instance) {
		try {
			instance->addref();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_Mat_release(cv::Mat* instance) {
		try {
			instance->release();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_Mat_deallocate(cv::Mat* instance) {
		try {
			instance->deallocate();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_Mat_reserve_size_t(cv::Mat* instance, size_t sz) {
		try {
			instance->reserve(sz);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_Mat_resize_size_t(cv::Mat* instance, size_t sz) {
		try {
			instance->resize(sz);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_Mat_resize_size_t_const_ScalarR(cv::Mat* instance, size_t sz, const cv::Scalar* s) {
		try {
			instance->resize(sz, *s);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_Mat_push_back_const_MatR(cv::Mat* instance, const cv::Mat* m) {
		try {
			instance->push_back(*m);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_Mat_pop_back_size_t(cv::Mat* instance, size_t nelems) {
		try {
			instance->pop_back(nelems);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_Mat_locateROI_const_SizeR_PointR(const cv::Mat* instance, cv::Size* wholeSize, cv::Point* ofs) {
		try {
			instance->locateROI(*wholeSize, *ofs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_Mat_adjustROI_int_int_int_int(cv::Mat* instance, int dtop, int dbottom, int dleft, int dright) {
		try {
			cv::Mat ret = instance->adjustROI(dtop, dbottom, dleft, dright);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<bool> cv_Mat_isContinuous_const(const cv::Mat* instance) {
		try {
			bool ret = instance->isContinuous();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_Mat_isSubmatrix_const(const cv::Mat* instance) {
		try {
			bool ret = instance->isSubmatrix();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<size_t> cv_Mat_elemSize_const(const cv::Mat* instance) {
		try {
			size_t ret = instance->elemSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<size_t> cv_Mat_elemSize1_const(const cv::Mat* instance) {
		try {
			size_t ret = instance->elemSize1();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<int> cv_Mat_type_const(const cv::Mat* instance) {
		try {
			int ret = instance->type();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_Mat_depth_const(const cv::Mat* instance) {
		try {
			int ret = instance->depth();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_Mat_channels_const(const cv::Mat* instance) {
		try {
			int ret = instance->channels();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<size_t> cv_Mat_step1_const_int(const cv::Mat* instance, int i) {
		try {
			size_t ret = instance->step1(i);
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<bool> cv_Mat_empty_const(const cv::Mat* instance) {
		try {
			bool ret = instance->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<size_t> cv_Mat_total_const(const cv::Mat* instance) {
		try {
			size_t ret = instance->total();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<int> cv_Mat_checkVector_const_int_int_bool(const cv::Mat* instance, int elemChannels, int depth, bool requireContinuous) {
		try {
			int ret = instance->checkVector(elemChannels, depth, requireContinuous);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<unsigned char*> cv_Mat_ptr_int(cv::Mat* instance, int i0) {
		try {
			unsigned char* ret = instance->ptr(i0);
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	Result<const unsigned char*> cv_Mat_ptr_const_int(const cv::Mat* instance, int i0) {
		try {
			const unsigned char* ret = instance->ptr(i0);
			return Ok<const unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const unsigned char*>))
	}
	
	Result<unsigned char*> cv_Mat_ptr_int_int(cv::Mat* instance, int row, int col) {
		try {
			unsigned char* ret = instance->ptr(row, col);
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	Result<const unsigned char*> cv_Mat_ptr_const_int_int(const cv::Mat* instance, int row, int col) {
		try {
			const unsigned char* ret = instance->ptr(row, col);
			return Ok<const unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const unsigned char*>))
	}
	
	Result<unsigned char*> cv_Mat_ptr_int_int_int(cv::Mat* instance, int i0, int i1, int i2) {
		try {
			unsigned char* ret = instance->ptr(i0, i1, i2);
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	Result<const unsigned char*> cv_Mat_ptr_const_int_int_int(const cv::Mat* instance, int i0, int i1, int i2) {
		try {
			const unsigned char* ret = instance->ptr(i0, i1, i2);
			return Ok<const unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const unsigned char*>))
	}
	
	Result<unsigned char*> cv_Mat_ptr_const_intX(cv::Mat* instance, const int* idx) {
		try {
			unsigned char* ret = instance->ptr(idx);
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	Result<const unsigned char*> cv_Mat_ptr_const_const_intX(const cv::Mat* instance, const int* idx) {
		try {
			const unsigned char* ret = instance->ptr(idx);
			return Ok<const unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const unsigned char*>))
	}
	
	Result<cv::Mat*> cv_Mat_Mat_MatR(cv::Mat* m) {
		try {
			cv::Mat* ret = new cv::Mat(*m);
			return Ok<cv::Mat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<const cv::Mat**> cv_MatConstIterator_getPropM_const(const cv::MatConstIterator* instance) {
		try {
			const cv::Mat* ret = instance->m;
			return Ok(new const cv::Mat*(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<const cv::Mat**>))
	}
	
	Result<size_t> cv_MatConstIterator_getPropElemSize_const(const cv::MatConstIterator* instance) {
		try {
			size_t ret = instance->elemSize;
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result_void cv_MatConstIterator_setPropElemSize_size_t(cv::MatConstIterator* instance, size_t val) {
		try {
			instance->elemSize = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<const unsigned char*> cv_MatConstIterator_getPropPtr_const(const cv::MatConstIterator* instance) {
		try {
			const unsigned char* ret = instance->ptr;
			return Ok<const unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const unsigned char*>))
	}
	
	Result<const unsigned char*> cv_MatConstIterator_getPropSliceStart_const(const cv::MatConstIterator* instance) {
		try {
			const unsigned char* ret = instance->sliceStart;
			return Ok<const unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const unsigned char*>))
	}
	
	Result<const unsigned char*> cv_MatConstIterator_getPropSliceEnd_const(const cv::MatConstIterator* instance) {
		try {
			const unsigned char* ret = instance->sliceEnd;
			return Ok<const unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const unsigned char*>))
	}
	
	void cv_MatConstIterator_delete(cv::MatConstIterator* instance) {
		delete instance;
	}
	Result<cv::MatConstIterator*> cv_MatConstIterator_MatConstIterator() {
		try {
			cv::MatConstIterator* ret = new cv::MatConstIterator();
			return Ok<cv::MatConstIterator*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatConstIterator*>))
	}
	
	Result<cv::MatConstIterator*> cv_MatConstIterator_MatConstIterator_const_MatX(const cv::Mat* _m) {
		try {
			cv::MatConstIterator* ret = new cv::MatConstIterator(_m);
			return Ok<cv::MatConstIterator*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatConstIterator*>))
	}
	
	Result<cv::MatConstIterator*> cv_MatConstIterator_MatConstIterator_const_MatX_int_int(const cv::Mat* _m, int _row, int _col) {
		try {
			cv::MatConstIterator* ret = new cv::MatConstIterator(_m, _row, _col);
			return Ok<cv::MatConstIterator*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatConstIterator*>))
	}
	
	Result<cv::MatConstIterator*> cv_MatConstIterator_MatConstIterator_const_MatX_Point(const cv::Mat* _m, cv::Point* _pt) {
		try {
			cv::MatConstIterator* ret = new cv::MatConstIterator(_m, *_pt);
			return Ok<cv::MatConstIterator*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatConstIterator*>))
	}
	
	#if !defined(OCVRS_TARGET_OS_WINDOWS)
	Result<cv::MatConstIterator*> cv_MatConstIterator_MatConstIterator_const_MatX_const_intX(const cv::Mat* _m, const int* _idx) {
		try {
			cv::MatConstIterator* ret = new cv::MatConstIterator(_m, _idx);
			return Ok<cv::MatConstIterator*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatConstIterator*>))
	}
	#endif
	
	Result<cv::MatConstIterator*> cv_MatConstIterator_MatConstIterator_const_MatConstIteratorR(const cv::MatConstIterator* it) {
		try {
			cv::MatConstIterator* ret = new cv::MatConstIterator(*it);
			return Ok<cv::MatConstIterator*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatConstIterator*>))
	}
	
	Result<const unsigned char*> cv_MatConstIterator_operatorX_const(const cv::MatConstIterator* instance) {
		try {
			const unsigned char* ret = instance->operator*();
			return Ok<const unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const unsigned char*>))
	}
	
	Result<const unsigned char*> cv_MatConstIterator_operator___const_ptrdiff_t(const cv::MatConstIterator* instance, ptrdiff_t i) {
		try {
			const unsigned char* ret = instance->operator[](i);
			return Ok<const unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const unsigned char*>))
	}
	
	Result<cv::Point> cv_MatConstIterator_pos_const(const cv::MatConstIterator* instance) {
		try {
			cv::Point ret = instance->pos();
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	Result_void cv_MatConstIterator_pos_const_intX(const cv::MatConstIterator* instance, int* _idx) {
		try {
			instance->pos(_idx);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<ptrdiff_t> cv_MatConstIterator_lpos_const(const cv::MatConstIterator* instance) {
		try {
			ptrdiff_t ret = instance->lpos();
			return Ok<ptrdiff_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<ptrdiff_t>))
	}
	
	Result_void cv_MatConstIterator_seek_ptrdiff_t_bool(cv::MatConstIterator* instance, ptrdiff_t ofs, bool relative) {
		try {
			instance->seek(ofs, relative);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_MatConstIterator_seek_const_intX_bool(cv::MatConstIterator* instance, const int* _idx, bool relative) {
		try {
			instance->seek(_idx, relative);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_MatExpr_getPropFlags_const(const cv::MatExpr* instance) {
		try {
			int ret = instance->flags;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_MatExpr_setPropFlags_int(cv::MatExpr* instance, int val) {
		try {
			instance->flags = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_MatExpr_getPropA(cv::MatExpr* instance) {
		try {
			cv::Mat ret = instance->a;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_MatExpr_setPropA_Mat(cv::MatExpr* instance, cv::Mat* val) {
		try {
			instance->a = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_MatExpr_getPropB(cv::MatExpr* instance) {
		try {
			cv::Mat ret = instance->b;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_MatExpr_setPropB_Mat(cv::MatExpr* instance, cv::Mat* val) {
		try {
			instance->b = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_MatExpr_getPropC(cv::MatExpr* instance) {
		try {
			cv::Mat ret = instance->c;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_MatExpr_setPropC_Mat(cv::MatExpr* instance, cv::Mat* val) {
		try {
			instance->c = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_MatExpr_getPropAlpha_const(const cv::MatExpr* instance) {
		try {
			double ret = instance->alpha;
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_MatExpr_setPropAlpha_double(cv::MatExpr* instance, double val) {
		try {
			instance->alpha = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_MatExpr_getPropBeta_const(const cv::MatExpr* instance) {
		try {
			double ret = instance->beta;
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_MatExpr_setPropBeta_double(cv::MatExpr* instance, double val) {
		try {
			instance->beta = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Scalar> cv_MatExpr_getPropS_const(const cv::MatExpr* instance) {
		try {
			cv::Scalar ret = instance->s;
			return Ok<cv::Scalar>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	Result_void cv_MatExpr_setPropS_Scalar(cv::MatExpr* instance, cv::Scalar* val) {
		try {
			instance->s = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_MatExpr_delete(cv::MatExpr* instance) {
		delete instance;
	}
	Result<cv::MatExpr*> cv_MatExpr_MatExpr() {
		try {
			cv::MatExpr* ret = new cv::MatExpr();
			return Ok<cv::MatExpr*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_MatExpr_MatExpr_const_MatR(const cv::Mat* m) {
		try {
			cv::MatExpr* ret = new cv::MatExpr(*m);
			return Ok<cv::MatExpr*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_MatExpr_MatExpr_const_MatOpX_int_const_MatR_const_MatR_const_MatR_double_double_const_ScalarR(const cv::MatOp* _op, int _flags, const cv::Mat* _a, const cv::Mat* _b, const cv::Mat* _c, double _alpha, double _beta, const cv::Scalar* _s) {
		try {
			cv::MatExpr* ret = new cv::MatExpr(_op, _flags, *_a, *_b, *_c, _alpha, _beta, *_s);
			return Ok<cv::MatExpr*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::Mat*> cv_MatExpr_operator_cv_Mat_const(const cv::MatExpr* instance) {
		try {
			cv::Mat ret = instance->operator cv::Mat();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Size> cv_MatExpr_size_const(const cv::MatExpr* instance) {
		try {
			cv::Size ret = instance->size();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result<int> cv_MatExpr_type_const(const cv::MatExpr* instance) {
		try {
			int ret = instance->type();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<cv::MatExpr*> cv_MatExpr_row_const_int(const cv::MatExpr* instance, int y) {
		try {
			cv::MatExpr ret = instance->row(y);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_MatExpr_col_const_int(const cv::MatExpr* instance, int x) {
		try {
			cv::MatExpr ret = instance->col(x);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_MatExpr_diag_const_int(const cv::MatExpr* instance, int d) {
		try {
			cv::MatExpr ret = instance->diag(d);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_MatExpr_t_const(const cv::MatExpr* instance) {
		try {
			cv::MatExpr ret = instance->t();
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_MatExpr_inv_const_int(const cv::MatExpr* instance, int method) {
		try {
			cv::MatExpr ret = instance->inv(method);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_MatExpr_mul_const_const_MatExprR_double(const cv::MatExpr* instance, const cv::MatExpr* e, double scale) {
		try {
			cv::MatExpr ret = instance->mul(*e, scale);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::MatExpr*> cv_MatExpr_mul_const_const_MatR_double(const cv::MatExpr* instance, const cv::Mat* m, double scale) {
		try {
			cv::MatExpr ret = instance->mul(*m, scale);
			return Ok(new cv::MatExpr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	Result<cv::Mat*> cv_MatExpr_cross_const_const_MatR(const cv::MatExpr* instance, const cv::Mat* m) {
		try {
			cv::Mat ret = instance->cross(*m);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<double> cv_MatExpr_dot_const_const_MatR(const cv::MatExpr* instance, const cv::Mat* m) {
		try {
			double ret = instance->dot(*m);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<bool> cv_MatOp_elementWise_const_const_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr) {
		try {
			bool ret = instance->elementWise(*expr);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_MatOp_assign_const_const_MatExprR_MatR_int(const cv::MatOp* instance, const cv::MatExpr* expr, cv::Mat* m, int type) {
		try {
			instance->assign(*expr, *m, type);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_MatOp_roi_const_const_MatExprR_const_RangeR_const_RangeR_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr, const cv::Range* rowRange, const cv::Range* colRange, cv::MatExpr* res) {
		try {
			instance->roi(*expr, *rowRange, *colRange, *res);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_MatOp_diag_const_const_MatExprR_int_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr, int d, cv::MatExpr* res) {
		try {
			instance->diag(*expr, d, *res);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_MatOp_augAssignAdd_const_const_MatExprR_MatR(const cv::MatOp* instance, const cv::MatExpr* expr, cv::Mat* m) {
		try {
			instance->augAssignAdd(*expr, *m);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_MatOp_augAssignSubtract_const_const_MatExprR_MatR(const cv::MatOp* instance, const cv::MatExpr* expr, cv::Mat* m) {
		try {
			instance->augAssignSubtract(*expr, *m);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_MatOp_augAssignMultiply_const_const_MatExprR_MatR(const cv::MatOp* instance, const cv::MatExpr* expr, cv::Mat* m) {
		try {
			instance->augAssignMultiply(*expr, *m);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_MatOp_augAssignDivide_const_const_MatExprR_MatR(const cv::MatOp* instance, const cv::MatExpr* expr, cv::Mat* m) {
		try {
			instance->augAssignDivide(*expr, *m);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_MatOp_augAssignAnd_const_const_MatExprR_MatR(const cv::MatOp* instance, const cv::MatExpr* expr, cv::Mat* m) {
		try {
			instance->augAssignAnd(*expr, *m);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_MatOp_augAssignOr_const_const_MatExprR_MatR(const cv::MatOp* instance, const cv::MatExpr* expr, cv::Mat* m) {
		try {
			instance->augAssignOr(*expr, *m);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_MatOp_augAssignXor_const_const_MatExprR_MatR(const cv::MatOp* instance, const cv::MatExpr* expr, cv::Mat* m) {
		try {
			instance->augAssignXor(*expr, *m);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_MatOp_add_const_const_MatExprR_const_MatExprR_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr1, const cv::MatExpr* expr2, cv::MatExpr* res) {
		try {
			instance->add(*expr1, *expr2, *res);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_MatOp_add_const_const_MatExprR_const_ScalarR_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr1, const cv::Scalar* s, cv::MatExpr* res) {
		try {
			instance->add(*expr1, *s, *res);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_MatOp_subtract_const_const_MatExprR_const_MatExprR_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr1, const cv::MatExpr* expr2, cv::MatExpr* res) {
		try {
			instance->subtract(*expr1, *expr2, *res);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_MatOp_subtract_const_const_ScalarR_const_MatExprR_MatExprR(const cv::MatOp* instance, const cv::Scalar* s, const cv::MatExpr* expr, cv::MatExpr* res) {
		try {
			instance->subtract(*s, *expr, *res);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_MatOp_multiply_const_const_MatExprR_const_MatExprR_MatExprR_double(const cv::MatOp* instance, const cv::MatExpr* expr1, const cv::MatExpr* expr2, cv::MatExpr* res, double scale) {
		try {
			instance->multiply(*expr1, *expr2, *res, scale);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_MatOp_multiply_const_const_MatExprR_double_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr1, double s, cv::MatExpr* res) {
		try {
			instance->multiply(*expr1, s, *res);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_MatOp_divide_const_const_MatExprR_const_MatExprR_MatExprR_double(const cv::MatOp* instance, const cv::MatExpr* expr1, const cv::MatExpr* expr2, cv::MatExpr* res, double scale) {
		try {
			instance->divide(*expr1, *expr2, *res, scale);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_MatOp_divide_const_double_const_MatExprR_MatExprR(const cv::MatOp* instance, double s, const cv::MatExpr* expr, cv::MatExpr* res) {
		try {
			instance->divide(s, *expr, *res);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_MatOp_abs_const_const_MatExprR_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr, cv::MatExpr* res) {
		try {
			instance->abs(*expr, *res);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_MatOp_transpose_const_const_MatExprR_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr, cv::MatExpr* res) {
		try {
			instance->transpose(*expr, *res);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_MatOp_matmul_const_const_MatExprR_const_MatExprR_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr1, const cv::MatExpr* expr2, cv::MatExpr* res) {
		try {
			instance->matmul(*expr1, *expr2, *res);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_MatOp_invert_const_const_MatExprR_int_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr, int method, cv::MatExpr* res) {
		try {
			instance->invert(*expr, method, *res);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_MatOp_size_const_const_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr) {
		try {
			cv::Size ret = instance->size(*expr);
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result<int> cv_MatOp_type_const_const_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr) {
		try {
			int ret = instance->type(*expr);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int*> cv_MatSize_getPropP(cv::MatSize* instance) {
		try {
			int* ret = instance->p;
			return Ok<int*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int*>))
	}
	
	Result_void cv_MatSize_setPropP_intX(cv::MatSize* instance, int* val) {
		try {
			instance->p = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_MatSize_delete(cv::MatSize* instance) {
		delete instance;
	}
	Result<cv::MatSize*> cv_MatSize_MatSize_intX(int* _p) {
		try {
			cv::MatSize* ret = new cv::MatSize(_p);
			return Ok<cv::MatSize*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatSize*>))
	}
	
	Result<const int> cv_MatSize_operator___const_int(const cv::MatSize* instance, int i) {
		try {
			const int ret = instance->operator[](i);
			return Ok<const int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const int>))
	}
	
	Result<int> cv_MatSize_operator___int(cv::MatSize* instance, int i) {
		try {
			int ret = instance->operator[](i);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<const int*> cv_MatSize_operator_const_intX_const(const cv::MatSize* instance) {
		try {
			const int* ret = instance->operator const int*();
			return Ok<const int*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const int*>))
	}
	
	Result<size_t*> cv_MatStep_getPropP(cv::MatStep* instance) {
		try {
			size_t* ret = instance->p;
			return Ok<size_t*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t*>))
	}
	
	Result_void cv_MatStep_setPropP_size_tX(cv::MatStep* instance, size_t* val) {
		try {
			instance->p = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<size_t(*)[2]> cv_MatStep_getPropBuf(cv::MatStep* instance) {
		try {
			size_t(*ret)[2] = &instance->buf;
			return Ok<size_t(*)[2]>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t(*)[2]>))
	}
	
	void cv_MatStep_delete(cv::MatStep* instance) {
		delete instance;
	}
	Result<cv::MatStep*> cv_MatStep_MatStep() {
		try {
			cv::MatStep* ret = new cv::MatStep();
			return Ok<cv::MatStep*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatStep*>))
	}
	
	Result<cv::MatStep*> cv_MatStep_MatStep_size_t(size_t s) {
		try {
			cv::MatStep* ret = new cv::MatStep(s);
			return Ok<cv::MatStep*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatStep*>))
	}
	
	Result<const size_t> cv_MatStep_operator___const_int(const cv::MatStep* instance, int i) {
		try {
			const size_t ret = instance->operator[](i);
			return Ok<const size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const size_t>))
	}
	
	Result<size_t> cv_MatStep_operator___int(cv::MatStep* instance, int i) {
		try {
			size_t ret = instance->operator[](i);
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<size_t> cv_MatStep_operator_size_t_const(const cv::MatStep* instance) {
		try {
			size_t ret = instance->operator size_t();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	void cv_Matx_AddOp_delete(cv::Matx_AddOp* instance) {
		delete instance;
	}
	void cv_Matx_DivOp_delete(cv::Matx_DivOp* instance) {
		delete instance;
	}
	void cv_Matx_MatMulOp_delete(cv::Matx_MatMulOp* instance) {
		delete instance;
	}
	void cv_Matx_MulOp_delete(cv::Matx_MulOp* instance) {
		delete instance;
	}
	void cv_Matx_ScaleOp_delete(cv::Matx_ScaleOp* instance) {
		delete instance;
	}
	void cv_Matx_SubOp_delete(cv::Matx_SubOp* instance) {
		delete instance;
	}
	void cv_Matx_TOp_delete(cv::Matx_TOp* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::MinProblemSolver::Function>*> cv_MinProblemSolver_getFunction_const(const cv::MinProblemSolver* instance) {
		try {
			cv::Ptr<cv::MinProblemSolver::Function> ret = instance->getFunction();
			return Ok(new cv::Ptr<cv::MinProblemSolver::Function>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::MinProblemSolver::Function>*>))
	}
	
	Result_void cv_MinProblemSolver_setFunction_const_Ptr_Function_R(cv::MinProblemSolver* instance, const cv::Ptr<cv::MinProblemSolver::Function>* f) {
		try {
			instance->setFunction(*f);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::TermCriteria> cv_MinProblemSolver_getTermCriteria_const(const cv::MinProblemSolver* instance) {
		try {
			cv::TermCriteria ret = instance->getTermCriteria();
			return Ok<cv::TermCriteria>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TermCriteria>))
	}
	
	Result_void cv_MinProblemSolver_setTermCriteria_const_TermCriteriaR(cv::MinProblemSolver* instance, const cv::TermCriteria* termcrit) {
		try {
			instance->setTermCriteria(*termcrit);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_MinProblemSolver_minimize_const__InputOutputArrayR(cv::MinProblemSolver* instance, const cv::_InputOutputArray* x) {
		try {
			double ret = instance->minimize(*x);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<int> cv_MinProblemSolver_Function_getDims_const(const cv::MinProblemSolver::Function* instance) {
		try {
			int ret = instance->getDims();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<double> cv_MinProblemSolver_Function_getGradientEps_const(const cv::MinProblemSolver::Function* instance) {
		try {
			double ret = instance->getGradientEps();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<double> cv_MinProblemSolver_Function_calc_const_const_doubleX(const cv::MinProblemSolver::Function* instance, const double* x) {
		try {
			double ret = instance->calc(x);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_MinProblemSolver_Function_getGradient_const_doubleX_doubleX(cv::MinProblemSolver::Function* instance, const double* x, double* grad) {
		try {
			instance->getGradient(x, grad);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Moments> cv_Moments_Moments() {
		try {
			cv::Moments ret;
			return Ok<cv::Moments>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Moments>))
	}
	
	Result<cv::Moments> cv_Moments_Moments_double_double_double_double_double_double_double_double_double_double(double m00, double m10, double m01, double m20, double m11, double m02, double m30, double m21, double m12, double m03) {
		try {
			cv::Moments ret(m00, m10, m01, m20, m11, m02, m30, m21, m12, m03);
			return Ok<cv::Moments>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Moments>))
	}
	
	void cv_Mutex_delete(cv::Mutex* instance) {
		delete instance;
	}
	Result<cv::Mutex*> cv_Mutex_Mutex() {
		try {
			cv::Mutex* ret = new cv::Mutex();
			return Ok<cv::Mutex*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mutex*>))
	}
	
	Result<cv::Mutex*> cv_Mutex_Mutex_const_MutexR(const cv::Mutex* m) {
		try {
			cv::Mutex* ret = new cv::Mutex(*m);
			return Ok<cv::Mutex*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mutex*>))
	}
	
	Result_void cv_Mutex_lock(cv::Mutex* instance) {
		try {
			instance->lock();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_Mutex_trylock(cv::Mutex* instance) {
		try {
			bool ret = instance->trylock();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_Mutex_unlock(cv::Mutex* instance) {
		try {
			instance->unlock();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_PCA_getPropEigenvectors(cv::PCA* instance) {
		try {
			cv::Mat ret = instance->eigenvectors;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_PCA_setPropEigenvectors_Mat(cv::PCA* instance, cv::Mat* val) {
		try {
			instance->eigenvectors = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_PCA_getPropEigenvalues(cv::PCA* instance) {
		try {
			cv::Mat ret = instance->eigenvalues;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_PCA_setPropEigenvalues_Mat(cv::PCA* instance, cv::Mat* val) {
		try {
			instance->eigenvalues = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_PCA_getPropMean(cv::PCA* instance) {
		try {
			cv::Mat ret = instance->mean;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_PCA_setPropMean_Mat(cv::PCA* instance, cv::Mat* val) {
		try {
			instance->mean = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_PCA_delete(cv::PCA* instance) {
		delete instance;
	}
	Result<cv::PCA*> cv_PCA_PCA() {
		try {
			cv::PCA* ret = new cv::PCA();
			return Ok<cv::PCA*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::PCA*>))
	}
	
	Result<cv::PCA*> cv_PCA_PCA_const__InputArrayR_const__InputArrayR_int_int(const cv::_InputArray* data, const cv::_InputArray* mean, int flags, int maxComponents) {
		try {
			cv::PCA* ret = new cv::PCA(*data, *mean, flags, maxComponents);
			return Ok<cv::PCA*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::PCA*>))
	}
	
	Result<cv::PCA*> cv_PCA_PCA_const__InputArrayR_const__InputArrayR_int_double(const cv::_InputArray* data, const cv::_InputArray* mean, int flags, double retainedVariance) {
		try {
			cv::PCA* ret = new cv::PCA(*data, *mean, flags, retainedVariance);
			return Ok<cv::PCA*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::PCA*>))
	}
	
	Result<cv::Mat*> cv_PCA_project_const_const__InputArrayR(const cv::PCA* instance, const cv::_InputArray* vec) {
		try {
			cv::Mat ret = instance->project(*vec);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_PCA_project_const_const__InputArrayR_const__OutputArrayR(const cv::PCA* instance, const cv::_InputArray* vec, const cv::_OutputArray* result) {
		try {
			instance->project(*vec, *result);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_PCA_backProject_const_const__InputArrayR(const cv::PCA* instance, const cv::_InputArray* vec) {
		try {
			cv::Mat ret = instance->backProject(*vec);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_PCA_backProject_const_const__InputArrayR_const__OutputArrayR(const cv::PCA* instance, const cv::_InputArray* vec, const cv::_OutputArray* result) {
		try {
			instance->backProject(*vec, *result);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_PCA_write_const_FileStorageR(const cv::PCA* instance, cv::FileStorage* fs) {
		try {
			instance->write(*fs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_PCA_read_const_FileNodeR(cv::PCA* instance, const cv::FileNode* fn) {
		try {
			instance->read(*fn);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<uint64_t> cv_RNG_getPropState_const(const cv::RNG* instance) {
		try {
			uint64_t ret = instance->state;
			return Ok<uint64_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<uint64_t>))
	}
	
	Result_void cv_RNG_setPropState_uint64_t(cv::RNG* instance, uint64_t val) {
		try {
			instance->state = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_RNG_delete(cv::RNG* instance) {
		delete instance;
	}
	Result<cv::RNG*> cv_RNG_RNG() {
		try {
			cv::RNG* ret = new cv::RNG();
			return Ok<cv::RNG*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::RNG*>))
	}
	
	Result<cv::RNG*> cv_RNG_RNG_uint64_t(uint64_t state) {
		try {
			cv::RNG* ret = new cv::RNG(state);
			return Ok<cv::RNG*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::RNG*>))
	}
	
	Result<unsigned int> cv_RNG_next(cv::RNG* instance) {
		try {
			unsigned int ret = instance->next();
			return Ok<unsigned int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned int>))
	}
	
	Result<unsigned char> cv_RNG_operator_unsigned_char(cv::RNG* instance) {
		try {
			unsigned char ret = instance->operator unsigned char();
			return Ok<unsigned char>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char>))
	}
	
	Result<signed char> cv_RNG_operator_signed_char(cv::RNG* instance) {
		try {
			signed char ret = instance->operator signed char();
			return Ok<signed char>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<signed char>))
	}
	
	Result<unsigned short> cv_RNG_operator_unsigned_short(cv::RNG* instance) {
		try {
			unsigned short ret = instance->operator unsigned short();
			return Ok<unsigned short>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned short>))
	}
	
	Result<short> cv_RNG_operator_short(cv::RNG* instance) {
		try {
			short ret = instance->operator short();
			return Ok<short>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<short>))
	}
	
	Result<unsigned int> cv_RNG_operator_unsigned_int(cv::RNG* instance) {
		try {
			unsigned int ret = instance->operator unsigned int();
			return Ok<unsigned int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned int>))
	}
	
	Result<int> cv_RNG_operator_int(cv::RNG* instance) {
		try {
			int ret = instance->operator int();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<float> cv_RNG_operator_float(cv::RNG* instance) {
		try {
			float ret = instance->operator float();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result<double> cv_RNG_operator_double(cv::RNG* instance) {
		try {
			double ret = instance->operator double();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<int> cv_RNG_uniform_int_int(cv::RNG* instance, int a, int b) {
		try {
			int ret = instance->uniform(a, b);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<float> cv_RNG_uniform_float_float(cv::RNG* instance, float a, float b) {
		try {
			float ret = instance->uniform(a, b);
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result<double> cv_RNG_uniform_double_double(cv::RNG* instance, double a, double b) {
		try {
			double ret = instance->uniform(a, b);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_RNG_fill_const__InputOutputArrayR_int_const__InputArrayR_const__InputArrayR_bool(cv::RNG* instance, const cv::_InputOutputArray* mat, int distType, const cv::_InputArray* a, const cv::_InputArray* b, bool saturateRange) {
		try {
			instance->fill(*mat, distType, *a, *b, saturateRange);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_RNG_gaussian_double(cv::RNG* instance, double sigma) {
		try {
			double ret = instance->gaussian(sigma);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	void cv_RNG_MT19937_delete(cv::RNG_MT19937* instance) {
		delete instance;
	}
	Result<cv::RNG_MT19937*> cv_RNG_MT19937_RNG_MT19937() {
		try {
			cv::RNG_MT19937* ret = new cv::RNG_MT19937();
			return Ok<cv::RNG_MT19937*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::RNG_MT19937*>))
	}
	
	Result<cv::RNG_MT19937*> cv_RNG_MT19937_RNG_MT19937_unsigned_int(unsigned int s) {
		try {
			cv::RNG_MT19937* ret = new cv::RNG_MT19937(s);
			return Ok<cv::RNG_MT19937*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::RNG_MT19937*>))
	}
	
	Result_void cv_RNG_MT19937_seed_unsigned_int(cv::RNG_MT19937* instance, unsigned int s) {
		try {
			instance->seed(s);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<unsigned int> cv_RNG_MT19937_next(cv::RNG_MT19937* instance) {
		try {
			unsigned int ret = instance->next();
			return Ok<unsigned int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned int>))
	}
	
	Result<int> cv_RNG_MT19937_operator_int(cv::RNG_MT19937* instance) {
		try {
			int ret = instance->operator int();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<unsigned int> cv_RNG_MT19937_operator_unsigned_int(cv::RNG_MT19937* instance) {
		try {
			unsigned int ret = instance->operator unsigned int();
			return Ok<unsigned int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned int>))
	}
	
	Result<float> cv_RNG_MT19937_operator_float(cv::RNG_MT19937* instance) {
		try {
			float ret = instance->operator float();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result<double> cv_RNG_MT19937_operator_double(cv::RNG_MT19937* instance) {
		try {
			double ret = instance->operator double();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<int> cv_RNG_MT19937_uniform_int_int(cv::RNG_MT19937* instance, int a, int b) {
		try {
			int ret = instance->uniform(a, b);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<float> cv_RNG_MT19937_uniform_float_float(cv::RNG_MT19937* instance, float a, float b) {
		try {
			float ret = instance->uniform(a, b);
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result<double> cv_RNG_MT19937_uniform_double_double(cv::RNG_MT19937* instance, double a, double b) {
		try {
			double ret = instance->uniform(a, b);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<int> cv_Range_getPropStart_const(const cv::Range* instance) {
		try {
			int ret = instance->start;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_Range_setPropStart_int(cv::Range* instance, int val) {
		try {
			instance->start = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_Range_getPropEnd_const(const cv::Range* instance) {
		try {
			int ret = instance->end;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_Range_setPropEnd_int(cv::Range* instance, int val) {
		try {
			instance->end = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Range_delete(cv::Range* instance) {
		delete instance;
	}
	Result<cv::Range*> cv_Range_Range() {
		try {
			cv::Range* ret = new cv::Range();
			return Ok<cv::Range*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Range*>))
	}
	
	Result<cv::Range*> cv_Range_Range_int_int(int _start, int _end) {
		try {
			cv::Range* ret = new cv::Range(_start, _end);
			return Ok<cv::Range*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Range*>))
	}
	
	Result<int> cv_Range_size_const(const cv::Range* instance) {
		try {
			int ret = instance->size();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<bool> cv_Range_empty_const(const cv::Range* instance) {
		try {
			bool ret = instance->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<cv::Range*> cv_Range_all() {
		try {
			cv::Range ret = cv::Range::all();
			return Ok(new cv::Range(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Range*>))
	}
	
	Result<cv::Point2f> cv_RotatedRect_getPropCenter_const(const cv::RotatedRect* instance) {
		try {
			cv::Point2f ret = instance->center;
			return Ok<cv::Point2f>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point2f>))
	}
	
	Result_void cv_RotatedRect_setPropCenter_Point2f(cv::RotatedRect* instance, cv::Point2f* val) {
		try {
			instance->center = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size2f> cv_RotatedRect_getPropSize_const(const cv::RotatedRect* instance) {
		try {
			cv::Size2f ret = instance->size;
			return Ok<cv::Size2f>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size2f>))
	}
	
	Result_void cv_RotatedRect_setPropSize_Size2f(cv::RotatedRect* instance, cv::Size2f* val) {
		try {
			instance->size = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_RotatedRect_getPropAngle_const(const cv::RotatedRect* instance) {
		try {
			float ret = instance->angle;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_RotatedRect_setPropAngle_float(cv::RotatedRect* instance, float val) {
		try {
			instance->angle = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_RotatedRect_delete(cv::RotatedRect* instance) {
		delete instance;
	}
	Result<cv::RotatedRect*> cv_RotatedRect_RotatedRect() {
		try {
			cv::RotatedRect* ret = new cv::RotatedRect();
			return Ok<cv::RotatedRect*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::RotatedRect*>))
	}
	
	Result<cv::RotatedRect*> cv_RotatedRect_RotatedRect_const_Point2fR_const_Size2fR_float(const cv::Point2f* center, const cv::Size2f* size, float angle) {
		try {
			cv::RotatedRect* ret = new cv::RotatedRect(*center, *size, angle);
			return Ok<cv::RotatedRect*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::RotatedRect*>))
	}
	
	Result<cv::RotatedRect*> cv_RotatedRect_RotatedRect_const_Point2fR_const_Point2fR_const_Point2fR(const cv::Point2f* point1, const cv::Point2f* point2, const cv::Point2f* point3) {
		try {
			cv::RotatedRect* ret = new cv::RotatedRect(*point1, *point2, *point3);
			return Ok<cv::RotatedRect*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::RotatedRect*>))
	}
	
	Result_void cv_RotatedRect_points_const_Point2fX(const cv::RotatedRect* instance, cv::Point2f* pts) {
		try {
			instance->points(pts);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Rect> cv_RotatedRect_boundingRect_const(const cv::RotatedRect* instance) {
		try {
			cv::Rect ret = instance->boundingRect();
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	Result<cv::Rect_<float>> cv_RotatedRect_boundingRect2f_const(const cv::RotatedRect* instance) {
		try {
			cv::Rect_<float> ret = instance->boundingRect2f();
			return Ok<cv::Rect_<float>>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect_<float>>))
	}
	
	Result<cv::Mat*> cv_SVD_getPropU(cv::SVD* instance) {
		try {
			cv::Mat ret = instance->u;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_SVD_setPropU_Mat(cv::SVD* instance, cv::Mat* val) {
		try {
			instance->u = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_SVD_getPropW(cv::SVD* instance) {
		try {
			cv::Mat ret = instance->w;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_SVD_setPropW_Mat(cv::SVD* instance, cv::Mat* val) {
		try {
			instance->w = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_SVD_getPropVt(cv::SVD* instance) {
		try {
			cv::Mat ret = instance->vt;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_SVD_setPropVt_Mat(cv::SVD* instance, cv::Mat* val) {
		try {
			instance->vt = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_SVD_delete(cv::SVD* instance) {
		delete instance;
	}
	Result<cv::SVD*> cv_SVD_SVD() {
		try {
			cv::SVD* ret = new cv::SVD();
			return Ok<cv::SVD*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SVD*>))
	}
	
	Result<cv::SVD*> cv_SVD_SVD_const__InputArrayR_int(const cv::_InputArray* src, int flags) {
		try {
			cv::SVD* ret = new cv::SVD(*src, flags);
			return Ok<cv::SVD*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SVD*>))
	}
	
	Result_void cv_SVD_compute_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* w, const cv::_OutputArray* u, const cv::_OutputArray* vt, int flags) {
		try {
			cv::SVD::compute(*src, *w, *u, *vt, flags);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_SVD_compute_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* w, int flags) {
		try {
			cv::SVD::compute(*src, *w, flags);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_SVD_backSubst_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* w, const cv::_InputArray* u, const cv::_InputArray* vt, const cv::_InputArray* rhs, const cv::_OutputArray* dst) {
		try {
			cv::SVD::backSubst(*w, *u, *vt, *rhs, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_SVD_solveZ_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst) {
		try {
			cv::SVD::solveZ(*src, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_SVD_backSubst_const_const__InputArrayR_const__OutputArrayR(const cv::SVD* instance, const cv::_InputArray* rhs, const cv::_OutputArray* dst) {
		try {
			instance->backSubst(*rhs, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_SparseMat_getPropFlags_const(const cv::SparseMat* instance) {
		try {
			int ret = instance->flags;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_SparseMat_setPropFlags_int(cv::SparseMat* instance, int val) {
		try {
			instance->flags = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::SparseMat::Hdr**> cv_SparseMat_getPropHdr(cv::SparseMat* instance) {
		try {
			cv::SparseMat::Hdr* ret = instance->hdr;
			return Ok(new cv::SparseMat::Hdr*(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMat::Hdr**>))
	}
	
	Result_void cv_SparseMat_setPropHdr_HdrX(cv::SparseMat* instance, cv::SparseMat::Hdr* val) {
		try {
			instance->hdr = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_SparseMat_delete(cv::SparseMat* instance) {
		delete instance;
	}
	Result<cv::SparseMat*> cv_SparseMat_SparseMat() {
		try {
			cv::SparseMat* ret = new cv::SparseMat();
			return Ok<cv::SparseMat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMat*>))
	}
	
	Result<cv::SparseMat*> cv_SparseMat_SparseMat_int_const_intX_int(int dims, const int* _sizes, int _type) {
		try {
			cv::SparseMat* ret = new cv::SparseMat(dims, _sizes, _type);
			return Ok<cv::SparseMat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMat*>))
	}
	
	Result<cv::SparseMat*> cv_SparseMat_SparseMat_const_SparseMatR(const cv::SparseMat* m) {
		try {
			cv::SparseMat* ret = new cv::SparseMat(*m);
			return Ok<cv::SparseMat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMat*>))
	}
	
	Result<cv::SparseMat*> cv_SparseMat_SparseMat_const_MatR(const cv::Mat* m) {
		try {
			cv::SparseMat* ret = new cv::SparseMat(*m);
			return Ok<cv::SparseMat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMat*>))
	}
	
	Result<cv::SparseMat*> cv_SparseMat_clone_const(const cv::SparseMat* instance) {
		try {
			cv::SparseMat ret = instance->clone();
			return Ok(new cv::SparseMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMat*>))
	}
	
	Result_void cv_SparseMat_copyTo_const_SparseMatR(const cv::SparseMat* instance, cv::SparseMat* m) {
		try {
			instance->copyTo(*m);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_SparseMat_copyTo_const_MatR(const cv::SparseMat* instance, cv::Mat* m) {
		try {
			instance->copyTo(*m);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_SparseMat_convertTo_const_SparseMatR_int_double(const cv::SparseMat* instance, cv::SparseMat* m, int rtype, double alpha) {
		try {
			instance->convertTo(*m, rtype, alpha);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_SparseMat_convertTo_const_MatR_int_double_double(const cv::SparseMat* instance, cv::Mat* m, int rtype, double alpha, double beta) {
		try {
			instance->convertTo(*m, rtype, alpha, beta);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_SparseMat_assignTo_const_SparseMatR_int(const cv::SparseMat* instance, cv::SparseMat* m, int type) {
		try {
			instance->assignTo(*m, type);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_SparseMat_create_int_const_intX_int(cv::SparseMat* instance, int dims, const int* _sizes, int _type) {
		try {
			instance->create(dims, _sizes, _type);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_SparseMat_clear(cv::SparseMat* instance) {
		try {
			instance->clear();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_SparseMat_addref(cv::SparseMat* instance) {
		try {
			instance->addref();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_SparseMat_release(cv::SparseMat* instance) {
		try {
			instance->release();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<size_t> cv_SparseMat_elemSize_const(const cv::SparseMat* instance) {
		try {
			size_t ret = instance->elemSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<size_t> cv_SparseMat_elemSize1_const(const cv::SparseMat* instance) {
		try {
			size_t ret = instance->elemSize1();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<int> cv_SparseMat_type_const(const cv::SparseMat* instance) {
		try {
			int ret = instance->type();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_SparseMat_depth_const(const cv::SparseMat* instance) {
		try {
			int ret = instance->depth();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_SparseMat_channels_const(const cv::SparseMat* instance) {
		try {
			int ret = instance->channels();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<const int*> cv_SparseMat_size_const(const cv::SparseMat* instance) {
		try {
			const int* ret = instance->size();
			return Ok<const int*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const int*>))
	}
	
	Result<int> cv_SparseMat_size_const_int(const cv::SparseMat* instance, int i) {
		try {
			int ret = instance->size(i);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_SparseMat_dims_const(const cv::SparseMat* instance) {
		try {
			int ret = instance->dims();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<size_t> cv_SparseMat_nzcount_const(const cv::SparseMat* instance) {
		try {
			size_t ret = instance->nzcount();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<size_t> cv_SparseMat_hash_const_int(const cv::SparseMat* instance, int i0) {
		try {
			size_t ret = instance->hash(i0);
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<size_t> cv_SparseMat_hash_const_int_int(const cv::SparseMat* instance, int i0, int i1) {
		try {
			size_t ret = instance->hash(i0, i1);
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<size_t> cv_SparseMat_hash_const_int_int_int(const cv::SparseMat* instance, int i0, int i1, int i2) {
		try {
			size_t ret = instance->hash(i0, i1, i2);
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<size_t> cv_SparseMat_hash_const_const_intX(const cv::SparseMat* instance, const int* idx) {
		try {
			size_t ret = instance->hash(idx);
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<unsigned char*> cv_SparseMat_ptr_int_bool_size_tX(cv::SparseMat* instance, int i0, bool createMissing, size_t* hashval) {
		try {
			unsigned char* ret = instance->ptr(i0, createMissing, hashval);
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	Result<unsigned char*> cv_SparseMat_ptr_int_int_bool_size_tX(cv::SparseMat* instance, int i0, int i1, bool createMissing, size_t* hashval) {
		try {
			unsigned char* ret = instance->ptr(i0, i1, createMissing, hashval);
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	Result<unsigned char*> cv_SparseMat_ptr_int_int_int_bool_size_tX(cv::SparseMat* instance, int i0, int i1, int i2, bool createMissing, size_t* hashval) {
		try {
			unsigned char* ret = instance->ptr(i0, i1, i2, createMissing, hashval);
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	Result<unsigned char*> cv_SparseMat_ptr_const_intX_bool_size_tX(cv::SparseMat* instance, const int* idx, bool createMissing, size_t* hashval) {
		try {
			unsigned char* ret = instance->ptr(idx, createMissing, hashval);
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	Result_void cv_SparseMat_erase_int_int_size_tX(cv::SparseMat* instance, int i0, int i1, size_t* hashval) {
		try {
			instance->erase(i0, i1, hashval);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_SparseMat_erase_int_int_int_size_tX(cv::SparseMat* instance, int i0, int i1, int i2, size_t* hashval) {
		try {
			instance->erase(i0, i1, i2, hashval);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_SparseMat_erase_const_intX_size_tX(cv::SparseMat* instance, const int* idx, size_t* hashval) {
		try {
			instance->erase(idx, hashval);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::SparseMatIterator*> cv_SparseMat_begin(cv::SparseMat* instance) {
		try {
			cv::SparseMatIterator ret = instance->begin();
			return Ok(new cv::SparseMatIterator(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMatIterator*>))
	}
	
	Result<cv::SparseMatConstIterator*> cv_SparseMat_begin_const(const cv::SparseMat* instance) {
		try {
			cv::SparseMatConstIterator ret = instance->begin();
			return Ok(new cv::SparseMatConstIterator(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMatConstIterator*>))
	}
	
	Result<cv::SparseMatIterator*> cv_SparseMat_end(cv::SparseMat* instance) {
		try {
			cv::SparseMatIterator ret = instance->end();
			return Ok(new cv::SparseMatIterator(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMatIterator*>))
	}
	
	Result<cv::SparseMatConstIterator*> cv_SparseMat_end_const(const cv::SparseMat* instance) {
		try {
			cv::SparseMatConstIterator ret = instance->end();
			return Ok(new cv::SparseMatConstIterator(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMatConstIterator*>))
	}
	
	Result<cv::SparseMat::Node**> cv_SparseMat_node_size_t(cv::SparseMat* instance, size_t nidx) {
		try {
			cv::SparseMat::Node* ret = instance->node(nidx);
			return Ok(new cv::SparseMat::Node*(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMat::Node**>))
	}
	
	Result<const cv::SparseMat::Node**> cv_SparseMat_node_const_size_t(const cv::SparseMat* instance, size_t nidx) {
		try {
			const cv::SparseMat::Node* ret = instance->node(nidx);
			return Ok(new const cv::SparseMat::Node*(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<const cv::SparseMat::Node**>))
	}
	
	Result<unsigned char*> cv_SparseMat_newNode_const_intX_size_t(cv::SparseMat* instance, const int* idx, size_t hashval) {
		try {
			unsigned char* ret = instance->newNode(idx, hashval);
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	Result_void cv_SparseMat_removeNode_size_t_size_t_size_t(cv::SparseMat* instance, size_t hidx, size_t nidx, size_t previdx) {
		try {
			instance->removeNode(hidx, nidx, previdx);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_SparseMat_resizeHashTab_size_t(cv::SparseMat* instance, size_t newsize) {
		try {
			instance->resizeHashTab(newsize);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_SparseMat_Hdr_getPropRefcount_const(const cv::SparseMat::Hdr* instance) {
		try {
			int ret = instance->refcount;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_SparseMat_Hdr_setPropRefcount_int(cv::SparseMat::Hdr* instance, int val) {
		try {
			instance->refcount = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_SparseMat_Hdr_getPropDims_const(const cv::SparseMat::Hdr* instance) {
		try {
			int ret = instance->dims;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_SparseMat_Hdr_setPropDims_int(cv::SparseMat::Hdr* instance, int val) {
		try {
			instance->dims = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_SparseMat_Hdr_getPropValueOffset_const(const cv::SparseMat::Hdr* instance) {
		try {
			int ret = instance->valueOffset;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_SparseMat_Hdr_setPropValueOffset_int(cv::SparseMat::Hdr* instance, int val) {
		try {
			instance->valueOffset = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<size_t> cv_SparseMat_Hdr_getPropNodeSize_const(const cv::SparseMat::Hdr* instance) {
		try {
			size_t ret = instance->nodeSize;
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result_void cv_SparseMat_Hdr_setPropNodeSize_size_t(cv::SparseMat::Hdr* instance, size_t val) {
		try {
			instance->nodeSize = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<size_t> cv_SparseMat_Hdr_getPropNodeCount_const(const cv::SparseMat::Hdr* instance) {
		try {
			size_t ret = instance->nodeCount;
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result_void cv_SparseMat_Hdr_setPropNodeCount_size_t(cv::SparseMat::Hdr* instance, size_t val) {
		try {
			instance->nodeCount = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<size_t> cv_SparseMat_Hdr_getPropFreeList_const(const cv::SparseMat::Hdr* instance) {
		try {
			size_t ret = instance->freeList;
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result_void cv_SparseMat_Hdr_setPropFreeList_size_t(cv::SparseMat::Hdr* instance, size_t val) {
		try {
			instance->freeList = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<unsigned char>*> cv_SparseMat_Hdr_getPropPool(cv::SparseMat::Hdr* instance) {
		try {
			std::vector<unsigned char> ret = instance->pool;
			return Ok(new std::vector<unsigned char>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<unsigned char>*>))
	}
	
	Result_void cv_SparseMat_Hdr_setPropPool_vector_unsigned_char_(cv::SparseMat::Hdr* instance, std::vector<unsigned char>* val) {
		try {
			instance->pool = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<size_t>*> cv_SparseMat_Hdr_getPropHashtab(cv::SparseMat::Hdr* instance) {
		try {
			std::vector<size_t> ret = instance->hashtab;
			return Ok(new std::vector<size_t>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<size_t>*>))
	}
	
	Result_void cv_SparseMat_Hdr_setPropHashtab_vector_size_t_(cv::SparseMat::Hdr* instance, std::vector<size_t>* val) {
		try {
			instance->hashtab = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int(*)[32]> cv_SparseMat_Hdr_getPropSize(cv::SparseMat::Hdr* instance) {
		try {
			int(*ret)[32] = &instance->size;
			return Ok<int(*)[32]>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int(*)[32]>))
	}
	
	void cv_SparseMat_Hdr_delete(cv::SparseMat::Hdr* instance) {
		delete instance;
	}
	Result<cv::SparseMat::Hdr*> cv_SparseMat_Hdr_Hdr_int_const_intX_int(int _dims, const int* _sizes, int _type) {
		try {
			cv::SparseMat::Hdr* ret = new cv::SparseMat::Hdr(_dims, _sizes, _type);
			return Ok<cv::SparseMat::Hdr*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMat::Hdr*>))
	}
	
	Result_void cv_SparseMat_Hdr_clear(cv::SparseMat::Hdr* instance) {
		try {
			instance->clear();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<size_t> cv_SparseMat_Node_getPropHashval_const(const cv::SparseMat::Node* instance) {
		try {
			size_t ret = instance->hashval;
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result_void cv_SparseMat_Node_setPropHashval_size_t(cv::SparseMat::Node* instance, size_t val) {
		try {
			instance->hashval = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<size_t> cv_SparseMat_Node_getPropNext_const(const cv::SparseMat::Node* instance) {
		try {
			size_t ret = instance->next;
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result_void cv_SparseMat_Node_setPropNext_size_t(cv::SparseMat::Node* instance, size_t val) {
		try {
			instance->next = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int(*)[32]> cv_SparseMat_Node_getPropIdx(cv::SparseMat::Node* instance) {
		try {
			int(*ret)[32] = &instance->idx;
			return Ok<int(*)[32]>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int(*)[32]>))
	}
	
	void cv_SparseMat_Node_delete(cv::SparseMat::Node* instance) {
		delete instance;
	}
	Result<const cv::SparseMat**> cv_SparseMatConstIterator_getPropM_const(const cv::SparseMatConstIterator* instance) {
		try {
			const cv::SparseMat* ret = instance->m;
			return Ok(new const cv::SparseMat*(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<const cv::SparseMat**>))
	}
	
	Result<size_t> cv_SparseMatConstIterator_getPropHashidx_const(const cv::SparseMatConstIterator* instance) {
		try {
			size_t ret = instance->hashidx;
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result_void cv_SparseMatConstIterator_setPropHashidx_size_t(cv::SparseMatConstIterator* instance, size_t val) {
		try {
			instance->hashidx = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<unsigned char*> cv_SparseMatConstIterator_getPropPtr(cv::SparseMatConstIterator* instance) {
		try {
			unsigned char* ret = instance->ptr;
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	Result_void cv_SparseMatConstIterator_setPropPtr_unsigned_charX(cv::SparseMatConstIterator* instance, unsigned char* val) {
		try {
			instance->ptr = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_SparseMatConstIterator_delete(cv::SparseMatConstIterator* instance) {
		delete instance;
	}
	Result<cv::SparseMatConstIterator*> cv_SparseMatConstIterator_SparseMatConstIterator() {
		try {
			cv::SparseMatConstIterator* ret = new cv::SparseMatConstIterator();
			return Ok<cv::SparseMatConstIterator*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMatConstIterator*>))
	}
	
	Result<cv::SparseMatConstIterator*> cv_SparseMatConstIterator_SparseMatConstIterator_const_SparseMatX(const cv::SparseMat* _m) {
		try {
			cv::SparseMatConstIterator* ret = new cv::SparseMatConstIterator(_m);
			return Ok<cv::SparseMatConstIterator*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMatConstIterator*>))
	}
	
	Result<cv::SparseMatConstIterator*> cv_SparseMatConstIterator_SparseMatConstIterator_const_SparseMatConstIteratorR(const cv::SparseMatConstIterator* it) {
		try {
			cv::SparseMatConstIterator* ret = new cv::SparseMatConstIterator(*it);
			return Ok<cv::SparseMatConstIterator*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMatConstIterator*>))
	}
	
	Result<const cv::SparseMat::Node**> cv_SparseMatConstIterator_node_const(const cv::SparseMatConstIterator* instance) {
		try {
			const cv::SparseMat::Node* ret = instance->node();
			return Ok(new const cv::SparseMat::Node*(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<const cv::SparseMat::Node**>))
	}
	
	Result_void cv_SparseMatConstIterator_seekEnd(cv::SparseMatConstIterator* instance) {
		try {
			instance->seekEnd();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_SparseMatIterator_delete(cv::SparseMatIterator* instance) {
		delete instance;
	}
	Result<cv::SparseMatIterator*> cv_SparseMatIterator_SparseMatIterator() {
		try {
			cv::SparseMatIterator* ret = new cv::SparseMatIterator();
			return Ok<cv::SparseMatIterator*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMatIterator*>))
	}
	
	Result<cv::SparseMatIterator*> cv_SparseMatIterator_SparseMatIterator_SparseMatX(cv::SparseMat* _m) {
		try {
			cv::SparseMatIterator* ret = new cv::SparseMatIterator(_m);
			return Ok<cv::SparseMatIterator*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMatIterator*>))
	}
	
	#if !defined(OCVRS_TARGET_OS_WINDOWS)
	Result<cv::SparseMatIterator*> cv_SparseMatIterator_SparseMatIterator_SparseMatX_const_intX(cv::SparseMat* _m, const int* idx) {
		try {
			cv::SparseMatIterator* ret = new cv::SparseMatIterator(_m, idx);
			return Ok<cv::SparseMatIterator*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMatIterator*>))
	}
	#endif
	
	Result<cv::SparseMatIterator*> cv_SparseMatIterator_SparseMatIterator_const_SparseMatIteratorR(const cv::SparseMatIterator* it) {
		try {
			cv::SparseMatIterator* ret = new cv::SparseMatIterator(*it);
			return Ok<cv::SparseMatIterator*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMatIterator*>))
	}
	
	Result<cv::SparseMat::Node**> cv_SparseMatIterator_node_const(const cv::SparseMatIterator* instance) {
		try {
			cv::SparseMat::Node* ret = instance->node();
			return Ok(new cv::SparseMat::Node*(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMat::Node**>))
	}
	
	Result<int> cv_TLSDataContainer_getPropKey__const(const cv::TLSDataContainer* instance) {
		try {
			int ret = instance->key_;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_TLSDataContainer_setPropKey__int(cv::TLSDataContainer* instance, int val) {
		try {
			instance->key_ = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_TLSDataContainer_getData_const(const cv::TLSDataContainer* instance) {
		try {
			void* ret = instance->getData();
			return Ok<void*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<void*> cv_TLSDataContainer_createDataInstance_const(const cv::TLSDataContainer* instance) {
		try {
			void* ret = instance->createDataInstance();
			return Ok<void*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_TLSDataContainer_deleteDataInstance_const_voidX(const cv::TLSDataContainer* instance, void* pData) {
		try {
			instance->deleteDataInstance(pData);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::TermCriteria> cv_TermCriteria_TermCriteria() {
		try {
			cv::TermCriteria ret;
			return Ok<cv::TermCriteria>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TermCriteria>))
	}
	
	Result<cv::TermCriteria> cv_TermCriteria_TermCriteria_int_int_double(int type, int maxCount, double epsilon) {
		try {
			cv::TermCriteria ret(type, maxCount, epsilon);
			return Ok<cv::TermCriteria>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TermCriteria>))
	}
	
	void cv_TickMeter_delete(cv::TickMeter* instance) {
		delete instance;
	}
	Result<cv::TickMeter*> cv_TickMeter_TickMeter() {
		try {
			cv::TickMeter* ret = new cv::TickMeter();
			return Ok<cv::TickMeter*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TickMeter*>))
	}
	
	Result_void cv_TickMeter_start(cv::TickMeter* instance) {
		try {
			instance->start();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_TickMeter_stop(cv::TickMeter* instance) {
		try {
			instance->stop();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int64_t> cv_TickMeter_getTimeTicks_const(const cv::TickMeter* instance) {
		try {
			int64_t ret = instance->getTimeTicks();
			return Ok<int64_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int64_t>))
	}
	
	Result<double> cv_TickMeter_getTimeMicro_const(const cv::TickMeter* instance) {
		try {
			double ret = instance->getTimeMicro();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<double> cv_TickMeter_getTimeMilli_const(const cv::TickMeter* instance) {
		try {
			double ret = instance->getTimeMilli();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<double> cv_TickMeter_getTimeSec_const(const cv::TickMeter* instance) {
		try {
			double ret = instance->getTimeSec();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<int64_t> cv_TickMeter_getCounter_const(const cv::TickMeter* instance) {
		try {
			int64_t ret = instance->getCounter();
			return Ok<int64_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int64_t>))
	}
	
	Result_void cv_TickMeter_reset(cv::TickMeter* instance) {
		try {
			instance->reset();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_UMat_getPropFlags_const(const cv::UMat* instance) {
		try {
			int ret = instance->flags;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_UMat_setPropFlags_int(cv::UMat* instance, int val) {
		try {
			instance->flags = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_UMat_getPropDims_const(const cv::UMat* instance) {
		try {
			int ret = instance->dims;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_UMat_setPropDims_int(cv::UMat* instance, int val) {
		try {
			instance->dims = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_UMat_getPropRows_const(const cv::UMat* instance) {
		try {
			int ret = instance->rows;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_UMat_setPropRows_int(cv::UMat* instance, int val) {
		try {
			instance->rows = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_UMat_getPropCols_const(const cv::UMat* instance) {
		try {
			int ret = instance->cols;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_UMat_setPropCols_int(cv::UMat* instance, int val) {
		try {
			instance->cols = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::UMatUsageFlags> cv_UMat_getPropUsageFlags_const(const cv::UMat* instance) {
		try {
			cv::UMatUsageFlags ret = instance->usageFlags;
			return Ok<cv::UMatUsageFlags>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMatUsageFlags>))
	}
	
	Result_void cv_UMat_setPropUsageFlags_UMatUsageFlags(cv::UMat* instance, cv::UMatUsageFlags val) {
		try {
			instance->usageFlags = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::UMatData**> cv_UMat_getPropU(cv::UMat* instance) {
		try {
			cv::UMatData* ret = instance->u;
			return Ok(new cv::UMatData*(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMatData**>))
	}
	
	Result_void cv_UMat_setPropU_UMatDataX(cv::UMat* instance, cv::UMatData* val) {
		try {
			instance->u = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<size_t> cv_UMat_getPropOffset_const(const cv::UMat* instance) {
		try {
			size_t ret = instance->offset;
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result_void cv_UMat_setPropOffset_size_t(cv::UMat* instance, size_t val) {
		try {
			instance->offset = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::MatSize*> cv_UMat_getPropSize_const(const cv::UMat* instance) {
		try {
			cv::MatSize ret = instance->size;
			return Ok(new cv::MatSize(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatSize*>))
	}
	
	Result<cv::MatStep*> cv_UMat_getPropStep_const(const cv::UMat* instance) {
		try {
			cv::MatStep ret = instance->step;
			return Ok(new cv::MatStep(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatStep*>))
	}
	
	void cv_UMat_delete(cv::UMat* instance) {
		delete instance;
	}
	Result<cv::UMat*> cv_UMat_UMat_UMatUsageFlags(cv::UMatUsageFlags usageFlags) {
		try {
			cv::UMat* ret = new cv::UMat(usageFlags);
			return Ok<cv::UMat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::UMat*> cv_UMat_UMat_int_int_int_UMatUsageFlags(int rows, int cols, int type, cv::UMatUsageFlags usageFlags) {
		try {
			cv::UMat* ret = new cv::UMat(rows, cols, type, usageFlags);
			return Ok<cv::UMat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::UMat*> cv_UMat_UMat_Size_int_UMatUsageFlags(cv::Size* size, int type, cv::UMatUsageFlags usageFlags) {
		try {
			cv::UMat* ret = new cv::UMat(*size, type, usageFlags);
			return Ok<cv::UMat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::UMat*> cv_UMat_UMat_int_int_int_const_ScalarR_UMatUsageFlags(int rows, int cols, int type, const cv::Scalar* s, cv::UMatUsageFlags usageFlags) {
		try {
			cv::UMat* ret = new cv::UMat(rows, cols, type, *s, usageFlags);
			return Ok<cv::UMat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::UMat*> cv_UMat_UMat_Size_int_const_ScalarR_UMatUsageFlags(cv::Size* size, int type, const cv::Scalar* s, cv::UMatUsageFlags usageFlags) {
		try {
			cv::UMat* ret = new cv::UMat(*size, type, *s, usageFlags);
			return Ok<cv::UMat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::UMat*> cv_UMat_UMat_int_const_intX_int_UMatUsageFlags(int ndims, const int* sizes, int type, cv::UMatUsageFlags usageFlags) {
		try {
			cv::UMat* ret = new cv::UMat(ndims, sizes, type, usageFlags);
			return Ok<cv::UMat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::UMat*> cv_UMat_UMat_int_const_intX_int_const_ScalarR_UMatUsageFlags(int ndims, const int* sizes, int type, const cv::Scalar* s, cv::UMatUsageFlags usageFlags) {
		try {
			cv::UMat* ret = new cv::UMat(ndims, sizes, type, *s, usageFlags);
			return Ok<cv::UMat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::UMat*> cv_UMat_UMat_const_UMatR(const cv::UMat* m) {
		try {
			cv::UMat* ret = new cv::UMat(*m);
			return Ok<cv::UMat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::UMat*> cv_UMat_UMat_const_UMatR_const_RangeR_const_RangeR(const cv::UMat* m, const cv::Range* rowRange, const cv::Range* colRange) {
		try {
			cv::UMat* ret = new cv::UMat(*m, *rowRange, *colRange);
			return Ok<cv::UMat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::UMat*> cv_UMat_UMat_const_UMatR_const_RectR(const cv::UMat* m, const cv::Rect* roi) {
		try {
			cv::UMat* ret = new cv::UMat(*m, *roi);
			return Ok<cv::UMat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::UMat*> cv_UMat_UMat_const_UMatR_const_vector_Range_R(const cv::UMat* m, const std::vector<cv::Range>* ranges) {
		try {
			cv::UMat* ret = new cv::UMat(*m, *ranges);
			return Ok<cv::UMat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::Mat*> cv_UMat_getMat_const_int(const cv::UMat* instance, int flags) {
		try {
			cv::Mat ret = instance->getMat(flags);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::UMat*> cv_UMat_row_const_int(const cv::UMat* instance, int y) {
		try {
			cv::UMat ret = instance->row(y);
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::UMat*> cv_UMat_col_const_int(const cv::UMat* instance, int x) {
		try {
			cv::UMat ret = instance->col(x);
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::UMat*> cv_UMat_rowRange_const_int_int(const cv::UMat* instance, int startrow, int endrow) {
		try {
			cv::UMat ret = instance->rowRange(startrow, endrow);
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::UMat*> cv_UMat_rowRange_const_const_RangeR(const cv::UMat* instance, const cv::Range* r) {
		try {
			cv::UMat ret = instance->rowRange(*r);
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::UMat*> cv_UMat_colRange_const_int_int(const cv::UMat* instance, int startcol, int endcol) {
		try {
			cv::UMat ret = instance->colRange(startcol, endcol);
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::UMat*> cv_UMat_colRange_const_const_RangeR(const cv::UMat* instance, const cv::Range* r) {
		try {
			cv::UMat ret = instance->colRange(*r);
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::UMat*> cv_UMat_diag_const_int(const cv::UMat* instance, int d) {
		try {
			cv::UMat ret = instance->diag(d);
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::UMat*> cv_UMat_diag_const_UMatR(const cv::UMat* d) {
		try {
			cv::UMat ret = cv::UMat::diag(*d);
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::UMat*> cv_UMat_clone_const(const cv::UMat* instance) {
		try {
			cv::UMat ret = instance->clone();
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result_void cv_UMat_copyTo_const_const__OutputArrayR(const cv::UMat* instance, const cv::_OutputArray* m) {
		try {
			instance->copyTo(*m);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_UMat_copyTo_const_const__OutputArrayR_const__InputArrayR(const cv::UMat* instance, const cv::_OutputArray* m, const cv::_InputArray* mask) {
		try {
			instance->copyTo(*m, *mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_UMat_convertTo_const_const__OutputArrayR_int_double_double(const cv::UMat* instance, const cv::_OutputArray* m, int rtype, double alpha, double beta) {
		try {
			instance->convertTo(*m, rtype, alpha, beta);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_UMat_assignTo_const_UMatR_int(const cv::UMat* instance, cv::UMat* m, int type) {
		try {
			instance->assignTo(*m, type);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::UMat*> cv_UMat_setTo_const__InputArrayR_const__InputArrayR(cv::UMat* instance, const cv::_InputArray* value, const cv::_InputArray* mask) {
		try {
			cv::UMat ret = instance->setTo(*value, *mask);
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::UMat*> cv_UMat_reshape_const_int_int(const cv::UMat* instance, int cn, int rows) {
		try {
			cv::UMat ret = instance->reshape(cn, rows);
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::UMat*> cv_UMat_reshape_const_int_int_const_intX(const cv::UMat* instance, int cn, int newndims, const int* newsz) {
		try {
			cv::UMat ret = instance->reshape(cn, newndims, newsz);
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::UMat*> cv_UMat_t_const(const cv::UMat* instance) {
		try {
			cv::UMat ret = instance->t();
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::UMat*> cv_UMat_inv_const_int(const cv::UMat* instance, int method) {
		try {
			cv::UMat ret = instance->inv(method);
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::UMat*> cv_UMat_mul_const_const__InputArrayR_double(const cv::UMat* instance, const cv::_InputArray* m, double scale) {
		try {
			cv::UMat ret = instance->mul(*m, scale);
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<double> cv_UMat_dot_const_const__InputArrayR(const cv::UMat* instance, const cv::_InputArray* m) {
		try {
			double ret = instance->dot(*m);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<cv::UMat*> cv_UMat_zeros_int_int_int(int rows, int cols, int type) {
		try {
			cv::UMat ret = cv::UMat::zeros(rows, cols, type);
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::UMat*> cv_UMat_zeros_Size_int(cv::Size* size, int type) {
		try {
			cv::UMat ret = cv::UMat::zeros(*size, type);
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::UMat*> cv_UMat_zeros_int_const_intX_int(int ndims, const int* sz, int type) {
		try {
			cv::UMat ret = cv::UMat::zeros(ndims, sz, type);
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::UMat*> cv_UMat_ones_int_int_int(int rows, int cols, int type) {
		try {
			cv::UMat ret = cv::UMat::ones(rows, cols, type);
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::UMat*> cv_UMat_ones_Size_int(cv::Size* size, int type) {
		try {
			cv::UMat ret = cv::UMat::ones(*size, type);
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::UMat*> cv_UMat_ones_int_const_intX_int(int ndims, const int* sz, int type) {
		try {
			cv::UMat ret = cv::UMat::ones(ndims, sz, type);
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::UMat*> cv_UMat_eye_int_int_int(int rows, int cols, int type) {
		try {
			cv::UMat ret = cv::UMat::eye(rows, cols, type);
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::UMat*> cv_UMat_eye_Size_int(cv::Size* size, int type) {
		try {
			cv::UMat ret = cv::UMat::eye(*size, type);
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result_void cv_UMat_create_int_int_int_UMatUsageFlags(cv::UMat* instance, int rows, int cols, int type, cv::UMatUsageFlags usageFlags) {
		try {
			instance->create(rows, cols, type, usageFlags);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_UMat_create_Size_int_UMatUsageFlags(cv::UMat* instance, cv::Size* size, int type, cv::UMatUsageFlags usageFlags) {
		try {
			instance->create(*size, type, usageFlags);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_UMat_create_int_const_intX_int_UMatUsageFlags(cv::UMat* instance, int ndims, const int* sizes, int type, cv::UMatUsageFlags usageFlags) {
		try {
			instance->create(ndims, sizes, type, usageFlags);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_UMat_create_const_vector_int_R_int_UMatUsageFlags(cv::UMat* instance, const std::vector<int>* sizes, int type, cv::UMatUsageFlags usageFlags) {
		try {
			instance->create(*sizes, type, usageFlags);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_UMat_addref(cv::UMat* instance) {
		try {
			instance->addref();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_UMat_release(cv::UMat* instance) {
		try {
			instance->release();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_UMat_deallocate(cv::UMat* instance) {
		try {
			instance->deallocate();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_UMat_locateROI_const_SizeR_PointR(const cv::UMat* instance, cv::Size* wholeSize, cv::Point* ofs) {
		try {
			instance->locateROI(*wholeSize, *ofs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::UMat*> cv_UMat_adjustROI_int_int_int_int(cv::UMat* instance, int dtop, int dbottom, int dleft, int dright) {
		try {
			cv::UMat ret = instance->adjustROI(dtop, dbottom, dleft, dright);
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<bool> cv_UMat_isContinuous_const(const cv::UMat* instance) {
		try {
			bool ret = instance->isContinuous();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_UMat_isSubmatrix_const(const cv::UMat* instance) {
		try {
			bool ret = instance->isSubmatrix();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<size_t> cv_UMat_elemSize_const(const cv::UMat* instance) {
		try {
			size_t ret = instance->elemSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<size_t> cv_UMat_elemSize1_const(const cv::UMat* instance) {
		try {
			size_t ret = instance->elemSize1();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<int> cv_UMat_type_const(const cv::UMat* instance) {
		try {
			int ret = instance->type();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_UMat_depth_const(const cv::UMat* instance) {
		try {
			int ret = instance->depth();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_UMat_channels_const(const cv::UMat* instance) {
		try {
			int ret = instance->channels();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<size_t> cv_UMat_step1_const_int(const cv::UMat* instance, int i) {
		try {
			size_t ret = instance->step1(i);
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<bool> cv_UMat_empty_const(const cv::UMat* instance) {
		try {
			bool ret = instance->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<size_t> cv_UMat_total_const(const cv::UMat* instance) {
		try {
			size_t ret = instance->total();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<int> cv_UMat_checkVector_const_int_int_bool(const cv::UMat* instance, int elemChannels, int depth, bool requireContinuous) {
		try {
			int ret = instance->checkVector(elemChannels, depth, requireContinuous);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<cv::UMat*> cv_UMat_UMat_UMatR(cv::UMat* m) {
		try {
			cv::UMat* ret = new cv::UMat(*m);
			return Ok<cv::UMat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<void*> cv_UMat_handle_const_int(const cv::UMat* instance, int accessFlags) {
		try {
			void* ret = instance->handle(accessFlags);
			return Ok<void*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_UMat_ndoffset_const_size_tX(const cv::UMat* instance, size_t* ofs) {
		try {
			instance->ndoffset(ofs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_UMatData_getPropUrefcount_const(const cv::UMatData* instance) {
		try {
			int ret = instance->urefcount;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_UMatData_setPropUrefcount_int(cv::UMatData* instance, int val) {
		try {
			instance->urefcount = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_UMatData_getPropRefcount_const(const cv::UMatData* instance) {
		try {
			int ret = instance->refcount;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_UMatData_setPropRefcount_int(cv::UMatData* instance, int val) {
		try {
			instance->refcount = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<unsigned char*> cv_UMatData_getPropData(cv::UMatData* instance) {
		try {
			unsigned char* ret = instance->data;
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	Result_void cv_UMatData_setPropData_unsigned_charX(cv::UMatData* instance, unsigned char* val) {
		try {
			instance->data = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<unsigned char*> cv_UMatData_getPropOrigdata(cv::UMatData* instance) {
		try {
			unsigned char* ret = instance->origdata;
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	Result_void cv_UMatData_setPropOrigdata_unsigned_charX(cv::UMatData* instance, unsigned char* val) {
		try {
			instance->origdata = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<size_t> cv_UMatData_getPropSize_const(const cv::UMatData* instance) {
		try {
			size_t ret = instance->size;
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result_void cv_UMatData_setPropSize_size_t(cv::UMatData* instance, size_t val) {
		try {
			instance->size = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_UMatData_getPropFlags_const(const cv::UMatData* instance) {
		try {
			int ret = instance->flags;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_UMatData_setPropFlags_int(cv::UMatData* instance, int val) {
		try {
			instance->flags = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_UMatData_getPropHandle(cv::UMatData* instance) {
		try {
			void* ret = instance->handle;
			return Ok<void*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_UMatData_setPropHandle_voidX(cv::UMatData* instance, void* val) {
		try {
			instance->handle = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_UMatData_getPropUserdata(cv::UMatData* instance) {
		try {
			void* ret = instance->userdata;
			return Ok<void*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_UMatData_setPropUserdata_voidX(cv::UMatData* instance, void* val) {
		try {
			instance->userdata = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_UMatData_getPropAllocatorFlags__const(const cv::UMatData* instance) {
		try {
			int ret = instance->allocatorFlags_;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_UMatData_setPropAllocatorFlags__int(cv::UMatData* instance, int val) {
		try {
			instance->allocatorFlags_ = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_UMatData_getPropMapcount_const(const cv::UMatData* instance) {
		try {
			int ret = instance->mapcount;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_UMatData_setPropMapcount_int(cv::UMatData* instance, int val) {
		try {
			instance->mapcount = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::UMatData**> cv_UMatData_getPropOriginalUMatData(cv::UMatData* instance) {
		try {
			cv::UMatData* ret = instance->originalUMatData;
			return Ok(new cv::UMatData*(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMatData**>))
	}
	
	Result_void cv_UMatData_setPropOriginalUMatData_UMatDataX(cv::UMatData* instance, cv::UMatData* val) {
		try {
			instance->originalUMatData = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_UMatData_delete(cv::UMatData* instance) {
		delete instance;
	}
	Result_void cv_UMatData_lock(cv::UMatData* instance) {
		try {
			instance->lock();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_UMatData_unlock(cv::UMatData* instance) {
		try {
			instance->unlock();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_UMatData_hostCopyObsolete_const(const cv::UMatData* instance) {
		try {
			bool ret = instance->hostCopyObsolete();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_UMatData_deviceCopyObsolete_const(const cv::UMatData* instance) {
		try {
			bool ret = instance->deviceCopyObsolete();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_UMatData_deviceMemMapped_const(const cv::UMatData* instance) {
		try {
			bool ret = instance->deviceMemMapped();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_UMatData_copyOnMap_const(const cv::UMatData* instance) {
		try {
			bool ret = instance->copyOnMap();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_UMatData_tempUMat_const(const cv::UMatData* instance) {
		try {
			bool ret = instance->tempUMat();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_UMatData_tempCopiedUMat_const(const cv::UMatData* instance) {
		try {
			bool ret = instance->tempCopiedUMat();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_UMatData_markHostCopyObsolete_bool(cv::UMatData* instance, bool flag) {
		try {
			instance->markHostCopyObsolete(flag);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_UMatData_markDeviceCopyObsolete_bool(cv::UMatData* instance, bool flag) {
		try {
			instance->markDeviceCopyObsolete(flag);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_UMatData_markDeviceMemMapped_bool(cv::UMatData* instance, bool flag) {
		try {
			instance->markDeviceMemMapped(flag);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::UMatData**> cv_UMatDataAutoLock_getPropU(cv::UMatDataAutoLock* instance) {
		try {
			cv::UMatData* ret = instance->u;
			return Ok(new cv::UMatData*(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMatData**>))
	}
	
	Result_void cv_UMatDataAutoLock_setPropU_UMatDataX(cv::UMatDataAutoLock* instance, cv::UMatData* val) {
		try {
			instance->u = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_UMatDataAutoLock_delete(cv::UMatDataAutoLock* instance) {
		delete instance;
	}
	Result<cv::UMatDataAutoLock*> cv_UMatDataAutoLock_UMatDataAutoLock_UMatDataX(cv::UMatData* u) {
		try {
			cv::UMatDataAutoLock* ret = new cv::UMatDataAutoLock(u);
			return Ok<cv::UMatDataAutoLock*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMatDataAutoLock*>))
	}
	
	void cv__InputArray_delete(cv::_InputArray* instance) {
		delete instance;
	}
	Result<cv::_InputArray*> cv__InputArray__InputArray() {
		try {
			cv::_InputArray* ret = new cv::_InputArray();
			return Ok<cv::_InputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputArray*>))
	}
	
	Result<cv::_InputArray*> cv__InputArray__InputArray_int_voidX(int _flags, void* _obj) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(_flags, _obj);
			return Ok<cv::_InputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputArray*>))
	}
	
	Result<cv::_InputArray*> cv__InputArray__InputArray_const_MatR(const cv::Mat* m) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(*m);
			return Ok<cv::_InputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputArray*>))
	}
	
	Result<cv::_InputArray*> cv__InputArray__InputArray_const_MatExprR(const cv::MatExpr* expr) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(*expr);
			return Ok<cv::_InputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputArray*>))
	}
	
	Result<cv::_InputArray*> cv__InputArray__InputArray_const_vector_Mat_R(const std::vector<cv::Mat>* vec) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(*vec);
			return Ok<cv::_InputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputArray*>))
	}
	
	Result<cv::_InputArray*> cv__InputArray__InputArray_const_vector_bool_R(const std::vector<bool>* vec) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(*vec);
			return Ok<cv::_InputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputArray*>))
	}
	
	Result<cv::_InputArray*> cv__InputArray__InputArray_const_doubleR(const double* val) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(*val);
			return Ok<cv::_InputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputArray*>))
	}
	
	Result<cv::_InputArray*> cv__InputArray__InputArray_const_GpuMatR(const cv::cuda::GpuMat* d_mat) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(*d_mat);
			return Ok<cv::_InputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputArray*>))
	}
	
	Result<cv::_InputArray*> cv__InputArray__InputArray_const_vector_GpuMat_R(const std::vector<cv::cuda::GpuMat>* d_mat_array) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(*d_mat_array);
			return Ok<cv::_InputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputArray*>))
	}
	
	Result<cv::_InputArray*> cv__InputArray__InputArray_const_HostMemR(const cv::cuda::HostMem* cuda_mem) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(*cuda_mem);
			return Ok<cv::_InputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputArray*>))
	}
	
	Result<cv::_InputArray*> cv__InputArray__InputArray_const_UMatR(const cv::UMat* um) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(*um);
			return Ok<cv::_InputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputArray*>))
	}
	
	Result<cv::_InputArray*> cv__InputArray__InputArray_const_vector_UMat_R(const std::vector<cv::UMat>* umv) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(*umv);
			return Ok<cv::_InputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputArray*>))
	}
	
	Result<cv::Mat*> cv__InputArray_getMat_const_int(const cv::_InputArray* instance, int idx) {
		try {
			cv::Mat ret = instance->getMat(idx);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv__InputArray_getMat__const_int(const cv::_InputArray* instance, int idx) {
		try {
			cv::Mat ret = instance->getMat_(idx);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::UMat*> cv__InputArray_getUMat_const_int(const cv::_InputArray* instance, int idx) {
		try {
			cv::UMat ret = instance->getUMat(idx);
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result_void cv__InputArray_getMatVector_const_vector_Mat_R(const cv::_InputArray* instance, std::vector<cv::Mat>* mv) {
		try {
			instance->getMatVector(*mv);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv__InputArray_getUMatVector_const_vector_UMat_R(const cv::_InputArray* instance, std::vector<cv::UMat>* umv) {
		try {
			instance->getUMatVector(*umv);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv__InputArray_getGpuMatVector_const_vector_GpuMat_R(const cv::_InputArray* instance, std::vector<cv::cuda::GpuMat>* gpumv) {
		try {
			instance->getGpuMatVector(*gpumv);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::cuda::GpuMat*> cv__InputArray_getGpuMat_const(const cv::_InputArray* instance) {
		try {
			cv::cuda::GpuMat ret = instance->getGpuMat();
			return Ok(new cv::cuda::GpuMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	Result<int> cv__InputArray_getFlags_const(const cv::_InputArray* instance) {
		try {
			int ret = instance->getFlags();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<void*> cv__InputArray_getObj_const(const cv::_InputArray* instance) {
		try {
			void* ret = instance->getObj();
			return Ok<void*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<cv::Size> cv__InputArray_getSz_const(const cv::_InputArray* instance) {
		try {
			cv::Size ret = instance->getSz();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result<int> cv__InputArray_kind_const(const cv::_InputArray* instance) {
		try {
			int ret = instance->kind();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv__InputArray_dims_const_int(const cv::_InputArray* instance, int i) {
		try {
			int ret = instance->dims(i);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv__InputArray_cols_const_int(const cv::_InputArray* instance, int i) {
		try {
			int ret = instance->cols(i);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv__InputArray_rows_const_int(const cv::_InputArray* instance, int i) {
		try {
			int ret = instance->rows(i);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<cv::Size> cv__InputArray_size_const_int(const cv::_InputArray* instance, int i) {
		try {
			cv::Size ret = instance->size(i);
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result<int> cv__InputArray_sizend_const_intX_int(const cv::_InputArray* instance, int* sz, int i) {
		try {
			int ret = instance->sizend(sz, i);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<bool> cv__InputArray_sameSize_const_const__InputArrayR(const cv::_InputArray* instance, const cv::_InputArray* arr) {
		try {
			bool ret = instance->sameSize(*arr);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<size_t> cv__InputArray_total_const_int(const cv::_InputArray* instance, int i) {
		try {
			size_t ret = instance->total(i);
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<int> cv__InputArray_type_const_int(const cv::_InputArray* instance, int i) {
		try {
			int ret = instance->type(i);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv__InputArray_depth_const_int(const cv::_InputArray* instance, int i) {
		try {
			int ret = instance->depth(i);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv__InputArray_channels_const_int(const cv::_InputArray* instance, int i) {
		try {
			int ret = instance->channels(i);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<bool> cv__InputArray_isContinuous_const_int(const cv::_InputArray* instance, int i) {
		try {
			bool ret = instance->isContinuous(i);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv__InputArray_isSubmatrix_const_int(const cv::_InputArray* instance, int i) {
		try {
			bool ret = instance->isSubmatrix(i);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv__InputArray_empty_const(const cv::_InputArray* instance) {
		try {
			bool ret = instance->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv__InputArray_copyTo_const_const__OutputArrayR(const cv::_InputArray* instance, const cv::_OutputArray* arr) {
		try {
			instance->copyTo(*arr);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv__InputArray_copyTo_const_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* instance, const cv::_OutputArray* arr, const cv::_InputArray* mask) {
		try {
			instance->copyTo(*arr, *mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<size_t> cv__InputArray_offset_const_int(const cv::_InputArray* instance, int i) {
		try {
			size_t ret = instance->offset(i);
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<size_t> cv__InputArray_step_const_int(const cv::_InputArray* instance, int i) {
		try {
			size_t ret = instance->step(i);
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<bool> cv__InputArray_isMat_const(const cv::_InputArray* instance) {
		try {
			bool ret = instance->isMat();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv__InputArray_isUMat_const(const cv::_InputArray* instance) {
		try {
			bool ret = instance->isUMat();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv__InputArray_isMatVector_const(const cv::_InputArray* instance) {
		try {
			bool ret = instance->isMatVector();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv__InputArray_isUMatVector_const(const cv::_InputArray* instance) {
		try {
			bool ret = instance->isUMatVector();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv__InputArray_isMatx_const(const cv::_InputArray* instance) {
		try {
			bool ret = instance->isMatx();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv__InputArray_isVector_const(const cv::_InputArray* instance) {
		try {
			bool ret = instance->isVector();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv__InputArray_isGpuMatVector_const(const cv::_InputArray* instance) {
		try {
			bool ret = instance->isGpuMatVector();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	void cv__InputOutputArray_delete(cv::_InputOutputArray* instance) {
		delete instance;
	}
	Result<cv::_InputOutputArray*> cv__InputOutputArray__InputOutputArray() {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray();
			return Ok<cv::_InputOutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	Result<cv::_InputOutputArray*> cv__InputOutputArray__InputOutputArray_int_voidX(int _flags, void* _obj) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(_flags, _obj);
			return Ok<cv::_InputOutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	Result<cv::_InputOutputArray*> cv__InputOutputArray__InputOutputArray_MatR(cv::Mat* m) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*m);
			return Ok<cv::_InputOutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	Result<cv::_InputOutputArray*> cv__InputOutputArray__InputOutputArray_vector_Mat_R(std::vector<cv::Mat>* vec) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*vec);
			return Ok<cv::_InputOutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	Result<cv::_InputOutputArray*> cv__InputOutputArray__InputOutputArray_GpuMatR(cv::cuda::GpuMat* d_mat) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*d_mat);
			return Ok<cv::_InputOutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	Result<cv::_InputOutputArray*> cv__InputOutputArray__InputOutputArray_HostMemR(cv::cuda::HostMem* cuda_mem) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*cuda_mem);
			return Ok<cv::_InputOutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	Result<cv::_InputOutputArray*> cv__InputOutputArray__InputOutputArray_vector_bool_R(std::vector<bool>* vec) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*vec);
			return Ok<cv::_InputOutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	Result<cv::_InputOutputArray*> cv__InputOutputArray__InputOutputArray_UMatR(cv::UMat* m) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*m);
			return Ok<cv::_InputOutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	Result<cv::_InputOutputArray*> cv__InputOutputArray__InputOutputArray_vector_UMat_R(std::vector<cv::UMat>* vec) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*vec);
			return Ok<cv::_InputOutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	Result<cv::_InputOutputArray*> cv__InputOutputArray__InputOutputArray_const_MatR(const cv::Mat* m) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*m);
			return Ok<cv::_InputOutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	Result<cv::_InputOutputArray*> cv__InputOutputArray__InputOutputArray_const_vector_Mat_R(const std::vector<cv::Mat>* vec) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*vec);
			return Ok<cv::_InputOutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	Result<cv::_InputOutputArray*> cv__InputOutputArray__InputOutputArray_const_GpuMatR(const cv::cuda::GpuMat* d_mat) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*d_mat);
			return Ok<cv::_InputOutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	Result<cv::_InputOutputArray*> cv__InputOutputArray__InputOutputArray_const_vector_GpuMat_R(const std::vector<cv::cuda::GpuMat>* d_mat) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*d_mat);
			return Ok<cv::_InputOutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	Result<cv::_InputOutputArray*> cv__InputOutputArray__InputOutputArray_const_HostMemR(const cv::cuda::HostMem* cuda_mem) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*cuda_mem);
			return Ok<cv::_InputOutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	Result<cv::_InputOutputArray*> cv__InputOutputArray__InputOutputArray_const_UMatR(const cv::UMat* m) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*m);
			return Ok<cv::_InputOutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	Result<cv::_InputOutputArray*> cv__InputOutputArray__InputOutputArray_const_vector_UMat_R(const std::vector<cv::UMat>* vec) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*vec);
			return Ok<cv::_InputOutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	void cv__OutputArray_delete(cv::_OutputArray* instance) {
		delete instance;
	}
	Result<cv::_OutputArray*> cv__OutputArray__OutputArray() {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray();
			return Ok<cv::_OutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	Result<cv::_OutputArray*> cv__OutputArray__OutputArray_int_voidX(int _flags, void* _obj) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(_flags, _obj);
			return Ok<cv::_OutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	Result<cv::_OutputArray*> cv__OutputArray__OutputArray_MatR(cv::Mat* m) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*m);
			return Ok<cv::_OutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	Result<cv::_OutputArray*> cv__OutputArray__OutputArray_vector_Mat_R(std::vector<cv::Mat>* vec) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*vec);
			return Ok<cv::_OutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	Result<cv::_OutputArray*> cv__OutputArray__OutputArray_GpuMatR(cv::cuda::GpuMat* d_mat) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*d_mat);
			return Ok<cv::_OutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	Result<cv::_OutputArray*> cv__OutputArray__OutputArray_vector_GpuMat_R(std::vector<cv::cuda::GpuMat>* d_mat) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*d_mat);
			return Ok<cv::_OutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	Result<cv::_OutputArray*> cv__OutputArray__OutputArray_HostMemR(cv::cuda::HostMem* cuda_mem) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*cuda_mem);
			return Ok<cv::_OutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	Result<cv::_OutputArray*> cv__OutputArray__OutputArray_vector_bool_R(std::vector<bool>* vec) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*vec);
			return Ok<cv::_OutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	Result<cv::_OutputArray*> cv__OutputArray__OutputArray_UMatR(cv::UMat* m) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*m);
			return Ok<cv::_OutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	Result<cv::_OutputArray*> cv__OutputArray__OutputArray_vector_UMat_R(std::vector<cv::UMat>* vec) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*vec);
			return Ok<cv::_OutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	Result<cv::_OutputArray*> cv__OutputArray__OutputArray_const_MatR(const cv::Mat* m) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*m);
			return Ok<cv::_OutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	Result<cv::_OutputArray*> cv__OutputArray__OutputArray_const_vector_Mat_R(const std::vector<cv::Mat>* vec) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*vec);
			return Ok<cv::_OutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	Result<cv::_OutputArray*> cv__OutputArray__OutputArray_const_GpuMatR(const cv::cuda::GpuMat* d_mat) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*d_mat);
			return Ok<cv::_OutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	#if !defined(OCVRS_TARGET_OS_WINDOWS)
	Result<cv::_OutputArray*> cv__OutputArray__OutputArray_const_vector_GpuMat_R(const std::vector<cv::cuda::GpuMat>* d_mat) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*d_mat);
			return Ok<cv::_OutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	#endif
	
	Result<cv::_OutputArray*> cv__OutputArray__OutputArray_const_HostMemR(const cv::cuda::HostMem* cuda_mem) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*cuda_mem);
			return Ok<cv::_OutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	Result<cv::_OutputArray*> cv__OutputArray__OutputArray_const_UMatR(const cv::UMat* m) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*m);
			return Ok<cv::_OutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	Result<cv::_OutputArray*> cv__OutputArray__OutputArray_const_vector_UMat_R(const std::vector<cv::UMat>* vec) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*vec);
			return Ok<cv::_OutputArray*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	Result<bool> cv__OutputArray_fixedSize_const(const cv::_OutputArray* instance) {
		try {
			bool ret = instance->fixedSize();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv__OutputArray_fixedType_const(const cv::_OutputArray* instance) {
		try {
			bool ret = instance->fixedType();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv__OutputArray_needed_const(const cv::_OutputArray* instance) {
		try {
			bool ret = instance->needed();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<cv::Mat*> cv__OutputArray_getMatRef_const_int(const cv::_OutputArray* instance, int i) {
		try {
			cv::Mat ret = instance->getMatRef(i);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::UMat*> cv__OutputArray_getUMatRef_const_int(const cv::_OutputArray* instance, int i) {
		try {
			cv::UMat ret = instance->getUMatRef(i);
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result<cv::cuda::GpuMat*> cv__OutputArray_getGpuMatRef_const(const cv::_OutputArray* instance) {
		try {
			cv::cuda::GpuMat ret = instance->getGpuMatRef();
			return Ok(new cv::cuda::GpuMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	Result<std::vector<cv::cuda::GpuMat>*> cv__OutputArray_getGpuMatVecRef_const(const cv::_OutputArray* instance) {
		try {
			std::vector<cv::cuda::GpuMat> ret = instance->getGpuMatVecRef();
			return Ok(new std::vector<cv::cuda::GpuMat>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::cuda::GpuMat>*>))
	}
	
	Result<cv::cuda::HostMem*> cv__OutputArray_getHostMemRef_const(const cv::_OutputArray* instance) {
		try {
			cv::cuda::HostMem ret = instance->getHostMemRef();
			return Ok(new cv::cuda::HostMem(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::HostMem*>))
	}
	
	Result_void cv__OutputArray_create_const_Size_int_int_bool_int(const cv::_OutputArray* instance, cv::Size* sz, int type, int i, bool allowTransposed, int fixedDepthMask) {
		try {
			instance->create(*sz, type, i, allowTransposed, fixedDepthMask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv__OutputArray_create_const_int_int_int_int_bool_int(const cv::_OutputArray* instance, int rows, int cols, int type, int i, bool allowTransposed, int fixedDepthMask) {
		try {
			instance->create(rows, cols, type, i, allowTransposed, fixedDepthMask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv__OutputArray_create_const_int_const_intX_int_int_bool_int(const cv::_OutputArray* instance, int dims, const int* size, int type, int i, bool allowTransposed, int fixedDepthMask) {
		try {
			instance->create(dims, size, type, i, allowTransposed, fixedDepthMask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv__OutputArray_createSameSize_const_const__InputArrayR_int(const cv::_OutputArray* instance, const cv::_InputArray* arr, int mtype) {
		try {
			instance->createSameSize(*arr, mtype);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv__OutputArray_release_const(const cv::_OutputArray* instance) {
		try {
			instance->release();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv__OutputArray_clear_const(const cv::_OutputArray* instance) {
		try {
			instance->clear();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv__OutputArray_setTo_const_const__InputArrayR_const__InputArrayR(const cv::_OutputArray* instance, const cv::_InputArray* value, const cv::_InputArray* mask) {
		try {
			instance->setTo(*value, *mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv__OutputArray_assign_const_const_UMatR(const cv::_OutputArray* instance, const cv::UMat* u) {
		try {
			instance->assign(*u);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv__OutputArray_assign_const_const_MatR(const cv::_OutputArray* instance, const cv::Mat* m) {
		try {
			instance->assign(*m);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_DeviceInfo_delete(cv::cuda::DeviceInfo* instance) {
		delete instance;
	}
	Result<cv::cuda::DeviceInfo*> cv_cuda_DeviceInfo_DeviceInfo() {
		try {
			cv::cuda::DeviceInfo* ret = new cv::cuda::DeviceInfo();
			return Ok<cv::cuda::DeviceInfo*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::DeviceInfo*>))
	}
	
	Result<cv::cuda::DeviceInfo*> cv_cuda_DeviceInfo_DeviceInfo_int(int device_id) {
		try {
			cv::cuda::DeviceInfo* ret = new cv::cuda::DeviceInfo(device_id);
			return Ok<cv::cuda::DeviceInfo*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::DeviceInfo*>))
	}
	
	Result<int> cv_cuda_DeviceInfo_deviceID_const(const cv::cuda::DeviceInfo* instance) {
		try {
			int ret = instance->deviceID();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<void*> cv_cuda_DeviceInfo_name_const(const cv::cuda::DeviceInfo* instance) {
		try {
			const char* ret = instance->name();
			return Ok(ocvrs_create_string(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<size_t> cv_cuda_DeviceInfo_totalGlobalMem_const(const cv::cuda::DeviceInfo* instance) {
		try {
			size_t ret = instance->totalGlobalMem();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<size_t> cv_cuda_DeviceInfo_sharedMemPerBlock_const(const cv::cuda::DeviceInfo* instance) {
		try {
			size_t ret = instance->sharedMemPerBlock();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<int> cv_cuda_DeviceInfo_regsPerBlock_const(const cv::cuda::DeviceInfo* instance) {
		try {
			int ret = instance->regsPerBlock();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_cuda_DeviceInfo_warpSize_const(const cv::cuda::DeviceInfo* instance) {
		try {
			int ret = instance->warpSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<size_t> cv_cuda_DeviceInfo_memPitch_const(const cv::cuda::DeviceInfo* instance) {
		try {
			size_t ret = instance->memPitch();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<int> cv_cuda_DeviceInfo_maxThreadsPerBlock_const(const cv::cuda::DeviceInfo* instance) {
		try {
			int ret = instance->maxThreadsPerBlock();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<cv::Vec3i> cv_cuda_DeviceInfo_maxThreadsDim_const(const cv::cuda::DeviceInfo* instance) {
		try {
			cv::Vec3i ret = instance->maxThreadsDim();
			return Ok<cv::Vec3i>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec3i>))
	}
	
	Result<cv::Vec3i> cv_cuda_DeviceInfo_maxGridSize_const(const cv::cuda::DeviceInfo* instance) {
		try {
			cv::Vec3i ret = instance->maxGridSize();
			return Ok<cv::Vec3i>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec3i>))
	}
	
	Result<int> cv_cuda_DeviceInfo_clockRate_const(const cv::cuda::DeviceInfo* instance) {
		try {
			int ret = instance->clockRate();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<size_t> cv_cuda_DeviceInfo_totalConstMem_const(const cv::cuda::DeviceInfo* instance) {
		try {
			size_t ret = instance->totalConstMem();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<int> cv_cuda_DeviceInfo_majorVersion_const(const cv::cuda::DeviceInfo* instance) {
		try {
			int ret = instance->majorVersion();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_cuda_DeviceInfo_minorVersion_const(const cv::cuda::DeviceInfo* instance) {
		try {
			int ret = instance->minorVersion();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<size_t> cv_cuda_DeviceInfo_textureAlignment_const(const cv::cuda::DeviceInfo* instance) {
		try {
			size_t ret = instance->textureAlignment();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<size_t> cv_cuda_DeviceInfo_texturePitchAlignment_const(const cv::cuda::DeviceInfo* instance) {
		try {
			size_t ret = instance->texturePitchAlignment();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<int> cv_cuda_DeviceInfo_multiProcessorCount_const(const cv::cuda::DeviceInfo* instance) {
		try {
			int ret = instance->multiProcessorCount();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<bool> cv_cuda_DeviceInfo_kernelExecTimeoutEnabled_const(const cv::cuda::DeviceInfo* instance) {
		try {
			bool ret = instance->kernelExecTimeoutEnabled();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_cuda_DeviceInfo_integrated_const(const cv::cuda::DeviceInfo* instance) {
		try {
			bool ret = instance->integrated();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_cuda_DeviceInfo_canMapHostMemory_const(const cv::cuda::DeviceInfo* instance) {
		try {
			bool ret = instance->canMapHostMemory();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<cv::cuda::DeviceInfo::ComputeMode> cv_cuda_DeviceInfo_computeMode_const(const cv::cuda::DeviceInfo* instance) {
		try {
			cv::cuda::DeviceInfo::ComputeMode ret = instance->computeMode();
			return Ok<cv::cuda::DeviceInfo::ComputeMode>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::DeviceInfo::ComputeMode>))
	}
	
	Result<int> cv_cuda_DeviceInfo_maxTexture1D_const(const cv::cuda::DeviceInfo* instance) {
		try {
			int ret = instance->maxTexture1D();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_cuda_DeviceInfo_maxTexture1DMipmap_const(const cv::cuda::DeviceInfo* instance) {
		try {
			int ret = instance->maxTexture1DMipmap();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_cuda_DeviceInfo_maxTexture1DLinear_const(const cv::cuda::DeviceInfo* instance) {
		try {
			int ret = instance->maxTexture1DLinear();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<cv::Vec2i> cv_cuda_DeviceInfo_maxTexture2D_const(const cv::cuda::DeviceInfo* instance) {
		try {
			cv::Vec2i ret = instance->maxTexture2D();
			return Ok<cv::Vec2i>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec2i>))
	}
	
	Result<cv::Vec2i> cv_cuda_DeviceInfo_maxTexture2DMipmap_const(const cv::cuda::DeviceInfo* instance) {
		try {
			cv::Vec2i ret = instance->maxTexture2DMipmap();
			return Ok<cv::Vec2i>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec2i>))
	}
	
	Result<cv::Vec3i> cv_cuda_DeviceInfo_maxTexture2DLinear_const(const cv::cuda::DeviceInfo* instance) {
		try {
			cv::Vec3i ret = instance->maxTexture2DLinear();
			return Ok<cv::Vec3i>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec3i>))
	}
	
	Result<cv::Vec2i> cv_cuda_DeviceInfo_maxTexture2DGather_const(const cv::cuda::DeviceInfo* instance) {
		try {
			cv::Vec2i ret = instance->maxTexture2DGather();
			return Ok<cv::Vec2i>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec2i>))
	}
	
	Result<cv::Vec3i> cv_cuda_DeviceInfo_maxTexture3D_const(const cv::cuda::DeviceInfo* instance) {
		try {
			cv::Vec3i ret = instance->maxTexture3D();
			return Ok<cv::Vec3i>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec3i>))
	}
	
	Result<int> cv_cuda_DeviceInfo_maxTextureCubemap_const(const cv::cuda::DeviceInfo* instance) {
		try {
			int ret = instance->maxTextureCubemap();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<cv::Vec2i> cv_cuda_DeviceInfo_maxTexture1DLayered_const(const cv::cuda::DeviceInfo* instance) {
		try {
			cv::Vec2i ret = instance->maxTexture1DLayered();
			return Ok<cv::Vec2i>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec2i>))
	}
	
	Result<cv::Vec3i> cv_cuda_DeviceInfo_maxTexture2DLayered_const(const cv::cuda::DeviceInfo* instance) {
		try {
			cv::Vec3i ret = instance->maxTexture2DLayered();
			return Ok<cv::Vec3i>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec3i>))
	}
	
	Result<cv::Vec2i> cv_cuda_DeviceInfo_maxTextureCubemapLayered_const(const cv::cuda::DeviceInfo* instance) {
		try {
			cv::Vec2i ret = instance->maxTextureCubemapLayered();
			return Ok<cv::Vec2i>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec2i>))
	}
	
	Result<int> cv_cuda_DeviceInfo_maxSurface1D_const(const cv::cuda::DeviceInfo* instance) {
		try {
			int ret = instance->maxSurface1D();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<cv::Vec2i> cv_cuda_DeviceInfo_maxSurface2D_const(const cv::cuda::DeviceInfo* instance) {
		try {
			cv::Vec2i ret = instance->maxSurface2D();
			return Ok<cv::Vec2i>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec2i>))
	}
	
	Result<cv::Vec3i> cv_cuda_DeviceInfo_maxSurface3D_const(const cv::cuda::DeviceInfo* instance) {
		try {
			cv::Vec3i ret = instance->maxSurface3D();
			return Ok<cv::Vec3i>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec3i>))
	}
	
	Result<cv::Vec2i> cv_cuda_DeviceInfo_maxSurface1DLayered_const(const cv::cuda::DeviceInfo* instance) {
		try {
			cv::Vec2i ret = instance->maxSurface1DLayered();
			return Ok<cv::Vec2i>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec2i>))
	}
	
	Result<cv::Vec3i> cv_cuda_DeviceInfo_maxSurface2DLayered_const(const cv::cuda::DeviceInfo* instance) {
		try {
			cv::Vec3i ret = instance->maxSurface2DLayered();
			return Ok<cv::Vec3i>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec3i>))
	}
	
	Result<int> cv_cuda_DeviceInfo_maxSurfaceCubemap_const(const cv::cuda::DeviceInfo* instance) {
		try {
			int ret = instance->maxSurfaceCubemap();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<cv::Vec2i> cv_cuda_DeviceInfo_maxSurfaceCubemapLayered_const(const cv::cuda::DeviceInfo* instance) {
		try {
			cv::Vec2i ret = instance->maxSurfaceCubemapLayered();
			return Ok<cv::Vec2i>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec2i>))
	}
	
	Result<size_t> cv_cuda_DeviceInfo_surfaceAlignment_const(const cv::cuda::DeviceInfo* instance) {
		try {
			size_t ret = instance->surfaceAlignment();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<bool> cv_cuda_DeviceInfo_concurrentKernels_const(const cv::cuda::DeviceInfo* instance) {
		try {
			bool ret = instance->concurrentKernels();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_cuda_DeviceInfo_ECCEnabled_const(const cv::cuda::DeviceInfo* instance) {
		try {
			bool ret = instance->ECCEnabled();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<int> cv_cuda_DeviceInfo_pciBusID_const(const cv::cuda::DeviceInfo* instance) {
		try {
			int ret = instance->pciBusID();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_cuda_DeviceInfo_pciDeviceID_const(const cv::cuda::DeviceInfo* instance) {
		try {
			int ret = instance->pciDeviceID();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_cuda_DeviceInfo_pciDomainID_const(const cv::cuda::DeviceInfo* instance) {
		try {
			int ret = instance->pciDomainID();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<bool> cv_cuda_DeviceInfo_tccDriver_const(const cv::cuda::DeviceInfo* instance) {
		try {
			bool ret = instance->tccDriver();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<int> cv_cuda_DeviceInfo_asyncEngineCount_const(const cv::cuda::DeviceInfo* instance) {
		try {
			int ret = instance->asyncEngineCount();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<bool> cv_cuda_DeviceInfo_unifiedAddressing_const(const cv::cuda::DeviceInfo* instance) {
		try {
			bool ret = instance->unifiedAddressing();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<int> cv_cuda_DeviceInfo_memoryClockRate_const(const cv::cuda::DeviceInfo* instance) {
		try {
			int ret = instance->memoryClockRate();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_cuda_DeviceInfo_memoryBusWidth_const(const cv::cuda::DeviceInfo* instance) {
		try {
			int ret = instance->memoryBusWidth();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_cuda_DeviceInfo_l2CacheSize_const(const cv::cuda::DeviceInfo* instance) {
		try {
			int ret = instance->l2CacheSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_cuda_DeviceInfo_maxThreadsPerMultiProcessor_const(const cv::cuda::DeviceInfo* instance) {
		try {
			int ret = instance->maxThreadsPerMultiProcessor();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_DeviceInfo_queryMemory_const_size_tR_size_tR(const cv::cuda::DeviceInfo* instance, size_t* totalMemory, size_t* freeMemory) {
		try {
			instance->queryMemory(*totalMemory, *freeMemory);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<size_t> cv_cuda_DeviceInfo_freeMemory_const(const cv::cuda::DeviceInfo* instance) {
		try {
			size_t ret = instance->freeMemory();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<size_t> cv_cuda_DeviceInfo_totalMemory_const(const cv::cuda::DeviceInfo* instance) {
		try {
			size_t ret = instance->totalMemory();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<bool> cv_cuda_DeviceInfo_supports_const_FeatureSet(const cv::cuda::DeviceInfo* instance, cv::cuda::FeatureSet feature_set) {
		try {
			bool ret = instance->supports(feature_set);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_cuda_DeviceInfo_isCompatible_const(const cv::cuda::DeviceInfo* instance) {
		try {
			bool ret = instance->isCompatible();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	void cv_Event_delete(cv::cuda::Event* instance) {
		delete instance;
	}
	Result<cv::cuda::Event*> cv_cuda_Event_Event_CreateFlags(cv::cuda::Event::CreateFlags flags) {
		try {
			cv::cuda::Event* ret = new cv::cuda::Event(flags);
			return Ok<cv::cuda::Event*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::Event*>))
	}
	
	Result_void cv_cuda_Event_record_StreamR(cv::cuda::Event* instance, cv::cuda::Stream* stream) {
		try {
			instance->record(*stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_cuda_Event_queryIfComplete_const(const cv::cuda::Event* instance) {
		try {
			bool ret = instance->queryIfComplete();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_cuda_Event_waitForCompletion(cv::cuda::Event* instance) {
		try {
			instance->waitForCompletion();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_cuda_Event_elapsedTime_const_EventR_const_EventR(const cv::cuda::Event* start, const cv::cuda::Event* end) {
		try {
			float ret = cv::cuda::Event::elapsedTime(*start, *end);
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result<int> cv_cuda_GpuMat_getPropFlags_const(const cv::cuda::GpuMat* instance) {
		try {
			int ret = instance->flags;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_GpuMat_setPropFlags_int(cv::cuda::GpuMat* instance, int val) {
		try {
			instance->flags = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_GpuMat_getPropRows_const(const cv::cuda::GpuMat* instance) {
		try {
			int ret = instance->rows;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_GpuMat_setPropRows_int(cv::cuda::GpuMat* instance, int val) {
		try {
			instance->rows = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_GpuMat_getPropCols_const(const cv::cuda::GpuMat* instance) {
		try {
			int ret = instance->cols;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_GpuMat_setPropCols_int(cv::cuda::GpuMat* instance, int val) {
		try {
			instance->cols = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<size_t> cv_cuda_GpuMat_getPropStep_const(const cv::cuda::GpuMat* instance) {
		try {
			size_t ret = instance->step;
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result_void cv_cuda_GpuMat_setPropStep_size_t(cv::cuda::GpuMat* instance, size_t val) {
		try {
			instance->step = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<unsigned char*> cv_cuda_GpuMat_getPropData(cv::cuda::GpuMat* instance) {
		try {
			unsigned char* ret = instance->data;
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	Result_void cv_cuda_GpuMat_setPropData_unsigned_charX(cv::cuda::GpuMat* instance, unsigned char* val) {
		try {
			instance->data = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int*> cv_cuda_GpuMat_getPropRefcount(cv::cuda::GpuMat* instance) {
		try {
			int* ret = instance->refcount;
			return Ok<int*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int*>))
	}
	
	Result_void cv_cuda_GpuMat_setPropRefcount_intX(cv::cuda::GpuMat* instance, int* val) {
		try {
			instance->refcount = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<unsigned char*> cv_cuda_GpuMat_getPropDatastart(cv::cuda::GpuMat* instance) {
		try {
			unsigned char* ret = instance->datastart;
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	Result_void cv_cuda_GpuMat_setPropDatastart_unsigned_charX(cv::cuda::GpuMat* instance, unsigned char* val) {
		try {
			instance->datastart = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<const unsigned char*> cv_cuda_GpuMat_getPropDataend_const(const cv::cuda::GpuMat* instance) {
		try {
			const unsigned char* ret = instance->dataend;
			return Ok<const unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const unsigned char*>))
	}
	
	Result<cv::cuda::GpuMat::Allocator*> cv_cuda_GpuMat_getPropAllocator(cv::cuda::GpuMat* instance) {
		try {
			cv::cuda::GpuMat::Allocator* ret = instance->allocator;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat::Allocator*>))
	}
	
	Result_void cv_cuda_GpuMat_setPropAllocator_AllocatorX(cv::cuda::GpuMat* instance, cv::cuda::GpuMat::Allocator* val) {
		try {
			instance->allocator = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_GpuMat_delete(cv::cuda::GpuMat* instance) {
		delete instance;
	}
	Result<cv::cuda::GpuMat::Allocator*> cv_cuda_GpuMat_defaultAllocator() {
		try {
			cv::cuda::GpuMat::Allocator* ret = cv::cuda::GpuMat::defaultAllocator();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat::Allocator*>))
	}
	
	Result_void cv_cuda_GpuMat_setDefaultAllocator_AllocatorX(cv::cuda::GpuMat::Allocator* allocator) {
		try {
			cv::cuda::GpuMat::setDefaultAllocator(allocator);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::cuda::GpuMat*> cv_cuda_GpuMat_GpuMat_AllocatorX(cv::cuda::GpuMat::Allocator* allocator) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(allocator);
			return Ok<cv::cuda::GpuMat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	Result<cv::cuda::GpuMat*> cv_cuda_GpuMat_GpuMat_int_int_int_AllocatorX(int rows, int cols, int type, cv::cuda::GpuMat::Allocator* allocator) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(rows, cols, type, allocator);
			return Ok<cv::cuda::GpuMat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	Result<cv::cuda::GpuMat*> cv_cuda_GpuMat_GpuMat_Size_int_AllocatorX(cv::Size* size, int type, cv::cuda::GpuMat::Allocator* allocator) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(*size, type, allocator);
			return Ok<cv::cuda::GpuMat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	Result<cv::cuda::GpuMat*> cv_cuda_GpuMat_GpuMat_int_int_int_Scalar_AllocatorX(int rows, int cols, int type, cv::Scalar* s, cv::cuda::GpuMat::Allocator* allocator) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(rows, cols, type, *s, allocator);
			return Ok<cv::cuda::GpuMat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	Result<cv::cuda::GpuMat*> cv_cuda_GpuMat_GpuMat_Size_int_Scalar_AllocatorX(cv::Size* size, int type, cv::Scalar* s, cv::cuda::GpuMat::Allocator* allocator) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(*size, type, *s, allocator);
			return Ok<cv::cuda::GpuMat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	Result<cv::cuda::GpuMat*> cv_cuda_GpuMat_GpuMat_const_GpuMatR(const cv::cuda::GpuMat* m) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(*m);
			return Ok<cv::cuda::GpuMat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	Result<cv::cuda::GpuMat*> cv_cuda_GpuMat_GpuMat_int_int_int_voidX_size_t(int rows, int cols, int type, void* data, size_t step) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(rows, cols, type, data, step);
			return Ok<cv::cuda::GpuMat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	Result<cv::cuda::GpuMat*> cv_cuda_GpuMat_GpuMat_Size_int_voidX_size_t(cv::Size* size, int type, void* data, size_t step) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(*size, type, data, step);
			return Ok<cv::cuda::GpuMat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	Result<cv::cuda::GpuMat*> cv_cuda_GpuMat_GpuMat_const_GpuMatR_Range_Range(const cv::cuda::GpuMat* m, cv::Range* rowRange, cv::Range* colRange) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(*m, *rowRange, *colRange);
			return Ok<cv::cuda::GpuMat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	Result<cv::cuda::GpuMat*> cv_cuda_GpuMat_GpuMat_const_GpuMatR_Rect(const cv::cuda::GpuMat* m, cv::Rect* roi) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(*m, *roi);
			return Ok<cv::cuda::GpuMat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	Result<cv::cuda::GpuMat*> cv_cuda_GpuMat_GpuMat_const__InputArrayR_AllocatorX(const cv::_InputArray* arr, cv::cuda::GpuMat::Allocator* allocator) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(*arr, allocator);
			return Ok<cv::cuda::GpuMat*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	Result_void cv_cuda_GpuMat_create_int_int_int(cv::cuda::GpuMat* instance, int rows, int cols, int type) {
		try {
			instance->create(rows, cols, type);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_GpuMat_create_Size_int(cv::cuda::GpuMat* instance, cv::Size* size, int type) {
		try {
			instance->create(*size, type);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_GpuMat_release(cv::cuda::GpuMat* instance) {
		try {
			instance->release();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_GpuMat_swap_GpuMatR(cv::cuda::GpuMat* instance, cv::cuda::GpuMat* mat) {
		try {
			instance->swap(*mat);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_GpuMat_upload_const__InputArrayR(cv::cuda::GpuMat* instance, const cv::_InputArray* arr) {
		try {
			instance->upload(*arr);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_GpuMat_upload_const__InputArrayR_StreamR(cv::cuda::GpuMat* instance, const cv::_InputArray* arr, cv::cuda::Stream* stream) {
		try {
			instance->upload(*arr, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_GpuMat_download_const_const__OutputArrayR(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst) {
		try {
			instance->download(*dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_GpuMat_download_const_const__OutputArrayR_StreamR(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, cv::cuda::Stream* stream) {
		try {
			instance->download(*dst, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::cuda::GpuMat*> cv_cuda_GpuMat_clone_const(const cv::cuda::GpuMat* instance) {
		try {
			cv::cuda::GpuMat ret = instance->clone();
			return Ok(new cv::cuda::GpuMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	Result_void cv_cuda_GpuMat_copyTo_const_const__OutputArrayR(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst) {
		try {
			instance->copyTo(*dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_GpuMat_copyTo_const_const__OutputArrayR_StreamR(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, cv::cuda::Stream* stream) {
		try {
			instance->copyTo(*dst, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_GpuMat_copyTo_const_const__OutputArrayR_const__InputArrayR(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, const cv::_InputArray* mask) {
		try {
			instance->copyTo(*dst, *mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_GpuMat_copyTo_const_const__OutputArrayR_const__InputArrayR_StreamR(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, const cv::_InputArray* mask, cv::cuda::Stream* stream) {
		try {
			instance->copyTo(*dst, *mask, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::cuda::GpuMat*> cv_cuda_GpuMat_setTo_Scalar(cv::cuda::GpuMat* instance, cv::Scalar* s) {
		try {
			cv::cuda::GpuMat ret = instance->setTo(*s);
			return Ok(new cv::cuda::GpuMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	Result<cv::cuda::GpuMat*> cv_cuda_GpuMat_setTo_Scalar_StreamR(cv::cuda::GpuMat* instance, cv::Scalar* s, cv::cuda::Stream* stream) {
		try {
			cv::cuda::GpuMat ret = instance->setTo(*s, *stream);
			return Ok(new cv::cuda::GpuMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	Result<cv::cuda::GpuMat*> cv_cuda_GpuMat_setTo_Scalar_const__InputArrayR(cv::cuda::GpuMat* instance, cv::Scalar* s, const cv::_InputArray* mask) {
		try {
			cv::cuda::GpuMat ret = instance->setTo(*s, *mask);
			return Ok(new cv::cuda::GpuMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	Result<cv::cuda::GpuMat*> cv_cuda_GpuMat_setTo_Scalar_const__InputArrayR_StreamR(cv::cuda::GpuMat* instance, cv::Scalar* s, const cv::_InputArray* mask, cv::cuda::Stream* stream) {
		try {
			cv::cuda::GpuMat ret = instance->setTo(*s, *mask, *stream);
			return Ok(new cv::cuda::GpuMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	Result_void cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, int rtype) {
		try {
			instance->convertTo(*dst, rtype);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int_StreamR(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, int rtype, cv::cuda::Stream* stream) {
		try {
			instance->convertTo(*dst, rtype, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int_double_double(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, int rtype, double alpha, double beta) {
		try {
			instance->convertTo(*dst, rtype, alpha, beta);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int_double_StreamR(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, int rtype, double alpha, cv::cuda::Stream* stream) {
		try {
			instance->convertTo(*dst, rtype, alpha, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int_double_double_StreamR(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, int rtype, double alpha, double beta, cv::cuda::Stream* stream) {
		try {
			instance->convertTo(*dst, rtype, alpha, beta, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_GpuMat_assignTo_const_GpuMatR_int(const cv::cuda::GpuMat* instance, cv::cuda::GpuMat* m, int type) {
		try {
			instance->assignTo(*m, type);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<unsigned char*> cv_cuda_GpuMat_ptr_int(cv::cuda::GpuMat* instance, int y) {
		try {
			unsigned char* ret = instance->ptr(y);
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	Result<const unsigned char*> cv_cuda_GpuMat_ptr_const_int(const cv::cuda::GpuMat* instance, int y) {
		try {
			const unsigned char* ret = instance->ptr(y);
			return Ok<const unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const unsigned char*>))
	}
	
	Result<cv::cuda::GpuMat*> cv_cuda_GpuMat_row_const_int(const cv::cuda::GpuMat* instance, int y) {
		try {
			cv::cuda::GpuMat ret = instance->row(y);
			return Ok(new cv::cuda::GpuMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	Result<cv::cuda::GpuMat*> cv_cuda_GpuMat_col_const_int(const cv::cuda::GpuMat* instance, int x) {
		try {
			cv::cuda::GpuMat ret = instance->col(x);
			return Ok(new cv::cuda::GpuMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	Result<cv::cuda::GpuMat*> cv_cuda_GpuMat_rowRange_const_int_int(const cv::cuda::GpuMat* instance, int startrow, int endrow) {
		try {
			cv::cuda::GpuMat ret = instance->rowRange(startrow, endrow);
			return Ok(new cv::cuda::GpuMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	Result<cv::cuda::GpuMat*> cv_cuda_GpuMat_rowRange_const_Range(const cv::cuda::GpuMat* instance, cv::Range* r) {
		try {
			cv::cuda::GpuMat ret = instance->rowRange(*r);
			return Ok(new cv::cuda::GpuMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	Result<cv::cuda::GpuMat*> cv_cuda_GpuMat_colRange_const_int_int(const cv::cuda::GpuMat* instance, int startcol, int endcol) {
		try {
			cv::cuda::GpuMat ret = instance->colRange(startcol, endcol);
			return Ok(new cv::cuda::GpuMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	Result<cv::cuda::GpuMat*> cv_cuda_GpuMat_colRange_const_Range(const cv::cuda::GpuMat* instance, cv::Range* r) {
		try {
			cv::cuda::GpuMat ret = instance->colRange(*r);
			return Ok(new cv::cuda::GpuMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	Result<cv::cuda::GpuMat*> cv_cuda_GpuMat_reshape_const_int_int(const cv::cuda::GpuMat* instance, int cn, int rows) {
		try {
			cv::cuda::GpuMat ret = instance->reshape(cn, rows);
			return Ok(new cv::cuda::GpuMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	Result_void cv_cuda_GpuMat_locateROI_const_SizeR_PointR(const cv::cuda::GpuMat* instance, cv::Size* wholeSize, cv::Point* ofs) {
		try {
			instance->locateROI(*wholeSize, *ofs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::cuda::GpuMat*> cv_cuda_GpuMat_adjustROI_int_int_int_int(cv::cuda::GpuMat* instance, int dtop, int dbottom, int dleft, int dright) {
		try {
			cv::cuda::GpuMat ret = instance->adjustROI(dtop, dbottom, dleft, dright);
			return Ok(new cv::cuda::GpuMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	Result<bool> cv_cuda_GpuMat_isContinuous_const(const cv::cuda::GpuMat* instance) {
		try {
			bool ret = instance->isContinuous();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<size_t> cv_cuda_GpuMat_elemSize_const(const cv::cuda::GpuMat* instance) {
		try {
			size_t ret = instance->elemSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<size_t> cv_cuda_GpuMat_elemSize1_const(const cv::cuda::GpuMat* instance) {
		try {
			size_t ret = instance->elemSize1();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<int> cv_cuda_GpuMat_type_const(const cv::cuda::GpuMat* instance) {
		try {
			int ret = instance->type();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_cuda_GpuMat_depth_const(const cv::cuda::GpuMat* instance) {
		try {
			int ret = instance->depth();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_cuda_GpuMat_channels_const(const cv::cuda::GpuMat* instance) {
		try {
			int ret = instance->channels();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<size_t> cv_cuda_GpuMat_step1_const(const cv::cuda::GpuMat* instance) {
		try {
			size_t ret = instance->step1();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<cv::Size> cv_cuda_GpuMat_size_const(const cv::cuda::GpuMat* instance) {
		try {
			cv::Size ret = instance->size();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result<bool> cv_cuda_GpuMat_empty_const(const cv::cuda::GpuMat* instance) {
		try {
			bool ret = instance->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_cuda_GpuMat_Allocator_allocate_GpuMatX_int_int_size_t(cv::cuda::GpuMat::Allocator* instance, cv::cuda::GpuMat* mat, int rows, int cols, size_t elemSize) {
		try {
			bool ret = instance->allocate(mat, rows, cols, elemSize);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_cuda_GpuMat_Allocator_free_GpuMatX(cv::cuda::GpuMat::Allocator* instance, cv::cuda::GpuMat* mat) {
		try {
			instance->free(mat);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_HostMem_getPropFlags_const(const cv::cuda::HostMem* instance) {
		try {
			int ret = instance->flags;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_HostMem_setPropFlags_int(cv::cuda::HostMem* instance, int val) {
		try {
			instance->flags = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_HostMem_getPropRows_const(const cv::cuda::HostMem* instance) {
		try {
			int ret = instance->rows;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_HostMem_setPropRows_int(cv::cuda::HostMem* instance, int val) {
		try {
			instance->rows = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_HostMem_getPropCols_const(const cv::cuda::HostMem* instance) {
		try {
			int ret = instance->cols;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_HostMem_setPropCols_int(cv::cuda::HostMem* instance, int val) {
		try {
			instance->cols = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<size_t> cv_cuda_HostMem_getPropStep_const(const cv::cuda::HostMem* instance) {
		try {
			size_t ret = instance->step;
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result_void cv_cuda_HostMem_setPropStep_size_t(cv::cuda::HostMem* instance, size_t val) {
		try {
			instance->step = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<unsigned char*> cv_cuda_HostMem_getPropData(cv::cuda::HostMem* instance) {
		try {
			unsigned char* ret = instance->data;
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	Result_void cv_cuda_HostMem_setPropData_unsigned_charX(cv::cuda::HostMem* instance, unsigned char* val) {
		try {
			instance->data = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int*> cv_cuda_HostMem_getPropRefcount(cv::cuda::HostMem* instance) {
		try {
			int* ret = instance->refcount;
			return Ok<int*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int*>))
	}
	
	Result_void cv_cuda_HostMem_setPropRefcount_intX(cv::cuda::HostMem* instance, int* val) {
		try {
			instance->refcount = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<unsigned char*> cv_cuda_HostMem_getPropDatastart(cv::cuda::HostMem* instance) {
		try {
			unsigned char* ret = instance->datastart;
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	Result_void cv_cuda_HostMem_setPropDatastart_unsigned_charX(cv::cuda::HostMem* instance, unsigned char* val) {
		try {
			instance->datastart = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<const unsigned char*> cv_cuda_HostMem_getPropDataend_const(const cv::cuda::HostMem* instance) {
		try {
			const unsigned char* ret = instance->dataend;
			return Ok<const unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const unsigned char*>))
	}
	
	Result<cv::cuda::HostMem::AllocType> cv_cuda_HostMem_getPropAlloc_type_const(const cv::cuda::HostMem* instance) {
		try {
			cv::cuda::HostMem::AllocType ret = instance->alloc_type;
			return Ok<cv::cuda::HostMem::AllocType>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::HostMem::AllocType>))
	}
	
	Result_void cv_cuda_HostMem_setPropAlloc_type_AllocType(cv::cuda::HostMem* instance, cv::cuda::HostMem::AllocType val) {
		try {
			instance->alloc_type = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_HostMem_delete(cv::cuda::HostMem* instance) {
		delete instance;
	}
	Result<cv::cuda::HostMem*> cv_cuda_HostMem_HostMem_AllocType(cv::cuda::HostMem::AllocType alloc_type) {
		try {
			cv::cuda::HostMem* ret = new cv::cuda::HostMem(alloc_type);
			return Ok<cv::cuda::HostMem*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::HostMem*>))
	}
	
	Result<cv::cuda::HostMem*> cv_cuda_HostMem_HostMem_const_HostMemR(const cv::cuda::HostMem* m) {
		try {
			cv::cuda::HostMem* ret = new cv::cuda::HostMem(*m);
			return Ok<cv::cuda::HostMem*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::HostMem*>))
	}
	
	Result<cv::cuda::HostMem*> cv_cuda_HostMem_HostMem_int_int_int_AllocType(int rows, int cols, int type, cv::cuda::HostMem::AllocType alloc_type) {
		try {
			cv::cuda::HostMem* ret = new cv::cuda::HostMem(rows, cols, type, alloc_type);
			return Ok<cv::cuda::HostMem*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::HostMem*>))
	}
	
	Result<cv::cuda::HostMem*> cv_cuda_HostMem_HostMem_Size_int_AllocType(cv::Size* size, int type, cv::cuda::HostMem::AllocType alloc_type) {
		try {
			cv::cuda::HostMem* ret = new cv::cuda::HostMem(*size, type, alloc_type);
			return Ok<cv::cuda::HostMem*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::HostMem*>))
	}
	
	Result<cv::cuda::HostMem*> cv_cuda_HostMem_HostMem_const__InputArrayR_AllocType(const cv::_InputArray* arr, cv::cuda::HostMem::AllocType alloc_type) {
		try {
			cv::cuda::HostMem* ret = new cv::cuda::HostMem(*arr, alloc_type);
			return Ok<cv::cuda::HostMem*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::HostMem*>))
	}
	
	Result_void cv_cuda_HostMem_swap_HostMemR(cv::cuda::HostMem* instance, cv::cuda::HostMem* b) {
		try {
			instance->swap(*b);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::cuda::HostMem*> cv_cuda_HostMem_clone_const(const cv::cuda::HostMem* instance) {
		try {
			cv::cuda::HostMem ret = instance->clone();
			return Ok(new cv::cuda::HostMem(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::HostMem*>))
	}
	
	Result_void cv_cuda_HostMem_create_int_int_int(cv::cuda::HostMem* instance, int rows, int cols, int type) {
		try {
			instance->create(rows, cols, type);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_HostMem_create_Size_int(cv::cuda::HostMem* instance, cv::Size* size, int type) {
		try {
			instance->create(*size, type);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::cuda::HostMem*> cv_cuda_HostMem_reshape_const_int_int(const cv::cuda::HostMem* instance, int cn, int rows) {
		try {
			cv::cuda::HostMem ret = instance->reshape(cn, rows);
			return Ok(new cv::cuda::HostMem(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::HostMem*>))
	}
	
	Result_void cv_cuda_HostMem_release(cv::cuda::HostMem* instance) {
		try {
			instance->release();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_cuda_HostMem_createMatHeader_const(const cv::cuda::HostMem* instance) {
		try {
			cv::Mat ret = instance->createMatHeader();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::cuda::GpuMat*> cv_cuda_HostMem_createGpuMatHeader_const(const cv::cuda::HostMem* instance) {
		try {
			cv::cuda::GpuMat ret = instance->createGpuMatHeader();
			return Ok(new cv::cuda::GpuMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	Result<bool> cv_cuda_HostMem_isContinuous_const(const cv::cuda::HostMem* instance) {
		try {
			bool ret = instance->isContinuous();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<size_t> cv_cuda_HostMem_elemSize_const(const cv::cuda::HostMem* instance) {
		try {
			size_t ret = instance->elemSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<size_t> cv_cuda_HostMem_elemSize1_const(const cv::cuda::HostMem* instance) {
		try {
			size_t ret = instance->elemSize1();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<int> cv_cuda_HostMem_type_const(const cv::cuda::HostMem* instance) {
		try {
			int ret = instance->type();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_cuda_HostMem_depth_const(const cv::cuda::HostMem* instance) {
		try {
			int ret = instance->depth();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_cuda_HostMem_channels_const(const cv::cuda::HostMem* instance) {
		try {
			int ret = instance->channels();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<size_t> cv_cuda_HostMem_step1_const(const cv::cuda::HostMem* instance) {
		try {
			size_t ret = instance->step1();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<cv::Size> cv_cuda_HostMem_size_const(const cv::cuda::HostMem* instance) {
		try {
			cv::Size ret = instance->size();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result<bool> cv_cuda_HostMem_empty_const(const cv::cuda::HostMem* instance) {
		try {
			bool ret = instance->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	void cv_Stream_delete(cv::cuda::Stream* instance) {
		delete instance;
	}
	Result<cv::cuda::Stream*> cv_cuda_Stream_Stream() {
		try {
			cv::cuda::Stream* ret = new cv::cuda::Stream();
			return Ok<cv::cuda::Stream*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::Stream*>))
	}
	
	Result<bool> cv_cuda_Stream_queryIfComplete_const(const cv::cuda::Stream* instance) {
		try {
			bool ret = instance->queryIfComplete();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_cuda_Stream_waitForCompletion(cv::cuda::Stream* instance) {
		try {
			instance->waitForCompletion();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_Stream_waitEvent_const_EventR(cv::cuda::Stream* instance, const cv::cuda::Event* event) {
		try {
			instance->waitEvent(*event);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_Stream_enqueueHostCallback_StreamCallback_voidX(cv::cuda::Stream* instance, cv::cuda::Stream::StreamCallback callback, void* userData) {
		try {
			instance->enqueueHostCallback(callback, userData);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::cuda::Stream*> cv_cuda_Stream_Null() {
		try {
			cv::cuda::Stream ret = cv::cuda::Stream::Null();
			return Ok(new cv::cuda::Stream(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::Stream*>))
	}
	
	void cv_TargetArchs_delete(cv::cuda::TargetArchs* instance) {
		delete instance;
	}
	Result<bool> cv_cuda_TargetArchs_builtWith_FeatureSet(cv::cuda::FeatureSet feature_set) {
		try {
			bool ret = cv::cuda::TargetArchs::builtWith(feature_set);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_cuda_TargetArchs_has_int_int(int major, int minor) {
		try {
			bool ret = cv::cuda::TargetArchs::has(major, minor);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_cuda_TargetArchs_hasPtx_int_int(int major, int minor) {
		try {
			bool ret = cv::cuda::TargetArchs::hasPtx(major, minor);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_cuda_TargetArchs_hasBin_int_int(int major, int minor) {
		try {
			bool ret = cv::cuda::TargetArchs::hasBin(major, minor);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_cuda_TargetArchs_hasEqualOrLessPtx_int_int(int major, int minor) {
		try {
			bool ret = cv::cuda::TargetArchs::hasEqualOrLessPtx(major, minor);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_cuda_TargetArchs_hasEqualOrGreater_int_int(int major, int minor) {
		try {
			bool ret = cv::cuda::TargetArchs::hasEqualOrGreater(major, minor);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_cuda_TargetArchs_hasEqualOrGreaterPtx_int_int(int major, int minor) {
		try {
			bool ret = cv::cuda::TargetArchs::hasEqualOrGreaterPtx(major, minor);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_cuda_TargetArchs_hasEqualOrGreaterBin_int_int(int major, int minor) {
		try {
			bool ret = cv::cuda::TargetArchs::hasEqualOrGreaterBin(major, minor);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<void*> cv_instr_NodeData_getPropM_funName_const(const cv::instr::NodeData* instance) {
		try {
			cv::String ret = instance->m_funName;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_instr_NodeData_setPropM_funName_String(cv::instr::NodeData* instance, char* val) {
		try {
			instance->m_funName = cv::String(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::instr::TYPE> cv_instr_NodeData_getPropM_instrType_const(const cv::instr::NodeData* instance) {
		try {
			cv::instr::TYPE ret = instance->m_instrType;
			return Ok<cv::instr::TYPE>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::instr::TYPE>))
	}
	
	Result_void cv_instr_NodeData_setPropM_instrType_TYPE(cv::instr::NodeData* instance, cv::instr::TYPE val) {
		try {
			instance->m_instrType = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::instr::IMPL> cv_instr_NodeData_getPropM_implType_const(const cv::instr::NodeData* instance) {
		try {
			cv::instr::IMPL ret = instance->m_implType;
			return Ok<cv::instr::IMPL>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::instr::IMPL>))
	}
	
	Result_void cv_instr_NodeData_setPropM_implType_IMPL(cv::instr::NodeData* instance, cv::instr::IMPL val) {
		try {
			instance->m_implType = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_instr_NodeData_getPropM_fileName_const(const cv::instr::NodeData* instance) {
		try {
			const char* ret = instance->m_fileName;
			return Ok(ocvrs_create_string(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<int> cv_instr_NodeData_getPropM_lineNum_const(const cv::instr::NodeData* instance) {
		try {
			int ret = instance->m_lineNum;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_instr_NodeData_setPropM_lineNum_int(cv::instr::NodeData* instance, int val) {
		try {
			instance->m_lineNum = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_instr_NodeData_getPropM_retAddress(cv::instr::NodeData* instance) {
		try {
			void* ret = instance->m_retAddress;
			return Ok<void*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_instr_NodeData_setPropM_retAddress_voidX(cv::instr::NodeData* instance, void* val) {
		try {
			instance->m_retAddress = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_instr_NodeData_getPropM_alwaysExpand_const(const cv::instr::NodeData* instance) {
		try {
			bool ret = instance->m_alwaysExpand;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_instr_NodeData_setPropM_alwaysExpand_bool(cv::instr::NodeData* instance, bool val) {
		try {
			instance->m_alwaysExpand = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_instr_NodeData_getPropM_funError_const(const cv::instr::NodeData* instance) {
		try {
			bool ret = instance->m_funError;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_instr_NodeData_setPropM_funError_bool(cv::instr::NodeData* instance, bool val) {
		try {
			instance->m_funError = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_instr_NodeData_getPropM_counter_const(const cv::instr::NodeData* instance) {
		try {
			int ret = instance->m_counter;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_instr_NodeData_setPropM_counter_int(cv::instr::NodeData* instance, int val) {
		try {
			instance->m_counter = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<uint64_t> cv_instr_NodeData_getPropM_ticksTotal_const(const cv::instr::NodeData* instance) {
		try {
			uint64_t ret = instance->m_ticksTotal;
			return Ok<uint64_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<uint64_t>))
	}
	
	Result_void cv_instr_NodeData_setPropM_ticksTotal_uint64_t(cv::instr::NodeData* instance, uint64_t val) {
		try {
			instance->m_ticksTotal = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_instr_NodeData_getPropM_threads_const(const cv::instr::NodeData* instance) {
		try {
			int ret = instance->m_threads;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_instr_NodeData_setPropM_threads_int(cv::instr::NodeData* instance, int val) {
		try {
			instance->m_threads = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_NodeData_delete(cv::instr::NodeData* instance) {
		delete instance;
	}
	Result<cv::instr::NodeData*> cv_instr_NodeData_NodeData_const_charX_const_charX_int_voidX_bool_TYPE_IMPL(const char* funName, const char* fileName, int lineNum, void* retAddress, bool alwaysExpand, cv::instr::TYPE instrType, cv::instr::IMPL implType) {
		try {
			cv::instr::NodeData* ret = new cv::instr::NodeData(funName, fileName, lineNum, retAddress, alwaysExpand, instrType, implType);
			return Ok<cv::instr::NodeData*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::instr::NodeData*>))
	}
	
	Result<cv::instr::NodeData*> cv_instr_NodeData_NodeData_NodeDataR(cv::instr::NodeData* ref) {
		try {
			cv::instr::NodeData* ret = new cv::instr::NodeData(*ref);
			return Ok<cv::instr::NodeData*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::instr::NodeData*>))
	}
	
	Result<double> cv_instr_NodeData_getTotalMs_const(const cv::instr::NodeData* instance) {
		try {
			double ret = instance->getTotalMs();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<double> cv_instr_NodeData_getMeanMs_const(const cv::instr::NodeData* instance) {
		try {
			double ret = instance->getMeanMs();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	void cv_WriteStructContext_delete(cv::internal::WriteStructContext* instance) {
		delete instance;
	}
	Result<cv::internal::WriteStructContext*> cv_internal_WriteStructContext_WriteStructContext_FileStorageR_const_StringR_int_const_StringR(cv::FileStorage* _fs, const char* name, int flags, const char* typeName) {
		try {
			cv::internal::WriteStructContext* ret = new cv::internal::WriteStructContext(*_fs, cv::String(name), flags, cv::String(typeName));
			return Ok<cv::internal::WriteStructContext*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::internal::WriteStructContext*>))
	}
	
	void cv_Context_delete(cv::ocl::Context* instance) {
		delete instance;
	}
	Result<cv::ocl::Context*> cv_ocl_Context_Context() {
		try {
			cv::ocl::Context* ret = new cv::ocl::Context();
			return Ok<cv::ocl::Context*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Context*>))
	}
	
	Result<cv::ocl::Context*> cv_ocl_Context_Context_int(int dtype) {
		try {
			cv::ocl::Context* ret = new cv::ocl::Context(dtype);
			return Ok<cv::ocl::Context*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Context*>))
	}
	
	Result<cv::ocl::Context*> cv_ocl_Context_Context_const_ContextR(const cv::ocl::Context* c) {
		try {
			cv::ocl::Context* ret = new cv::ocl::Context(*c);
			return Ok<cv::ocl::Context*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Context*>))
	}
	
	Result<bool> cv_ocl_Context_create(cv::ocl::Context* instance) {
		try {
			bool ret = instance->create();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_ocl_Context_create_int(cv::ocl::Context* instance, int dtype) {
		try {
			bool ret = instance->create(dtype);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<size_t> cv_ocl_Context_ndevices_const(const cv::ocl::Context* instance) {
		try {
			size_t ret = instance->ndevices();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<const cv::ocl::Device*> cv_ocl_Context_device_const_size_t(const cv::ocl::Context* instance, size_t idx) {
		try {
			const cv::ocl::Device ret = instance->device(idx);
			return Ok(new const cv::ocl::Device(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<const cv::ocl::Device*>))
	}
	
	Result<cv::ocl::Program*> cv_ocl_Context_getProg_const_ProgramSourceR_const_StringR_StringR(cv::ocl::Context* instance, const cv::ocl::ProgramSource* prog, const char* buildopt, void** errmsg) {
		try {
			cv::String errmsg_out;
			cv::ocl::Program ret = instance->getProg(*prog, cv::String(buildopt), errmsg_out);
			*errmsg = ocvrs_create_string(errmsg_out.c_str());
			return Ok(new cv::ocl::Program(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Program*>))
	}
	
	Result<cv::ocl::Context*> cv_ocl_Context_getDefault_bool(bool initialize) {
		try {
			cv::ocl::Context ret = cv::ocl::Context::getDefault(initialize);
			return Ok(new cv::ocl::Context(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Context*>))
	}
	
	Result<void*> cv_ocl_Context_ptr_const(const cv::ocl::Context* instance) {
		try {
			void* ret = instance->ptr();
			return Ok<void*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<bool> cv_ocl_Context_useSVM_const(const cv::ocl::Context* instance) {
		try {
			bool ret = instance->useSVM();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_ocl_Context_setUseSVM_bool(cv::ocl::Context* instance, bool enabled) {
		try {
			instance->setUseSVM(enabled);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Device_delete(cv::ocl::Device* instance) {
		delete instance;
	}
	Result<cv::ocl::Device*> cv_ocl_Device_Device() {
		try {
			cv::ocl::Device* ret = new cv::ocl::Device();
			return Ok<cv::ocl::Device*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Device*>))
	}
	
	Result<cv::ocl::Device*> cv_ocl_Device_Device_voidX(void* d) {
		try {
			cv::ocl::Device* ret = new cv::ocl::Device(d);
			return Ok<cv::ocl::Device*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Device*>))
	}
	
	Result<cv::ocl::Device*> cv_ocl_Device_Device_const_DeviceR(const cv::ocl::Device* d) {
		try {
			cv::ocl::Device* ret = new cv::ocl::Device(*d);
			return Ok<cv::ocl::Device*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Device*>))
	}
	
	Result_void cv_ocl_Device_set_voidX(cv::ocl::Device* instance, void* d) {
		try {
			instance->set(d);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_ocl_Device_name_const(const cv::ocl::Device* instance) {
		try {
			cv::String ret = instance->name();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<void*> cv_ocl_Device_extensions_const(const cv::ocl::Device* instance) {
		try {
			cv::String ret = instance->extensions();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<void*> cv_ocl_Device_version_const(const cv::ocl::Device* instance) {
		try {
			cv::String ret = instance->version();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<void*> cv_ocl_Device_vendorName_const(const cv::ocl::Device* instance) {
		try {
			cv::String ret = instance->vendorName();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<void*> cv_ocl_Device_OpenCL_C_Version_const(const cv::ocl::Device* instance) {
		try {
			cv::String ret = instance->OpenCL_C_Version();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<void*> cv_ocl_Device_OpenCLVersion_const(const cv::ocl::Device* instance) {
		try {
			cv::String ret = instance->OpenCLVersion();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<int> cv_ocl_Device_deviceVersionMajor_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->deviceVersionMajor();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_ocl_Device_deviceVersionMinor_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->deviceVersionMinor();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<void*> cv_ocl_Device_driverVersion_const(const cv::ocl::Device* instance) {
		try {
			cv::String ret = instance->driverVersion();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<void*> cv_ocl_Device_ptr_const(const cv::ocl::Device* instance) {
		try {
			void* ret = instance->ptr();
			return Ok<void*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<int> cv_ocl_Device_type_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->type();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_ocl_Device_addressBits_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->addressBits();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<bool> cv_ocl_Device_available_const(const cv::ocl::Device* instance) {
		try {
			bool ret = instance->available();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_ocl_Device_compilerAvailable_const(const cv::ocl::Device* instance) {
		try {
			bool ret = instance->compilerAvailable();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_ocl_Device_linkerAvailable_const(const cv::ocl::Device* instance) {
		try {
			bool ret = instance->linkerAvailable();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<int> cv_ocl_Device_doubleFPConfig_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->doubleFPConfig();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_ocl_Device_singleFPConfig_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->singleFPConfig();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_ocl_Device_halfFPConfig_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->halfFPConfig();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<bool> cv_ocl_Device_endianLittle_const(const cv::ocl::Device* instance) {
		try {
			bool ret = instance->endianLittle();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_ocl_Device_errorCorrectionSupport_const(const cv::ocl::Device* instance) {
		try {
			bool ret = instance->errorCorrectionSupport();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<int> cv_ocl_Device_executionCapabilities_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->executionCapabilities();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<size_t> cv_ocl_Device_globalMemCacheSize_const(const cv::ocl::Device* instance) {
		try {
			size_t ret = instance->globalMemCacheSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<int> cv_ocl_Device_globalMemCacheType_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->globalMemCacheType();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_ocl_Device_globalMemCacheLineSize_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->globalMemCacheLineSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<size_t> cv_ocl_Device_globalMemSize_const(const cv::ocl::Device* instance) {
		try {
			size_t ret = instance->globalMemSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<size_t> cv_ocl_Device_localMemSize_const(const cv::ocl::Device* instance) {
		try {
			size_t ret = instance->localMemSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<int> cv_ocl_Device_localMemType_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->localMemType();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<bool> cv_ocl_Device_hostUnifiedMemory_const(const cv::ocl::Device* instance) {
		try {
			bool ret = instance->hostUnifiedMemory();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_ocl_Device_imageSupport_const(const cv::ocl::Device* instance) {
		try {
			bool ret = instance->imageSupport();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_ocl_Device_imageFromBufferSupport_const(const cv::ocl::Device* instance) {
		try {
			bool ret = instance->imageFromBufferSupport();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<unsigned int> cv_ocl_Device_imagePitchAlignment_const(const cv::ocl::Device* instance) {
		try {
			unsigned int ret = instance->imagePitchAlignment();
			return Ok<unsigned int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned int>))
	}
	
	Result<unsigned int> cv_ocl_Device_imageBaseAddressAlignment_const(const cv::ocl::Device* instance) {
		try {
			unsigned int ret = instance->imageBaseAddressAlignment();
			return Ok<unsigned int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned int>))
	}
	
	Result<size_t> cv_ocl_Device_image2DMaxWidth_const(const cv::ocl::Device* instance) {
		try {
			size_t ret = instance->image2DMaxWidth();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<size_t> cv_ocl_Device_image2DMaxHeight_const(const cv::ocl::Device* instance) {
		try {
			size_t ret = instance->image2DMaxHeight();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<size_t> cv_ocl_Device_image3DMaxWidth_const(const cv::ocl::Device* instance) {
		try {
			size_t ret = instance->image3DMaxWidth();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<size_t> cv_ocl_Device_image3DMaxHeight_const(const cv::ocl::Device* instance) {
		try {
			size_t ret = instance->image3DMaxHeight();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<size_t> cv_ocl_Device_image3DMaxDepth_const(const cv::ocl::Device* instance) {
		try {
			size_t ret = instance->image3DMaxDepth();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<size_t> cv_ocl_Device_imageMaxBufferSize_const(const cv::ocl::Device* instance) {
		try {
			size_t ret = instance->imageMaxBufferSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<size_t> cv_ocl_Device_imageMaxArraySize_const(const cv::ocl::Device* instance) {
		try {
			size_t ret = instance->imageMaxArraySize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<int> cv_ocl_Device_vendorID_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->vendorID();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<bool> cv_ocl_Device_isAMD_const(const cv::ocl::Device* instance) {
		try {
			bool ret = instance->isAMD();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_ocl_Device_isIntel_const(const cv::ocl::Device* instance) {
		try {
			bool ret = instance->isIntel();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_ocl_Device_isNVidia_const(const cv::ocl::Device* instance) {
		try {
			bool ret = instance->isNVidia();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<int> cv_ocl_Device_maxClockFrequency_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->maxClockFrequency();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_ocl_Device_maxComputeUnits_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->maxComputeUnits();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_ocl_Device_maxConstantArgs_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->maxConstantArgs();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<size_t> cv_ocl_Device_maxConstantBufferSize_const(const cv::ocl::Device* instance) {
		try {
			size_t ret = instance->maxConstantBufferSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<size_t> cv_ocl_Device_maxMemAllocSize_const(const cv::ocl::Device* instance) {
		try {
			size_t ret = instance->maxMemAllocSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<size_t> cv_ocl_Device_maxParameterSize_const(const cv::ocl::Device* instance) {
		try {
			size_t ret = instance->maxParameterSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<int> cv_ocl_Device_maxReadImageArgs_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->maxReadImageArgs();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_ocl_Device_maxWriteImageArgs_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->maxWriteImageArgs();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_ocl_Device_maxSamplers_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->maxSamplers();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<size_t> cv_ocl_Device_maxWorkGroupSize_const(const cv::ocl::Device* instance) {
		try {
			size_t ret = instance->maxWorkGroupSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<int> cv_ocl_Device_maxWorkItemDims_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->maxWorkItemDims();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_ocl_Device_maxWorkItemSizes_const_size_tX(const cv::ocl::Device* instance, size_t* unnamed) {
		try {
			instance->maxWorkItemSizes(unnamed);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_ocl_Device_memBaseAddrAlign_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->memBaseAddrAlign();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_ocl_Device_nativeVectorWidthChar_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->nativeVectorWidthChar();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_ocl_Device_nativeVectorWidthShort_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->nativeVectorWidthShort();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_ocl_Device_nativeVectorWidthInt_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->nativeVectorWidthInt();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_ocl_Device_nativeVectorWidthLong_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->nativeVectorWidthLong();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_ocl_Device_nativeVectorWidthFloat_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->nativeVectorWidthFloat();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_ocl_Device_nativeVectorWidthDouble_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->nativeVectorWidthDouble();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_ocl_Device_nativeVectorWidthHalf_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->nativeVectorWidthHalf();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_ocl_Device_preferredVectorWidthChar_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->preferredVectorWidthChar();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_ocl_Device_preferredVectorWidthShort_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->preferredVectorWidthShort();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_ocl_Device_preferredVectorWidthInt_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->preferredVectorWidthInt();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_ocl_Device_preferredVectorWidthLong_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->preferredVectorWidthLong();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_ocl_Device_preferredVectorWidthFloat_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->preferredVectorWidthFloat();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_ocl_Device_preferredVectorWidthDouble_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->preferredVectorWidthDouble();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_ocl_Device_preferredVectorWidthHalf_const(const cv::ocl::Device* instance) {
		try {
			int ret = instance->preferredVectorWidthHalf();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<size_t> cv_ocl_Device_printfBufferSize_const(const cv::ocl::Device* instance) {
		try {
			size_t ret = instance->printfBufferSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<size_t> cv_ocl_Device_profilingTimerResolution_const(const cv::ocl::Device* instance) {
		try {
			size_t ret = instance->profilingTimerResolution();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<const cv::ocl::Device*> cv_ocl_Device_getDefault() {
		try {
			const cv::ocl::Device ret = cv::ocl::Device::getDefault();
			return Ok(new const cv::ocl::Device(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<const cv::ocl::Device*>))
	}
	
	void cv_Image2D_delete(cv::ocl::Image2D* instance) {
		delete instance;
	}
	Result<cv::ocl::Image2D*> cv_ocl_Image2D_Image2D() {
		try {
			cv::ocl::Image2D* ret = new cv::ocl::Image2D();
			return Ok<cv::ocl::Image2D*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Image2D*>))
	}
	
	Result<cv::ocl::Image2D*> cv_ocl_Image2D_Image2D_const_UMatR_bool_bool(const cv::UMat* src, bool norm, bool alias) {
		try {
			cv::ocl::Image2D* ret = new cv::ocl::Image2D(*src, norm, alias);
			return Ok<cv::ocl::Image2D*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Image2D*>))
	}
	
	Result<cv::ocl::Image2D*> cv_ocl_Image2D_Image2D_const_Image2DR(const cv::ocl::Image2D* i) {
		try {
			cv::ocl::Image2D* ret = new cv::ocl::Image2D(*i);
			return Ok<cv::ocl::Image2D*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Image2D*>))
	}
	
	Result<bool> cv_ocl_Image2D_canCreateAlias_const_UMatR(const cv::UMat* u) {
		try {
			bool ret = cv::ocl::Image2D::canCreateAlias(*u);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_ocl_Image2D_isFormatSupported_int_int_bool(int depth, int cn, bool norm) {
		try {
			bool ret = cv::ocl::Image2D::isFormatSupported(depth, cn, norm);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<void*> cv_ocl_Image2D_ptr_const(const cv::ocl::Image2D* instance) {
		try {
			void* ret = instance->ptr();
			return Ok<void*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	void cv_Kernel_delete(cv::ocl::Kernel* instance) {
		delete instance;
	}
	Result<cv::ocl::Kernel*> cv_ocl_Kernel_Kernel() {
		try {
			cv::ocl::Kernel* ret = new cv::ocl::Kernel();
			return Ok<cv::ocl::Kernel*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Kernel*>))
	}
	
	Result<cv::ocl::Kernel*> cv_ocl_Kernel_Kernel_const_charX_const_ProgramR(const char* kname, const cv::ocl::Program* prog) {
		try {
			cv::ocl::Kernel* ret = new cv::ocl::Kernel(kname, *prog);
			return Ok<cv::ocl::Kernel*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Kernel*>))
	}
	
	Result<cv::ocl::Kernel*> cv_ocl_Kernel_Kernel_const_charX_const_ProgramSourceR_const_StringR_StringX(const char* kname, const cv::ocl::ProgramSource* prog, const char* buildopts, void** errmsg) {
		try {
			cv::String errmsg_out;
			cv::ocl::Kernel* ret = new cv::ocl::Kernel(kname, *prog, cv::String(buildopts), &errmsg_out);
			*errmsg = ocvrs_create_string(errmsg_out.c_str());
			return Ok<cv::ocl::Kernel*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Kernel*>))
	}
	
	Result<cv::ocl::Kernel*> cv_ocl_Kernel_Kernel_const_KernelR(const cv::ocl::Kernel* k) {
		try {
			cv::ocl::Kernel* ret = new cv::ocl::Kernel(*k);
			return Ok<cv::ocl::Kernel*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Kernel*>))
	}
	
	Result<bool> cv_ocl_Kernel_empty_const(const cv::ocl::Kernel* instance) {
		try {
			bool ret = instance->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_ocl_Kernel_create_const_charX_const_ProgramR(cv::ocl::Kernel* instance, const char* kname, const cv::ocl::Program* prog) {
		try {
			bool ret = instance->create(kname, *prog);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_ocl_Kernel_create_const_charX_const_ProgramSourceR_const_StringR_StringX(cv::ocl::Kernel* instance, const char* kname, const cv::ocl::ProgramSource* prog, const char* buildopts, void** errmsg) {
		try {
			cv::String errmsg_out;
			bool ret = instance->create(kname, *prog, cv::String(buildopts), &errmsg_out);
			*errmsg = ocvrs_create_string(errmsg_out.c_str());
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<int> cv_ocl_Kernel_set_int_const_voidX_size_t(cv::ocl::Kernel* instance, int i, const void* value, size_t sz) {
		try {
			int ret = instance->set(i, value, sz);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_ocl_Kernel_set_int_const_Image2DR(cv::ocl::Kernel* instance, int i, const cv::ocl::Image2D* image2D) {
		try {
			int ret = instance->set(i, *image2D);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_ocl_Kernel_set_int_const_UMatR(cv::ocl::Kernel* instance, int i, const cv::UMat* m) {
		try {
			int ret = instance->set(i, *m);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_ocl_Kernel_set_int_const_KernelArgR(cv::ocl::Kernel* instance, int i, const cv::ocl::KernelArg* arg) {
		try {
			int ret = instance->set(i, *arg);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<bool> cv_ocl_Kernel_run_int_size_tX_size_tX_bool_const_QueueR(cv::ocl::Kernel* instance, int dims, size_t* globalsize, size_t* localsize, bool sync, const cv::ocl::Queue* q) {
		try {
			bool ret = instance->run(dims, globalsize, localsize, sync, *q);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_ocl_Kernel_runTask_bool_const_QueueR(cv::ocl::Kernel* instance, bool sync, const cv::ocl::Queue* q) {
		try {
			bool ret = instance->runTask(sync, *q);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<size_t> cv_ocl_Kernel_workGroupSize_const(const cv::ocl::Kernel* instance) {
		try {
			size_t ret = instance->workGroupSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<size_t> cv_ocl_Kernel_preferedWorkGroupSizeMultiple_const(const cv::ocl::Kernel* instance) {
		try {
			size_t ret = instance->preferedWorkGroupSizeMultiple();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<bool> cv_ocl_Kernel_compileWorkGroupSize_const_size_tX(const cv::ocl::Kernel* instance, size_t* wsz) {
		try {
			bool ret = instance->compileWorkGroupSize(wsz);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<size_t> cv_ocl_Kernel_localMemSize_const(const cv::ocl::Kernel* instance) {
		try {
			size_t ret = instance->localMemSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<void*> cv_ocl_Kernel_ptr_const(const cv::ocl::Kernel* instance) {
		try {
			void* ret = instance->ptr();
			return Ok<void*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<int> cv_ocl_KernelArg_getPropFlags_const(const cv::ocl::KernelArg* instance) {
		try {
			int ret = instance->flags;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_ocl_KernelArg_setPropFlags_int(cv::ocl::KernelArg* instance, int val) {
		try {
			instance->flags = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::UMat**> cv_ocl_KernelArg_getPropM(cv::ocl::KernelArg* instance) {
		try {
			cv::UMat* ret = instance->m;
			return Ok(new cv::UMat*(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat**>))
	}
	
	Result_void cv_ocl_KernelArg_setPropM_UMatX(cv::ocl::KernelArg* instance, cv::UMat* val) {
		try {
			instance->m = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<const void*> cv_ocl_KernelArg_getPropObj_const(const cv::ocl::KernelArg* instance) {
		try {
			const void* ret = instance->obj;
			return Ok<const void*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const void*>))
	}
	
	Result<size_t> cv_ocl_KernelArg_getPropSz_const(const cv::ocl::KernelArg* instance) {
		try {
			size_t ret = instance->sz;
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result_void cv_ocl_KernelArg_setPropSz_size_t(cv::ocl::KernelArg* instance, size_t val) {
		try {
			instance->sz = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_ocl_KernelArg_getPropWscale_const(const cv::ocl::KernelArg* instance) {
		try {
			int ret = instance->wscale;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_ocl_KernelArg_setPropWscale_int(cv::ocl::KernelArg* instance, int val) {
		try {
			instance->wscale = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_ocl_KernelArg_getPropIwscale_const(const cv::ocl::KernelArg* instance) {
		try {
			int ret = instance->iwscale;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_ocl_KernelArg_setPropIwscale_int(cv::ocl::KernelArg* instance, int val) {
		try {
			instance->iwscale = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_KernelArg_delete(cv::ocl::KernelArg* instance) {
		delete instance;
	}
	Result<cv::ocl::KernelArg*> cv_ocl_KernelArg_KernelArg_int_UMatX_int_int_const_voidX_size_t(int _flags, cv::UMat* _m, int wscale, int iwscale, const void* _obj, size_t _sz) {
		try {
			cv::ocl::KernelArg* ret = new cv::ocl::KernelArg(_flags, _m, wscale, iwscale, _obj, _sz);
			return Ok<cv::ocl::KernelArg*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::KernelArg*>))
	}
	
	Result<cv::ocl::KernelArg*> cv_ocl_KernelArg_KernelArg() {
		try {
			cv::ocl::KernelArg* ret = new cv::ocl::KernelArg();
			return Ok<cv::ocl::KernelArg*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::KernelArg*>))
	}
	
	Result<cv::ocl::KernelArg*> cv_ocl_KernelArg_Local() {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::Local();
			return Ok(new cv::ocl::KernelArg(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::KernelArg*>))
	}
	
	Result<cv::ocl::KernelArg*> cv_ocl_KernelArg_PtrWriteOnly_const_UMatR(const cv::UMat* m) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::PtrWriteOnly(*m);
			return Ok(new cv::ocl::KernelArg(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::KernelArg*>))
	}
	
	Result<cv::ocl::KernelArg*> cv_ocl_KernelArg_PtrReadOnly_const_UMatR(const cv::UMat* m) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::PtrReadOnly(*m);
			return Ok(new cv::ocl::KernelArg(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::KernelArg*>))
	}
	
	Result<cv::ocl::KernelArg*> cv_ocl_KernelArg_PtrReadWrite_const_UMatR(const cv::UMat* m) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::PtrReadWrite(*m);
			return Ok(new cv::ocl::KernelArg(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::KernelArg*>))
	}
	
	Result<cv::ocl::KernelArg*> cv_ocl_KernelArg_ReadWrite_const_UMatR_int_int(const cv::UMat* m, int wscale, int iwscale) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::ReadWrite(*m, wscale, iwscale);
			return Ok(new cv::ocl::KernelArg(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::KernelArg*>))
	}
	
	Result<cv::ocl::KernelArg*> cv_ocl_KernelArg_ReadWriteNoSize_const_UMatR_int_int(const cv::UMat* m, int wscale, int iwscale) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::ReadWriteNoSize(*m, wscale, iwscale);
			return Ok(new cv::ocl::KernelArg(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::KernelArg*>))
	}
	
	Result<cv::ocl::KernelArg*> cv_ocl_KernelArg_ReadOnly_const_UMatR_int_int(const cv::UMat* m, int wscale, int iwscale) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::ReadOnly(*m, wscale, iwscale);
			return Ok(new cv::ocl::KernelArg(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::KernelArg*>))
	}
	
	Result<cv::ocl::KernelArg*> cv_ocl_KernelArg_WriteOnly_const_UMatR_int_int(const cv::UMat* m, int wscale, int iwscale) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::WriteOnly(*m, wscale, iwscale);
			return Ok(new cv::ocl::KernelArg(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::KernelArg*>))
	}
	
	Result<cv::ocl::KernelArg*> cv_ocl_KernelArg_ReadOnlyNoSize_const_UMatR_int_int(const cv::UMat* m, int wscale, int iwscale) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::ReadOnlyNoSize(*m, wscale, iwscale);
			return Ok(new cv::ocl::KernelArg(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::KernelArg*>))
	}
	
	Result<cv::ocl::KernelArg*> cv_ocl_KernelArg_WriteOnlyNoSize_const_UMatR_int_int(const cv::UMat* m, int wscale, int iwscale) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::WriteOnlyNoSize(*m, wscale, iwscale);
			return Ok(new cv::ocl::KernelArg(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::KernelArg*>))
	}
	
	Result<cv::ocl::KernelArg*> cv_ocl_KernelArg_Constant_const_MatR(const cv::Mat* m) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::Constant(*m);
			return Ok(new cv::ocl::KernelArg(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::KernelArg*>))
	}
	
	void cv_Platform_delete(cv::ocl::Platform* instance) {
		delete instance;
	}
	Result<cv::ocl::Platform*> cv_ocl_Platform_Platform() {
		try {
			cv::ocl::Platform* ret = new cv::ocl::Platform();
			return Ok<cv::ocl::Platform*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Platform*>))
	}
	
	Result<cv::ocl::Platform*> cv_ocl_Platform_Platform_const_PlatformR(const cv::ocl::Platform* p) {
		try {
			cv::ocl::Platform* ret = new cv::ocl::Platform(*p);
			return Ok<cv::ocl::Platform*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Platform*>))
	}
	
	Result<void*> cv_ocl_Platform_ptr_const(const cv::ocl::Platform* instance) {
		try {
			void* ret = instance->ptr();
			return Ok<void*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<cv::ocl::Platform*> cv_ocl_Platform_getDefault() {
		try {
			cv::ocl::Platform ret = cv::ocl::Platform::getDefault();
			return Ok(new cv::ocl::Platform(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Platform*>))
	}
	
	void cv_PlatformInfo_delete(cv::ocl::PlatformInfo* instance) {
		delete instance;
	}
	Result<cv::ocl::PlatformInfo*> cv_ocl_PlatformInfo_PlatformInfo() {
		try {
			cv::ocl::PlatformInfo* ret = new cv::ocl::PlatformInfo();
			return Ok<cv::ocl::PlatformInfo*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::PlatformInfo*>))
	}
	
	Result<cv::ocl::PlatformInfo*> cv_ocl_PlatformInfo_PlatformInfo_voidX(void* id) {
		try {
			cv::ocl::PlatformInfo* ret = new cv::ocl::PlatformInfo(id);
			return Ok<cv::ocl::PlatformInfo*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::PlatformInfo*>))
	}
	
	Result<cv::ocl::PlatformInfo*> cv_ocl_PlatformInfo_PlatformInfo_const_PlatformInfoR(const cv::ocl::PlatformInfo* i) {
		try {
			cv::ocl::PlatformInfo* ret = new cv::ocl::PlatformInfo(*i);
			return Ok<cv::ocl::PlatformInfo*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::PlatformInfo*>))
	}
	
	Result<void*> cv_ocl_PlatformInfo_name_const(const cv::ocl::PlatformInfo* instance) {
		try {
			cv::String ret = instance->name();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<void*> cv_ocl_PlatformInfo_vendor_const(const cv::ocl::PlatformInfo* instance) {
		try {
			cv::String ret = instance->vendor();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<void*> cv_ocl_PlatformInfo_version_const(const cv::ocl::PlatformInfo* instance) {
		try {
			cv::String ret = instance->version();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<int> cv_ocl_PlatformInfo_deviceNumber_const(const cv::ocl::PlatformInfo* instance) {
		try {
			int ret = instance->deviceNumber();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_ocl_PlatformInfo_getDevice_const_DeviceR_int(const cv::ocl::PlatformInfo* instance, cv::ocl::Device* device, int d) {
		try {
			instance->getDevice(*device, d);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Program_delete(cv::ocl::Program* instance) {
		delete instance;
	}
	Result<cv::ocl::Program*> cv_ocl_Program_Program() {
		try {
			cv::ocl::Program* ret = new cv::ocl::Program();
			return Ok<cv::ocl::Program*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Program*>))
	}
	
	Result<cv::ocl::Program*> cv_ocl_Program_Program_const_ProgramSourceR_const_StringR_StringR(const cv::ocl::ProgramSource* src, const char* buildflags, void** errmsg) {
		try {
			cv::String errmsg_out;
			cv::ocl::Program* ret = new cv::ocl::Program(*src, cv::String(buildflags), errmsg_out);
			*errmsg = ocvrs_create_string(errmsg_out.c_str());
			return Ok<cv::ocl::Program*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Program*>))
	}
	
	Result<cv::ocl::Program*> cv_ocl_Program_Program_const_StringR(const char* buf) {
		try {
			cv::ocl::Program* ret = new cv::ocl::Program(cv::String(buf));
			return Ok<cv::ocl::Program*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Program*>))
	}
	
	Result<cv::ocl::Program*> cv_ocl_Program_Program_const_ProgramR(const cv::ocl::Program* prog) {
		try {
			cv::ocl::Program* ret = new cv::ocl::Program(*prog);
			return Ok<cv::ocl::Program*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Program*>))
	}
	
	Result<bool> cv_ocl_Program_create_const_ProgramSourceR_const_StringR_StringR(cv::ocl::Program* instance, const cv::ocl::ProgramSource* src, const char* buildflags, void** errmsg) {
		try {
			cv::String errmsg_out;
			bool ret = instance->create(*src, cv::String(buildflags), errmsg_out);
			*errmsg = ocvrs_create_string(errmsg_out.c_str());
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_ocl_Program_read_const_StringR_const_StringR(cv::ocl::Program* instance, const char* buf, const char* buildflags) {
		try {
			bool ret = instance->read(cv::String(buf), cv::String(buildflags));
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_ocl_Program_write_const_StringR(const cv::ocl::Program* instance, void** buf) {
		try {
			cv::String buf_out;
			bool ret = instance->write(buf_out);
			*buf = ocvrs_create_string(buf_out.c_str());
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<const cv::ocl::ProgramSource*> cv_ocl_Program_source_const(const cv::ocl::Program* instance) {
		try {
			const cv::ocl::ProgramSource ret = instance->source();
			return Ok(new const cv::ocl::ProgramSource(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<const cv::ocl::ProgramSource*>))
	}
	
	Result<void*> cv_ocl_Program_ptr_const(const cv::ocl::Program* instance) {
		try {
			void* ret = instance->ptr();
			return Ok<void*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<void*> cv_ocl_Program_getPrefix_const(const cv::ocl::Program* instance) {
		try {
			cv::String ret = instance->getPrefix();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<void*> cv_ocl_Program_getPrefix_const_StringR(const char* buildflags) {
		try {
			cv::String ret = cv::ocl::Program::getPrefix(cv::String(buildflags));
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	void cv_ProgramSource_delete(cv::ocl::ProgramSource* instance) {
		delete instance;
	}
	Result<cv::ocl::ProgramSource*> cv_ocl_ProgramSource_ProgramSource() {
		try {
			cv::ocl::ProgramSource* ret = new cv::ocl::ProgramSource();
			return Ok<cv::ocl::ProgramSource*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::ProgramSource*>))
	}
	
	Result<cv::ocl::ProgramSource*> cv_ocl_ProgramSource_ProgramSource_const_StringR(const char* prog) {
		try {
			cv::ocl::ProgramSource* ret = new cv::ocl::ProgramSource(cv::String(prog));
			return Ok<cv::ocl::ProgramSource*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::ProgramSource*>))
	}
	
	Result<cv::ocl::ProgramSource*> cv_ocl_ProgramSource_ProgramSource_const_ProgramSourceR(const cv::ocl::ProgramSource* prog) {
		try {
			cv::ocl::ProgramSource* ret = new cv::ocl::ProgramSource(*prog);
			return Ok<cv::ocl::ProgramSource*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::ProgramSource*>))
	}
	
	Result<void*> cv_ocl_ProgramSource_source_const(const cv::ocl::ProgramSource* instance) {
		try {
			const cv::String ret = instance->source();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<cv::ocl::ProgramSource::hash_t> cv_ocl_ProgramSource_hash_const(const cv::ocl::ProgramSource* instance) {
		try {
			cv::ocl::ProgramSource::hash_t ret = instance->hash();
			return Ok<cv::ocl::ProgramSource::hash_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::ProgramSource::hash_t>))
	}
	
	void cv_Queue_delete(cv::ocl::Queue* instance) {
		delete instance;
	}
	Result<cv::ocl::Queue*> cv_ocl_Queue_Queue() {
		try {
			cv::ocl::Queue* ret = new cv::ocl::Queue();
			return Ok<cv::ocl::Queue*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Queue*>))
	}
	
	Result<cv::ocl::Queue*> cv_ocl_Queue_Queue_const_ContextR_const_DeviceR(const cv::ocl::Context* c, const cv::ocl::Device* d) {
		try {
			cv::ocl::Queue* ret = new cv::ocl::Queue(*c, *d);
			return Ok<cv::ocl::Queue*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Queue*>))
	}
	
	Result<cv::ocl::Queue*> cv_ocl_Queue_Queue_const_QueueR(const cv::ocl::Queue* q) {
		try {
			cv::ocl::Queue* ret = new cv::ocl::Queue(*q);
			return Ok<cv::ocl::Queue*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Queue*>))
	}
	
	Result<bool> cv_ocl_Queue_create_const_ContextR_const_DeviceR(cv::ocl::Queue* instance, const cv::ocl::Context* c, const cv::ocl::Device* d) {
		try {
			bool ret = instance->create(*c, *d);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_ocl_Queue_finish(cv::ocl::Queue* instance) {
		try {
			instance->finish();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_ocl_Queue_ptr_const(const cv::ocl::Queue* instance) {
		try {
			void* ret = instance->ptr();
			return Ok<void*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<cv::ocl::Queue*> cv_ocl_Queue_getDefault() {
		try {
			cv::ocl::Queue ret = cv::ocl::Queue::getDefault();
			return Ok(new cv::ocl::Queue(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Queue*>))
	}
	
}
