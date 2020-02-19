#include "core.hpp"
#include "core_types.hpp"

extern "C" {
	Result<bool> cv_Cholesky_doubleX_size_t_int_doubleX_size_t_int(double* A, size_t astep, int m, double* b, size_t bstep, int n) {
		try {
			bool ret = cv::Cholesky(A, astep, m, b, bstep, n);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_Cholesky_floatX_size_t_int_floatX_size_t_int(float* A, size_t astep, int m, float* b, size_t bstep, int n) {
		try {
			bool ret = cv::Cholesky(A, astep, m, b, bstep, n);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_LUT_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* src, void* lut, void* dst) {
		try {
			cv::LUT(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputArray*>(lut), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_LU_doubleX_size_t_int_doubleX_size_t_int(double* A, size_t astep, int m, double* b, size_t bstep, int n) {
		try {
			int ret = cv::LU(A, astep, m, b, bstep, n);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_LU_floatX_size_t_int_floatX_size_t_int(float* A, size_t astep, int m, float* b, size_t bstep, int n) {
		try {
			int ret = cv::LU(A, astep, m, b, bstep, n);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<double> cv_Mahalanobis_const__InputArrayX_const__InputArrayX_const__InputArrayX(void* v1, void* v2, void* icovar) {
		try {
			double ret = cv::Mahalanobis(*reinterpret_cast<const cv::_InputArray*>(v1), *reinterpret_cast<const cv::_InputArray*>(v2), *reinterpret_cast<const cv::_InputArray*>(icovar));
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_PCABackProject_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* data, void* mean, void* eigenvectors, void* result) {
		try {
			cv::PCABackProject(*reinterpret_cast<const cv::_InputArray*>(data), *reinterpret_cast<const cv::_InputArray*>(mean), *reinterpret_cast<const cv::_InputArray*>(eigenvectors), *reinterpret_cast<const cv::_OutputArray*>(result));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_PCACompute_const__InputArrayX_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX_double(void* data, void* mean, void* eigenvectors, void* eigenvalues, double retainedVariance) {
		try {
			cv::PCACompute(*reinterpret_cast<const cv::_InputArray*>(data), *reinterpret_cast<const cv::_InputOutputArray*>(mean), *reinterpret_cast<const cv::_OutputArray*>(eigenvectors), *reinterpret_cast<const cv::_OutputArray*>(eigenvalues), retainedVariance);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_PCACompute_const__InputArrayX_const__InputOutputArrayX_const__OutputArrayX_const__OutputArrayX_int(void* data, void* mean, void* eigenvectors, void* eigenvalues, int maxComponents) {
		try {
			cv::PCACompute(*reinterpret_cast<const cv::_InputArray*>(data), *reinterpret_cast<const cv::_InputOutputArray*>(mean), *reinterpret_cast<const cv::_OutputArray*>(eigenvectors), *reinterpret_cast<const cv::_OutputArray*>(eigenvalues), maxComponents);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_PCACompute_const__InputArrayX_const__InputOutputArrayX_const__OutputArrayX_double(void* data, void* mean, void* eigenvectors, double retainedVariance) {
		try {
			cv::PCACompute(*reinterpret_cast<const cv::_InputArray*>(data), *reinterpret_cast<const cv::_InputOutputArray*>(mean), *reinterpret_cast<const cv::_OutputArray*>(eigenvectors), retainedVariance);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_PCACompute_const__InputArrayX_const__InputOutputArrayX_const__OutputArrayX_int(void* data, void* mean, void* eigenvectors, int maxComponents) {
		try {
			cv::PCACompute(*reinterpret_cast<const cv::_InputArray*>(data), *reinterpret_cast<const cv::_InputOutputArray*>(mean), *reinterpret_cast<const cv::_OutputArray*>(eigenvectors), maxComponents);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_PCAProject_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* data, void* mean, void* eigenvectors, void* result) {
		try {
			cv::PCAProject(*reinterpret_cast<const cv::_InputArray*>(data), *reinterpret_cast<const cv::_InputArray*>(mean), *reinterpret_cast<const cv::_InputArray*>(eigenvectors), *reinterpret_cast<const cv::_OutputArray*>(result));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_PSNR_const__InputArrayX_const__InputArrayX_double(void* src1, void* src2, double R) {
		try {
			double ret = cv::PSNR(*reinterpret_cast<const cv::_InputArray*>(src1), *reinterpret_cast<const cv::_InputArray*>(src2), R);
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_SVBackSubst_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* w, void* u, void* vt, void* rhs, void* dst) {
		try {
			cv::SVBackSubst(*reinterpret_cast<const cv::_InputArray*>(w), *reinterpret_cast<const cv::_InputArray*>(u), *reinterpret_cast<const cv::_InputArray*>(vt), *reinterpret_cast<const cv::_InputArray*>(rhs), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_SVDecomp_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_int(void* src, void* w, void* u, void* vt, int flags) {
		try {
			cv::SVDecomp(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(w), *reinterpret_cast<const cv::_OutputArray*>(u), *reinterpret_cast<const cv::_OutputArray*>(vt), flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_abs_const_MatExprX(void* e) {
		try {
			cv::MatExpr ret = cv::abs(*reinterpret_cast<const cv::MatExpr*>(e));
			return Ok<void*>(new cv::MatExpr(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_abs_const_MatX(void* m) {
		try {
			cv::MatExpr ret = cv::abs(*reinterpret_cast<const cv::Mat*>(m));
			return Ok<void*>(new cv::MatExpr(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_absdiff_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* src1, void* src2, void* dst) {
		try {
			cv::absdiff(*reinterpret_cast<const cv::_InputArray*>(src1), *reinterpret_cast<const cv::_InputArray*>(src2), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_addWeighted_const__InputArrayX_double_const__InputArrayX_double_double_const__OutputArrayX_int(void* src1, double alpha, void* src2, double beta, double gamma, void* dst, int dtype) {
		try {
			cv::addWeighted(*reinterpret_cast<const cv::_InputArray*>(src1), alpha, *reinterpret_cast<const cv::_InputArray*>(src2), beta, gamma, *reinterpret_cast<const cv::_OutputArray*>(dst), dtype);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_add_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__InputArrayX_int(void* src1, void* src2, void* dst, void* mask, int dtype) {
		try {
			cv::add(*reinterpret_cast<const cv::_InputArray*>(src1), *reinterpret_cast<const cv::_InputArray*>(src2), *reinterpret_cast<const cv::_OutputArray*>(dst), *reinterpret_cast<const cv::_InputArray*>(mask), dtype);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_batchDistance_const__InputArrayX_const__InputArrayX_const__OutputArrayX_int_const__OutputArrayX_int_int_const__InputArrayX_int_bool(void* src1, void* src2, void* dist, int dtype, void* nidx, int normType, int K, void* mask, int update, bool crosscheck) {
		try {
			cv::batchDistance(*reinterpret_cast<const cv::_InputArray*>(src1), *reinterpret_cast<const cv::_InputArray*>(src2), *reinterpret_cast<const cv::_OutputArray*>(dist), dtype, *reinterpret_cast<const cv::_OutputArray*>(nidx), normType, K, *reinterpret_cast<const cv::_InputArray*>(mask), update, crosscheck);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bitwise_and_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__InputArrayX(void* src1, void* src2, void* dst, void* mask) {
		try {
			cv::bitwise_and(*reinterpret_cast<const cv::_InputArray*>(src1), *reinterpret_cast<const cv::_InputArray*>(src2), *reinterpret_cast<const cv::_OutputArray*>(dst), *reinterpret_cast<const cv::_InputArray*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bitwise_not_const__InputArrayX_const__OutputArrayX_const__InputArrayX(void* src, void* dst, void* mask) {
		try {
			cv::bitwise_not(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), *reinterpret_cast<const cv::_InputArray*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bitwise_or_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__InputArrayX(void* src1, void* src2, void* dst, void* mask) {
		try {
			cv::bitwise_or(*reinterpret_cast<const cv::_InputArray*>(src1), *reinterpret_cast<const cv::_InputArray*>(src2), *reinterpret_cast<const cv::_OutputArray*>(dst), *reinterpret_cast<const cv::_InputArray*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bitwise_xor_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__InputArrayX(void* src1, void* src2, void* dst, void* mask) {
		try {
			cv::bitwise_xor(*reinterpret_cast<const cv::_InputArray*>(src1), *reinterpret_cast<const cv::_InputArray*>(src2), *reinterpret_cast<const cv::_OutputArray*>(dst), *reinterpret_cast<const cv::_InputArray*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_borderInterpolate_int_int_int(int p, int len, int borderType) {
		try {
			int ret = cv::borderInterpolate(p, len, borderType);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_calcCovarMatrix_const__InputArrayX_const__OutputArrayX_const__InputOutputArrayX_int_int(void* samples, void* covar, void* mean, int flags, int ctype) {
		try {
			cv::calcCovarMatrix(*reinterpret_cast<const cv::_InputArray*>(samples), *reinterpret_cast<const cv::_OutputArray*>(covar), *reinterpret_cast<const cv::_InputOutputArray*>(mean), flags, ctype);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_cartToPolar_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_bool(void* x, void* y, void* magnitude, void* angle, bool angleInDegrees) {
		try {
			cv::cartToPolar(*reinterpret_cast<const cv::_InputArray*>(x), *reinterpret_cast<const cv::_InputArray*>(y), *reinterpret_cast<const cv::_OutputArray*>(magnitude), *reinterpret_cast<const cv::_OutputArray*>(angle), angleInDegrees);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_checkHardwareSupport_int(int feature) {
		try {
			bool ret = cv::checkHardwareSupport(feature);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_checkRange_const__InputArrayX_bool_PointX_double_double(void* a, bool quiet, cv::Point* pos, double minVal, double maxVal) {
		try {
			bool ret = cv::checkRange(*reinterpret_cast<const cv::_InputArray*>(a), quiet, pos, minVal, maxVal);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_compare_const__InputArrayX_const__InputArrayX_const__OutputArrayX_int(void* src1, void* src2, void* dst, int cmpop) {
		try {
			cv::compare(*reinterpret_cast<const cv::_InputArray*>(src1), *reinterpret_cast<const cv::_InputArray*>(src2), *reinterpret_cast<const cv::_OutputArray*>(dst), cmpop);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_completeSymm_const__InputOutputArrayX_bool(void* m, bool lowerToUpper) {
		try {
			cv::completeSymm(*reinterpret_cast<const cv::_InputOutputArray*>(m), lowerToUpper);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_convertFp16_const__InputArrayX_const__OutputArrayX(void* src, void* dst) {
		try {
			cv::convertFp16(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_convertScaleAbs_const__InputArrayX_const__OutputArrayX_double_double(void* src, void* dst, double alpha, double beta) {
		try {
			cv::convertScaleAbs(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), alpha, beta);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_copyMakeBorder_const__InputArrayX_const__OutputArrayX_int_int_int_int_int_const_ScalarX(void* src, void* dst, int top, int bottom, int left, int right, int borderType, const cv::Scalar* value) {
		try {
			cv::copyMakeBorder(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), top, bottom, left, right, borderType, *value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_copyTo_const__InputArrayX_const__OutputArrayX_const__InputArrayX(void* src, void* dst, void* mask) {
		try {
			cv::copyTo(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), *reinterpret_cast<const cv::_InputArray*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_countNonZero_const__InputArrayX(void* src) {
		try {
			int ret = cv::countNonZero(*reinterpret_cast<const cv::_InputArray*>(src));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<float> cv_cubeRoot_float(float val) {
		try {
			float ret = cv::cubeRoot(val);
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_dct_const__InputArrayX_const__OutputArrayX_int(void* src, void* dst, int flags) {
		try {
			cv::dct(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_depthToString_int(int depth) {
		try {
			const char* ret = cv::depthToString(depth);
			return Ok<void*>(ocvrs_create_string(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_detail_check_failed_MatChannels_int_const_CheckContextX(int v, void* ctx) {
		try {
			cv::detail::check_failed_MatChannels(v, *reinterpret_cast<const cv::detail::CheckContext*>(ctx));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_check_failed_MatChannels_int_int_const_CheckContextX(int v1, int v2, void* ctx) {
		try {
			cv::detail::check_failed_MatChannels(v1, v2, *reinterpret_cast<const cv::detail::CheckContext*>(ctx));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_check_failed_MatDepth_int_const_CheckContextX(int v, void* ctx) {
		try {
			cv::detail::check_failed_MatDepth(v, *reinterpret_cast<const cv::detail::CheckContext*>(ctx));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_check_failed_MatDepth_int_int_const_CheckContextX(int v1, int v2, void* ctx) {
		try {
			cv::detail::check_failed_MatDepth(v1, v2, *reinterpret_cast<const cv::detail::CheckContext*>(ctx));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_check_failed_MatType_int_const_CheckContextX(int v, void* ctx) {
		try {
			cv::detail::check_failed_MatType(v, *reinterpret_cast<const cv::detail::CheckContext*>(ctx));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_check_failed_MatType_int_int_const_CheckContextX(int v1, int v2, void* ctx) {
		try {
			cv::detail::check_failed_MatType(v1, v2, *reinterpret_cast<const cv::detail::CheckContext*>(ctx));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_check_failed_auto_Size__int__Size__int__const_CheckContextX(cv::Size_<int> v1, cv::Size_<int> v2, void* ctx) {
		try {
			cv::detail::check_failed_auto(v1, v2, *reinterpret_cast<const cv::detail::CheckContext*>(ctx));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_check_failed_auto_Size__int__const_CheckContextX(cv::Size_<int> v, void* ctx) {
		try {
			cv::detail::check_failed_auto(v, *reinterpret_cast<const cv::detail::CheckContext*>(ctx));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_check_failed_auto_double_const_CheckContextX(double v, void* ctx) {
		try {
			cv::detail::check_failed_auto(v, *reinterpret_cast<const cv::detail::CheckContext*>(ctx));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_check_failed_auto_double_double_const_CheckContextX(double v1, double v2, void* ctx) {
		try {
			cv::detail::check_failed_auto(v1, v2, *reinterpret_cast<const cv::detail::CheckContext*>(ctx));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_check_failed_auto_float_const_CheckContextX(float v, void* ctx) {
		try {
			cv::detail::check_failed_auto(v, *reinterpret_cast<const cv::detail::CheckContext*>(ctx));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_check_failed_auto_float_float_const_CheckContextX(float v1, float v2, void* ctx) {
		try {
			cv::detail::check_failed_auto(v1, v2, *reinterpret_cast<const cv::detail::CheckContext*>(ctx));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_check_failed_auto_int_const_CheckContextX(int v, void* ctx) {
		try {
			cv::detail::check_failed_auto(v, *reinterpret_cast<const cv::detail::CheckContext*>(ctx));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_check_failed_auto_int_int_const_CheckContextX(int v1, int v2, void* ctx) {
		try {
			cv::detail::check_failed_auto(v1, v2, *reinterpret_cast<const cv::detail::CheckContext*>(ctx));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_check_failed_auto_size_t_const_CheckContextX(size_t v, void* ctx) {
		try {
			cv::detail::check_failed_auto(v, *reinterpret_cast<const cv::detail::CheckContext*>(ctx));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_detail_check_failed_auto_size_t_size_t_const_CheckContextX(size_t v1, size_t v2, void* ctx) {
		try {
			cv::detail::check_failed_auto(v1, v2, *reinterpret_cast<const cv::detail::CheckContext*>(ctx));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_determinant_const__InputArrayX(void* mtx) {
		try {
			double ret = cv::determinant(*reinterpret_cast<const cv::_InputArray*>(mtx));
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_dft_const__InputArrayX_const__OutputArrayX_int_int(void* src, void* dst, int flags, int nonzeroRows) {
		try {
			cv::dft(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), flags, nonzeroRows);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_directx_getTypeFromD3DFORMAT_int(int iD3DFORMAT) {
		try {
			int ret = cv::directx::getTypeFromD3DFORMAT(iD3DFORMAT);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_directx_getTypeFromDXGI_FORMAT_int(int iDXGI_FORMAT) {
		try {
			int ret = cv::directx::getTypeFromDXGI_FORMAT(iDXGI_FORMAT);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_divide_const__InputArrayX_const__InputArrayX_const__OutputArrayX_double_int(void* src1, void* src2, void* dst, double scale, int dtype) {
		try {
			cv::divide(*reinterpret_cast<const cv::_InputArray*>(src1), *reinterpret_cast<const cv::_InputArray*>(src2), *reinterpret_cast<const cv::_OutputArray*>(dst), scale, dtype);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_divide_double_const__InputArrayX_const__OutputArrayX_int(double scale, void* src2, void* dst, int dtype) {
		try {
			cv::divide(scale, *reinterpret_cast<const cv::_InputArray*>(src2), *reinterpret_cast<const cv::_OutputArray*>(dst), dtype);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_eigenNonSymmetric_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(void* src, void* eigenvalues, void* eigenvectors) {
		try {
			cv::eigenNonSymmetric(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(eigenvalues), *reinterpret_cast<const cv::_OutputArray*>(eigenvectors));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_eigen_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(void* src, void* eigenvalues, void* eigenvectors) {
		try {
			bool ret = cv::eigen(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(eigenvalues), *reinterpret_cast<const cv::_OutputArray*>(eigenvectors));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_error_const_ExceptionX(void* exc) {
		try {
			cv::error(*reinterpret_cast<const cv::Exception*>(exc));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_error_int_const_StringX_const_charX_const_charX_int(int _code, const char* _err, const char* _func, const char* _file, int _line) {
		try {
			cv::error(_code, std::string(_err), _func, _file, _line);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_exp_const__InputArrayX_const__OutputArrayX(void* src, void* dst) {
		try {
			cv::exp(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_extractChannel_const__InputArrayX_const__OutputArrayX_int(void* src, void* dst, int coi) {
		try {
			cv::extractChannel(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), coi);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_fastAtan2_float_float(float y, float x) {
		try {
			float ret = cv::fastAtan2(y, x);
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_findNonZero_const__InputArrayX_const__OutputArrayX(void* src, void* idx) {
		try {
			cv::findNonZero(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(idx));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_flip_const__InputArrayX_const__OutputArrayX_int(void* src, void* dst, int flipCode) {
		try {
			cv::flip(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), flipCode);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_gemm_const__InputArrayX_const__InputArrayX_double_const__InputArrayX_double_const__OutputArrayX_int(void* src1, void* src2, double alpha, void* src3, double beta, void* dst, int flags) {
		try {
			cv::gemm(*reinterpret_cast<const cv::_InputArray*>(src1), *reinterpret_cast<const cv::_InputArray*>(src2), alpha, *reinterpret_cast<const cv::_InputArray*>(src3), beta, *reinterpret_cast<const cv::_OutputArray*>(dst), flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_getBuildInformation() {
		try {
			cv::String ret = cv::getBuildInformation();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_getCPUFeaturesLine() {
		try {
			std::string ret = cv::getCPUFeaturesLine();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int64_t> cv_getCPUTickCount() {
		try {
			int64_t ret = cv::getCPUTickCount();
			return Ok<int64_t>(ret);
		} OCVRS_CATCH(Result<int64_t>)
	}
	
	Result<size_t> cv_getElemSize_int(int type) {
		try {
			size_t ret = cv::getElemSize(type);
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<void*> cv_getHardwareFeatureName_int(int feature) {
		try {
			cv::String ret = cv::getHardwareFeatureName(feature);
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_getImpl_vector_int_X_vector_String_X(void* impl, void* funName) {
		try {
			int ret = cv::getImpl(*reinterpret_cast<std::vector<int>*>(impl), *reinterpret_cast<std::vector<cv::String>*>(funName));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_getNumThreads() {
		try {
			int ret = cv::getNumThreads();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_getNumberOfCPUs() {
		try {
			int ret = cv::getNumberOfCPUs();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_getOptimalDFTSize_int(int vecsize) {
		try {
			int ret = cv::getOptimalDFTSize(vecsize);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_getThreadNum() {
		try {
			int ret = cv::getThreadNum();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int64_t> cv_getTickCount() {
		try {
			int64_t ret = cv::getTickCount();
			return Ok<int64_t>(ret);
		} OCVRS_CATCH(Result<int64_t>)
	}
	
	Result<double> cv_getTickFrequency() {
		try {
			double ret = cv::getTickFrequency();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<int> cv_getVersionMajor() {
		try {
			int ret = cv::getVersionMajor();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_getVersionMinor() {
		try {
			int ret = cv::getVersionMinor();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_getVersionRevision() {
		try {
			int ret = cv::getVersionRevision();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv_getVersionString() {
		try {
			cv::String ret = cv::getVersionString();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_glob_String_vector_String_X_bool(char* pattern, void* result, bool recursive) {
		try {
			cv::glob(std::string(pattern), *reinterpret_cast<std::vector<cv::String>*>(result), recursive);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_haveOpenVX() {
		try {
			bool ret = cv::haveOpenVX();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_hconcat_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* src1, void* src2, void* dst) {
		try {
			cv::hconcat(*reinterpret_cast<const cv::_InputArray*>(src1), *reinterpret_cast<const cv::_InputArray*>(src2), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hconcat_const__InputArrayX_const__OutputArrayX(void* src, void* dst) {
		try {
			cv::hconcat(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_idct_const__InputArrayX_const__OutputArrayX_int(void* src, void* dst, int flags) {
		try {
			cv::idct(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_idft_const__InputArrayX_const__OutputArrayX_int_int(void* src, void* dst, int flags, int nonzeroRows) {
		try {
			cv::idft(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), flags, nonzeroRows);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_inRange_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* src, void* lowerb, void* upperb, void* dst) {
		try {
			cv::inRange(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputArray*>(lowerb), *reinterpret_cast<const cv::_InputArray*>(upperb), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_insertChannel_const__InputArrayX_const__InputOutputArrayX_int(void* src, void* dst, int coi) {
		try {
			cv::insertChannel(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputOutputArray*>(dst), coi);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::instr::FLAGS> cv_instr_getFlags() {
		try {
			cv::instr::FLAGS ret = cv::instr::getFlags();
			return Ok<cv::instr::FLAGS>(ret);
		} OCVRS_CATCH(Result<cv::instr::FLAGS>)
	}
	
	Result_void cv_instr_resetTrace() {
		try {
			cv::instr::resetTrace();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_instr_setFlags_FLAGS(cv::instr::FLAGS modeFlags) {
		try {
			cv::instr::setFlags(modeFlags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_instr_setUseInstrumentation_bool(bool flag) {
		try {
			cv::instr::setUseInstrumentation(flag);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_instr_useInstrumentation() {
		try {
			bool ret = cv::instr::useInstrumentation();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<double> cv_invert_const__InputArrayX_const__OutputArrayX_int(void* src, void* dst, int flags) {
		try {
			double ret = cv::invert(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), flags);
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<void*> cv_ipp_getIppErrorLocation() {
		try {
			cv::String ret = cv::ipp::getIppErrorLocation();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<unsigned long long> cv_ipp_getIppFeatures() {
		try {
			unsigned long long ret = cv::ipp::getIppFeatures();
			return Ok<unsigned long long>(ret);
		} OCVRS_CATCH(Result<unsigned long long>)
	}
	
	Result<int> cv_ipp_getIppStatus() {
		try {
			int ret = cv::ipp::getIppStatus();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv_ipp_getIppVersion() {
		try {
			cv::String ret = cv::ipp::getIppVersion();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_ipp_setIppStatus_int_const_charX_const_charX_int(int status, const char* funcname, const char* filename, int line) {
		try {
			cv::ipp::setIppStatus(status, funcname, filename, line);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ipp_setUseIPP_NotExact_bool(bool flag) {
		try {
			cv::ipp::setUseIPP_NotExact(flag);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ipp_setUseIPP_bool(bool flag) {
		try {
			cv::ipp::setUseIPP(flag);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_ipp_useIPP() {
		try {
			bool ret = cv::ipp::useIPP();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ipp_useIPP_NotExact() {
		try {
			bool ret = cv::ipp::useIPP_NotExact();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<double> cv_kmeans_const__InputArrayX_int_const__InputOutputArrayX_TermCriteria_int_int_const__OutputArrayX(void* data, int K, void* bestLabels, void* criteria, int attempts, int flags, void* centers) {
		try {
			double ret = cv::kmeans(*reinterpret_cast<const cv::_InputArray*>(data), K, *reinterpret_cast<const cv::_InputOutputArray*>(bestLabels), *reinterpret_cast<cv::TermCriteria*>(criteria), attempts, flags, *reinterpret_cast<const cv::_OutputArray*>(centers));
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_log_const__InputArrayX_const__OutputArrayX(void* src, void* dst) {
		try {
			cv::log(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_magnitude_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* x, void* y, void* magnitude) {
		try {
			cv::magnitude(*reinterpret_cast<const cv::_InputArray*>(x), *reinterpret_cast<const cv::_InputArray*>(y), *reinterpret_cast<const cv::_OutputArray*>(magnitude));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_max_const_MatX_const_MatX(void* a, void* b) {
		try {
			cv::MatExpr ret = cv::max(*reinterpret_cast<const cv::Mat*>(a), *reinterpret_cast<const cv::Mat*>(b));
			return Ok<void*>(new cv::MatExpr(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_max_const_MatX_const_MatX_MatX(void* src1, void* src2, void* dst) {
		try {
			cv::max(*reinterpret_cast<const cv::Mat*>(src1), *reinterpret_cast<const cv::Mat*>(src2), *reinterpret_cast<cv::Mat*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_max_const_MatX_double(void* a, double s) {
		try {
			cv::MatExpr ret = cv::max(*reinterpret_cast<const cv::Mat*>(a), s);
			return Ok<void*>(new cv::MatExpr(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_max_const_UMatX_const_UMatX_UMatX(void* src1, void* src2, void* dst) {
		try {
			cv::max(*reinterpret_cast<const cv::UMat*>(src1), *reinterpret_cast<const cv::UMat*>(src2), *reinterpret_cast<cv::UMat*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_max_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* src1, void* src2, void* dst) {
		try {
			cv::max(*reinterpret_cast<const cv::_InputArray*>(src1), *reinterpret_cast<const cv::_InputArray*>(src2), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_max_double_const_MatX(double s, void* a) {
		try {
			cv::MatExpr ret = cv::max(s, *reinterpret_cast<const cv::Mat*>(a));
			return Ok<void*>(new cv::MatExpr(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_meanStdDev_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__InputArrayX(void* src, void* mean, void* stddev, void* mask) {
		try {
			cv::meanStdDev(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(mean), *reinterpret_cast<const cv::_OutputArray*>(stddev), *reinterpret_cast<const cv::_InputArray*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Scalar> cv_mean_const__InputArrayX_const__InputArrayX(void* src, void* mask) {
		try {
			cv::Scalar ret = cv::mean(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputArray*>(mask));
			return Ok<cv::Scalar>(ret);
		} OCVRS_CATCH(Result<cv::Scalar>)
	}
	
	Result_void cv_merge_const_MatX_size_t_const__OutputArrayX(void* mv, size_t count, void* dst) {
		try {
			cv::merge(reinterpret_cast<const cv::Mat*>(mv), count, *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_merge_const__InputArrayX_const__OutputArrayX(void* mv, void* dst) {
		try {
			cv::merge(*reinterpret_cast<const cv::_InputArray*>(mv), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_minMaxIdx_const__InputArrayX_doubleX_doubleX_intX_intX_const__InputArrayX(void* src, double* minVal, double* maxVal, int* minIdx, int* maxIdx, void* mask) {
		try {
			cv::minMaxIdx(*reinterpret_cast<const cv::_InputArray*>(src), minVal, maxVal, minIdx, maxIdx, *reinterpret_cast<const cv::_InputArray*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_minMaxLoc_const_SparseMatX_doubleX_doubleX_intX_intX(void* a, double* minVal, double* maxVal, int* minIdx, int* maxIdx) {
		try {
			cv::minMaxLoc(*reinterpret_cast<const cv::SparseMat*>(a), minVal, maxVal, minIdx, maxIdx);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_minMaxLoc_const__InputArrayX_doubleX_doubleX_PointX_PointX_const__InputArrayX(void* src, double* minVal, double* maxVal, cv::Point* minLoc, cv::Point* maxLoc, void* mask) {
		try {
			cv::minMaxLoc(*reinterpret_cast<const cv::_InputArray*>(src), minVal, maxVal, minLoc, maxLoc, *reinterpret_cast<const cv::_InputArray*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_min_const_MatX_const_MatX(void* a, void* b) {
		try {
			cv::MatExpr ret = cv::min(*reinterpret_cast<const cv::Mat*>(a), *reinterpret_cast<const cv::Mat*>(b));
			return Ok<void*>(new cv::MatExpr(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_min_const_MatX_const_MatX_MatX(void* src1, void* src2, void* dst) {
		try {
			cv::min(*reinterpret_cast<const cv::Mat*>(src1), *reinterpret_cast<const cv::Mat*>(src2), *reinterpret_cast<cv::Mat*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_min_const_MatX_double(void* a, double s) {
		try {
			cv::MatExpr ret = cv::min(*reinterpret_cast<const cv::Mat*>(a), s);
			return Ok<void*>(new cv::MatExpr(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_min_const_UMatX_const_UMatX_UMatX(void* src1, void* src2, void* dst) {
		try {
			cv::min(*reinterpret_cast<const cv::UMat*>(src1), *reinterpret_cast<const cv::UMat*>(src2), *reinterpret_cast<cv::UMat*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_min_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* src1, void* src2, void* dst) {
		try {
			cv::min(*reinterpret_cast<const cv::_InputArray*>(src1), *reinterpret_cast<const cv::_InputArray*>(src2), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_min_double_const_MatX(double s, void* a) {
		try {
			cv::MatExpr ret = cv::min(s, *reinterpret_cast<const cv::Mat*>(a));
			return Ok<void*>(new cv::MatExpr(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_mixChannels_const__InputArrayX_const__InputOutputArrayX_const_intX_size_t(void* src, void* dst, const int* fromTo, size_t npairs) {
		try {
			cv::mixChannels(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputOutputArray*>(dst), fromTo, npairs);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_mixChannels_const__InputArrayX_const__InputOutputArrayX_const_vector_int_X(void* src, void* dst, void* fromTo) {
		try {
			cv::mixChannels(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputOutputArray*>(dst), *reinterpret_cast<const std::vector<int>*>(fromTo));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_mulSpectrums_const__InputArrayX_const__InputArrayX_const__OutputArrayX_int_bool(void* a, void* b, void* c, int flags, bool conjB) {
		try {
			cv::mulSpectrums(*reinterpret_cast<const cv::_InputArray*>(a), *reinterpret_cast<const cv::_InputArray*>(b), *reinterpret_cast<const cv::_OutputArray*>(c), flags, conjB);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_mulTransposed_const__InputArrayX_const__OutputArrayX_bool_const__InputArrayX_double_int(void* src, void* dst, bool aTa, void* delta, double scale, int dtype) {
		try {
			cv::mulTransposed(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), aTa, *reinterpret_cast<const cv::_InputArray*>(delta), scale, dtype);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_multiply_const__InputArrayX_const__InputArrayX_const__OutputArrayX_double_int(void* src1, void* src2, void* dst, double scale, int dtype) {
		try {
			cv::multiply(*reinterpret_cast<const cv::_InputArray*>(src1), *reinterpret_cast<const cv::_InputArray*>(src2), *reinterpret_cast<const cv::_OutputArray*>(dst), scale, dtype);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_noArray() {
		try {
			cv::_InputOutputArray ret = cv::noArray();
			return Ok<void*>(new cv::_InputOutputArray(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<double> cv_norm_const_SparseMatX_int(void* src, int normType) {
		try {
			double ret = cv::norm(*reinterpret_cast<const cv::SparseMat*>(src), normType);
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<double> cv_norm_const__InputArrayX_const__InputArrayX_int_const__InputArrayX(void* src1, void* src2, int normType, void* mask) {
		try {
			double ret = cv::norm(*reinterpret_cast<const cv::_InputArray*>(src1), *reinterpret_cast<const cv::_InputArray*>(src2), normType, *reinterpret_cast<const cv::_InputArray*>(mask));
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<double> cv_norm_const__InputArrayX_int_const__InputArrayX(void* src1, int normType, void* mask) {
		try {
			double ret = cv::norm(*reinterpret_cast<const cv::_InputArray*>(src1), normType, *reinterpret_cast<const cv::_InputArray*>(mask));
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_normalize_const_SparseMatX_SparseMatX_double_int(void* src, void* dst, double alpha, int normType) {
		try {
			cv::normalize(*reinterpret_cast<const cv::SparseMat*>(src), *reinterpret_cast<cv::SparseMat*>(dst), alpha, normType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_normalize_const__InputArrayX_const__InputOutputArrayX_double_double_int_int_const__InputArrayX(void* src, void* dst, double alpha, double beta, int norm_type, int dtype, void* mask) {
		try {
			cv::normalize(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputOutputArray*>(dst), alpha, beta, norm_type, dtype, *reinterpret_cast<const cv::_InputArray*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ocl_attachContext_const_StringX_voidX_voidX_voidX(const char* platformName, void* platformID, void* context, void* deviceID) {
		try {
			cv::ocl::attachContext(std::string(platformName), platformID, context, deviceID);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ocl_buildOptionsAddMatrixDescription_StringX_const_StringX_const__InputArrayX(void** buildOptions, const char* name, void* _m) {
		try {
			std::string buildOptions_out;
			cv::ocl::buildOptionsAddMatrixDescription(buildOptions_out, std::string(name), *reinterpret_cast<const cv::_InputArray*>(_m));
			*buildOptions = ocvrs_create_string(buildOptions_out.c_str());
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ocl_checkOptimalVectorWidth_const_intX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_OclVectorStrategy(const int* vectorWidths, void* src1, void* src2, void* src3, void* src4, void* src5, void* src6, void* src7, void* src8, void* src9, cv::ocl::OclVectorStrategy strat) {
		try {
			int ret = cv::ocl::checkOptimalVectorWidth(vectorWidths, *reinterpret_cast<const cv::_InputArray*>(src1), *reinterpret_cast<const cv::_InputArray*>(src2), *reinterpret_cast<const cv::_InputArray*>(src3), *reinterpret_cast<const cv::_InputArray*>(src4), *reinterpret_cast<const cv::_InputArray*>(src5), *reinterpret_cast<const cv::_InputArray*>(src6), *reinterpret_cast<const cv::_InputArray*>(src7), *reinterpret_cast<const cv::_InputArray*>(src8), *reinterpret_cast<const cv::_InputArray*>(src9), strat);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ocl_convertFromBuffer_voidX_size_t_int_int_int_UMatX(void* cl_mem_buffer, size_t step, int rows, int cols, int type, void* dst) {
		try {
			cv::ocl::convertFromBuffer(cl_mem_buffer, step, rows, cols, type, *reinterpret_cast<cv::UMat*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ocl_convertFromImage_voidX_UMatX(void* cl_mem_image, void* dst) {
		try {
			cv::ocl::convertFromImage(cl_mem_image, *reinterpret_cast<cv::UMat*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_ocl_convertTypeStr_int_int_int_charX(int sdepth, int ddepth, int cn, char* buf) {
		try {
			const char* ret = cv::ocl::convertTypeStr(sdepth, ddepth, cn, buf);
			return Ok<void*>(ocvrs_create_string(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_ocl_finish() {
		try {
			cv::ocl::finish();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_ocl_getOpenCLErrorString_int(int errorCode) {
		try {
			const char* ret = cv::ocl::getOpenCLErrorString(errorCode);
			return Ok<void*>(ocvrs_create_string(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_ocl_getPlatfomsInfo_vector_PlatformInfo_X(void* platform_info) {
		try {
			cv::ocl::getPlatfomsInfo(*reinterpret_cast<std::vector<cv::ocl::PlatformInfo>*>(platform_info));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_ocl_haveAmdBlas() {
		try {
			bool ret = cv::ocl::haveAmdBlas();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ocl_haveAmdFft() {
		try {
			bool ret = cv::ocl::haveAmdFft();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ocl_haveOpenCL() {
		try {
			bool ret = cv::ocl::haveOpenCL();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ocl_haveSVM() {
		try {
			bool ret = cv::ocl::haveSVM();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_ocl_kernelToStr_const__InputArrayX_int_const_charX(void* _kernel, int ddepth, const char* name) {
		try {
			cv::String ret = cv::ocl::kernelToStr(*reinterpret_cast<const cv::_InputArray*>(_kernel), ddepth, name);
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_memopTypeToStr_int(int t) {
		try {
			const char* ret = cv::ocl::memopTypeToStr(t);
			return Ok<void*>(ocvrs_create_string(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_ocl_predictOptimalVectorWidthMax_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX(void* src1, void* src2, void* src3, void* src4, void* src5, void* src6, void* src7, void* src8, void* src9) {
		try {
			int ret = cv::ocl::predictOptimalVectorWidthMax(*reinterpret_cast<const cv::_InputArray*>(src1), *reinterpret_cast<const cv::_InputArray*>(src2), *reinterpret_cast<const cv::_InputArray*>(src3), *reinterpret_cast<const cv::_InputArray*>(src4), *reinterpret_cast<const cv::_InputArray*>(src5), *reinterpret_cast<const cv::_InputArray*>(src6), *reinterpret_cast<const cv::_InputArray*>(src7), *reinterpret_cast<const cv::_InputArray*>(src8), *reinterpret_cast<const cv::_InputArray*>(src9));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ocl_predictOptimalVectorWidth_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_OclVectorStrategy(void* src1, void* src2, void* src3, void* src4, void* src5, void* src6, void* src7, void* src8, void* src9, cv::ocl::OclVectorStrategy strat) {
		try {
			int ret = cv::ocl::predictOptimalVectorWidth(*reinterpret_cast<const cv::_InputArray*>(src1), *reinterpret_cast<const cv::_InputArray*>(src2), *reinterpret_cast<const cv::_InputArray*>(src3), *reinterpret_cast<const cv::_InputArray*>(src4), *reinterpret_cast<const cv::_InputArray*>(src5), *reinterpret_cast<const cv::_InputArray*>(src6), *reinterpret_cast<const cv::_InputArray*>(src7), *reinterpret_cast<const cv::_InputArray*>(src8), *reinterpret_cast<const cv::_InputArray*>(src9), strat);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ocl_setUseOpenCL_bool(bool flag) {
		try {
			cv::ocl::setUseOpenCL(flag);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_ocl_typeToStr_int(int t) {
		try {
			const char* ret = cv::ocl::typeToStr(t);
			return Ok<void*>(ocvrs_create_string(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_ocl_useOpenCL() {
		try {
			bool ret = cv::ocl::useOpenCL();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_ocl_vecopTypeToStr_int(int t) {
		try {
			const char* ret = cv::ocl::vecopTypeToStr(t);
			return Ok<void*>(ocvrs_create_string(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_parallel_for__const_RangeX_const_ParallelLoopBodyX_double(void* range, void* body, double nstripes) {
		try {
			cv::parallel_for_(*reinterpret_cast<const cv::Range*>(range), *reinterpret_cast<const cv::ParallelLoopBody*>(body), nstripes);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_patchNaNs_const__InputOutputArrayX_double(void* a, double val) {
		try {
			cv::patchNaNs(*reinterpret_cast<const cv::_InputOutputArray*>(a), val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_perspectiveTransform_const__InputArrayX_const__OutputArrayX_const__InputArrayX(void* src, void* dst, void* m) {
		try {
			cv::perspectiveTransform(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), *reinterpret_cast<const cv::_InputArray*>(m));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_phase_const__InputArrayX_const__InputArrayX_const__OutputArrayX_bool(void* x, void* y, void* angle, bool angleInDegrees) {
		try {
			cv::phase(*reinterpret_cast<const cv::_InputArray*>(x), *reinterpret_cast<const cv::_InputArray*>(y), *reinterpret_cast<const cv::_OutputArray*>(angle), angleInDegrees);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_polarToCart_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_bool(void* magnitude, void* angle, void* x, void* y, bool angleInDegrees) {
		try {
			cv::polarToCart(*reinterpret_cast<const cv::_InputArray*>(magnitude), *reinterpret_cast<const cv::_InputArray*>(angle), *reinterpret_cast<const cv::_OutputArray*>(x), *reinterpret_cast<const cv::_OutputArray*>(y), angleInDegrees);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_pow_const__InputArrayX_double_const__OutputArrayX(void* src, double power, void* dst) {
		try {
			cv::pow(*reinterpret_cast<const cv::_InputArray*>(src), power, *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_randShuffle_const__InputOutputArrayX_double_RNGX(void* dst, double iterFactor, void* rng) {
		try {
			cv::randShuffle(*reinterpret_cast<const cv::_InputOutputArray*>(dst), iterFactor, reinterpret_cast<cv::RNG*>(rng));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_randn_const__InputOutputArrayX_const__InputArrayX_const__InputArrayX(void* dst, void* mean, void* stddev) {
		try {
			cv::randn(*reinterpret_cast<const cv::_InputOutputArray*>(dst), *reinterpret_cast<const cv::_InputArray*>(mean), *reinterpret_cast<const cv::_InputArray*>(stddev));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_randu_const__InputOutputArrayX_const__InputArrayX_const__InputArrayX(void* dst, void* low, void* high) {
		try {
			cv::randu(*reinterpret_cast<const cv::_InputOutputArray*>(dst), *reinterpret_cast<const cv::_InputArray*>(low), *reinterpret_cast<const cv::_InputArray*>(high));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_read_const_FileNodeX_DMatchX_const_DMatchX(void* node, cv::DMatch* value, const cv::DMatch* default_value) {
		try {
			cv::read(*reinterpret_cast<const cv::FileNode*>(node), *value, *default_value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_read_const_FileNodeX_KeyPointX_const_KeyPointX(void* node, cv::KeyPoint* value, const cv::KeyPoint* default_value) {
		try {
			cv::read(*reinterpret_cast<const cv::FileNode*>(node), *value, *default_value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_read_const_FileNodeX_MatX_const_MatX(void* node, void* mat, void* default_mat) {
		try {
			cv::read(*reinterpret_cast<const cv::FileNode*>(node), *reinterpret_cast<cv::Mat*>(mat), *reinterpret_cast<const cv::Mat*>(default_mat));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_read_const_FileNodeX_SparseMatX_const_SparseMatX(void* node, void* mat, void* default_mat) {
		try {
			cv::read(*reinterpret_cast<const cv::FileNode*>(node), *reinterpret_cast<cv::SparseMat*>(mat), *reinterpret_cast<const cv::SparseMat*>(default_mat));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_read_const_FileNodeX_doubleX_double(void* node, double* value, double default_value) {
		try {
			cv::read(*reinterpret_cast<const cv::FileNode*>(node), *value, default_value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_read_const_FileNodeX_floatX_float(void* node, float* value, float default_value) {
		try {
			cv::read(*reinterpret_cast<const cv::FileNode*>(node), *value, default_value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_read_const_FileNodeX_intX_int(void* node, int* value, int default_value) {
		try {
			cv::read(*reinterpret_cast<const cv::FileNode*>(node), *value, default_value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_read_const_FileNodeX_stringX_const_stringX(void* node, void** value, const char* default_value) {
		try {
			std::string value_out;
			cv::read(*reinterpret_cast<const cv::FileNode*>(node), value_out, std::string(default_value));
			*value = ocvrs_create_string(value_out.c_str());
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_read_const_FileNodeX_vector_DMatch_X(void* node, void* matches) {
		try {
			cv::read(*reinterpret_cast<const cv::FileNode*>(node), *reinterpret_cast<std::vector<cv::DMatch>*>(matches));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_read_const_FileNodeX_vector_KeyPoint_X(void* node, void* keypoints) {
		try {
			cv::read(*reinterpret_cast<const cv::FileNode*>(node), *reinterpret_cast<std::vector<cv::KeyPoint>*>(keypoints));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_reduce_const__InputArrayX_const__OutputArrayX_int_int_int(void* src, void* dst, int dim, int rtype, int dtype) {
		try {
			cv::reduce(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), dim, rtype, dtype);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_repeat_const_MatX_int_int(void* src, int ny, int nx) {
		try {
			cv::Mat ret = cv::repeat(*reinterpret_cast<const cv::Mat*>(src), ny, nx);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_repeat_const__InputArrayX_int_int_const__OutputArrayX(void* src, int ny, int nx, void* dst) {
		try {
			cv::repeat(*reinterpret_cast<const cv::_InputArray*>(src), ny, nx, *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_rotate_const__InputArrayX_const__OutputArrayX_int(void* src, void* dst, int rotateCode) {
		try {
			cv::rotate(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), rotateCode);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_samples_addSamplesDataSearchPath_const_StringX(const char* path) {
		try {
			cv::samples::addSamplesDataSearchPath(std::string(path));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_samples_addSamplesDataSearchSubDirectory_const_StringX(const char* subdir) {
		try {
			cv::samples::addSamplesDataSearchSubDirectory(std::string(subdir));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_samples_findFileOrKeep_const_StringX_bool(const char* relative_path, bool silentMode) {
		try {
			cv::String ret = cv::samples::findFileOrKeep(std::string(relative_path), silentMode);
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_samples_findFile_const_StringX_bool_bool(const char* relative_path, bool required, bool silentMode) {
		try {
			cv::String ret = cv::samples::findFile(std::string(relative_path), required, silentMode);
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_scaleAdd_const__InputArrayX_double_const__InputArrayX_const__OutputArrayX(void* src1, double alpha, void* src2, void* dst) {
		try {
			cv::scaleAdd(*reinterpret_cast<const cv::_InputArray*>(src1), alpha, *reinterpret_cast<const cv::_InputArray*>(src2), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_setBreakOnError_bool(bool flag) {
		try {
			bool ret = cv::setBreakOnError(flag);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_setIdentity_const__InputOutputArrayX_const_ScalarX(void* mtx, const cv::Scalar* s) {
		try {
			cv::setIdentity(*reinterpret_cast<const cv::_InputOutputArray*>(mtx), *s);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_setNumThreads_int(int nthreads) {
		try {
			cv::setNumThreads(nthreads);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_setRNGSeed_int(int seed) {
		try {
			cv::setRNGSeed(seed);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_setUseOpenVX_bool(bool flag) {
		try {
			cv::setUseOpenVX(flag);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_setUseOptimized_bool(bool onoff) {
		try {
			cv::setUseOptimized(onoff);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_solveCubic_const__InputArrayX_const__OutputArrayX(void* coeffs, void* roots) {
		try {
			int ret = cv::solveCubic(*reinterpret_cast<const cv::_InputArray*>(coeffs), *reinterpret_cast<const cv::_OutputArray*>(roots));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_solveLP_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* Func, void* Constr, void* z) {
		try {
			int ret = cv::solveLP(*reinterpret_cast<const cv::_InputArray*>(Func), *reinterpret_cast<const cv::_InputArray*>(Constr), *reinterpret_cast<const cv::_OutputArray*>(z));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<double> cv_solvePoly_const__InputArrayX_const__OutputArrayX_int(void* coeffs, void* roots, int maxIters) {
		try {
			double ret = cv::solvePoly(*reinterpret_cast<const cv::_InputArray*>(coeffs), *reinterpret_cast<const cv::_OutputArray*>(roots), maxIters);
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<bool> cv_solve_const__InputArrayX_const__InputArrayX_const__OutputArrayX_int(void* src1, void* src2, void* dst, int flags) {
		try {
			bool ret = cv::solve(*reinterpret_cast<const cv::_InputArray*>(src1), *reinterpret_cast<const cv::_InputArray*>(src2), *reinterpret_cast<const cv::_OutputArray*>(dst), flags);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_sortIdx_const__InputArrayX_const__OutputArrayX_int(void* src, void* dst, int flags) {
		try {
			cv::sortIdx(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sort_const__InputArrayX_const__OutputArrayX_int(void* src, void* dst, int flags) {
		try {
			cv::sort(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_split_const_MatX_MatX(void* src, void* mvbegin) {
		try {
			cv::split(*reinterpret_cast<const cv::Mat*>(src), reinterpret_cast<cv::Mat*>(mvbegin));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_split_const__InputArrayX_const__OutputArrayX(void* m, void* mv) {
		try {
			cv::split(*reinterpret_cast<const cv::_InputArray*>(m), *reinterpret_cast<const cv::_OutputArray*>(mv));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sqrt_const__InputArrayX_const__OutputArrayX(void* src, void* dst) {
		try {
			cv::sqrt(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_subtract_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__InputArrayX_int(void* src1, void* src2, void* dst, void* mask, int dtype) {
		try {
			cv::subtract(*reinterpret_cast<const cv::_InputArray*>(src1), *reinterpret_cast<const cv::_InputArray*>(src2), *reinterpret_cast<const cv::_OutputArray*>(dst), *reinterpret_cast<const cv::_InputArray*>(mask), dtype);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Scalar> cv_sum_const__InputArrayX(void* src) {
		try {
			cv::Scalar ret = cv::sum(*reinterpret_cast<const cv::_InputArray*>(src));
			return Ok<cv::Scalar>(ret);
		} OCVRS_CATCH(Result<cv::Scalar>)
	}
	
	Result_void cv_swap_MatX_MatX(void* a, void* b) {
		try {
			cv::swap(*reinterpret_cast<cv::Mat*>(a), *reinterpret_cast<cv::Mat*>(b));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_swap_UMatX_UMatX(void* a, void* b) {
		try {
			cv::swap(*reinterpret_cast<cv::UMat*>(a), *reinterpret_cast<cv::UMat*>(b));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_tempfile_const_charX(const char* suffix) {
		try {
			cv::String ret = cv::tempfile(suffix);
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_theRNG() {
		try {
			cv::RNG ret = cv::theRNG();
			return Ok<void*>(new cv::RNG(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::Scalar> cv_trace_const__InputArrayX(void* mtx) {
		try {
			cv::Scalar ret = cv::trace(*reinterpret_cast<const cv::_InputArray*>(mtx));
			return Ok<cv::Scalar>(ret);
		} OCVRS_CATCH(Result<cv::Scalar>)
	}
	
	Result_void cv_transform_const__InputArrayX_const__OutputArrayX_const__InputArrayX(void* src, void* dst, void* m) {
		try {
			cv::transform(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), *reinterpret_cast<const cv::_InputArray*>(m));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_transpose_const__InputArrayX_const__OutputArrayX(void* src, void* dst) {
		try {
			cv::transpose(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_typeToString_int(int type) {
		try {
			cv::String ret = cv::typeToString(type);
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_useOpenVX() {
		try {
			bool ret = cv::useOpenVX();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_useOptimized() {
		try {
			bool ret = cv::useOptimized();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_utils_dumpBool_bool(bool argument) {
		try {
			cv::String ret = cv::utils::dumpBool(argument);
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_utils_dumpCString_const_charX(const char* argument) {
		try {
			cv::String ret = cv::utils::dumpCString(argument);
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_utils_dumpDouble_double(double argument) {
		try {
			cv::String ret = cv::utils::dumpDouble(argument);
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_utils_dumpFloat_float(float argument) {
		try {
			cv::String ret = cv::utils::dumpFloat(argument);
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_utils_dumpInputArrayOfArrays_const__InputArrayX(void* argument) {
		try {
			cv::String ret = cv::utils::dumpInputArrayOfArrays(*reinterpret_cast<const cv::_InputArray*>(argument));
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_utils_dumpInputArray_const__InputArrayX(void* argument) {
		try {
			cv::String ret = cv::utils::dumpInputArray(*reinterpret_cast<const cv::_InputArray*>(argument));
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_utils_dumpInputOutputArrayOfArrays_const__InputOutputArrayX(void* argument) {
		try {
			cv::String ret = cv::utils::dumpInputOutputArrayOfArrays(*reinterpret_cast<const cv::_InputOutputArray*>(argument));
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_utils_dumpInputOutputArray_const__InputOutputArrayX(void* argument) {
		try {
			cv::String ret = cv::utils::dumpInputOutputArray(*reinterpret_cast<const cv::_InputOutputArray*>(argument));
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_utils_dumpInt_int(int argument) {
		try {
			cv::String ret = cv::utils::dumpInt(argument);
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_utils_dumpSizeT_size_t(size_t argument) {
		try {
			cv::String ret = cv::utils::dumpSizeT(argument);
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_utils_getThreadID() {
		try {
			int ret = cv::utils::getThreadID();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<cv::utils::logging::LogLevel> cv_utils_logging_getLogLevel() {
		try {
			cv::utils::logging::LogLevel ret = cv::utils::logging::getLogLevel();
			return Ok<cv::utils::logging::LogLevel>(ret);
		} OCVRS_CATCH(Result<cv::utils::logging::LogLevel>)
	}
	
	Result<cv::utils::logging::LogLevel> cv_utils_logging_getLogTagLevel_const_charX(const char* tag) {
		try {
			cv::utils::logging::LogLevel ret = cv::utils::logging::getLogTagLevel(tag);
			return Ok<cv::utils::logging::LogLevel>(ret);
		} OCVRS_CATCH(Result<cv::utils::logging::LogLevel>)
	}
	
	Result<void*> cv_utils_logging_internal_getGlobalLogTag() {
		try {
			cv::utils::logging::LogTag* ret = cv::utils::logging::internal::getGlobalLogTag();
			return Ok<void*>(new cv::utils::logging::LogTag*(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_utils_logging_internal_writeLogMessageEx_LogLevel_const_charX_const_charX_int_const_charX_const_charX(cv::utils::logging::LogLevel logLevel, const char* tag, const char* file, int line, const char* func, const char* message) {
		try {
			cv::utils::logging::internal::writeLogMessageEx(logLevel, tag, file, line, func, message);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_utils_logging_internal_writeLogMessage_LogLevel_const_charX(cv::utils::logging::LogLevel logLevel, const char* message) {
		try {
			cv::utils::logging::internal::writeLogMessage(logLevel, message);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_utils_logging_registerLogTag_LogTagX(void* plogtag) {
		try {
			cv::utils::logging::registerLogTag(reinterpret_cast<cv::utils::logging::LogTag*>(plogtag));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::utils::logging::LogLevel> cv_utils_logging_setLogLevel_LogLevel(cv::utils::logging::LogLevel logLevel) {
		try {
			cv::utils::logging::LogLevel ret = cv::utils::logging::setLogLevel(logLevel);
			return Ok<cv::utils::logging::LogLevel>(ret);
		} OCVRS_CATCH(Result<cv::utils::logging::LogLevel>)
	}
	
	Result_void cv_utils_logging_setLogTagLevel_const_charX_LogLevel(const char* tag, cv::utils::logging::LogLevel level) {
		try {
			cv::utils::logging::setLogTagLevel(tag, level);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_utils_testAsyncArray_const__InputArrayX(void* argument) {
		try {
			cv::AsyncArray ret = cv::utils::testAsyncArray(*reinterpret_cast<const cv::_InputArray*>(argument));
			return Ok<void*>(new cv::AsyncArray(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_utils_testAsyncException() {
		try {
			cv::AsyncArray ret = cv::utils::testAsyncException();
			return Ok<void*>(new cv::AsyncArray(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_va_intel_convertFromVASurface_VADisplay_VASurfaceID_Size_const__OutputArrayX(VADisplay display, VASurfaceID surface, cv::Size size, void* dst) {
		try {
			cv::va_intel::convertFromVASurface(display, surface, size, *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_va_intel_convertToVASurface_VADisplay_const__InputArrayX_VASurfaceID_Size(VADisplay display, void* src, VASurfaceID surface, cv::Size size) {
		try {
			cv::va_intel::convertToVASurface(display, *reinterpret_cast<const cv::_InputArray*>(src), surface, size);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_va_intel_ocl_initializeContextFromVA_VADisplay_bool(VADisplay display, bool tryInterop) {
		try {
			cv::ocl::Context ret = cv::va_intel::ocl::initializeContextFromVA(display, tryInterop);
			return Ok<void*>(new cv::ocl::Context(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_vconcat_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* src1, void* src2, void* dst) {
		try {
			cv::vconcat(*reinterpret_cast<const cv::_InputArray*>(src1), *reinterpret_cast<const cv::_InputArray*>(src2), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_vconcat_const__InputArrayX_const__OutputArrayX(void* src, void* dst) {
		try {
			cv::vconcat(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_writeScalar_FileStorageX_const_StringX(void* fs, const char* value) {
		try {
			cv::writeScalar(*reinterpret_cast<cv::FileStorage*>(fs), std::string(value));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_writeScalar_FileStorageX_double(void* fs, double value) {
		try {
			cv::writeScalar(*reinterpret_cast<cv::FileStorage*>(fs), value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_writeScalar_FileStorageX_float(void* fs, float value) {
		try {
			cv::writeScalar(*reinterpret_cast<cv::FileStorage*>(fs), value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_writeScalar_FileStorageX_int(void* fs, int value) {
		try {
			cv::writeScalar(*reinterpret_cast<cv::FileStorage*>(fs), value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_write_FileStorageX_const_StringX_const_MatX(void* fs, const char* name, void* value) {
		try {
			cv::write(*reinterpret_cast<cv::FileStorage*>(fs), std::string(name), *reinterpret_cast<const cv::Mat*>(value));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_write_FileStorageX_const_StringX_const_SparseMatX(void* fs, const char* name, void* value) {
		try {
			cv::write(*reinterpret_cast<cv::FileStorage*>(fs), std::string(name), *reinterpret_cast<const cv::SparseMat*>(value));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_write_FileStorageX_const_StringX_const_StringX(void* fs, const char* name, const char* value) {
		try {
			cv::write(*reinterpret_cast<cv::FileStorage*>(fs), std::string(name), std::string(value));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_write_FileStorageX_const_StringX_const_vector_DMatch_X(void* fs, const char* name, void* value) {
		try {
			cv::write(*reinterpret_cast<cv::FileStorage*>(fs), std::string(name), *reinterpret_cast<const std::vector<cv::DMatch>*>(value));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_write_FileStorageX_const_StringX_const_vector_KeyPoint_X(void* fs, const char* name, void* value) {
		try {
			cv::write(*reinterpret_cast<cv::FileStorage*>(fs), std::string(name), *reinterpret_cast<const std::vector<cv::KeyPoint>*>(value));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_write_FileStorageX_const_StringX_double(void* fs, const char* name, double value) {
		try {
			cv::write(*reinterpret_cast<cv::FileStorage*>(fs), std::string(name), value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_write_FileStorageX_const_StringX_float(void* fs, const char* name, float value) {
		try {
			cv::write(*reinterpret_cast<cv::FileStorage*>(fs), std::string(name), value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_write_FileStorageX_const_StringX_int(void* fs, const char* name, int value) {
		try {
			cv::write(*reinterpret_cast<cv::FileStorage*>(fs), std::string(name), value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Algorithm_delete(cv::Algorithm* instance) {
		delete instance;
	}
	Result<void*> cv_Algorithm_Algorithm() {
		try {
			cv::Algorithm* ret = new cv::Algorithm();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_Algorithm_clear(void* instance) {
		try {
			reinterpret_cast<cv::Algorithm*>(instance)->clear();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Algorithm_write_const_FileStorageX(void* instance, void* fs) {
		try {
			reinterpret_cast<cv::Algorithm*>(instance)->write(*reinterpret_cast<cv::FileStorage*>(fs));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Algorithm_write_const_const_Ptr_FileStorage_X_const_StringX(void* instance, void* fs, const char* name) {
		try {
			reinterpret_cast<cv::Algorithm*>(instance)->write(*reinterpret_cast<const cv::Ptr<cv::FileStorage>*>(fs), std::string(name));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Algorithm_read_const_FileNodeX(void* instance, void* fn) {
		try {
			reinterpret_cast<cv::Algorithm*>(instance)->read(*reinterpret_cast<const cv::FileNode*>(fn));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_Algorithm_empty_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::Algorithm*>(instance)->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_Algorithm_save_const_const_StringX(void* instance, const char* filename) {
		try {
			reinterpret_cast<cv::Algorithm*>(instance)->save(std::string(filename));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_Algorithm_getDefaultName_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::Algorithm*>(instance)->getDefaultName();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_AsyncArray_delete(cv::AsyncArray* instance) {
		delete instance;
	}
	Result<void*> cv_AsyncArray_AsyncArray() {
		try {
			cv::AsyncArray* ret = new cv::AsyncArray();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_AsyncArray_AsyncArray_const_AsyncArrayX(void* o) {
		try {
			cv::AsyncArray* ret = new cv::AsyncArray(*reinterpret_cast<const cv::AsyncArray*>(o));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_AsyncArray_release(void* instance) {
		try {
			reinterpret_cast<cv::AsyncArray*>(instance)->release();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_AsyncArray_get_const_const__OutputArrayX(void* instance, void* dst) {
		try {
			reinterpret_cast<cv::AsyncArray*>(instance)->get(*reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_AsyncArray_get_const_const__OutputArrayX_int64_t(void* instance, void* dst, int64_t timeoutNs) {
		try {
			bool ret = reinterpret_cast<cv::AsyncArray*>(instance)->get(*reinterpret_cast<const cv::_OutputArray*>(dst), timeoutNs);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_AsyncArray_get_const_const__OutputArrayX_double(void* instance, void* dst, double timeoutNs) {
		try {
			bool ret = reinterpret_cast<cv::AsyncArray*>(instance)->get(*reinterpret_cast<const cv::_OutputArray*>(dst), timeoutNs);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_AsyncArray_wait_for_const_int64_t(void* instance, int64_t timeoutNs) {
		try {
			bool ret = reinterpret_cast<cv::AsyncArray*>(instance)->wait_for(timeoutNs);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_AsyncArray_wait_for_const_double(void* instance, double timeoutNs) {
		try {
			bool ret = reinterpret_cast<cv::AsyncArray*>(instance)->wait_for(timeoutNs);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_AsyncArray_valid_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::AsyncArray*>(instance)->valid();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_AsyncArray_AsyncArray_AsyncArrayX(void* o) {
		try {
			cv::AsyncArray* ret = new cv::AsyncArray(*reinterpret_cast<cv::AsyncArray*>(o));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_AsyncPromise_delete(cv::AsyncPromise* instance) {
		delete instance;
	}
	Result<void*> cv_AsyncPromise_AsyncPromise() {
		try {
			cv::AsyncPromise* ret = new cv::AsyncPromise();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_AsyncPromise_AsyncPromise_const_AsyncPromiseX(void* o) {
		try {
			cv::AsyncPromise* ret = new cv::AsyncPromise(*reinterpret_cast<const cv::AsyncPromise*>(o));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_AsyncPromise_release(void* instance) {
		try {
			reinterpret_cast<cv::AsyncPromise*>(instance)->release();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_AsyncPromise_getArrayResult(void* instance) {
		try {
			cv::AsyncArray ret = reinterpret_cast<cv::AsyncPromise*>(instance)->getArrayResult();
			return Ok<void*>(new cv::AsyncArray(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_AsyncPromise_setValue_const__InputArrayX(void* instance, void* value) {
		try {
			reinterpret_cast<cv::AsyncPromise*>(instance)->setValue(*reinterpret_cast<const cv::_InputArray*>(value));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_AsyncPromise_setException_const_ExceptionX(void* instance, void* exception) {
		try {
			reinterpret_cast<cv::AsyncPromise*>(instance)->setException(*reinterpret_cast<const cv::Exception*>(exception));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_AsyncPromise_AsyncPromise_AsyncPromiseX(void* o) {
		try {
			cv::AsyncPromise* ret = new cv::AsyncPromise(*reinterpret_cast<cv::AsyncPromise*>(o));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_AsyncPromise__getImpl_const(void* instance) {
		try {
			void* ret = reinterpret_cast<cv::AsyncPromise*>(instance)->_getImpl();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_CommandLineParser_delete(cv::CommandLineParser* instance) {
		delete instance;
	}
	Result<void*> cv_CommandLineParser_CommandLineParser_int_const_charXX_const_StringX(int argc, const char** argv, const char* keys) {
		try {
			cv::CommandLineParser* ret = new cv::CommandLineParser(argc, argv, std::string(keys));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_CommandLineParser_CommandLineParser_const_CommandLineParserX(void* parser) {
		try {
			cv::CommandLineParser* ret = new cv::CommandLineParser(*reinterpret_cast<const cv::CommandLineParser*>(parser));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_CommandLineParser_getPathToApplication_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::CommandLineParser*>(instance)->getPathToApplication();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_CommandLineParser_has_const_const_StringX(void* instance, const char* name) {
		try {
			bool ret = reinterpret_cast<cv::CommandLineParser*>(instance)->has(std::string(name));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_CommandLineParser_check_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::CommandLineParser*>(instance)->check();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_CommandLineParser_about_const_StringX(void* instance, const char* message) {
		try {
			reinterpret_cast<cv::CommandLineParser*>(instance)->about(std::string(message));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_CommandLineParser_printMessage_const(void* instance) {
		try {
			reinterpret_cast<cv::CommandLineParser*>(instance)->printMessage();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_CommandLineParser_printErrors_const(void* instance) {
		try {
			reinterpret_cast<cv::CommandLineParser*>(instance)->printErrors();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_ConjGradSolver_create_const_Ptr_Function_X_TermCriteria(void* f, void* termcrit) {
		try {
			cv::Ptr<cv::ConjGradSolver> ret = cv::ConjGradSolver::create(*reinterpret_cast<const cv::Ptr<cv::MinProblemSolver::Function>*>(f), *reinterpret_cast<cv::TermCriteria*>(termcrit));
			return Ok<void*>(new cv::Ptr<cv::ConjGradSolver>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::DMatch> cv_DMatch_DMatch() {
		try {
			cv::DMatch ret;
			return Ok<cv::DMatch>(ret);
		} OCVRS_CATCH(Result<cv::DMatch>)
	}
	
	Result<cv::DMatch> cv_DMatch_DMatch_int_int_float(int _queryIdx, int _trainIdx, float _distance) {
		try {
			cv::DMatch ret(_queryIdx, _trainIdx, _distance);
			return Ok<cv::DMatch>(ret);
		} OCVRS_CATCH(Result<cv::DMatch>)
	}
	
	Result<cv::DMatch> cv_DMatch_DMatch_int_int_int_float(int _queryIdx, int _trainIdx, int _imgIdx, float _distance) {
		try {
			cv::DMatch ret(_queryIdx, _trainIdx, _imgIdx, _distance);
			return Ok<cv::DMatch>(ret);
		} OCVRS_CATCH(Result<cv::DMatch>)
	}
	
	Result_void cv_DownhillSolver_getInitStep_const_const__OutputArrayX(void* instance, void* step) {
		try {
			reinterpret_cast<cv::DownhillSolver*>(instance)->getInitStep(*reinterpret_cast<const cv::_OutputArray*>(step));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DownhillSolver_setInitStep_const__InputArrayX(void* instance, void* step) {
		try {
			reinterpret_cast<cv::DownhillSolver*>(instance)->setInitStep(*reinterpret_cast<const cv::_InputArray*>(step));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_DownhillSolver_create_const_Ptr_Function_X_const__InputArrayX_TermCriteria(void* f, void* initStep, void* termcrit) {
		try {
			cv::Ptr<cv::DownhillSolver> ret = cv::DownhillSolver::create(*reinterpret_cast<const cv::Ptr<cv::MinProblemSolver::Function>*>(f), *reinterpret_cast<const cv::_InputArray*>(initStep), *reinterpret_cast<cv::TermCriteria*>(termcrit));
			return Ok<void*>(new cv::Ptr<cv::DownhillSolver>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Exception_msg_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::Exception*>(instance)->msg;
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_Exception_setMsg_String(void* instance, char* val) {
		try {
			reinterpret_cast<cv::Exception*>(instance)->msg = std::string(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_Exception_code_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::Exception*>(instance)->code;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_Exception_setCode_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::Exception*>(instance)->code = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_Exception_err_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::Exception*>(instance)->err;
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_Exception_setErr_String(void* instance, char* val) {
		try {
			reinterpret_cast<cv::Exception*>(instance)->err = std::string(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_Exception_func_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::Exception*>(instance)->func;
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_Exception_setFunc_String(void* instance, char* val) {
		try {
			reinterpret_cast<cv::Exception*>(instance)->func = std::string(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_Exception_file_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::Exception*>(instance)->file;
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_Exception_setFile_String(void* instance, char* val) {
		try {
			reinterpret_cast<cv::Exception*>(instance)->file = std::string(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_Exception_line_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::Exception*>(instance)->line;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_Exception_setLine_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::Exception*>(instance)->line = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Exception_delete(cv::Exception* instance) {
		delete instance;
	}
	Result<void*> cv_Exception_Exception() {
		try {
			cv::Exception* ret = new cv::Exception();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Exception_Exception_int_const_StringX_const_StringX_const_StringX_int(int _code, const char* _err, const char* _func, const char* _file, int _line) {
		try {
			cv::Exception* ret = new cv::Exception(_code, std::string(_err), std::string(_func), std::string(_file), _line);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Exception_what_const(void* instance) {
		try {
			const char* ret = reinterpret_cast<cv::Exception*>(instance)->what();
			return Ok<void*>(ocvrs_create_string(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_Exception_formatMessage(void* instance) {
		try {
			reinterpret_cast<cv::Exception*>(instance)->formatMessage();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_FileNode_fs_const(void* instance) {
		try {
			const cv::FileStorage* ret = reinterpret_cast<cv::FileNode*>(instance)->fs;
			return Ok<void*>(new const cv::FileStorage*(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<size_t> cv_FileNode_blockIdx_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::FileNode*>(instance)->blockIdx;
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result_void cv_FileNode_setBlockIdx_size_t(void* instance, size_t val) {
		try {
			reinterpret_cast<cv::FileNode*>(instance)->blockIdx = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<size_t> cv_FileNode_ofs_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::FileNode*>(instance)->ofs;
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result_void cv_FileNode_setOfs_size_t(void* instance, size_t val) {
		try {
			reinterpret_cast<cv::FileNode*>(instance)->ofs = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_FileNode_delete(cv::FileNode* instance) {
		delete instance;
	}
	Result<void*> cv_FileNode_FileNode() {
		try {
			cv::FileNode* ret = new cv::FileNode();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_FileNode_FileNode_const_FileStorageX_size_t_size_t(void* fs, size_t blockIdx, size_t ofs) {
		try {
			cv::FileNode* ret = new cv::FileNode(reinterpret_cast<const cv::FileStorage*>(fs), blockIdx, ofs);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_FileNode_FileNode_const_FileNodeX(void* node) {
		try {
			cv::FileNode* ret = new cv::FileNode(*reinterpret_cast<const cv::FileNode*>(node));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_FileNode_operator___const_const_StringX(void* instance, const char* nodename) {
		try {
			cv::FileNode ret = reinterpret_cast<cv::FileNode*>(instance)->operator[](std::string(nodename));
			return Ok<void*>(new cv::FileNode(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_FileNode_operator___const_const_charX(void* instance, const char* nodename) {
		try {
			cv::FileNode ret = reinterpret_cast<cv::FileNode*>(instance)->operator[](nodename);
			return Ok<void*>(new cv::FileNode(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_FileNode_operator___const_int(void* instance, int i) {
		try {
			cv::FileNode ret = reinterpret_cast<cv::FileNode*>(instance)->operator[](i);
			return Ok<void*>(new cv::FileNode(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_FileNode_keys_const(void* instance) {
		try {
			std::vector<cv::String> ret = reinterpret_cast<cv::FileNode*>(instance)->keys();
			return Ok<void*>(new std::vector<cv::String>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_FileNode_type_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::FileNode*>(instance)->type();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<bool> cv_FileNode_empty_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::FileNode*>(instance)->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_FileNode_isNone_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::FileNode*>(instance)->isNone();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_FileNode_isSeq_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::FileNode*>(instance)->isSeq();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_FileNode_isMap_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::FileNode*>(instance)->isMap();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_FileNode_isInt_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::FileNode*>(instance)->isInt();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_FileNode_isReal_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::FileNode*>(instance)->isReal();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_FileNode_isString_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::FileNode*>(instance)->isString();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_FileNode_isNamed_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::FileNode*>(instance)->isNamed();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_FileNode_name_const(void* instance) {
		try {
			std::string ret = reinterpret_cast<cv::FileNode*>(instance)->name();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<size_t> cv_FileNode_size_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::FileNode*>(instance)->size();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<size_t> cv_FileNode_rawSize_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::FileNode*>(instance)->rawSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<int> cv_FileNode_operator_int_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::FileNode*>(instance)->operator int();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<float> cv_FileNode_operator_float_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::FileNode*>(instance)->operator float();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<double> cv_FileNode_operator_double_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::FileNode*>(instance)->operator double();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<void*> cv_FileNode_operator_std_string_const(void* instance) {
		try {
			std::string ret = reinterpret_cast<cv::FileNode*>(instance)->operator std::string();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_FileNode_isMap_int(int flags) {
		try {
			bool ret = cv::FileNode::isMap(flags);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_FileNode_isSeq_int(int flags) {
		try {
			bool ret = cv::FileNode::isSeq(flags);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_FileNode_isCollection_int(int flags) {
		try {
			bool ret = cv::FileNode::isCollection(flags);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_FileNode_isEmptyCollection_int(int flags) {
		try {
			bool ret = cv::FileNode::isEmptyCollection(flags);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_FileNode_isFlow_int(int flags) {
		try {
			bool ret = cv::FileNode::isFlow(flags);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<unsigned char*> cv_FileNode_ptr(void* instance) {
		try {
			unsigned char* ret = reinterpret_cast<cv::FileNode*>(instance)->ptr();
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(Result<unsigned char*>)
	}
	
	Result<const unsigned char*> cv_FileNode_ptr_const(void* instance) {
		try {
			const unsigned char* ret = reinterpret_cast<cv::FileNode*>(instance)->ptr();
			return Ok<const unsigned char*>(ret);
		} OCVRS_CATCH(Result<const unsigned char*>)
	}
	
	Result<void*> cv_FileNode_begin_const(void* instance) {
		try {
			cv::FileNodeIterator ret = reinterpret_cast<cv::FileNode*>(instance)->begin();
			return Ok<void*>(new cv::FileNodeIterator(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_FileNode_end_const(void* instance) {
		try {
			cv::FileNodeIterator ret = reinterpret_cast<cv::FileNode*>(instance)->end();
			return Ok<void*>(new cv::FileNodeIterator(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_FileNode_readRaw_const_const_StringX_voidX_size_t(void* instance, const char* fmt, void* vec, size_t len) {
		try {
			reinterpret_cast<cv::FileNode*>(instance)->readRaw(std::string(fmt), vec, len);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_FileNode_setValue_int_const_voidX_int(void* instance, int type, const void* value, int len) {
		try {
			reinterpret_cast<cv::FileNode*>(instance)->setValue(type, value, len);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_FileNode_real_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::FileNode*>(instance)->real();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<void*> cv_FileNode_string_const(void* instance) {
		try {
			std::string ret = reinterpret_cast<cv::FileNode*>(instance)->string();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_FileNode_mat_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::FileNode*>(instance)->mat();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_FileNodeIterator_delete(cv::FileNodeIterator* instance) {
		delete instance;
	}
	Result<void*> cv_FileNodeIterator_FileNodeIterator() {
		try {
			cv::FileNodeIterator* ret = new cv::FileNodeIterator();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_FileNodeIterator_FileNodeIterator_const_FileNodeX_bool(void* node, bool seekEnd) {
		try {
			cv::FileNodeIterator* ret = new cv::FileNodeIterator(*reinterpret_cast<const cv::FileNode*>(node), seekEnd);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_FileNodeIterator_FileNodeIterator_const_FileNodeIteratorX(void* it) {
		try {
			cv::FileNodeIterator* ret = new cv::FileNodeIterator(*reinterpret_cast<const cv::FileNodeIterator*>(it));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_FileNodeIterator_readRaw_const_StringX_voidX_size_t(void* instance, const char* fmt, void* vec, size_t len) {
		try {
			cv::FileNodeIterator ret = reinterpret_cast<cv::FileNodeIterator*>(instance)->readRaw(std::string(fmt), vec, len);
			return Ok<void*>(new cv::FileNodeIterator(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<size_t> cv_FileNodeIterator_remaining_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::FileNodeIterator*>(instance)->remaining();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<bool> cv_FileNodeIterator_equalTo_const_const_FileNodeIteratorX(void* instance, void* it) {
		try {
			bool ret = reinterpret_cast<cv::FileNodeIterator*>(instance)->equalTo(*reinterpret_cast<const cv::FileNodeIterator*>(it));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<int> cv_FileStorage_state_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::FileStorage*>(instance)->state;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_FileStorage_setState_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::FileStorage*>(instance)->state = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_FileStorage_elname_const(void* instance) {
		try {
			std::string ret = reinterpret_cast<cv::FileStorage*>(instance)->elname;
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_FileStorage_setElname_string(void* instance, char* val) {
		try {
			reinterpret_cast<cv::FileStorage*>(instance)->elname = std::string(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_FileStorage_delete(cv::FileStorage* instance) {
		delete instance;
	}
	Result<void*> cv_FileStorage_FileStorage() {
		try {
			cv::FileStorage* ret = new cv::FileStorage();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_FileStorage_FileStorage_const_StringX_int_const_StringX(const char* filename, int flags, const char* encoding) {
		try {
			cv::FileStorage* ret = new cv::FileStorage(std::string(filename), flags, std::string(encoding));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_FileStorage_open_const_StringX_int_const_StringX(void* instance, const char* filename, int flags, const char* encoding) {
		try {
			bool ret = reinterpret_cast<cv::FileStorage*>(instance)->open(std::string(filename), flags, std::string(encoding));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_FileStorage_isOpened_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::FileStorage*>(instance)->isOpened();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_FileStorage_release(void* instance) {
		try {
			reinterpret_cast<cv::FileStorage*>(instance)->release();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_FileStorage_releaseAndGetString(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::FileStorage*>(instance)->releaseAndGetString();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_FileStorage_getFirstTopLevelNode_const(void* instance) {
		try {
			cv::FileNode ret = reinterpret_cast<cv::FileStorage*>(instance)->getFirstTopLevelNode();
			return Ok<void*>(new cv::FileNode(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_FileStorage_root_const_int(void* instance, int streamidx) {
		try {
			cv::FileNode ret = reinterpret_cast<cv::FileStorage*>(instance)->root(streamidx);
			return Ok<void*>(new cv::FileNode(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_FileStorage_operator___const_const_StringX(void* instance, const char* nodename) {
		try {
			cv::FileNode ret = reinterpret_cast<cv::FileStorage*>(instance)->operator[](std::string(nodename));
			return Ok<void*>(new cv::FileNode(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_FileStorage_operator___const_const_charX(void* instance, const char* nodename) {
		try {
			cv::FileNode ret = reinterpret_cast<cv::FileStorage*>(instance)->operator[](nodename);
			return Ok<void*>(new cv::FileNode(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_FileStorage_write_const_StringX_int(void* instance, const char* name, int val) {
		try {
			reinterpret_cast<cv::FileStorage*>(instance)->write(std::string(name), val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_FileStorage_write_const_StringX_double(void* instance, const char* name, double val) {
		try {
			reinterpret_cast<cv::FileStorage*>(instance)->write(std::string(name), val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_FileStorage_write_const_StringX_const_StringX(void* instance, const char* name, const char* val) {
		try {
			reinterpret_cast<cv::FileStorage*>(instance)->write(std::string(name), std::string(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_FileStorage_write_const_StringX_const_MatX(void* instance, const char* name, void* val) {
		try {
			reinterpret_cast<cv::FileStorage*>(instance)->write(std::string(name), *reinterpret_cast<const cv::Mat*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_FileStorage_write_const_StringX_const_vector_String_X(void* instance, const char* name, void* val) {
		try {
			reinterpret_cast<cv::FileStorage*>(instance)->write(std::string(name), *reinterpret_cast<const std::vector<cv::String>*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_FileStorage_writeRaw_const_StringX_const_voidX_size_t(void* instance, const char* fmt, const void* vec, size_t len) {
		try {
			reinterpret_cast<cv::FileStorage*>(instance)->writeRaw(std::string(fmt), vec, len);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_FileStorage_writeComment_const_StringX_bool(void* instance, const char* comment, bool append) {
		try {
			reinterpret_cast<cv::FileStorage*>(instance)->writeComment(std::string(comment), append);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_FileStorage_startWriteStruct_const_StringX_int_const_StringX(void* instance, const char* name, int flags, const char* typeName) {
		try {
			reinterpret_cast<cv::FileStorage*>(instance)->startWriteStruct(std::string(name), flags, std::string(typeName));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_FileStorage_endWriteStruct(void* instance) {
		try {
			reinterpret_cast<cv::FileStorage*>(instance)->endWriteStruct();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_FileStorage_getDefaultObjectName_const_StringX(const char* filename) {
		try {
			cv::String ret = cv::FileStorage::getDefaultObjectName(std::string(filename));
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_FileStorage_getFormat_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::FileStorage*>(instance)->getFormat();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv_Formatted_next(void* instance) {
		try {
			const char* ret = reinterpret_cast<cv::Formatted*>(instance)->next();
			return Ok<void*>(ocvrs_create_string(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_Formatted_reset(void* instance) {
		try {
			reinterpret_cast<cv::Formatted*>(instance)->reset();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_Formatter_format_const_const_MatX(void* instance, void* mtx) {
		try {
			cv::Ptr<cv::Formatted> ret = reinterpret_cast<cv::Formatter*>(instance)->format(*reinterpret_cast<const cv::Mat*>(mtx));
			return Ok<void*>(new cv::Ptr<cv::Formatted>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_Formatter_set16fPrecision_int(void* instance, int p) {
		try {
			reinterpret_cast<cv::Formatter*>(instance)->set16fPrecision(p);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Formatter_set32fPrecision_int(void* instance, int p) {
		try {
			reinterpret_cast<cv::Formatter*>(instance)->set32fPrecision(p);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Formatter_set64fPrecision_int(void* instance, int p) {
		try {
			reinterpret_cast<cv::Formatter*>(instance)->set64fPrecision(p);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Formatter_setMultiline_bool(void* instance, bool ml) {
		try {
			reinterpret_cast<cv::Formatter*>(instance)->setMultiline(ml);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_Formatter_get_FormatType(cv::Formatter::FormatType fmt) {
		try {
			cv::Ptr<cv::Formatter> ret = cv::Formatter::get(fmt);
			return Ok<void*>(new cv::Ptr<cv::Formatter>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::NormTypes> cv_Hamming_normType_const(void* instance) {
		try {
			cv::NormTypes ret = reinterpret_cast<cv::Hamming*>(instance)->normType;
			return Ok<cv::NormTypes>(ret);
		} OCVRS_CATCH(Result<cv::NormTypes>)
	}
	
	void cv_Hamming_delete(cv::Hamming* instance) {
		delete instance;
	}
	Result<cv::KeyPoint> cv_KeyPoint_KeyPoint() {
		try {
			cv::KeyPoint ret;
			return Ok<cv::KeyPoint>(ret);
		} OCVRS_CATCH(Result<cv::KeyPoint>)
	}
	
	Result<cv::KeyPoint> cv_KeyPoint_KeyPoint_Point2f_float_float_float_int_int(cv::Point2f _pt, float _size, float _angle, float _response, int _octave, int _class_id) {
		try {
			cv::KeyPoint ret(_pt, _size, _angle, _response, _octave, _class_id);
			return Ok<cv::KeyPoint>(ret);
		} OCVRS_CATCH(Result<cv::KeyPoint>)
	}
	
	Result<cv::KeyPoint> cv_KeyPoint_KeyPoint_float_float_float_float_float_int_int(float x, float y, float _size, float _angle, float _response, int _octave, int _class_id) {
		try {
			cv::KeyPoint ret(x, y, _size, _angle, _response, _octave, _class_id);
			return Ok<cv::KeyPoint>(ret);
		} OCVRS_CATCH(Result<cv::KeyPoint>)
	}
	
	Result<size_t> cv_KeyPoint_hash_const(cv::KeyPoint instance) {
		try {
			size_t ret = reinterpret_cast<cv::KeyPoint*>(&instance)->hash();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result_void cv_KeyPoint_convert_const_vector_KeyPoint_X_vector_Point2f_X_const_vector_int_X(void* keypoints, void* points2f, void* keypointIndexes) {
		try {
			cv::KeyPoint::convert(*reinterpret_cast<const std::vector<cv::KeyPoint>*>(keypoints), *reinterpret_cast<std::vector<cv::Point2f>*>(points2f), *reinterpret_cast<const std::vector<int>*>(keypointIndexes));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_KeyPoint_convert_const_vector_Point2f_X_vector_KeyPoint_X_float_float_int_int(void* points2f, void* keypoints, float size, float response, int octave, int class_id) {
		try {
			cv::KeyPoint::convert(*reinterpret_cast<const std::vector<cv::Point2f>*>(points2f), *reinterpret_cast<std::vector<cv::KeyPoint>*>(keypoints), size, response, octave, class_id);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_KeyPoint_overlap_const_KeyPointX_const_KeyPointX(const cv::KeyPoint* kp1, const cv::KeyPoint* kp2) {
		try {
			float ret = cv::KeyPoint::overlap(*kp1, *kp2);
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	void cv_LDA_delete(cv::LDA* instance) {
		delete instance;
	}
	Result<void*> cv_LDA_LDA_int(int num_components) {
		try {
			cv::LDA* ret = new cv::LDA(num_components);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_LDA_LDA_const__InputArrayX_const__InputArrayX_int(void* src, void* labels, int num_components) {
		try {
			cv::LDA* ret = new cv::LDA(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputArray*>(labels), num_components);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_LDA_save_const_const_StringX(void* instance, const char* filename) {
		try {
			reinterpret_cast<cv::LDA*>(instance)->save(std::string(filename));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_LDA_load_const_StringX(void* instance, const char* filename) {
		try {
			reinterpret_cast<cv::LDA*>(instance)->load(std::string(filename));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_LDA_save_const_FileStorageX(void* instance, void* fs) {
		try {
			reinterpret_cast<cv::LDA*>(instance)->save(*reinterpret_cast<cv::FileStorage*>(fs));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_LDA_load_const_FileStorageX(void* instance, void* node) {
		try {
			reinterpret_cast<cv::LDA*>(instance)->load(*reinterpret_cast<const cv::FileStorage*>(node));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_LDA_compute_const__InputArrayX_const__InputArrayX(void* instance, void* src, void* labels) {
		try {
			reinterpret_cast<cv::LDA*>(instance)->compute(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputArray*>(labels));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_LDA_project_const__InputArrayX(void* instance, void* src) {
		try {
			cv::Mat ret = reinterpret_cast<cv::LDA*>(instance)->project(*reinterpret_cast<const cv::_InputArray*>(src));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_LDA_reconstruct_const__InputArrayX(void* instance, void* src) {
		try {
			cv::Mat ret = reinterpret_cast<cv::LDA*>(instance)->reconstruct(*reinterpret_cast<const cv::_InputArray*>(src));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_LDA_eigenvectors_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::LDA*>(instance)->eigenvectors();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_LDA_eigenvalues_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::LDA*>(instance)->eigenvalues();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_LDA_subspaceProject_const__InputArrayX_const__InputArrayX_const__InputArrayX(void* W, void* mean, void* src) {
		try {
			cv::Mat ret = cv::LDA::subspaceProject(*reinterpret_cast<const cv::_InputArray*>(W), *reinterpret_cast<const cv::_InputArray*>(mean), *reinterpret_cast<const cv::_InputArray*>(src));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_LDA_subspaceReconstruct_const__InputArrayX_const__InputArrayX_const__InputArrayX(void* W, void* mean, void* src) {
		try {
			cv::Mat ret = cv::LDA::subspaceReconstruct(*reinterpret_cast<const cv::_InputArray*>(W), *reinterpret_cast<const cv::_InputArray*>(mean), *reinterpret_cast<const cv::_InputArray*>(src));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_Mat_flags_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::Mat*>(instance)->flags;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_Mat_setFlags_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::Mat*>(instance)->flags = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_Mat_dims_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::Mat*>(instance)->dims;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_Mat_setDims_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::Mat*>(instance)->dims = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_Mat_rows_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::Mat*>(instance)->rows;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_Mat_setRows_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::Mat*>(instance)->rows = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_Mat_cols_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::Mat*>(instance)->cols;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_Mat_setCols_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::Mat*>(instance)->cols = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<unsigned char*> cv_Mat_data(void* instance) {
		try {
			unsigned char* ret = reinterpret_cast<cv::Mat*>(instance)->data;
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(Result<unsigned char*>)
	}
	
	Result_void cv_Mat_setData_unsigned_charX(void* instance, unsigned char* val) {
		try {
			reinterpret_cast<cv::Mat*>(instance)->data = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<const unsigned char*> cv_Mat_datastart_const(void* instance) {
		try {
			const unsigned char* ret = reinterpret_cast<cv::Mat*>(instance)->datastart;
			return Ok<const unsigned char*>(ret);
		} OCVRS_CATCH(Result<const unsigned char*>)
	}
	
	Result<const unsigned char*> cv_Mat_dataend_const(void* instance) {
		try {
			const unsigned char* ret = reinterpret_cast<cv::Mat*>(instance)->dataend;
			return Ok<const unsigned char*>(ret);
		} OCVRS_CATCH(Result<const unsigned char*>)
	}
	
	Result<const unsigned char*> cv_Mat_datalimit_const(void* instance) {
		try {
			const unsigned char* ret = reinterpret_cast<cv::Mat*>(instance)->datalimit;
			return Ok<const unsigned char*>(ret);
		} OCVRS_CATCH(Result<const unsigned char*>)
	}
	
	Result<void*> cv_Mat_u(void* instance) {
		try {
			cv::UMatData* ret = reinterpret_cast<cv::Mat*>(instance)->u;
			return Ok<void*>(new cv::UMatData*(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_Mat_setU_UMatDataX(void* instance, void* val) {
		try {
			reinterpret_cast<cv::Mat*>(instance)->u = reinterpret_cast<cv::UMatData*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_Mat_size_const(void* instance) {
		try {
			cv::MatSize ret = reinterpret_cast<cv::Mat*>(instance)->size;
			return Ok<void*>(new cv::MatSize(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_step_const(void* instance) {
		try {
			cv::MatStep ret = reinterpret_cast<cv::Mat*>(instance)->step;
			return Ok<void*>(new cv::MatStep(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Mat_delete(cv::Mat* instance) {
		delete instance;
	}
	Result<void*> cv_Mat_Mat() {
		try {
			cv::Mat* ret = new cv::Mat();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_Mat_int_int_int(int rows, int cols, int type) {
		try {
			cv::Mat* ret = new cv::Mat(rows, cols, type);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_Mat_Size_int(cv::Size size, int type) {
		try {
			cv::Mat* ret = new cv::Mat(size, type);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_Mat_int_int_int_const_ScalarX(int rows, int cols, int type, const cv::Scalar* s) {
		try {
			cv::Mat* ret = new cv::Mat(rows, cols, type, *s);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_Mat_Size_int_const_ScalarX(cv::Size size, int type, const cv::Scalar* s) {
		try {
			cv::Mat* ret = new cv::Mat(size, type, *s);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_Mat_int_const_intX_int(int ndims, const int* sizes, int type) {
		try {
			cv::Mat* ret = new cv::Mat(ndims, sizes, type);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_Mat_const_vector_int_X_int(void* sizes, int type) {
		try {
			cv::Mat* ret = new cv::Mat(*reinterpret_cast<const std::vector<int>*>(sizes), type);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_Mat_int_const_intX_int_const_ScalarX(int ndims, const int* sizes, int type, const cv::Scalar* s) {
		try {
			cv::Mat* ret = new cv::Mat(ndims, sizes, type, *s);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_Mat_const_vector_int_X_int_const_ScalarX(void* sizes, int type, const cv::Scalar* s) {
		try {
			cv::Mat* ret = new cv::Mat(*reinterpret_cast<const std::vector<int>*>(sizes), type, *s);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_Mat_const_MatX(void* m) {
		try {
			cv::Mat* ret = new cv::Mat(*reinterpret_cast<const cv::Mat*>(m));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_Mat_int_int_int_voidX_size_t(int rows, int cols, int type, void* data, size_t step) {
		try {
			cv::Mat* ret = new cv::Mat(rows, cols, type, data, step);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_Mat_Size_int_voidX_size_t(cv::Size size, int type, void* data, size_t step) {
		try {
			cv::Mat* ret = new cv::Mat(size, type, data, step);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_Mat_int_const_intX_int_voidX_const_size_tX(int ndims, const int* sizes, int type, void* data, const size_t* steps) {
		try {
			cv::Mat* ret = new cv::Mat(ndims, sizes, type, data, steps);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_Mat_const_vector_int_X_int_voidX_const_size_tX(void* sizes, int type, void* data, const size_t* steps) {
		try {
			cv::Mat* ret = new cv::Mat(*reinterpret_cast<const std::vector<int>*>(sizes), type, data, steps);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_Mat_const_MatX_const_RangeX_const_RangeX(void* m, void* rowRange, void* colRange) {
		try {
			cv::Mat* ret = new cv::Mat(*reinterpret_cast<const cv::Mat*>(m), *reinterpret_cast<const cv::Range*>(rowRange), *reinterpret_cast<const cv::Range*>(colRange));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_Mat_const_MatX_const_RectX(void* m, const cv::Rect* roi) {
		try {
			cv::Mat* ret = new cv::Mat(*reinterpret_cast<const cv::Mat*>(m), *roi);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_Mat_const_MatX_const_vector_Range_X(void* m, void* ranges) {
		try {
			cv::Mat* ret = new cv::Mat(*reinterpret_cast<const cv::Mat*>(m), *reinterpret_cast<const std::vector<cv::Range>*>(ranges));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_getUMat_const_AccessFlag_UMatUsageFlags(void* instance, cv::AccessFlag accessFlags, cv::UMatUsageFlags usageFlags) {
		try {
			cv::UMat ret = reinterpret_cast<cv::Mat*>(instance)->getUMat(accessFlags, usageFlags);
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_row_const_int(void* instance, int y) {
		try {
			cv::Mat ret = reinterpret_cast<cv::Mat*>(instance)->row(y);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_col_const_int(void* instance, int x) {
		try {
			cv::Mat ret = reinterpret_cast<cv::Mat*>(instance)->col(x);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_rowRange_const_int_int(void* instance, int startrow, int endrow) {
		try {
			cv::Mat ret = reinterpret_cast<cv::Mat*>(instance)->rowRange(startrow, endrow);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_rowRange_const_const_RangeX(void* instance, void* r) {
		try {
			cv::Mat ret = reinterpret_cast<cv::Mat*>(instance)->rowRange(*reinterpret_cast<const cv::Range*>(r));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_colRange_const_int_int(void* instance, int startcol, int endcol) {
		try {
			cv::Mat ret = reinterpret_cast<cv::Mat*>(instance)->colRange(startcol, endcol);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_colRange_const_const_RangeX(void* instance, void* r) {
		try {
			cv::Mat ret = reinterpret_cast<cv::Mat*>(instance)->colRange(*reinterpret_cast<const cv::Range*>(r));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_diag_const_int(void* instance, int d) {
		try {
			cv::Mat ret = reinterpret_cast<cv::Mat*>(instance)->diag(d);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_diag_const_MatX(void* d) {
		try {
			cv::Mat ret = cv::Mat::diag(*reinterpret_cast<const cv::Mat*>(d));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_clone_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::Mat*>(instance)->clone();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_Mat_copyTo_const_const__OutputArrayX(void* instance, void* m) {
		try {
			reinterpret_cast<cv::Mat*>(instance)->copyTo(*reinterpret_cast<const cv::_OutputArray*>(m));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Mat_copyTo_const_const__OutputArrayX_const__InputArrayX(void* instance, void* m, void* mask) {
		try {
			reinterpret_cast<cv::Mat*>(instance)->copyTo(*reinterpret_cast<const cv::_OutputArray*>(m), *reinterpret_cast<const cv::_InputArray*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Mat_convertTo_const_const__OutputArrayX_int_double_double(void* instance, void* m, int rtype, double alpha, double beta) {
		try {
			reinterpret_cast<cv::Mat*>(instance)->convertTo(*reinterpret_cast<const cv::_OutputArray*>(m), rtype, alpha, beta);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Mat_assignTo_const_MatX_int(void* instance, void* m, int type) {
		try {
			reinterpret_cast<cv::Mat*>(instance)->assignTo(*reinterpret_cast<cv::Mat*>(m), type);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_Mat_setTo_const__InputArrayX_const__InputArrayX(void* instance, void* value, void* mask) {
		try {
			cv::Mat ret = reinterpret_cast<cv::Mat*>(instance)->setTo(*reinterpret_cast<const cv::_InputArray*>(value), *reinterpret_cast<const cv::_InputArray*>(mask));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_reshape_const_int_int(void* instance, int cn, int rows) {
		try {
			cv::Mat ret = reinterpret_cast<cv::Mat*>(instance)->reshape(cn, rows);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_reshape_const_int_int_const_intX(void* instance, int cn, int newndims, const int* newsz) {
		try {
			cv::Mat ret = reinterpret_cast<cv::Mat*>(instance)->reshape(cn, newndims, newsz);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_reshape_const_int_const_vector_int_X(void* instance, int cn, void* newshape) {
		try {
			cv::Mat ret = reinterpret_cast<cv::Mat*>(instance)->reshape(cn, *reinterpret_cast<const std::vector<int>*>(newshape));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_t_const(void* instance) {
		try {
			cv::MatExpr ret = reinterpret_cast<cv::Mat*>(instance)->t();
			return Ok<void*>(new cv::MatExpr(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_inv_const_int(void* instance, int method) {
		try {
			cv::MatExpr ret = reinterpret_cast<cv::Mat*>(instance)->inv(method);
			return Ok<void*>(new cv::MatExpr(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_mul_const_const__InputArrayX_double(void* instance, void* m, double scale) {
		try {
			cv::MatExpr ret = reinterpret_cast<cv::Mat*>(instance)->mul(*reinterpret_cast<const cv::_InputArray*>(m), scale);
			return Ok<void*>(new cv::MatExpr(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_cross_const_const__InputArrayX(void* instance, void* m) {
		try {
			cv::Mat ret = reinterpret_cast<cv::Mat*>(instance)->cross(*reinterpret_cast<const cv::_InputArray*>(m));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<double> cv_Mat_dot_const_const__InputArrayX(void* instance, void* m) {
		try {
			double ret = reinterpret_cast<cv::Mat*>(instance)->dot(*reinterpret_cast<const cv::_InputArray*>(m));
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<void*> cv_Mat_zeros_int_int_int(int rows, int cols, int type) {
		try {
			cv::MatExpr ret = cv::Mat::zeros(rows, cols, type);
			return Ok<void*>(new cv::MatExpr(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_zeros_Size_int(cv::Size size, int type) {
		try {
			cv::MatExpr ret = cv::Mat::zeros(size, type);
			return Ok<void*>(new cv::MatExpr(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_zeros_int_const_intX_int(int ndims, const int* sz, int type) {
		try {
			cv::MatExpr ret = cv::Mat::zeros(ndims, sz, type);
			return Ok<void*>(new cv::MatExpr(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_ones_int_int_int(int rows, int cols, int type) {
		try {
			cv::MatExpr ret = cv::Mat::ones(rows, cols, type);
			return Ok<void*>(new cv::MatExpr(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_ones_Size_int(cv::Size size, int type) {
		try {
			cv::MatExpr ret = cv::Mat::ones(size, type);
			return Ok<void*>(new cv::MatExpr(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_ones_int_const_intX_int(int ndims, const int* sz, int type) {
		try {
			cv::MatExpr ret = cv::Mat::ones(ndims, sz, type);
			return Ok<void*>(new cv::MatExpr(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_eye_int_int_int(int rows, int cols, int type) {
		try {
			cv::MatExpr ret = cv::Mat::eye(rows, cols, type);
			return Ok<void*>(new cv::MatExpr(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Mat_eye_Size_int(cv::Size size, int type) {
		try {
			cv::MatExpr ret = cv::Mat::eye(size, type);
			return Ok<void*>(new cv::MatExpr(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_Mat_create_int_int_int(void* instance, int rows, int cols, int type) {
		try {
			reinterpret_cast<cv::Mat*>(instance)->create(rows, cols, type);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Mat_create_Size_int(void* instance, cv::Size size, int type) {
		try {
			reinterpret_cast<cv::Mat*>(instance)->create(size, type);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Mat_create_int_const_intX_int(void* instance, int ndims, const int* sizes, int type) {
		try {
			reinterpret_cast<cv::Mat*>(instance)->create(ndims, sizes, type);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Mat_create_const_vector_int_X_int(void* instance, void* sizes, int type) {
		try {
			reinterpret_cast<cv::Mat*>(instance)->create(*reinterpret_cast<const std::vector<int>*>(sizes), type);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Mat_addref(void* instance) {
		try {
			reinterpret_cast<cv::Mat*>(instance)->addref();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Mat_release(void* instance) {
		try {
			reinterpret_cast<cv::Mat*>(instance)->release();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Mat_deallocate(void* instance) {
		try {
			reinterpret_cast<cv::Mat*>(instance)->deallocate();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Mat_reserve_size_t(void* instance, size_t sz) {
		try {
			reinterpret_cast<cv::Mat*>(instance)->reserve(sz);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Mat_reserveBuffer_size_t(void* instance, size_t sz) {
		try {
			reinterpret_cast<cv::Mat*>(instance)->reserveBuffer(sz);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Mat_resize_size_t(void* instance, size_t sz) {
		try {
			reinterpret_cast<cv::Mat*>(instance)->resize(sz);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Mat_resize_size_t_const_ScalarX(void* instance, size_t sz, const cv::Scalar* s) {
		try {
			reinterpret_cast<cv::Mat*>(instance)->resize(sz, *s);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Mat_push_back_const_MatX(void* instance, void* m) {
		try {
			reinterpret_cast<cv::Mat*>(instance)->push_back(*reinterpret_cast<const cv::Mat*>(m));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Mat_pop_back_size_t(void* instance, size_t nelems) {
		try {
			reinterpret_cast<cv::Mat*>(instance)->pop_back(nelems);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Mat_locateROI_const_SizeX_PointX(void* instance, cv::Size* wholeSize, cv::Point* ofs) {
		try {
			reinterpret_cast<cv::Mat*>(instance)->locateROI(*wholeSize, *ofs);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_Mat_adjustROI_int_int_int_int(void* instance, int dtop, int dbottom, int dleft, int dright) {
		try {
			cv::Mat ret = reinterpret_cast<cv::Mat*>(instance)->adjustROI(dtop, dbottom, dleft, dright);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_Mat_isContinuous_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::Mat*>(instance)->isContinuous();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_Mat_isSubmatrix_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::Mat*>(instance)->isSubmatrix();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<size_t> cv_Mat_elemSize_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::Mat*>(instance)->elemSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<size_t> cv_Mat_elemSize1_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::Mat*>(instance)->elemSize1();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<int> cv_Mat_type_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::Mat*>(instance)->type();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_Mat_depth_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::Mat*>(instance)->depth();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_Mat_channels_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::Mat*>(instance)->channels();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<size_t> cv_Mat_step1_const_int(void* instance, int i) {
		try {
			size_t ret = reinterpret_cast<cv::Mat*>(instance)->step1(i);
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<bool> cv_Mat_empty_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::Mat*>(instance)->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<size_t> cv_Mat_total_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::Mat*>(instance)->total();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<size_t> cv_Mat_total_const_int_int(void* instance, int startDim, int endDim) {
		try {
			size_t ret = reinterpret_cast<cv::Mat*>(instance)->total(startDim, endDim);
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<int> cv_Mat_checkVector_const_int_int_bool(void* instance, int elemChannels, int depth, bool requireContinuous) {
		try {
			int ret = reinterpret_cast<cv::Mat*>(instance)->checkVector(elemChannels, depth, requireContinuous);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<unsigned char*> cv_Mat_ptr_int(void* instance, int i0) {
		try {
			unsigned char* ret = reinterpret_cast<cv::Mat*>(instance)->ptr(i0);
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(Result<unsigned char*>)
	}
	
	Result<const unsigned char*> cv_Mat_ptr_const_int(void* instance, int i0) {
		try {
			const unsigned char* ret = reinterpret_cast<cv::Mat*>(instance)->ptr(i0);
			return Ok<const unsigned char*>(ret);
		} OCVRS_CATCH(Result<const unsigned char*>)
	}
	
	Result<unsigned char*> cv_Mat_ptr_int_int(void* instance, int row, int col) {
		try {
			unsigned char* ret = reinterpret_cast<cv::Mat*>(instance)->ptr(row, col);
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(Result<unsigned char*>)
	}
	
	Result<const unsigned char*> cv_Mat_ptr_const_int_int(void* instance, int row, int col) {
		try {
			const unsigned char* ret = reinterpret_cast<cv::Mat*>(instance)->ptr(row, col);
			return Ok<const unsigned char*>(ret);
		} OCVRS_CATCH(Result<const unsigned char*>)
	}
	
	Result<unsigned char*> cv_Mat_ptr_int_int_int(void* instance, int i0, int i1, int i2) {
		try {
			unsigned char* ret = reinterpret_cast<cv::Mat*>(instance)->ptr(i0, i1, i2);
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(Result<unsigned char*>)
	}
	
	Result<const unsigned char*> cv_Mat_ptr_const_int_int_int(void* instance, int i0, int i1, int i2) {
		try {
			const unsigned char* ret = reinterpret_cast<cv::Mat*>(instance)->ptr(i0, i1, i2);
			return Ok<const unsigned char*>(ret);
		} OCVRS_CATCH(Result<const unsigned char*>)
	}
	
	Result<unsigned char*> cv_Mat_ptr_const_intX(void* instance, const int* idx) {
		try {
			unsigned char* ret = reinterpret_cast<cv::Mat*>(instance)->ptr(idx);
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(Result<unsigned char*>)
	}
	
	Result<const unsigned char*> cv_Mat_ptr_const_const_intX(void* instance, const int* idx) {
		try {
			const unsigned char* ret = reinterpret_cast<cv::Mat*>(instance)->ptr(idx);
			return Ok<const unsigned char*>(ret);
		} OCVRS_CATCH(Result<const unsigned char*>)
	}
	
	Result<void*> cv_Mat_Mat_MatX(void* m) {
		try {
			cv::Mat* ret = new cv::Mat(*reinterpret_cast<cv::Mat*>(m));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_Mat_updateContinuityFlag(void* instance) {
		try {
			reinterpret_cast<cv::Mat*>(instance)->updateContinuityFlag();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_MatConstIterator_m_const(void* instance) {
		try {
			const cv::Mat* ret = reinterpret_cast<cv::MatConstIterator*>(instance)->m;
			return Ok<void*>(new const cv::Mat*(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<size_t> cv_MatConstIterator_elemSize_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::MatConstIterator*>(instance)->elemSize;
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result_void cv_MatConstIterator_setElemSize_size_t(void* instance, size_t val) {
		try {
			reinterpret_cast<cv::MatConstIterator*>(instance)->elemSize = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<const unsigned char*> cv_MatConstIterator_ptr_const(void* instance) {
		try {
			const unsigned char* ret = reinterpret_cast<cv::MatConstIterator*>(instance)->ptr;
			return Ok<const unsigned char*>(ret);
		} OCVRS_CATCH(Result<const unsigned char*>)
	}
	
	Result<const unsigned char*> cv_MatConstIterator_sliceStart_const(void* instance) {
		try {
			const unsigned char* ret = reinterpret_cast<cv::MatConstIterator*>(instance)->sliceStart;
			return Ok<const unsigned char*>(ret);
		} OCVRS_CATCH(Result<const unsigned char*>)
	}
	
	Result<const unsigned char*> cv_MatConstIterator_sliceEnd_const(void* instance) {
		try {
			const unsigned char* ret = reinterpret_cast<cv::MatConstIterator*>(instance)->sliceEnd;
			return Ok<const unsigned char*>(ret);
		} OCVRS_CATCH(Result<const unsigned char*>)
	}
	
	void cv_MatConstIterator_delete(cv::MatConstIterator* instance) {
		delete instance;
	}
	Result<void*> cv_MatConstIterator_MatConstIterator() {
		try {
			cv::MatConstIterator* ret = new cv::MatConstIterator();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_MatConstIterator_MatConstIterator_const_MatX(void* _m) {
		try {
			cv::MatConstIterator* ret = new cv::MatConstIterator(reinterpret_cast<const cv::Mat*>(_m));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_MatConstIterator_MatConstIterator_const_MatX_int_int(void* _m, int _row, int _col) {
		try {
			cv::MatConstIterator* ret = new cv::MatConstIterator(reinterpret_cast<const cv::Mat*>(_m), _row, _col);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_MatConstIterator_MatConstIterator_const_MatX_Point(void* _m, cv::Point _pt) {
		try {
			cv::MatConstIterator* ret = new cv::MatConstIterator(reinterpret_cast<const cv::Mat*>(_m), _pt);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_MatConstIterator_MatConstIterator_const_MatX_const_intX(void* _m, const int* _idx) {
		try {
			cv::MatConstIterator* ret = new cv::MatConstIterator(reinterpret_cast<const cv::Mat*>(_m), _idx);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_MatConstIterator_MatConstIterator_const_MatConstIteratorX(void* it) {
		try {
			cv::MatConstIterator* ret = new cv::MatConstIterator(*reinterpret_cast<const cv::MatConstIterator*>(it));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<const unsigned char*> cv_MatConstIterator_operator___const_ptrdiff_t(void* instance, ptrdiff_t i) {
		try {
			const unsigned char* ret = reinterpret_cast<cv::MatConstIterator*>(instance)->operator[](i);
			return Ok<const unsigned char*>(ret);
		} OCVRS_CATCH(Result<const unsigned char*>)
	}
	
	Result<cv::Point> cv_MatConstIterator_pos_const(void* instance) {
		try {
			cv::Point ret = reinterpret_cast<cv::MatConstIterator*>(instance)->pos();
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(Result<cv::Point>)
	}
	
	Result_void cv_MatConstIterator_pos_const_intX(void* instance, int* _idx) {
		try {
			reinterpret_cast<cv::MatConstIterator*>(instance)->pos(_idx);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<ptrdiff_t> cv_MatConstIterator_lpos_const(void* instance) {
		try {
			ptrdiff_t ret = reinterpret_cast<cv::MatConstIterator*>(instance)->lpos();
			return Ok<ptrdiff_t>(ret);
		} OCVRS_CATCH(Result<ptrdiff_t>)
	}
	
	Result_void cv_MatConstIterator_seek_ptrdiff_t_bool(void* instance, ptrdiff_t ofs, bool relative) {
		try {
			reinterpret_cast<cv::MatConstIterator*>(instance)->seek(ofs, relative);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MatConstIterator_seek_const_intX_bool(void* instance, const int* _idx, bool relative) {
		try {
			reinterpret_cast<cv::MatConstIterator*>(instance)->seek(_idx, relative);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_MatExpr_flags_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::MatExpr*>(instance)->flags;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_MatExpr_setFlags_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::MatExpr*>(instance)->flags = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_MatExpr_a(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::MatExpr*>(instance)->a;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_MatExpr_setA_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::MatExpr*>(instance)->a = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_MatExpr_b(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::MatExpr*>(instance)->b;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_MatExpr_setB_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::MatExpr*>(instance)->b = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_MatExpr_c(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::MatExpr*>(instance)->c;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_MatExpr_setC_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::MatExpr*>(instance)->c = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_MatExpr_alpha_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::MatExpr*>(instance)->alpha;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_MatExpr_setAlpha_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::MatExpr*>(instance)->alpha = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_MatExpr_beta_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::MatExpr*>(instance)->beta;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_MatExpr_setBeta_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::MatExpr*>(instance)->beta = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Scalar> cv_MatExpr_s_const(void* instance) {
		try {
			cv::Scalar ret = reinterpret_cast<cv::MatExpr*>(instance)->s;
			return Ok<cv::Scalar>(ret);
		} OCVRS_CATCH(Result<cv::Scalar>)
	}
	
	Result_void cv_MatExpr_setS_Scalar(void* instance, cv::Scalar val) {
		try {
			reinterpret_cast<cv::MatExpr*>(instance)->s = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_MatExpr_delete(cv::MatExpr* instance) {
		delete instance;
	}
	Result<void*> cv_MatExpr_MatExpr() {
		try {
			cv::MatExpr* ret = new cv::MatExpr();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_MatExpr_MatExpr_const_MatX(void* m) {
		try {
			cv::MatExpr* ret = new cv::MatExpr(*reinterpret_cast<const cv::Mat*>(m));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_MatExpr_MatExpr_const_MatOpX_int_const_MatX_const_MatX_const_MatX_double_double_const_ScalarX(void* _op, int _flags, void* _a, void* _b, void* _c, double _alpha, double _beta, const cv::Scalar* _s) {
		try {
			cv::MatExpr* ret = new cv::MatExpr(reinterpret_cast<const cv::MatOp*>(_op), _flags, *reinterpret_cast<const cv::Mat*>(_a), *reinterpret_cast<const cv::Mat*>(_b), *reinterpret_cast<const cv::Mat*>(_c), _alpha, _beta, *_s);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_MatExpr_operator_cv_Mat_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::MatExpr*>(instance)->operator cv::Mat();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::Size> cv_MatExpr_size_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::MatExpr*>(instance)->size();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result<int> cv_MatExpr_type_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::MatExpr*>(instance)->type();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv_MatExpr_row_const_int(void* instance, int y) {
		try {
			cv::MatExpr ret = reinterpret_cast<cv::MatExpr*>(instance)->row(y);
			return Ok<void*>(new cv::MatExpr(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_MatExpr_col_const_int(void* instance, int x) {
		try {
			cv::MatExpr ret = reinterpret_cast<cv::MatExpr*>(instance)->col(x);
			return Ok<void*>(new cv::MatExpr(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_MatExpr_diag_const_int(void* instance, int d) {
		try {
			cv::MatExpr ret = reinterpret_cast<cv::MatExpr*>(instance)->diag(d);
			return Ok<void*>(new cv::MatExpr(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_MatExpr_t_const(void* instance) {
		try {
			cv::MatExpr ret = reinterpret_cast<cv::MatExpr*>(instance)->t();
			return Ok<void*>(new cv::MatExpr(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_MatExpr_inv_const_int(void* instance, int method) {
		try {
			cv::MatExpr ret = reinterpret_cast<cv::MatExpr*>(instance)->inv(method);
			return Ok<void*>(new cv::MatExpr(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_MatExpr_mul_const_const_MatExprX_double(void* instance, void* e, double scale) {
		try {
			cv::MatExpr ret = reinterpret_cast<cv::MatExpr*>(instance)->mul(*reinterpret_cast<const cv::MatExpr*>(e), scale);
			return Ok<void*>(new cv::MatExpr(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_MatExpr_mul_const_const_MatX_double(void* instance, void* m, double scale) {
		try {
			cv::MatExpr ret = reinterpret_cast<cv::MatExpr*>(instance)->mul(*reinterpret_cast<const cv::Mat*>(m), scale);
			return Ok<void*>(new cv::MatExpr(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_MatExpr_cross_const_const_MatX(void* instance, void* m) {
		try {
			cv::Mat ret = reinterpret_cast<cv::MatExpr*>(instance)->cross(*reinterpret_cast<const cv::Mat*>(m));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<double> cv_MatExpr_dot_const_const_MatX(void* instance, void* m) {
		try {
			double ret = reinterpret_cast<cv::MatExpr*>(instance)->dot(*reinterpret_cast<const cv::Mat*>(m));
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<bool> cv_MatOp_elementWise_const_const_MatExprX(void* instance, void* expr) {
		try {
			bool ret = reinterpret_cast<cv::MatOp*>(instance)->elementWise(*reinterpret_cast<const cv::MatExpr*>(expr));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_MatOp_assign_const_const_MatExprX_MatX_int(void* instance, void* expr, void* m, int type) {
		try {
			reinterpret_cast<cv::MatOp*>(instance)->assign(*reinterpret_cast<const cv::MatExpr*>(expr), *reinterpret_cast<cv::Mat*>(m), type);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MatOp_roi_const_const_MatExprX_const_RangeX_const_RangeX_MatExprX(void* instance, void* expr, void* rowRange, void* colRange, void* res) {
		try {
			reinterpret_cast<cv::MatOp*>(instance)->roi(*reinterpret_cast<const cv::MatExpr*>(expr), *reinterpret_cast<const cv::Range*>(rowRange), *reinterpret_cast<const cv::Range*>(colRange), *reinterpret_cast<cv::MatExpr*>(res));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MatOp_diag_const_const_MatExprX_int_MatExprX(void* instance, void* expr, int d, void* res) {
		try {
			reinterpret_cast<cv::MatOp*>(instance)->diag(*reinterpret_cast<const cv::MatExpr*>(expr), d, *reinterpret_cast<cv::MatExpr*>(res));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MatOp_augAssignAdd_const_const_MatExprX_MatX(void* instance, void* expr, void* m) {
		try {
			reinterpret_cast<cv::MatOp*>(instance)->augAssignAdd(*reinterpret_cast<const cv::MatExpr*>(expr), *reinterpret_cast<cv::Mat*>(m));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MatOp_augAssignSubtract_const_const_MatExprX_MatX(void* instance, void* expr, void* m) {
		try {
			reinterpret_cast<cv::MatOp*>(instance)->augAssignSubtract(*reinterpret_cast<const cv::MatExpr*>(expr), *reinterpret_cast<cv::Mat*>(m));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MatOp_augAssignMultiply_const_const_MatExprX_MatX(void* instance, void* expr, void* m) {
		try {
			reinterpret_cast<cv::MatOp*>(instance)->augAssignMultiply(*reinterpret_cast<const cv::MatExpr*>(expr), *reinterpret_cast<cv::Mat*>(m));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MatOp_augAssignDivide_const_const_MatExprX_MatX(void* instance, void* expr, void* m) {
		try {
			reinterpret_cast<cv::MatOp*>(instance)->augAssignDivide(*reinterpret_cast<const cv::MatExpr*>(expr), *reinterpret_cast<cv::Mat*>(m));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MatOp_augAssignAnd_const_const_MatExprX_MatX(void* instance, void* expr, void* m) {
		try {
			reinterpret_cast<cv::MatOp*>(instance)->augAssignAnd(*reinterpret_cast<const cv::MatExpr*>(expr), *reinterpret_cast<cv::Mat*>(m));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MatOp_augAssignOr_const_const_MatExprX_MatX(void* instance, void* expr, void* m) {
		try {
			reinterpret_cast<cv::MatOp*>(instance)->augAssignOr(*reinterpret_cast<const cv::MatExpr*>(expr), *reinterpret_cast<cv::Mat*>(m));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MatOp_augAssignXor_const_const_MatExprX_MatX(void* instance, void* expr, void* m) {
		try {
			reinterpret_cast<cv::MatOp*>(instance)->augAssignXor(*reinterpret_cast<const cv::MatExpr*>(expr), *reinterpret_cast<cv::Mat*>(m));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MatOp_add_const_const_MatExprX_const_MatExprX_MatExprX(void* instance, void* expr1, void* expr2, void* res) {
		try {
			reinterpret_cast<cv::MatOp*>(instance)->add(*reinterpret_cast<const cv::MatExpr*>(expr1), *reinterpret_cast<const cv::MatExpr*>(expr2), *reinterpret_cast<cv::MatExpr*>(res));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MatOp_add_const_const_MatExprX_const_ScalarX_MatExprX(void* instance, void* expr1, const cv::Scalar* s, void* res) {
		try {
			reinterpret_cast<cv::MatOp*>(instance)->add(*reinterpret_cast<const cv::MatExpr*>(expr1), *s, *reinterpret_cast<cv::MatExpr*>(res));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MatOp_subtract_const_const_MatExprX_const_MatExprX_MatExprX(void* instance, void* expr1, void* expr2, void* res) {
		try {
			reinterpret_cast<cv::MatOp*>(instance)->subtract(*reinterpret_cast<const cv::MatExpr*>(expr1), *reinterpret_cast<const cv::MatExpr*>(expr2), *reinterpret_cast<cv::MatExpr*>(res));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MatOp_subtract_const_const_ScalarX_const_MatExprX_MatExprX(void* instance, const cv::Scalar* s, void* expr, void* res) {
		try {
			reinterpret_cast<cv::MatOp*>(instance)->subtract(*s, *reinterpret_cast<const cv::MatExpr*>(expr), *reinterpret_cast<cv::MatExpr*>(res));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MatOp_multiply_const_const_MatExprX_const_MatExprX_MatExprX_double(void* instance, void* expr1, void* expr2, void* res, double scale) {
		try {
			reinterpret_cast<cv::MatOp*>(instance)->multiply(*reinterpret_cast<const cv::MatExpr*>(expr1), *reinterpret_cast<const cv::MatExpr*>(expr2), *reinterpret_cast<cv::MatExpr*>(res), scale);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MatOp_multiply_const_const_MatExprX_double_MatExprX(void* instance, void* expr1, double s, void* res) {
		try {
			reinterpret_cast<cv::MatOp*>(instance)->multiply(*reinterpret_cast<const cv::MatExpr*>(expr1), s, *reinterpret_cast<cv::MatExpr*>(res));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MatOp_divide_const_const_MatExprX_const_MatExprX_MatExprX_double(void* instance, void* expr1, void* expr2, void* res, double scale) {
		try {
			reinterpret_cast<cv::MatOp*>(instance)->divide(*reinterpret_cast<const cv::MatExpr*>(expr1), *reinterpret_cast<const cv::MatExpr*>(expr2), *reinterpret_cast<cv::MatExpr*>(res), scale);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MatOp_divide_const_double_const_MatExprX_MatExprX(void* instance, double s, void* expr, void* res) {
		try {
			reinterpret_cast<cv::MatOp*>(instance)->divide(s, *reinterpret_cast<const cv::MatExpr*>(expr), *reinterpret_cast<cv::MatExpr*>(res));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MatOp_abs_const_const_MatExprX_MatExprX(void* instance, void* expr, void* res) {
		try {
			reinterpret_cast<cv::MatOp*>(instance)->abs(*reinterpret_cast<const cv::MatExpr*>(expr), *reinterpret_cast<cv::MatExpr*>(res));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MatOp_transpose_const_const_MatExprX_MatExprX(void* instance, void* expr, void* res) {
		try {
			reinterpret_cast<cv::MatOp*>(instance)->transpose(*reinterpret_cast<const cv::MatExpr*>(expr), *reinterpret_cast<cv::MatExpr*>(res));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MatOp_matmul_const_const_MatExprX_const_MatExprX_MatExprX(void* instance, void* expr1, void* expr2, void* res) {
		try {
			reinterpret_cast<cv::MatOp*>(instance)->matmul(*reinterpret_cast<const cv::MatExpr*>(expr1), *reinterpret_cast<const cv::MatExpr*>(expr2), *reinterpret_cast<cv::MatExpr*>(res));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_MatOp_invert_const_const_MatExprX_int_MatExprX(void* instance, void* expr, int method, void* res) {
		try {
			reinterpret_cast<cv::MatOp*>(instance)->invert(*reinterpret_cast<const cv::MatExpr*>(expr), method, *reinterpret_cast<cv::MatExpr*>(res));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_MatOp_size_const_const_MatExprX(void* instance, void* expr) {
		try {
			cv::Size ret = reinterpret_cast<cv::MatOp*>(instance)->size(*reinterpret_cast<const cv::MatExpr*>(expr));
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result<int> cv_MatOp_type_const_const_MatExprX(void* instance, void* expr) {
		try {
			int ret = reinterpret_cast<cv::MatOp*>(instance)->type(*reinterpret_cast<const cv::MatExpr*>(expr));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int*> cv_MatSize_p(void* instance) {
		try {
			int* ret = reinterpret_cast<cv::MatSize*>(instance)->p;
			return Ok<int*>(ret);
		} OCVRS_CATCH(Result<int*>)
	}
	
	Result_void cv_MatSize_setP_intX(void* instance, int* val) {
		try {
			reinterpret_cast<cv::MatSize*>(instance)->p = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_MatSize_delete(cv::MatSize* instance) {
		delete instance;
	}
	Result<void*> cv_MatSize_MatSize_intX(int* _p) {
		try {
			cv::MatSize* ret = new cv::MatSize(_p);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_MatSize_dims_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::MatSize*>(instance)->dims();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_MatSize_operator___const_int(void* instance, int i) {
		try {
			int ret = reinterpret_cast<cv::MatSize*>(instance)->operator[](i);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_MatSize_operator___int(void* instance, int i) {
		try {
			int ret = reinterpret_cast<cv::MatSize*>(instance)->operator[](i);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<const int*> cv_MatSize_operator_const_intX_const(void* instance) {
		try {
			const int* ret = reinterpret_cast<cv::MatSize*>(instance)->operator const int*();
			return Ok<const int*>(ret);
		} OCVRS_CATCH(Result<const int*>)
	}
	
	Result<size_t*> cv_MatStep_p(void* instance) {
		try {
			size_t* ret = reinterpret_cast<cv::MatStep*>(instance)->p;
			return Ok<size_t*>(ret);
		} OCVRS_CATCH(Result<size_t*>)
	}
	
	Result_void cv_MatStep_setP_size_tX(void* instance, size_t* val) {
		try {
			reinterpret_cast<cv::MatStep*>(instance)->p = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<size_t(*)[2]> cv_MatStep_buf(void* instance) {
		try {
			size_t(*ret)[2] = &reinterpret_cast<cv::MatStep*>(instance)->buf;
			return Ok<size_t(*)[2]>(ret);
		} OCVRS_CATCH(Result<size_t(*)[2]>)
	}
	
	void cv_MatStep_delete(cv::MatStep* instance) {
		delete instance;
	}
	Result<void*> cv_MatStep_MatStep() {
		try {
			cv::MatStep* ret = new cv::MatStep();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_MatStep_MatStep_size_t(size_t s) {
		try {
			cv::MatStep* ret = new cv::MatStep(s);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<size_t> cv_MatStep_operator___const_int(void* instance, int i) {
		try {
			size_t ret = reinterpret_cast<cv::MatStep*>(instance)->operator[](i);
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<size_t> cv_MatStep_operator___int(void* instance, int i) {
		try {
			size_t ret = reinterpret_cast<cv::MatStep*>(instance)->operator[](i);
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<size_t> cv_MatStep_operator_size_t_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::MatStep*>(instance)->operator size_t();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	void cv_Matx_AddOp_delete(cv::Matx_AddOp* instance) {
		delete instance;
	}
	Result<void*> cv_Matx_AddOp_Matx_AddOp() {
		try {
			cv::Matx_AddOp* ret = new cv::Matx_AddOp();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Matx_AddOp_Matx_AddOp_const_Matx_AddOpX(void* unnamed) {
		try {
			cv::Matx_AddOp* ret = new cv::Matx_AddOp(*reinterpret_cast<const cv::Matx_AddOp*>(unnamed));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Matx_DivOp_delete(cv::Matx_DivOp* instance) {
		delete instance;
	}
	Result<void*> cv_Matx_DivOp_Matx_DivOp() {
		try {
			cv::Matx_DivOp* ret = new cv::Matx_DivOp();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Matx_DivOp_Matx_DivOp_const_Matx_DivOpX(void* unnamed) {
		try {
			cv::Matx_DivOp* ret = new cv::Matx_DivOp(*reinterpret_cast<const cv::Matx_DivOp*>(unnamed));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Matx_MatMulOp_delete(cv::Matx_MatMulOp* instance) {
		delete instance;
	}
	Result<void*> cv_Matx_MatMulOp_Matx_MatMulOp() {
		try {
			cv::Matx_MatMulOp* ret = new cv::Matx_MatMulOp();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Matx_MatMulOp_Matx_MatMulOp_const_Matx_MatMulOpX(void* unnamed) {
		try {
			cv::Matx_MatMulOp* ret = new cv::Matx_MatMulOp(*reinterpret_cast<const cv::Matx_MatMulOp*>(unnamed));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Matx_MulOp_delete(cv::Matx_MulOp* instance) {
		delete instance;
	}
	Result<void*> cv_Matx_MulOp_Matx_MulOp() {
		try {
			cv::Matx_MulOp* ret = new cv::Matx_MulOp();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Matx_MulOp_Matx_MulOp_const_Matx_MulOpX(void* unnamed) {
		try {
			cv::Matx_MulOp* ret = new cv::Matx_MulOp(*reinterpret_cast<const cv::Matx_MulOp*>(unnamed));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Matx_ScaleOp_delete(cv::Matx_ScaleOp* instance) {
		delete instance;
	}
	Result<void*> cv_Matx_ScaleOp_Matx_ScaleOp() {
		try {
			cv::Matx_ScaleOp* ret = new cv::Matx_ScaleOp();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Matx_ScaleOp_Matx_ScaleOp_const_Matx_ScaleOpX(void* unnamed) {
		try {
			cv::Matx_ScaleOp* ret = new cv::Matx_ScaleOp(*reinterpret_cast<const cv::Matx_ScaleOp*>(unnamed));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Matx_SubOp_delete(cv::Matx_SubOp* instance) {
		delete instance;
	}
	Result<void*> cv_Matx_SubOp_Matx_SubOp() {
		try {
			cv::Matx_SubOp* ret = new cv::Matx_SubOp();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Matx_SubOp_Matx_SubOp_const_Matx_SubOpX(void* unnamed) {
		try {
			cv::Matx_SubOp* ret = new cv::Matx_SubOp(*reinterpret_cast<const cv::Matx_SubOp*>(unnamed));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Matx_TOp_delete(cv::Matx_TOp* instance) {
		delete instance;
	}
	Result<void*> cv_Matx_TOp_Matx_TOp() {
		try {
			cv::Matx_TOp* ret = new cv::Matx_TOp();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Matx_TOp_Matx_TOp_const_Matx_TOpX(void* unnamed) {
		try {
			cv::Matx_TOp* ret = new cv::Matx_TOp(*reinterpret_cast<const cv::Matx_TOp*>(unnamed));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_MinProblemSolver_getFunction_const(void* instance) {
		try {
			cv::Ptr<cv::MinProblemSolver::Function> ret = reinterpret_cast<cv::MinProblemSolver*>(instance)->getFunction();
			return Ok<void*>(new cv::Ptr<cv::MinProblemSolver::Function>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_MinProblemSolver_setFunction_const_Ptr_Function_X(void* instance, void* f) {
		try {
			reinterpret_cast<cv::MinProblemSolver*>(instance)->setFunction(*reinterpret_cast<const cv::Ptr<cv::MinProblemSolver::Function>*>(f));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_MinProblemSolver_getTermCriteria_const(void* instance) {
		try {
			cv::TermCriteria ret = reinterpret_cast<cv::MinProblemSolver*>(instance)->getTermCriteria();
			return Ok<void*>(new cv::TermCriteria(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_MinProblemSolver_setTermCriteria_const_TermCriteriaX(void* instance, void* termcrit) {
		try {
			reinterpret_cast<cv::MinProblemSolver*>(instance)->setTermCriteria(*reinterpret_cast<const cv::TermCriteria*>(termcrit));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_MinProblemSolver_minimize_const__InputOutputArrayX(void* instance, void* x) {
		try {
			double ret = reinterpret_cast<cv::MinProblemSolver*>(instance)->minimize(*reinterpret_cast<const cv::_InputOutputArray*>(x));
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<int> cv_MinProblemSolver_Function_getDims_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::MinProblemSolver::Function*>(instance)->getDims();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<double> cv_MinProblemSolver_Function_getGradientEps_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::MinProblemSolver::Function*>(instance)->getGradientEps();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<double> cv_MinProblemSolver_Function_calc_const_const_doubleX(void* instance, const double* x) {
		try {
			double ret = reinterpret_cast<cv::MinProblemSolver::Function*>(instance)->calc(x);
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_MinProblemSolver_Function_getGradient_const_doubleX_doubleX(void* instance, const double* x, double* grad) {
		try {
			reinterpret_cast<cv::MinProblemSolver::Function*>(instance)->getGradient(x, grad);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Moments> cv_Moments_Moments() {
		try {
			cv::Moments ret;
			return Ok<cv::Moments>(ret);
		} OCVRS_CATCH(Result<cv::Moments>)
	}
	
	Result<cv::Moments> cv_Moments_Moments_double_double_double_double_double_double_double_double_double_double(double m00, double m10, double m01, double m20, double m11, double m02, double m30, double m21, double m12, double m03) {
		try {
			cv::Moments ret(m00, m10, m01, m20, m11, m02, m30, m21, m12, m03);
			return Ok<cv::Moments>(ret);
		} OCVRS_CATCH(Result<cv::Moments>)
	}
	
	Result<void*> cv_PCA_eigenvectors(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::PCA*>(instance)->eigenvectors;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_PCA_setEigenvectors_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::PCA*>(instance)->eigenvectors = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_PCA_eigenvalues(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::PCA*>(instance)->eigenvalues;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_PCA_setEigenvalues_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::PCA*>(instance)->eigenvalues = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_PCA_mean(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::PCA*>(instance)->mean;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_PCA_setMean_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::PCA*>(instance)->mean = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_PCA_delete(cv::PCA* instance) {
		delete instance;
	}
	Result<void*> cv_PCA_PCA() {
		try {
			cv::PCA* ret = new cv::PCA();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_PCA_PCA_const__InputArrayX_const__InputArrayX_int_int(void* data, void* mean, int flags, int maxComponents) {
		try {
			cv::PCA* ret = new cv::PCA(*reinterpret_cast<const cv::_InputArray*>(data), *reinterpret_cast<const cv::_InputArray*>(mean), flags, maxComponents);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_PCA_PCA_const__InputArrayX_const__InputArrayX_int_double(void* data, void* mean, int flags, double retainedVariance) {
		try {
			cv::PCA* ret = new cv::PCA(*reinterpret_cast<const cv::_InputArray*>(data), *reinterpret_cast<const cv::_InputArray*>(mean), flags, retainedVariance);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_PCA_project_const_const__InputArrayX(void* instance, void* vec) {
		try {
			cv::Mat ret = reinterpret_cast<cv::PCA*>(instance)->project(*reinterpret_cast<const cv::_InputArray*>(vec));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_PCA_project_const_const__InputArrayX_const__OutputArrayX(void* instance, void* vec, void* result) {
		try {
			reinterpret_cast<cv::PCA*>(instance)->project(*reinterpret_cast<const cv::_InputArray*>(vec), *reinterpret_cast<const cv::_OutputArray*>(result));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_PCA_backProject_const_const__InputArrayX(void* instance, void* vec) {
		try {
			cv::Mat ret = reinterpret_cast<cv::PCA*>(instance)->backProject(*reinterpret_cast<const cv::_InputArray*>(vec));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_PCA_backProject_const_const__InputArrayX_const__OutputArrayX(void* instance, void* vec, void* result) {
		try {
			reinterpret_cast<cv::PCA*>(instance)->backProject(*reinterpret_cast<const cv::_InputArray*>(vec), *reinterpret_cast<const cv::_OutputArray*>(result));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_PCA_write_const_FileStorageX(void* instance, void* fs) {
		try {
			reinterpret_cast<cv::PCA*>(instance)->write(*reinterpret_cast<cv::FileStorage*>(fs));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_PCA_read_const_FileNodeX(void* instance, void* fn) {
		try {
			reinterpret_cast<cv::PCA*>(instance)->read(*reinterpret_cast<const cv::FileNode*>(fn));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<uint64_t> cv_RNG_state_const(void* instance) {
		try {
			uint64_t ret = reinterpret_cast<cv::RNG*>(instance)->state;
			return Ok<uint64_t>(ret);
		} OCVRS_CATCH(Result<uint64_t>)
	}
	
	Result_void cv_RNG_setState_uint64_t(void* instance, uint64_t val) {
		try {
			reinterpret_cast<cv::RNG*>(instance)->state = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_RNG_delete(cv::RNG* instance) {
		delete instance;
	}
	Result<void*> cv_RNG_RNG() {
		try {
			cv::RNG* ret = new cv::RNG();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_RNG_RNG_uint64_t(uint64_t state) {
		try {
			cv::RNG* ret = new cv::RNG(state);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<unsigned int> cv_RNG_next(void* instance) {
		try {
			unsigned int ret = reinterpret_cast<cv::RNG*>(instance)->next();
			return Ok<unsigned int>(ret);
		} OCVRS_CATCH(Result<unsigned int>)
	}
	
	Result<unsigned char> cv_RNG_operator_unsigned_char(void* instance) {
		try {
			unsigned char ret = reinterpret_cast<cv::RNG*>(instance)->operator unsigned char();
			return Ok<unsigned char>(ret);
		} OCVRS_CATCH(Result<unsigned char>)
	}
	
	Result<signed char> cv_RNG_operator_signed_char(void* instance) {
		try {
			signed char ret = reinterpret_cast<cv::RNG*>(instance)->operator signed char();
			return Ok<signed char>(ret);
		} OCVRS_CATCH(Result<signed char>)
	}
	
	Result<unsigned short> cv_RNG_operator_unsigned_short(void* instance) {
		try {
			unsigned short ret = reinterpret_cast<cv::RNG*>(instance)->operator unsigned short();
			return Ok<unsigned short>(ret);
		} OCVRS_CATCH(Result<unsigned short>)
	}
	
	Result<short> cv_RNG_operator_short(void* instance) {
		try {
			short ret = reinterpret_cast<cv::RNG*>(instance)->operator short();
			return Ok<short>(ret);
		} OCVRS_CATCH(Result<short>)
	}
	
	Result<unsigned int> cv_RNG_operator_unsigned_int(void* instance) {
		try {
			unsigned int ret = reinterpret_cast<cv::RNG*>(instance)->operator unsigned int();
			return Ok<unsigned int>(ret);
		} OCVRS_CATCH(Result<unsigned int>)
	}
	
	Result<int> cv_RNG_operator_int(void* instance) {
		try {
			int ret = reinterpret_cast<cv::RNG*>(instance)->operator int();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<float> cv_RNG_operator_float(void* instance) {
		try {
			float ret = reinterpret_cast<cv::RNG*>(instance)->operator float();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<double> cv_RNG_operator_double(void* instance) {
		try {
			double ret = reinterpret_cast<cv::RNG*>(instance)->operator double();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<int> cv_RNG_uniform_int_int(void* instance, int a, int b) {
		try {
			int ret = reinterpret_cast<cv::RNG*>(instance)->uniform(a, b);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<float> cv_RNG_uniform_float_float(void* instance, float a, float b) {
		try {
			float ret = reinterpret_cast<cv::RNG*>(instance)->uniform(a, b);
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<double> cv_RNG_uniform_double_double(void* instance, double a, double b) {
		try {
			double ret = reinterpret_cast<cv::RNG*>(instance)->uniform(a, b);
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_RNG_fill_const__InputOutputArrayX_int_const__InputArrayX_const__InputArrayX_bool(void* instance, void* mat, int distType, void* a, void* b, bool saturateRange) {
		try {
			reinterpret_cast<cv::RNG*>(instance)->fill(*reinterpret_cast<const cv::_InputOutputArray*>(mat), distType, *reinterpret_cast<const cv::_InputArray*>(a), *reinterpret_cast<const cv::_InputArray*>(b), saturateRange);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_RNG_gaussian_double(void* instance, double sigma) {
		try {
			double ret = reinterpret_cast<cv::RNG*>(instance)->gaussian(sigma);
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	void cv_RNG_MT19937_delete(cv::RNG_MT19937* instance) {
		delete instance;
	}
	Result<void*> cv_RNG_MT19937_RNG_MT19937() {
		try {
			cv::RNG_MT19937* ret = new cv::RNG_MT19937();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_RNG_MT19937_RNG_MT19937_unsigned_int(unsigned int s) {
		try {
			cv::RNG_MT19937* ret = new cv::RNG_MT19937(s);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_RNG_MT19937_seed_unsigned_int(void* instance, unsigned int s) {
		try {
			reinterpret_cast<cv::RNG_MT19937*>(instance)->seed(s);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<unsigned int> cv_RNG_MT19937_next(void* instance) {
		try {
			unsigned int ret = reinterpret_cast<cv::RNG_MT19937*>(instance)->next();
			return Ok<unsigned int>(ret);
		} OCVRS_CATCH(Result<unsigned int>)
	}
	
	Result<int> cv_RNG_MT19937_operator_int(void* instance) {
		try {
			int ret = reinterpret_cast<cv::RNG_MT19937*>(instance)->operator int();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<unsigned int> cv_RNG_MT19937_operator_unsigned_int(void* instance) {
		try {
			unsigned int ret = reinterpret_cast<cv::RNG_MT19937*>(instance)->operator unsigned int();
			return Ok<unsigned int>(ret);
		} OCVRS_CATCH(Result<unsigned int>)
	}
	
	Result<float> cv_RNG_MT19937_operator_float(void* instance) {
		try {
			float ret = reinterpret_cast<cv::RNG_MT19937*>(instance)->operator float();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<double> cv_RNG_MT19937_operator_double(void* instance) {
		try {
			double ret = reinterpret_cast<cv::RNG_MT19937*>(instance)->operator double();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<int> cv_RNG_MT19937_uniform_int_int(void* instance, int a, int b) {
		try {
			int ret = reinterpret_cast<cv::RNG_MT19937*>(instance)->uniform(a, b);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<float> cv_RNG_MT19937_uniform_float_float(void* instance, float a, float b) {
		try {
			float ret = reinterpret_cast<cv::RNG_MT19937*>(instance)->uniform(a, b);
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<double> cv_RNG_MT19937_uniform_double_double(void* instance, double a, double b) {
		try {
			double ret = reinterpret_cast<cv::RNG_MT19937*>(instance)->uniform(a, b);
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<int> cv_Range_start_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::Range*>(instance)->start;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_Range_setStart_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::Range*>(instance)->start = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_Range_end_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::Range*>(instance)->end;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_Range_setEnd_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::Range*>(instance)->end = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Range_delete(cv::Range* instance) {
		delete instance;
	}
	Result<void*> cv_Range_Range() {
		try {
			cv::Range* ret = new cv::Range();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Range_Range_int_int(int _start, int _end) {
		try {
			cv::Range* ret = new cv::Range(_start, _end);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_Range_size_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::Range*>(instance)->size();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<bool> cv_Range_empty_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::Range*>(instance)->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_Range_all() {
		try {
			cv::Range ret = cv::Range::all();
			return Ok<void*>(new cv::Range(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::Point2f> cv_RotatedRect_center_const(void* instance) {
		try {
			cv::Point2f ret = reinterpret_cast<cv::RotatedRect*>(instance)->center;
			return Ok<cv::Point2f>(ret);
		} OCVRS_CATCH(Result<cv::Point2f>)
	}
	
	Result_void cv_RotatedRect_setCenter_Point2f(void* instance, cv::Point2f val) {
		try {
			reinterpret_cast<cv::RotatedRect*>(instance)->center = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size2f> cv_RotatedRect_size_const(void* instance) {
		try {
			cv::Size2f ret = reinterpret_cast<cv::RotatedRect*>(instance)->size;
			return Ok<cv::Size2f>(ret);
		} OCVRS_CATCH(Result<cv::Size2f>)
	}
	
	Result_void cv_RotatedRect_setSize_Size2f(void* instance, cv::Size2f val) {
		try {
			reinterpret_cast<cv::RotatedRect*>(instance)->size = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_RotatedRect_angle_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::RotatedRect*>(instance)->angle;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_RotatedRect_setAngle_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::RotatedRect*>(instance)->angle = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_RotatedRect_delete(cv::RotatedRect* instance) {
		delete instance;
	}
	Result<void*> cv_RotatedRect_RotatedRect() {
		try {
			cv::RotatedRect* ret = new cv::RotatedRect();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_RotatedRect_RotatedRect_const_Point2fX_const_Size2fX_float(const cv::Point2f* center, const cv::Size2f* size, float angle) {
		try {
			cv::RotatedRect* ret = new cv::RotatedRect(*center, *size, angle);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_RotatedRect_RotatedRect_const_Point2fX_const_Point2fX_const_Point2fX(const cv::Point2f* point1, const cv::Point2f* point2, const cv::Point2f* point3) {
		try {
			cv::RotatedRect* ret = new cv::RotatedRect(*point1, *point2, *point3);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_RotatedRect_points_const_Point2fX(void* instance, cv::Point2f* pts) {
		try {
			reinterpret_cast<cv::RotatedRect*>(instance)->points(pts);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Rect> cv_RotatedRect_boundingRect_const(void* instance) {
		try {
			cv::Rect ret = reinterpret_cast<cv::RotatedRect*>(instance)->boundingRect();
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<cv::Rect_<float>> cv_RotatedRect_boundingRect2f_const(void* instance) {
		try {
			cv::Rect_<float> ret = reinterpret_cast<cv::RotatedRect*>(instance)->boundingRect2f();
			return Ok<cv::Rect_<float>>(ret);
		} OCVRS_CATCH(Result<cv::Rect_<float>>)
	}
	
	Result<void*> cv_SVD_u(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::SVD*>(instance)->u;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_SVD_setU_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::SVD*>(instance)->u = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_SVD_w(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::SVD*>(instance)->w;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_SVD_setW_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::SVD*>(instance)->w = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_SVD_vt(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::SVD*>(instance)->vt;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_SVD_setVt_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::SVD*>(instance)->vt = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_SVD_delete(cv::SVD* instance) {
		delete instance;
	}
	Result<void*> cv_SVD_SVD() {
		try {
			cv::SVD* ret = new cv::SVD();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_SVD_SVD_const__InputArrayX_int(void* src, int flags) {
		try {
			cv::SVD* ret = new cv::SVD(*reinterpret_cast<const cv::_InputArray*>(src), flags);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_SVD_compute_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_int(void* src, void* w, void* u, void* vt, int flags) {
		try {
			cv::SVD::compute(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(w), *reinterpret_cast<const cv::_OutputArray*>(u), *reinterpret_cast<const cv::_OutputArray*>(vt), flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_SVD_compute_const__InputArrayX_const__OutputArrayX_int(void* src, void* w, int flags) {
		try {
			cv::SVD::compute(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(w), flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_SVD_backSubst_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* w, void* u, void* vt, void* rhs, void* dst) {
		try {
			cv::SVD::backSubst(*reinterpret_cast<const cv::_InputArray*>(w), *reinterpret_cast<const cv::_InputArray*>(u), *reinterpret_cast<const cv::_InputArray*>(vt), *reinterpret_cast<const cv::_InputArray*>(rhs), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_SVD_solveZ_const__InputArrayX_const__OutputArrayX(void* src, void* dst) {
		try {
			cv::SVD::solveZ(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_SVD_backSubst_const_const__InputArrayX_const__OutputArrayX(void* instance, void* rhs, void* dst) {
		try {
			reinterpret_cast<cv::SVD*>(instance)->backSubst(*reinterpret_cast<const cv::_InputArray*>(rhs), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_SparseMat_flags_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::SparseMat*>(instance)->flags;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_SparseMat_setFlags_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::SparseMat*>(instance)->flags = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_SparseMat_hdr(void* instance) {
		try {
			cv::SparseMat::Hdr* ret = reinterpret_cast<cv::SparseMat*>(instance)->hdr;
			return Ok<void*>(new cv::SparseMat::Hdr*(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_SparseMat_setHdr_HdrX(void* instance, void* val) {
		try {
			reinterpret_cast<cv::SparseMat*>(instance)->hdr = reinterpret_cast<cv::SparseMat::Hdr*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_SparseMat_delete(cv::SparseMat* instance) {
		delete instance;
	}
	Result<void*> cv_SparseMat_SparseMat() {
		try {
			cv::SparseMat* ret = new cv::SparseMat();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_SparseMat_SparseMat_int_const_intX_int(int dims, const int* _sizes, int _type) {
		try {
			cv::SparseMat* ret = new cv::SparseMat(dims, _sizes, _type);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_SparseMat_SparseMat_const_SparseMatX(void* m) {
		try {
			cv::SparseMat* ret = new cv::SparseMat(*reinterpret_cast<const cv::SparseMat*>(m));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_SparseMat_SparseMat_const_MatX(void* m) {
		try {
			cv::SparseMat* ret = new cv::SparseMat(*reinterpret_cast<const cv::Mat*>(m));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_SparseMat_clone_const(void* instance) {
		try {
			cv::SparseMat ret = reinterpret_cast<cv::SparseMat*>(instance)->clone();
			return Ok<void*>(new cv::SparseMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_SparseMat_copyTo_const_SparseMatX(void* instance, void* m) {
		try {
			reinterpret_cast<cv::SparseMat*>(instance)->copyTo(*reinterpret_cast<cv::SparseMat*>(m));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_SparseMat_copyTo_const_MatX(void* instance, void* m) {
		try {
			reinterpret_cast<cv::SparseMat*>(instance)->copyTo(*reinterpret_cast<cv::Mat*>(m));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_SparseMat_convertTo_const_SparseMatX_int_double(void* instance, void* m, int rtype, double alpha) {
		try {
			reinterpret_cast<cv::SparseMat*>(instance)->convertTo(*reinterpret_cast<cv::SparseMat*>(m), rtype, alpha);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_SparseMat_convertTo_const_MatX_int_double_double(void* instance, void* m, int rtype, double alpha, double beta) {
		try {
			reinterpret_cast<cv::SparseMat*>(instance)->convertTo(*reinterpret_cast<cv::Mat*>(m), rtype, alpha, beta);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_SparseMat_assignTo_const_SparseMatX_int(void* instance, void* m, int type) {
		try {
			reinterpret_cast<cv::SparseMat*>(instance)->assignTo(*reinterpret_cast<cv::SparseMat*>(m), type);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_SparseMat_create_int_const_intX_int(void* instance, int dims, const int* _sizes, int _type) {
		try {
			reinterpret_cast<cv::SparseMat*>(instance)->create(dims, _sizes, _type);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_SparseMat_clear(void* instance) {
		try {
			reinterpret_cast<cv::SparseMat*>(instance)->clear();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_SparseMat_addref(void* instance) {
		try {
			reinterpret_cast<cv::SparseMat*>(instance)->addref();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_SparseMat_release(void* instance) {
		try {
			reinterpret_cast<cv::SparseMat*>(instance)->release();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<size_t> cv_SparseMat_elemSize_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::SparseMat*>(instance)->elemSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<size_t> cv_SparseMat_elemSize1_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::SparseMat*>(instance)->elemSize1();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<int> cv_SparseMat_type_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::SparseMat*>(instance)->type();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_SparseMat_depth_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::SparseMat*>(instance)->depth();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_SparseMat_channels_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::SparseMat*>(instance)->channels();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<const int*> cv_SparseMat_size_const(void* instance) {
		try {
			const int* ret = reinterpret_cast<cv::SparseMat*>(instance)->size();
			return Ok<const int*>(ret);
		} OCVRS_CATCH(Result<const int*>)
	}
	
	Result<int> cv_SparseMat_size_const_int(void* instance, int i) {
		try {
			int ret = reinterpret_cast<cv::SparseMat*>(instance)->size(i);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_SparseMat_dims_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::SparseMat*>(instance)->dims();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<size_t> cv_SparseMat_nzcount_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::SparseMat*>(instance)->nzcount();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<size_t> cv_SparseMat_hash_const_int(void* instance, int i0) {
		try {
			size_t ret = reinterpret_cast<cv::SparseMat*>(instance)->hash(i0);
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<size_t> cv_SparseMat_hash_const_int_int(void* instance, int i0, int i1) {
		try {
			size_t ret = reinterpret_cast<cv::SparseMat*>(instance)->hash(i0, i1);
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<size_t> cv_SparseMat_hash_const_int_int_int(void* instance, int i0, int i1, int i2) {
		try {
			size_t ret = reinterpret_cast<cv::SparseMat*>(instance)->hash(i0, i1, i2);
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<size_t> cv_SparseMat_hash_const_const_intX(void* instance, const int* idx) {
		try {
			size_t ret = reinterpret_cast<cv::SparseMat*>(instance)->hash(idx);
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<unsigned char*> cv_SparseMat_ptr_int_bool_size_tX(void* instance, int i0, bool createMissing, size_t* hashval) {
		try {
			unsigned char* ret = reinterpret_cast<cv::SparseMat*>(instance)->ptr(i0, createMissing, hashval);
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(Result<unsigned char*>)
	}
	
	Result<unsigned char*> cv_SparseMat_ptr_int_int_bool_size_tX(void* instance, int i0, int i1, bool createMissing, size_t* hashval) {
		try {
			unsigned char* ret = reinterpret_cast<cv::SparseMat*>(instance)->ptr(i0, i1, createMissing, hashval);
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(Result<unsigned char*>)
	}
	
	Result<unsigned char*> cv_SparseMat_ptr_int_int_int_bool_size_tX(void* instance, int i0, int i1, int i2, bool createMissing, size_t* hashval) {
		try {
			unsigned char* ret = reinterpret_cast<cv::SparseMat*>(instance)->ptr(i0, i1, i2, createMissing, hashval);
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(Result<unsigned char*>)
	}
	
	Result<unsigned char*> cv_SparseMat_ptr_const_intX_bool_size_tX(void* instance, const int* idx, bool createMissing, size_t* hashval) {
		try {
			unsigned char* ret = reinterpret_cast<cv::SparseMat*>(instance)->ptr(idx, createMissing, hashval);
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(Result<unsigned char*>)
	}
	
	Result_void cv_SparseMat_erase_int_int_size_tX(void* instance, int i0, int i1, size_t* hashval) {
		try {
			reinterpret_cast<cv::SparseMat*>(instance)->erase(i0, i1, hashval);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_SparseMat_erase_int_int_int_size_tX(void* instance, int i0, int i1, int i2, size_t* hashval) {
		try {
			reinterpret_cast<cv::SparseMat*>(instance)->erase(i0, i1, i2, hashval);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_SparseMat_erase_const_intX_size_tX(void* instance, const int* idx, size_t* hashval) {
		try {
			reinterpret_cast<cv::SparseMat*>(instance)->erase(idx, hashval);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_SparseMat_begin(void* instance) {
		try {
			cv::SparseMatIterator ret = reinterpret_cast<cv::SparseMat*>(instance)->begin();
			return Ok<void*>(new cv::SparseMatIterator(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_SparseMat_begin_const(void* instance) {
		try {
			cv::SparseMatConstIterator ret = reinterpret_cast<cv::SparseMat*>(instance)->begin();
			return Ok<void*>(new cv::SparseMatConstIterator(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_SparseMat_end(void* instance) {
		try {
			cv::SparseMatIterator ret = reinterpret_cast<cv::SparseMat*>(instance)->end();
			return Ok<void*>(new cv::SparseMatIterator(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_SparseMat_end_const(void* instance) {
		try {
			cv::SparseMatConstIterator ret = reinterpret_cast<cv::SparseMat*>(instance)->end();
			return Ok<void*>(new cv::SparseMatConstIterator(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_SparseMat_node_size_t(void* instance, size_t nidx) {
		try {
			cv::SparseMat::Node* ret = reinterpret_cast<cv::SparseMat*>(instance)->node(nidx);
			return Ok<void*>(new cv::SparseMat::Node*(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_SparseMat_node_const_size_t(void* instance, size_t nidx) {
		try {
			const cv::SparseMat::Node* ret = reinterpret_cast<cv::SparseMat*>(instance)->node(nidx);
			return Ok<void*>(new const cv::SparseMat::Node*(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<unsigned char*> cv_SparseMat_newNode_const_intX_size_t(void* instance, const int* idx, size_t hashval) {
		try {
			unsigned char* ret = reinterpret_cast<cv::SparseMat*>(instance)->newNode(idx, hashval);
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(Result<unsigned char*>)
	}
	
	Result_void cv_SparseMat_removeNode_size_t_size_t_size_t(void* instance, size_t hidx, size_t nidx, size_t previdx) {
		try {
			reinterpret_cast<cv::SparseMat*>(instance)->removeNode(hidx, nidx, previdx);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_SparseMat_resizeHashTab_size_t(void* instance, size_t newsize) {
		try {
			reinterpret_cast<cv::SparseMat*>(instance)->resizeHashTab(newsize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_SparseMat_Hdr_refcount_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::SparseMat::Hdr*>(instance)->refcount;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_SparseMat_Hdr_setRefcount_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::SparseMat::Hdr*>(instance)->refcount = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_SparseMat_Hdr_dims_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::SparseMat::Hdr*>(instance)->dims;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_SparseMat_Hdr_setDims_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::SparseMat::Hdr*>(instance)->dims = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_SparseMat_Hdr_valueOffset_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::SparseMat::Hdr*>(instance)->valueOffset;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_SparseMat_Hdr_setValueOffset_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::SparseMat::Hdr*>(instance)->valueOffset = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<size_t> cv_SparseMat_Hdr_nodeSize_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::SparseMat::Hdr*>(instance)->nodeSize;
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result_void cv_SparseMat_Hdr_setNodeSize_size_t(void* instance, size_t val) {
		try {
			reinterpret_cast<cv::SparseMat::Hdr*>(instance)->nodeSize = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<size_t> cv_SparseMat_Hdr_nodeCount_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::SparseMat::Hdr*>(instance)->nodeCount;
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result_void cv_SparseMat_Hdr_setNodeCount_size_t(void* instance, size_t val) {
		try {
			reinterpret_cast<cv::SparseMat::Hdr*>(instance)->nodeCount = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<size_t> cv_SparseMat_Hdr_freeList_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::SparseMat::Hdr*>(instance)->freeList;
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result_void cv_SparseMat_Hdr_setFreeList_size_t(void* instance, size_t val) {
		try {
			reinterpret_cast<cv::SparseMat::Hdr*>(instance)->freeList = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_SparseMat_Hdr_pool(void* instance) {
		try {
			std::vector<unsigned char> ret = reinterpret_cast<cv::SparseMat::Hdr*>(instance)->pool;
			return Ok<void*>(new std::vector<unsigned char>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_SparseMat_Hdr_setPool_vector_unsigned_char_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::SparseMat::Hdr*>(instance)->pool = *reinterpret_cast<std::vector<unsigned char>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_SparseMat_Hdr_hashtab(void* instance) {
		try {
			std::vector<size_t> ret = reinterpret_cast<cv::SparseMat::Hdr*>(instance)->hashtab;
			return Ok<void*>(new std::vector<size_t>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_SparseMat_Hdr_setHashtab_vector_size_t_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::SparseMat::Hdr*>(instance)->hashtab = *reinterpret_cast<std::vector<size_t>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int(*)[32]> cv_SparseMat_Hdr_size(void* instance) {
		try {
			int(*ret)[32] = &reinterpret_cast<cv::SparseMat::Hdr*>(instance)->size;
			return Ok<int(*)[32]>(ret);
		} OCVRS_CATCH(Result<int(*)[32]>)
	}
	
	void cv_SparseMat_Hdr_delete(cv::SparseMat::Hdr* instance) {
		delete instance;
	}
	Result<void*> cv_SparseMat_Hdr_Hdr_int_const_intX_int(int _dims, const int* _sizes, int _type) {
		try {
			cv::SparseMat::Hdr* ret = new cv::SparseMat::Hdr(_dims, _sizes, _type);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_SparseMat_Hdr_clear(void* instance) {
		try {
			reinterpret_cast<cv::SparseMat::Hdr*>(instance)->clear();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<size_t> cv_SparseMat_Node_hashval_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::SparseMat::Node*>(instance)->hashval;
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result_void cv_SparseMat_Node_setHashval_size_t(void* instance, size_t val) {
		try {
			reinterpret_cast<cv::SparseMat::Node*>(instance)->hashval = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<size_t> cv_SparseMat_Node_next_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::SparseMat::Node*>(instance)->next;
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result_void cv_SparseMat_Node_setNext_size_t(void* instance, size_t val) {
		try {
			reinterpret_cast<cv::SparseMat::Node*>(instance)->next = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int(*)[32]> cv_SparseMat_Node_idx(void* instance) {
		try {
			int(*ret)[32] = &reinterpret_cast<cv::SparseMat::Node*>(instance)->idx;
			return Ok<int(*)[32]>(ret);
		} OCVRS_CATCH(Result<int(*)[32]>)
	}
	
	void cv_SparseMat_Node_delete(cv::SparseMat::Node* instance) {
		delete instance;
	}
	Result<void*> cv_SparseMatConstIterator_m_const(void* instance) {
		try {
			const cv::SparseMat* ret = reinterpret_cast<cv::SparseMatConstIterator*>(instance)->m;
			return Ok<void*>(new const cv::SparseMat*(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<size_t> cv_SparseMatConstIterator_hashidx_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::SparseMatConstIterator*>(instance)->hashidx;
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result_void cv_SparseMatConstIterator_setHashidx_size_t(void* instance, size_t val) {
		try {
			reinterpret_cast<cv::SparseMatConstIterator*>(instance)->hashidx = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<unsigned char*> cv_SparseMatConstIterator_ptr(void* instance) {
		try {
			unsigned char* ret = reinterpret_cast<cv::SparseMatConstIterator*>(instance)->ptr;
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(Result<unsigned char*>)
	}
	
	Result_void cv_SparseMatConstIterator_setPtr_unsigned_charX(void* instance, unsigned char* val) {
		try {
			reinterpret_cast<cv::SparseMatConstIterator*>(instance)->ptr = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_SparseMatConstIterator_delete(cv::SparseMatConstIterator* instance) {
		delete instance;
	}
	Result<void*> cv_SparseMatConstIterator_SparseMatConstIterator() {
		try {
			cv::SparseMatConstIterator* ret = new cv::SparseMatConstIterator();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_SparseMatConstIterator_SparseMatConstIterator_const_SparseMatX(void* _m) {
		try {
			cv::SparseMatConstIterator* ret = new cv::SparseMatConstIterator(reinterpret_cast<const cv::SparseMat*>(_m));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_SparseMatConstIterator_SparseMatConstIterator_const_SparseMatConstIteratorX(void* it) {
		try {
			cv::SparseMatConstIterator* ret = new cv::SparseMatConstIterator(*reinterpret_cast<const cv::SparseMatConstIterator*>(it));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_SparseMatConstIterator_node_const(void* instance) {
		try {
			const cv::SparseMat::Node* ret = reinterpret_cast<cv::SparseMatConstIterator*>(instance)->node();
			return Ok<void*>(new const cv::SparseMat::Node*(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_SparseMatConstIterator_seekEnd(void* instance) {
		try {
			reinterpret_cast<cv::SparseMatConstIterator*>(instance)->seekEnd();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_SparseMatIterator_delete(cv::SparseMatIterator* instance) {
		delete instance;
	}
	Result<void*> cv_SparseMatIterator_SparseMatIterator() {
		try {
			cv::SparseMatIterator* ret = new cv::SparseMatIterator();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_SparseMatIterator_SparseMatIterator_SparseMatX(void* _m) {
		try {
			cv::SparseMatIterator* ret = new cv::SparseMatIterator(reinterpret_cast<cv::SparseMat*>(_m));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_SparseMatIterator_SparseMatIterator_SparseMatX_const_intX(void* _m, const int* idx) {
		try {
			cv::SparseMatIterator* ret = new cv::SparseMatIterator(reinterpret_cast<cv::SparseMat*>(_m), idx);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_SparseMatIterator_SparseMatIterator_const_SparseMatIteratorX(void* it) {
		try {
			cv::SparseMatIterator* ret = new cv::SparseMatIterator(*reinterpret_cast<const cv::SparseMatIterator*>(it));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_SparseMatIterator_node_const(void* instance) {
		try {
			cv::SparseMat::Node* ret = reinterpret_cast<cv::SparseMatIterator*>(instance)->node();
			return Ok<void*>(new cv::SparseMat::Node*(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_TLSDataContainer_cleanup(void* instance) {
		try {
			reinterpret_cast<cv::TLSDataContainer*>(instance)->cleanup();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_TermCriteria_type_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::TermCriteria*>(instance)->type;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_TermCriteria_setType_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::TermCriteria*>(instance)->type = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_TermCriteria_maxCount_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::TermCriteria*>(instance)->maxCount;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_TermCriteria_setMaxCount_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::TermCriteria*>(instance)->maxCount = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_TermCriteria_epsilon_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::TermCriteria*>(instance)->epsilon;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_TermCriteria_setEpsilon_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::TermCriteria*>(instance)->epsilon = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_TermCriteria_delete(cv::TermCriteria* instance) {
		delete instance;
	}
	Result<void*> cv_TermCriteria_TermCriteria() {
		try {
			cv::TermCriteria* ret = new cv::TermCriteria();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_TermCriteria_TermCriteria_int_int_double(int type, int maxCount, double epsilon) {
		try {
			cv::TermCriteria* ret = new cv::TermCriteria(type, maxCount, epsilon);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_TermCriteria_isValid_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::TermCriteria*>(instance)->isValid();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	void cv_TickMeter_delete(cv::TickMeter* instance) {
		delete instance;
	}
	Result<void*> cv_TickMeter_TickMeter() {
		try {
			cv::TickMeter* ret = new cv::TickMeter();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_TickMeter_start(void* instance) {
		try {
			reinterpret_cast<cv::TickMeter*>(instance)->start();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_TickMeter_stop(void* instance) {
		try {
			reinterpret_cast<cv::TickMeter*>(instance)->stop();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int64_t> cv_TickMeter_getTimeTicks_const(void* instance) {
		try {
			int64_t ret = reinterpret_cast<cv::TickMeter*>(instance)->getTimeTicks();
			return Ok<int64_t>(ret);
		} OCVRS_CATCH(Result<int64_t>)
	}
	
	Result<double> cv_TickMeter_getTimeMicro_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::TickMeter*>(instance)->getTimeMicro();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<double> cv_TickMeter_getTimeMilli_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::TickMeter*>(instance)->getTimeMilli();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<double> cv_TickMeter_getTimeSec_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::TickMeter*>(instance)->getTimeSec();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<int64_t> cv_TickMeter_getCounter_const(void* instance) {
		try {
			int64_t ret = reinterpret_cast<cv::TickMeter*>(instance)->getCounter();
			return Ok<int64_t>(ret);
		} OCVRS_CATCH(Result<int64_t>)
	}
	
	Result_void cv_TickMeter_reset(void* instance) {
		try {
			reinterpret_cast<cv::TickMeter*>(instance)->reset();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_UMat_flags_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::UMat*>(instance)->flags;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_UMat_setFlags_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::UMat*>(instance)->flags = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_UMat_dims_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::UMat*>(instance)->dims;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_UMat_setDims_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::UMat*>(instance)->dims = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_UMat_rows_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::UMat*>(instance)->rows;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_UMat_setRows_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::UMat*>(instance)->rows = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_UMat_cols_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::UMat*>(instance)->cols;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_UMat_setCols_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::UMat*>(instance)->cols = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::UMatUsageFlags> cv_UMat_usageFlags_const(void* instance) {
		try {
			cv::UMatUsageFlags ret = reinterpret_cast<cv::UMat*>(instance)->usageFlags;
			return Ok<cv::UMatUsageFlags>(ret);
		} OCVRS_CATCH(Result<cv::UMatUsageFlags>)
	}
	
	Result_void cv_UMat_setUsageFlags_UMatUsageFlags(void* instance, cv::UMatUsageFlags val) {
		try {
			reinterpret_cast<cv::UMat*>(instance)->usageFlags = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_UMat_u(void* instance) {
		try {
			cv::UMatData* ret = reinterpret_cast<cv::UMat*>(instance)->u;
			return Ok<void*>(new cv::UMatData*(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_UMat_setU_UMatDataX(void* instance, void* val) {
		try {
			reinterpret_cast<cv::UMat*>(instance)->u = reinterpret_cast<cv::UMatData*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<size_t> cv_UMat_offset_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::UMat*>(instance)->offset;
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result_void cv_UMat_setOffset_size_t(void* instance, size_t val) {
		try {
			reinterpret_cast<cv::UMat*>(instance)->offset = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_UMat_size_const(void* instance) {
		try {
			cv::MatSize ret = reinterpret_cast<cv::UMat*>(instance)->size;
			return Ok<void*>(new cv::MatSize(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_step_const(void* instance) {
		try {
			cv::MatStep ret = reinterpret_cast<cv::UMat*>(instance)->step;
			return Ok<void*>(new cv::MatStep(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_UMat_delete(cv::UMat* instance) {
		delete instance;
	}
	Result<void*> cv_UMat_UMat_UMatUsageFlags(cv::UMatUsageFlags usageFlags) {
		try {
			cv::UMat* ret = new cv::UMat(usageFlags);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_UMat_int_int_int_UMatUsageFlags(int rows, int cols, int type, cv::UMatUsageFlags usageFlags) {
		try {
			cv::UMat* ret = new cv::UMat(rows, cols, type, usageFlags);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_UMat_Size_int_UMatUsageFlags(cv::Size size, int type, cv::UMatUsageFlags usageFlags) {
		try {
			cv::UMat* ret = new cv::UMat(size, type, usageFlags);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_UMat_int_int_int_const_ScalarX_UMatUsageFlags(int rows, int cols, int type, const cv::Scalar* s, cv::UMatUsageFlags usageFlags) {
		try {
			cv::UMat* ret = new cv::UMat(rows, cols, type, *s, usageFlags);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_UMat_Size_int_const_ScalarX_UMatUsageFlags(cv::Size size, int type, const cv::Scalar* s, cv::UMatUsageFlags usageFlags) {
		try {
			cv::UMat* ret = new cv::UMat(size, type, *s, usageFlags);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_UMat_int_const_intX_int_UMatUsageFlags(int ndims, const int* sizes, int type, cv::UMatUsageFlags usageFlags) {
		try {
			cv::UMat* ret = new cv::UMat(ndims, sizes, type, usageFlags);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_UMat_int_const_intX_int_const_ScalarX_UMatUsageFlags(int ndims, const int* sizes, int type, const cv::Scalar* s, cv::UMatUsageFlags usageFlags) {
		try {
			cv::UMat* ret = new cv::UMat(ndims, sizes, type, *s, usageFlags);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_UMat_const_UMatX(void* m) {
		try {
			cv::UMat* ret = new cv::UMat(*reinterpret_cast<const cv::UMat*>(m));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_UMat_const_UMatX_const_RangeX_const_RangeX(void* m, void* rowRange, void* colRange) {
		try {
			cv::UMat* ret = new cv::UMat(*reinterpret_cast<const cv::UMat*>(m), *reinterpret_cast<const cv::Range*>(rowRange), *reinterpret_cast<const cv::Range*>(colRange));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_UMat_const_UMatX_const_RectX(void* m, const cv::Rect* roi) {
		try {
			cv::UMat* ret = new cv::UMat(*reinterpret_cast<const cv::UMat*>(m), *roi);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_UMat_const_UMatX_const_vector_Range_X(void* m, void* ranges) {
		try {
			cv::UMat* ret = new cv::UMat(*reinterpret_cast<const cv::UMat*>(m), *reinterpret_cast<const std::vector<cv::Range>*>(ranges));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_getMat_const_AccessFlag(void* instance, cv::AccessFlag flags) {
		try {
			cv::Mat ret = reinterpret_cast<cv::UMat*>(instance)->getMat(flags);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_row_const_int(void* instance, int y) {
		try {
			cv::UMat ret = reinterpret_cast<cv::UMat*>(instance)->row(y);
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_col_const_int(void* instance, int x) {
		try {
			cv::UMat ret = reinterpret_cast<cv::UMat*>(instance)->col(x);
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_rowRange_const_int_int(void* instance, int startrow, int endrow) {
		try {
			cv::UMat ret = reinterpret_cast<cv::UMat*>(instance)->rowRange(startrow, endrow);
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_rowRange_const_const_RangeX(void* instance, void* r) {
		try {
			cv::UMat ret = reinterpret_cast<cv::UMat*>(instance)->rowRange(*reinterpret_cast<const cv::Range*>(r));
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_colRange_const_int_int(void* instance, int startcol, int endcol) {
		try {
			cv::UMat ret = reinterpret_cast<cv::UMat*>(instance)->colRange(startcol, endcol);
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_colRange_const_const_RangeX(void* instance, void* r) {
		try {
			cv::UMat ret = reinterpret_cast<cv::UMat*>(instance)->colRange(*reinterpret_cast<const cv::Range*>(r));
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_diag_const_int(void* instance, int d) {
		try {
			cv::UMat ret = reinterpret_cast<cv::UMat*>(instance)->diag(d);
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_diag_const_UMatX(void* d) {
		try {
			cv::UMat ret = cv::UMat::diag(*reinterpret_cast<const cv::UMat*>(d));
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_clone_const(void* instance) {
		try {
			cv::UMat ret = reinterpret_cast<cv::UMat*>(instance)->clone();
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_UMat_copyTo_const_const__OutputArrayX(void* instance, void* m) {
		try {
			reinterpret_cast<cv::UMat*>(instance)->copyTo(*reinterpret_cast<const cv::_OutputArray*>(m));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_UMat_copyTo_const_const__OutputArrayX_const__InputArrayX(void* instance, void* m, void* mask) {
		try {
			reinterpret_cast<cv::UMat*>(instance)->copyTo(*reinterpret_cast<const cv::_OutputArray*>(m), *reinterpret_cast<const cv::_InputArray*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_UMat_convertTo_const_const__OutputArrayX_int_double_double(void* instance, void* m, int rtype, double alpha, double beta) {
		try {
			reinterpret_cast<cv::UMat*>(instance)->convertTo(*reinterpret_cast<const cv::_OutputArray*>(m), rtype, alpha, beta);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_UMat_assignTo_const_UMatX_int(void* instance, void* m, int type) {
		try {
			reinterpret_cast<cv::UMat*>(instance)->assignTo(*reinterpret_cast<cv::UMat*>(m), type);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_UMat_setTo_const__InputArrayX_const__InputArrayX(void* instance, void* value, void* mask) {
		try {
			cv::UMat ret = reinterpret_cast<cv::UMat*>(instance)->setTo(*reinterpret_cast<const cv::_InputArray*>(value), *reinterpret_cast<const cv::_InputArray*>(mask));
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_reshape_const_int_int(void* instance, int cn, int rows) {
		try {
			cv::UMat ret = reinterpret_cast<cv::UMat*>(instance)->reshape(cn, rows);
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_reshape_const_int_int_const_intX(void* instance, int cn, int newndims, const int* newsz) {
		try {
			cv::UMat ret = reinterpret_cast<cv::UMat*>(instance)->reshape(cn, newndims, newsz);
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_t_const(void* instance) {
		try {
			cv::UMat ret = reinterpret_cast<cv::UMat*>(instance)->t();
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_inv_const_int(void* instance, int method) {
		try {
			cv::UMat ret = reinterpret_cast<cv::UMat*>(instance)->inv(method);
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_mul_const_const__InputArrayX_double(void* instance, void* m, double scale) {
		try {
			cv::UMat ret = reinterpret_cast<cv::UMat*>(instance)->mul(*reinterpret_cast<const cv::_InputArray*>(m), scale);
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<double> cv_UMat_dot_const_const__InputArrayX(void* instance, void* m) {
		try {
			double ret = reinterpret_cast<cv::UMat*>(instance)->dot(*reinterpret_cast<const cv::_InputArray*>(m));
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<void*> cv_UMat_zeros_int_int_int(int rows, int cols, int type) {
		try {
			cv::UMat ret = cv::UMat::zeros(rows, cols, type);
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_zeros_Size_int(cv::Size size, int type) {
		try {
			cv::UMat ret = cv::UMat::zeros(size, type);
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_zeros_int_const_intX_int(int ndims, const int* sz, int type) {
		try {
			cv::UMat ret = cv::UMat::zeros(ndims, sz, type);
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_ones_int_int_int(int rows, int cols, int type) {
		try {
			cv::UMat ret = cv::UMat::ones(rows, cols, type);
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_ones_Size_int(cv::Size size, int type) {
		try {
			cv::UMat ret = cv::UMat::ones(size, type);
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_ones_int_const_intX_int(int ndims, const int* sz, int type) {
		try {
			cv::UMat ret = cv::UMat::ones(ndims, sz, type);
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_eye_int_int_int(int rows, int cols, int type) {
		try {
			cv::UMat ret = cv::UMat::eye(rows, cols, type);
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_eye_Size_int(cv::Size size, int type) {
		try {
			cv::UMat ret = cv::UMat::eye(size, type);
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_UMat_create_int_int_int_UMatUsageFlags(void* instance, int rows, int cols, int type, cv::UMatUsageFlags usageFlags) {
		try {
			reinterpret_cast<cv::UMat*>(instance)->create(rows, cols, type, usageFlags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_UMat_create_Size_int_UMatUsageFlags(void* instance, cv::Size size, int type, cv::UMatUsageFlags usageFlags) {
		try {
			reinterpret_cast<cv::UMat*>(instance)->create(size, type, usageFlags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_UMat_create_int_const_intX_int_UMatUsageFlags(void* instance, int ndims, const int* sizes, int type, cv::UMatUsageFlags usageFlags) {
		try {
			reinterpret_cast<cv::UMat*>(instance)->create(ndims, sizes, type, usageFlags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_UMat_create_const_vector_int_X_int_UMatUsageFlags(void* instance, void* sizes, int type, cv::UMatUsageFlags usageFlags) {
		try {
			reinterpret_cast<cv::UMat*>(instance)->create(*reinterpret_cast<const std::vector<int>*>(sizes), type, usageFlags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_UMat_addref(void* instance) {
		try {
			reinterpret_cast<cv::UMat*>(instance)->addref();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_UMat_release(void* instance) {
		try {
			reinterpret_cast<cv::UMat*>(instance)->release();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_UMat_deallocate(void* instance) {
		try {
			reinterpret_cast<cv::UMat*>(instance)->deallocate();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_UMat_locateROI_const_SizeX_PointX(void* instance, cv::Size* wholeSize, cv::Point* ofs) {
		try {
			reinterpret_cast<cv::UMat*>(instance)->locateROI(*wholeSize, *ofs);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_UMat_adjustROI_int_int_int_int(void* instance, int dtop, int dbottom, int dleft, int dright) {
		try {
			cv::UMat ret = reinterpret_cast<cv::UMat*>(instance)->adjustROI(dtop, dbottom, dleft, dright);
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_UMat_isContinuous_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::UMat*>(instance)->isContinuous();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_UMat_isSubmatrix_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::UMat*>(instance)->isSubmatrix();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<size_t> cv_UMat_elemSize_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::UMat*>(instance)->elemSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<size_t> cv_UMat_elemSize1_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::UMat*>(instance)->elemSize1();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<int> cv_UMat_type_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::UMat*>(instance)->type();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_UMat_depth_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::UMat*>(instance)->depth();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_UMat_channels_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::UMat*>(instance)->channels();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<size_t> cv_UMat_step1_const_int(void* instance, int i) {
		try {
			size_t ret = reinterpret_cast<cv::UMat*>(instance)->step1(i);
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<bool> cv_UMat_empty_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::UMat*>(instance)->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<size_t> cv_UMat_total_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::UMat*>(instance)->total();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<int> cv_UMat_checkVector_const_int_int_bool(void* instance, int elemChannels, int depth, bool requireContinuous) {
		try {
			int ret = reinterpret_cast<cv::UMat*>(instance)->checkVector(elemChannels, depth, requireContinuous);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv_UMat_UMat_UMatX(void* m) {
		try {
			cv::UMat* ret = new cv::UMat(*reinterpret_cast<cv::UMat*>(m));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_UMat_handle_const_AccessFlag(void* instance, cv::AccessFlag accessFlags) {
		try {
			void* ret = reinterpret_cast<cv::UMat*>(instance)->handle(accessFlags);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_UMat_ndoffset_const_size_tX(void* instance, size_t* ofs) {
		try {
			reinterpret_cast<cv::UMat*>(instance)->ndoffset(ofs);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_UMat_updateContinuityFlag(void* instance) {
		try {
			reinterpret_cast<cv::UMat*>(instance)->updateContinuityFlag();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_UMatData_urefcount_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::UMatData*>(instance)->urefcount;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_UMatData_setUrefcount_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::UMatData*>(instance)->urefcount = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_UMatData_refcount_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::UMatData*>(instance)->refcount;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_UMatData_setRefcount_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::UMatData*>(instance)->refcount = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<unsigned char*> cv_UMatData_data(void* instance) {
		try {
			unsigned char* ret = reinterpret_cast<cv::UMatData*>(instance)->data;
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(Result<unsigned char*>)
	}
	
	Result_void cv_UMatData_setData_unsigned_charX(void* instance, unsigned char* val) {
		try {
			reinterpret_cast<cv::UMatData*>(instance)->data = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<unsigned char*> cv_UMatData_origdata(void* instance) {
		try {
			unsigned char* ret = reinterpret_cast<cv::UMatData*>(instance)->origdata;
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(Result<unsigned char*>)
	}
	
	Result_void cv_UMatData_setOrigdata_unsigned_charX(void* instance, unsigned char* val) {
		try {
			reinterpret_cast<cv::UMatData*>(instance)->origdata = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<size_t> cv_UMatData_size_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::UMatData*>(instance)->size;
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result_void cv_UMatData_setSize_size_t(void* instance, size_t val) {
		try {
			reinterpret_cast<cv::UMatData*>(instance)->size = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::UMatData::MemoryFlag> cv_UMatData_flags_const(void* instance) {
		try {
			cv::UMatData::MemoryFlag ret = reinterpret_cast<cv::UMatData*>(instance)->flags;
			return Ok<cv::UMatData::MemoryFlag>(ret);
		} OCVRS_CATCH(Result<cv::UMatData::MemoryFlag>)
	}
	
	Result_void cv_UMatData_setFlags_MemoryFlag(void* instance, cv::UMatData::MemoryFlag val) {
		try {
			reinterpret_cast<cv::UMatData*>(instance)->flags = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_UMatData_handle(void* instance) {
		try {
			void* ret = reinterpret_cast<cv::UMatData*>(instance)->handle;
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_UMatData_setHandle_voidX(void* instance, void* val) {
		try {
			reinterpret_cast<cv::UMatData*>(instance)->handle = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_UMatData_userdata(void* instance) {
		try {
			void* ret = reinterpret_cast<cv::UMatData*>(instance)->userdata;
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_UMatData_setUserdata_voidX(void* instance, void* val) {
		try {
			reinterpret_cast<cv::UMatData*>(instance)->userdata = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_UMatData_allocatorFlags__const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::UMatData*>(instance)->allocatorFlags_;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_UMatData_setAllocatorFlags__int(void* instance, int val) {
		try {
			reinterpret_cast<cv::UMatData*>(instance)->allocatorFlags_ = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_UMatData_mapcount_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::UMatData*>(instance)->mapcount;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_UMatData_setMapcount_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::UMatData*>(instance)->mapcount = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_UMatData_originalUMatData(void* instance) {
		try {
			cv::UMatData* ret = reinterpret_cast<cv::UMatData*>(instance)->originalUMatData;
			return Ok<void*>(new cv::UMatData*(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_UMatData_setOriginalUMatData_UMatDataX(void* instance, void* val) {
		try {
			reinterpret_cast<cv::UMatData*>(instance)->originalUMatData = reinterpret_cast<cv::UMatData*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_UMatData_delete(cv::UMatData* instance) {
		delete instance;
	}
	Result_void cv_UMatData_lock(void* instance) {
		try {
			reinterpret_cast<cv::UMatData*>(instance)->lock();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_UMatData_unlock(void* instance) {
		try {
			reinterpret_cast<cv::UMatData*>(instance)->unlock();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_UMatData_hostCopyObsolete_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::UMatData*>(instance)->hostCopyObsolete();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_UMatData_deviceCopyObsolete_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::UMatData*>(instance)->deviceCopyObsolete();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_UMatData_deviceMemMapped_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::UMatData*>(instance)->deviceMemMapped();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_UMatData_copyOnMap_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::UMatData*>(instance)->copyOnMap();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_UMatData_tempUMat_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::UMatData*>(instance)->tempUMat();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_UMatData_tempCopiedUMat_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::UMatData*>(instance)->tempCopiedUMat();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_UMatData_markHostCopyObsolete_bool(void* instance, bool flag) {
		try {
			reinterpret_cast<cv::UMatData*>(instance)->markHostCopyObsolete(flag);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_UMatData_markDeviceCopyObsolete_bool(void* instance, bool flag) {
		try {
			reinterpret_cast<cv::UMatData*>(instance)->markDeviceCopyObsolete(flag);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_UMatData_markDeviceMemMapped_bool(void* instance, bool flag) {
		try {
			reinterpret_cast<cv::UMatData*>(instance)->markDeviceMemMapped(flag);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv__InputArray_delete(cv::_InputArray* instance) {
		delete instance;
	}
	Result<void*> cv__InputArray__InputArray() {
		try {
			cv::_InputArray* ret = new cv::_InputArray();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv__InputArray__InputArray_int_voidX(int _flags, void* _obj) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(_flags, _obj);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv__InputArray__InputArray_const_MatX(void* m) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(*reinterpret_cast<const cv::Mat*>(m));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv__InputArray__InputArray_const_MatExprX(void* expr) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(*reinterpret_cast<const cv::MatExpr*>(expr));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv__InputArray__InputArray_const_vector_Mat_X(void* vec) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(*reinterpret_cast<const std::vector<cv::Mat>*>(vec));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv__InputArray__InputArray_const_vector_bool_X(void* vec) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(*reinterpret_cast<const std::vector<bool>*>(vec));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv__InputArray__InputArray_const_doubleX(const double* val) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(*val);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv__InputArray__InputArray_const_UMatX(void* um) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(*reinterpret_cast<const cv::UMat*>(um));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv__InputArray__InputArray_const_vector_UMat_X(void* umv) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(*reinterpret_cast<const std::vector<cv::UMat>*>(umv));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv__InputArray_getMat_const_int(void* instance, int idx) {
		try {
			cv::Mat ret = reinterpret_cast<cv::_InputArray*>(instance)->getMat(idx);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv__InputArray_getMat__const_int(void* instance, int idx) {
		try {
			cv::Mat ret = reinterpret_cast<cv::_InputArray*>(instance)->getMat_(idx);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv__InputArray_getUMat_const_int(void* instance, int idx) {
		try {
			cv::UMat ret = reinterpret_cast<cv::_InputArray*>(instance)->getUMat(idx);
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv__InputArray_getMatVector_const_vector_Mat_X(void* instance, void* mv) {
		try {
			reinterpret_cast<cv::_InputArray*>(instance)->getMatVector(*reinterpret_cast<std::vector<cv::Mat>*>(mv));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv__InputArray_getUMatVector_const_vector_UMat_X(void* instance, void* umv) {
		try {
			reinterpret_cast<cv::_InputArray*>(instance)->getUMatVector(*reinterpret_cast<std::vector<cv::UMat>*>(umv));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv__InputArray_getFlags_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::_InputArray*>(instance)->getFlags();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv__InputArray_getObj_const(void* instance) {
		try {
			void* ret = reinterpret_cast<cv::_InputArray*>(instance)->getObj();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::Size> cv__InputArray_getSz_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::_InputArray*>(instance)->getSz();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result<cv::_InputArray::KindFlag> cv__InputArray_kind_const(void* instance) {
		try {
			cv::_InputArray::KindFlag ret = reinterpret_cast<cv::_InputArray*>(instance)->kind();
			return Ok<cv::_InputArray::KindFlag>(ret);
		} OCVRS_CATCH(Result<cv::_InputArray::KindFlag>)
	}
	
	Result<int> cv__InputArray_dims_const_int(void* instance, int i) {
		try {
			int ret = reinterpret_cast<cv::_InputArray*>(instance)->dims(i);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv__InputArray_cols_const_int(void* instance, int i) {
		try {
			int ret = reinterpret_cast<cv::_InputArray*>(instance)->cols(i);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv__InputArray_rows_const_int(void* instance, int i) {
		try {
			int ret = reinterpret_cast<cv::_InputArray*>(instance)->rows(i);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<cv::Size> cv__InputArray_size_const_int(void* instance, int i) {
		try {
			cv::Size ret = reinterpret_cast<cv::_InputArray*>(instance)->size(i);
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result<int> cv__InputArray_sizend_const_intX_int(void* instance, int* sz, int i) {
		try {
			int ret = reinterpret_cast<cv::_InputArray*>(instance)->sizend(sz, i);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<bool> cv__InputArray_sameSize_const_const__InputArrayX(void* instance, void* arr) {
		try {
			bool ret = reinterpret_cast<cv::_InputArray*>(instance)->sameSize(*reinterpret_cast<const cv::_InputArray*>(arr));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<size_t> cv__InputArray_total_const_int(void* instance, int i) {
		try {
			size_t ret = reinterpret_cast<cv::_InputArray*>(instance)->total(i);
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<int> cv__InputArray_type_const_int(void* instance, int i) {
		try {
			int ret = reinterpret_cast<cv::_InputArray*>(instance)->type(i);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv__InputArray_depth_const_int(void* instance, int i) {
		try {
			int ret = reinterpret_cast<cv::_InputArray*>(instance)->depth(i);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv__InputArray_channels_const_int(void* instance, int i) {
		try {
			int ret = reinterpret_cast<cv::_InputArray*>(instance)->channels(i);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<bool> cv__InputArray_isContinuous_const_int(void* instance, int i) {
		try {
			bool ret = reinterpret_cast<cv::_InputArray*>(instance)->isContinuous(i);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv__InputArray_isSubmatrix_const_int(void* instance, int i) {
		try {
			bool ret = reinterpret_cast<cv::_InputArray*>(instance)->isSubmatrix(i);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv__InputArray_empty_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::_InputArray*>(instance)->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv__InputArray_copyTo_const_const__OutputArrayX(void* instance, void* arr) {
		try {
			reinterpret_cast<cv::_InputArray*>(instance)->copyTo(*reinterpret_cast<const cv::_OutputArray*>(arr));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv__InputArray_copyTo_const_const__OutputArrayX_const__InputArrayX(void* instance, void* arr, void* mask) {
		try {
			reinterpret_cast<cv::_InputArray*>(instance)->copyTo(*reinterpret_cast<const cv::_OutputArray*>(arr), *reinterpret_cast<const cv::_InputArray*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<size_t> cv__InputArray_offset_const_int(void* instance, int i) {
		try {
			size_t ret = reinterpret_cast<cv::_InputArray*>(instance)->offset(i);
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<size_t> cv__InputArray_step_const_int(void* instance, int i) {
		try {
			size_t ret = reinterpret_cast<cv::_InputArray*>(instance)->step(i);
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<bool> cv__InputArray_isMat_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::_InputArray*>(instance)->isMat();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv__InputArray_isUMat_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::_InputArray*>(instance)->isUMat();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv__InputArray_isMatVector_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::_InputArray*>(instance)->isMatVector();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv__InputArray_isUMatVector_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::_InputArray*>(instance)->isUMatVector();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv__InputArray_isMatx_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::_InputArray*>(instance)->isMatx();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv__InputArray_isVector_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::_InputArray*>(instance)->isVector();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv__InputArray_isGpuMat_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::_InputArray*>(instance)->isGpuMat();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv__InputArray_isGpuMatVector_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::_InputArray*>(instance)->isGpuMatVector();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	void cv__InputOutputArray_delete(cv::_InputOutputArray* instance) {
		delete instance;
	}
	Result<void*> cv__InputOutputArray__InputOutputArray() {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv__InputOutputArray__InputOutputArray_int_voidX(int _flags, void* _obj) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(_flags, _obj);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv__InputOutputArray__InputOutputArray_MatX(void* m) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*reinterpret_cast<cv::Mat*>(m));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv__InputOutputArray__InputOutputArray_vector_Mat_X(void* vec) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*reinterpret_cast<std::vector<cv::Mat>*>(vec));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv__InputOutputArray__InputOutputArray_UMatX(void* m) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*reinterpret_cast<cv::UMat*>(m));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv__InputOutputArray__InputOutputArray_vector_UMat_X(void* vec) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*reinterpret_cast<std::vector<cv::UMat>*>(vec));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv__InputOutputArray__InputOutputArray_const_MatX(void* m) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*reinterpret_cast<const cv::Mat*>(m));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv__InputOutputArray__InputOutputArray_const_vector_Mat_X(void* vec) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*reinterpret_cast<const std::vector<cv::Mat>*>(vec));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv__InputOutputArray__InputOutputArray_const_UMatX(void* m) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*reinterpret_cast<const cv::UMat*>(m));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv__InputOutputArray__InputOutputArray_const_vector_UMat_X(void* vec) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*reinterpret_cast<const std::vector<cv::UMat>*>(vec));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv__OutputArray_delete(cv::_OutputArray* instance) {
		delete instance;
	}
	Result<void*> cv__OutputArray__OutputArray() {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv__OutputArray__OutputArray_int_voidX(int _flags, void* _obj) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(_flags, _obj);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv__OutputArray__OutputArray_MatX(void* m) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*reinterpret_cast<cv::Mat*>(m));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv__OutputArray__OutputArray_vector_Mat_X(void* vec) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*reinterpret_cast<std::vector<cv::Mat>*>(vec));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv__OutputArray__OutputArray_UMatX(void* m) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*reinterpret_cast<cv::UMat*>(m));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv__OutputArray__OutputArray_vector_UMat_X(void* vec) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*reinterpret_cast<std::vector<cv::UMat>*>(vec));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv__OutputArray__OutputArray_const_MatX(void* m) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*reinterpret_cast<const cv::Mat*>(m));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv__OutputArray__OutputArray_const_vector_Mat_X(void* vec) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*reinterpret_cast<const std::vector<cv::Mat>*>(vec));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv__OutputArray__OutputArray_const_UMatX(void* m) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*reinterpret_cast<const cv::UMat*>(m));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv__OutputArray__OutputArray_const_vector_UMat_X(void* vec) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*reinterpret_cast<const std::vector<cv::UMat>*>(vec));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv__OutputArray_fixedSize_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::_OutputArray*>(instance)->fixedSize();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv__OutputArray_fixedType_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::_OutputArray*>(instance)->fixedType();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv__OutputArray_needed_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::_OutputArray*>(instance)->needed();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv__OutputArray_getMatRef_const_int(void* instance, int i) {
		try {
			cv::Mat ret = reinterpret_cast<cv::_OutputArray*>(instance)->getMatRef(i);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv__OutputArray_getUMatRef_const_int(void* instance, int i) {
		try {
			cv::UMat ret = reinterpret_cast<cv::_OutputArray*>(instance)->getUMatRef(i);
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv__OutputArray_create_const_Size_int_int_bool_DepthMask(void* instance, cv::Size sz, int type, int i, bool allowTransposed, cv::_OutputArray::DepthMask fixedDepthMask) {
		try {
			reinterpret_cast<cv::_OutputArray*>(instance)->create(sz, type, i, allowTransposed, fixedDepthMask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv__OutputArray_create_const_int_int_int_int_bool_DepthMask(void* instance, int rows, int cols, int type, int i, bool allowTransposed, cv::_OutputArray::DepthMask fixedDepthMask) {
		try {
			reinterpret_cast<cv::_OutputArray*>(instance)->create(rows, cols, type, i, allowTransposed, fixedDepthMask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv__OutputArray_create_const_int_const_intX_int_int_bool_DepthMask(void* instance, int dims, const int* size, int type, int i, bool allowTransposed, cv::_OutputArray::DepthMask fixedDepthMask) {
		try {
			reinterpret_cast<cv::_OutputArray*>(instance)->create(dims, size, type, i, allowTransposed, fixedDepthMask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv__OutputArray_createSameSize_const_const__InputArrayX_int(void* instance, void* arr, int mtype) {
		try {
			reinterpret_cast<cv::_OutputArray*>(instance)->createSameSize(*reinterpret_cast<const cv::_InputArray*>(arr), mtype);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv__OutputArray_release_const(void* instance) {
		try {
			reinterpret_cast<cv::_OutputArray*>(instance)->release();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv__OutputArray_clear_const(void* instance) {
		try {
			reinterpret_cast<cv::_OutputArray*>(instance)->clear();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv__OutputArray_setTo_const_const__InputArrayX_const__InputArrayX(void* instance, void* value, void* mask) {
		try {
			reinterpret_cast<cv::_OutputArray*>(instance)->setTo(*reinterpret_cast<const cv::_InputArray*>(value), *reinterpret_cast<const cv::_InputArray*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv__OutputArray_assign_const_const_UMatX(void* instance, void* u) {
		try {
			reinterpret_cast<cv::_OutputArray*>(instance)->assign(*reinterpret_cast<const cv::UMat*>(u));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv__OutputArray_assign_const_const_MatX(void* instance, void* m) {
		try {
			reinterpret_cast<cv::_OutputArray*>(instance)->assign(*reinterpret_cast<const cv::Mat*>(m));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv__OutputArray_assign_const_const_vector_UMat_X(void* instance, void* v) {
		try {
			reinterpret_cast<cv::_OutputArray*>(instance)->assign(*reinterpret_cast<const std::vector<cv::UMat>*>(v));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv__OutputArray_assign_const_const_vector_Mat_X(void* instance, void* v) {
		try {
			reinterpret_cast<cv::_OutputArray*>(instance)->assign(*reinterpret_cast<const std::vector<cv::Mat>*>(v));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv__OutputArray_move_const_UMatX(void* instance, void* u) {
		try {
			reinterpret_cast<cv::_OutputArray*>(instance)->move(*reinterpret_cast<cv::UMat*>(u));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv__OutputArray_move_const_MatX(void* instance, void* m) {
		try {
			reinterpret_cast<cv::_OutputArray*>(instance)->move(*reinterpret_cast<cv::Mat*>(m));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_detail_CheckContext_func_const(void* instance) {
		try {
			const char* ret = reinterpret_cast<cv::detail::CheckContext*>(instance)->func;
			return Ok<void*>(ocvrs_create_string(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_detail_CheckContext_file_const(void* instance) {
		try {
			const char* ret = reinterpret_cast<cv::detail::CheckContext*>(instance)->file;
			return Ok<void*>(ocvrs_create_string(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_detail_CheckContext_line_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::detail::CheckContext*>(instance)->line;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_detail_CheckContext_setLine_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::detail::CheckContext*>(instance)->line = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::detail::TestOp> cv_detail_CheckContext_testOp_const(void* instance) {
		try {
			cv::detail::TestOp ret = reinterpret_cast<cv::detail::CheckContext*>(instance)->testOp;
			return Ok<cv::detail::TestOp>(ret);
		} OCVRS_CATCH(Result<cv::detail::TestOp>)
	}
	
	Result_void cv_detail_CheckContext_setTestOp_TestOp(void* instance, cv::detail::TestOp val) {
		try {
			reinterpret_cast<cv::detail::CheckContext*>(instance)->testOp = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_detail_CheckContext_message_const(void* instance) {
		try {
			const char* ret = reinterpret_cast<cv::detail::CheckContext*>(instance)->message;
			return Ok<void*>(ocvrs_create_string(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_detail_CheckContext_p1_str_const(void* instance) {
		try {
			const char* ret = reinterpret_cast<cv::detail::CheckContext*>(instance)->p1_str;
			return Ok<void*>(ocvrs_create_string(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_detail_CheckContext_p2_str_const(void* instance) {
		try {
			const char* ret = reinterpret_cast<cv::detail::CheckContext*>(instance)->p2_str;
			return Ok<void*>(ocvrs_create_string(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Detail_CheckContext_delete(cv::detail::CheckContext* instance) {
		delete instance;
	}
	Result<void*> cv_instr_NodeData_m_funName_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::instr::NodeData*>(instance)->m_funName;
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_instr_NodeData_setM_funName_String(void* instance, char* val) {
		try {
			reinterpret_cast<cv::instr::NodeData*>(instance)->m_funName = std::string(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::instr::TYPE> cv_instr_NodeData_m_instrType_const(void* instance) {
		try {
			cv::instr::TYPE ret = reinterpret_cast<cv::instr::NodeData*>(instance)->m_instrType;
			return Ok<cv::instr::TYPE>(ret);
		} OCVRS_CATCH(Result<cv::instr::TYPE>)
	}
	
	Result_void cv_instr_NodeData_setM_instrType_TYPE(void* instance, cv::instr::TYPE val) {
		try {
			reinterpret_cast<cv::instr::NodeData*>(instance)->m_instrType = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::instr::IMPL> cv_instr_NodeData_m_implType_const(void* instance) {
		try {
			cv::instr::IMPL ret = reinterpret_cast<cv::instr::NodeData*>(instance)->m_implType;
			return Ok<cv::instr::IMPL>(ret);
		} OCVRS_CATCH(Result<cv::instr::IMPL>)
	}
	
	Result_void cv_instr_NodeData_setM_implType_IMPL(void* instance, cv::instr::IMPL val) {
		try {
			reinterpret_cast<cv::instr::NodeData*>(instance)->m_implType = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_instr_NodeData_m_fileName_const(void* instance) {
		try {
			const char* ret = reinterpret_cast<cv::instr::NodeData*>(instance)->m_fileName;
			return Ok<void*>(ocvrs_create_string(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_instr_NodeData_m_lineNum_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::instr::NodeData*>(instance)->m_lineNum;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_instr_NodeData_setM_lineNum_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::instr::NodeData*>(instance)->m_lineNum = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_instr_NodeData_m_retAddress(void* instance) {
		try {
			void* ret = reinterpret_cast<cv::instr::NodeData*>(instance)->m_retAddress;
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_instr_NodeData_setM_retAddress_voidX(void* instance, void* val) {
		try {
			reinterpret_cast<cv::instr::NodeData*>(instance)->m_retAddress = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_instr_NodeData_m_alwaysExpand_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::instr::NodeData*>(instance)->m_alwaysExpand;
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_instr_NodeData_setM_alwaysExpand_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::instr::NodeData*>(instance)->m_alwaysExpand = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_instr_NodeData_m_funError_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::instr::NodeData*>(instance)->m_funError;
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_instr_NodeData_setM_funError_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::instr::NodeData*>(instance)->m_funError = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_instr_NodeData_m_counter_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::instr::NodeData*>(instance)->m_counter;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_instr_NodeData_setM_counter_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::instr::NodeData*>(instance)->m_counter = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<uint64_t> cv_instr_NodeData_m_ticksTotal_const(void* instance) {
		try {
			uint64_t ret = reinterpret_cast<cv::instr::NodeData*>(instance)->m_ticksTotal;
			return Ok<uint64_t>(ret);
		} OCVRS_CATCH(Result<uint64_t>)
	}
	
	Result_void cv_instr_NodeData_setM_ticksTotal_uint64_t(void* instance, uint64_t val) {
		try {
			reinterpret_cast<cv::instr::NodeData*>(instance)->m_ticksTotal = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_instr_NodeData_m_threads_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::instr::NodeData*>(instance)->m_threads;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_instr_NodeData_setM_threads_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::instr::NodeData*>(instance)->m_threads = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_NodeData_delete(cv::instr::NodeData* instance) {
		delete instance;
	}
	Result<void*> cv_instr_NodeData_NodeData_const_charX_const_charX_int_voidX_bool_TYPE_IMPL(const char* funName, const char* fileName, int lineNum, void* retAddress, bool alwaysExpand, cv::instr::TYPE instrType, cv::instr::IMPL implType) {
		try {
			cv::instr::NodeData* ret = new cv::instr::NodeData(funName, fileName, lineNum, retAddress, alwaysExpand, instrType, implType);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_instr_NodeData_NodeData_NodeDataX(void* ref) {
		try {
			cv::instr::NodeData* ret = new cv::instr::NodeData(*reinterpret_cast<cv::instr::NodeData*>(ref));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<double> cv_instr_NodeData_getTotalMs_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::instr::NodeData*>(instance)->getTotalMs();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<double> cv_instr_NodeData_getMeanMs_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::instr::NodeData*>(instance)->getMeanMs();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	void cv_WriteStructContext_delete(cv::internal::WriteStructContext* instance) {
		delete instance;
	}
	Result<void*> cv_internal_WriteStructContext_WriteStructContext_FileStorageX_const_StringX_int_const_StringX(void* _fs, const char* name, int flags, const char* typeName) {
		try {
			cv::internal::WriteStructContext* ret = new cv::internal::WriteStructContext(*reinterpret_cast<cv::FileStorage*>(_fs), std::string(name), flags, std::string(typeName));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Context_delete(cv::ocl::Context* instance) {
		delete instance;
	}
	Result<void*> cv_ocl_Context_Context() {
		try {
			cv::ocl::Context* ret = new cv::ocl::Context();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_Context_Context_int(int dtype) {
		try {
			cv::ocl::Context* ret = new cv::ocl::Context(dtype);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_Context_Context_const_ContextX(void* c) {
		try {
			cv::ocl::Context* ret = new cv::ocl::Context(*reinterpret_cast<const cv::ocl::Context*>(c));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_ocl_Context_create(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::ocl::Context*>(instance)->create();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ocl_Context_create_int(void* instance, int dtype) {
		try {
			bool ret = reinterpret_cast<cv::ocl::Context*>(instance)->create(dtype);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<size_t> cv_ocl_Context_ndevices_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::ocl::Context*>(instance)->ndevices();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<void*> cv_ocl_Context_device_const_size_t(void* instance, size_t idx) {
		try {
			cv::ocl::Device ret = reinterpret_cast<cv::ocl::Context*>(instance)->device(idx);
			return Ok<void*>(new cv::ocl::Device(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_Context_getProg_const_ProgramSourceX_const_StringX_StringX(void* instance, void* prog, const char* buildopt, void** errmsg) {
		try {
			std::string errmsg_out;
			cv::ocl::Program ret = reinterpret_cast<cv::ocl::Context*>(instance)->getProg(*reinterpret_cast<const cv::ocl::ProgramSource*>(prog), std::string(buildopt), errmsg_out);
			*errmsg = ocvrs_create_string(errmsg_out.c_str());
			return Ok<void*>(new cv::ocl::Program(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_ocl_Context_unloadProg_ProgramX(void* instance, void* prog) {
		try {
			reinterpret_cast<cv::ocl::Context*>(instance)->unloadProg(*reinterpret_cast<cv::ocl::Program*>(prog));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_ocl_Context_getDefault_bool(bool initialize) {
		try {
			cv::ocl::Context ret = cv::ocl::Context::getDefault(initialize);
			return Ok<void*>(new cv::ocl::Context(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_Context_ptr_const(void* instance) {
		try {
			void* ret = reinterpret_cast<cv::ocl::Context*>(instance)->ptr();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_ocl_Context_useSVM_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::ocl::Context*>(instance)->useSVM();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_ocl_Context_setUseSVM_bool(void* instance, bool enabled) {
		try {
			reinterpret_cast<cv::ocl::Context*>(instance)->setUseSVM(enabled);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Device_delete(cv::ocl::Device* instance) {
		delete instance;
	}
	Result<void*> cv_ocl_Device_Device() {
		try {
			cv::ocl::Device* ret = new cv::ocl::Device();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_Device_Device_voidX(void* d) {
		try {
			cv::ocl::Device* ret = new cv::ocl::Device(d);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_Device_Device_const_DeviceX(void* d) {
		try {
			cv::ocl::Device* ret = new cv::ocl::Device(*reinterpret_cast<const cv::ocl::Device*>(d));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_ocl_Device_set_voidX(void* instance, void* d) {
		try {
			reinterpret_cast<cv::ocl::Device*>(instance)->set(d);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_ocl_Device_name_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::ocl::Device*>(instance)->name();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_Device_extensions_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::ocl::Device*>(instance)->extensions();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_ocl_Device_isExtensionSupported_const_const_StringX(void* instance, const char* extensionName) {
		try {
			bool ret = reinterpret_cast<cv::ocl::Device*>(instance)->isExtensionSupported(std::string(extensionName));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_ocl_Device_version_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::ocl::Device*>(instance)->version();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_Device_vendorName_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::ocl::Device*>(instance)->vendorName();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_Device_OpenCL_C_Version_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::ocl::Device*>(instance)->OpenCL_C_Version();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_Device_OpenCLVersion_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::ocl::Device*>(instance)->OpenCLVersion();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_ocl_Device_deviceVersionMajor_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->deviceVersionMajor();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ocl_Device_deviceVersionMinor_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->deviceVersionMinor();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv_ocl_Device_driverVersion_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::ocl::Device*>(instance)->driverVersion();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_Device_ptr_const(void* instance) {
		try {
			void* ret = reinterpret_cast<cv::ocl::Device*>(instance)->ptr();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_ocl_Device_type_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->type();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ocl_Device_addressBits_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->addressBits();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<bool> cv_ocl_Device_available_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::ocl::Device*>(instance)->available();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ocl_Device_compilerAvailable_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::ocl::Device*>(instance)->compilerAvailable();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ocl_Device_linkerAvailable_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::ocl::Device*>(instance)->linkerAvailable();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<int> cv_ocl_Device_doubleFPConfig_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->doubleFPConfig();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ocl_Device_singleFPConfig_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->singleFPConfig();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ocl_Device_halfFPConfig_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->halfFPConfig();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<bool> cv_ocl_Device_endianLittle_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::ocl::Device*>(instance)->endianLittle();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ocl_Device_errorCorrectionSupport_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::ocl::Device*>(instance)->errorCorrectionSupport();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<int> cv_ocl_Device_executionCapabilities_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->executionCapabilities();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<size_t> cv_ocl_Device_globalMemCacheSize_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::ocl::Device*>(instance)->globalMemCacheSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<int> cv_ocl_Device_globalMemCacheType_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->globalMemCacheType();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ocl_Device_globalMemCacheLineSize_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->globalMemCacheLineSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<size_t> cv_ocl_Device_globalMemSize_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::ocl::Device*>(instance)->globalMemSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<size_t> cv_ocl_Device_localMemSize_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::ocl::Device*>(instance)->localMemSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<int> cv_ocl_Device_localMemType_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->localMemType();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<bool> cv_ocl_Device_hostUnifiedMemory_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::ocl::Device*>(instance)->hostUnifiedMemory();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ocl_Device_imageSupport_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::ocl::Device*>(instance)->imageSupport();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ocl_Device_imageFromBufferSupport_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::ocl::Device*>(instance)->imageFromBufferSupport();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<unsigned int> cv_ocl_Device_imagePitchAlignment_const(void* instance) {
		try {
			unsigned int ret = reinterpret_cast<cv::ocl::Device*>(instance)->imagePitchAlignment();
			return Ok<unsigned int>(ret);
		} OCVRS_CATCH(Result<unsigned int>)
	}
	
	Result<unsigned int> cv_ocl_Device_imageBaseAddressAlignment_const(void* instance) {
		try {
			unsigned int ret = reinterpret_cast<cv::ocl::Device*>(instance)->imageBaseAddressAlignment();
			return Ok<unsigned int>(ret);
		} OCVRS_CATCH(Result<unsigned int>)
	}
	
	Result<bool> cv_ocl_Device_intelSubgroupsSupport_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::ocl::Device*>(instance)->intelSubgroupsSupport();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<size_t> cv_ocl_Device_image2DMaxWidth_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::ocl::Device*>(instance)->image2DMaxWidth();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<size_t> cv_ocl_Device_image2DMaxHeight_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::ocl::Device*>(instance)->image2DMaxHeight();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<size_t> cv_ocl_Device_image3DMaxWidth_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::ocl::Device*>(instance)->image3DMaxWidth();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<size_t> cv_ocl_Device_image3DMaxHeight_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::ocl::Device*>(instance)->image3DMaxHeight();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<size_t> cv_ocl_Device_image3DMaxDepth_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::ocl::Device*>(instance)->image3DMaxDepth();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<size_t> cv_ocl_Device_imageMaxBufferSize_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::ocl::Device*>(instance)->imageMaxBufferSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<size_t> cv_ocl_Device_imageMaxArraySize_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::ocl::Device*>(instance)->imageMaxArraySize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<int> cv_ocl_Device_vendorID_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->vendorID();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<bool> cv_ocl_Device_isAMD_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::ocl::Device*>(instance)->isAMD();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ocl_Device_isIntel_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::ocl::Device*>(instance)->isIntel();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ocl_Device_isNVidia_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::ocl::Device*>(instance)->isNVidia();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<int> cv_ocl_Device_maxClockFrequency_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->maxClockFrequency();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ocl_Device_maxComputeUnits_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->maxComputeUnits();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ocl_Device_maxConstantArgs_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->maxConstantArgs();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<size_t> cv_ocl_Device_maxConstantBufferSize_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::ocl::Device*>(instance)->maxConstantBufferSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<size_t> cv_ocl_Device_maxMemAllocSize_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::ocl::Device*>(instance)->maxMemAllocSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<size_t> cv_ocl_Device_maxParameterSize_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::ocl::Device*>(instance)->maxParameterSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<int> cv_ocl_Device_maxReadImageArgs_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->maxReadImageArgs();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ocl_Device_maxWriteImageArgs_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->maxWriteImageArgs();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ocl_Device_maxSamplers_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->maxSamplers();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<size_t> cv_ocl_Device_maxWorkGroupSize_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::ocl::Device*>(instance)->maxWorkGroupSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<int> cv_ocl_Device_maxWorkItemDims_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->maxWorkItemDims();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ocl_Device_maxWorkItemSizes_const_size_tX(void* instance, size_t* unnamed) {
		try {
			reinterpret_cast<cv::ocl::Device*>(instance)->maxWorkItemSizes(unnamed);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ocl_Device_memBaseAddrAlign_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->memBaseAddrAlign();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ocl_Device_nativeVectorWidthChar_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->nativeVectorWidthChar();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ocl_Device_nativeVectorWidthShort_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->nativeVectorWidthShort();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ocl_Device_nativeVectorWidthInt_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->nativeVectorWidthInt();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ocl_Device_nativeVectorWidthLong_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->nativeVectorWidthLong();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ocl_Device_nativeVectorWidthFloat_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->nativeVectorWidthFloat();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ocl_Device_nativeVectorWidthDouble_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->nativeVectorWidthDouble();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ocl_Device_nativeVectorWidthHalf_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->nativeVectorWidthHalf();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ocl_Device_preferredVectorWidthChar_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->preferredVectorWidthChar();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ocl_Device_preferredVectorWidthShort_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->preferredVectorWidthShort();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ocl_Device_preferredVectorWidthInt_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->preferredVectorWidthInt();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ocl_Device_preferredVectorWidthLong_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->preferredVectorWidthLong();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ocl_Device_preferredVectorWidthFloat_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->preferredVectorWidthFloat();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ocl_Device_preferredVectorWidthDouble_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->preferredVectorWidthDouble();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ocl_Device_preferredVectorWidthHalf_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::Device*>(instance)->preferredVectorWidthHalf();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<size_t> cv_ocl_Device_printfBufferSize_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::ocl::Device*>(instance)->printfBufferSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<size_t> cv_ocl_Device_profilingTimerResolution_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::ocl::Device*>(instance)->profilingTimerResolution();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<void*> cv_ocl_Device_getDefault() {
		try {
			cv::ocl::Device ret = cv::ocl::Device::getDefault();
			return Ok<void*>(new cv::ocl::Device(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Image2D_delete(cv::ocl::Image2D* instance) {
		delete instance;
	}
	Result<void*> cv_ocl_Image2D_Image2D() {
		try {
			cv::ocl::Image2D* ret = new cv::ocl::Image2D();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_Image2D_Image2D_const_UMatX_bool_bool(void* src, bool norm, bool alias) {
		try {
			cv::ocl::Image2D* ret = new cv::ocl::Image2D(*reinterpret_cast<const cv::UMat*>(src), norm, alias);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_Image2D_Image2D_const_Image2DX(void* i) {
		try {
			cv::ocl::Image2D* ret = new cv::ocl::Image2D(*reinterpret_cast<const cv::ocl::Image2D*>(i));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_ocl_Image2D_canCreateAlias_const_UMatX(void* u) {
		try {
			bool ret = cv::ocl::Image2D::canCreateAlias(*reinterpret_cast<const cv::UMat*>(u));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ocl_Image2D_isFormatSupported_int_int_bool(int depth, int cn, bool norm) {
		try {
			bool ret = cv::ocl::Image2D::isFormatSupported(depth, cn, norm);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_ocl_Image2D_ptr_const(void* instance) {
		try {
			void* ret = reinterpret_cast<cv::ocl::Image2D*>(instance)->ptr();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Kernel_delete(cv::ocl::Kernel* instance) {
		delete instance;
	}
	Result<void*> cv_ocl_Kernel_Kernel() {
		try {
			cv::ocl::Kernel* ret = new cv::ocl::Kernel();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_Kernel_Kernel_const_charX_const_ProgramX(const char* kname, void* prog) {
		try {
			cv::ocl::Kernel* ret = new cv::ocl::Kernel(kname, *reinterpret_cast<const cv::ocl::Program*>(prog));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_Kernel_Kernel_const_charX_const_ProgramSourceX_const_StringX_StringX(const char* kname, void* prog, const char* buildopts, void** errmsg) {
		try {
			std::string errmsg_out;
			cv::ocl::Kernel* ret = new cv::ocl::Kernel(kname, *reinterpret_cast<const cv::ocl::ProgramSource*>(prog), std::string(buildopts), &errmsg_out);
			*errmsg = ocvrs_create_string(errmsg_out.c_str());
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_Kernel_Kernel_const_KernelX(void* k) {
		try {
			cv::ocl::Kernel* ret = new cv::ocl::Kernel(*reinterpret_cast<const cv::ocl::Kernel*>(k));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_ocl_Kernel_empty_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::ocl::Kernel*>(instance)->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ocl_Kernel_create_const_charX_const_ProgramX(void* instance, const char* kname, void* prog) {
		try {
			bool ret = reinterpret_cast<cv::ocl::Kernel*>(instance)->create(kname, *reinterpret_cast<const cv::ocl::Program*>(prog));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ocl_Kernel_create_const_charX_const_ProgramSourceX_const_StringX_StringX(void* instance, const char* kname, void* prog, const char* buildopts, void** errmsg) {
		try {
			std::string errmsg_out;
			bool ret = reinterpret_cast<cv::ocl::Kernel*>(instance)->create(kname, *reinterpret_cast<const cv::ocl::ProgramSource*>(prog), std::string(buildopts), &errmsg_out);
			*errmsg = ocvrs_create_string(errmsg_out.c_str());
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<int> cv_ocl_Kernel_set_int_const_voidX_size_t(void* instance, int i, const void* value, size_t sz) {
		try {
			int ret = reinterpret_cast<cv::ocl::Kernel*>(instance)->set(i, value, sz);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ocl_Kernel_set_int_const_Image2DX(void* instance, int i, void* image2D) {
		try {
			int ret = reinterpret_cast<cv::ocl::Kernel*>(instance)->set(i, *reinterpret_cast<const cv::ocl::Image2D*>(image2D));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ocl_Kernel_set_int_const_UMatX(void* instance, int i, void* m) {
		try {
			int ret = reinterpret_cast<cv::ocl::Kernel*>(instance)->set(i, *reinterpret_cast<const cv::UMat*>(m));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ocl_Kernel_set_int_const_KernelArgX(void* instance, int i, void* arg) {
		try {
			int ret = reinterpret_cast<cv::ocl::Kernel*>(instance)->set(i, *reinterpret_cast<const cv::ocl::KernelArg*>(arg));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<bool> cv_ocl_Kernel_run_int_size_tX_size_tX_bool_const_QueueX(void* instance, int dims, size_t* globalsize, size_t* localsize, bool sync, void* q) {
		try {
			bool ret = reinterpret_cast<cv::ocl::Kernel*>(instance)->run(dims, globalsize, localsize, sync, *reinterpret_cast<const cv::ocl::Queue*>(q));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ocl_Kernel_runTask_bool_const_QueueX(void* instance, bool sync, void* q) {
		try {
			bool ret = reinterpret_cast<cv::ocl::Kernel*>(instance)->runTask(sync, *reinterpret_cast<const cv::ocl::Queue*>(q));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<int64_t> cv_ocl_Kernel_runProfiling_int_size_tX_size_tX_const_QueueX(void* instance, int dims, size_t* globalsize, size_t* localsize, void* q) {
		try {
			int64_t ret = reinterpret_cast<cv::ocl::Kernel*>(instance)->runProfiling(dims, globalsize, localsize, *reinterpret_cast<const cv::ocl::Queue*>(q));
			return Ok<int64_t>(ret);
		} OCVRS_CATCH(Result<int64_t>)
	}
	
	Result<size_t> cv_ocl_Kernel_workGroupSize_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::ocl::Kernel*>(instance)->workGroupSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<size_t> cv_ocl_Kernel_preferedWorkGroupSizeMultiple_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::ocl::Kernel*>(instance)->preferedWorkGroupSizeMultiple();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<bool> cv_ocl_Kernel_compileWorkGroupSize_const_size_tX(void* instance, size_t* wsz) {
		try {
			bool ret = reinterpret_cast<cv::ocl::Kernel*>(instance)->compileWorkGroupSize(wsz);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<size_t> cv_ocl_Kernel_localMemSize_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::ocl::Kernel*>(instance)->localMemSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<void*> cv_ocl_Kernel_ptr_const(void* instance) {
		try {
			void* ret = reinterpret_cast<cv::ocl::Kernel*>(instance)->ptr();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_ocl_KernelArg_flags_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::KernelArg*>(instance)->flags;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ocl_KernelArg_setFlags_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::ocl::KernelArg*>(instance)->flags = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_ocl_KernelArg_m(void* instance) {
		try {
			cv::UMat* ret = reinterpret_cast<cv::ocl::KernelArg*>(instance)->m;
			return Ok<void*>(new cv::UMat*(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_ocl_KernelArg_setM_UMatX(void* instance, void* val) {
		try {
			reinterpret_cast<cv::ocl::KernelArg*>(instance)->m = reinterpret_cast<cv::UMat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<const void*> cv_ocl_KernelArg_obj_const(void* instance) {
		try {
			const void* ret = reinterpret_cast<cv::ocl::KernelArg*>(instance)->obj;
			return Ok<const void*>(ret);
		} OCVRS_CATCH(Result<const void*>)
	}
	
	Result<size_t> cv_ocl_KernelArg_sz_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::ocl::KernelArg*>(instance)->sz;
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result_void cv_ocl_KernelArg_setSz_size_t(void* instance, size_t val) {
		try {
			reinterpret_cast<cv::ocl::KernelArg*>(instance)->sz = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ocl_KernelArg_wscale_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::KernelArg*>(instance)->wscale;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ocl_KernelArg_setWscale_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::ocl::KernelArg*>(instance)->wscale = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ocl_KernelArg_iwscale_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::KernelArg*>(instance)->iwscale;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ocl_KernelArg_setIwscale_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::ocl::KernelArg*>(instance)->iwscale = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_KernelArg_delete(cv::ocl::KernelArg* instance) {
		delete instance;
	}
	Result<void*> cv_ocl_KernelArg_KernelArg_int_UMatX_int_int_const_voidX_size_t(int _flags, void* _m, int wscale, int iwscale, const void* _obj, size_t _sz) {
		try {
			cv::ocl::KernelArg* ret = new cv::ocl::KernelArg(_flags, reinterpret_cast<cv::UMat*>(_m), wscale, iwscale, _obj, _sz);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_KernelArg_KernelArg() {
		try {
			cv::ocl::KernelArg* ret = new cv::ocl::KernelArg();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_KernelArg_Local_size_t(size_t localMemSize) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::Local(localMemSize);
			return Ok<void*>(new cv::ocl::KernelArg(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_KernelArg_PtrWriteOnly_const_UMatX(void* m) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::PtrWriteOnly(*reinterpret_cast<const cv::UMat*>(m));
			return Ok<void*>(new cv::ocl::KernelArg(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_KernelArg_PtrReadOnly_const_UMatX(void* m) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::PtrReadOnly(*reinterpret_cast<const cv::UMat*>(m));
			return Ok<void*>(new cv::ocl::KernelArg(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_KernelArg_PtrReadWrite_const_UMatX(void* m) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::PtrReadWrite(*reinterpret_cast<const cv::UMat*>(m));
			return Ok<void*>(new cv::ocl::KernelArg(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_KernelArg_ReadWrite_const_UMatX_int_int(void* m, int wscale, int iwscale) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::ReadWrite(*reinterpret_cast<const cv::UMat*>(m), wscale, iwscale);
			return Ok<void*>(new cv::ocl::KernelArg(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_KernelArg_ReadWriteNoSize_const_UMatX_int_int(void* m, int wscale, int iwscale) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::ReadWriteNoSize(*reinterpret_cast<const cv::UMat*>(m), wscale, iwscale);
			return Ok<void*>(new cv::ocl::KernelArg(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_KernelArg_ReadOnly_const_UMatX_int_int(void* m, int wscale, int iwscale) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::ReadOnly(*reinterpret_cast<const cv::UMat*>(m), wscale, iwscale);
			return Ok<void*>(new cv::ocl::KernelArg(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_KernelArg_WriteOnly_const_UMatX_int_int(void* m, int wscale, int iwscale) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::WriteOnly(*reinterpret_cast<const cv::UMat*>(m), wscale, iwscale);
			return Ok<void*>(new cv::ocl::KernelArg(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_KernelArg_ReadOnlyNoSize_const_UMatX_int_int(void* m, int wscale, int iwscale) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::ReadOnlyNoSize(*reinterpret_cast<const cv::UMat*>(m), wscale, iwscale);
			return Ok<void*>(new cv::ocl::KernelArg(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_KernelArg_WriteOnlyNoSize_const_UMatX_int_int(void* m, int wscale, int iwscale) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::WriteOnlyNoSize(*reinterpret_cast<const cv::UMat*>(m), wscale, iwscale);
			return Ok<void*>(new cv::ocl::KernelArg(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_KernelArg_Constant_const_MatX(void* m) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::Constant(*reinterpret_cast<const cv::Mat*>(m));
			return Ok<void*>(new cv::ocl::KernelArg(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Platform_delete(cv::ocl::Platform* instance) {
		delete instance;
	}
	Result<void*> cv_ocl_Platform_Platform() {
		try {
			cv::ocl::Platform* ret = new cv::ocl::Platform();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_Platform_Platform_const_PlatformX(void* p) {
		try {
			cv::ocl::Platform* ret = new cv::ocl::Platform(*reinterpret_cast<const cv::ocl::Platform*>(p));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_Platform_ptr_const(void* instance) {
		try {
			void* ret = reinterpret_cast<cv::ocl::Platform*>(instance)->ptr();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_Platform_getDefault() {
		try {
			cv::ocl::Platform ret = cv::ocl::Platform::getDefault();
			return Ok<void*>(new cv::ocl::Platform(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_PlatformInfo_delete(cv::ocl::PlatformInfo* instance) {
		delete instance;
	}
	Result<void*> cv_ocl_PlatformInfo_PlatformInfo() {
		try {
			cv::ocl::PlatformInfo* ret = new cv::ocl::PlatformInfo();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_PlatformInfo_PlatformInfo_voidX(void* id) {
		try {
			cv::ocl::PlatformInfo* ret = new cv::ocl::PlatformInfo(id);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_PlatformInfo_PlatformInfo_const_PlatformInfoX(void* i) {
		try {
			cv::ocl::PlatformInfo* ret = new cv::ocl::PlatformInfo(*reinterpret_cast<const cv::ocl::PlatformInfo*>(i));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_PlatformInfo_name_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::ocl::PlatformInfo*>(instance)->name();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_PlatformInfo_vendor_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::ocl::PlatformInfo*>(instance)->vendor();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_PlatformInfo_version_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::ocl::PlatformInfo*>(instance)->version();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_ocl_PlatformInfo_deviceNumber_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ocl::PlatformInfo*>(instance)->deviceNumber();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ocl_PlatformInfo_getDevice_const_DeviceX_int(void* instance, void* device, int d) {
		try {
			reinterpret_cast<cv::ocl::PlatformInfo*>(instance)->getDevice(*reinterpret_cast<cv::ocl::Device*>(device), d);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Program_delete(cv::ocl::Program* instance) {
		delete instance;
	}
	Result<void*> cv_ocl_Program_Program() {
		try {
			cv::ocl::Program* ret = new cv::ocl::Program();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_Program_Program_const_ProgramSourceX_const_StringX_StringX(void* src, const char* buildflags, void** errmsg) {
		try {
			std::string errmsg_out;
			cv::ocl::Program* ret = new cv::ocl::Program(*reinterpret_cast<const cv::ocl::ProgramSource*>(src), std::string(buildflags), errmsg_out);
			*errmsg = ocvrs_create_string(errmsg_out.c_str());
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_Program_Program_const_ProgramX(void* prog) {
		try {
			cv::ocl::Program* ret = new cv::ocl::Program(*reinterpret_cast<const cv::ocl::Program*>(prog));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_ocl_Program_create_const_ProgramSourceX_const_StringX_StringX(void* instance, void* src, const char* buildflags, void** errmsg) {
		try {
			std::string errmsg_out;
			bool ret = reinterpret_cast<cv::ocl::Program*>(instance)->create(*reinterpret_cast<const cv::ocl::ProgramSource*>(src), std::string(buildflags), errmsg_out);
			*errmsg = ocvrs_create_string(errmsg_out.c_str());
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_ocl_Program_ptr_const(void* instance) {
		try {
			void* ret = reinterpret_cast<cv::ocl::Program*>(instance)->ptr();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_ocl_Program_getBinary_const_vector_char_X(void* instance, void* binary) {
		try {
			reinterpret_cast<cv::ocl::Program*>(instance)->getBinary(*reinterpret_cast<std::vector<char>*>(binary));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_ocl_Program_read_const_StringX_const_StringX(void* instance, const char* buf, const char* buildflags) {
		try {
			bool ret = reinterpret_cast<cv::ocl::Program*>(instance)->read(std::string(buf), std::string(buildflags));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ocl_Program_write_const_StringX(void* instance, void** buf) {
		try {
			std::string buf_out;
			bool ret = reinterpret_cast<cv::ocl::Program*>(instance)->write(buf_out);
			*buf = ocvrs_create_string(buf_out.c_str());
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_ocl_Program_source_const(void* instance) {
		try {
			cv::ocl::ProgramSource ret = reinterpret_cast<cv::ocl::Program*>(instance)->source();
			return Ok<void*>(new cv::ocl::ProgramSource(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_Program_getPrefix_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::ocl::Program*>(instance)->getPrefix();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_Program_getPrefix_const_StringX(const char* buildflags) {
		try {
			cv::String ret = cv::ocl::Program::getPrefix(std::string(buildflags));
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_ProgramSource_delete(cv::ocl::ProgramSource* instance) {
		delete instance;
	}
	Result<void*> cv_ocl_ProgramSource_ProgramSource() {
		try {
			cv::ocl::ProgramSource* ret = new cv::ocl::ProgramSource();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_ProgramSource_ProgramSource_const_StringX_const_StringX_const_StringX_const_StringX(const char* module, const char* name, const char* codeStr, const char* codeHash) {
		try {
			cv::ocl::ProgramSource* ret = new cv::ocl::ProgramSource(std::string(module), std::string(name), std::string(codeStr), std::string(codeHash));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_ProgramSource_ProgramSource_const_StringX(const char* prog) {
		try {
			cv::ocl::ProgramSource* ret = new cv::ocl::ProgramSource(std::string(prog));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_ProgramSource_ProgramSource_const_ProgramSourceX(void* prog) {
		try {
			cv::ocl::ProgramSource* ret = new cv::ocl::ProgramSource(*reinterpret_cast<const cv::ocl::ProgramSource*>(prog));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_ProgramSource_source_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::ocl::ProgramSource*>(instance)->source();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::ocl::ProgramSource::hash_t> cv_ocl_ProgramSource_hash_const(void* instance) {
		try {
			cv::ocl::ProgramSource::hash_t ret = reinterpret_cast<cv::ocl::ProgramSource*>(instance)->hash();
			return Ok<cv::ocl::ProgramSource::hash_t>(ret);
		} OCVRS_CATCH(Result<cv::ocl::ProgramSource::hash_t>)
	}
	
	Result<void*> cv_ocl_ProgramSource_fromBinary_const_StringX_const_StringX_const_unsigned_charX_size_t_const_StringX(const char* module, const char* name, const unsigned char* binary, size_t size, const char* buildOptions) {
		try {
			cv::ocl::ProgramSource ret = cv::ocl::ProgramSource::fromBinary(std::string(module), std::string(name), binary, size, std::string(buildOptions));
			return Ok<void*>(new cv::ocl::ProgramSource(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_ProgramSource_fromSPIR_const_StringX_const_StringX_const_unsigned_charX_size_t_const_StringX(const char* module, const char* name, const unsigned char* binary, size_t size, const char* buildOptions) {
		try {
			cv::ocl::ProgramSource ret = cv::ocl::ProgramSource::fromSPIR(std::string(module), std::string(name), binary, size, std::string(buildOptions));
			return Ok<void*>(new cv::ocl::ProgramSource(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Queue_delete(cv::ocl::Queue* instance) {
		delete instance;
	}
	Result<void*> cv_ocl_Queue_Queue() {
		try {
			cv::ocl::Queue* ret = new cv::ocl::Queue();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_Queue_Queue_const_ContextX_const_DeviceX(void* c, void* d) {
		try {
			cv::ocl::Queue* ret = new cv::ocl::Queue(*reinterpret_cast<const cv::ocl::Context*>(c), *reinterpret_cast<const cv::ocl::Device*>(d));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_Queue_Queue_const_QueueX(void* q) {
		try {
			cv::ocl::Queue* ret = new cv::ocl::Queue(*reinterpret_cast<const cv::ocl::Queue*>(q));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_ocl_Queue_create_const_ContextX_const_DeviceX(void* instance, void* c, void* d) {
		try {
			bool ret = reinterpret_cast<cv::ocl::Queue*>(instance)->create(*reinterpret_cast<const cv::ocl::Context*>(c), *reinterpret_cast<const cv::ocl::Device*>(d));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_ocl_Queue_finish(void* instance) {
		try {
			reinterpret_cast<cv::ocl::Queue*>(instance)->finish();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_ocl_Queue_ptr_const(void* instance) {
		try {
			void* ret = reinterpret_cast<cv::ocl::Queue*>(instance)->ptr();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_Queue_getDefault() {
		try {
			cv::ocl::Queue ret = cv::ocl::Queue::getDefault();
			return Ok<void*>(new cv::ocl::Queue(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ocl_Queue_getProfilingQueue_const(void* instance) {
		try {
			cv::ocl::Queue ret = reinterpret_cast<cv::ocl::Queue*>(instance)->getProfilingQueue();
			return Ok<void*>(new cv::ocl::Queue(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Timer_delete(cv::ocl::Timer* instance) {
		delete instance;
	}
	Result<void*> cv_ocl_Timer_Timer_const_QueueX(void* q) {
		try {
			cv::ocl::Timer* ret = new cv::ocl::Timer(*reinterpret_cast<const cv::ocl::Queue*>(q));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_ocl_Timer_start(void* instance) {
		try {
			reinterpret_cast<cv::ocl::Timer*>(instance)->start();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ocl_Timer_stop(void* instance) {
		try {
			reinterpret_cast<cv::ocl::Timer*>(instance)->stop();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<uint64_t> cv_ocl_Timer_durationNS_const(void* instance) {
		try {
			uint64_t ret = reinterpret_cast<cv::ocl::Timer*>(instance)->durationNS();
			return Ok<uint64_t>(ret);
		} OCVRS_CATCH(Result<uint64_t>)
	}
	
	Result<void*> cv_utils_logging_LogTag_name_const(void* instance) {
		try {
			const char* ret = reinterpret_cast<cv::utils::logging::LogTag*>(instance)->name;
			return Ok<void*>(ocvrs_create_string(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::utils::logging::LogLevel> cv_utils_logging_LogTag_level_const(void* instance) {
		try {
			cv::utils::logging::LogLevel ret = reinterpret_cast<cv::utils::logging::LogTag*>(instance)->level;
			return Ok<cv::utils::logging::LogLevel>(ret);
		} OCVRS_CATCH(Result<cv::utils::logging::LogLevel>)
	}
	
	Result_void cv_utils_logging_LogTag_setLevel_LogLevel(void* instance, cv::utils::logging::LogLevel val) {
		try {
			reinterpret_cast<cv::utils::logging::LogTag*>(instance)->level = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_LogTag_delete(cv::utils::logging::LogTag* instance) {
		delete instance;
	}
	Result<void*> cv_utils_logging_LogTag_LogTag_const_charX_LogLevel(const char* _name, cv::utils::logging::LogLevel _level) {
		try {
			cv::utils::logging::LogTag* ret = new cv::utils::logging::LogTag(_name, _level);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
}
