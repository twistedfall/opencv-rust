#include "core.hpp"
#include "core_types.hpp"

extern "C" {
	// Cholesky(double *, size_t, int, double *, size_t, int)(Indirect, Primitive, Primitive, Indirect, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/base.hpp:607
	// ("cv::Cholesky", vec![(pred!(mut, ["A", "astep", "m", "b", "bstep", "n"], ["double*", "size_t", "int", "double*", "size_t", "int"]), _)]),
	void cv_Cholesky_doubleX_size_t_int_doubleX_size_t_int(double* A, size_t astep, int m, double* b, size_t bstep, int n, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::Cholesky(A, astep, m, b, bstep, n);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Cholesky(float *, size_t, int, float *, size_t, int)(Indirect, Primitive, Primitive, Indirect, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/base.hpp:605
	// ("cv::Cholesky", vec![(pred!(mut, ["A", "astep", "m", "b", "bstep", "n"], ["float*", "size_t", "int", "float*", "size_t", "int"]), _)]),
	void cv_Cholesky_floatX_size_t_int_floatX_size_t_int(float* A, size_t astep, int m, float* b, size_t bstep, int n, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::Cholesky(A, astep, m, b, bstep, n);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// LUT(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:574
	// ("cv::LUT", vec![(pred!(mut, ["src", "lut", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_LUT_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_InputArray* lut, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::LUT(*src, *lut, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// LU(double *, size_t, int, double *, size_t, int)(Indirect, Primitive, Primitive, Indirect, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/base.hpp:603
	// ("cv::LU", vec![(pred!(mut, ["A", "astep", "m", "b", "bstep", "n"], ["double*", "size_t", "int", "double*", "size_t", "int"]), _)]),
	void cv_LU_doubleX_size_t_int_doubleX_size_t_int(double* A, size_t astep, int m, double* b, size_t bstep, int n, Result<int>* ocvrs_return) {
		try {
			int ret = cv::LU(A, astep, m, b, bstep, n);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// LU(float *, size_t, int, float *, size_t, int)(Indirect, Primitive, Primitive, Indirect, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/base.hpp:601
	// ("cv::LU", vec![(pred!(mut, ["A", "astep", "m", "b", "bstep", "n"], ["float*", "size_t", "int", "float*", "size_t", "int"]), _)]),
	void cv_LU_floatX_size_t_int_floatX_size_t_int(float* A, size_t astep, int m, float* b, size_t bstep, int n, Result<int>* ocvrs_return) {
		try {
			int ret = cv::LU(A, astep, m, b, bstep, n);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Mahalanobis(InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2037
	// ("cv::Mahalanobis", vec![(pred!(mut, ["v1", "v2", "icovar"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_Mahalanobis_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* v1, const cv::_InputArray* v2, const cv::_InputArray* icovar, Result<double>* ocvrs_return) {
		try {
			double ret = cv::Mahalanobis(*v1, *v2, *icovar);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// PCABackProject(InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2017
	// ("cv::PCABackProject", vec![(pred!(mut, ["data", "mean", "eigenvectors", "result"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_PCABackProject_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* data, const cv::_InputArray* mean, const cv::_InputArray* eigenvectors, const cv::_OutputArray* result, ResultVoid* ocvrs_return) {
		try {
			cv::PCABackProject(*data, *mean, *eigenvectors, *result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::PCACompute(InputArray, InputOutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1995
	// ("cv::PCACompute", vec![(pred!(mut, ["data", "mean", "eigenvectors"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR(const cv::_InputArray* data, const cv::_InputOutputArray* mean, const cv::_OutputArray* eigenvectors, ResultVoid* ocvrs_return) {
		try {
			cv::PCACompute(*data, *mean, *eigenvectors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::PCACompute(InputArray, InputOutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1999
	// ("cv::PCACompute", vec![(pred!(mut, ["data", "mean", "eigenvectors", "eigenvalues"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* data, const cv::_InputOutputArray* mean, const cv::_OutputArray* eigenvectors, const cv::_OutputArray* eigenvalues, ResultVoid* ocvrs_return) {
		try {
			cv::PCACompute(*data, *mean, *eigenvectors, *eigenvalues);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// PCACompute(InputArray, InputOutputArray, OutputArray, OutputArray, double)(InputArray, InputOutputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2008
	// ("cv::PCACompute", vec![(pred!(mut, ["data", "mean", "eigenvectors", "eigenvalues", "retainedVariance"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double"]), _)]),
	void cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_double(const cv::_InputArray* data, const cv::_InputOutputArray* mean, const cv::_OutputArray* eigenvectors, const cv::_OutputArray* eigenvalues, double retainedVariance, ResultVoid* ocvrs_return) {
		try {
			cv::PCACompute(*data, *mean, *eigenvectors, *eigenvalues, retainedVariance);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// PCACompute(InputArray, InputOutputArray, OutputArray, OutputArray, int)(InputArray, InputOutputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1999
	// ("cv::PCACompute", vec![(pred!(mut, ["data", "mean", "eigenvectors", "eigenvalues", "maxComponents"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int(const cv::_InputArray* data, const cv::_InputOutputArray* mean, const cv::_OutputArray* eigenvectors, const cv::_OutputArray* eigenvalues, int maxComponents, ResultVoid* ocvrs_return) {
		try {
			cv::PCACompute(*data, *mean, *eigenvectors, *eigenvalues, maxComponents);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// PCACompute(InputArray, InputOutputArray, OutputArray, double)(InputArray, InputOutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2004
	// ("cv::PCACompute", vec![(pred!(mut, ["data", "mean", "eigenvectors", "retainedVariance"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "double"]), _)]),
	void cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_double(const cv::_InputArray* data, const cv::_InputOutputArray* mean, const cv::_OutputArray* eigenvectors, double retainedVariance, ResultVoid* ocvrs_return) {
		try {
			cv::PCACompute(*data, *mean, *eigenvectors, retainedVariance);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// PCACompute(InputArray, InputOutputArray, OutputArray, int)(InputArray, InputOutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1995
	// ("cv::PCACompute", vec![(pred!(mut, ["data", "mean", "eigenvectors", "maxComponents"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_int(const cv::_InputArray* data, const cv::_InputOutputArray* mean, const cv::_OutputArray* eigenvectors, int maxComponents, ResultVoid* ocvrs_return) {
		try {
			cv::PCACompute(*data, *mean, *eigenvectors, maxComponents);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// PCAProject(InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2013
	// ("cv::PCAProject", vec![(pred!(mut, ["data", "mean", "eigenvectors", "result"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_PCAProject_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* data, const cv::_InputArray* mean, const cv::_InputArray* eigenvectors, const cv::_OutputArray* result, ResultVoid* ocvrs_return) {
		try {
			cv::PCAProject(*data, *mean, *eigenvectors, *result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// PSNR(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:731
	// ("cv::PSNR", vec![(pred!(mut, ["src1", "src2"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_PSNR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, Result<double>* ocvrs_return) {
		try {
			double ret = cv::PSNR(*src1, *src2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// SVBackSubst(InputArray, InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2024
	// ("cv::SVBackSubst", vec![(pred!(mut, ["w", "u", "vt", "rhs", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_SVBackSubst_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* w, const cv::_InputArray* u, const cv::_InputArray* vt, const cv::_InputArray* rhs, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::SVBackSubst(*w, *u, *vt, *rhs, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SVDecomp(InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2021
	// ("cv::SVDecomp", vec![(pred!(mut, ["src", "w", "u", "vt"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_SVDecomp_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* w, const cv::_OutputArray* u, const cv::_OutputArray* vt, ResultVoid* ocvrs_return) {
		try {
			cv::SVDecomp(*src, *w, *u, *vt);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// SVDecomp(InputArray, OutputArray, OutputArray, OutputArray, int)(InputArray, OutputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2021
	// ("cv::SVDecomp", vec![(pred!(mut, ["src", "w", "u", "vt", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_SVDecomp_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* w, const cv::_OutputArray* u, const cv::_OutputArray* vt, int flags, ResultVoid* ocvrs_return) {
		try {
			cv::SVDecomp(*src, *w, *u, *vt, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// abs(const MatExpr &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3789
	// ("cv::abs", vec![(pred!(mut, ["e"], ["const cv::MatExpr*"]), _)]),
	void cv_abs_const_MatExprR(const cv::MatExpr* e, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::abs(*e);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// abs(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3785
	// ("cv::abs", vec![(pred!(mut, ["m"], ["const cv::Mat*"]), _)]),
	void cv_abs_const_MatR(const cv::Mat* m, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::abs(*m);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// absdiff(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1372
	// ("cv::absdiff", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_absdiff_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::absdiff(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::addWeighted(InputArray, Primitive, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:517
	// ("cv::addWeighted", vec![(pred!(mut, ["src1", "alpha", "src2", "beta", "gamma", "dst"], ["const cv::_InputArray*", "double", "const cv::_InputArray*", "double", "double", "const cv::_OutputArray*"]), _)]),
	void cv_addWeighted_const__InputArrayR_double_const__InputArrayR_double_double_const__OutputArrayR(const cv::_InputArray* src1, double alpha, const cv::_InputArray* src2, double beta, double gamma, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::addWeighted(*src1, alpha, *src2, beta, gamma, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addWeighted(InputArray, double, InputArray, double, double, OutputArray, int)(InputArray, Primitive, InputArray, Primitive, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:517
	// ("cv::addWeighted", vec![(pred!(mut, ["src1", "alpha", "src2", "beta", "gamma", "dst", "dtype"], ["const cv::_InputArray*", "double", "const cv::_InputArray*", "double", "double", "const cv::_OutputArray*", "int"]), _)]),
	void cv_addWeighted_const__InputArrayR_double_const__InputArrayR_double_double_const__OutputArrayR_int(const cv::_InputArray* src1, double alpha, const cv::_InputArray* src2, double beta, double gamma, const cv::_OutputArray* dst, int dtype, ResultVoid* ocvrs_return) {
		try {
			cv::addWeighted(*src1, alpha, *src2, beta, gamma, *dst, dtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::add(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:376
	// ("cv::add", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_add_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::add(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// add(InputArray, InputArray, OutputArray, InputArray, int)(InputArray, InputArray, OutputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:376
	// ("cv::add", vec![(pred!(mut, ["src1", "src2", "dst", "mask", "dtype"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "int"]), _)]),
	void cv_add_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_int(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, const cv::_InputArray* mask, int dtype, ResultVoid* ocvrs_return) {
		try {
			cv::add(*src1, *src2, *dst, *mask, dtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::batchDistance(InputArray, InputArray, OutputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:738
	// ("cv::batchDistance", vec![(pred!(mut, ["src1", "src2", "dist", "dtype", "nidx"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "const cv::_OutputArray*"]), _)]),
	void cv_batchDistance_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dist, int dtype, const cv::_OutputArray* nidx, ResultVoid* ocvrs_return) {
		try {
			cv::batchDistance(*src1, *src2, *dist, dtype, *nidx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// batchDistance(InputArray, InputArray, OutputArray, int, OutputArray, int, int, InputArray, int, bool)(InputArray, InputArray, OutputArray, Primitive, OutputArray, Primitive, Primitive, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:738
	// ("cv::batchDistance", vec![(pred!(mut, ["src1", "src2", "dist", "dtype", "nidx", "normType", "K", "mask", "update", "crosscheck"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "const cv::_OutputArray*", "int", "int", "const cv::_InputArray*", "int", "bool"]), _)]),
	void cv_batchDistance_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_const__OutputArrayR_int_int_const__InputArrayR_int_bool(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dist, int dtype, const cv::_OutputArray* nidx, int normType, int K, const cv::_InputArray* mask, int update, bool crosscheck, ResultVoid* ocvrs_return) {
		try {
			cv::batchDistance(*src1, *src2, *dist, dtype, *nidx, normType, K, *mask, update, crosscheck);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bitwise_and(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1274
	// ("cv::bitwise_and", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_bitwise_and_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::bitwise_and(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bitwise_and(InputArray, InputArray, OutputArray, InputArray)(InputArray, InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1274
	// ("cv::bitwise_and", vec![(pred!(mut, ["src1", "src2", "dst", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_bitwise_and_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			cv::bitwise_and(*src1, *src2, *dst, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bitwise_not(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1346
	// ("cv::bitwise_not", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_bitwise_not_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::bitwise_not(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bitwise_not(InputArray, OutputArray, InputArray)(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1346
	// ("cv::bitwise_not", vec![(pred!(mut, ["src", "dst", "mask"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_bitwise_not_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			cv::bitwise_not(*src, *dst, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bitwise_or(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1301
	// ("cv::bitwise_or", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_bitwise_or_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::bitwise_or(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bitwise_or(InputArray, InputArray, OutputArray, InputArray)(InputArray, InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1301
	// ("cv::bitwise_or", vec![(pred!(mut, ["src1", "src2", "dst", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_bitwise_or_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			cv::bitwise_or(*src1, *src2, *dst, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bitwise_xor(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1329
	// ("cv::bitwise_xor", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_bitwise_xor_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::bitwise_xor(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bitwise_xor(InputArray, InputArray, OutputArray, InputArray)(InputArray, InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1329
	// ("cv::bitwise_xor", vec![(pred!(mut, ["src1", "src2", "dst", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_bitwise_xor_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			cv::bitwise_xor(*src1, *src2, *dst, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// borderInterpolate(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:286
	// ("cv::borderInterpolate", vec![(pred!(mut, ["p", "len", "borderType"], ["int", "int", "int"]), _)]),
	void cv_borderInterpolate_int_int_int(int p, int len, int borderType, Result<int>* ocvrs_return) {
		try {
			int ret = cv::borderInterpolate(p, len, borderType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::calcCovarMatrix(InputArray, OutputArray, InputOutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1991
	// ("cv::calcCovarMatrix", vec![(pred!(mut, ["samples", "covar", "mean", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "int"]), _)]),
	void cv_calcCovarMatrix_const__InputArrayR_const__OutputArrayR_const__InputOutputArrayR_int(const cv::_InputArray* samples, const cv::_OutputArray* covar, const cv::_InputOutputArray* mean, int flags, ResultVoid* ocvrs_return) {
		try {
			cv::calcCovarMatrix(*samples, *covar, *mean, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calcCovarMatrix(InputArray, OutputArray, InputOutputArray, int, int)(InputArray, OutputArray, InputOutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1991
	// ("cv::calcCovarMatrix", vec![(pred!(mut, ["samples", "covar", "mean", "flags", "ctype"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "int", "int"]), _)]),
	void cv_calcCovarMatrix_const__InputArrayR_const__OutputArrayR_const__InputOutputArrayR_int_int(const cv::_InputArray* samples, const cv::_OutputArray* covar, const cv::_InputOutputArray* mean, int flags, int ctype, ResultVoid* ocvrs_return) {
		try {
			cv::calcCovarMatrix(*samples, *covar, *mean, flags, ctype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cartToPolar(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1572
	// ("cv::cartToPolar", vec![(pred!(mut, ["x", "y", "magnitude", "angle"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cartToPolar_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* x, const cv::_InputArray* y, const cv::_OutputArray* magnitude, const cv::_OutputArray* angle, ResultVoid* ocvrs_return) {
		try {
			cv::cartToPolar(*x, *y, *magnitude, *angle);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cartToPolar(InputArray, InputArray, OutputArray, OutputArray, bool)(InputArray, InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1572
	// ("cv::cartToPolar", vec![(pred!(mut, ["x", "y", "magnitude", "angle", "angleInDegrees"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool"]), _)]),
	void cv_cartToPolar_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool(const cv::_InputArray* x, const cv::_InputArray* y, const cv::_OutputArray* magnitude, const cv::_OutputArray* angle, bool angleInDegrees, ResultVoid* ocvrs_return) {
		try {
			cv::cartToPolar(*x, *y, *magnitude, *angle, angleInDegrees);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// checkHardwareSupport(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:423
	// ("cv::checkHardwareSupport", vec![(pred!(mut, ["feature"], ["int"]), _)]),
	void cv_checkHardwareSupport_int(int feature, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::checkHardwareSupport(feature);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::checkRange(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1623
	// ("cv::checkRange", vec![(pred!(mut, ["a"], ["const cv::_InputArray*"]), _)]),
	void cv_checkRange_const__InputArrayR(const cv::_InputArray* a, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::checkRange(*a);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// checkRange(InputArray, bool, Point *, double, double)(InputArray, Primitive, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1623
	// ("cv::checkRange", vec![(pred!(mut, ["a", "quiet", "pos", "minVal", "maxVal"], ["const cv::_InputArray*", "bool", "cv::Point*", "double", "double"]), _)]),
	void cv_checkRange_const__InputArrayR_bool_PointX_double_double(const cv::_InputArray* a, bool quiet, cv::Point* pos, double minVal, double maxVal, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::checkRange(*a, quiet, pos, minVal, maxVal);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compare(InputArray, InputArray, OutputArray, int)(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1422
	// ("cv::compare", vec![(pred!(mut, ["src1", "src2", "dst", "cmpop"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_compare_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, int cmpop, ResultVoid* ocvrs_return) {
		try {
			cv::compare(*src1, *src2, *dst, cmpop);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::completeSymm(InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1773
	// ("cv::completeSymm", vec![(pred!(mut, ["m"], ["const cv::_InputOutputArray*"]), _)]),
	void cv_completeSymm_const__InputOutputArrayR(const cv::_InputOutputArray* m, ResultVoid* ocvrs_return) {
		try {
			cv::completeSymm(*m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// completeSymm(InputOutputArray, bool)(InputOutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1773
	// ("cv::completeSymm", vec![(pred!(mut, ["m", "lowerToUpper"], ["const cv::_InputOutputArray*", "bool"]), _)]),
	void cv_completeSymm_const__InputOutputArrayR_bool(const cv::_InputOutputArray* m, bool lowerToUpper, ResultVoid* ocvrs_return) {
		try {
			cv::completeSymm(*m, lowerToUpper);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertFp16(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:558
	// ("cv::convertFp16", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_convertFp16_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::convertFp16(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::convertScaleAbs(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:545
	// ("cv::convertScaleAbs", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_convertScaleAbs_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::convertScaleAbs(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertScaleAbs(InputArray, OutputArray, double, double)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:545
	// ("cv::convertScaleAbs", vec![(pred!(mut, ["src", "dst", "alpha", "beta"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
	void cv_convertScaleAbs_const__InputArrayR_const__OutputArrayR_double_double(const cv::_InputArray* src, const cv::_OutputArray* dst, double alpha, double beta, ResultVoid* ocvrs_return) {
		try {
			cv::convertScaleAbs(*src, *dst, alpha, beta);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::copyMakeBorder(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:336
	// ("cv::copyMakeBorder", vec![(pred!(mut, ["src", "dst", "top", "bottom", "left", "right", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int", "int", "int"]), _)]),
	void cv_copyMakeBorder_const__InputArrayR_const__OutputArrayR_int_int_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int top, int bottom, int left, int right, int borderType, ResultVoid* ocvrs_return) {
		try {
			cv::copyMakeBorder(*src, *dst, top, bottom, left, right, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// copyMakeBorder(InputArray, OutputArray, int, int, int, int, int, const Scalar &)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:336
	// ("cv::copyMakeBorder", vec![(pred!(mut, ["src", "dst", "top", "bottom", "left", "right", "borderType", "value"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int", "int", "int", "const cv::Scalar*"]), _)]),
	void cv_copyMakeBorder_const__InputArrayR_const__OutputArrayR_int_int_int_int_int_const_ScalarR(const cv::_InputArray* src, const cv::_OutputArray* dst, int top, int bottom, int left, int right, int borderType, const cv::Scalar* value, ResultVoid* ocvrs_return) {
		try {
			cv::copyMakeBorder(*src, *dst, top, bottom, left, right, borderType, *value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// countNonZero(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:592
	// ("cv::countNonZero", vec![(pred!(mut, ["src"], ["const cv::_InputArray*"]), _)]),
	void cv_countNonZero_const__InputArrayR(const cv::_InputArray* src, Result<int>* ocvrs_return) {
		try {
			int ret = cv::countNonZero(*src);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cubeRoot(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/base.hpp:574
	// ("cv::cubeRoot", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_cubeRoot_float(float val, Result<float>* ocvrs_return) {
		try {
			float ret = cv::cubeRoot(val);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createContinuous(int, int, int, OutputArray)(Primitive, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:351
	// ("cv::cuda::createContinuous", vec![(pred!(mut, ["rows", "cols", "type", "arr"], ["int", "int", "int", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_createContinuous_int_int_int_const__OutputArrayR(int rows, int cols, int type, const cv::_OutputArray* arr, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::createContinuous(rows, cols, type, *arr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// deviceSupports(FeatureSet)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:796
	// ("cv::cuda::deviceSupports", vec![(pred!(mut, ["feature_set"], ["cv::cuda::FeatureSet"]), _)]),
	void cv_cuda_deviceSupports_FeatureSet(cv::cuda::FeatureSet feature_set, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::cuda::deviceSupports(feature_set);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ensureSizeIsEnough(int, int, int, OutputArray)(Primitive, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:362
	// ("cv::cuda::ensureSizeIsEnough", vec![(pred!(mut, ["rows", "cols", "type", "arr"], ["int", "int", "int", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_ensureSizeIsEnough_int_int_int_const__OutputArrayR(int rows, int cols, int type, const cv::_OutputArray* arr, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::ensureSizeIsEnough(rows, cols, type, *arr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCudaEnabledDeviceCount()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:752
	// ("cv::cuda::getCudaEnabledDeviceCount", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_getCudaEnabledDeviceCount(Result<int>* ocvrs_return) {
		try {
			int ret = cv::cuda::getCudaEnabledDeviceCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDevice()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:764
	// ("cv::cuda::getDevice", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_getDevice(Result<int>* ocvrs_return) {
		try {
			int ret = cv::cuda::getDevice();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// printCudaDeviceInfo(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:1038
	// ("cv::cuda::printCudaDeviceInfo", vec![(pred!(mut, ["device"], ["int"]), _)]),
	void cv_cuda_printCudaDeviceInfo_int(int device, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::printCudaDeviceInfo(device);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// printShortCudaDeviceInfo(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:1039
	// ("cv::cuda::printShortCudaDeviceInfo", vec![(pred!(mut, ["device"], ["int"]), _)]),
	void cv_cuda_printShortCudaDeviceInfo_int(int device, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::printShortCudaDeviceInfo(device);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// registerPageLocked(Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:611
	// ("cv::cuda::registerPageLocked", vec![(pred!(mut, ["m"], ["cv::Mat*"]), _)]),
	void cv_cuda_registerPageLocked_MatR(cv::Mat* m, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::registerPageLocked(*m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// resetDevice()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:771
	// ("cv::cuda::resetDevice", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_resetDevice(ResultVoid* ocvrs_return) {
		try {
			cv::cuda::resetDevice();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBufferPoolConfig(int, size_t, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:511
	// ("cv::cuda::setBufferPoolConfig", vec![(pred!(mut, ["deviceId", "stackSize", "stackCount"], ["int", "size_t", "int"]), _)]),
	void cv_cuda_setBufferPoolConfig_int_size_t_int(int deviceId, size_t stackSize, int stackCount, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::setBufferPoolConfig(deviceId, stackSize, stackCount);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBufferPoolUsage(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:510
	// ("cv::cuda::setBufferPoolUsage", vec![(pred!(mut, ["on"], ["bool"]), _)]),
	void cv_cuda_setBufferPoolUsage_bool(bool on, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::setBufferPoolUsage(on);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDevice(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:760
	// ("cv::cuda::setDevice", vec![(pred!(mut, ["device"], ["int"]), _)]),
	void cv_cuda_setDevice_int(int device, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::setDevice(device);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::setGlDevice() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:572
	// ("cv::cuda::setGlDevice", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_setGlDevice(ResultVoid* ocvrs_return) {
		try {
			cv::cuda::setGlDevice();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setGlDevice(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:572
	// ("cv::cuda::setGlDevice", vec![(pred!(mut, ["device"], ["int"]), _)]),
	void cv_cuda_setGlDevice_int(int device, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::setGlDevice(device);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// unregisterPageLocked(Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:617
	// ("cv::cuda::unregisterPageLocked", vec![(pred!(mut, ["m"], ["cv::Mat*"]), _)]),
	void cv_cuda_unregisterPageLocked_MatR(cv::Mat* m, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::unregisterPageLocked(*m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dct(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2226
	// ("cv::dct", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_dct_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::dct(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dct(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2226
	// ("cv::dct", vec![(pred!(mut, ["src", "dst", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_dct_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int flags, ResultVoid* ocvrs_return) {
		try {
			cv::dct(*src, *dst, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// depthToString(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/check.hpp:13
	// ("cv::depthToString", vec![(pred!(mut, ["depth"], ["int"]), _)]),
	void cv_depthToString_int(int depth, Result<void*>* ocvrs_return) {
		try {
			const char* ret = cv::depthToString(depth);
			Ok(ocvrs_create_string(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// check_failed_MatChannels(const int, const CheckContext &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/check.hpp:89
	// ("cv::detail::check_failed_MatChannels", vec![(pred!(mut, ["v", "ctx"], ["const int", "const cv::detail::CheckContext*"]), _)]),
	void cv_detail_check_failed_MatChannels_const_int_const_CheckContextR(const int v, const cv::detail::CheckContext* ctx, ResultVoid* ocvrs_return) {
		try {
			cv::detail::check_failed_MatChannels(v, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// check_failed_MatChannels(const int, const int, const CheckContext &)(Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/check.hpp:76
	// ("cv::detail::check_failed_MatChannels", vec![(pred!(mut, ["v1", "v2", "ctx"], ["const int", "const int", "const cv::detail::CheckContext*"]), _)]),
	void cv_detail_check_failed_MatChannels_const_int_const_int_const_CheckContextR(const int v1, const int v2, const cv::detail::CheckContext* ctx, ResultVoid* ocvrs_return) {
		try {
			cv::detail::check_failed_MatChannels(v1, v2, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// check_failed_MatDepth(const int, const CheckContext &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/check.hpp:87
	// ("cv::detail::check_failed_MatDepth", vec![(pred!(mut, ["v", "ctx"], ["const int", "const cv::detail::CheckContext*"]), _)]),
	void cv_detail_check_failed_MatDepth_const_int_const_CheckContextR(const int v, const cv::detail::CheckContext* ctx, ResultVoid* ocvrs_return) {
		try {
			cv::detail::check_failed_MatDepth(v, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// check_failed_MatDepth(const int, const int, const CheckContext &)(Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/check.hpp:74
	// ("cv::detail::check_failed_MatDepth", vec![(pred!(mut, ["v1", "v2", "ctx"], ["const int", "const int", "const cv::detail::CheckContext*"]), _)]),
	void cv_detail_check_failed_MatDepth_const_int_const_int_const_CheckContextR(const int v1, const int v2, const cv::detail::CheckContext* ctx, ResultVoid* ocvrs_return) {
		try {
			cv::detail::check_failed_MatDepth(v1, v2, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// check_failed_MatType(const int, const CheckContext &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/check.hpp:88
	// ("cv::detail::check_failed_MatType", vec![(pred!(mut, ["v", "ctx"], ["const int", "const cv::detail::CheckContext*"]), _)]),
	void cv_detail_check_failed_MatType_const_int_const_CheckContextR(const int v, const cv::detail::CheckContext* ctx, ResultVoid* ocvrs_return) {
		try {
			cv::detail::check_failed_MatType(v, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// check_failed_MatType(const int, const int, const CheckContext &)(Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/check.hpp:75
	// ("cv::detail::check_failed_MatType", vec![(pred!(mut, ["v1", "v2", "ctx"], ["const int", "const int", "const cv::detail::CheckContext*"]), _)]),
	void cv_detail_check_failed_MatType_const_int_const_int_const_CheckContextR(const int v1, const int v2, const cv::detail::CheckContext* ctx, ResultVoid* ocvrs_return) {
		try {
			cv::detail::check_failed_MatType(v1, v2, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// check_failed_auto(const Size_<int>, const CheckContext &)(SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/check.hpp:85
	// ("cv::detail::check_failed_auto", vec![(pred!(mut, ["v", "ctx"], ["const cv::Size_<int>", "const cv::detail::CheckContext*"]), _)]),
	void cv_detail_check_failed_auto_const_Size_LintG_const_CheckContextR(const cv::Size_<int>* v, const cv::detail::CheckContext* ctx, ResultVoid* ocvrs_return) {
		try {
			cv::detail::check_failed_auto(*v, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// check_failed_auto(const Size_<int>, const Size_<int>, const CheckContext &)(SimpleClass, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/check.hpp:73
	// ("cv::detail::check_failed_auto", vec![(pred!(mut, ["v1", "v2", "ctx"], ["const cv::Size_<int>", "const cv::Size_<int>", "const cv::detail::CheckContext*"]), _)]),
	void cv_detail_check_failed_auto_const_Size_LintG_const_Size_LintG_const_CheckContextR(const cv::Size_<int>* v1, const cv::Size_<int>* v2, const cv::detail::CheckContext* ctx, ResultVoid* ocvrs_return) {
		try {
			cv::detail::check_failed_auto(*v1, *v2, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// check_failed_auto(const bool, const bool, const CheckContext &)(Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/check.hpp:68
	// ("cv::detail::check_failed_auto", vec![(pred!(mut, ["v1", "v2", "ctx"], ["const bool", "const bool", "const cv::detail::CheckContext*"]), _)]),
	void cv_detail_check_failed_auto_const_bool_const_bool_const_CheckContextR(const bool v1, const bool v2, const cv::detail::CheckContext* ctx, ResultVoid* ocvrs_return) {
		try {
			cv::detail::check_failed_auto(v1, v2, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// check_failed_auto(const double, const CheckContext &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/check.hpp:84
	// ("cv::detail::check_failed_auto", vec![(pred!(mut, ["v", "ctx"], ["const double", "const cv::detail::CheckContext*"]), _)]),
	void cv_detail_check_failed_auto_const_double_const_CheckContextR(const double v, const cv::detail::CheckContext* ctx, ResultVoid* ocvrs_return) {
		try {
			cv::detail::check_failed_auto(v, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// check_failed_auto(const double, const double, const CheckContext &)(Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/check.hpp:72
	// ("cv::detail::check_failed_auto", vec![(pred!(mut, ["v1", "v2", "ctx"], ["const double", "const double", "const cv::detail::CheckContext*"]), _)]),
	void cv_detail_check_failed_auto_const_double_const_double_const_CheckContextR(const double v1, const double v2, const cv::detail::CheckContext* ctx, ResultVoid* ocvrs_return) {
		try {
			cv::detail::check_failed_auto(v1, v2, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// check_failed_auto(const float, const CheckContext &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/check.hpp:83
	// ("cv::detail::check_failed_auto", vec![(pred!(mut, ["v", "ctx"], ["const float", "const cv::detail::CheckContext*"]), _)]),
	void cv_detail_check_failed_auto_const_float_const_CheckContextR(const float v, const cv::detail::CheckContext* ctx, ResultVoid* ocvrs_return) {
		try {
			cv::detail::check_failed_auto(v, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// check_failed_auto(const float, const float, const CheckContext &)(Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/check.hpp:71
	// ("cv::detail::check_failed_auto", vec![(pred!(mut, ["v1", "v2", "ctx"], ["const float", "const float", "const cv::detail::CheckContext*"]), _)]),
	void cv_detail_check_failed_auto_const_float_const_float_const_CheckContextR(const float v1, const float v2, const cv::detail::CheckContext* ctx, ResultVoid* ocvrs_return) {
		try {
			cv::detail::check_failed_auto(v1, v2, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// check_failed_auto(const int, const CheckContext &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/check.hpp:81
	// ("cv::detail::check_failed_auto", vec![(pred!(mut, ["v", "ctx"], ["const int", "const cv::detail::CheckContext*"]), _)]),
	void cv_detail_check_failed_auto_const_int_const_CheckContextR(const int v, const cv::detail::CheckContext* ctx, ResultVoid* ocvrs_return) {
		try {
			cv::detail::check_failed_auto(v, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// check_failed_auto(const int, const int, const CheckContext &)(Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/check.hpp:69
	// ("cv::detail::check_failed_auto", vec![(pred!(mut, ["v1", "v2", "ctx"], ["const int", "const int", "const cv::detail::CheckContext*"]), _)]),
	void cv_detail_check_failed_auto_const_int_const_int_const_CheckContextR(const int v1, const int v2, const cv::detail::CheckContext* ctx, ResultVoid* ocvrs_return) {
		try {
			cv::detail::check_failed_auto(v1, v2, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// check_failed_auto(const size_t, const CheckContext &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/check.hpp:82
	// ("cv::detail::check_failed_auto", vec![(pred!(mut, ["v", "ctx"], ["const size_t", "const cv::detail::CheckContext*"]), _)]),
	void cv_detail_check_failed_auto_const_size_t_const_CheckContextR(const size_t v, const cv::detail::CheckContext* ctx, ResultVoid* ocvrs_return) {
		try {
			cv::detail::check_failed_auto(v, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// check_failed_auto(const size_t, const size_t, const CheckContext &)(Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/check.hpp:70
	// ("cv::detail::check_failed_auto", vec![(pred!(mut, ["v1", "v2", "ctx"], ["const size_t", "const size_t", "const cv::detail::CheckContext*"]), _)]),
	void cv_detail_check_failed_auto_const_size_t_const_size_t_const_CheckContextR(const size_t v1, const size_t v2, const cv::detail::CheckContext* ctx, ResultVoid* ocvrs_return) {
		try {
			cv::detail::check_failed_auto(v1, v2, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// check_failed_auto(const std::string &, const CheckContext &)(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/check.hpp:86
	// ("cv::detail::check_failed_auto", vec![(pred!(mut, ["v1", "ctx"], ["const std::string*", "const cv::detail::CheckContext*"]), _)]),
	void cv_detail_check_failed_auto_const_stringR_const_CheckContextR(const char* v1, const cv::detail::CheckContext* ctx, ResultVoid* ocvrs_return) {
		try {
			cv::detail::check_failed_auto(std::string(v1), *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// check_failed_false(const bool, const CheckContext &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/check.hpp:79
	// ("cv::detail::check_failed_false", vec![(pred!(mut, ["v", "ctx"], ["const bool", "const cv::detail::CheckContext*"]), _)]),
	void cv_detail_check_failed_false_const_bool_const_CheckContextR(const bool v, const cv::detail::CheckContext* ctx, ResultVoid* ocvrs_return) {
		try {
			cv::detail::check_failed_false(v, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// check_failed_true(const bool, const CheckContext &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/check.hpp:78
	// ("cv::detail::check_failed_true", vec![(pred!(mut, ["v", "ctx"], ["const bool", "const cv::detail::CheckContext*"]), _)]),
	void cv_detail_check_failed_true_const_bool_const_CheckContextR(const bool v, const cv::detail::CheckContext* ctx, ResultVoid* ocvrs_return) {
		try {
			cv::detail::check_failed_true(v, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// determinant(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1805
	// ("cv::determinant", vec![(pred!(mut, ["mtx"], ["const cv::_InputArray*"]), _)]),
	void cv_determinant_const__InputArrayR(const cv::_InputArray* mtx, Result<double>* ocvrs_return) {
		try {
			double ret = cv::determinant(*mtx);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dft(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2171
	// ("cv::dft", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_dft_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::dft(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dft(InputArray, OutputArray, int, int)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2171
	// ("cv::dft", vec![(pred!(mut, ["src", "dst", "flags", "nonzeroRows"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_dft_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int flags, int nonzeroRows, ResultVoid* ocvrs_return) {
		try {
			cv::dft(*src, *dst, flags, nonzeroRows);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertFromD3D10Texture2D(ID3D10Texture2D *, OutputArray)(TraitClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/directx.hpp:148
	// ("cv::directx::convertFromD3D10Texture2D", vec![(pred!(mut, ["pD3D10Texture2D", "dst"], ["ID3D10Texture2D*", "const cv::_OutputArray*"]), _)]),
	void cv_directx_convertFromD3D10Texture2D_ID3D10Texture2DX_const__OutputArrayR(ID3D10Texture2D* pD3D10Texture2D, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::directx::convertFromD3D10Texture2D(pD3D10Texture2D, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertFromD3D11Texture2D(ID3D11Texture2D *, OutputArray)(TraitClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/directx.hpp:130
	// ("cv::directx::convertFromD3D11Texture2D", vec![(pred!(mut, ["pD3D11Texture2D", "dst"], ["ID3D11Texture2D*", "const cv::_OutputArray*"]), _)]),
	void cv_directx_convertFromD3D11Texture2D_ID3D11Texture2DX_const__OutputArrayR(ID3D11Texture2D* pD3D11Texture2D, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::directx::convertFromD3D11Texture2D(pD3D11Texture2D, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::directx::convertFromDirect3DSurface9(TraitClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/directx.hpp:168
	// ("cv::directx::convertFromDirect3DSurface9", vec![(pred!(mut, ["pDirect3DSurface9", "dst"], ["IDirect3DSurface9*", "const cv::_OutputArray*"]), _)]),
	void cv_directx_convertFromDirect3DSurface9_IDirect3DSurface9X_const__OutputArrayR(IDirect3DSurface9* pDirect3DSurface9, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::directx::convertFromDirect3DSurface9(pDirect3DSurface9, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertFromDirect3DSurface9(IDirect3DSurface9 *, OutputArray, void *)(TraitClass, OutputArray, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/directx.hpp:168
	// ("cv::directx::convertFromDirect3DSurface9", vec![(pred!(mut, ["pDirect3DSurface9", "dst", "surfaceSharedHandle"], ["IDirect3DSurface9*", "const cv::_OutputArray*", "void*"]), _)]),
	void cv_directx_convertFromDirect3DSurface9_IDirect3DSurface9X_const__OutputArrayR_voidX(IDirect3DSurface9* pDirect3DSurface9, const cv::_OutputArray* dst, void* surfaceSharedHandle, ResultVoid* ocvrs_return) {
		try {
			cv::directx::convertFromDirect3DSurface9(pDirect3DSurface9, *dst, surfaceSharedHandle);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertToD3D10Texture2D(InputArray, ID3D10Texture2D *)(InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/directx.hpp:139
	// ("cv::directx::convertToD3D10Texture2D", vec![(pred!(mut, ["src", "pD3D10Texture2D"], ["const cv::_InputArray*", "ID3D10Texture2D*"]), _)]),
	void cv_directx_convertToD3D10Texture2D_const__InputArrayR_ID3D10Texture2DX(const cv::_InputArray* src, ID3D10Texture2D* pD3D10Texture2D, ResultVoid* ocvrs_return) {
		try {
			cv::directx::convertToD3D10Texture2D(*src, pD3D10Texture2D);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertToD3D11Texture2D(InputArray, ID3D11Texture2D *)(InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/directx.hpp:120
	// ("cv::directx::convertToD3D11Texture2D", vec![(pred!(mut, ["src", "pD3D11Texture2D"], ["const cv::_InputArray*", "ID3D11Texture2D*"]), _)]),
	void cv_directx_convertToD3D11Texture2D_const__InputArrayR_ID3D11Texture2DX(const cv::_InputArray* src, ID3D11Texture2D* pD3D11Texture2D, ResultVoid* ocvrs_return) {
		try {
			cv::directx::convertToD3D11Texture2D(*src, pD3D11Texture2D);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::directx::convertToDirect3DSurface9(InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/directx.hpp:158
	// ("cv::directx::convertToDirect3DSurface9", vec![(pred!(mut, ["src", "pDirect3DSurface9"], ["const cv::_InputArray*", "IDirect3DSurface9*"]), _)]),
	void cv_directx_convertToDirect3DSurface9_const__InputArrayR_IDirect3DSurface9X(const cv::_InputArray* src, IDirect3DSurface9* pDirect3DSurface9, ResultVoid* ocvrs_return) {
		try {
			cv::directx::convertToDirect3DSurface9(*src, pDirect3DSurface9);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertToDirect3DSurface9(InputArray, IDirect3DSurface9 *, void *)(InputArray, TraitClass, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/directx.hpp:158
	// ("cv::directx::convertToDirect3DSurface9", vec![(pred!(mut, ["src", "pDirect3DSurface9", "surfaceSharedHandle"], ["const cv::_InputArray*", "IDirect3DSurface9*", "void*"]), _)]),
	void cv_directx_convertToDirect3DSurface9_const__InputArrayR_IDirect3DSurface9X_voidX(const cv::_InputArray* src, IDirect3DSurface9* pDirect3DSurface9, void* surfaceSharedHandle, ResultVoid* ocvrs_return) {
		try {
			cv::directx::convertToDirect3DSurface9(*src, pDirect3DSurface9, surfaceSharedHandle);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTypeFromD3DFORMAT(const int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/directx.hpp:178
	// ("cv::directx::getTypeFromD3DFORMAT", vec![(pred!(mut, ["iD3DFORMAT"], ["const int"]), _)]),
	void cv_directx_getTypeFromD3DFORMAT_const_int(const int iD3DFORMAT, Result<int>* ocvrs_return) {
		try {
			int ret = cv::directx::getTypeFromD3DFORMAT(iD3DFORMAT);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTypeFromDXGI_FORMAT(const int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/directx.hpp:173
	// ("cv::directx::getTypeFromDXGI_FORMAT", vec![(pred!(mut, ["iDXGI_FORMAT"], ["const int"]), _)]),
	void cv_directx_getTypeFromDXGI_FORMAT_const_int(const int iDXGI_FORMAT, Result<int>* ocvrs_return) {
		try {
			int ret = cv::directx::getTypeFromDXGI_FORMAT(iDXGI_FORMAT);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initializeContextFromD3D10Device(ID3D10Device *)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/directx.hpp:91
	// ("cv::directx::ocl::initializeContextFromD3D10Device", vec![(pred!(mut, ["pD3D10Device"], ["ID3D10Device*"]), _)]),
	void cv_directx_ocl_initializeContextFromD3D10Device_ID3D10DeviceX(ID3D10Device* pD3D10Device, Result<cv::ocl::Context*>* ocvrs_return) {
		try {
			cv::ocl::Context ret = cv::directx::ocl::initializeContextFromD3D10Device(pD3D10Device);
			Ok(new cv::ocl::Context(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initializeContextFromD3D11Device(ID3D11Device *)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/directx.hpp:85
	// ("cv::directx::ocl::initializeContextFromD3D11Device", vec![(pred!(mut, ["pD3D11Device"], ["ID3D11Device*"]), _)]),
	void cv_directx_ocl_initializeContextFromD3D11Device_ID3D11DeviceX(ID3D11Device* pD3D11Device, Result<cv::ocl::Context*>* ocvrs_return) {
		try {
			cv::ocl::Context ret = cv::directx::ocl::initializeContextFromD3D11Device(pD3D11Device);
			Ok(new cv::ocl::Context(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initializeContextFromDirect3DDevice9Ex(IDirect3DDevice9Ex *)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/directx.hpp:97
	// ("cv::directx::ocl::initializeContextFromDirect3DDevice9Ex", vec![(pred!(mut, ["pDirect3DDevice9Ex"], ["IDirect3DDevice9Ex*"]), _)]),
	void cv_directx_ocl_initializeContextFromDirect3DDevice9Ex_IDirect3DDevice9ExX(IDirect3DDevice9Ex* pDirect3DDevice9Ex, Result<cv::ocl::Context*>* ocvrs_return) {
		try {
			cv::ocl::Context ret = cv::directx::ocl::initializeContextFromDirect3DDevice9Ex(pDirect3DDevice9Ex);
			Ok(new cv::ocl::Context(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initializeContextFromDirect3DDevice9(IDirect3DDevice9 *)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/directx.hpp:103
	// ("cv::directx::ocl::initializeContextFromDirect3DDevice9", vec![(pred!(mut, ["pDirect3DDevice9"], ["IDirect3DDevice9*"]), _)]),
	void cv_directx_ocl_initializeContextFromDirect3DDevice9_IDirect3DDevice9X(IDirect3DDevice9* pDirect3DDevice9, Result<cv::ocl::Context*>* ocvrs_return) {
		try {
			cv::ocl::Context ret = cv::directx::ocl::initializeContextFromDirect3DDevice9(pDirect3DDevice9);
			Ok(new cv::ocl::Context(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::divide(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:464
	// ("cv::divide", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_divide_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::divide(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// divide(InputArray, InputArray, OutputArray, double, int)(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:464
	// ("cv::divide", vec![(pred!(mut, ["src1", "src2", "dst", "scale", "dtype"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "int"]), _)]),
	void cv_divide_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_int(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, double scale, int dtype, ResultVoid* ocvrs_return) {
		try {
			cv::divide(*src1, *src2, *dst, scale, dtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::divide(Primitive, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:468
	// ("cv::divide", vec![(pred!(mut, ["scale", "src2", "dst"], ["double", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_divide_double_const__InputArrayR_const__OutputArrayR(double scale, const cv::_InputArray* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::divide(scale, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// divide(double, InputArray, OutputArray, int)(Primitive, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:468
	// ("cv::divide", vec![(pred!(mut, ["scale", "src2", "dst", "dtype"], ["double", "const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_divide_double_const__InputArrayR_const__OutputArrayR_int(double scale, const cv::_InputArray* src2, const cv::_OutputArray* dst, int dtype, ResultVoid* ocvrs_return) {
		try {
			cv::divide(scale, *src2, *dst, dtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// eigenNonSymmetric(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1964
	// ("cv::eigenNonSymmetric", vec![(pred!(mut, ["src", "eigenvalues", "eigenvectors"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_eigenNonSymmetric_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* eigenvalues, const cv::_OutputArray* eigenvectors, ResultVoid* ocvrs_return) {
		try {
			cv::eigenNonSymmetric(*src, *eigenvalues, *eigenvectors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::eigen(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1947
	// ("cv::eigen", vec![(pred!(mut, ["src", "eigenvalues"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_eigen_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* eigenvalues, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::eigen(*src, *eigenvalues);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// eigen(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1947
	// ("cv::eigen", vec![(pred!(mut, ["src", "eigenvalues", "eigenvectors"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_eigen_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* eigenvalues, const cv::_OutputArray* eigenvectors, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::eigen(*src, *eigenvalues, *eigenvectors);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// errorNoReturn(int, const String &, const char *, const char *, int)(Primitive, InString, InString, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/base.hpp:301
	// ("cv::errorNoReturn", vec![(pred!(mut, ["_code", "_err", "_func", "_file", "_line"], ["int", "const cv::String*", "const char*", "const char*", "int"]), _)]),
	void cv_errorNoReturn_int_const_StringR_const_charX_const_charX_int(int _code, const char* _err, const char* _func, const char* _file, int _line, ResultVoid* ocvrs_return) {
		try {
			cv::errorNoReturn(_code, cv::String(_err), _func, _file, _line);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// error(const Exception &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:150
	// ("cv::error", vec![(pred!(mut, ["exc"], ["const cv::Exception*"]), _)]),
	void cv_error_const_ExceptionR(const cv::Exception* exc, ResultVoid* ocvrs_return) {
		try {
			cv::error(*exc);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// error(int, const String &, const char *, const char *, int)(Primitive, InString, InString, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/base.hpp:298
	// ("cv::error", vec![(pred!(mut, ["_code", "_err", "_func", "_file", "_line"], ["int", "const cv::String*", "const char*", "const char*", "int"]), _)]),
	void cv_error_int_const_StringR_const_charX_const_charX_int(int _code, const char* _err, const char* _func, const char* _file, int _line, ResultVoid* ocvrs_return) {
		try {
			cv::error(_code, cv::String(_err), _func, _file, _line);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// exp(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1516
	// ("cv::exp", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_exp_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::exp(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// extractChannel(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1036
	// ("cv::extractChannel", vec![(pred!(mut, ["src", "dst", "coi"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_extractChannel_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int coi, ResultVoid* ocvrs_return) {
		try {
			cv::extractChannel(*src, *dst, coi);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::extractImageCOI(Indirect, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/core_c.h:2803
	// ("cv::extractImageCOI", vec![(pred!(mut, ["arr", "coiimg"], ["const CvArr*", "const cv::_OutputArray*"]), _)]),
	void cv_extractImageCOI_const_CvArrX_const__OutputArrayR(const CvArr* arr, const cv::_OutputArray* coiimg, ResultVoid* ocvrs_return) {
		try {
			cv::extractImageCOI(arr, *coiimg);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// extractImageCOI(const CvArr *, OutputArray, int)(Indirect, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/core_c.h:2803
	// ("cv::extractImageCOI", vec![(pred!(mut, ["arr", "coiimg", "coi"], ["const CvArr*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_extractImageCOI_const_CvArrX_const__OutputArrayR_int(const CvArr* arr, const cv::_OutputArray* coiimg, int coi, ResultVoid* ocvrs_return) {
		try {
			cv::extractImageCOI(arr, *coiimg, coi);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fastAtan2(float, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/base.hpp:598
	// ("cv::fastAtan2", vec![(pred!(mut, ["y", "x"], ["float", "float"]), _)]),
	void cv_fastAtan2_float_float(float y, float x, Result<float>* ocvrs_return) {
		try {
			float ret = cv::fastAtan2(y, x);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findNonZero(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:620
	// ("cv::findNonZero", vec![(pred!(mut, ["src", "idx"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_findNonZero_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* idx, ResultVoid* ocvrs_return) {
		try {
			cv::findNonZero(*src, *idx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// flip(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1078
	// ("cv::flip", vec![(pred!(mut, ["src", "dst", "flipCode"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_flip_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int flipCode, ResultVoid* ocvrs_return) {
		try {
			cv::flip(*src, *dst, flipCode);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::gemm(InputArray, InputArray, Primitive, InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1660
	// ("cv::gemm", vec![(pred!(mut, ["src1", "src2", "alpha", "src3", "beta", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_InputArray*", "double", "const cv::_OutputArray*"]), _)]),
	void cv_gemm_const__InputArrayR_const__InputArrayR_double_const__InputArrayR_double_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, double alpha, const cv::_InputArray* src3, double beta, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::gemm(*src1, *src2, alpha, *src3, beta, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// gemm(InputArray, InputArray, double, InputArray, double, OutputArray, int)(InputArray, InputArray, Primitive, InputArray, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1660
	// ("cv::gemm", vec![(pred!(mut, ["src1", "src2", "alpha", "src3", "beta", "dst", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_InputArray*", "double", "const cv::_OutputArray*", "int"]), _)]),
	void cv_gemm_const__InputArrayR_const__InputArrayR_double_const__InputArrayR_double_const__OutputArrayR_int(const cv::_InputArray* src1, const cv::_InputArray* src2, double alpha, const cv::_InputArray* src3, double beta, const cv::_OutputArray* dst, int flags, ResultVoid* ocvrs_return) {
		try {
			cv::gemm(*src1, *src2, alpha, *src3, beta, *dst, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBuildInformation()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:240
	// ("cv::getBuildInformation", vec![(pred!(mut, [], []), _)]),
	void cv_getBuildInformation(Result<void*>* ocvrs_return) {
		try {
			const cv::String ret = cv::getBuildInformation();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCPUFeaturesLine()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:441
	// ("cv::getCPUFeaturesLine", vec![(pred!(mut, [], []), _)]),
	void cv_getCPUFeaturesLine(Result<void*>* ocvrs_return) {
		try {
			std::string ret = cv::getCPUFeaturesLine();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCPUTickCount()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:413
	// ("cv::getCPUTickCount", vec![(pred!(mut, [], []), _)]),
	void cv_getCPUTickCount(Result<int64_t>* ocvrs_return) {
		try {
			int64_t ret = cv::getCPUTickCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getElemSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:566
	// ("cv::getElemSize", vec![(pred!(mut, ["type"], ["int"]), _)]),
	void cv_getElemSize_int(int type, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = cv::getElemSize(type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getHardwareFeatureName(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:429
	// ("cv::getHardwareFeatureName", vec![(pred!(mut, ["feature"], ["int"]), _)]),
	void cv_getHardwareFeatureName_int(int feature, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::getHardwareFeatureName(feature);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLogLevel()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:293
	// ("cv::getLogLevel", vec![(pred!(mut, [], []), _)]),
	void cv_getLogLevel(Result<int>* ocvrs_return) {
		try {
			int ret = cv::getLogLevel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumThreads()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:216
	// ("cv::getNumThreads", vec![(pred!(mut, [], []), _)]),
	void cv_getNumThreads(Result<int>* ocvrs_return) {
		try {
			int ret = cv::getNumThreads();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumberOfCPUs()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:445
	// ("cv::getNumberOfCPUs", vec![(pred!(mut, [], []), _)]),
	void cv_getNumberOfCPUs(Result<int>* ocvrs_return) {
		try {
			int ret = cv::getNumberOfCPUs();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOptimalDFTSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2279
	// ("cv::getOptimalDFTSize", vec![(pred!(mut, ["vecsize"], ["int"]), _)]),
	void cv_getOptimalDFTSize_int(int vecsize, Result<int>* ocvrs_return) {
		try {
			int ret = cv::getOptimalDFTSize(vecsize);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getThreadNum()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:232
	// ("cv::getThreadNum", vec![(pred!(mut, [], []), _)]),
	void cv_getThreadNum(Result<int>* ocvrs_return) {
		try {
			int ret = cv::getThreadNum();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTickCount()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:266
	// ("cv::getTickCount", vec![(pred!(mut, [], []), _)]),
	void cv_getTickCount(Result<int64_t>* ocvrs_return) {
		try {
			int64_t ret = cv::getTickCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTickFrequency()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:279
	// ("cv::getTickFrequency", vec![(pred!(mut, [], []), _)]),
	void cv_getTickFrequency(Result<double>* ocvrs_return) {
		try {
			double ret = cv::getTickFrequency();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVersionMajor()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:251
	// ("cv::getVersionMajor", vec![(pred!(mut, [], []), _)]),
	int cv_getVersionMajor() {
			int ret = cv::getVersionMajor();
			return ret;
	}

	// getVersionMinor()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:254
	// ("cv::getVersionMinor", vec![(pred!(mut, [], []), _)]),
	int cv_getVersionMinor() {
			int ret = cv::getVersionMinor();
			return ret;
	}

	// getVersionRevision()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:257
	// ("cv::getVersionRevision", vec![(pred!(mut, [], []), _)]),
	int cv_getVersionRevision() {
			int ret = cv::getVersionRevision();
			return ret;
	}

	// getVersionString()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:248
	// ("cv::getVersionString", vec![(pred!(mut, [], []), _)]),
	void cv_getVersionString(Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::getVersionString();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::glob(InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:178
	// ("cv::glob", vec![(pred!(mut, ["pattern", "result"], ["cv::String", "std::vector<cv::String>*"]), _)]),
	void cv_glob_String_vectorLStringGR(const char* pattern, std::vector<cv::String>* result, ResultVoid* ocvrs_return) {
		try {
			cv::glob(cv::String(pattern), *result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// glob(String, std::vector<String> &, bool)(InString, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:178
	// ("cv::glob", vec![(pred!(mut, ["pattern", "result", "recursive"], ["cv::String", "std::vector<cv::String>*", "bool"]), _)]),
	void cv_glob_String_vectorLStringGR_bool(const char* pattern, std::vector<cv::String>* result, bool recursive, ResultVoid* ocvrs_return) {
		try {
			cv::glob(cv::String(pattern), *result, recursive);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// haveOpenVX()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ovx.hpp:19
	// ("cv::haveOpenVX", vec![(pred!(mut, [], []), _)]),
	void cv_haveOpenVX(Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::haveOpenVX();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// hconcat(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1164
	// ("cv::hconcat", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_hconcat_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::hconcat(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// hconcat(InputArrayOfArrays, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1183
	// ("cv::hconcat", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_hconcat_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::hconcat(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::idct(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2236
	// ("cv::idct", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_idct_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::idct(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// idct(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2236
	// ("cv::idct", vec![(pred!(mut, ["src", "dst", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_idct_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int flags, ResultVoid* ocvrs_return) {
		try {
			cv::idct(*src, *dst, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::idft(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2185
	// ("cv::idft", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_idft_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::idft(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// idft(InputArray, OutputArray, int, int)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2185
	// ("cv::idft", vec![(pred!(mut, ["src", "dst", "flags", "nonzeroRows"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_idft_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int flags, int nonzeroRows, ResultVoid* ocvrs_return) {
		try {
			cv::idft(*src, *dst, flags, nonzeroRows);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// inRange(InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1393
	// ("cv::inRange", vec![(pred!(mut, ["src", "lowerb", "upperb", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_inRange_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_InputArray* lowerb, const cv::_InputArray* upperb, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::inRange(*src, *lowerb, *upperb, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// insertChannel(InputArray, InputOutputArray, int)(InputArray, InputOutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1044
	// ("cv::insertChannel", vec![(pred!(mut, ["src", "dst", "coi"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "int"]), _)]),
	void cv_insertChannel_const__InputArrayR_const__InputOutputArrayR_int(const cv::_InputArray* src, const cv::_InputOutputArray* dst, int coi, ResultVoid* ocvrs_return) {
		try {
			cv::insertChannel(*src, *dst, coi);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::insertImageCOI(InputArray, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/core_c.h:2805
	// ("cv::insertImageCOI", vec![(pred!(mut, ["coiimg", "arr"], ["const cv::_InputArray*", "CvArr*"]), _)]),
	void cv_insertImageCOI_const__InputArrayR_CvArrX(const cv::_InputArray* coiimg, CvArr* arr, ResultVoid* ocvrs_return) {
		try {
			cv::insertImageCOI(*coiimg, arr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// insertImageCOI(InputArray, CvArr *, int)(InputArray, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/core_c.h:2805
	// ("cv::insertImageCOI", vec![(pred!(mut, ["coiimg", "arr", "coi"], ["const cv::_InputArray*", "CvArr*", "int"]), _)]),
	void cv_insertImageCOI_const__InputArrayR_CvArrX_int(const cv::_InputArray* coiimg, CvArr* arr, int coi, ResultVoid* ocvrs_return) {
		try {
			cv::insertImageCOI(*coiimg, arr, coi);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFlags()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:117
	// ("cv::instr::getFlags", vec![(pred!(mut, [], []), _)]),
	void cv_instr_getFlags(Result<cv::instr::FLAGS>* ocvrs_return) {
		try {
			cv::instr::FLAGS ret = cv::instr::getFlags();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// resetTrace()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:106
	// ("cv::instr::resetTrace", vec![(pred!(mut, [], []), _)]),
	void cv_instr_resetTrace(ResultVoid* ocvrs_return) {
		try {
			cv::instr::resetTrace();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFlags(FLAGS)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:115
	// ("cv::instr::setFlags", vec![(pred!(mut, ["modeFlags"], ["cv::instr::FLAGS"]), _)]),
	void cv_instr_setFlags_FLAGS(cv::instr::FLAGS modeFlags, ResultVoid* ocvrs_return) {
		try {
			cv::instr::setFlags(modeFlags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseInstrumentation(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:105
	// ("cv::instr::setUseInstrumentation", vec![(pred!(mut, ["flag"], ["bool"]), _)]),
	void cv_instr_setUseInstrumentation_bool(bool flag, ResultVoid* ocvrs_return) {
		try {
			cv::instr::setUseInstrumentation(flag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// useInstrumentation()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:104
	// ("cv::instr::useInstrumentation", vec![(pred!(mut, [], []), _)]),
	void cv_instr_useInstrumentation(Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::instr::useInstrumentation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::invert(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1841
	// ("cv::invert", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_invert_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, Result<double>* ocvrs_return) {
		try {
			double ret = cv::invert(*src, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// invert(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1841
	// ("cv::invert", vec![(pred!(mut, ["src", "dst", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_invert_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int flags, Result<double>* ocvrs_return) {
		try {
			double ret = cv::invert(*src, *dst, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getIppErrorLocation()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/base.hpp:679
	// ("cv::ipp::getIppErrorLocation", vec![(pred!(mut, [], []), _)]),
	void cv_ipp_getIppErrorLocation(Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::ipp::getIppErrorLocation();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getIppFeatures()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/base.hpp:674
	// ("cv::ipp::getIppFeatures", vec![(pred!(mut, [], []), _)]),
	void cv_ipp_getIppFeatures(Result<int>* ocvrs_return) {
		try {
			int ret = cv::ipp::getIppFeatures();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getIppStatus()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/base.hpp:678
	// ("cv::ipp::getIppStatus", vec![(pred!(mut, [], []), _)]),
	void cv_ipp_getIppStatus(Result<int>* ocvrs_return) {
		try {
			int ret = cv::ipp::getIppStatus();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getIppVersion()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/base.hpp:682
	// ("cv::ipp::getIppVersion", vec![(pred!(mut, [], []), _)]),
	void cv_ipp_getIppVersion(Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::ipp::getIppVersion();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ipp::setIppStatus(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/base.hpp:676
	// ("cv::ipp::setIppStatus", vec![(pred!(mut, ["status"], ["int"]), _)]),
	void cv_ipp_setIppStatus_int(int status, ResultVoid* ocvrs_return) {
		try {
			cv::ipp::setIppStatus(status);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setIppStatus(int, const char *const, const char *const, int)(Primitive, InString, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/base.hpp:676
	// ("cv::ipp::setIppStatus", vec![(pred!(mut, ["status", "funcname", "filename", "line"], ["int", "const char*", "const char*", "int"]), _)]),
	void cv_ipp_setIppStatus_int_const_charX_const_charX_int(int status, const char* funcname, const char* filename, int line, ResultVoid* ocvrs_return) {
		try {
			cv::ipp::setIppStatus(status, funcname, filename, line);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseIPP_NE(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/base.hpp:690
	// ("cv::ipp::setUseIPP_NE", vec![(pred!(mut, ["flag"], ["bool"]), _)]),
	void cv_ipp_setUseIPP_NE_bool(bool flag, ResultVoid* ocvrs_return) {
		try {
			cv::ipp::setUseIPP_NE(flag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseIPP_NotExact(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/base.hpp:687
	// ("cv::ipp::setUseIPP_NotExact", vec![(pred!(mut, ["flag"], ["bool"]), _)]),
	void cv_ipp_setUseIPP_NotExact_bool(bool flag, ResultVoid* ocvrs_return) {
		try {
			cv::ipp::setUseIPP_NotExact(flag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseIPP(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/base.hpp:681
	// ("cv::ipp::setUseIPP", vec![(pred!(mut, ["flag"], ["bool"]), _)]),
	void cv_ipp_setUseIPP_bool(bool flag, ResultVoid* ocvrs_return) {
		try {
			cv::ipp::setUseIPP(flag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// useIPP()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/base.hpp:680
	// ("cv::ipp::useIPP", vec![(pred!(mut, [], []), _)]),
	void cv_ipp_useIPP(Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::ipp::useIPP();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// useIPP_NE()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/base.hpp:689
	// ("cv::ipp::useIPP_NE", vec![(pred!(mut, [], []), _)]),
	void cv_ipp_useIPP_NE(Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::ipp::useIPP_NE();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// useIPP_NotExact()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/base.hpp:686
	// ("cv::ipp::useIPP_NotExact", vec![(pred!(mut, [], []), _)]),
	void cv_ipp_useIPP_NotExact(Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::ipp::useIPP_NotExact();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::kmeans(InputArray, Primitive, InputOutputArray, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:3009
	// ("cv::kmeans", vec![(pred!(mut, ["data", "K", "bestLabels", "criteria", "attempts", "flags"], ["const cv::_InputArray*", "int", "const cv::_InputOutputArray*", "cv::TermCriteria", "int", "int"]), _)]),
	void cv_kmeans_const__InputArrayR_int_const__InputOutputArrayR_TermCriteria_int_int(const cv::_InputArray* data, int K, const cv::_InputOutputArray* bestLabels, cv::TermCriteria* criteria, int attempts, int flags, Result<double>* ocvrs_return) {
		try {
			double ret = cv::kmeans(*data, K, *bestLabels, *criteria, attempts, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// kmeans(InputArray, int, InputOutputArray, TermCriteria, int, int, OutputArray)(InputArray, Primitive, InputOutputArray, SimpleClass, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:3009
	// ("cv::kmeans", vec![(pred!(mut, ["data", "K", "bestLabels", "criteria", "attempts", "flags", "centers"], ["const cv::_InputArray*", "int", "const cv::_InputOutputArray*", "cv::TermCriteria", "int", "int", "const cv::_OutputArray*"]), _)]),
	void cv_kmeans_const__InputArrayR_int_const__InputOutputArrayR_TermCriteria_int_int_const__OutputArrayR(const cv::_InputArray* data, int K, const cv::_InputOutputArray* bestLabels, cv::TermCriteria* criteria, int attempts, int flags, const cv::_OutputArray* centers, Result<double>* ocvrs_return) {
		try {
			double ret = cv::kmeans(*data, K, *bestLabels, *criteria, attempts, flags, *centers);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// log(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1529
	// ("cv::log", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_log_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::log(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// magnitude(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1606
	// ("cv::magnitude", vec![(pred!(mut, ["x", "y", "magnitude"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_magnitude_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* x, const cv::_InputArray* y, const cv::_OutputArray* magnitude, ResultVoid* ocvrs_return) {
		try {
			cv::magnitude(*x, *y, *magnitude);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// max(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3764
	// ("cv::max", vec![(pred!(mut, ["a", "b"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_max_const_MatR_const_MatR(const cv::Mat* a, const cv::Mat* b, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::max(*a, *b);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// max(const Mat &, const Mat &, Mat &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1460
	// ("cv::max", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::Mat*", "const cv::Mat*", "cv::Mat*"]), _)]),
	void cv_max_const_MatR_const_MatR_MatR(const cv::Mat* src1, const cv::Mat* src2, cv::Mat* dst, ResultVoid* ocvrs_return) {
		try {
			cv::max(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// max(const Mat &, double)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3765
	// ("cv::max", vec![(pred!(mut, ["a", "s"], ["const cv::Mat*", "double"]), _)]),
	void cv_max_const_MatR_double(const cv::Mat* a, double s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::max(*a, s);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// max(const UMat &, const UMat &, UMat &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1464
	// ("cv::max", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::UMat*", "const cv::UMat*", "cv::UMat*"]), _)]),
	void cv_max_const_UMatR_const_UMatR_UMatR(const cv::UMat* src1, const cv::UMat* src2, cv::UMat* dst, ResultVoid* ocvrs_return) {
		try {
			cv::max(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// max(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1456
	// ("cv::max", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_max_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::max(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// max(double, const Mat &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3766
	// ("cv::max", vec![(pred!(mut, ["s", "a"], ["double", "const cv::Mat*"]), _)]),
	void cv_max_double_const_MatR(double s, const cv::Mat* a, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::max(s, *a);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::meanStdDev(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:655
	// ("cv::meanStdDev", vec![(pred!(mut, ["src", "mean", "stddev"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_meanStdDev_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* mean, const cv::_OutputArray* stddev, ResultVoid* ocvrs_return) {
		try {
			cv::meanStdDev(*src, *mean, *stddev);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// meanStdDev(InputArray, OutputArray, OutputArray, InputArray)(InputArray, OutputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:655
	// ("cv::meanStdDev", vec![(pred!(mut, ["src", "mean", "stddev", "mask"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_meanStdDev_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_OutputArray* mean, const cv::_OutputArray* stddev, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			cv::meanStdDev(*src, *mean, *stddev, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::mean(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:633
	// ("cv::mean", vec![(pred!(mut, ["src"], ["const cv::_InputArray*"]), _)]),
	void cv_mean_const__InputArrayR(const cv::_InputArray* src, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = cv::mean(*src);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mean(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:633
	// ("cv::mean", vec![(pred!(mut, ["src", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_mean_const__InputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_InputArray* mask, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = cv::mean(*src, *mask);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// merge(InputArrayOfArrays, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:928
	// ("cv::merge", vec![(pred!(mut, ["mv", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_merge_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* mv, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::merge(*mv, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::minMaxIdx(InputArray, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:860
	// ("cv::minMaxIdx", vec![(pred!(mut, ["src", "minVal"], ["const cv::_InputArray*", "double*"]), _)]),
	void cv_minMaxIdx_const__InputArrayR_doubleX(const cv::_InputArray* src, double* minVal, ResultVoid* ocvrs_return) {
		try {
			cv::minMaxIdx(*src, minVal);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// minMaxIdx(InputArray, double *, double *, int *, int *, InputArray)(InputArray, Indirect, Indirect, Indirect, Indirect, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:860
	// ("cv::minMaxIdx", vec![(pred!(mut, ["src", "minVal", "maxVal", "minIdx", "maxIdx", "mask"], ["const cv::_InputArray*", "double*", "double*", "int*", "int*", "const cv::_InputArray*"]), _)]),
	void cv_minMaxIdx_const__InputArrayR_doubleX_doubleX_intX_intX_const__InputArrayR(const cv::_InputArray* src, double* minVal, double* maxVal, int* minIdx, int* maxIdx, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			cv::minMaxIdx(*src, minVal, maxVal, minIdx, maxIdx, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::minMaxLoc(TraitClass, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:872
	// ("cv::minMaxLoc", vec![(pred!(mut, ["a", "minVal", "maxVal"], ["const cv::SparseMat*", "double*", "double*"]), _)]),
	void cv_minMaxLoc_const_SparseMatR_doubleX_doubleX(const cv::SparseMat* a, double* minVal, double* maxVal, ResultVoid* ocvrs_return) {
		try {
			cv::minMaxLoc(*a, minVal, maxVal);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// minMaxLoc(const SparseMat &, double *, double *, int *, int *)(TraitClass, Indirect, Indirect, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:872
	// ("cv::minMaxLoc", vec![(pred!(mut, ["a", "minVal", "maxVal", "minIdx", "maxIdx"], ["const cv::SparseMat*", "double*", "double*", "int*", "int*"]), _)]),
	void cv_minMaxLoc_const_SparseMatR_doubleX_doubleX_intX_intX(const cv::SparseMat* a, double* minVal, double* maxVal, int* minIdx, int* maxIdx, ResultVoid* ocvrs_return) {
		try {
			cv::minMaxLoc(*a, minVal, maxVal, minIdx, maxIdx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::minMaxLoc(InputArray, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:832
	// ("cv::minMaxLoc", vec![(pred!(mut, ["src", "minVal"], ["const cv::_InputArray*", "double*"]), _)]),
	void cv_minMaxLoc_const__InputArrayR_doubleX(const cv::_InputArray* src, double* minVal, ResultVoid* ocvrs_return) {
		try {
			cv::minMaxLoc(*src, minVal);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// minMaxLoc(InputArray, double *, double *, Point *, Point *, InputArray)(InputArray, Indirect, Indirect, SimpleClass, SimpleClass, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:832
	// ("cv::minMaxLoc", vec![(pred!(mut, ["src", "minVal", "maxVal", "minLoc", "maxLoc", "mask"], ["const cv::_InputArray*", "double*", "double*", "cv::Point*", "cv::Point*", "const cv::_InputArray*"]), _)]),
	void cv_minMaxLoc_const__InputArrayR_doubleX_doubleX_PointX_PointX_const__InputArrayR(const cv::_InputArray* src, double* minVal, double* maxVal, cv::Point* minLoc, cv::Point* maxLoc, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			cv::minMaxLoc(*src, minVal, maxVal, minLoc, maxLoc, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// min(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3756
	// ("cv::min", vec![(pred!(mut, ["a", "b"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_min_const_MatR_const_MatR(const cv::Mat* a, const cv::Mat* b, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::min(*a, *b);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// min(const Mat &, const Mat &, Mat &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1439
	// ("cv::min", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::Mat*", "const cv::Mat*", "cv::Mat*"]), _)]),
	void cv_min_const_MatR_const_MatR_MatR(const cv::Mat* src1, const cv::Mat* src2, cv::Mat* dst, ResultVoid* ocvrs_return) {
		try {
			cv::min(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// min(const Mat &, double)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3757
	// ("cv::min", vec![(pred!(mut, ["a", "s"], ["const cv::Mat*", "double"]), _)]),
	void cv_min_const_MatR_double(const cv::Mat* a, double s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::min(*a, s);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// min(const UMat &, const UMat &, UMat &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1443
	// ("cv::min", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::UMat*", "const cv::UMat*", "cv::UMat*"]), _)]),
	void cv_min_const_UMatR_const_UMatR_UMatR(const cv::UMat* src1, const cv::UMat* src2, cv::UMat* dst, ResultVoid* ocvrs_return) {
		try {
			cv::min(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// min(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1435
	// ("cv::min", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_min_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::min(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// min(double, const Mat &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3758
	// ("cv::min", vec![(pred!(mut, ["s", "a"], ["double", "const cv::Mat*"]), _)]),
	void cv_min_double_const_MatR(double s, const cv::Mat* a, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::min(s, *a);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mixChannels(InputArrayOfArrays, InputOutputArrayOfArrays, const int *, size_t)(InputArray, InputOutputArray, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1011
	// ("cv::mixChannels", vec![(pred!(mut, ["src", "dst", "fromTo", "npairs"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const int*", "size_t"]), _)]),
	void cv_mixChannels_const__InputArrayR_const__InputOutputArrayR_const_intX_size_t(const cv::_InputArray* src, const cv::_InputOutputArray* dst, const int* fromTo, size_t npairs, ResultVoid* ocvrs_return) {
		try {
			cv::mixChannels(*src, *dst, fromTo, npairs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mixChannels(InputArrayOfArrays, InputOutputArrayOfArrays, const std::vector<int> &)(InputArray, InputOutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1027
	// ("cv::mixChannels", vec![(pred!(mut, ["src", "dst", "fromTo"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const std::vector<int>*"]), _)]),
	void cv_mixChannels_const__InputArrayR_const__InputOutputArrayR_const_vectorLintGR(const cv::_InputArray* src, const cv::_InputOutputArray* dst, const std::vector<int>* fromTo, ResultVoid* ocvrs_return) {
		try {
			cv::mixChannels(*src, *dst, *fromTo);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::mulSpectrums(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2255
	// ("cv::mulSpectrums", vec![(pred!(mut, ["a", "b", "c", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_mulSpectrums_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* a, const cv::_InputArray* b, const cv::_OutputArray* c, int flags, ResultVoid* ocvrs_return) {
		try {
			cv::mulSpectrums(*a, *b, *c, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mulSpectrums(InputArray, InputArray, OutputArray, int, bool)(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2255
	// ("cv::mulSpectrums", vec![(pred!(mut, ["a", "b", "c", "flags", "conjB"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "bool"]), _)]),
	void cv_mulSpectrums_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_bool(const cv::_InputArray* a, const cv::_InputArray* b, const cv::_OutputArray* c, int flags, bool conjB, ResultVoid* ocvrs_return) {
		try {
			cv::mulSpectrums(*a, *b, *c, flags, conjB);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::mulTransposed(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1691
	// ("cv::mulTransposed", vec![(pred!(mut, ["src", "dst", "aTa"], ["const cv::_InputArray*", "const cv::_OutputArray*", "bool"]), _)]),
	void cv_mulTransposed_const__InputArrayR_const__OutputArrayR_bool(const cv::_InputArray* src, const cv::_OutputArray* dst, bool aTa, ResultVoid* ocvrs_return) {
		try {
			cv::mulTransposed(*src, *dst, aTa);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mulTransposed(InputArray, OutputArray, bool, InputArray, double, int)(InputArray, OutputArray, Primitive, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1691
	// ("cv::mulTransposed", vec![(pred!(mut, ["src", "dst", "aTa", "delta", "scale", "dtype"], ["const cv::_InputArray*", "const cv::_OutputArray*", "bool", "const cv::_InputArray*", "double", "int"]), _)]),
	void cv_mulTransposed_const__InputArrayR_const__OutputArrayR_bool_const__InputArrayR_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, bool aTa, const cv::_InputArray* delta, double scale, int dtype, ResultVoid* ocvrs_return) {
		try {
			cv::mulTransposed(*src, *dst, aTa, *delta, scale, dtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::multiply(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:441
	// ("cv::multiply", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_multiply_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::multiply(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// multiply(InputArray, InputArray, OutputArray, double, int)(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:441
	// ("cv::multiply", vec![(pred!(mut, ["src1", "src2", "dst", "scale", "dtype"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "int"]), _)]),
	void cv_multiply_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_int(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, double scale, int dtype, ResultVoid* ocvrs_return) {
		try {
			cv::multiply(*src1, *src2, *dst, scale, dtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// noArray()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:457
	// ("cv::noArray", vec![(pred!(mut, [], []), _)]),
	const cv::_InputOutputArray* cv_noArray() {
			const cv::_InputOutputArray ret = cv::noArray();
			return new const cv::_InputOutputArray(ret);
	}

	// norm(const SparseMat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:713
	// ("cv::norm", vec![(pred!(mut, ["src", "normType"], ["const cv::SparseMat*", "int"]), _)]),
	void cv_norm_const_SparseMatR_int(const cv::SparseMat* src, int normType, Result<double>* ocvrs_return) {
		try {
			double ret = cv::norm(*src, normType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::norm(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:694
	// ("cv::norm", vec![(pred!(mut, ["src1"], ["const cv::_InputArray*"]), _)]),
	void cv_norm_const__InputArrayR(const cv::_InputArray* src1, Result<double>* ocvrs_return) {
		try {
			double ret = cv::norm(*src1);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::norm(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:707
	// ("cv::norm", vec![(pred!(mut, ["src1", "src2"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_norm_const__InputArrayR_const__InputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, Result<double>* ocvrs_return) {
		try {
			double ret = cv::norm(*src1, *src2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// norm(InputArray, InputArray, int, InputArray)(InputArray, InputArray, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:707
	// ("cv::norm", vec![(pred!(mut, ["src1", "src2", "normType", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "const cv::_InputArray*"]), _)]),
	void cv_norm_const__InputArrayR_const__InputArrayR_int_const__InputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, int normType, const cv::_InputArray* mask, Result<double>* ocvrs_return) {
		try {
			double ret = cv::norm(*src1, *src2, normType, *mask);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// norm(InputArray, int, InputArray)(InputArray, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:694
	// ("cv::norm", vec![(pred!(mut, ["src1", "normType", "mask"], ["const cv::_InputArray*", "int", "const cv::_InputArray*"]), _)]),
	void cv_norm_const__InputArrayR_int_const__InputArrayR(const cv::_InputArray* src1, int normType, const cv::_InputArray* mask, Result<double>* ocvrs_return) {
		try {
			double ret = cv::norm(*src1, normType, *mask);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// normalize(const SparseMat &, SparseMat &, double, int)(TraitClass, TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:812
	// ("cv::normalize", vec![(pred!(mut, ["src", "dst", "alpha", "normType"], ["const cv::SparseMat*", "cv::SparseMat*", "double", "int"]), _)]),
	void cv_normalize_const_SparseMatR_SparseMatR_double_int(const cv::SparseMat* src, cv::SparseMat* dst, double alpha, int normType, ResultVoid* ocvrs_return) {
		try {
			cv::normalize(*src, *dst, alpha, normType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::normalize(InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:802
	// ("cv::normalize", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_normalize_const__InputArrayR_const__InputOutputArrayR(const cv::_InputArray* src, const cv::_InputOutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::normalize(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// normalize(InputArray, InputOutputArray, double, double, int, int, InputArray)(InputArray, InputOutputArray, Primitive, Primitive, Primitive, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:802
	// ("cv::normalize", vec![(pred!(mut, ["src", "dst", "alpha", "beta", "norm_type", "dtype", "mask"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "double", "double", "int", "int", "const cv::_InputArray*"]), _)]),
	void cv_normalize_const__InputArrayR_const__InputOutputArrayR_double_double_int_int_const__InputArrayR(const cv::_InputArray* src, const cv::_InputOutputArray* dst, double alpha, double beta, int norm_type, int dtype, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			cv::normalize(*src, *dst, alpha, beta, norm_type, dtype, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// attachContext(const String &, void *, void *, void *)(InString, Indirect, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:296
	// ("cv::ocl::attachContext", vec![(pred!(mut, ["platformName", "platformID", "context", "deviceID"], ["const cv::String*", "void*", "void*", "void*"]), _)]),
	void cv_ocl_attachContext_const_StringR_voidX_voidX_voidX(const char* platformName, void* platformID, void* context, void* deviceID, ResultVoid* ocvrs_return) {
		try {
			cv::ocl::attachContext(cv::String(platformName), platformID, context, deviceID);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// buildOptionsAddMatrixDescription(String &, const String &, InputArray)(OutString, InString, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:790
	// ("cv::ocl::buildOptionsAddMatrixDescription", vec![(pred!(mut, ["buildOptions", "name", "_m"], ["cv::String*", "const cv::String*", "const cv::_InputArray*"]), _)]),
	void cv_ocl_buildOptionsAddMatrixDescription_StringR_const_StringR_const__InputArrayR(void** buildOptions, const char* name, const cv::_InputArray* _m, ResultVoid* ocvrs_return) {
		try {
			cv::String buildOptions_out;
			cv::ocl::buildOptionsAddMatrixDescription(buildOptions_out, cv::String(name), *_m);
			*buildOptions = ocvrs_create_string(buildOptions_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::checkOptimalVectorWidth(Indirect, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:779
	// ("cv::ocl::checkOptimalVectorWidth", vec![(pred!(mut, ["vectorWidths", "src1"], ["const int*", "const cv::_InputArray*"]), _)]),
	void cv_ocl_checkOptimalVectorWidth_const_intX_const__InputArrayR(const int* vectorWidths, const cv::_InputArray* src1, Result<int>* ocvrs_return) {
		try {
			int ret = cv::ocl::checkOptimalVectorWidth(vectorWidths, *src1);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// checkOptimalVectorWidth(const int *, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, OclVectorStrategy)(Indirect, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:779
	// ("cv::ocl::checkOptimalVectorWidth", vec![(pred!(mut, ["vectorWidths", "src1", "src2", "src3", "src4", "src5", "src6", "src7", "src8", "src9", "strat"], ["const int*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::ocl::OclVectorStrategy"]), _)]),
	void cv_ocl_checkOptimalVectorWidth_const_intX_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_OclVectorStrategy(const int* vectorWidths, const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_InputArray* src3, const cv::_InputArray* src4, const cv::_InputArray* src5, const cv::_InputArray* src6, const cv::_InputArray* src7, const cv::_InputArray* src8, const cv::_InputArray* src9, cv::ocl::OclVectorStrategy strat, Result<int>* ocvrs_return) {
		try {
			int ret = cv::ocl::checkOptimalVectorWidth(vectorWidths, *src1, *src2, *src3, *src4, *src5, *src6, *src7, *src8, *src9, strat);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertFromBuffer(void *, size_t, int, int, int, UMat &)(Indirect, Primitive, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:310
	// ("cv::ocl::convertFromBuffer", vec![(pred!(mut, ["cl_mem_buffer", "step", "rows", "cols", "type", "dst"], ["void*", "size_t", "int", "int", "int", "cv::UMat*"]), _)]),
	void cv_ocl_convertFromBuffer_voidX_size_t_int_int_int_UMatR(void* cl_mem_buffer, size_t step, int rows, int cols, int type, cv::UMat* dst, ResultVoid* ocvrs_return) {
		try {
			cv::ocl::convertFromBuffer(cl_mem_buffer, step, rows, cols, type, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertFromImage(void *, UMat &)(Indirect, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:319
	// ("cv::ocl::convertFromImage", vec![(pred!(mut, ["cl_mem_image", "dst"], ["void*", "cv::UMat*"]), _)]),
	void cv_ocl_convertFromImage_voidX_UMatR(void* cl_mem_image, cv::UMat* dst, ResultVoid* ocvrs_return) {
		try {
			cv::ocl::convertFromImage(cl_mem_image, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertTypeStr(int, int, int, char *)(Primitive, Primitive, Primitive, OutString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:753
	// ("cv::ocl::convertTypeStr", vec![(pred!(mut, ["sdepth", "ddepth", "cn", "buf"], ["int", "int", "int", "char*"]), _)]),
	void cv_ocl_convertTypeStr_int_int_int_charX(int sdepth, int ddepth, int cn, void** buf, Result<void*>* ocvrs_return) {
		try {
			std::unique_ptr<char[]> buf_out = std::make_unique<char[]>(1024);
			const char* ret = cv::ocl::convertTypeStr(sdepth, ddepth, cn, buf_out.get());
			*buf = ocvrs_create_string(buf_out.get());
			Ok(ocvrs_create_string(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// finish()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:57
	// ("cv::ocl::finish", vec![(pred!(mut, [], []), _)]),
	void cv_ocl_finish(ResultVoid* ocvrs_return) {
		try {
			cv::ocl::finish();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOpenCLErrorString(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:757
	// ("cv::ocl::getOpenCLErrorString", vec![(pred!(mut, ["errorCode"], ["int"]), _)]),
	void cv_ocl_getOpenCLErrorString_int(int errorCode, Result<void*>* ocvrs_return) {
		try {
			const char* ret = cv::ocl::getOpenCLErrorString(errorCode);
			Ok(ocvrs_create_string(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPlatfomsInfo(std::vector<PlatformInfo> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:759
	// ("cv::ocl::getPlatfomsInfo", vec![(pred!(mut, ["platform_info"], ["std::vector<cv::ocl::PlatformInfo>*"]), _)]),
	void cv_ocl_getPlatfomsInfo_vectorLPlatformInfoGR(std::vector<cv::ocl::PlatformInfo>* platform_info, ResultVoid* ocvrs_return) {
		try {
			cv::ocl::getPlatfomsInfo(*platform_info);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// haveAmdBlas()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:54
	// ("cv::ocl::haveAmdBlas", vec![(pred!(mut, [], []), _)]),
	void cv_ocl_haveAmdBlas(Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::ocl::haveAmdBlas();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// haveAmdFft()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:55
	// ("cv::ocl::haveAmdFft", vec![(pred!(mut, [], []), _)]),
	void cv_ocl_haveAmdFft(Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::ocl::haveAmdFft();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// haveOpenCL()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:52
	// ("cv::ocl::haveOpenCL", vec![(pred!(mut, [], []), _)]),
	void cv_ocl_haveOpenCL(Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::ocl::haveOpenCL();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// haveSVM()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:59
	// ("cv::ocl::haveSVM", vec![(pred!(mut, [], []), _)]),
	void cv_ocl_haveSVM(Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::ocl::haveSVM();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::kernelToStr(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:758
	// ("cv::ocl::kernelToStr", vec![(pred!(mut, ["_kernel"], ["const cv::_InputArray*"]), _)]),
	void cv_ocl_kernelToStr_const__InputArrayR(const cv::_InputArray* _kernel, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::ocl::kernelToStr(*_kernel);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// kernelToStr(InputArray, int, const char *)(InputArray, Primitive, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:758
	// ("cv::ocl::kernelToStr", vec![(pred!(mut, ["_kernel", "ddepth", "name"], ["const cv::_InputArray*", "int", "const char*"]), _)]),
	void cv_ocl_kernelToStr_const__InputArrayR_int_const_charX(const cv::_InputArray* _kernel, int ddepth, const char* name, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::ocl::kernelToStr(*_kernel, ddepth, name);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// memopTypeToStr(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:755
	// ("cv::ocl::memopTypeToStr", vec![(pred!(mut, ["t"], ["int"]), _)]),
	void cv_ocl_memopTypeToStr_int(int t, Result<void*>* ocvrs_return) {
		try {
			const char* ret = cv::ocl::memopTypeToStr(t);
			Ok(ocvrs_create_string(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::predictOptimalVectorWidthMax(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:786
	// ("cv::ocl::predictOptimalVectorWidthMax", vec![(pred!(mut, ["src1"], ["const cv::_InputArray*"]), _)]),
	void cv_ocl_predictOptimalVectorWidthMax_const__InputArrayR(const cv::_InputArray* src1, Result<int>* ocvrs_return) {
		try {
			int ret = cv::ocl::predictOptimalVectorWidthMax(*src1);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// predictOptimalVectorWidthMax(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:786
	// ("cv::ocl::predictOptimalVectorWidthMax", vec![(pred!(mut, ["src1", "src2", "src3", "src4", "src5", "src6", "src7", "src8", "src9"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_ocl_predictOptimalVectorWidthMax_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_InputArray* src3, const cv::_InputArray* src4, const cv::_InputArray* src5, const cv::_InputArray* src6, const cv::_InputArray* src7, const cv::_InputArray* src8, const cv::_InputArray* src9, Result<int>* ocvrs_return) {
		try {
			int ret = cv::ocl::predictOptimalVectorWidthMax(*src1, *src2, *src3, *src4, *src5, *src6, *src7, *src8, *src9);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::predictOptimalVectorWidth(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:774
	// ("cv::ocl::predictOptimalVectorWidth", vec![(pred!(mut, ["src1"], ["const cv::_InputArray*"]), _)]),
	void cv_ocl_predictOptimalVectorWidth_const__InputArrayR(const cv::_InputArray* src1, Result<int>* ocvrs_return) {
		try {
			int ret = cv::ocl::predictOptimalVectorWidth(*src1);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// predictOptimalVectorWidth(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, OclVectorStrategy)(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:774
	// ("cv::ocl::predictOptimalVectorWidth", vec![(pred!(mut, ["src1", "src2", "src3", "src4", "src5", "src6", "src7", "src8", "src9", "strat"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::ocl::OclVectorStrategy"]), _)]),
	void cv_ocl_predictOptimalVectorWidth_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_OclVectorStrategy(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_InputArray* src3, const cv::_InputArray* src4, const cv::_InputArray* src5, const cv::_InputArray* src6, const cv::_InputArray* src7, const cv::_InputArray* src8, const cv::_InputArray* src9, cv::ocl::OclVectorStrategy strat, Result<int>* ocvrs_return) {
		try {
			int ret = cv::ocl::predictOptimalVectorWidth(*src1, *src2, *src3, *src4, *src5, *src6, *src7, *src8, *src9, strat);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseOpenCL(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:56
	// ("cv::ocl::setUseOpenCL", vec![(pred!(mut, ["flag"], ["bool"]), _)]),
	void cv_ocl_setUseOpenCL_bool(bool flag, ResultVoid* ocvrs_return) {
		try {
			cv::ocl::setUseOpenCL(flag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// typeToStr(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:754
	// ("cv::ocl::typeToStr", vec![(pred!(mut, ["t"], ["int"]), _)]),
	void cv_ocl_typeToStr_int(int t, Result<void*>* ocvrs_return) {
		try {
			const char* ret = cv::ocl::typeToStr(t);
			Ok(ocvrs_create_string(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// useOpenCL()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:53
	// ("cv::ocl::useOpenCL", vec![(pred!(mut, [], []), _)]),
	void cv_ocl_useOpenCL(Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::ocl::useOpenCL();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// vecopTypeToStr(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:756
	// ("cv::ocl::vecopTypeToStr", vec![(pred!(mut, ["t"], ["int"]), _)]),
	void cv_ocl_vecopTypeToStr_int(int t, Result<void*>* ocvrs_return) {
		try {
			const char* ret = cv::ocl::vecopTypeToStr(t);
			Ok(ocvrs_create_string(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertFromGLTexture2D(const Texture2D &, OutputArray)(TraitClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:538
	// ("cv::ogl::convertFromGLTexture2D", vec![(pred!(mut, ["texture", "dst"], ["const cv::ogl::Texture2D*", "const cv::_OutputArray*"]), _)]),
	void cv_ogl_convertFromGLTexture2D_const_Texture2DR_const__OutputArrayR(const cv::ogl::Texture2D* texture, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::ogl::convertFromGLTexture2D(*texture, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertToGLTexture2D(InputArray, Texture2D &)(InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:532
	// ("cv::ogl::convertToGLTexture2D", vec![(pred!(mut, ["src", "texture"], ["const cv::_InputArray*", "cv::ogl::Texture2D*"]), _)]),
	void cv_ogl_convertToGLTexture2D_const__InputArrayR_Texture2DR(const cv::_InputArray* src, cv::ogl::Texture2D* texture, ResultVoid* ocvrs_return) {
		try {
			cv::ogl::convertToGLTexture2D(*src, *texture);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ogl::mapGLBuffer(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:551
	// ("cv::ogl::mapGLBuffer", vec![(pred!(mut, ["buffer"], ["const cv::ogl::Buffer*"]), _)]),
	void cv_ogl_mapGLBuffer_const_BufferR(const cv::ogl::Buffer* buffer, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = cv::ogl::mapGLBuffer(*buffer);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mapGLBuffer(const Buffer &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:551
	// ("cv::ogl::mapGLBuffer", vec![(pred!(mut, ["buffer", "accessFlags"], ["const cv::ogl::Buffer*", "int"]), _)]),
	void cv_ogl_mapGLBuffer_const_BufferR_int(const cv::ogl::Buffer* buffer, int accessFlags, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = cv::ogl::mapGLBuffer(*buffer, accessFlags);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initializeContextFromGL()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:524
	// ("cv::ogl::ocl::initializeContextFromGL", vec![(pred!(mut, [], []), _)]),
	void cv_ogl_ocl_initializeContextFromGL(Result<cv::ocl::Context*>* ocvrs_return) {
		try {
			cv::ocl::Context ret = cv::ogl::ocl::initializeContextFromGL();
			Ok(new cv::ocl::Context(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ogl::render(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:505
	// ("cv::ogl::render", vec![(pred!(mut, ["arr"], ["const cv::ogl::Arrays*"]), _)]),
	void cv_ogl_render_const_ArraysR(const cv::ogl::Arrays* arr, ResultVoid* ocvrs_return) {
		try {
			cv::ogl::render(*arr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ogl::render(TraitClass, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:513
	// ("cv::ogl::render", vec![(pred!(mut, ["arr", "indices"], ["const cv::ogl::Arrays*", "const cv::_InputArray*"]), _)]),
	void cv_ogl_render_const_ArraysR_const__InputArrayR(const cv::ogl::Arrays* arr, const cv::_InputArray* indices, ResultVoid* ocvrs_return) {
		try {
			cv::ogl::render(*arr, *indices);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// render(const Arrays &, InputArray, int, Scalar)(TraitClass, InputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:513
	// ("cv::ogl::render", vec![(pred!(mut, ["arr", "indices", "mode", "color"], ["const cv::ogl::Arrays*", "const cv::_InputArray*", "int", "cv::Scalar"]), _)]),
	void cv_ogl_render_const_ArraysR_const__InputArrayR_int_Scalar(const cv::ogl::Arrays* arr, const cv::_InputArray* indices, int mode, cv::Scalar* color, ResultVoid* ocvrs_return) {
		try {
			cv::ogl::render(*arr, *indices, mode, *color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// render(const Arrays &, int, Scalar)(TraitClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:505
	// ("cv::ogl::render", vec![(pred!(mut, ["arr", "mode", "color"], ["const cv::ogl::Arrays*", "int", "cv::Scalar"]), _)]),
	void cv_ogl_render_const_ArraysR_int_Scalar(const cv::ogl::Arrays* arr, int mode, cv::Scalar* color, ResultVoid* ocvrs_return) {
		try {
			cv::ogl::render(*arr, mode, *color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ogl::render(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:496
	// ("cv::ogl::render", vec![(pred!(mut, ["tex"], ["const cv::ogl::Texture2D*"]), _)]),
	void cv_ogl_render_const_Texture2DR(const cv::ogl::Texture2D* tex, ResultVoid* ocvrs_return) {
		try {
			cv::ogl::render(*tex);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// render(const Texture2D &, Rect_<double>, Rect_<double>)(TraitClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:496
	// ("cv::ogl::render", vec![(pred!(mut, ["tex", "wndRect", "texRect"], ["const cv::ogl::Texture2D*", "cv::Rect_<double>", "cv::Rect_<double>"]), _)]),
	void cv_ogl_render_const_Texture2DR_Rect_LdoubleG_Rect_LdoubleG(const cv::ogl::Texture2D* tex, cv::Rect_<double>* wndRect, cv::Rect_<double>* texRect, ResultVoid* ocvrs_return) {
		try {
			cv::ogl::render(*tex, *wndRect, *texRect);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// unmapGLBuffer(UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:559
	// ("cv::ogl::unmapGLBuffer", vec![(pred!(mut, ["u"], ["cv::UMat*"]), _)]),
	void cv_ogl_unmapGLBuffer_UMatR(cv::UMat* u, ResultVoid* ocvrs_return) {
		try {
			cv::ogl::unmapGLBuffer(*u);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator+(const MatExpr &, const MatExpr &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3634
	// ("cv::operator+", vec![(pred!(mut, ["e1", "e2"], ["const cv::MatExpr*", "const cv::MatExpr*"]), _)]),
	void cv_operatorA_const_MatExprR_const_MatExprR(const cv::MatExpr* e1, const cv::MatExpr* e2, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator+(*e1, *e2);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator+(const MatExpr &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3630
	// ("cv::operator+", vec![(pred!(mut, ["e", "m"], ["const cv::MatExpr*", "const cv::Mat*"]), _)]),
	void cv_operatorA_const_MatExprR_const_MatR(const cv::MatExpr* e, const cv::Mat* m, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator+(*e, *m);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator+(const MatExpr &, const Scalar &)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3632
	// ("cv::operator+", vec![(pred!(mut, ["e", "s"], ["const cv::MatExpr*", "const cv::Scalar*"]), _)]),
	void cv_operatorA_const_MatExprR_const_ScalarR(const cv::MatExpr* e, const cv::Scalar* s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator+(*e, *s);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator+(const Mat &, const MatExpr &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3631
	// ("cv::operator+", vec![(pred!(mut, ["m", "e"], ["const cv::Mat*", "const cv::MatExpr*"]), _)]),
	void cv_operatorA_const_MatR_const_MatExprR(const cv::Mat* m, const cv::MatExpr* e, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator+(*m, *e);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator+(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3627
	// ("cv::operator+", vec![(pred!(mut, ["a", "b"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_operatorA_const_MatR_const_MatR(const cv::Mat* a, const cv::Mat* b, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator+(*a, *b);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator+(const Mat &, const Scalar &)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3628
	// ("cv::operator+", vec![(pred!(mut, ["a", "s"], ["const cv::Mat*", "const cv::Scalar*"]), _)]),
	void cv_operatorA_const_MatR_const_ScalarR(const cv::Mat* a, const cv::Scalar* s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator+(*a, *s);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator+(const Scalar &, const MatExpr &)(SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3633
	// ("cv::operator+", vec![(pred!(mut, ["s", "e"], ["const cv::Scalar*", "const cv::MatExpr*"]), _)]),
	void cv_operatorA_const_ScalarR_const_MatExprR(const cv::Scalar* s, const cv::MatExpr* e, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator+(*s, *e);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator+(const Scalar &, const Mat &)(SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3629
	// ("cv::operator+", vec![(pred!(mut, ["s", "a"], ["const cv::Scalar*", "const cv::Mat*"]), _)]),
	void cv_operatorA_const_ScalarR_const_MatR(const cv::Scalar* s, const cv::Mat* a, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator+(*s, *a);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator/(const MatExpr &, const MatExpr &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3676
	// ("cv::operator/", vec![(pred!(mut, ["e1", "e2"], ["const cv::MatExpr*", "const cv::MatExpr*"]), _)]),
	void cv_operatorD_const_MatExprR_const_MatExprR(const cv::MatExpr* e1, const cv::MatExpr* e2, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator/(*e1, *e2);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator/(const MatExpr &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3672
	// ("cv::operator/", vec![(pred!(mut, ["e", "m"], ["const cv::MatExpr*", "const cv::Mat*"]), _)]),
	void cv_operatorD_const_MatExprR_const_MatR(const cv::MatExpr* e, const cv::Mat* m, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator/(*e, *m);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator/(const MatExpr &, double)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3674
	// ("cv::operator/", vec![(pred!(mut, ["e", "s"], ["const cv::MatExpr*", "double"]), _)]),
	void cv_operatorD_const_MatExprR_double(const cv::MatExpr* e, double s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator/(*e, s);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator/(const Mat &, const MatExpr &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3673
	// ("cv::operator/", vec![(pred!(mut, ["m", "e"], ["const cv::Mat*", "const cv::MatExpr*"]), _)]),
	void cv_operatorD_const_MatR_const_MatExprR(const cv::Mat* m, const cv::MatExpr* e, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator/(*m, *e);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator/(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3669
	// ("cv::operator/", vec![(pred!(mut, ["a", "b"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_operatorD_const_MatR_const_MatR(const cv::Mat* a, const cv::Mat* b, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator/(*a, *b);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator/(const Mat &, double)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3670
	// ("cv::operator/", vec![(pred!(mut, ["a", "s"], ["const cv::Mat*", "double"]), _)]),
	void cv_operatorD_const_MatR_double(const cv::Mat* a, double s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator/(*a, s);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator/(double, const MatExpr &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3675
	// ("cv::operator/", vec![(pred!(mut, ["s", "e"], ["double", "const cv::MatExpr*"]), _)]),
	void cv_operatorD_double_const_MatExprR(double s, const cv::MatExpr* e, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator/(s, *e);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator/(double, const Mat &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3671
	// ("cv::operator/", vec![(pred!(mut, ["s", "a"], ["double", "const cv::Mat*"]), _)]),
	void cv_operatorD_double_const_MatR(double s, const cv::Mat* a, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator/(s, *a);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator==(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3698
	// ("cv::operator==", vec![(pred!(mut, ["a", "b"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_operatorEQ_const_MatR_const_MatR(const cv::Mat* a, const cv::Mat* b, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator==(*a, *b);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator==(const Mat &, double)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3699
	// ("cv::operator==", vec![(pred!(mut, ["a", "s"], ["const cv::Mat*", "double"]), _)]),
	void cv_operatorEQ_const_MatR_double(const cv::Mat* a, double s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator==(*a, s);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator==(double, const Mat &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3700
	// ("cv::operator==", vec![(pred!(mut, ["s", "a"], ["double", "const cv::Mat*"]), _)]),
	void cv_operatorEQ_double_const_MatR(double s, const cv::Mat* a, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator==(s, *a);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator>=(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3714
	// ("cv::operator>=", vec![(pred!(mut, ["a", "b"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_operatorGE_const_MatR_const_MatR(const cv::Mat* a, const cv::Mat* b, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator>=(*a, *b);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator>=(const Mat &, double)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3715
	// ("cv::operator>=", vec![(pred!(mut, ["a", "s"], ["const cv::Mat*", "double"]), _)]),
	void cv_operatorGE_const_MatR_double(const cv::Mat* a, double s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator>=(*a, s);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator>=(double, const Mat &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3716
	// ("cv::operator>=", vec![(pred!(mut, ["s", "a"], ["double", "const cv::Mat*"]), _)]),
	void cv_operatorGE_double_const_MatR(double s, const cv::Mat* a, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator>=(s, *a);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator>(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3722
	// ("cv::operator>", vec![(pred!(mut, ["a", "b"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_operatorG_const_MatR_const_MatR(const cv::Mat* a, const cv::Mat* b, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator>(*a, *b);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator>(const Mat &, double)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3723
	// ("cv::operator>", vec![(pred!(mut, ["a", "s"], ["const cv::Mat*", "double"]), _)]),
	void cv_operatorG_const_MatR_double(const cv::Mat* a, double s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator>(*a, s);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator>(double, const Mat &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3724
	// ("cv::operator>", vec![(pred!(mut, ["s", "a"], ["double", "const cv::Mat*"]), _)]),
	void cv_operatorG_double_const_MatR(double s, const cv::Mat* a, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator>(s, *a);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator<=(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3690
	// ("cv::operator<=", vec![(pred!(mut, ["a", "b"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_operatorLE_const_MatR_const_MatR(const cv::Mat* a, const cv::Mat* b, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator<=(*a, *b);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator<=(const Mat &, double)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3691
	// ("cv::operator<=", vec![(pred!(mut, ["a", "s"], ["const cv::Mat*", "double"]), _)]),
	void cv_operatorLE_const_MatR_double(const cv::Mat* a, double s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator<=(*a, s);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator<=(double, const Mat &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3692
	// ("cv::operator<=", vec![(pred!(mut, ["s", "a"], ["double", "const cv::Mat*"]), _)]),
	void cv_operatorLE_double_const_MatR(double s, const cv::Mat* a, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator<=(s, *a);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator<(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3682
	// ("cv::operator<", vec![(pred!(mut, ["a", "b"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_operatorL_const_MatR_const_MatR(const cv::Mat* a, const cv::Mat* b, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator<(*a, *b);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator<(const Mat &, double)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3683
	// ("cv::operator<", vec![(pred!(mut, ["a", "s"], ["const cv::Mat*", "double"]), _)]),
	void cv_operatorL_const_MatR_double(const cv::Mat* a, double s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator<(*a, s);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator<(double, const Mat &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3684
	// ("cv::operator<", vec![(pred!(mut, ["s", "a"], ["double", "const cv::Mat*"]), _)]),
	void cv_operatorL_double_const_MatR(double s, const cv::Mat* a, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator<(s, *a);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator!=(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3706
	// ("cv::operator!=", vec![(pred!(mut, ["a", "b"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_operatorNE_const_MatR_const_MatR(const cv::Mat* a, const cv::Mat* b, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator!=(*a, *b);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator!=(const Mat &, double)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3707
	// ("cv::operator!=", vec![(pred!(mut, ["a", "s"], ["const cv::Mat*", "double"]), _)]),
	void cv_operatorNE_const_MatR_double(const cv::Mat* a, double s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator!=(*a, s);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator!=(double, const Mat &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3708
	// ("cv::operator!=", vec![(pred!(mut, ["s", "a"], ["double", "const cv::Mat*"]), _)]),
	void cv_operatorNE_double_const_MatR(double s, const cv::Mat* a, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator!=(s, *a);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator~(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3754
	// ("cv::operator~", vec![(pred!(mut, ["m"], ["const cv::Mat*"]), _)]),
	void cv_operatorNOTB_const_MatR(const cv::Mat* m, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator~(*m);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator|(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3738
	// ("cv::operator|", vec![(pred!(mut, ["a", "b"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_operatorOR_const_MatR_const_MatR(const cv::Mat* a, const cv::Mat* b, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator|(*a, *b);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator|(const Mat &, const Scalar &)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3739
	// ("cv::operator|", vec![(pred!(mut, ["a", "s"], ["const cv::Mat*", "const cv::Scalar*"]), _)]),
	void cv_operatorOR_const_MatR_const_ScalarR(const cv::Mat* a, const cv::Scalar* s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator|(*a, *s);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator|(const Scalar &, const Mat &)(SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3740
	// ("cv::operator|", vec![(pred!(mut, ["s", "a"], ["const cv::Scalar*", "const cv::Mat*"]), _)]),
	void cv_operatorOR_const_ScalarR_const_MatR(const cv::Scalar* s, const cv::Mat* a, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator|(*s, *a);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator&(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3730
	// ("cv::operator&", vec![(pred!(mut, ["a", "b"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_operatorR_const_MatR_const_MatR(const cv::Mat* a, const cv::Mat* b, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator&(*a, *b);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator&(const Mat &, const Scalar &)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3731
	// ("cv::operator&", vec![(pred!(mut, ["a", "s"], ["const cv::Mat*", "const cv::Scalar*"]), _)]),
	void cv_operatorR_const_MatR_const_ScalarR(const cv::Mat* a, const cv::Scalar* s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator&(*a, *s);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator&(const Scalar &, const Mat &)(SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3732
	// ("cv::operator&", vec![(pred!(mut, ["s", "a"], ["const cv::Scalar*", "const cv::Mat*"]), _)]),
	void cv_operatorR_const_ScalarR_const_MatR(const cv::Scalar* s, const cv::Mat* a, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator&(*s, *a);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator-(const MatExpr &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3654
	// ("cv::operator-", vec![(pred!(mut, ["e"], ["const cv::MatExpr*"]), _)]),
	void cv_operatorS_const_MatExprR(const cv::MatExpr* e, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator-(*e);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator-(const MatExpr &, const MatExpr &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3647
	// ("cv::operator-", vec![(pred!(mut, ["e1", "e2"], ["const cv::MatExpr*", "const cv::MatExpr*"]), _)]),
	void cv_operatorS_const_MatExprR_const_MatExprR(const cv::MatExpr* e1, const cv::MatExpr* e2, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator-(*e1, *e2);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator-(const MatExpr &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3643
	// ("cv::operator-", vec![(pred!(mut, ["e", "m"], ["const cv::MatExpr*", "const cv::Mat*"]), _)]),
	void cv_operatorS_const_MatExprR_const_MatR(const cv::MatExpr* e, const cv::Mat* m, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator-(*e, *m);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator-(const MatExpr &, const Scalar &)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3645
	// ("cv::operator-", vec![(pred!(mut, ["e", "s"], ["const cv::MatExpr*", "const cv::Scalar*"]), _)]),
	void cv_operatorS_const_MatExprR_const_ScalarR(const cv::MatExpr* e, const cv::Scalar* s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator-(*e, *s);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator-(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3653
	// ("cv::operator-", vec![(pred!(mut, ["m"], ["const cv::Mat*"]), _)]),
	void cv_operatorS_const_MatR(const cv::Mat* m, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator-(*m);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator-(const Mat &, const MatExpr &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3644
	// ("cv::operator-", vec![(pred!(mut, ["m", "e"], ["const cv::Mat*", "const cv::MatExpr*"]), _)]),
	void cv_operatorS_const_MatR_const_MatExprR(const cv::Mat* m, const cv::MatExpr* e, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator-(*m, *e);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator-(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3640
	// ("cv::operator-", vec![(pred!(mut, ["a", "b"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_operatorS_const_MatR_const_MatR(const cv::Mat* a, const cv::Mat* b, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator-(*a, *b);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator-(const Mat &, const Scalar &)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3641
	// ("cv::operator-", vec![(pred!(mut, ["a", "s"], ["const cv::Mat*", "const cv::Scalar*"]), _)]),
	void cv_operatorS_const_MatR_const_ScalarR(const cv::Mat* a, const cv::Scalar* s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator-(*a, *s);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator-(const Scalar &, const MatExpr &)(SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3646
	// ("cv::operator-", vec![(pred!(mut, ["s", "e"], ["const cv::Scalar*", "const cv::MatExpr*"]), _)]),
	void cv_operatorS_const_ScalarR_const_MatExprR(const cv::Scalar* s, const cv::MatExpr* e, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator-(*s, *e);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator-(const Scalar &, const Mat &)(SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3642
	// ("cv::operator-", vec![(pred!(mut, ["s", "a"], ["const cv::Scalar*", "const cv::Mat*"]), _)]),
	void cv_operatorS_const_ScalarR_const_MatR(const cv::Scalar* s, const cv::Mat* a, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator-(*s, *a);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator^(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3746
	// ("cv::operator^", vec![(pred!(mut, ["a", "b"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_operatorXOR_const_MatR_const_MatR(const cv::Mat* a, const cv::Mat* b, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator^(*a, *b);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator^(const Mat &, const Scalar &)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3747
	// ("cv::operator^", vec![(pred!(mut, ["a", "s"], ["const cv::Mat*", "const cv::Scalar*"]), _)]),
	void cv_operatorXOR_const_MatR_const_ScalarR(const cv::Mat* a, const cv::Scalar* s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator^(*a, *s);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator^(const Scalar &, const Mat &)(SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3748
	// ("cv::operator^", vec![(pred!(mut, ["s", "a"], ["const cv::Scalar*", "const cv::Mat*"]), _)]),
	void cv_operatorXOR_const_ScalarR_const_MatR(const cv::Scalar* s, const cv::Mat* a, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator^(*s, *a);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator*(const MatExpr &, const MatExpr &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3663
	// ("cv::operator*", vec![(pred!(mut, ["e1", "e2"], ["const cv::MatExpr*", "const cv::MatExpr*"]), _)]),
	void cv_operatorX_const_MatExprR_const_MatExprR(const cv::MatExpr* e1, const cv::MatExpr* e2, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator*(*e1, *e2);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator*(const MatExpr &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3659
	// ("cv::operator*", vec![(pred!(mut, ["e", "m"], ["const cv::MatExpr*", "const cv::Mat*"]), _)]),
	void cv_operatorX_const_MatExprR_const_MatR(const cv::MatExpr* e, const cv::Mat* m, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator*(*e, *m);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator*(const MatExpr &, double)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3661
	// ("cv::operator*", vec![(pred!(mut, ["e", "s"], ["const cv::MatExpr*", "double"]), _)]),
	void cv_operatorX_const_MatExprR_double(const cv::MatExpr* e, double s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator*(*e, s);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator*(const Mat &, const MatExpr &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3660
	// ("cv::operator*", vec![(pred!(mut, ["m", "e"], ["const cv::Mat*", "const cv::MatExpr*"]), _)]),
	void cv_operatorX_const_MatR_const_MatExprR(const cv::Mat* m, const cv::MatExpr* e, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator*(*m, *e);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator*(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3656
	// ("cv::operator*", vec![(pred!(mut, ["a", "b"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_operatorX_const_MatR_const_MatR(const cv::Mat* a, const cv::Mat* b, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator*(*a, *b);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator*(const Mat &, double)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3657
	// ("cv::operator*", vec![(pred!(mut, ["a", "s"], ["const cv::Mat*", "double"]), _)]),
	void cv_operatorX_const_MatR_double(const cv::Mat* a, double s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator*(*a, s);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator*(double, const MatExpr &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3662
	// ("cv::operator*", vec![(pred!(mut, ["s", "e"], ["double", "const cv::MatExpr*"]), _)]),
	void cv_operatorX_double_const_MatExprR(double s, const cv::MatExpr* e, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator*(s, *e);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator*(double, const Mat &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3658
	// ("cv::operator*", vec![(pred!(mut, ["s", "a"], ["double", "const cv::Mat*"]), _)]),
	void cv_operatorX_double_const_MatR(double s, const cv::Mat* a, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator*(s, *a);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::parallel_for_(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:581
	// ("cv::parallel_for_", vec![(pred!(mut, ["range", "body"], ["const cv::Range*", "const cv::ParallelLoopBody*"]), _)]),
	void cv_parallel_for__const_RangeR_const_ParallelLoopBodyR(const cv::Range* range, const cv::ParallelLoopBody* body, ResultVoid* ocvrs_return) {
		try {
			cv::parallel_for_(*range, *body);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// parallel_for_(const Range &, const ParallelLoopBody &, double)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:581
	// ("cv::parallel_for_", vec![(pred!(mut, ["range", "body", "nstripes"], ["const cv::Range*", "const cv::ParallelLoopBody*", "double"]), _)]),
	void cv_parallel_for__const_RangeR_const_ParallelLoopBodyR_double(const cv::Range* range, const cv::ParallelLoopBody* body, double nstripes, ResultVoid* ocvrs_return) {
		try {
			cv::parallel_for_(*range, *body, nstripes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::patchNaNs(InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1630
	// ("cv::patchNaNs", vec![(pred!(mut, ["a"], ["const cv::_InputOutputArray*"]), _)]),
	void cv_patchNaNs_const__InputOutputArrayR(const cv::_InputOutputArray* a, ResultVoid* ocvrs_return) {
		try {
			cv::patchNaNs(*a);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// patchNaNs(InputOutputArray, double)(InputOutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1630
	// ("cv::patchNaNs", vec![(pred!(mut, ["a", "val"], ["const cv::_InputOutputArray*", "double"]), _)]),
	void cv_patchNaNs_const__InputOutputArrayR_double(const cv::_InputOutputArray* a, double val, ResultVoid* ocvrs_return) {
		try {
			cv::patchNaNs(*a, val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// perspectiveTransform(InputArray, OutputArray, InputArray)(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1757
	// ("cv::perspectiveTransform", vec![(pred!(mut, ["src", "dst", "m"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_perspectiveTransform_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* m, ResultVoid* ocvrs_return) {
		try {
			cv::perspectiveTransform(*src, *dst, *m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::phase(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1592
	// ("cv::phase", vec![(pred!(mut, ["x", "y", "angle"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_phase_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* x, const cv::_InputArray* y, const cv::_OutputArray* angle, ResultVoid* ocvrs_return) {
		try {
			cv::phase(*x, *y, *angle);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// phase(InputArray, InputArray, OutputArray, bool)(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1592
	// ("cv::phase", vec![(pred!(mut, ["x", "y", "angle", "angleInDegrees"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "bool"]), _)]),
	void cv_phase_const__InputArrayR_const__InputArrayR_const__OutputArrayR_bool(const cv::_InputArray* x, const cv::_InputArray* y, const cv::_OutputArray* angle, bool angleInDegrees, ResultVoid* ocvrs_return) {
		try {
			cv::phase(*x, *y, *angle, angleInDegrees);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::polarToCart(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1551
	// ("cv::polarToCart", vec![(pred!(mut, ["magnitude", "angle", "x", "y"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_polarToCart_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* magnitude, const cv::_InputArray* angle, const cv::_OutputArray* x, const cv::_OutputArray* y, ResultVoid* ocvrs_return) {
		try {
			cv::polarToCart(*magnitude, *angle, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// polarToCart(InputArray, InputArray, OutputArray, OutputArray, bool)(InputArray, InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1551
	// ("cv::polarToCart", vec![(pred!(mut, ["magnitude", "angle", "x", "y", "angleInDegrees"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool"]), _)]),
	void cv_polarToCart_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool(const cv::_InputArray* magnitude, const cv::_InputArray* angle, const cv::_OutputArray* x, const cv::_OutputArray* y, bool angleInDegrees, ResultVoid* ocvrs_return) {
		try {
			cv::polarToCart(*magnitude, *angle, *x, *y, angleInDegrees);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// pow(InputArray, double, OutputArray)(InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1500
	// ("cv::pow", vec![(pred!(mut, ["src", "power", "dst"], ["const cv::_InputArray*", "double", "const cv::_OutputArray*"]), _)]),
	void cv_pow_const__InputArrayR_double_const__OutputArrayR(const cv::_InputArray* src, double power, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::pow(*src, power, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::randShuffle(InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2336
	// ("cv::randShuffle", vec![(pred!(mut, ["dst"], ["const cv::_InputOutputArray*"]), _)]),
	void cv_randShuffle_const__InputOutputArrayR(const cv::_InputOutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::randShuffle(*dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// randShuffle(InputOutputArray, double, RNG *)(InputOutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2336
	// ("cv::randShuffle", vec![(pred!(mut, ["dst", "iterFactor", "rng"], ["const cv::_InputOutputArray*", "double", "cv::RNG*"]), _)]),
	void cv_randShuffle_const__InputOutputArrayR_double_RNGX(const cv::_InputOutputArray* dst, double iterFactor, cv::RNG* rng, ResultVoid* ocvrs_return) {
		try {
			cv::randShuffle(*dst, iterFactor, rng);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// randn(InputOutputArray, InputArray, InputArray)(InputOutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2323
	// ("cv::randn", vec![(pred!(mut, ["dst", "mean", "stddev"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_randn_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputOutputArray* dst, const cv::_InputArray* mean, const cv::_InputArray* stddev, ResultVoid* ocvrs_return) {
		try {
			cv::randn(*dst, *mean, *stddev);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// randu(InputOutputArray, InputArray, InputArray)(InputOutputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2310
	// ("cv::randu", vec![(pred!(mut, ["dst", "low", "high"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_randu_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputOutputArray* dst, const cv::_InputArray* low, const cv::_InputArray* high, ResultVoid* ocvrs_return) {
		try {
			cv::randu(*dst, *low, *high);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &, DMatch &, const DMatch &)(TraitClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:751
	// ("cv::read", vec![(pred!(mut, ["node", "value", "default_value"], ["const cv::FileNode*", "cv::DMatch*", "const cv::DMatch*"]), _)]),
	void cv_read_const_FileNodeR_DMatchR_const_DMatchR(const cv::FileNode* node, cv::DMatch* value, const cv::DMatch* default_value, ResultVoid* ocvrs_return) {
		try {
			cv::read(*node, *value, *default_value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &, KeyPoint &, const KeyPoint &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:750
	// ("cv::read", vec![(pred!(mut, ["node", "value", "default_value"], ["const cv::FileNode*", "cv::KeyPoint*", "const cv::KeyPoint*"]), _)]),
	void cv_read_const_FileNodeR_KeyPointR_const_KeyPointR(const cv::FileNode* node, cv::KeyPoint* value, const cv::KeyPoint* default_value, ResultVoid* ocvrs_return) {
		try {
			cv::read(*node, *value, *default_value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::read(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:744
	// ("cv::read", vec![(pred!(mut, ["node", "mat"], ["const cv::FileNode*", "cv::Mat*"]), _)]),
	void cv_read_const_FileNodeR_MatR(const cv::FileNode* node, cv::Mat* mat, ResultVoid* ocvrs_return) {
		try {
			cv::read(*node, *mat);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &, Mat &, const Mat &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:744
	// ("cv::read", vec![(pred!(mut, ["node", "mat", "default_mat"], ["const cv::FileNode*", "cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_read_const_FileNodeR_MatR_const_MatR(const cv::FileNode* node, cv::Mat* mat, const cv::Mat* default_mat, ResultVoid* ocvrs_return) {
		try {
			cv::read(*node, *mat, *default_mat);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::read(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:745
	// ("cv::read", vec![(pred!(mut, ["node", "mat"], ["const cv::FileNode*", "cv::SparseMat*"]), _)]),
	void cv_read_const_FileNodeR_SparseMatR(const cv::FileNode* node, cv::SparseMat* mat, ResultVoid* ocvrs_return) {
		try {
			cv::read(*node, *mat);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &, SparseMat &, const SparseMat &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:745
	// ("cv::read", vec![(pred!(mut, ["node", "mat", "default_mat"], ["const cv::FileNode*", "cv::SparseMat*", "const cv::SparseMat*"]), _)]),
	void cv_read_const_FileNodeR_SparseMatR_const_SparseMatR(const cv::FileNode* node, cv::SparseMat* mat, const cv::SparseMat* default_mat, ResultVoid* ocvrs_return) {
		try {
			cv::read(*node, *mat, *default_mat);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &, String &, const String &)(TraitClass, OutString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:742
	// ("cv::read", vec![(pred!(mut, ["node", "value", "default_value"], ["const cv::FileNode*", "cv::String*", "const cv::String*"]), _)]),
	void cv_read_const_FileNodeR_StringR_const_StringR(const cv::FileNode* node, void** value, const char* default_value, ResultVoid* ocvrs_return) {
		try {
			cv::String value_out;
			cv::read(*node, value_out, cv::String(default_value));
			*value = ocvrs_create_string(value_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &, double &, double)(TraitClass, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:741
	// ("cv::read", vec![(pred!(mut, ["node", "value", "default_value"], ["const cv::FileNode*", "double*", "double"]), _)]),
	void cv_read_const_FileNodeR_doubleR_double(const cv::FileNode* node, double* value, double default_value, ResultVoid* ocvrs_return) {
		try {
			cv::read(*node, *value, default_value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &, float &, float)(TraitClass, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:740
	// ("cv::read", vec![(pred!(mut, ["node", "value", "default_value"], ["const cv::FileNode*", "float*", "float"]), _)]),
	void cv_read_const_FileNodeR_floatR_float(const cv::FileNode* node, float* value, float default_value, ResultVoid* ocvrs_return) {
		try {
			cv::read(*node, *value, default_value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &, int &, int)(TraitClass, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:739
	// ("cv::read", vec![(pred!(mut, ["node", "value", "default_value"], ["const cv::FileNode*", "int*", "int"]), _)]),
	void cv_read_const_FileNodeR_intR_int(const cv::FileNode* node, int* value, int default_value, ResultVoid* ocvrs_return) {
		try {
			cv::read(*node, *value, default_value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &, std::string &, const std::string &)(TraitClass, OutString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:743
	// ("cv::read", vec![(pred!(mut, ["node", "value", "default_value"], ["const cv::FileNode*", "std::string*", "const std::string*"]), _)]),
	void cv_read_const_FileNodeR_stringR_const_stringR(const cv::FileNode* node, void** value, const char* default_value, ResultVoid* ocvrs_return) {
		try {
			std::string value_out;
			cv::read(*node, value_out, std::string(default_value));
			*value = ocvrs_create_string(value_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &, std::vector<DMatch> &)(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:748
	// ("cv::read", vec![(pred!(mut, ["node", "matches"], ["const cv::FileNode*", "std::vector<cv::DMatch>*"]), _)]),
	void cv_read_const_FileNodeR_vectorLDMatchGR(const cv::FileNode* node, std::vector<cv::DMatch>* matches, ResultVoid* ocvrs_return) {
		try {
			cv::read(*node, *matches);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &, std::vector<KeyPoint> &)(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:747
	// ("cv::read", vec![(pred!(mut, ["node", "keypoints"], ["const cv::FileNode*", "std::vector<cv::KeyPoint>*"]), _)]),
	void cv_read_const_FileNodeR_vectorLKeyPointGR(const cv::FileNode* node, std::vector<cv::KeyPoint>* keypoints, ResultVoid* ocvrs_return) {
		try {
			cv::read(*node, *keypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::reduce(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:899
	// ("cv::reduce", vec![(pred!(mut, ["src", "dst", "dim", "rtype"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_reduce_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int dim, int rtype, ResultVoid* ocvrs_return) {
		try {
			cv::reduce(*src, *dst, dim, rtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reduce(InputArray, OutputArray, int, int, int)(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:899
	// ("cv::reduce", vec![(pred!(mut, ["src", "dst", "dim", "rtype", "dtype"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
	void cv_reduce_const__InputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int dim, int rtype, int dtype, ResultVoid* ocvrs_return) {
		try {
			cv::reduce(*src, *dst, dim, rtype, dtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// repeat(const Mat &, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1120
	// ("cv::repeat", vec![(pred!(mut, ["src", "ny", "nx"], ["const cv::Mat*", "int", "int"]), _)]),
	void cv_repeat_const_MatR_int_int(const cv::Mat* src, int ny, int nx, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::repeat(*src, ny, nx);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// repeat(InputArray, int, int, OutputArray)(InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1111
	// ("cv::repeat", vec![(pred!(mut, ["src", "ny", "nx", "dst"], ["const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
	void cv_repeat_const__InputArrayR_int_int_const__OutputArrayR(const cv::_InputArray* src, int ny, int nx, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::repeat(*src, ny, nx, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// rotate(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1096
	// ("cv::rotate", vec![(pred!(mut, ["src", "dst", "rotateCode"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_rotate_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int rotateCode, ResultVoid* ocvrs_return) {
		try {
			cv::rotate(*src, *dst, rotateCode);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addSamplesDataSearchPath(const cv::String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:1204
	// ("cv::samples::addSamplesDataSearchPath", vec![(pred!(mut, ["path"], ["const cv::String*"]), _)]),
	void cv_samples_addSamplesDataSearchPath_const_StringR(const char* path, ResultVoid* ocvrs_return) {
		try {
			cv::samples::addSamplesDataSearchPath(cv::String(path));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addSamplesDataSearchSubDirectory(const cv::String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:1213
	// ("cv::samples::addSamplesDataSearchSubDirectory", vec![(pred!(mut, ["subdir"], ["const cv::String*"]), _)]),
	void cv_samples_addSamplesDataSearchSubDirectory_const_StringR(const char* subdir, ResultVoid* ocvrs_return) {
		try {
			cv::samples::addSamplesDataSearchSubDirectory(cv::String(subdir));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::samples::findFileOrKeep(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:1187
	// ("cv::samples::findFileOrKeep", vec![(pred!(mut, ["relative_path"], ["const cv::String*"]), _)]),
	void cv_samples_findFileOrKeep_const_StringR(const char* relative_path, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::samples::findFileOrKeep(cv::String(relative_path));
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findFileOrKeep(const cv::String &, bool)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:1187
	// ("cv::samples::findFileOrKeep", vec![(pred!(mut, ["relative_path", "silentMode"], ["const cv::String*", "bool"]), _)]),
	void cv_samples_findFileOrKeep_const_StringR_bool(const char* relative_path, bool silentMode, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::samples::findFileOrKeep(cv::String(relative_path), silentMode);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::samples::findFile(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:1185
	// ("cv::samples::findFile", vec![(pred!(mut, ["relative_path"], ["const cv::String*"]), _)]),
	void cv_samples_findFile_const_StringR(const char* relative_path, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::samples::findFile(cv::String(relative_path));
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findFile(const cv::String &, bool, bool)(InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:1185
	// ("cv::samples::findFile", vec![(pred!(mut, ["relative_path", "required", "silentMode"], ["const cv::String*", "bool", "bool"]), _)]),
	void cv_samples_findFile_const_StringR_bool_bool(const char* relative_path, bool required, bool silentMode, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::samples::findFile(cv::String(relative_path), required, silentMode);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// scaleAdd(InputArray, double, InputArray, OutputArray)(InputArray, Primitive, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:489
	// ("cv::scaleAdd", vec![(pred!(mut, ["src1", "alpha", "src2", "dst"], ["const cv::_InputArray*", "double", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_scaleAdd_const__InputArrayR_double_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, double alpha, const cv::_InputArray* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::scaleAdd(*src1, alpha, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBreakOnError(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:158
	// ("cv::setBreakOnError", vec![(pred!(mut, ["flag"], ["bool"]), _)]),
	void cv_setBreakOnError_bool(bool flag, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::setBreakOnError(flag);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::setIdentity(InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1790
	// ("cv::setIdentity", vec![(pred!(mut, ["mtx"], ["const cv::_InputOutputArray*"]), _)]),
	void cv_setIdentity_const__InputOutputArrayR(const cv::_InputOutputArray* mtx, ResultVoid* ocvrs_return) {
		try {
			cv::setIdentity(*mtx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setIdentity(InputOutputArray, const Scalar &)(InputOutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1790
	// ("cv::setIdentity", vec![(pred!(mut, ["mtx", "s"], ["const cv::_InputOutputArray*", "const cv::Scalar*"]), _)]),
	void cv_setIdentity_const__InputOutputArrayR_const_ScalarR(const cv::_InputOutputArray* mtx, const cv::Scalar* s, ResultVoid* ocvrs_return) {
		try {
			cv::setIdentity(*mtx, *s);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setLogLevel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:286
	// ("cv::setLogLevel", vec![(pred!(mut, ["level"], ["int"]), _)]),
	void cv_setLogLevel_int(int level, Result<int>* ocvrs_return) {
		try {
			int ret = cv::setLogLevel(level);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNumThreads(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:198
	// ("cv::setNumThreads", vec![(pred!(mut, ["nthreads"], ["int"]), _)]),
	void cv_setNumThreads_int(int nthreads, ResultVoid* ocvrs_return) {
		try {
			cv::setNumThreads(nthreads);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRNGSeed(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2298
	// ("cv::setRNGSeed", vec![(pred!(mut, ["seed"], ["int"]), _)]),
	void cv_setRNGSeed_int(int seed, ResultVoid* ocvrs_return) {
		try {
			cv::setRNGSeed(seed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseOpenVX(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ovx.hpp:25
	// ("cv::setUseOpenVX", vec![(pred!(mut, ["flag"], ["bool"]), _)]),
	void cv_setUseOpenVX_bool(bool flag, ResultVoid* ocvrs_return) {
		try {
			cv::setUseOpenVX(flag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseOptimized(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:558
	// ("cv::setUseOptimized", vec![(pred!(mut, ["onoff"], ["bool"]), _)]),
	void cv_setUseOptimized_bool(bool onoff, ResultVoid* ocvrs_return) {
		try {
			cv::setUseOptimized(onoff);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// solveCubic(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1916
	// ("cv::solveCubic", vec![(pred!(mut, ["coeffs", "roots"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_solveCubic_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* coeffs, const cv::_OutputArray* roots, Result<int>* ocvrs_return) {
		try {
			int ret = cv::solveCubic(*coeffs, *roots);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// solveLP(const Mat &, const Mat &, Mat &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/optim.hpp:301
	// ("cv::solveLP", vec![(pred!(mut, ["Func", "Constr", "z"], ["const cv::Mat*", "const cv::Mat*", "cv::Mat*"]), _)]),
	void cv_solveLP_const_MatR_const_MatR_MatR(const cv::Mat* Func, const cv::Mat* Constr, cv::Mat* z, Result<int>* ocvrs_return) {
		try {
			int ret = cv::solveLP(*Func, *Constr, *z);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// solveLP(const Mat &, const Mat &, Mat &, double)(TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/optim.hpp:298
	// ("cv::solveLP", vec![(pred!(mut, ["Func", "Constr", "z", "constr_eps"], ["const cv::Mat*", "const cv::Mat*", "cv::Mat*", "double"]), _)]),
	void cv_solveLP_const_MatR_const_MatR_MatR_double(const cv::Mat* Func, const cv::Mat* Constr, cv::Mat* z, double constr_eps, Result<int>* ocvrs_return) {
		try {
			int ret = cv::solveLP(*Func, *Constr, *z, constr_eps);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::solvePoly(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1926
	// ("cv::solvePoly", vec![(pred!(mut, ["coeffs", "roots"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_solvePoly_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* coeffs, const cv::_OutputArray* roots, Result<double>* ocvrs_return) {
		try {
			double ret = cv::solvePoly(*coeffs, *roots);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// solvePoly(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1926
	// ("cv::solvePoly", vec![(pred!(mut, ["coeffs", "roots", "maxIters"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_solvePoly_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* coeffs, const cv::_OutputArray* roots, int maxIters, Result<double>* ocvrs_return) {
		try {
			double ret = cv::solvePoly(*coeffs, *roots, maxIters);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::solve(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1865
	// ("cv::solve", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_solve_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::solve(*src1, *src2, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// solve(InputArray, InputArray, OutputArray, int)(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1865
	// ("cv::solve", vec![(pred!(mut, ["src1", "src2", "dst", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_solve_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::solve(*src1, *src2, *dst, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// sortIdx(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1901
	// ("cv::sortIdx", vec![(pred!(mut, ["src", "dst", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_sortIdx_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int flags, ResultVoid* ocvrs_return) {
		try {
			cv::sortIdx(*src, *dst, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// sort(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1881
	// ("cv::sort", vec![(pred!(mut, ["src", "dst", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_sort_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int flags, ResultVoid* ocvrs_return) {
		try {
			cv::sort(*src, *dst, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// split(const Mat &, Mat *)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:945
	// ("cv::split", vec![(pred!(mut, ["src", "mvbegin"], ["const cv::Mat*", "cv::Mat*"]), _)]),
	void cv_split_const_MatR_MatX(const cv::Mat* src, cv::Mat* mvbegin, ResultVoid* ocvrs_return) {
		try {
			cv::split(*src, mvbegin);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// split(InputArray, OutputArrayOfArrays)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:951
	// ("cv::split", vec![(pred!(mut, ["m", "mv"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_split_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* m, const cv::_OutputArray* mv, ResultVoid* ocvrs_return) {
		try {
			cv::split(*m, *mv);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// sqrt(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1475
	// ("cv::sqrt", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_sqrt_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::sqrt(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::subtract(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:416
	// ("cv::subtract", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_subtract_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::subtract(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// subtract(InputArray, InputArray, OutputArray, InputArray, int)(InputArray, InputArray, OutputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:416
	// ("cv::subtract", vec![(pred!(mut, ["src1", "src2", "dst", "mask", "dtype"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "int"]), _)]),
	void cv_subtract_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_int(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, const cv::_InputArray* mask, int dtype, ResultVoid* ocvrs_return) {
		try {
			cv::subtract(*src1, *src2, *dst, *mask, dtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// sum(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:583
	// ("cv::sum", vec![(pred!(mut, ["src"], ["const cv::_InputArray*"]), _)]),
	void cv_sum_const__InputArrayR(const cv::_InputArray* src, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = cv::sum(*src);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// swap(Mat &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:256
	// ("cv::swap", vec![(pred!(mut, ["a", "b"], ["cv::Mat*", "cv::Mat*"]), _)]),
	void cv_swap_MatR_MatR(cv::Mat* a, cv::Mat* b, ResultVoid* ocvrs_return) {
		try {
			cv::swap(*a, *b);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// swap(UMat &, UMat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:258
	// ("cv::swap", vec![(pred!(mut, ["a", "b"], ["cv::UMat*", "cv::UMat*"]), _)]),
	void cv_swap_UMatR_UMatR(cv::UMat* a, cv::UMat* b, ResultVoid* ocvrs_return) {
		try {
			cv::swap(*a, *b);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::tempfile() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:177
	// ("cv::tempfile", vec![(pred!(mut, [], []), _)]),
	void cv_tempfile(Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::tempfile();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// tempfile(const char *)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:177
	// ("cv::tempfile", vec![(pred!(mut, ["suffix"], ["const char*"]), _)]),
	void cv_tempfile_const_charX(const char* suffix, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::tempfile(suffix);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// theRNG()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2290
	// ("cv::theRNG", vec![(pred!(mut, [], []), _)]),
	void cv_theRNG(Result<cv::RNG*>* ocvrs_return) {
		try {
			cv::RNG ret = cv::theRNG();
			Ok(new cv::RNG(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// trace(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1814
	// ("cv::trace", vec![(pred!(mut, ["mtx"], ["const cv::_InputArray*"]), _)]),
	void cv_trace_const__InputArrayR(const cv::_InputArray* mtx, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = cv::trace(*mtx);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// transform(InputArray, OutputArray, InputArray)(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1730
	// ("cv::transform", vec![(pred!(mut, ["src", "dst", "m"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_transform_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* m, ResultVoid* ocvrs_return) {
		try {
			cv::transform(*src, *dst, *m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// transpose(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1704
	// ("cv::transpose", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_transpose_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::transpose(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// typeToString(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/check.hpp:16
	// ("cv::typeToString", vec![(pred!(mut, ["type"], ["int"]), _)]),
	void cv_typeToString_int(int type, Result<void*>* ocvrs_return) {
		try {
			const cv::String ret = cv::typeToString(type);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// useOpenVX()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ovx.hpp:22
	// ("cv::useOpenVX", vec![(pred!(mut, [], []), _)]),
	void cv_useOpenVX(Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::useOpenVX();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// useOptimized()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:564
	// ("cv::useOptimized", vec![(pred!(mut, [], []), _)]),
	void cv_useOptimized(Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::useOptimized();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dumpBool(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:27
	// ("cv::utils::dumpBool", vec![(pred!(mut, ["argument"], ["bool"]), _)]),
	void cv_utils_dumpBool_bool(bool argument, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpBool(argument);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dumpCString(const char *)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:59
	// ("cv::utils::dumpCString", vec![(pred!(mut, ["argument"], ["const char*"]), _)]),
	void cv_utils_dumpCString_const_charX(const char* argument, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpCString(argument);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dumpDouble(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:53
	// ("cv::utils::dumpDouble", vec![(pred!(mut, ["argument"], ["double"]), _)]),
	void cv_utils_dumpDouble_double(double argument, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpDouble(argument);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dumpFloat(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:47
	// ("cv::utils::dumpFloat", vec![(pred!(mut, ["argument"], ["float"]), _)]),
	void cv_utils_dumpFloat_float(float argument, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpFloat(argument);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dumpInputArrayOfArrays(InputArrayOfArrays)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:20
	// ("cv::utils::dumpInputArrayOfArrays", vec![(pred!(mut, ["argument"], ["const cv::_InputArray*"]), _)]),
	void cv_utils_dumpInputArrayOfArrays_const__InputArrayR(const cv::_InputArray* argument, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpInputArrayOfArrays(*argument);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dumpInputArray(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:18
	// ("cv::utils::dumpInputArray", vec![(pred!(mut, ["argument"], ["const cv::_InputArray*"]), _)]),
	void cv_utils_dumpInputArray_const__InputArrayR(const cv::_InputArray* argument, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpInputArray(*argument);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dumpInputOutputArrayOfArrays(InputOutputArrayOfArrays)(InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:24
	// ("cv::utils::dumpInputOutputArrayOfArrays", vec![(pred!(mut, ["argument"], ["const cv::_InputOutputArray*"]), _)]),
	void cv_utils_dumpInputOutputArrayOfArrays_const__InputOutputArrayR(const cv::_InputOutputArray* argument, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpInputOutputArrayOfArrays(*argument);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dumpInputOutputArray(InputOutputArray)(InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:22
	// ("cv::utils::dumpInputOutputArray", vec![(pred!(mut, ["argument"], ["const cv::_InputOutputArray*"]), _)]),
	void cv_utils_dumpInputOutputArray_const__InputOutputArrayR(const cv::_InputOutputArray* argument, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpInputOutputArray(*argument);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dumpInt(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:33
	// ("cv::utils::dumpInt", vec![(pred!(mut, ["argument"], ["int"]), _)]),
	void cv_utils_dumpInt_int(int argument, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpInt(argument);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dumpRange(const Range &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:122
	// ("cv::utils::dumpRange", vec![(pred!(mut, ["argument"], ["const cv::Range*"]), _)]),
	void cv_utils_dumpRange_const_RangeR(const cv::Range* argument, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpRange(*argument);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dumpRect(const Rect &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:85
	// ("cv::utils::dumpRect", vec![(pred!(mut, ["argument"], ["const cv::Rect*"]), _)]),
	void cv_utils_dumpRect_const_RectR(const cv::Rect* argument, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpRect(*argument);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dumpRotatedRect(const RotatedRect &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:99
	// ("cv::utils::dumpRotatedRect", vec![(pred!(mut, ["argument"], ["const cv::RotatedRect*"]), _)]),
	void cv_utils_dumpRotatedRect_const_RotatedRectR(const cv::RotatedRect* argument, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpRotatedRect(*argument);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dumpSizeT(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:39
	// ("cv::utils::dumpSizeT", vec![(pred!(mut, ["argument"], ["size_t"]), _)]),
	void cv_utils_dumpSizeT_size_t(size_t argument, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpSizeT(argument);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dumpString(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:65
	// ("cv::utils::dumpString", vec![(pred!(mut, ["argument"], ["const cv::String*"]), _)]),
	void cv_utils_dumpString_const_StringR(const char* argument, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpString(cv::String(argument));
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dumpTermCriteria(const TermCriteria &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:92
	// ("cv::utils::dumpTermCriteria", vec![(pred!(mut, ["argument"], ["const cv::TermCriteria*"]), _)]),
	void cv_utils_dumpTermCriteria_const_TermCriteriaR(const cv::TermCriteria* argument, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpTermCriteria(*argument);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dumpVectorOfDouble(const std::vector<double> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:142
	// ("cv::utils::dumpVectorOfDouble", vec![(pred!(mut, ["vec"], ["const std::vector<double>*"]), _)]),
	void cv_utils_dumpVectorOfDouble_const_vectorLdoubleGR(const std::vector<double>* vec, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpVectorOfDouble(*vec);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dumpVectorOfInt(const std::vector<int> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:140
	// ("cv::utils::dumpVectorOfInt", vec![(pred!(mut, ["vec"], ["const std::vector<int>*"]), _)]),
	void cv_utils_dumpVectorOfInt_const_vectorLintGR(const std::vector<int>* vec, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpVectorOfInt(*vec);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dumpVectorOfRect(const std::vector<Rect> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:144
	// ("cv::utils::dumpVectorOfRect", vec![(pred!(mut, ["vec"], ["const std::vector<cv::Rect>*"]), _)]),
	void cv_utils_dumpVectorOfRect_const_vectorLRectGR(const std::vector<cv::Rect>* vec, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpVectorOfRect(*vec);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// generateVectorOfInt(size_t, std::vector<int> &)(Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:160
	// ("cv::utils::generateVectorOfInt", vec![(pred!(mut, ["len", "vec"], ["size_t", "std::vector<int>*"]), _)]),
	void cv_utils_generateVectorOfInt_size_t_vectorLintGR(size_t len, std::vector<int>* vec, ResultVoid* ocvrs_return) {
		try {
			cv::utils::generateVectorOfInt(len, *vec);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// generateVectorOfMat(size_t, int, int, int, std::vector<Mat> &)(Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:173
	// ("cv::utils::generateVectorOfMat", vec![(pred!(mut, ["len", "rows", "cols", "dtype", "vec"], ["size_t", "int", "int", "int", "std::vector<cv::Mat>*"]), _)]),
	void cv_utils_generateVectorOfMat_size_t_int_int_int_vectorLMatGR(size_t len, int rows, int cols, int dtype, std::vector<cv::Mat>* vec, ResultVoid* ocvrs_return) {
		try {
			cv::utils::generateVectorOfMat(len, rows, cols, dtype, *vec);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// generateVectorOfRect(size_t, std::vector<Rect> &)(Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:147
	// ("cv::utils::generateVectorOfRect", vec![(pred!(mut, ["len", "vec"], ["size_t", "std::vector<cv::Rect>*"]), _)]),
	void cv_utils_generateVectorOfRect_size_t_vectorLRectGR(size_t len, std::vector<cv::Rect>* vec, ResultVoid* ocvrs_return) {
		try {
			cv::utils::generateVectorOfRect(len, *vec);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getThreadID()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:1220
	// ("cv::utils::getThreadID", vec![(pred!(mut, [], []), _)]),
	void cv_utils_getThreadID(Result<int>* ocvrs_return) {
		try {
			int ret = cv::utils::getThreadID();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLogLevel()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/logger.hpp:40
	// ("cv::utils::logging::getLogLevel", vec![(pred!(mut, [], []), _)]),
	void cv_utils_logging_getLogLevel(Result<cv::utils::logging::LogLevel>* ocvrs_return) {
		try {
			cv::utils::logging::LogLevel ret = cv::utils::logging::getLogLevel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writeLogMessage(LogLevel, const char *)(Enum, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/logger.hpp:44
	// ("cv::utils::logging::internal::writeLogMessage", vec![(pred!(mut, ["logLevel", "message"], ["cv::utils::logging::LogLevel", "const char*"]), _)]),
	void cv_utils_logging_internal_writeLogMessage_LogLevel_const_charX(cv::utils::logging::LogLevel logLevel, const char* message, ResultVoid* ocvrs_return) {
		try {
			cv::utils::logging::internal::writeLogMessage(logLevel, message);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setLogLevel(LogLevel)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/logger.hpp:38
	// ("cv::utils::logging::setLogLevel", vec![(pred!(mut, ["logLevel"], ["cv::utils::logging::LogLevel"]), _)]),
	void cv_utils_logging_setLogLevel_LogLevel(cv::utils::logging::LogLevel logLevel, Result<cv::utils::logging::LogLevel>* ocvrs_return) {
		try {
			cv::utils::logging::LogLevel ret = cv::utils::logging::setLogLevel(logLevel);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// testEchoBooleanFunction(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:228
	// ("cv::utils::nested::testEchoBooleanFunction", vec![(pred!(mut, ["flag"], ["bool"]), _)]),
	void cv_utils_nested_testEchoBooleanFunction_bool(bool flag, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::utils::nested::testEchoBooleanFunction(flag);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// testAsyncArray(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:194
	// ("cv::utils::testAsyncArray", vec![(pred!(mut, ["argument"], ["const cv::_InputArray*"]), _)]),
	void cv_utils_testAsyncArray_const__InputArrayR(const cv::_InputArray* argument, Result<cv::AsyncArray*>* ocvrs_return) {
		try {
			cv::AsyncArray ret = cv::utils::testAsyncArray(*argument);
			Ok(new cv::AsyncArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// testAsyncException()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:202
	// ("cv::utils::testAsyncException", vec![(pred!(mut, [], []), _)]),
	void cv_utils_testAsyncException(Result<cv::AsyncArray*>* ocvrs_return) {
		try {
			cv::AsyncArray ret = cv::utils::testAsyncException();
			Ok(new cv::AsyncArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// testOverloadResolution(const Rect &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:78
	// ("cv::utils::testOverloadResolution", vec![(pred!(mut, ["rect"], ["const cv::Rect*"]), _)]),
	void cv_utils_testOverloadResolution_const_RectR(const cv::Rect* rect, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::testOverloadResolution(*rect);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::utils::testOverloadResolution(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:71
	// ("cv::utils::testOverloadResolution", vec![(pred!(mut, ["value"], ["int"]), _)]),
	void cv_utils_testOverloadResolution_int(int value, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::testOverloadResolution(value);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// testOverloadResolution(int, const Point &)(Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:71
	// ("cv::utils::testOverloadResolution", vec![(pred!(mut, ["value", "point"], ["int", "const cv::Point*"]), _)]),
	void cv_utils_testOverloadResolution_int_const_PointR(int value, const cv::Point* point, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::testOverloadResolution(value, *point);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// testRaiseGeneralException()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:188
	// ("cv::utils::testRaiseGeneralException", vec![(pred!(mut, [], []), _)]),
	void cv_utils_testRaiseGeneralException(ResultVoid* ocvrs_return) {
		try {
			cv::utils::testRaiseGeneralException();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::utils::testReservedKeywordConversion(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:135
	// ("cv::utils::testReservedKeywordConversion", vec![(pred!(mut, ["positional_argument"], ["int"]), _)]),
	void cv_utils_testReservedKeywordConversion_int(int positional_argument, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::testReservedKeywordConversion(positional_argument);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// testReservedKeywordConversion(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:135
	// ("cv::utils::testReservedKeywordConversion", vec![(pred!(mut, ["positional_argument", "lambda", "from"], ["int", "int", "int"]), _)]),
	void cv_utils_testReservedKeywordConversion_int_int_int(int positional_argument, int lambda, int from, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::testReservedKeywordConversion(positional_argument, lambda, from);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// testRotatedRectVector(float, float, float, float, float)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:113
	// ("cv::utils::testRotatedRectVector", vec![(pred!(mut, ["x", "y", "w", "h", "angle"], ["float", "float", "float", "float", "float"]), _)]),
	void cv_utils_testRotatedRectVector_float_float_float_float_float(float x, float y, float w, float h, float angle, Result<std::vector<cv::RotatedRect>*>* ocvrs_return) {
		try {
			std::vector<cv::RotatedRect> ret = cv::utils::testRotatedRectVector(x, y, w, h, angle);
			Ok(new std::vector<cv::RotatedRect>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// testRotatedRect(float, float, float, float, float)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:107
	// ("cv::utils::testRotatedRect", vec![(pred!(mut, ["x", "y", "w", "h", "angle"], ["float", "float", "float", "float", "float"]), _)]),
	void cv_utils_testRotatedRect_float_float_float_float_float(float x, float y, float w, float h, float angle, Result<cv::RotatedRect>* ocvrs_return) {
		try {
			cv::RotatedRect ret = cv::utils::testRotatedRect(x, y, w, h, angle);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertFromVASurface(VADisplay, VASurfaceID, Size, OutputArray)(Indirect, Primitive, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/va_intel.hpp:72
	// ("cv::va_intel::convertFromVASurface", vec![(pred!(mut, ["display", "surface", "size", "dst"], ["VADisplay", "VASurfaceID", "cv::Size", "const cv::_OutputArray*"]), _)]),
	void cv_va_intel_convertFromVASurface_VADisplay_VASurfaceID_Size_const__OutputArrayR(VADisplay display, VASurfaceID surface, cv::Size* size, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::va_intel::convertFromVASurface(display, surface, *size, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertToVASurface(VADisplay, InputArray, VASurfaceID, Size)(Indirect, InputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/va_intel.hpp:64
	// ("cv::va_intel::convertToVASurface", vec![(pred!(mut, ["display", "src", "surface", "size"], ["VADisplay", "const cv::_InputArray*", "VASurfaceID", "cv::Size"]), _)]),
	void cv_va_intel_convertToVASurface_VADisplay_const__InputArrayR_VASurfaceID_Size(VADisplay display, const cv::_InputArray* src, VASurfaceID surface, cv::Size* size, ResultVoid* ocvrs_return) {
		try {
			cv::va_intel::convertToVASurface(display, *src, surface, *size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::va_intel::ocl::initializeContextFromVA(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/va_intel.hpp:54
	// ("cv::va_intel::ocl::initializeContextFromVA", vec![(pred!(mut, ["display"], ["VADisplay"]), _)]),
	void cv_va_intel_ocl_initializeContextFromVA_VADisplay(VADisplay display, Result<cv::ocl::Context*>* ocvrs_return) {
		try {
			cv::ocl::Context ret = cv::va_intel::ocl::initializeContextFromVA(display);
			Ok(new cv::ocl::Context(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initializeContextFromVA(VADisplay, bool)(Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/va_intel.hpp:54
	// ("cv::va_intel::ocl::initializeContextFromVA", vec![(pred!(mut, ["display", "tryInterop"], ["VADisplay", "bool"]), _)]),
	void cv_va_intel_ocl_initializeContextFromVA_VADisplay_bool(VADisplay display, bool tryInterop, Result<cv::ocl::Context*>* ocvrs_return) {
		try {
			cv::ocl::Context ret = cv::va_intel::ocl::initializeContextFromVA(display, tryInterop);
			Ok(new cv::ocl::Context(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// vconcat(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1229
	// ("cv::vconcat", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_vconcat_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::vconcat(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// vconcat(InputArrayOfArrays, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:1247
	// ("cv::vconcat", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_vconcat_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::vconcat(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writeScalar(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:732
	// ("cv::writeScalar", vec![(pred!(mut, ["fs", "value"], ["cv::FileStorage*", "const cv::String*"]), _)]),
	void cv_writeScalar_FileStorageR_const_StringR(cv::FileStorage* fs, const char* value, ResultVoid* ocvrs_return) {
		try {
			cv::writeScalar(*fs, cv::String(value));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writeScalar(FileStorage &, double)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:731
	// ("cv::writeScalar", vec![(pred!(mut, ["fs", "value"], ["cv::FileStorage*", "double"]), _)]),
	void cv_writeScalar_FileStorageR_double(cv::FileStorage* fs, double value, ResultVoid* ocvrs_return) {
		try {
			cv::writeScalar(*fs, value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writeScalar(FileStorage &, float)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:730
	// ("cv::writeScalar", vec![(pred!(mut, ["fs", "value"], ["cv::FileStorage*", "float"]), _)]),
	void cv_writeScalar_FileStorageR_float(cv::FileStorage* fs, float value, ResultVoid* ocvrs_return) {
		try {
			cv::writeScalar(*fs, value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writeScalar(FileStorage &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:729
	// ("cv::writeScalar", vec![(pred!(mut, ["fs", "value"], ["cv::FileStorage*", "int"]), _)]),
	void cv_writeScalar_FileStorageR_int(cv::FileStorage* fs, int value, ResultVoid* ocvrs_return) {
		try {
			cv::writeScalar(*fs, value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &, const String &, const Mat &)(TraitClass, InString, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:722
	// ("cv::write", vec![(pred!(mut, ["fs", "name", "value"], ["cv::FileStorage*", "const cv::String*", "const cv::Mat*"]), _)]),
	void cv_write_FileStorageR_const_StringR_const_MatR(cv::FileStorage* fs, const char* name, const cv::Mat* value, ResultVoid* ocvrs_return) {
		try {
			cv::write(*fs, cv::String(name), *value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &, const String &, const SparseMat &)(TraitClass, InString, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:723
	// ("cv::write", vec![(pred!(mut, ["fs", "name", "value"], ["cv::FileStorage*", "const cv::String*", "const cv::SparseMat*"]), _)]),
	void cv_write_FileStorageR_const_StringR_const_SparseMatR(cv::FileStorage* fs, const char* name, const cv::SparseMat* value, ResultVoid* ocvrs_return) {
		try {
			cv::write(*fs, cv::String(name), *value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &, const String &, const String &)(TraitClass, InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:721
	// ("cv::write", vec![(pred!(mut, ["fs", "name", "value"], ["cv::FileStorage*", "const cv::String*", "const cv::String*"]), _)]),
	void cv_write_FileStorageR_const_StringR_const_StringR(cv::FileStorage* fs, const char* name, const char* value, ResultVoid* ocvrs_return) {
		try {
			cv::write(*fs, cv::String(name), cv::String(value));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &, const String &, const std::vector<DMatch> &)(TraitClass, InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:726
	// ("cv::write", vec![(pred!(mut, ["fs", "name", "value"], ["cv::FileStorage*", "const cv::String*", "const std::vector<cv::DMatch>*"]), _)]),
	void cv_write_FileStorageR_const_StringR_const_vectorLDMatchGR(cv::FileStorage* fs, const char* name, const std::vector<cv::DMatch>* value, ResultVoid* ocvrs_return) {
		try {
			cv::write(*fs, cv::String(name), *value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &, const String &, const std::vector<KeyPoint> &)(TraitClass, InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:725
	// ("cv::write", vec![(pred!(mut, ["fs", "name", "value"], ["cv::FileStorage*", "const cv::String*", "const std::vector<cv::KeyPoint>*"]), _)]),
	void cv_write_FileStorageR_const_StringR_const_vectorLKeyPointGR(cv::FileStorage* fs, const char* name, const std::vector<cv::KeyPoint>* value, ResultVoid* ocvrs_return) {
		try {
			cv::write(*fs, cv::String(name), *value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &, const String &, double)(TraitClass, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:720
	// ("cv::write", vec![(pred!(mut, ["fs", "name", "value"], ["cv::FileStorage*", "const cv::String*", "double"]), _)]),
	void cv_write_FileStorageR_const_StringR_double(cv::FileStorage* fs, const char* name, double value, ResultVoid* ocvrs_return) {
		try {
			cv::write(*fs, cv::String(name), value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &, const String &, float)(TraitClass, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:719
	// ("cv::write", vec![(pred!(mut, ["fs", "name", "value"], ["cv::FileStorage*", "const cv::String*", "float"]), _)]),
	void cv_write_FileStorageR_const_StringR_float(cv::FileStorage* fs, const char* name, float value, ResultVoid* ocvrs_return) {
		try {
			cv::write(*fs, cv::String(name), value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &, const String &, int)(TraitClass, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:718
	// ("cv::write", vec![(pred!(mut, ["fs", "name", "value"], ["cv::FileStorage*", "const cv::String*", "int"]), _)]),
	void cv_write_FileStorageR_const_StringR_int(cv::FileStorage* fs, const char* name, int value, ResultVoid* ocvrs_return) {
		try {
			cv::write(*fs, cv::String(name), value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Algorithm()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:3089
	// ("cv::Algorithm::Algorithm", vec![(pred!(mut, [], []), _)]),
	void cv_Algorithm_Algorithm(Result<cv::Algorithm*>* ocvrs_return) {
		try {
			cv::Algorithm* ret = new cv::Algorithm();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clear()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:3094
	// ("cv::Algorithm::clear", vec![(pred!(mut, [], []), _)]),
	void cv_Algorithm_clear(cv::Algorithm* instance, ResultVoid* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:3098
	// ("cv::Algorithm::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_Algorithm_write_const_FileStorageR(const cv::Algorithm* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:3103
	// ("cv::Algorithm::write", vec![(pred!(const, ["fs", "name"], ["cv::FileStorage*", "const cv::String*"]), _)]),
	void cv_Algorithm_write_const_FileStorageR_const_StringR(const cv::Algorithm* instance, cv::FileStorage* fs, const char* name, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs, cv::String(name));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(const Ptr<FileStorage> &, const String &)(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:3106
	// ("cv::Algorithm::write", vec![(pred!(const, ["fs", "name"], ["const cv::Ptr<cv::FileStorage>*", "const cv::String*"]), _)]),
	void cv_Algorithm_write_const_const_PtrLFileStorageGR_const_StringR(const cv::Algorithm* instance, const cv::Ptr<cv::FileStorage>* fs, const char* name, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs, cv::String(name));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Algorithm::write(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:3106
	// ("cv::Algorithm::write", vec![(pred!(const, ["fs"], ["const cv::Ptr<cv::FileStorage>*"]), _)]),
	void cv_Algorithm_write_const_const_PtrLFileStorageGR(const cv::Algorithm* instance, const cv::Ptr<cv::FileStorage>* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:3111
	// ("cv::Algorithm::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_Algorithm_read_const_FileNodeR(cv::Algorithm* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:3115
	// ("cv::Algorithm::empty", vec![(pred!(const, [], []), _)]),
	void cv_Algorithm_empty_const(const cv::Algorithm* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// save(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:3179
	// ("cv::Algorithm::save", vec![(pred!(const, ["filename"], ["const cv::String*"]), _)]),
	void cv_Algorithm_save_const_const_StringR(const cv::Algorithm* instance, const char* filename, ResultVoid* ocvrs_return) {
		try {
			instance->save(cv::String(filename));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:3183
	// ("cv::Algorithm::getDefaultName", vec![(pred!(const, [], []), _)]),
	void cv_Algorithm_getDefaultName_const(const cv::Algorithm* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Algorithm::to_ConjGradSolver() generated
	// ("cv::Algorithm::to_ConjGradSolver", vec![(pred!(mut, [], []), _)]),
	cv::ConjGradSolver* cv_Algorithm_to_ConjGradSolver(cv::Algorithm* instance) {
			return dynamic_cast<cv::ConjGradSolver*>(instance);
	}

	// cv::Algorithm::to_DownhillSolver() generated
	// ("cv::Algorithm::to_DownhillSolver", vec![(pred!(mut, [], []), _)]),
	cv::DownhillSolver* cv_Algorithm_to_DownhillSolver(cv::Algorithm* instance) {
			return dynamic_cast<cv::DownhillSolver*>(instance);
	}

	// cv::Algorithm::to_MinProblemSolver() generated
	// ("cv::Algorithm::to_MinProblemSolver", vec![(pred!(mut, [], []), _)]),
	cv::MinProblemSolver* cv_Algorithm_to_MinProblemSolver(cv::Algorithm* instance) {
			return dynamic_cast<cv::MinProblemSolver*>(instance);
	}

	// cv::Algorithm::delete() generated
	// ("cv::Algorithm::delete", vec![(pred!(mut, [], []), _)]),
	void cv_Algorithm_delete(cv::Algorithm* instance) {
			delete instance;
	}

	// AsyncArray()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/async.hpp:35
	// ("cv::AsyncArray::AsyncArray", vec![(pred!(mut, [], []), _)]),
	cv::AsyncArray* cv_AsyncArray_AsyncArray() {
			cv::AsyncArray* ret = new cv::AsyncArray();
			return ret;
	}

	// AsyncArray(const AsyncArray &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/async.hpp:36
	// ("cv::AsyncArray::AsyncArray", vec![(pred!(mut, ["o"], ["const cv::AsyncArray*"]), _)]),
	cv::AsyncArray* cv_AsyncArray_AsyncArray_const_AsyncArrayR(const cv::AsyncArray* o) {
			cv::AsyncArray* ret = new cv::AsyncArray(*o);
			return ret;
	}

	// operator=(const AsyncArray &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/async.hpp:37
	// ("cv::AsyncArray::operator=", vec![(pred!(mut, ["o"], ["const cv::AsyncArray*"]), _)]),
	void cv_AsyncArray_operatorST_const_AsyncArrayR(cv::AsyncArray* instance, const cv::AsyncArray* o) {
			instance->operator=(*o);
	}

	// release()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/async.hpp:38
	// ("cv::AsyncArray::release", vec![(pred!(mut, [], []), _)]),
	void cv_AsyncArray_release(cv::AsyncArray* instance) {
			instance->release();
	}

	// get(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/async.hpp:50
	// ("cv::AsyncArray::get", vec![(pred!(const, ["dst"], ["const cv::_OutputArray*"]), _)]),
	void cv_AsyncArray_get_const_const__OutputArrayR(const cv::AsyncArray* instance, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->get(*dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// get(OutputArray, int64)(OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/async.hpp:60
	// ("cv::AsyncArray::get", vec![(pred!(const, ["dst", "timeoutNs"], ["const cv::_OutputArray*", "int64_t"]), _)]),
	void cv_AsyncArray_get_const_const__OutputArrayR_int64_t(const cv::AsyncArray* instance, const cv::_OutputArray* dst, int64_t timeoutNs, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->get(*dst, timeoutNs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// get(OutputArray, double)(OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/async.hpp:63
	// ("cv::AsyncArray::get", vec![(pred!(const, ["dst", "timeoutNs"], ["const cv::_OutputArray*", "double"]), _)]),
	void cv_AsyncArray_get_const_const__OutputArrayR_double(const cv::AsyncArray* instance, const cv::_OutputArray* dst, double timeoutNs, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->get(*dst, timeoutNs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// wait_for(int64)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/async.hpp:65
	// ("cv::AsyncArray::wait_for", vec![(pred!(const, ["timeoutNs"], ["int64_t"]), _)]),
	void cv_AsyncArray_wait_for_const_int64_t(const cv::AsyncArray* instance, int64_t timeoutNs, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->wait_for(timeoutNs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// wait_for(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/async.hpp:68
	// ("cv::AsyncArray::wait_for", vec![(pred!(const, ["timeoutNs"], ["double"]), _)]),
	void cv_AsyncArray_wait_for_const_double(const cv::AsyncArray* instance, double timeoutNs, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->wait_for(timeoutNs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// valid()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/async.hpp:70
	// ("cv::AsyncArray::valid", vec![(pred!(const, [], []), _)]),
	bool cv_AsyncArray_valid_const(const cv::AsyncArray* instance) {
			bool ret = instance->valid();
			return ret;
	}

	// AsyncArray(AsyncArray &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/async.hpp:73
	// ("cv::AsyncArray::AsyncArray", vec![(pred!(mut, ["o"], ["cv::AsyncArray*"]), _)]),
	void cv_AsyncArray_AsyncArray_AsyncArrayRR(cv::AsyncArray* o, Result<cv::AsyncArray*>* ocvrs_return) {
		try {
			cv::AsyncArray* ret = new cv::AsyncArray(std::move(*o));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(AsyncArray &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/async.hpp:74
	// ("cv::AsyncArray::operator=", vec![(pred!(mut, ["o"], ["cv::AsyncArray*"]), _)]),
	void cv_AsyncArray_operatorST_AsyncArrayRR(cv::AsyncArray* instance, cv::AsyncArray* o) {
			instance->operator=(std::move(*o));
	}

	// cv::AsyncArray::delete() generated
	// ("cv::AsyncArray::delete", vec![(pred!(mut, [], []), _)]),
	void cv_AsyncArray_delete(cv::AsyncArray* instance) {
			delete instance;
	}

	// AsyncPromise()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/detail/async_promise.hpp:26
	// ("cv::AsyncPromise::AsyncPromise", vec![(pred!(mut, [], []), _)]),
	cv::AsyncPromise* cv_AsyncPromise_AsyncPromise() {
			cv::AsyncPromise* ret = new cv::AsyncPromise();
			return ret;
	}

	// AsyncPromise(const AsyncPromise &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/detail/async_promise.hpp:27
	// ("cv::AsyncPromise::AsyncPromise", vec![(pred!(mut, ["o"], ["const cv::AsyncPromise*"]), _)]),
	cv::AsyncPromise* cv_AsyncPromise_AsyncPromise_const_AsyncPromiseR(const cv::AsyncPromise* o) {
			cv::AsyncPromise* ret = new cv::AsyncPromise(*o);
			return ret;
	}

	// operator=(const AsyncPromise &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/detail/async_promise.hpp:28
	// ("cv::AsyncPromise::operator=", vec![(pred!(mut, ["o"], ["const cv::AsyncPromise*"]), _)]),
	void cv_AsyncPromise_operatorST_const_AsyncPromiseR(cv::AsyncPromise* instance, const cv::AsyncPromise* o) {
			instance->operator=(*o);
	}

	// release()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/detail/async_promise.hpp:29
	// ("cv::AsyncPromise::release", vec![(pred!(mut, [], []), _)]),
	void cv_AsyncPromise_release(cv::AsyncPromise* instance) {
			instance->release();
	}

	// getArrayResult()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/detail/async_promise.hpp:34
	// ("cv::AsyncPromise::getArrayResult", vec![(pred!(mut, [], []), _)]),
	void cv_AsyncPromise_getArrayResult(cv::AsyncPromise* instance, Result<cv::AsyncArray*>* ocvrs_return) {
		try {
			cv::AsyncArray ret = instance->getArrayResult();
			Ok(new cv::AsyncArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setValue(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/detail/async_promise.hpp:39
	// ("cv::AsyncPromise::setValue", vec![(pred!(mut, ["value"], ["const cv::_InputArray*"]), _)]),
	void cv_AsyncPromise_setValue_const__InputArrayR(cv::AsyncPromise* instance, const cv::_InputArray* value, ResultVoid* ocvrs_return) {
		try {
			instance->setValue(*value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setException(const cv::Exception &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/detail/async_promise.hpp:53
	// ("cv::AsyncPromise::setException", vec![(pred!(mut, ["exception"], ["const cv::Exception*"]), _)]),
	void cv_AsyncPromise_setException_const_ExceptionR(cv::AsyncPromise* instance, const cv::Exception* exception, ResultVoid* ocvrs_return) {
		try {
			instance->setException(*exception);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// AsyncPromise(AsyncPromise &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/detail/async_promise.hpp:56
	// ("cv::AsyncPromise::AsyncPromise", vec![(pred!(mut, ["o"], ["cv::AsyncPromise*"]), _)]),
	void cv_AsyncPromise_AsyncPromise_AsyncPromiseRR(cv::AsyncPromise* o, Result<cv::AsyncPromise*>* ocvrs_return) {
		try {
			cv::AsyncPromise* ret = new cv::AsyncPromise(std::move(*o));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(AsyncPromise &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/detail/async_promise.hpp:57
	// ("cv::AsyncPromise::operator=", vec![(pred!(mut, ["o"], ["cv::AsyncPromise*"]), _)]),
	void cv_AsyncPromise_operatorST_AsyncPromiseRR(cv::AsyncPromise* instance, cv::AsyncPromise* o) {
			instance->operator=(std::move(*o));
	}

	// _getImpl()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/detail/async_promise.hpp:63
	// ("cv::AsyncPromise::_getImpl", vec![(pred!(const, [], []), _)]),
	void* cv_AsyncPromise__getImpl_const(const cv::AsyncPromise* instance) {
			void* ret = instance->_getImpl();
			return ret;
	}

	// cv::AsyncPromise::delete() generated
	// ("cv::AsyncPromise::delete", vec![(pred!(mut, [], []), _)]),
	void cv_AsyncPromise_delete(cv::AsyncPromise* instance) {
			delete instance;
	}

	// AutoLock(Mutex &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:725
	// ("cv::AutoLock::AutoLock", vec![(pred!(mut, ["m"], ["cv::Mutex*"]), _)]),
	void cv_AutoLock_AutoLock_MutexR(cv::Mutex* m, Result<cv::AutoLock*>* ocvrs_return) {
		try {
			cv::AutoLock* ret = new cv::AutoLock(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::AutoLock::delete() generated
	// ("cv::AutoLock::delete", vec![(pred!(mut, [], []), _)]),
	void cv_AutoLock_delete(cv::AutoLock* instance) {
			delete instance;
	}

	// CommandLineParser(int, const char *const *, const String &)(Primitive, VariableArray, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:824
	// ("cv::CommandLineParser::CommandLineParser", vec![(pred!(mut, ["argc", "argv", "keys"], ["int", "const char**", "const cv::String*"]), _)]),
	void cv_CommandLineParser_CommandLineParser_int_const_charXX_const_StringR(int argc, const char** argv, const char* keys, Result<cv::CommandLineParser*>* ocvrs_return) {
		try {
			cv::CommandLineParser* ret = new cv::CommandLineParser(argc, argv, cv::String(keys));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// CommandLineParser(const CommandLineParser &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:827
	// ("cv::CommandLineParser::CommandLineParser", vec![(pred!(mut, ["parser"], ["const cv::CommandLineParser*"]), _)]),
	void cv_CommandLineParser_CommandLineParser_const_CommandLineParserR(const cv::CommandLineParser* parser, Result<cv::CommandLineParser*>* ocvrs_return) {
		try {
			cv::CommandLineParser* ret = new cv::CommandLineParser(*parser);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const CommandLineParser &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:830
	// ("cv::CommandLineParser::operator=", vec![(pred!(mut, ["parser"], ["const cv::CommandLineParser*"]), _)]),
	void cv_CommandLineParser_operatorST_const_CommandLineParserR(cv::CommandLineParser* instance, const cv::CommandLineParser* parser, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*parser);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPathToApplication()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:845
	// ("cv::CommandLineParser::getPathToApplication", vec![(pred!(const, [], []), _)]),
	void cv_CommandLineParser_getPathToApplication_const(const cv::CommandLineParser* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getPathToApplication();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CommandLineParser::get(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:879
	// ("cv::CommandLineParser::get", vec![(pred!(const, ["name", "space_delete"], ["const cv::String*", "bool"]), _)]),
	void cv_CommandLineParser_get_bool_const_const_StringR_bool(const cv::CommandLineParser* instance, const char* name, bool space_delete, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->get<bool>(cv::String(name), space_delete);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CommandLineParser::get(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:879
	// ("cv::CommandLineParser::get", vec![(pred!(const, ["name"], ["const cv::String*"]), _)]),
	void cv_CommandLineParser_get_bool_const_const_StringR(const cv::CommandLineParser* instance, const char* name, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->get<bool>(cv::String(name));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CommandLineParser::get(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:879
	// ("cv::CommandLineParser::get", vec![(pred!(const, ["name", "space_delete"], ["const cv::String*", "bool"]), _)]),
	void cv_CommandLineParser_get_int_const_const_StringR_bool(const cv::CommandLineParser* instance, const char* name, bool space_delete, Result<int>* ocvrs_return) {
		try {
			int ret = instance->get<int>(cv::String(name), space_delete);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CommandLineParser::get(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:879
	// ("cv::CommandLineParser::get", vec![(pred!(const, ["name"], ["const cv::String*"]), _)]),
	void cv_CommandLineParser_get_int_const_const_StringR(const cv::CommandLineParser* instance, const char* name, Result<int>* ocvrs_return) {
		try {
			int ret = instance->get<int>(cv::String(name));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CommandLineParser::get(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:879
	// ("cv::CommandLineParser::get", vec![(pred!(const, ["name", "space_delete"], ["const cv::String*", "bool"]), _)]),
	void cv_CommandLineParser_get_double_const_const_StringR_bool(const cv::CommandLineParser* instance, const char* name, bool space_delete, Result<double>* ocvrs_return) {
		try {
			double ret = instance->get<double>(cv::String(name), space_delete);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CommandLineParser::get(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:879
	// ("cv::CommandLineParser::get", vec![(pred!(const, ["name"], ["const cv::String*"]), _)]),
	void cv_CommandLineParser_get_double_const_const_StringR(const cv::CommandLineParser* instance, const char* name, Result<double>* ocvrs_return) {
		try {
			double ret = instance->get<double>(cv::String(name));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CommandLineParser::get(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:879
	// ("cv::CommandLineParser::get", vec![(pred!(const, ["name", "space_delete"], ["const cv::String*", "bool"]), _)]),
	void cv_CommandLineParser_get_cv_String_const_const_StringR_bool(const cv::CommandLineParser* instance, const char* name, bool space_delete, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->get<cv::String>(cv::String(name), space_delete);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CommandLineParser::get(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:879
	// ("cv::CommandLineParser::get", vec![(pred!(const, ["name"], ["const cv::String*"]), _)]),
	void cv_CommandLineParser_get_cv_String_const_const_StringR(const cv::CommandLineParser* instance, const char* name, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->get<cv::String>(cv::String(name));
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CommandLineParser::get(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:879
	// ("cv::CommandLineParser::get", vec![(pred!(const, ["name", "space_delete"], ["const cv::String*", "bool"]), _)]),
	void cv_CommandLineParser_get_uint64_t_const_const_StringR_bool(const cv::CommandLineParser* instance, const char* name, bool space_delete, Result<uint64_t>* ocvrs_return) {
		try {
			uint64_t ret = instance->get<uint64_t>(cv::String(name), space_delete);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CommandLineParser::get(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:879
	// ("cv::CommandLineParser::get", vec![(pred!(const, ["name"], ["const cv::String*"]), _)]),
	void cv_CommandLineParser_get_uint64_t_const_const_StringR(const cv::CommandLineParser* instance, const char* name, Result<uint64_t>* ocvrs_return) {
		try {
			uint64_t ret = instance->get<uint64_t>(cv::String(name));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CommandLineParser::get(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:911
	// ("cv::CommandLineParser::get", vec![(pred!(const, ["index", "space_delete"], ["int", "bool"]), _)]),
	void cv_CommandLineParser_get_bool_const_int_bool(const cv::CommandLineParser* instance, int index, bool space_delete, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->get<bool>(index, space_delete);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CommandLineParser::get(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:911
	// ("cv::CommandLineParser::get", vec![(pred!(const, ["index"], ["int"]), _)]),
	void cv_CommandLineParser_get_bool_const_int(const cv::CommandLineParser* instance, int index, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->get<bool>(index);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CommandLineParser::get(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:911
	// ("cv::CommandLineParser::get", vec![(pred!(const, ["index", "space_delete"], ["int", "bool"]), _)]),
	void cv_CommandLineParser_get_int_const_int_bool(const cv::CommandLineParser* instance, int index, bool space_delete, Result<int>* ocvrs_return) {
		try {
			int ret = instance->get<int>(index, space_delete);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CommandLineParser::get(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:911
	// ("cv::CommandLineParser::get", vec![(pred!(const, ["index"], ["int"]), _)]),
	void cv_CommandLineParser_get_int_const_int(const cv::CommandLineParser* instance, int index, Result<int>* ocvrs_return) {
		try {
			int ret = instance->get<int>(index);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CommandLineParser::get(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:911
	// ("cv::CommandLineParser::get", vec![(pred!(const, ["index", "space_delete"], ["int", "bool"]), _)]),
	void cv_CommandLineParser_get_double_const_int_bool(const cv::CommandLineParser* instance, int index, bool space_delete, Result<double>* ocvrs_return) {
		try {
			double ret = instance->get<double>(index, space_delete);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CommandLineParser::get(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:911
	// ("cv::CommandLineParser::get", vec![(pred!(const, ["index"], ["int"]), _)]),
	void cv_CommandLineParser_get_double_const_int(const cv::CommandLineParser* instance, int index, Result<double>* ocvrs_return) {
		try {
			double ret = instance->get<double>(index);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CommandLineParser::get(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:911
	// ("cv::CommandLineParser::get", vec![(pred!(const, ["index", "space_delete"], ["int", "bool"]), _)]),
	void cv_CommandLineParser_get_cv_String_const_int_bool(const cv::CommandLineParser* instance, int index, bool space_delete, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->get<cv::String>(index, space_delete);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CommandLineParser::get(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:911
	// ("cv::CommandLineParser::get", vec![(pred!(const, ["index"], ["int"]), _)]),
	void cv_CommandLineParser_get_cv_String_const_int(const cv::CommandLineParser* instance, int index, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->get<cv::String>(index);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CommandLineParser::get(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:911
	// ("cv::CommandLineParser::get", vec![(pred!(const, ["index", "space_delete"], ["int", "bool"]), _)]),
	void cv_CommandLineParser_get_uint64_t_const_int_bool(const cv::CommandLineParser* instance, int index, bool space_delete, Result<uint64_t>* ocvrs_return) {
		try {
			uint64_t ret = instance->get<uint64_t>(index, space_delete);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CommandLineParser::get(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:911
	// ("cv::CommandLineParser::get", vec![(pred!(const, ["index"], ["int"]), _)]),
	void cv_CommandLineParser_get_uint64_t_const_int(const cv::CommandLineParser* instance, int index, Result<uint64_t>* ocvrs_return) {
		try {
			uint64_t ret = instance->get<uint64_t>(index);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// has(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:922
	// ("cv::CommandLineParser::has", vec![(pred!(const, ["name"], ["const cv::String*"]), _)]),
	void cv_CommandLineParser_has_const_const_StringR(const cv::CommandLineParser* instance, const char* name, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->has(cv::String(name));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// check()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:929
	// ("cv::CommandLineParser::check", vec![(pred!(const, [], []), _)]),
	void cv_CommandLineParser_check_const(const cv::CommandLineParser* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->check();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// about(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:935
	// ("cv::CommandLineParser::about", vec![(pred!(mut, ["message"], ["const cv::String*"]), _)]),
	void cv_CommandLineParser_about_const_StringR(cv::CommandLineParser* instance, const char* message, ResultVoid* ocvrs_return) {
		try {
			instance->about(cv::String(message));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// printMessage()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:943
	// ("cv::CommandLineParser::printMessage", vec![(pred!(const, [], []), _)]),
	void cv_CommandLineParser_printMessage_const(const cv::CommandLineParser* instance, ResultVoid* ocvrs_return) {
		try {
			instance->printMessage();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// printErrors()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:949
	// ("cv::CommandLineParser::printErrors", vec![(pred!(const, [], []), _)]),
	void cv_CommandLineParser_printErrors_const(const cv::CommandLineParser* instance, ResultVoid* ocvrs_return) {
		try {
			instance->printErrors();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CommandLineParser::delete() generated
	// ("cv::CommandLineParser::delete", vec![(pred!(mut, [], []), _)]),
	void cv_CommandLineParser_delete(cv::CommandLineParser* instance) {
			delete instance;
	}

	// create(const Ptr<MinProblemSolver::Function> &, TermCriteria)(CppPassByVoidPtr, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/optim.hpp:252
	// ("cv::ConjGradSolver::create", vec![(pred!(mut, ["f", "termcrit"], ["const cv::Ptr<cv::MinProblemSolver::Function>*", "cv::TermCriteria"]), _)]),
	void cv_ConjGradSolver_create_const_PtrLFunctionGR_TermCriteria(const cv::Ptr<cv::MinProblemSolver::Function>* f, cv::TermCriteria* termcrit, Result<cv::Ptr<cv::ConjGradSolver>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ConjGradSolver> ret = cv::ConjGradSolver::create(*f, *termcrit);
			Ok(new cv::Ptr<cv::ConjGradSolver>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ConjGradSolver::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/optim.hpp:252
	// ("cv::ConjGradSolver::create", vec![(pred!(mut, [], []), _)]),
	void cv_ConjGradSolver_create(Result<cv::Ptr<cv::ConjGradSolver>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ConjGradSolver> ret = cv::ConjGradSolver::create();
			Ok(new cv::Ptr<cv::ConjGradSolver>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ConjGradSolver::to_Algorithm() generated
	// ("cv::ConjGradSolver::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ConjGradSolver_to_Algorithm(cv::ConjGradSolver* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ConjGradSolver::to_MinProblemSolver() generated
	// ("cv::ConjGradSolver::to_MinProblemSolver", vec![(pred!(mut, [], []), _)]),
	cv::MinProblemSolver* cv_ConjGradSolver_to_MinProblemSolver(cv::ConjGradSolver* instance) {
			return dynamic_cast<cv::MinProblemSolver*>(instance);
	}

	// cv::ConjGradSolver::delete() generated
	// ("cv::ConjGradSolver::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ConjGradSolver_delete(cv::ConjGradSolver* instance) {
			delete instance;
	}

	// DMatch()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:799
	// ("cv::DMatch::DMatch", vec![(pred!(mut, [], []), _)]),
	void cv_DMatch_DMatch(Result<cv::DMatch>* ocvrs_return) {
		try {
			cv::DMatch ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DMatch(int, int, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:800
	// ("cv::DMatch::DMatch", vec![(pred!(mut, ["_queryIdx", "_trainIdx", "_distance"], ["int", "int", "float"]), _)]),
	void cv_DMatch_DMatch_int_int_float(int _queryIdx, int _trainIdx, float _distance, Result<cv::DMatch>* ocvrs_return) {
		try {
			cv::DMatch ret(_queryIdx, _trainIdx, _distance);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DMatch(int, int, int, float)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:801
	// ("cv::DMatch::DMatch", vec![(pred!(mut, ["_queryIdx", "_trainIdx", "_imgIdx", "_distance"], ["int", "int", "int", "float"]), _)]),
	void cv_DMatch_DMatch_int_int_int_float(int _queryIdx, int _trainIdx, int _imgIdx, float _distance, Result<cv::DMatch>* ocvrs_return) {
		try {
			cv::DMatch ret(_queryIdx, _trainIdx, _imgIdx, _distance);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator<(const DMatch &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:810
	// ("cv::DMatch::operator<", vec![(pred!(const, ["m"], ["const cv::DMatch*"]), _)]),
	void cv_DMatch_operatorL_const_const_DMatchR(const cv::DMatch* instance, const cv::DMatch* m, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator<(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getInitStep(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/optim.hpp:164
	// ("cv::DownhillSolver::getInitStep", vec![(pred!(const, ["step"], ["const cv::_OutputArray*"]), _)]),
	void cv_DownhillSolver_getInitStep_const_const__OutputArrayR(const cv::DownhillSolver* instance, const cv::_OutputArray* step, ResultVoid* ocvrs_return) {
		try {
			instance->getInitStep(*step);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInitStep(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/optim.hpp:180
	// ("cv::DownhillSolver::setInitStep", vec![(pred!(mut, ["step"], ["const cv::_InputArray*"]), _)]),
	void cv_DownhillSolver_setInitStep_const__InputArrayR(cv::DownhillSolver* instance, const cv::_InputArray* step, ResultVoid* ocvrs_return) {
		try {
			instance->setInitStep(*step);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const Ptr<MinProblemSolver::Function> &, InputArray, TermCriteria)(CppPassByVoidPtr, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/optim.hpp:198
	// ("cv::DownhillSolver::create", vec![(pred!(mut, ["f", "initStep", "termcrit"], ["const cv::Ptr<cv::MinProblemSolver::Function>*", "const cv::_InputArray*", "cv::TermCriteria"]), _)]),
	void cv_DownhillSolver_create_const_PtrLFunctionGR_const__InputArrayR_TermCriteria(const cv::Ptr<cv::MinProblemSolver::Function>* f, const cv::_InputArray* initStep, cv::TermCriteria* termcrit, Result<cv::Ptr<cv::DownhillSolver>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DownhillSolver> ret = cv::DownhillSolver::create(*f, *initStep, *termcrit);
			Ok(new cv::Ptr<cv::DownhillSolver>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::DownhillSolver::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/optim.hpp:198
	// ("cv::DownhillSolver::create", vec![(pred!(mut, [], []), _)]),
	void cv_DownhillSolver_create(Result<cv::Ptr<cv::DownhillSolver>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DownhillSolver> ret = cv::DownhillSolver::create();
			Ok(new cv::Ptr<cv::DownhillSolver>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::DownhillSolver::to_Algorithm() generated
	// ("cv::DownhillSolver::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_DownhillSolver_to_Algorithm(cv::DownhillSolver* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::DownhillSolver::to_MinProblemSolver() generated
	// ("cv::DownhillSolver::to_MinProblemSolver", vec![(pred!(mut, [], []), _)]),
	cv::MinProblemSolver* cv_DownhillSolver_to_MinProblemSolver(cv::DownhillSolver* instance) {
			return dynamic_cast<cv::MinProblemSolver*>(instance);
	}

	// cv::DownhillSolver::delete() generated
	// ("cv::DownhillSolver::delete", vec![(pred!(mut, [], []), _)]),
	void cv_DownhillSolver_delete(cv::DownhillSolver* instance) {
			delete instance;
	}

	// Exception()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:119
	// ("cv::Exception::Exception", vec![(pred!(mut, [], []), _)]),
	void cv_Exception_Exception(Result<cv::Exception*>* ocvrs_return) {
		try {
			cv::Exception* ret = new cv::Exception();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Exception(int, const String &, const String &, const String &, int)(Primitive, InString, InString, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:124
	// ("cv::Exception::Exception", vec![(pred!(mut, ["_code", "_err", "_func", "_file", "_line"], ["int", "const cv::String*", "const cv::String*", "const cv::String*", "int"]), _)]),
	void cv_Exception_Exception_int_const_StringR_const_StringR_const_StringR_int(int _code, const char* _err, const char* _func, const char* _file, int _line, Result<cv::Exception*>* ocvrs_return) {
		try {
			cv::Exception* ret = new cv::Exception(_code, cv::String(_err), cv::String(_func), cv::String(_file), _line);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// what()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:130
	// ("cv::Exception::what", vec![(pred!(const, [], []), _)]),
	void cv_Exception_what_const(const cv::Exception* instance, Result<void*>* ocvrs_return) {
		try {
			const char* ret = instance->what();
			Ok(ocvrs_create_string(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// formatMessage()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:131
	// ("cv::Exception::formatMessage", vec![(pred!(mut, [], []), _)]),
	void cv_Exception_formatMessage(cv::Exception* instance, ResultVoid* ocvrs_return) {
		try {
			instance->formatMessage();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Exception::msg() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:133
	// ("cv::Exception::msg", vec![(pred!(const, [], []), _)]),
	void* cv_Exception_propMsg_const(const cv::Exception* instance) {
			cv::String ret = instance->msg;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::Exception::setMsg(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:133
	// ("cv::Exception::setMsg", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	void cv_Exception_propMsg_const_String(cv::Exception* instance, const char* val) {
			instance->msg = cv::String(val);
	}

	// cv::Exception::code() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:135
	// ("cv::Exception::code", vec![(pred!(const, [], []), _)]),
	int cv_Exception_propCode_const(const cv::Exception* instance) {
			int ret = instance->code;
			return ret;
	}

	// cv::Exception::setCode(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:135
	// ("cv::Exception::setCode", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_Exception_propCode_const_int(cv::Exception* instance, const int val) {
			instance->code = val;
	}

	// cv::Exception::err() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:136
	// ("cv::Exception::err", vec![(pred!(const, [], []), _)]),
	void* cv_Exception_propErr_const(const cv::Exception* instance) {
			cv::String ret = instance->err;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::Exception::setErr(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:136
	// ("cv::Exception::setErr", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	void cv_Exception_propErr_const_String(cv::Exception* instance, const char* val) {
			instance->err = cv::String(val);
	}

	// cv::Exception::func() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:137
	// ("cv::Exception::func", vec![(pred!(const, [], []), _)]),
	void* cv_Exception_propFunc_const(const cv::Exception* instance) {
			cv::String ret = instance->func;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::Exception::setFunc(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:137
	// ("cv::Exception::setFunc", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	void cv_Exception_propFunc_const_String(cv::Exception* instance, const char* val) {
			instance->func = cv::String(val);
	}

	// cv::Exception::file() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:138
	// ("cv::Exception::file", vec![(pred!(const, [], []), _)]),
	void* cv_Exception_propFile_const(const cv::Exception* instance) {
			cv::String ret = instance->file;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::Exception::setFile(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:138
	// ("cv::Exception::setFile", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	void cv_Exception_propFile_const_String(cv::Exception* instance, const char* val) {
			instance->file = cv::String(val);
	}

	// cv::Exception::line() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:139
	// ("cv::Exception::line", vec![(pred!(const, [], []), _)]),
	int cv_Exception_propLine_const(const cv::Exception* instance) {
			int ret = instance->line;
			return ret;
	}

	// cv::Exception::setLine(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:139
	// ("cv::Exception::setLine", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_Exception_propLine_const_int(cv::Exception* instance, const int val) {
			instance->line = val;
	}

	// cv::Exception::delete() generated
	// ("cv::Exception::delete", vec![(pred!(mut, [], []), _)]),
	void cv_Exception_delete(cv::Exception* instance) {
			delete instance;
	}

	// FileNode()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:527
	// ("cv::FileNode::FileNode", vec![(pred!(mut, [], []), _)]),
	void cv_FileNode_FileNode(Result<cv::FileNode*>* ocvrs_return) {
		try {
			cv::FileNode* ret = new cv::FileNode();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// FileNode(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:538
	// ("cv::FileNode::FileNode", vec![(pred!(mut, ["node"], ["const cv::FileNode*"]), _)]),
	void cv_FileNode_FileNode_const_FileNodeR(const cv::FileNode* node, Result<cv::FileNode*>* ocvrs_return) {
		try {
			cv::FileNode* ret = new cv::FileNode(*node);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:540
	// ("cv::FileNode::operator=", vec![(pred!(mut, ["node"], ["const cv::FileNode*"]), _)]),
	void cv_FileNode_operatorST_const_FileNodeR(cv::FileNode* instance, const cv::FileNode* node, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*node);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator[](const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:546
	// ("cv::FileNode::operator[]", vec![(pred!(const, ["nodename"], ["const cv::String*"]), _)]),
	void cv_FileNode_operator___const_const_StringR(const cv::FileNode* instance, const char* nodename, Result<cv::FileNode*>* ocvrs_return) {
		try {
			cv::FileNode ret = instance->operator[](cv::String(nodename));
			Ok(new cv::FileNode(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator[](const char *)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:551
	// ("cv::FileNode::operator[]", vec![(pred!(const, ["nodename"], ["const char*"]), _)]),
	void cv_FileNode_operator___const_const_charX(const cv::FileNode* instance, const char* nodename, Result<cv::FileNode*>* ocvrs_return) {
		try {
			cv::FileNode ret = instance->operator[](nodename);
			Ok(new cv::FileNode(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator[](int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:556
	// ("cv::FileNode::operator[]", vec![(pred!(const, ["i"], ["int"]), _)]),
	void cv_FileNode_operator___const_int(const cv::FileNode* instance, int i, Result<cv::FileNode*>* ocvrs_return) {
		try {
			cv::FileNode ret = instance->operator[](i);
			Ok(new cv::FileNode(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// keys()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:561
	// ("cv::FileNode::keys", vec![(pred!(const, [], []), _)]),
	void cv_FileNode_keys_const(const cv::FileNode* instance, Result<std::vector<cv::String>*>* ocvrs_return) {
		try {
			std::vector<cv::String> ret = instance->keys();
			Ok(new std::vector<cv::String>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// type()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:566
	// ("cv::FileNode::type", vec![(pred!(const, [], []), _)]),
	void cv_FileNode_type_const(const cv::FileNode* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->type();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:569
	// ("cv::FileNode::empty", vec![(pred!(const, [], []), _)]),
	void cv_FileNode_empty_const(const cv::FileNode* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isNone()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:571
	// ("cv::FileNode::isNone", vec![(pred!(const, [], []), _)]),
	void cv_FileNode_isNone_const(const cv::FileNode* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isNone();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isSeq()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:573
	// ("cv::FileNode::isSeq", vec![(pred!(const, [], []), _)]),
	void cv_FileNode_isSeq_const(const cv::FileNode* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isSeq();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isMap()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:575
	// ("cv::FileNode::isMap", vec![(pred!(const, [], []), _)]),
	void cv_FileNode_isMap_const(const cv::FileNode* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isMap();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isInt()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:577
	// ("cv::FileNode::isInt", vec![(pred!(const, [], []), _)]),
	void cv_FileNode_isInt_const(const cv::FileNode* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isInt();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isReal()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:579
	// ("cv::FileNode::isReal", vec![(pred!(const, [], []), _)]),
	void cv_FileNode_isReal_const(const cv::FileNode* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isReal();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isString()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:581
	// ("cv::FileNode::isString", vec![(pred!(const, [], []), _)]),
	void cv_FileNode_isString_const(const cv::FileNode* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isString();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isNamed()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:583
	// ("cv::FileNode::isNamed", vec![(pred!(const, [], []), _)]),
	void cv_FileNode_isNamed_const(const cv::FileNode* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isNamed();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// name()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:585
	// ("cv::FileNode::name", vec![(pred!(const, [], []), _)]),
	void cv_FileNode_name_const(const cv::FileNode* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->name();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// size()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:587
	// ("cv::FileNode::size", vec![(pred!(const, [], []), _)]),
	void cv_FileNode_size_const(const cv::FileNode* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->size();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator int()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:589
	// ("cv::FileNode::operator int", vec![(pred!(const, [], []), _)]),
	void cv_FileNode_operator_int_const(const cv::FileNode* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->operator int();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator float()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:591
	// ("cv::FileNode::operator float", vec![(pred!(const, [], []), _)]),
	void cv_FileNode_operator_float_const(const cv::FileNode* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->operator float();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator double()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:593
	// ("cv::FileNode::operator double", vec![(pred!(const, [], []), _)]),
	void cv_FileNode_operator_double_const(const cv::FileNode* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->operator double();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator String()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:595
	// ("cv::FileNode::operator cv::String", vec![(pred!(const, [], []), _)]),
	void cv_FileNode_operator_cv_String_const(const cv::FileNode* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->operator cv::String();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator basic_string()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:596
	// ("cv::FileNode::operator std::string", vec![(pred!(const, [], []), _)]),
	void cv_FileNode_operator_std_string_const(const cv::FileNode* instance, Result<void*>* ocvrs_return) {
		try {
			std::string ret = instance->operator std::string();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// begin()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:604
	// ("cv::FileNode::begin", vec![(pred!(const, [], []), _)]),
	void cv_FileNode_begin_const(const cv::FileNode* instance, Result<cv::FileNodeIterator*>* ocvrs_return) {
		try {
			cv::FileNodeIterator ret = instance->begin();
			Ok(new cv::FileNodeIterator(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// end()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:606
	// ("cv::FileNode::end", vec![(pred!(const, [], []), _)]),
	void cv_FileNode_end_const(const cv::FileNode* instance, Result<cv::FileNodeIterator*>* ocvrs_return) {
		try {
			cv::FileNodeIterator ret = instance->end();
			Ok(new cv::FileNodeIterator(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readRaw(const String &, uchar *, size_t)(InString, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:616
	// ("cv::FileNode::readRaw", vec![(pred!(const, ["fmt", "vec", "len"], ["const cv::String*", "unsigned char*", "size_t"]), _)]),
	void cv_FileNode_readRaw_const_const_StringR_unsigned_charX_size_t(const cv::FileNode* instance, const char* fmt, unsigned char* vec, size_t len, ResultVoid* ocvrs_return) {
		try {
			instance->readRaw(cv::String(fmt), vec, len);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readObj()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:619
	// ("cv::FileNode::readObj", vec![(pred!(const, [], []), _)]),
	void cv_FileNode_readObj_const(const cv::FileNode* instance, Result<void*>* ocvrs_return) {
		try {
			void* ret = instance->readObj();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// real()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:622
	// ("cv::FileNode::real", vec![(pred!(const, [], []), _)]),
	void cv_FileNode_real_const(const cv::FileNode* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->real();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// string()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:624
	// ("cv::FileNode::string", vec![(pred!(const, [], []), _)]),
	void cv_FileNode_string_const(const cv::FileNode* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->string();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mat()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:626
	// ("cv::FileNode::mat", vec![(pred!(const, [], []), _)]),
	void cv_FileNode_mat_const(const cv::FileNode* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->mat();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::FileNode::implicitClone() generated
	// ("cv::FileNode::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::FileNode* cv_FileNode_implicitClone_const(const cv::FileNode* instance) {
			return new cv::FileNode(*instance);
	}

	// cv::FileNode::delete() generated
	// ("cv::FileNode::delete", vec![(pred!(mut, [], []), _)]),
	void cv_FileNode_delete(cv::FileNode* instance) {
			delete instance;
	}

	// FileNodeIterator()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:647
	// ("cv::FileNodeIterator::FileNodeIterator", vec![(pred!(mut, [], []), _)]),
	void cv_FileNodeIterator_FileNodeIterator(Result<cv::FileNodeIterator*>* ocvrs_return) {
		try {
			cv::FileNodeIterator* ret = new cv::FileNodeIterator();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// FileNodeIterator(const FileNodeIterator &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:659
	// ("cv::FileNodeIterator::FileNodeIterator", vec![(pred!(mut, ["it"], ["const cv::FileNodeIterator*"]), _)]),
	void cv_FileNodeIterator_FileNodeIterator_const_FileNodeIteratorR(const cv::FileNodeIterator* it, Result<cv::FileNodeIterator*>* ocvrs_return) {
		try {
			cv::FileNodeIterator* ret = new cv::FileNodeIterator(*it);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const FileNodeIterator &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:661
	// ("cv::FileNodeIterator::operator=", vec![(pred!(mut, ["it"], ["const cv::FileNodeIterator*"]), _)]),
	void cv_FileNodeIterator_operatorST_const_FileNodeIteratorR(cv::FileNodeIterator* instance, const cv::FileNodeIterator* it, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*it);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator*()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:664
	// ("cv::FileNodeIterator::operator*", vec![(pred!(const, [], []), _)]),
	void cv_FileNodeIterator_operatorX_const(const cv::FileNodeIterator* instance, Result<cv::FileNode*>* ocvrs_return) {
		try {
			cv::FileNode ret = instance->operator*();
			Ok(new cv::FileNode(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator++()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:669
	// ("cv::FileNodeIterator::operator++", vec![(pred!(mut, [], []), _)]),
	void cv_FileNodeIterator_operatorAA(cv::FileNodeIterator* instance, Result<cv::FileNodeIterator*>* ocvrs_return) {
		try {
			cv::FileNodeIterator ret = instance->operator++();
			Ok(new cv::FileNodeIterator(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator--()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:673
	// ("cv::FileNodeIterator::operator--", vec![(pred!(mut, [], []), _)]),
	void cv_FileNodeIterator_operatorSS(cv::FileNodeIterator* instance, Result<cv::FileNodeIterator*>* ocvrs_return) {
		try {
			cv::FileNodeIterator ret = instance->operator--();
			Ok(new cv::FileNodeIterator(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readRaw(const String &, uchar *, size_t)(InString, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:690
	// ("cv::FileNodeIterator::readRaw", vec![(pred!(mut, ["fmt", "vec", "len"], ["const cv::String*", "unsigned char*", "size_t"]), _)]),
	void cv_FileNodeIterator_readRaw_const_StringR_unsigned_charX_size_t(cv::FileNodeIterator* instance, const char* fmt, unsigned char* vec, size_t len, Result<cv::FileNodeIterator*>* ocvrs_return) {
		try {
			cv::FileNodeIterator ret = instance->readRaw(cv::String(fmt), vec, len);
			Ok(new cv::FileNodeIterator(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::FileNodeIterator::reader() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:707
	// ("cv::FileNodeIterator::reader", vec![(pred!(const, [], []), _)]),
	cv::FileNodeIterator::SeqReader* cv_FileNodeIterator_propReader_const(const cv::FileNodeIterator* instance) {
			cv::FileNodeIterator::SeqReader ret = instance->reader;
			return new cv::FileNodeIterator::SeqReader(ret);
	}

	// cv::FileNodeIterator::setReader(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:707
	// ("cv::FileNodeIterator::setReader", vec![(pred!(mut, ["val"], ["const cv::FileNodeIterator::SeqReader"]), _)]),
	void cv_FileNodeIterator_propReader_const_SeqReader(cv::FileNodeIterator* instance, const cv::FileNodeIterator::SeqReader* val) {
			instance->reader = *val;
	}

	// cv::FileNodeIterator::remaining() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:708
	// ("cv::FileNodeIterator::remaining", vec![(pred!(const, [], []), _)]),
	size_t cv_FileNodeIterator_propRemaining_const(const cv::FileNodeIterator* instance) {
			size_t ret = instance->remaining;
			return ret;
	}

	// cv::FileNodeIterator::setRemaining(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:708
	// ("cv::FileNodeIterator::setRemaining", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
	void cv_FileNodeIterator_propRemaining_const_size_t(cv::FileNodeIterator* instance, const size_t val) {
			instance->remaining = val;
	}

	// cv::FileNodeIterator::delete() generated
	// ("cv::FileNodeIterator::delete", vec![(pred!(mut, [], []), _)]),
	void cv_FileNodeIterator_delete(cv::FileNodeIterator* instance) {
			delete instance;
	}

	// cv::FileNodeIterator::SeqReader::defaultNew() generated
	// ("cv::FileNodeIterator::SeqReader::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::FileNodeIterator::SeqReader* cv_FileNodeIterator_SeqReader_defaultNew_const() {
			cv::FileNodeIterator::SeqReader* ret = new cv::FileNodeIterator::SeqReader();
			return ret;
	}

	// cv::FileNodeIterator::SeqReader::header_size() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:695
	// ("cv::FileNodeIterator::SeqReader::header_size", vec![(pred!(const, [], []), _)]),
	int cv_FileNodeIterator_SeqReader_propHeader_size_const(const cv::FileNodeIterator::SeqReader* instance) {
			int ret = instance->header_size;
			return ret;
	}

	// cv::FileNodeIterator::SeqReader::setHeader_size(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:695
	// ("cv::FileNodeIterator::SeqReader::setHeader_size", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_FileNodeIterator_SeqReader_propHeader_size_const_int(cv::FileNodeIterator::SeqReader* instance, const int val) {
			instance->header_size = val;
	}

	// cv::FileNodeIterator::SeqReader::seq() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:696
	// ("cv::FileNodeIterator::SeqReader::seq", vec![(pred!(mut, [], []), _)]),
	void* cv_FileNodeIterator_SeqReader_propSeq(cv::FileNodeIterator::SeqReader* instance) {
			void* ret = instance->seq;
			return ret;
	}

	// cv::FileNodeIterator::SeqReader::setSeq(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:696
	// ("cv::FileNodeIterator::SeqReader::setSeq", vec![(pred!(mut, ["val"], ["void*"]), _)]),
	void cv_FileNodeIterator_SeqReader_propSeq_voidX(cv::FileNodeIterator::SeqReader* instance, void* const val) {
			instance->seq = val;
	}

	// cv::FileNodeIterator::SeqReader::block() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:697
	// ("cv::FileNodeIterator::SeqReader::block", vec![(pred!(mut, [], []), _)]),
	void* cv_FileNodeIterator_SeqReader_propBlock(cv::FileNodeIterator::SeqReader* instance) {
			void* ret = instance->block;
			return ret;
	}

	// cv::FileNodeIterator::SeqReader::setBlock(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:697
	// ("cv::FileNodeIterator::SeqReader::setBlock", vec![(pred!(mut, ["val"], ["void*"]), _)]),
	void cv_FileNodeIterator_SeqReader_propBlock_voidX(cv::FileNodeIterator::SeqReader* instance, void* const val) {
			instance->block = val;
	}

	// cv::FileNodeIterator::SeqReader::ptr() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:698
	// ("cv::FileNodeIterator::SeqReader::ptr", vec![(pred!(const, [], []), _)]),
	signed char* cv_FileNodeIterator_SeqReader_propPtr_const(const cv::FileNodeIterator::SeqReader* instance) {
			signed char* const ret = instance->ptr;
			return ret;
	}

	// cv::FileNodeIterator::SeqReader::ptrMut() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:698
	// ("cv::FileNodeIterator::SeqReader::ptrMut", vec![(pred!(mut, [], []), _)]),
	signed char* cv_FileNodeIterator_SeqReader_propPtr(cv::FileNodeIterator::SeqReader* instance) {
			signed char* ret = instance->ptr;
			return ret;
	}

	// cv::FileNodeIterator::SeqReader::setPtr(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:698
	// ("cv::FileNodeIterator::SeqReader::setPtr", vec![(pred!(mut, ["val"], ["signed char*"]), _)]),
	void cv_FileNodeIterator_SeqReader_propPtr_signed_charX(cv::FileNodeIterator::SeqReader* instance, signed char* const val) {
			instance->ptr = val;
	}

	// cv::FileNodeIterator::SeqReader::block_min() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:699
	// ("cv::FileNodeIterator::SeqReader::block_min", vec![(pred!(const, [], []), _)]),
	signed char* cv_FileNodeIterator_SeqReader_propBlock_min_const(const cv::FileNodeIterator::SeqReader* instance) {
			signed char* const ret = instance->block_min;
			return ret;
	}

	// cv::FileNodeIterator::SeqReader::block_minMut() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:699
	// ("cv::FileNodeIterator::SeqReader::block_minMut", vec![(pred!(mut, [], []), _)]),
	signed char* cv_FileNodeIterator_SeqReader_propBlock_min(cv::FileNodeIterator::SeqReader* instance) {
			signed char* ret = instance->block_min;
			return ret;
	}

	// cv::FileNodeIterator::SeqReader::setBlock_min(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:699
	// ("cv::FileNodeIterator::SeqReader::setBlock_min", vec![(pred!(mut, ["val"], ["signed char*"]), _)]),
	void cv_FileNodeIterator_SeqReader_propBlock_min_signed_charX(cv::FileNodeIterator::SeqReader* instance, signed char* const val) {
			instance->block_min = val;
	}

	// cv::FileNodeIterator::SeqReader::block_max() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:700
	// ("cv::FileNodeIterator::SeqReader::block_max", vec![(pred!(const, [], []), _)]),
	signed char* cv_FileNodeIterator_SeqReader_propBlock_max_const(const cv::FileNodeIterator::SeqReader* instance) {
			signed char* const ret = instance->block_max;
			return ret;
	}

	// cv::FileNodeIterator::SeqReader::block_maxMut() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:700
	// ("cv::FileNodeIterator::SeqReader::block_maxMut", vec![(pred!(mut, [], []), _)]),
	signed char* cv_FileNodeIterator_SeqReader_propBlock_max(cv::FileNodeIterator::SeqReader* instance) {
			signed char* ret = instance->block_max;
			return ret;
	}

	// cv::FileNodeIterator::SeqReader::setBlock_max(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:700
	// ("cv::FileNodeIterator::SeqReader::setBlock_max", vec![(pred!(mut, ["val"], ["signed char*"]), _)]),
	void cv_FileNodeIterator_SeqReader_propBlock_max_signed_charX(cv::FileNodeIterator::SeqReader* instance, signed char* const val) {
			instance->block_max = val;
	}

	// cv::FileNodeIterator::SeqReader::delta_index() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:701
	// ("cv::FileNodeIterator::SeqReader::delta_index", vec![(pred!(const, [], []), _)]),
	int cv_FileNodeIterator_SeqReader_propDelta_index_const(const cv::FileNodeIterator::SeqReader* instance) {
			int ret = instance->delta_index;
			return ret;
	}

	// cv::FileNodeIterator::SeqReader::setDelta_index(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:701
	// ("cv::FileNodeIterator::SeqReader::setDelta_index", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_FileNodeIterator_SeqReader_propDelta_index_const_int(cv::FileNodeIterator::SeqReader* instance, const int val) {
			instance->delta_index = val;
	}

	// cv::FileNodeIterator::SeqReader::prev_elem() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:702
	// ("cv::FileNodeIterator::SeqReader::prev_elem", vec![(pred!(const, [], []), _)]),
	signed char* cv_FileNodeIterator_SeqReader_propPrev_elem_const(const cv::FileNodeIterator::SeqReader* instance) {
			signed char* const ret = instance->prev_elem;
			return ret;
	}

	// cv::FileNodeIterator::SeqReader::prev_elemMut() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:702
	// ("cv::FileNodeIterator::SeqReader::prev_elemMut", vec![(pred!(mut, [], []), _)]),
	signed char* cv_FileNodeIterator_SeqReader_propPrev_elem(cv::FileNodeIterator::SeqReader* instance) {
			signed char* ret = instance->prev_elem;
			return ret;
	}

	// cv::FileNodeIterator::SeqReader::setPrev_elem(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:702
	// ("cv::FileNodeIterator::SeqReader::setPrev_elem", vec![(pred!(mut, ["val"], ["signed char*"]), _)]),
	void cv_FileNodeIterator_SeqReader_propPrev_elem_signed_charX(cv::FileNodeIterator::SeqReader* instance, signed char* const val) {
			instance->prev_elem = val;
	}

	// cv::FileNodeIterator::SeqReader::delete() generated
	// ("cv::FileNodeIterator::SeqReader::delete", vec![(pred!(mut, [], []), _)]),
	void cv_FileNodeIterator_SeqReader_delete(cv::FileNodeIterator::SeqReader* instance) {
			delete instance;
	}

	// FileStorage()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:340
	// ("cv::FileStorage::FileStorage", vec![(pred!(mut, [], []), _)]),
	void cv_FileStorage_FileStorage(Result<cv::FileStorage*>* ocvrs_return) {
		try {
			cv::FileStorage* ret = new cv::FileStorage();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// FileStorage(const String &, int, const String &)(InString, Primitive, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:345
	// ("cv::FileStorage::FileStorage", vec![(pred!(mut, ["filename", "flags", "encoding"], ["const cv::String*", "int", "const cv::String*"]), _)]),
	void cv_FileStorage_FileStorage_const_StringR_int_const_StringR(const char* filename, int flags, const char* encoding, Result<cv::FileStorage*>* ocvrs_return) {
		try {
			cv::FileStorage* ret = new cv::FileStorage(cv::String(filename), flags, cv::String(encoding));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::FileStorage::FileStorage(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:345
	// ("cv::FileStorage::FileStorage", vec![(pred!(mut, ["filename", "flags"], ["const cv::String*", "int"]), _)]),
	void cv_FileStorage_FileStorage_const_StringR_int(const char* filename, int flags, Result<cv::FileStorage*>* ocvrs_return) {
		try {
			cv::FileStorage* ret = new cv::FileStorage(cv::String(filename), flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// open(const String &, int, const String &)(InString, Primitive, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:368
	// ("cv::FileStorage::open", vec![(pred!(mut, ["filename", "flags", "encoding"], ["const cv::String*", "int", "const cv::String*"]), _)]),
	void cv_FileStorage_open_const_StringR_int_const_StringR(cv::FileStorage* instance, const char* filename, int flags, const char* encoding, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(cv::String(filename), flags, cv::String(encoding));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::FileStorage::open(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:368
	// ("cv::FileStorage::open", vec![(pred!(mut, ["filename", "flags"], ["const cv::String*", "int"]), _)]),
	void cv_FileStorage_open_const_StringR_int(cv::FileStorage* instance, const char* filename, int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(cv::String(filename), flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isOpened()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:375
	// ("cv::FileStorage::isOpened", vec![(pred!(const, [], []), _)]),
	void cv_FileStorage_isOpened_const(const cv::FileStorage* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isOpened();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// release()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:381
	// ("cv::FileStorage::release", vec![(pred!(mut, [], []), _)]),
	void cv_FileStorage_release(cv::FileStorage* instance, ResultVoid* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// releaseAndGetString()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:388
	// ("cv::FileStorage::releaseAndGetString", vec![(pred!(mut, [], []), _)]),
	void cv_FileStorage_releaseAndGetString(cv::FileStorage* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->releaseAndGetString();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFirstTopLevelNode()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:393
	// ("cv::FileStorage::getFirstTopLevelNode", vec![(pred!(const, [], []), _)]),
	void cv_FileStorage_getFirstTopLevelNode_const(const cv::FileStorage* instance, Result<cv::FileNode*>* ocvrs_return) {
		try {
			cv::FileNode ret = instance->getFirstTopLevelNode();
			Ok(new cv::FileNode(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// root(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:400
	// ("cv::FileStorage::root", vec![(pred!(const, ["streamidx"], ["int"]), _)]),
	void cv_FileStorage_root_const_int(const cv::FileStorage* instance, int streamidx, Result<cv::FileNode*>* ocvrs_return) {
		try {
			cv::FileNode ret = instance->root(streamidx);
			Ok(new cv::FileNode(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::FileStorage::root() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:400
	// ("cv::FileStorage::root", vec![(pred!(const, [], []), _)]),
	void cv_FileStorage_root_const(const cv::FileStorage* instance, Result<cv::FileNode*>* ocvrs_return) {
		try {
			cv::FileNode ret = instance->root();
			Ok(new cv::FileNode(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator[](const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:406
	// ("cv::FileStorage::operator[]", vec![(pred!(const, ["nodename"], ["const cv::String*"]), _)]),
	void cv_FileStorage_operator___const_const_StringR(const cv::FileStorage* instance, const char* nodename, Result<cv::FileNode*>* ocvrs_return) {
		try {
			cv::FileNode ret = instance->operator[](cv::String(nodename));
			Ok(new cv::FileNode(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator[](const char *)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:409
	// ("cv::FileStorage::operator[]", vec![(pred!(const, ["nodename"], ["const char*"]), _)]),
	void cv_FileStorage_operator___const_const_charX(const cv::FileStorage* instance, const char* nodename, Result<cv::FileNode*>* ocvrs_return) {
		try {
			cv::FileNode ret = instance->operator[](nodename);
			Ok(new cv::FileNode(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writeRaw(const String &, const uchar *, size_t)(InString, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:427
	// ("cv::FileStorage::writeRaw", vec![(pred!(mut, ["fmt", "vec", "len"], ["const cv::String*", "const unsigned char*", "size_t"]), _)]),
	void cv_FileStorage_writeRaw_const_StringR_const_unsigned_charX_size_t(cv::FileStorage* instance, const char* fmt, const unsigned char* vec, size_t len, ResultVoid* ocvrs_return) {
		try {
			instance->writeRaw(cv::String(fmt), vec, len);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writeObj(const String &, const void *)(InString, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:434
	// ("cv::FileStorage::writeObj", vec![(pred!(mut, ["name", "obj"], ["const cv::String*", "const void*"]), _)]),
	void cv_FileStorage_writeObj_const_StringR_const_voidX(cv::FileStorage* instance, const char* name, const void* obj, ResultVoid* ocvrs_return) {
		try {
			instance->writeObj(cv::String(name), obj);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:441
	// ("cv::FileStorage::write", vec![(pred!(mut, ["name", "val"], ["const cv::String*", "int"]), _)]),
	void cv_FileStorage_write_const_StringR_int(cv::FileStorage* instance, const char* name, int val, ResultVoid* ocvrs_return) {
		try {
			instance->write(cv::String(name), val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(const String &, double)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:443
	// ("cv::FileStorage::write", vec![(pred!(mut, ["name", "val"], ["const cv::String*", "double"]), _)]),
	void cv_FileStorage_write_const_StringR_double(cv::FileStorage* instance, const char* name, double val, ResultVoid* ocvrs_return) {
		try {
			instance->write(cv::String(name), val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:445
	// ("cv::FileStorage::write", vec![(pred!(mut, ["name", "val"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_FileStorage_write_const_StringR_const_StringR(cv::FileStorage* instance, const char* name, const char* val, ResultVoid* ocvrs_return) {
		try {
			instance->write(cv::String(name), cv::String(val));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(const String &, InputArray)(InString, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:447
	// ("cv::FileStorage::write", vec![(pred!(mut, ["name", "val"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
	void cv_FileStorage_write_const_StringR_const__InputArrayR(cv::FileStorage* instance, const char* name, const cv::_InputArray* val, ResultVoid* ocvrs_return) {
		try {
			instance->write(cv::String(name), *val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writeComment(const String &, bool)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:457
	// ("cv::FileStorage::writeComment", vec![(pred!(mut, ["comment", "append"], ["const cv::String*", "bool"]), _)]),
	void cv_FileStorage_writeComment_const_StringR_bool(cv::FileStorage* instance, const char* comment, bool append, ResultVoid* ocvrs_return) {
		try {
			instance->writeComment(cv::String(comment), append);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::FileStorage::writeComment(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:457
	// ("cv::FileStorage::writeComment", vec![(pred!(mut, ["comment"], ["const cv::String*"]), _)]),
	void cv_FileStorage_writeComment_const_StringR(cv::FileStorage* instance, const char* comment, ResultVoid* ocvrs_return) {
		try {
			instance->writeComment(cv::String(comment));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// startWriteStruct(const String &, int, const String &)(InString, Primitive, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:464
	// ("cv::FileStorage::startWriteStruct", vec![(pred!(mut, ["name", "flags", "typeName"], ["const cv::String*", "int", "const cv::String*"]), _)]),
	void cv_FileStorage_startWriteStruct_const_StringR_int_const_StringR(cv::FileStorage* instance, const char* name, int flags, const char* typeName, ResultVoid* ocvrs_return) {
		try {
			instance->startWriteStruct(cv::String(name), flags, cv::String(typeName));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::FileStorage::startWriteStruct(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:464
	// ("cv::FileStorage::startWriteStruct", vec![(pred!(mut, ["name", "flags"], ["const cv::String*", "int"]), _)]),
	void cv_FileStorage_startWriteStruct_const_StringR_int(cv::FileStorage* instance, const char* name, int flags, ResultVoid* ocvrs_return) {
		try {
			instance->startWriteStruct(cv::String(name), flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// endWriteStruct()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:468
	// ("cv::FileStorage::endWriteStruct", vec![(pred!(mut, [], []), _)]),
	void cv_FileStorage_endWriteStruct(cv::FileStorage* instance, ResultVoid* ocvrs_return) {
		try {
			instance->endWriteStruct();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultObjectName(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:474
	// ("cv::FileStorage::getDefaultObjectName", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_FileStorage_getDefaultObjectName_const_StringR(const char* filename, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::FileStorage::getDefaultObjectName(cv::String(filename));
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFormat()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:479
	// ("cv::FileStorage::getFormat", vec![(pred!(const, [], []), _)]),
	void cv_FileStorage_getFormat_const(const cv::FileStorage* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFormat();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::FileStorage::elname() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:482
	// ("cv::FileStorage::elname", vec![(pred!(const, [], []), _)]),
	void* cv_FileStorage_propElname_const(const cv::FileStorage* instance) {
			cv::String ret = instance->elname;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::FileStorage::setElname(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:482
	// ("cv::FileStorage::setElname", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	void cv_FileStorage_propElname_const_String(cv::FileStorage* instance, const char* val) {
			instance->elname = cv::String(val);
	}

	// cv::FileStorage::structs() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:483
	// ("cv::FileStorage::structs", vec![(pred!(const, [], []), _)]),
	std::vector<char>* cv_FileStorage_propStructs_const(const cv::FileStorage* instance) {
			std::vector<char> ret = instance->structs;
			return new std::vector<char>(ret);
	}

	// cv::FileStorage::setStructs(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:483
	// ("cv::FileStorage::setStructs", vec![(pred!(mut, ["val"], ["const std::vector<char>"]), _)]),
	void cv_FileStorage_propStructs_const_vectorLcharG(cv::FileStorage* instance, const std::vector<char>* val) {
			instance->structs = *val;
	}

	// cv::FileStorage::state() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:484
	// ("cv::FileStorage::state", vec![(pred!(const, [], []), _)]),
	int cv_FileStorage_propState_const(const cv::FileStorage* instance) {
			int ret = instance->state;
			return ret;
	}

	// cv::FileStorage::setState(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:484
	// ("cv::FileStorage::setState", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_FileStorage_propState_const_int(cv::FileStorage* instance, const int val) {
			instance->state = val;
	}

	// cv::FileStorage::delete() generated
	// ("cv::FileStorage::delete", vec![(pred!(mut, [], []), _)]),
	void cv_FileStorage_delete(cv::FileStorage* instance) {
			delete instance;
	}

	// next()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:3024
	// ("cv::Formatted::next", vec![(pred!(mut, [], []), _)]),
	void cv_Formatted_next(cv::Formatted* instance, Result<void*>* ocvrs_return) {
		try {
			const char* ret = instance->next();
			Ok(ocvrs_create_string(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reset()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:3025
	// ("cv::Formatted::reset", vec![(pred!(mut, [], []), _)]),
	void cv_Formatted_reset(cv::Formatted* instance, ResultVoid* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Formatted::delete() generated
	// ("cv::Formatted::delete", vec![(pred!(mut, [], []), _)]),
	void cv_Formatted_delete(cv::Formatted* instance) {
			delete instance;
	}

	// format(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:3043
	// ("cv::Formatter::format", vec![(pred!(const, ["mtx"], ["const cv::Mat*"]), _)]),
	void cv_Formatter_format_const_const_MatR(const cv::Formatter* instance, const cv::Mat* mtx, Result<cv::Ptr<cv::Formatted>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::Formatted> ret = instance->format(*mtx);
			Ok(new cv::Ptr<cv::Formatted>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// set32fPrecision(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:3045
	// ("cv::Formatter::set32fPrecision", vec![(pred!(mut, ["p"], ["int"]), _)]),
	void cv_Formatter_set32fPrecision_int(cv::Formatter* instance, int p, ResultVoid* ocvrs_return) {
		try {
			instance->set32fPrecision(p);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Formatter::set32fPrecision() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:3045
	// ("cv::Formatter::set32fPrecision", vec![(pred!(mut, [], []), _)]),
	void cv_Formatter_set32fPrecision(cv::Formatter* instance, ResultVoid* ocvrs_return) {
		try {
			instance->set32fPrecision();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// set64fPrecision(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:3046
	// ("cv::Formatter::set64fPrecision", vec![(pred!(mut, ["p"], ["int"]), _)]),
	void cv_Formatter_set64fPrecision_int(cv::Formatter* instance, int p, ResultVoid* ocvrs_return) {
		try {
			instance->set64fPrecision(p);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Formatter::set64fPrecision() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:3046
	// ("cv::Formatter::set64fPrecision", vec![(pred!(mut, [], []), _)]),
	void cv_Formatter_set64fPrecision(cv::Formatter* instance, ResultVoid* ocvrs_return) {
		try {
			instance->set64fPrecision();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMultiline(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:3047
	// ("cv::Formatter::setMultiline", vec![(pred!(mut, ["ml"], ["bool"]), _)]),
	void cv_Formatter_setMultiline_bool(cv::Formatter* instance, bool ml, ResultVoid* ocvrs_return) {
		try {
			instance->setMultiline(ml);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Formatter::setMultiline() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:3047
	// ("cv::Formatter::setMultiline", vec![(pred!(mut, [], []), _)]),
	void cv_Formatter_setMultiline(cv::Formatter* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setMultiline();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// get(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:3049
	// ("cv::Formatter::get", vec![(pred!(mut, ["fmt"], ["int"]), _)]),
	void cv_Formatter_get_int(int fmt, Result<cv::Ptr<cv::Formatter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::Formatter> ret = cv::Formatter::get(fmt);
			Ok(new cv::Ptr<cv::Formatter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Formatter::get() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:3049
	// ("cv::Formatter::get", vec![(pred!(mut, [], []), _)]),
	void cv_Formatter_get(Result<cv::Ptr<cv::Formatter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::Formatter> ret = cv::Formatter::get();
			Ok(new cv::Ptr<cv::Formatter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Formatter::delete() generated
	// ("cv::Formatter::delete", vec![(pred!(mut, [], []), _)]),
	void cv_Formatter_delete(cv::Formatter* instance) {
			delete instance;
	}

	// operator()(const unsigned char *, const unsigned char *, int)(VariableArray, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/base.hpp:425
	// ("cv::Hamming::operator()", vec![(pred!(const, ["a", "b", "size"], ["const unsigned char*", "const unsigned char*", "int"]), _)]),
	void cv_Hamming_operator___const_const_unsigned_charX_const_unsigned_charX_int(const cv::Hamming* instance, const unsigned char* a, const unsigned char* b, int size, Result<cv::Hamming::ResultType>* ocvrs_return) {
		try {
			cv::Hamming::ResultType ret = instance->operator()(a, b, size);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Hamming::defaultNew() generated
	// ("cv::Hamming::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::Hamming* cv_Hamming_defaultNew_const() {
			cv::Hamming* ret = new cv::Hamming();
			return ret;
	}

	// cv::Hamming::delete() generated
	// ("cv::Hamming::delete", vec![(pred!(mut, [], []), _)]),
	void cv_Hamming_delete(cv::Hamming* instance) {
			delete instance;
	}

	// KeyPoint()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:703
	// ("cv::KeyPoint::KeyPoint", vec![(pred!(mut, [], []), _)]),
	void cv_KeyPoint_KeyPoint(Result<cv::KeyPoint*>* ocvrs_return) {
		try {
			cv::KeyPoint* ret = new cv::KeyPoint();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// KeyPoint(Point2f, float, float, float, int, int)(SimpleClass, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:712
	// ("cv::KeyPoint::KeyPoint", vec![(pred!(mut, ["pt", "size", "angle", "response", "octave", "class_id"], ["cv::Point2f", "float", "float", "float", "int", "int"]), _)]),
	void cv_KeyPoint_KeyPoint_Point2f_float_float_float_int_int(cv::Point2f* pt, float size, float angle, float response, int octave, int class_id, Result<cv::KeyPoint*>* ocvrs_return) {
		try {
			cv::KeyPoint* ret = new cv::KeyPoint(*pt, size, angle, response, octave, class_id);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::KeyPoint::KeyPoint(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:712
	// ("cv::KeyPoint::KeyPoint", vec![(pred!(mut, ["pt", "size"], ["cv::Point2f", "float"]), _)]),
	void cv_KeyPoint_KeyPoint_Point2f_float(cv::Point2f* pt, float size, Result<cv::KeyPoint*>* ocvrs_return) {
		try {
			cv::KeyPoint* ret = new cv::KeyPoint(*pt, size);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// KeyPoint(float, float, float, float, float, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:722
	// ("cv::KeyPoint::KeyPoint", vec![(pred!(mut, ["x", "y", "size", "angle", "response", "octave", "class_id"], ["float", "float", "float", "float", "float", "int", "int"]), _)]),
	void cv_KeyPoint_KeyPoint_float_float_float_float_float_int_int(float x, float y, float size, float angle, float response, int octave, int class_id, Result<cv::KeyPoint*>* ocvrs_return) {
		try {
			cv::KeyPoint* ret = new cv::KeyPoint(x, y, size, angle, response, octave, class_id);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::KeyPoint::KeyPoint(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:722
	// ("cv::KeyPoint::KeyPoint", vec![(pred!(mut, ["x", "y", "size"], ["float", "float", "float"]), _)]),
	void cv_KeyPoint_KeyPoint_float_float_float(float x, float y, float size, Result<cv::KeyPoint*>* ocvrs_return) {
		try {
			cv::KeyPoint* ret = new cv::KeyPoint(x, y, size);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// hash()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:724
	// ("cv::KeyPoint::hash", vec![(pred!(const, [], []), _)]),
	void cv_KeyPoint_hash_const(const cv::KeyPoint* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->hash();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convert(const std::vector<KeyPoint> &, std::vector<Point2f> &, const std::vector<int> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:735
	// ("cv::KeyPoint::convert", vec![(pred!(mut, ["keypoints", "points2f", "keypointIndexes"], ["const std::vector<cv::KeyPoint>*", "std::vector<cv::Point2f>*", "const std::vector<int>*"]), _)]),
	void cv_KeyPoint_convert_const_vectorLKeyPointGR_vectorLPoint2fGR_const_vectorLintGR(const std::vector<cv::KeyPoint>* keypoints, std::vector<cv::Point2f>* points2f, const std::vector<int>* keypointIndexes, ResultVoid* ocvrs_return) {
		try {
			cv::KeyPoint::convert(*keypoints, *points2f, *keypointIndexes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::KeyPoint::convert(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:735
	// ("cv::KeyPoint::convert", vec![(pred!(mut, ["keypoints", "points2f"], ["const std::vector<cv::KeyPoint>*", "std::vector<cv::Point2f>*"]), _)]),
	void cv_KeyPoint_convert_const_vectorLKeyPointGR_vectorLPoint2fGR(const std::vector<cv::KeyPoint>* keypoints, std::vector<cv::Point2f>* points2f, ResultVoid* ocvrs_return) {
		try {
			cv::KeyPoint::convert(*keypoints, *points2f);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convert(const std::vector<Point2f> &, std::vector<KeyPoint> &, float, float, int, int)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:746
	// ("cv::KeyPoint::convert", vec![(pred!(mut, ["points2f", "keypoints", "size", "response", "octave", "class_id"], ["const std::vector<cv::Point2f>*", "std::vector<cv::KeyPoint>*", "float", "float", "int", "int"]), _)]),
	void cv_KeyPoint_convert_const_vectorLPoint2fGR_vectorLKeyPointGR_float_float_int_int(const std::vector<cv::Point2f>* points2f, std::vector<cv::KeyPoint>* keypoints, float size, float response, int octave, int class_id, ResultVoid* ocvrs_return) {
		try {
			cv::KeyPoint::convert(*points2f, *keypoints, size, response, octave, class_id);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::KeyPoint::convert(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:746
	// ("cv::KeyPoint::convert", vec![(pred!(mut, ["points2f", "keypoints"], ["const std::vector<cv::Point2f>*", "std::vector<cv::KeyPoint>*"]), _)]),
	void cv_KeyPoint_convert_const_vectorLPoint2fGR_vectorLKeyPointGR(const std::vector<cv::Point2f>* points2f, std::vector<cv::KeyPoint>* keypoints, ResultVoid* ocvrs_return) {
		try {
			cv::KeyPoint::convert(*points2f, *keypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// overlap(const KeyPoint &, const KeyPoint &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:757
	// ("cv::KeyPoint::overlap", vec![(pred!(mut, ["kp1", "kp2"], ["const cv::KeyPoint*", "const cv::KeyPoint*"]), _)]),
	void cv_KeyPoint_overlap_const_KeyPointR_const_KeyPointR(const cv::KeyPoint* kp1, const cv::KeyPoint* kp2, Result<float>* ocvrs_return) {
		try {
			float ret = cv::KeyPoint::overlap(*kp1, *kp2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::KeyPoint::implicitClone() generated
	// ("cv::KeyPoint::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::KeyPoint* cv_KeyPoint_implicitClone_const(const cv::KeyPoint* instance) {
			return new cv::KeyPoint(*instance);
	}

	// cv::KeyPoint::pt() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:759
	// ("cv::KeyPoint::pt", vec![(pred!(const, [], []), _)]),
	void cv_KeyPoint_propPt_const(const cv::KeyPoint* instance, cv::Point2f* ocvrs_return) {
			cv::Point2f ret = instance->pt;
			*ocvrs_return = ret;
	}

	// cv::KeyPoint::setPt(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:759
	// ("cv::KeyPoint::setPt", vec![(pred!(mut, ["val"], ["const cv::Point2f"]), _)]),
	void cv_KeyPoint_propPt_const_Point2f(cv::KeyPoint* instance, const cv::Point2f* val) {
			instance->pt = *val;
	}

	// cv::KeyPoint::size() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:760
	// ("cv::KeyPoint::size", vec![(pred!(const, [], []), _)]),
	float cv_KeyPoint_propSize_const(const cv::KeyPoint* instance) {
			float ret = instance->size;
			return ret;
	}

	// cv::KeyPoint::setSize(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:760
	// ("cv::KeyPoint::setSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_KeyPoint_propSize_const_float(cv::KeyPoint* instance, const float val) {
			instance->size = val;
	}

	// cv::KeyPoint::angle() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:761
	// ("cv::KeyPoint::angle", vec![(pred!(const, [], []), _)]),
	float cv_KeyPoint_propAngle_const(const cv::KeyPoint* instance) {
			float ret = instance->angle;
			return ret;
	}

	// cv::KeyPoint::setAngle(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:761
	// ("cv::KeyPoint::setAngle", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_KeyPoint_propAngle_const_float(cv::KeyPoint* instance, const float val) {
			instance->angle = val;
	}

	// cv::KeyPoint::response() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:764
	// ("cv::KeyPoint::response", vec![(pred!(const, [], []), _)]),
	float cv_KeyPoint_propResponse_const(const cv::KeyPoint* instance) {
			float ret = instance->response;
			return ret;
	}

	// cv::KeyPoint::setResponse(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:764
	// ("cv::KeyPoint::setResponse", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_KeyPoint_propResponse_const_float(cv::KeyPoint* instance, const float val) {
			instance->response = val;
	}

	// cv::KeyPoint::octave() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:765
	// ("cv::KeyPoint::octave", vec![(pred!(const, [], []), _)]),
	int cv_KeyPoint_propOctave_const(const cv::KeyPoint* instance) {
			int ret = instance->octave;
			return ret;
	}

	// cv::KeyPoint::setOctave(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:765
	// ("cv::KeyPoint::setOctave", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_KeyPoint_propOctave_const_int(cv::KeyPoint* instance, const int val) {
			instance->octave = val;
	}

	// cv::KeyPoint::class_id() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:766
	// ("cv::KeyPoint::class_id", vec![(pred!(const, [], []), _)]),
	int cv_KeyPoint_propClass_id_const(const cv::KeyPoint* instance) {
			int ret = instance->class_id;
			return ret;
	}

	// cv::KeyPoint::setClass_id(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:766
	// ("cv::KeyPoint::setClass_id", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_KeyPoint_propClass_id_const_int(cv::KeyPoint* instance, const int val) {
			instance->class_id = val;
	}

	// cv::KeyPoint::delete() generated
	// ("cv::KeyPoint::delete", vec![(pred!(mut, [], []), _)]),
	void cv_KeyPoint_delete(cv::KeyPoint* instance) {
			delete instance;
	}

	// LDA(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2565
	// ("cv::LDA::LDA", vec![(pred!(mut, ["num_components"], ["int"]), _)]),
	void cv_LDA_LDA_int(int num_components, Result<cv::LDA*>* ocvrs_return) {
		try {
			cv::LDA* ret = new cv::LDA(num_components);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::LDA::LDA() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2565
	// ("cv::LDA::LDA", vec![(pred!(mut, [], []), _)]),
	void cv_LDA_LDA(Result<cv::LDA*>* ocvrs_return) {
		try {
			cv::LDA* ret = new cv::LDA();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// LDA(InputArrayOfArrays, InputArray, int)(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2572
	// ("cv::LDA::LDA", vec![(pred!(mut, ["src", "labels", "num_components"], ["const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
	void cv_LDA_LDA_const__InputArrayR_const__InputArrayR_int(const cv::_InputArray* src, const cv::_InputArray* labels, int num_components, Result<cv::LDA*>* ocvrs_return) {
		try {
			cv::LDA* ret = new cv::LDA(*src, *labels, num_components);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::LDA::LDA(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2572
	// ("cv::LDA::LDA", vec![(pred!(mut, ["src", "labels"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_LDA_LDA_const__InputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_InputArray* labels, Result<cv::LDA*>* ocvrs_return) {
		try {
			cv::LDA* ret = new cv::LDA(*src, *labels);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// save(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2576
	// ("cv::LDA::save", vec![(pred!(const, ["filename"], ["const cv::String*"]), _)]),
	void cv_LDA_save_const_const_StringR(const cv::LDA* instance, const char* filename, ResultVoid* ocvrs_return) {
		try {
			instance->save(cv::String(filename));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// load(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2580
	// ("cv::LDA::load", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_LDA_load_const_StringR(cv::LDA* instance, const char* filename, ResultVoid* ocvrs_return) {
		try {
			instance->load(cv::String(filename));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// save(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2584
	// ("cv::LDA::save", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_LDA_save_const_FileStorageR(const cv::LDA* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->save(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// load(const FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2588
	// ("cv::LDA::load", vec![(pred!(mut, ["node"], ["const cv::FileStorage*"]), _)]),
	void cv_LDA_load_const_FileStorageR(cv::LDA* instance, const cv::FileStorage* node, ResultVoid* ocvrs_return) {
		try {
			instance->load(*node);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArrayOfArrays, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2596
	// ("cv::LDA::compute", vec![(pred!(mut, ["src", "labels"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_LDA_compute_const__InputArrayR_const__InputArrayR(cv::LDA* instance, const cv::_InputArray* src, const cv::_InputArray* labels, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*src, *labels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// project(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2601
	// ("cv::LDA::project", vec![(pred!(mut, ["src"], ["const cv::_InputArray*"]), _)]),
	void cv_LDA_project_const__InputArrayR(cv::LDA* instance, const cv::_InputArray* src, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->project(*src);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reconstruct(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2606
	// ("cv::LDA::reconstruct", vec![(pred!(mut, ["src"], ["const cv::_InputArray*"]), _)]),
	void cv_LDA_reconstruct_const__InputArrayR(cv::LDA* instance, const cv::_InputArray* src, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->reconstruct(*src);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// eigenvectors()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2610
	// ("cv::LDA::eigenvectors", vec![(pred!(const, [], []), _)]),
	void cv_LDA_eigenvectors_const(const cv::LDA* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->eigenvectors();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// eigenvalues()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2614
	// ("cv::LDA::eigenvalues", vec![(pred!(const, [], []), _)]),
	void cv_LDA_eigenvalues_const(const cv::LDA* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->eigenvalues();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// subspaceProject(InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2616
	// ("cv::LDA::subspaceProject", vec![(pred!(mut, ["W", "mean", "src"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_LDA_subspaceProject_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* W, const cv::_InputArray* mean, const cv::_InputArray* src, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::LDA::subspaceProject(*W, *mean, *src);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// subspaceReconstruct(InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2617
	// ("cv::LDA::subspaceReconstruct", vec![(pred!(mut, ["W", "mean", "src"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_LDA_subspaceReconstruct_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* W, const cv::_InputArray* mean, const cv::_InputArray* src, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::LDA::subspaceReconstruct(*W, *mean, *src);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::LDA::delete() generated
	// ("cv::LDA::delete", vec![(pred!(mut, [], []), _)]),
	void cv_LDA_delete(cv::LDA* instance) {
			delete instance;
	}

	// Mat()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:831
	// ("cv::Mat::Mat", vec![(pred!(mut, [], []), _)]),
	cv::Mat* cv_Mat_Mat() {
			cv::Mat* ret = new cv::Mat();
			return ret;
	}

	// Mat(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:839
	// ("cv::Mat::Mat", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
	void cv_Mat_Mat_int_int_int(int rows, int cols, int type, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(rows, cols, type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Mat(Size, int)(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:847
	// ("cv::Mat::Mat", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
	void cv_Mat_Mat_Size_int(cv::Size* size, int type, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(*size, type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Mat(int, int, int, const Scalar &)(Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:858
	// ("cv::Mat::Mat", vec![(pred!(mut, ["rows", "cols", "type", "s"], ["int", "int", "int", "const cv::Scalar*"]), _)]),
	void cv_Mat_Mat_int_int_int_const_ScalarR(int rows, int cols, int type, const cv::Scalar* s, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(rows, cols, type, *s);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Mat(Size, int, const Scalar &)(SimpleClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:869
	// ("cv::Mat::Mat", vec![(pred!(mut, ["size", "type", "s"], ["cv::Size", "int", "const cv::Scalar*"]), _)]),
	void cv_Mat_Mat_Size_int_const_ScalarR(cv::Size* size, int type, const cv::Scalar* s, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(*size, type, *s);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Mat(int, const int *, int)(Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:877
	// ("cv::Mat::Mat", vec![(pred!(mut, ["ndims", "sizes", "type"], ["int", "const int*", "int"]), _)]),
	void cv_Mat_Mat_int_const_intX_int(int ndims, const int* sizes, int type, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(ndims, sizes, type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Mat(const std::vector<int> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:884
	// ("cv::Mat::Mat", vec![(pred!(mut, ["sizes", "type"], ["const std::vector<int>*", "int"]), _)]),
	void cv_Mat_Mat_const_vectorLintGR_int(const std::vector<int>* sizes, int type, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(*sizes, type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Mat(int, const int *, int, const Scalar &)(Primitive, VariableArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:895
	// ("cv::Mat::Mat", vec![(pred!(mut, ["ndims", "sizes", "type", "s"], ["int", "const int*", "int", "const cv::Scalar*"]), _)]),
	void cv_Mat_Mat_int_const_intX_int_const_ScalarR(int ndims, const int* sizes, int type, const cv::Scalar* s, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(ndims, sizes, type, *s);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Mat(const std::vector<int> &, int, const Scalar &)(CppPassByVoidPtr, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:905
	// ("cv::Mat::Mat", vec![(pred!(mut, ["sizes", "type", "s"], ["const std::vector<int>*", "int", "const cv::Scalar*"]), _)]),
	void cv_Mat_Mat_const_vectorLintGR_int_const_ScalarR(const std::vector<int>* sizes, int type, const cv::Scalar* s, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(*sizes, type, *s);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Mat(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:915
	// ("cv::Mat::Mat", vec![(pred!(mut, ["m"], ["const cv::Mat*"]), _)]),
	void cv_Mat_Mat_const_MatR(const cv::Mat* m, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat* ret = new const cv::Mat(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::Mat(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:915
	// ("cv::Mat::Mat", vec![(pred!(mut, ["m"], ["cv::Mat*"]), _)]),
	void cv_Mat_Mat_MatR(cv::Mat* m, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Mat(int, int, int, void *, size_t)(Primitive, Primitive, Primitive, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:931
	// ("cv::Mat::Mat", vec![(pred!(mut, ["rows", "cols", "type", "data", "step"], ["int", "int", "int", "void*", "size_t"]), _)]),
	void cv_Mat_Mat_int_int_int_voidX_size_t(int rows, int cols, int type, void* data, size_t step, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(rows, cols, type, data, step);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::Mat(Primitive, Primitive, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:931
	// ("cv::Mat::Mat", vec![(pred!(mut, ["rows", "cols", "type", "data"], ["int", "int", "int", "void*"]), _)]),
	void cv_Mat_Mat_int_int_int_voidX(int rows, int cols, int type, void* data, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(rows, cols, type, data);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Mat(Size, int, void *, size_t)(SimpleClass, Primitive, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:947
	// ("cv::Mat::Mat", vec![(pred!(mut, ["size", "type", "data", "step"], ["cv::Size", "int", "void*", "size_t"]), _)]),
	void cv_Mat_Mat_Size_int_voidX_size_t(cv::Size* size, int type, void* data, size_t step, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(*size, type, data, step);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::Mat(SimpleClass, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:947
	// ("cv::Mat::Mat", vec![(pred!(mut, ["size", "type", "data"], ["cv::Size", "int", "void*"]), _)]),
	void cv_Mat_Mat_Size_int_voidX(cv::Size* size, int type, void* data, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(*size, type, data);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Mat(int, const int *, int, void *, const size_t *)(Primitive, VariableArray, Primitive, Indirect, VariableArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:962
	// ("cv::Mat::Mat", vec![(pred!(mut, ["ndims", "sizes", "type", "data", "steps"], ["int", "const int*", "int", "void*", "const size_t*"]), _)]),
	void cv_Mat_Mat_int_const_intX_int_voidX_const_size_tX(int ndims, const int* sizes, int type, void* data, const size_t* steps, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(ndims, sizes, type, data, steps);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::Mat(Primitive, VariableArray, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:962
	// ("cv::Mat::Mat", vec![(pred!(mut, ["ndims", "sizes", "type", "data"], ["int", "const int*", "int", "void*"]), _)]),
	void cv_Mat_Mat_int_const_intX_int_voidX(int ndims, const int* sizes, int type, void* data, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(ndims, sizes, type, data);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Mat(const std::vector<int> &, int, void *, const size_t *)(CppPassByVoidPtr, Primitive, Indirect, VariableArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:976
	// ("cv::Mat::Mat", vec![(pred!(mut, ["sizes", "type", "data", "steps"], ["const std::vector<int>*", "int", "void*", "const size_t*"]), _)]),
	void cv_Mat_Mat_const_vectorLintGR_int_voidX_const_size_tX(const std::vector<int>* sizes, int type, void* data, const size_t* steps, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(*sizes, type, data, steps);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::Mat(CppPassByVoidPtr, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:976
	// ("cv::Mat::Mat", vec![(pred!(mut, ["sizes", "type", "data"], ["const std::vector<int>*", "int", "void*"]), _)]),
	void cv_Mat_Mat_const_vectorLintGR_int_voidX(const std::vector<int>* sizes, int type, void* data, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(*sizes, type, data);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Mat(const Mat &, const Range &, const Range &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:988
	// ("cv::Mat::Mat", vec![(pred!(mut, ["m", "rowRange", "colRange"], ["const cv::Mat*", "const cv::Range*", "const cv::Range*"]), _)]),
	void cv_Mat_Mat_const_MatR_const_RangeR_const_RangeR(const cv::Mat* m, const cv::Range* rowRange, const cv::Range* colRange, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat* ret = new const cv::Mat(*m, *rowRange, *colRange);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::Mat(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:988
	// ("cv::Mat::Mat", vec![(pred!(mut, ["m", "rowRange"], ["cv::Mat*", "const cv::Range*"]), _)]),
	void cv_Mat_Mat_MatR_const_RangeR(cv::Mat* m, const cv::Range* rowRange, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(*m, *rowRange);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::Mat(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:988
	// ("cv::Mat::Mat", vec![(pred!(mut, ["m", "rowRange"], ["const cv::Mat*", "const cv::Range*"]), _)]),
	void cv_Mat_Mat_const_MatR_const_RangeR(const cv::Mat* m, const cv::Range* rowRange, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat* ret = new const cv::Mat(*m, *rowRange);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::Mat(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:988
	// ("cv::Mat::Mat", vec![(pred!(mut, ["m", "rowRange", "colRange"], ["cv::Mat*", "const cv::Range*", "const cv::Range*"]), _)]),
	void cv_Mat_Mat_MatR_const_RangeR_const_RangeR(cv::Mat* m, const cv::Range* rowRange, const cv::Range* colRange, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(*m, *rowRange, *colRange);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Mat(const Mat &, const Rect &)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:998
	// ("cv::Mat::Mat", vec![(pred!(mut, ["m", "roi"], ["const cv::Mat*", "const cv::Rect*"]), _)]),
	void cv_Mat_Mat_const_MatR_const_RectR(const cv::Mat* m, const cv::Rect* roi, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat* ret = new const cv::Mat(*m, *roi);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::Mat(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:998
	// ("cv::Mat::Mat", vec![(pred!(mut, ["m", "roi"], ["cv::Mat*", "const cv::Rect*"]), _)]),
	void cv_Mat_Mat_MatR_const_RectR(cv::Mat* m, const cv::Rect* roi, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(*m, *roi);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Mat(const Mat &, const std::vector<Range> &)(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1018
	// ("cv::Mat::Mat", vec![(pred!(mut, ["m", "ranges"], ["const cv::Mat*", "const std::vector<cv::Range>*"]), _)]),
	void cv_Mat_Mat_const_MatR_const_vectorLRangeGR(const cv::Mat* m, const std::vector<cv::Range>* ranges, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat* ret = new const cv::Mat(*m, *ranges);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::Mat(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1018
	// ("cv::Mat::Mat", vec![(pred!(mut, ["m", "ranges"], ["cv::Mat*", "const std::vector<cv::Range>*"]), _)]),
	void cv_Mat_Mat_MatR_const_vectorLRangeGR(cv::Mat* m, const std::vector<cv::Range>* ranges, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(*m, *ranges);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Mat(const cuda::GpuMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1076
	// ("cv::Mat::Mat", vec![(pred!(mut, ["m"], ["const cv::cuda::GpuMat*"]), _)]),
	void cv_Mat_Mat_const_GpuMatR(const cv::cuda::GpuMat* m, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const MatExpr &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1098
	// ("cv::Mat::operator=", vec![(pred!(mut, ["expr"], ["const cv::MatExpr*"]), _)]),
	void cv_Mat_operatorST_const_MatExprR(cv::Mat* instance, const cv::MatExpr* expr, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*expr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUMat(int, UMatUsageFlags)(Primitive, Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1101
	// ("cv::Mat::getUMat", vec![(pred!(const, ["accessFlags", "usageFlags"], ["int", "cv::UMatUsageFlags"]), _)]),
	void cv_Mat_getUMat_const_int_UMatUsageFlags(const cv::Mat* instance, int accessFlags, cv::UMatUsageFlags usageFlags, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->getUMat(accessFlags, usageFlags);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::getUMat(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1101
	// ("cv::Mat::getUMat", vec![(pred!(const, ["accessFlags"], ["int"]), _)]),
	void cv_Mat_getUMat_const_int(const cv::Mat* instance, int accessFlags, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->getUMat(accessFlags);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// row(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1136
	// ("cv::Mat::row", vec![(pred!(const, ["y"], ["int"]), _)]),
	void cv_Mat_row_const_int(const cv::Mat* instance, int y, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat ret = instance->row(y);
			Ok(new const cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::row(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1136
	// ("cv::Mat::row", vec![(pred!(mut, ["y"], ["int"]), _)]),
	void cv_Mat_row_int(cv::Mat* instance, int y, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->row(y);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// col(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1145
	// ("cv::Mat::col", vec![(pred!(const, ["x"], ["int"]), _)]),
	void cv_Mat_col_const_int(const cv::Mat* instance, int x, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat ret = instance->col(x);
			Ok(new const cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::col(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1145
	// ("cv::Mat::col", vec![(pred!(mut, ["x"], ["int"]), _)]),
	void cv_Mat_col_int(cv::Mat* instance, int x, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->col(x);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// rowRange(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1154
	// ("cv::Mat::rowRange", vec![(pred!(const, ["startrow", "endrow"], ["int", "int"]), _)]),
	void cv_Mat_rowRange_const_int_int(const cv::Mat* instance, int startrow, int endrow, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat ret = instance->rowRange(startrow, endrow);
			Ok(new const cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::rowRange(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1154
	// ("cv::Mat::rowRange", vec![(pred!(mut, ["startrow", "endrow"], ["int", "int"]), _)]),
	void cv_Mat_rowRange_int_int(cv::Mat* instance, int startrow, int endrow, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->rowRange(startrow, endrow);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// rowRange(const Range &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1159
	// ("cv::Mat::rowRange", vec![(pred!(const, ["r"], ["const cv::Range*"]), _)]),
	void cv_Mat_rowRange_const_const_RangeR(const cv::Mat* instance, const cv::Range* r, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat ret = instance->rowRange(*r);
			Ok(new const cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::rowRange(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1159
	// ("cv::Mat::rowRange", vec![(pred!(mut, ["r"], ["const cv::Range*"]), _)]),
	void cv_Mat_rowRange_const_RangeR(cv::Mat* instance, const cv::Range* r, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->rowRange(*r);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// colRange(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1168
	// ("cv::Mat::colRange", vec![(pred!(const, ["startcol", "endcol"], ["int", "int"]), _)]),
	void cv_Mat_colRange_const_int_int(const cv::Mat* instance, int startcol, int endcol, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat ret = instance->colRange(startcol, endcol);
			Ok(new const cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::colRange(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1168
	// ("cv::Mat::colRange", vec![(pred!(mut, ["startcol", "endcol"], ["int", "int"]), _)]),
	void cv_Mat_colRange_int_int(cv::Mat* instance, int startcol, int endcol, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->colRange(startcol, endcol);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// colRange(const Range &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1173
	// ("cv::Mat::colRange", vec![(pred!(const, ["r"], ["const cv::Range*"]), _)]),
	void cv_Mat_colRange_const_const_RangeR(const cv::Mat* instance, const cv::Range* r, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat ret = instance->colRange(*r);
			Ok(new const cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::colRange(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1173
	// ("cv::Mat::colRange", vec![(pred!(mut, ["r"], ["const cv::Range*"]), _)]),
	void cv_Mat_colRange_const_RangeR(cv::Mat* instance, const cv::Range* r, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->colRange(*r);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// diag(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1209
	// ("cv::Mat::diag", vec![(pred!(const, ["d"], ["int"]), _)]),
	void cv_Mat_diag_const_int(const cv::Mat* instance, int d, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat ret = instance->diag(d);
			Ok(new const cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::diag() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1209
	// ("cv::Mat::diag", vec![(pred!(mut, [], []), _)]),
	void cv_Mat_diag(cv::Mat* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->diag();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::diag() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1209
	// ("cv::Mat::diag", vec![(pred!(const, [], []), _)]),
	void cv_Mat_diag_const(const cv::Mat* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat ret = instance->diag();
			Ok(new const cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::diag(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1209
	// ("cv::Mat::diag", vec![(pred!(mut, ["d"], ["int"]), _)]),
	void cv_Mat_diag_int(cv::Mat* instance, int d, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->diag(d);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// diag(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1216
	// ("cv::Mat::diag", vec![(pred!(mut, ["d"], ["const cv::Mat*"]), _)]),
	void cv_Mat_diag_const_MatR(const cv::Mat* d, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::Mat::diag(*d);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clone()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1223
	// ("cv::Mat::clone", vec![(pred!(const, [], []), _)]),
	void cv_Mat_clone_const(const cv::Mat* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->clone();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// copyTo(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1240
	// ("cv::Mat::copyTo", vec![(pred!(const, ["m"], ["const cv::_OutputArray*"]), _)]),
	void cv_Mat_copyTo_const_const__OutputArrayR(const cv::Mat* instance, const cv::_OutputArray* m, ResultVoid* ocvrs_return) {
		try {
			instance->copyTo(*m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// copyTo(OutputArray, InputArray)(OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1248
	// ("cv::Mat::copyTo", vec![(pred!(const, ["m", "mask"], ["const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_Mat_copyTo_const_const__OutputArrayR_const__InputArrayR(const cv::Mat* instance, const cv::_OutputArray* m, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			instance->copyTo(*m, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertTo(OutputArray, int, double, double)(OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1263
	// ("cv::Mat::convertTo", vec![(pred!(const, ["m", "rtype", "alpha", "beta"], ["const cv::_OutputArray*", "int", "double", "double"]), _)]),
	void cv_Mat_convertTo_const_const__OutputArrayR_int_double_double(const cv::Mat* instance, const cv::_OutputArray* m, int rtype, double alpha, double beta, ResultVoid* ocvrs_return) {
		try {
			instance->convertTo(*m, rtype, alpha, beta);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::convertTo(OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1263
	// ("cv::Mat::convertTo", vec![(pred!(const, ["m", "rtype"], ["const cv::_OutputArray*", "int"]), _)]),
	void cv_Mat_convertTo_const_const__OutputArrayR_int(const cv::Mat* instance, const cv::_OutputArray* m, int rtype, ResultVoid* ocvrs_return) {
		try {
			instance->convertTo(*m, rtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// assignTo(Mat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1271
	// ("cv::Mat::assignTo", vec![(pred!(const, ["m", "type"], ["cv::Mat*", "int"]), _)]),
	void cv_Mat_assignTo_const_MatR_int(const cv::Mat* instance, cv::Mat* m, int type, ResultVoid* ocvrs_return) {
		try {
			instance->assignTo(*m, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::assignTo(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1271
	// ("cv::Mat::assignTo", vec![(pred!(const, ["m"], ["cv::Mat*"]), _)]),
	void cv_Mat_assignTo_const_MatR(const cv::Mat* instance, cv::Mat* m, ResultVoid* ocvrs_return) {
		try {
			instance->assignTo(*m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const Scalar &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1276
	// ("cv::Mat::operator=", vec![(pred!(mut, ["s"], ["const cv::Scalar*"]), _)]),
	void cv_Mat_operatorST_const_ScalarR(cv::Mat* instance, const cv::Scalar* s, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*s);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTo(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1285
	// ("cv::Mat::setTo", vec![(pred!(mut, ["value", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_Mat_setTo_const__InputArrayR_const__InputArrayR(cv::Mat* instance, const cv::_InputArray* value, const cv::_InputArray* mask, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->setTo(*value, *mask);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::setTo(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1285
	// ("cv::Mat::setTo", vec![(pred!(mut, ["value"], ["const cv::_InputArray*"]), _)]),
	void cv_Mat_setTo_const__InputArrayR(cv::Mat* instance, const cv::_InputArray* value, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->setTo(*value);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reshape(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1323
	// ("cv::Mat::reshape", vec![(pred!(const, ["cn", "rows"], ["int", "int"]), _)]),
	void cv_Mat_reshape_const_int_int(const cv::Mat* instance, int cn, int rows, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat ret = instance->reshape(cn, rows);
			Ok(new const cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::reshape(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1323
	// ("cv::Mat::reshape", vec![(pred!(mut, ["cn"], ["int"]), _)]),
	void cv_Mat_reshape_int(cv::Mat* instance, int cn, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->reshape(cn);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::reshape(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1323
	// ("cv::Mat::reshape", vec![(pred!(const, ["cn"], ["int"]), _)]),
	void cv_Mat_reshape_const_int(const cv::Mat* instance, int cn, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat ret = instance->reshape(cn);
			Ok(new const cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::reshape(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1323
	// ("cv::Mat::reshape", vec![(pred!(mut, ["cn", "rows"], ["int", "int"]), _)]),
	void cv_Mat_reshape_int_int(cv::Mat* instance, int cn, int rows, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->reshape(cn, rows);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reshape(int, int, const int *)(Primitive, Primitive, VariableArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1331
	// ("cv::Mat::reshape", vec![(pred!(const, ["cn", "newndims", "newsz"], ["int", "int", "const int*"]), _)]),
	void cv_Mat_reshape_const_int_int_const_intX(const cv::Mat* instance, int cn, int newndims, const int* newsz, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat ret = instance->reshape(cn, newndims, newsz);
			Ok(new const cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::reshape(Primitive, Primitive, VariableArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1331
	// ("cv::Mat::reshape", vec![(pred!(mut, ["cn", "newndims", "newsz"], ["int", "int", "const int*"]), _)]),
	void cv_Mat_reshape_int_int_const_intX(cv::Mat* instance, int cn, int newndims, const int* newsz, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->reshape(cn, newndims, newsz);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reshape(int, const std::vector<int> &)(Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1338
	// ("cv::Mat::reshape", vec![(pred!(const, ["cn", "newshape"], ["int", "const std::vector<int>*"]), _)]),
	void cv_Mat_reshape_const_int_const_vectorLintGR(const cv::Mat* instance, int cn, const std::vector<int>* newshape, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat ret = instance->reshape(cn, *newshape);
			Ok(new const cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::reshape(Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1338
	// ("cv::Mat::reshape", vec![(pred!(mut, ["cn", "newshape"], ["int", "const std::vector<int>*"]), _)]),
	void cv_Mat_reshape_int_const_vectorLintGR(cv::Mat* instance, int cn, const std::vector<int>* newshape, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->reshape(cn, *newshape);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// t()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1350
	// ("cv::Mat::t", vec![(pred!(const, [], []), _)]),
	void cv_Mat_t_const(const cv::Mat* instance, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = instance->t();
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// inv(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1359
	// ("cv::Mat::inv", vec![(pred!(const, ["method"], ["int"]), _)]),
	void cv_Mat_inv_const_int(const cv::Mat* instance, int method, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = instance->inv(method);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::inv() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1359
	// ("cv::Mat::inv", vec![(pred!(const, [], []), _)]),
	void cv_Mat_inv_const(const cv::Mat* instance, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = instance->inv();
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mul(InputArray, double)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1373
	// ("cv::Mat::mul", vec![(pred!(const, ["m", "scale"], ["const cv::_InputArray*", "double"]), _)]),
	void cv_Mat_mul_const_const__InputArrayR_double(const cv::Mat* instance, const cv::_InputArray* m, double scale, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = instance->mul(*m, scale);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::mul(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1373
	// ("cv::Mat::mul", vec![(pred!(const, ["m"], ["const cv::_InputArray*"]), _)]),
	void cv_Mat_mul_const_const__InputArrayR(const cv::Mat* instance, const cv::_InputArray* m, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = instance->mul(*m);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cross(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1382
	// ("cv::Mat::cross", vec![(pred!(const, ["m"], ["const cv::_InputArray*"]), _)]),
	void cv_Mat_cross_const_const__InputArrayR(const cv::Mat* instance, const cv::_InputArray* m, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->cross(*m);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dot(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1392
	// ("cv::Mat::dot", vec![(pred!(const, ["m"], ["const cv::_InputArray*"]), _)]),
	void cv_Mat_dot_const_const__InputArrayR(const cv::Mat* instance, const cv::_InputArray* m, Result<double>* ocvrs_return) {
		try {
			double ret = instance->dot(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// zeros(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1408
	// ("cv::Mat::zeros", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
	void cv_Mat_zeros_int_int_int(int rows, int cols, int type, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::Mat::zeros(rows, cols, type);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// zeros(Size, int)(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1414
	// ("cv::Mat::zeros", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
	void cv_Mat_zeros_Size_int(cv::Size* size, int type, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::Mat::zeros(*size, type);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// zeros(int, const int *, int)(Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1421
	// ("cv::Mat::zeros", vec![(pred!(mut, ["ndims", "sz", "type"], ["int", "const int*", "int"]), _)]),
	void cv_Mat_zeros_int_const_intX_int(int ndims, const int* sz, int type, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::Mat::zeros(ndims, sz, type);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ones(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1439
	// ("cv::Mat::ones", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
	void cv_Mat_ones_int_int_int(int rows, int cols, int type, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::Mat::ones(rows, cols, type);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ones(Size, int)(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1445
	// ("cv::Mat::ones", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
	void cv_Mat_ones_Size_int(cv::Size* size, int type, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::Mat::ones(*size, type);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ones(int, const int *, int)(Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1452
	// ("cv::Mat::ones", vec![(pred!(mut, ["ndims", "sz", "type"], ["int", "const int*", "int"]), _)]),
	void cv_Mat_ones_int_const_intX_int(int ndims, const int* sz, int type, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::Mat::ones(ndims, sz, type);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// eye(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1468
	// ("cv::Mat::eye", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
	void cv_Mat_eye_int_int_int(int rows, int cols, int type, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::Mat::eye(rows, cols, type);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// eye(Size, int)(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1474
	// ("cv::Mat::eye", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
	void cv_Mat_eye_Size_int(cv::Size* size, int type, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::Mat::eye(*size, type);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1509
	// ("cv::Mat::create", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
	void cv_Mat_create_int_int_int(cv::Mat* instance, int rows, int cols, int type, ResultVoid* ocvrs_return) {
		try {
			instance->create(rows, cols, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(Size, int)(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1515
	// ("cv::Mat::create", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
	void cv_Mat_create_Size_int(cv::Mat* instance, cv::Size* size, int type, ResultVoid* ocvrs_return) {
		try {
			instance->create(*size, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, const int *, int)(Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1522
	// ("cv::Mat::create", vec![(pred!(mut, ["ndims", "sizes", "type"], ["int", "const int*", "int"]), _)]),
	void cv_Mat_create_int_const_intX_int(cv::Mat* instance, int ndims, const int* sizes, int type, ResultVoid* ocvrs_return) {
		try {
			instance->create(ndims, sizes, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const std::vector<int> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1528
	// ("cv::Mat::create", vec![(pred!(mut, ["sizes", "type"], ["const std::vector<int>*", "int"]), _)]),
	void cv_Mat_create_const_vectorLintGR_int(cv::Mat* instance, const std::vector<int>* sizes, int type, ResultVoid* ocvrs_return) {
		try {
			instance->create(*sizes, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addref()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1539
	// ("cv::Mat::addref", vec![(pred!(mut, [], []), _)]),
	void cv_Mat_addref(cv::Mat* instance, ResultVoid* ocvrs_return) {
		try {
			instance->addref();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// release()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1554
	// ("cv::Mat::release", vec![(pred!(mut, [], []), _)]),
	void cv_Mat_release(cv::Mat* instance, ResultVoid* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// deallocate()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1557
	// ("cv::Mat::deallocate", vec![(pred!(mut, [], []), _)]),
	void cv_Mat_deallocate(cv::Mat* instance, ResultVoid* ocvrs_return) {
		try {
			instance->deallocate();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reserve(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1568
	// ("cv::Mat::reserve", vec![(pred!(mut, ["sz"], ["size_t"]), _)]),
	void cv_Mat_reserve_size_t(cv::Mat* instance, size_t sz, ResultVoid* ocvrs_return) {
		try {
			instance->reserve(sz);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reserveBuffer(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1576
	// ("cv::Mat::reserveBuffer", vec![(pred!(mut, ["sz"], ["size_t"]), _)]),
	void cv_Mat_reserveBuffer_size_t(cv::Mat* instance, size_t sz, ResultVoid* ocvrs_return) {
		try {
			instance->reserveBuffer(sz);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// resize(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1585
	// ("cv::Mat::resize", vec![(pred!(mut, ["sz"], ["size_t"]), _)]),
	void cv_Mat_resize_size_t(cv::Mat* instance, size_t sz, ResultVoid* ocvrs_return) {
		try {
			instance->resize(sz);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// resize(size_t, const Scalar &)(Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1591
	// ("cv::Mat::resize", vec![(pred!(mut, ["sz", "s"], ["size_t", "const cv::Scalar*"]), _)]),
	void cv_Mat_resize_size_t_const_ScalarR(cv::Mat* instance, size_t sz, const cv::Scalar* s, ResultVoid* ocvrs_return) {
		try {
			instance->resize(sz, *s);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// push_back(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1618
	// ("cv::Mat::push_back", vec![(pred!(mut, ["m"], ["const cv::Mat*"]), _)]),
	void cv_Mat_push_back_const_MatR(cv::Mat* instance, const cv::Mat* m, ResultVoid* ocvrs_return) {
		try {
			instance->push_back(*m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// pop_back(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1626
	// ("cv::Mat::pop_back", vec![(pred!(mut, ["nelems"], ["size_t"]), _)]),
	void cv_Mat_pop_back_size_t(cv::Mat* instance, size_t nelems, ResultVoid* ocvrs_return) {
		try {
			instance->pop_back(nelems);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::pop_back() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1626
	// ("cv::Mat::pop_back", vec![(pred!(mut, [], []), _)]),
	void cv_Mat_pop_back(cv::Mat* instance, ResultVoid* ocvrs_return) {
		try {
			instance->pop_back();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// locateROI(Size &, Point &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1639
	// ("cv::Mat::locateROI", vec![(pred!(const, ["wholeSize", "ofs"], ["cv::Size*", "cv::Point*"]), _)]),
	void cv_Mat_locateROI_const_SizeR_PointR(const cv::Mat* instance, cv::Size* wholeSize, cv::Point* ofs, ResultVoid* ocvrs_return) {
		try {
			instance->locateROI(*wholeSize, *ofs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// adjustROI(int, int, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1668
	// ("cv::Mat::adjustROI", vec![(pred!(mut, ["dtop", "dbottom", "dleft", "dright"], ["int", "int", "int", "int"]), _)]),
	void cv_Mat_adjustROI_int_int_int_int(cv::Mat* instance, int dtop, int dbottom, int dleft, int dright, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->adjustROI(dtop, dbottom, dleft, dright);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(Range, Range)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1681
	// ("cv::Mat::operator()", vec![(pred!(const, ["rowRange", "colRange"], ["cv::Range", "cv::Range"]), _)]),
	void cv_Mat_operator___const_Range_Range(const cv::Mat* instance, cv::Range* rowRange, cv::Range* colRange, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat ret = instance->operator()(*rowRange, *colRange);
			Ok(new const cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::operator()(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1681
	// ("cv::Mat::operator()", vec![(pred!(mut, ["rowRange", "colRange"], ["cv::Range", "cv::Range"]), _)]),
	void cv_Mat_operator___Range_Range(cv::Mat* instance, cv::Range* rowRange, cv::Range* colRange, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->operator()(*rowRange, *colRange);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(const Rect &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1686
	// ("cv::Mat::operator()", vec![(pred!(const, ["roi"], ["const cv::Rect*"]), _)]),
	void cv_Mat_operator___const_const_RectR(const cv::Mat* instance, const cv::Rect* roi, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat ret = instance->operator()(*roi);
			Ok(new const cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::operator()(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1686
	// ("cv::Mat::operator()", vec![(pred!(mut, ["roi"], ["const cv::Rect*"]), _)]),
	void cv_Mat_operator___const_RectR(cv::Mat* instance, const cv::Rect* roi, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->operator()(*roi);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(const std::vector<Range> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1696
	// ("cv::Mat::operator()", vec![(pred!(const, ["ranges"], ["const std::vector<cv::Range>*"]), _)]),
	void cv_Mat_operator___const_const_vectorLRangeGR(const cv::Mat* instance, const std::vector<cv::Range>* ranges, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat ret = instance->operator()(*ranges);
			Ok(new const cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::operator()(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1696
	// ("cv::Mat::operator()", vec![(pred!(mut, ["ranges"], ["const std::vector<cv::Range>*"]), _)]),
	void cv_Mat_operator___const_vectorLRangeGR(cv::Mat* instance, const std::vector<cv::Range>* ranges, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->operator()(*ranges);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isContinuous()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1787
	// ("cv::Mat::isContinuous", vec![(pred!(const, [], []), _)]),
	bool cv_Mat_isContinuous_const(const cv::Mat* instance) {
			bool ret = instance->isContinuous();
			return ret;
	}

	// isSubmatrix()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1790
	// ("cv::Mat::isSubmatrix", vec![(pred!(const, [], []), _)]),
	bool cv_Mat_isSubmatrix_const(const cv::Mat* instance) {
			bool ret = instance->isSubmatrix();
			return ret;
	}

	// elemSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1797
	// ("cv::Mat::elemSize", vec![(pred!(const, [], []), _)]),
	void cv_Mat_elemSize_const(const cv::Mat* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->elemSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// elemSize1()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1804
	// ("cv::Mat::elemSize1", vec![(pred!(const, [], []), _)]),
	size_t cv_Mat_elemSize1_const(const cv::Mat* instance) {
			size_t ret = instance->elemSize1();
			return ret;
	}

	// type()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1811
	// ("cv::Mat::type", vec![(pred!(const, [], []), _)]),
	int cv_Mat_type_const(const cv::Mat* instance) {
			int ret = instance->type();
			return ret;
	}

	// depth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1826
	// ("cv::Mat::depth", vec![(pred!(const, [], []), _)]),
	int cv_Mat_depth_const(const cv::Mat* instance) {
			int ret = instance->depth();
			return ret;
	}

	// channels()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1832
	// ("cv::Mat::channels", vec![(pred!(const, [], []), _)]),
	int cv_Mat_channels_const(const cv::Mat* instance) {
			int ret = instance->channels();
			return ret;
	}

	// step1(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1839
	// ("cv::Mat::step1", vec![(pred!(const, ["i"], ["int"]), _)]),
	void cv_Mat_step1_const_int(const cv::Mat* instance, int i, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->step1(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::step1() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1839
	// ("cv::Mat::step1", vec![(pred!(const, [], []), _)]),
	void cv_Mat_step1_const(const cv::Mat* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->step1();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1846
	// ("cv::Mat::empty", vec![(pred!(const, [], []), _)]),
	bool cv_Mat_empty_const(const cv::Mat* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// total()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1853
	// ("cv::Mat::total", vec![(pred!(const, [], []), _)]),
	size_t cv_Mat_total_const(const cv::Mat* instance) {
			size_t ret = instance->total();
			return ret;
	}

	// total(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1859
	// ("cv::Mat::total", vec![(pred!(const, ["startDim", "endDim"], ["int", "int"]), _)]),
	void cv_Mat_total_const_int_int(const cv::Mat* instance, int startDim, int endDim, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->total(startDim, endDim);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::total(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1859
	// ("cv::Mat::total", vec![(pred!(const, ["startDim"], ["int"]), _)]),
	void cv_Mat_total_const_int(const cv::Mat* instance, int startDim, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->total(startDim);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// checkVector(int, int, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1882
	// ("cv::Mat::checkVector", vec![(pred!(const, ["elemChannels", "depth", "requireContinuous"], ["int", "int", "bool"]), _)]),
	void cv_Mat_checkVector_const_int_int_bool(const cv::Mat* instance, int elemChannels, int depth, bool requireContinuous, Result<int>* ocvrs_return) {
		try {
			int ret = instance->checkVector(elemChannels, depth, requireContinuous);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::checkVector(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1882
	// ("cv::Mat::checkVector", vec![(pred!(const, ["elemChannels"], ["int"]), _)]),
	void cv_Mat_checkVector_const_int(const cv::Mat* instance, int elemChannels, Result<int>* ocvrs_return) {
		try {
			int ret = instance->checkVector(elemChannels);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ptr(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1890
	// ("cv::Mat::ptr", vec![(pred!(mut, ["i0"], ["int"]), _)]),
	void cv_Mat_ptr_int(cv::Mat* instance, int i0, Result<unsigned char*>* ocvrs_return) {
		try {
			unsigned char* ret = instance->ptr(i0);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::ptr() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1890
	// ("cv::Mat::ptr", vec![(pred!(mut, [], []), _)]),
	void cv_Mat_ptr(cv::Mat* instance, Result<unsigned char*>* ocvrs_return) {
		try {
			unsigned char* ret = instance->ptr();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ptr(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1892
	// ("cv::Mat::ptr", vec![(pred!(const, ["i0"], ["int"]), _)]),
	void cv_Mat_ptr_const_int(const cv::Mat* instance, int i0, Result<const unsigned char*>* ocvrs_return) {
		try {
			const unsigned char* ret = instance->ptr(i0);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::ptr() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1892
	// ("cv::Mat::ptr", vec![(pred!(const, [], []), _)]),
	void cv_Mat_ptr_const(const cv::Mat* instance, Result<const unsigned char*>* ocvrs_return) {
		try {
			const unsigned char* ret = instance->ptr();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ptr(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1898
	// ("cv::Mat::ptr", vec![(pred!(mut, ["row", "col"], ["int", "int"]), _)]),
	void cv_Mat_ptr_int_int(cv::Mat* instance, int row, int col, Result<unsigned char*>* ocvrs_return) {
		try {
			unsigned char* ret = instance->ptr(row, col);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ptr(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1903
	// ("cv::Mat::ptr", vec![(pred!(const, ["row", "col"], ["int", "int"]), _)]),
	void cv_Mat_ptr_const_int_int(const cv::Mat* instance, int row, int col, Result<const unsigned char*>* ocvrs_return) {
		try {
			const unsigned char* ret = instance->ptr(row, col);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ptr(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1906
	// ("cv::Mat::ptr", vec![(pred!(mut, ["i0", "i1", "i2"], ["int", "int", "int"]), _)]),
	void cv_Mat_ptr_int_int_int(cv::Mat* instance, int i0, int i1, int i2, Result<unsigned char*>* ocvrs_return) {
		try {
			unsigned char* ret = instance->ptr(i0, i1, i2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ptr(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1908
	// ("cv::Mat::ptr", vec![(pred!(const, ["i0", "i1", "i2"], ["int", "int", "int"]), _)]),
	void cv_Mat_ptr_const_int_int_int(const cv::Mat* instance, int i0, int i1, int i2, Result<const unsigned char*>* ocvrs_return) {
		try {
			const unsigned char* ret = instance->ptr(i0, i1, i2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ptr(const int *)(VariableArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1911
	// ("cv::Mat::ptr", vec![(pred!(mut, ["idx"], ["const int*"]), _)]),
	void cv_Mat_ptr_const_intX(cv::Mat* instance, const int* idx, Result<unsigned char*>* ocvrs_return) {
		try {
			unsigned char* ret = instance->ptr(idx);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ptr(const int *)(VariableArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:1913
	// ("cv::Mat::ptr", vec![(pred!(const, ["idx"], ["const int*"]), _)]),
	void cv_Mat_ptr_const_const_intX(const cv::Mat* instance, const int* idx, Result<const unsigned char*>* ocvrs_return) {
		try {
			const unsigned char* ret = instance->ptr(idx);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(Mat &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2137
	// ("cv::Mat::operator=", vec![(pred!(mut, ["m"], ["cv::Mat*"]), _)]),
	void cv_Mat_operatorST_MatRR(cv::Mat* instance, cv::Mat* m, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(std::move(*m));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// updateContinuityFlag()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2170
	// ("cv::Mat::updateContinuityFlag", vec![(pred!(mut, [], []), _)]),
	void cv_Mat_updateContinuityFlag(cv::Mat* instance, ResultVoid* ocvrs_return) {
		try {
			instance->updateContinuityFlag();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::size() generated
	// ("cv::Mat::size", vec![(pred!(const, [], []), _)]),
	void cv_Mat_size_const(const cv::Mat* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->size();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::getDataDump() generated
	// ("cv::Mat::getDataDump", vec![(pred!(const, [], []), _)]),
	void cv_Mat_getDataDump_const(const cv::Mat* instance, Result<void*>* ocvrs_return) {
		try {
			std::string ret = std::string();
			std::ostringstream oss;
			oss << *instance;
			ret = oss.str();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mat::flags() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2149
	// ("cv::Mat::flags", vec![(pred!(const, [], []), _)]),
	int cv_Mat_propFlags_const(const cv::Mat* instance) {
			int ret = instance->flags;
			return ret;
	}

	// cv::Mat::setFlags(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2149
	// ("cv::Mat::setFlags", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_Mat_propFlags_const_int(cv::Mat* instance, const int val) {
			instance->flags = val;
	}

	// cv::Mat::dims() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2151
	// ("cv::Mat::dims", vec![(pred!(const, [], []), _)]),
	int cv_Mat_propDims_const(const cv::Mat* instance) {
			int ret = instance->dims;
			return ret;
	}

	// cv::Mat::setDims(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2151
	// ("cv::Mat::setDims", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_Mat_propDims_const_int(cv::Mat* instance, const int val) {
			instance->dims = val;
	}

	// cv::Mat::rows() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2153
	// ("cv::Mat::rows", vec![(pred!(const, [], []), _)]),
	int cv_Mat_propRows_const(const cv::Mat* instance) {
			int ret = instance->rows;
			return ret;
	}

	// cv::Mat::setRows(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2153
	// ("cv::Mat::setRows", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_Mat_propRows_const_int(cv::Mat* instance, const int val) {
			instance->rows = val;
	}

	// cv::Mat::cols() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2153
	// ("cv::Mat::cols", vec![(pred!(const, [], []), _)]),
	int cv_Mat_propCols_const(const cv::Mat* instance) {
			int ret = instance->cols;
			return ret;
	}

	// cv::Mat::setCols(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2153
	// ("cv::Mat::setCols", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_Mat_propCols_const_int(cv::Mat* instance, const int val) {
			instance->cols = val;
	}

	// cv::Mat::data() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2155
	// ("cv::Mat::data", vec![(pred!(const, [], []), _)]),
	unsigned char* cv_Mat_propData_const(const cv::Mat* instance) {
			unsigned char* const ret = instance->data;
			return ret;
	}

	// cv::Mat::dataMut() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2155
	// ("cv::Mat::dataMut", vec![(pred!(mut, [], []), _)]),
	unsigned char* cv_Mat_propData(cv::Mat* instance) {
			unsigned char* ret = instance->data;
			return ret;
	}

	// cv::Mat::setData(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2155
	// ("cv::Mat::setData", vec![(pred!(mut, ["val"], ["unsigned char*"]), _)]),
	void cv_Mat_propData_unsigned_charX(cv::Mat* instance, unsigned char* const val) {
			instance->data = val;
	}

	// cv::Mat::datastart() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2158
	// ("cv::Mat::datastart", vec![(pred!(const, [], []), _)]),
	const unsigned char* cv_Mat_propDatastart_const(const cv::Mat* instance) {
			const unsigned char* ret = instance->datastart;
			return ret;
	}

	// cv::Mat::dataend() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2159
	// ("cv::Mat::dataend", vec![(pred!(const, [], []), _)]),
	const unsigned char* cv_Mat_propDataend_const(const cv::Mat* instance) {
			const unsigned char* ret = instance->dataend;
			return ret;
	}

	// cv::Mat::datalimit() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2160
	// ("cv::Mat::datalimit", vec![(pred!(const, [], []), _)]),
	const unsigned char* cv_Mat_propDatalimit_const(const cv::Mat* instance) {
			const unsigned char* ret = instance->datalimit;
			return ret;
	}

	// cv::Mat::u() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2173
	// ("cv::Mat::u", vec![(pred!(mut, [], []), _)]),
	cv::UMatData* cv_Mat_propU(cv::Mat* instance) {
			cv::UMatData* ret = instance->u;
			return new cv::UMatData(*ret);
	}

	// cv::Mat::setU(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2173
	// ("cv::Mat::setU", vec![(pred!(mut, ["val"], ["cv::UMatData*"]), _)]),
	void cv_Mat_propU_UMatDataX(cv::Mat* instance, cv::UMatData* const val) {
			instance->u = val;
	}

	// cv::Mat::size() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2175
	// ("cv::Mat::size", vec![(pred!(const, [], []), _)]),
	cv::MatSize* cv_Mat_propSize_const(const cv::Mat* instance) {
			cv::MatSize ret = instance->size;
			return new cv::MatSize(ret);
	}

	// cv::Mat::setSize(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2175
	// ("cv::Mat::setSize", vec![(pred!(mut, ["val"], ["const cv::MatSize"]), _)]),
	void cv_Mat_propSize_const_MatSize(cv::Mat* instance, const cv::MatSize* val) {
			instance->size = *val;
	}

	// cv::Mat::step() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2176
	// ("cv::Mat::step", vec![(pred!(const, [], []), _)]),
	cv::MatStep* cv_Mat_propStep_const(const cv::Mat* instance) {
			cv::MatStep ret = instance->step;
			return new cv::MatStep(ret);
	}

	// cv::Mat::delete() generated
	// ("cv::Mat::delete", vec![(pred!(mut, [], []), _)]),
	void cv_Mat_delete(cv::Mat* instance) {
			delete instance;
	}

	// MatConstIterator()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3093
	// ("cv::MatConstIterator::MatConstIterator", vec![(pred!(mut, [], []), _)]),
	void cv_MatConstIterator_MatConstIterator(Result<cv::MatConstIterator*>* ocvrs_return) {
		try {
			cv::MatConstIterator* ret = new cv::MatConstIterator();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// MatConstIterator(const Mat *)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3095
	// ("cv::MatConstIterator::MatConstIterator", vec![(pred!(mut, ["_m"], ["const cv::Mat*"]), _)]),
	void cv_MatConstIterator_MatConstIterator_const_MatX(const cv::Mat* _m, Result<cv::MatConstIterator*>* ocvrs_return) {
		try {
			cv::MatConstIterator* ret = new cv::MatConstIterator(_m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// MatConstIterator(const Mat *, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3097
	// ("cv::MatConstIterator::MatConstIterator", vec![(pred!(mut, ["_m", "_row", "_col"], ["const cv::Mat*", "int", "int"]), _)]),
	void cv_MatConstIterator_MatConstIterator_const_MatX_int_int(const cv::Mat* _m, int _row, int _col, Result<cv::MatConstIterator*>* ocvrs_return) {
		try {
			cv::MatConstIterator* ret = new cv::MatConstIterator(_m, _row, _col);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::MatConstIterator::MatConstIterator(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3097
	// ("cv::MatConstIterator::MatConstIterator", vec![(pred!(mut, ["_m", "_row"], ["const cv::Mat*", "int"]), _)]),
	void cv_MatConstIterator_MatConstIterator_const_MatX_int(const cv::Mat* _m, int _row, Result<cv::MatConstIterator*>* ocvrs_return) {
		try {
			cv::MatConstIterator* ret = new cv::MatConstIterator(_m, _row);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// MatConstIterator(const Mat *, Point)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3099
	// ("cv::MatConstIterator::MatConstIterator", vec![(pred!(mut, ["_m", "_pt"], ["const cv::Mat*", "cv::Point"]), _)]),
	void cv_MatConstIterator_MatConstIterator_const_MatX_Point(const cv::Mat* _m, cv::Point* _pt, Result<cv::MatConstIterator*>* ocvrs_return) {
		try {
			cv::MatConstIterator* ret = new cv::MatConstIterator(_m, *_pt);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// MatConstIterator(const MatConstIterator &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3103
	// ("cv::MatConstIterator::MatConstIterator", vec![(pred!(mut, ["it"], ["const cv::MatConstIterator*"]), _)]),
	void cv_MatConstIterator_MatConstIterator_const_MatConstIteratorR(const cv::MatConstIterator* it, Result<cv::MatConstIterator*>* ocvrs_return) {
		try {
			cv::MatConstIterator* ret = new cv::MatConstIterator(*it);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const MatConstIterator &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3106
	// ("cv::MatConstIterator::operator=", vec![(pred!(mut, ["it"], ["const cv::MatConstIterator*"]), _)]),
	void cv_MatConstIterator_operatorST_const_MatConstIteratorR(cv::MatConstIterator* instance, const cv::MatConstIterator* it, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*it);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator*()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3108
	// ("cv::MatConstIterator::operator*", vec![(pred!(const, [], []), _)]),
	void cv_MatConstIterator_operatorX_const(const cv::MatConstIterator* instance, Result<const unsigned char*>* ocvrs_return) {
		try {
			const unsigned char* ret = instance->operator*();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator[](ptrdiff_t)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3110
	// ("cv::MatConstIterator::operator[]", vec![(pred!(const, ["i"], ["ptrdiff_t"]), _)]),
	void cv_MatConstIterator_operator___const_ptrdiff_t(const cv::MatConstIterator* instance, ptrdiff_t i, Result<const unsigned char*>* ocvrs_return) {
		try {
			const unsigned char* ret = instance->operator[](i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator--()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3117
	// ("cv::MatConstIterator::operator--", vec![(pred!(mut, [], []), _)]),
	void cv_MatConstIterator_operatorSS(cv::MatConstIterator* instance, Result<cv::MatConstIterator*>* ocvrs_return) {
		try {
			cv::MatConstIterator ret = instance->operator--();
			Ok(new cv::MatConstIterator(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator++()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3121
	// ("cv::MatConstIterator::operator++", vec![(pred!(mut, [], []), _)]),
	void cv_MatConstIterator_operatorAA(cv::MatConstIterator* instance, Result<cv::MatConstIterator*>* ocvrs_return) {
		try {
			cv::MatConstIterator ret = instance->operator++();
			Ok(new cv::MatConstIterator(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// pos()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3125
	// ("cv::MatConstIterator::pos", vec![(pred!(const, [], []), _)]),
	void cv_MatConstIterator_pos_const(const cv::MatConstIterator* instance, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->pos();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// pos(int *)(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3127
	// ("cv::MatConstIterator::pos", vec![(pred!(const, ["_idx"], ["int*"]), _)]),
	void cv_MatConstIterator_pos_const_intX(const cv::MatConstIterator* instance, int* _idx, ResultVoid* ocvrs_return) {
		try {
			instance->pos(_idx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// lpos()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3129
	// ("cv::MatConstIterator::lpos", vec![(pred!(const, [], []), _)]),
	void cv_MatConstIterator_lpos_const(const cv::MatConstIterator* instance, Result<ptrdiff_t>* ocvrs_return) {
		try {
			ptrdiff_t ret = instance->lpos();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// seek(ptrdiff_t, bool)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3130
	// ("cv::MatConstIterator::seek", vec![(pred!(mut, ["ofs", "relative"], ["ptrdiff_t", "bool"]), _)]),
	void cv_MatConstIterator_seek_ptrdiff_t_bool(cv::MatConstIterator* instance, ptrdiff_t ofs, bool relative, ResultVoid* ocvrs_return) {
		try {
			instance->seek(ofs, relative);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::MatConstIterator::seek(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3130
	// ("cv::MatConstIterator::seek", vec![(pred!(mut, ["ofs"], ["ptrdiff_t"]), _)]),
	void cv_MatConstIterator_seek_ptrdiff_t(cv::MatConstIterator* instance, ptrdiff_t ofs, ResultVoid* ocvrs_return) {
		try {
			instance->seek(ofs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// seek(const int *, bool)(Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3131
	// ("cv::MatConstIterator::seek", vec![(pred!(mut, ["_idx", "relative"], ["const int*", "bool"]), _)]),
	void cv_MatConstIterator_seek_const_intX_bool(cv::MatConstIterator* instance, const int* _idx, bool relative, ResultVoid* ocvrs_return) {
		try {
			instance->seek(_idx, relative);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::MatConstIterator::seek(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3131
	// ("cv::MatConstIterator::seek", vec![(pred!(mut, ["_idx"], ["const int*"]), _)]),
	void cv_MatConstIterator_seek_const_intX(cv::MatConstIterator* instance, const int* _idx, ResultVoid* ocvrs_return) {
		try {
			instance->seek(_idx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::MatConstIterator::type() generated
	// ("cv::MatConstIterator::type", vec![(pred!(const, [], []), _)]),
	int cv_MatConstIterator_type_const(const cv::MatConstIterator* instance) {
			int ret = instance->m->type();
			return ret;
	}

	// cv::MatConstIterator::m() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3133
	// ("cv::MatConstIterator::m", vec![(pred!(const, [], []), _)]),
	const cv::Mat* cv_MatConstIterator_propM_const(const cv::MatConstIterator* instance) {
			const cv::Mat* ret = instance->m;
			return new const cv::Mat(*ret);
	}

	// cv::MatConstIterator::elemSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3134
	// ("cv::MatConstIterator::elemSize", vec![(pred!(const, [], []), _)]),
	size_t cv_MatConstIterator_propElemSize_const(const cv::MatConstIterator* instance) {
			size_t ret = instance->elemSize;
			return ret;
	}

	// cv::MatConstIterator::setElemSize(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3134
	// ("cv::MatConstIterator::setElemSize", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
	void cv_MatConstIterator_propElemSize_const_size_t(cv::MatConstIterator* instance, const size_t val) {
			instance->elemSize = val;
	}

	// cv::MatConstIterator::ptr() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3135
	// ("cv::MatConstIterator::ptr", vec![(pred!(const, [], []), _)]),
	const unsigned char* cv_MatConstIterator_propPtr_const(const cv::MatConstIterator* instance) {
			const unsigned char* ret = instance->ptr;
			return ret;
	}

	// cv::MatConstIterator::sliceStart() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3136
	// ("cv::MatConstIterator::sliceStart", vec![(pred!(const, [], []), _)]),
	const unsigned char* cv_MatConstIterator_propSliceStart_const(const cv::MatConstIterator* instance) {
			const unsigned char* ret = instance->sliceStart;
			return ret;
	}

	// cv::MatConstIterator::sliceEnd() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3137
	// ("cv::MatConstIterator::sliceEnd", vec![(pred!(const, [], []), _)]),
	const unsigned char* cv_MatConstIterator_propSliceEnd_const(const cv::MatConstIterator* instance) {
			const unsigned char* ret = instance->sliceEnd;
			return ret;
	}

	// cv::MatConstIterator::delete() generated
	// ("cv::MatConstIterator::delete", vec![(pred!(mut, [], []), _)]),
	void cv_MatConstIterator_delete(cv::MatConstIterator* instance) {
			delete instance;
	}

	// MatExpr()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3587
	// ("cv::MatExpr::MatExpr", vec![(pred!(mut, [], []), _)]),
	void cv_MatExpr_MatExpr(Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr* ret = new cv::MatExpr();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// MatExpr(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3588
	// ("cv::MatExpr::MatExpr", vec![(pred!(mut, ["m"], ["const cv::Mat*"]), _)]),
	void cv_MatExpr_MatExpr_const_MatR(const cv::Mat* m, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr* ret = new cv::MatExpr(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// MatExpr(const MatOp *, int, const Mat &, const Mat &, const Mat &, double, double, const Scalar &)(TraitClass, Primitive, TraitClass, TraitClass, TraitClass, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3590
	// ("cv::MatExpr::MatExpr", vec![(pred!(mut, ["_op", "_flags", "_a", "_b", "_c", "_alpha", "_beta", "_s"], ["const cv::MatOp*", "int", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "double", "double", "const cv::Scalar*"]), _)]),
	void cv_MatExpr_MatExpr_const_MatOpX_int_const_MatR_const_MatR_const_MatR_double_double_const_ScalarR(const cv::MatOp* _op, int _flags, const cv::Mat* _a, const cv::Mat* _b, const cv::Mat* _c, double _alpha, double _beta, const cv::Scalar* _s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr* ret = new cv::MatExpr(_op, _flags, *_a, *_b, *_c, _alpha, _beta, *_s);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::MatExpr::MatExpr(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3590
	// ("cv::MatExpr::MatExpr", vec![(pred!(mut, ["_op", "_flags"], ["const cv::MatOp*", "int"]), _)]),
	void cv_MatExpr_MatExpr_const_MatOpX_int(const cv::MatOp* _op, int _flags, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr* ret = new cv::MatExpr(_op, _flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator Mat()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3593
	// ("cv::MatExpr::operator cv::Mat", vec![(pred!(const, [], []), _)]),
	void cv_MatExpr_operator_cv_Mat_const(const cv::MatExpr* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->operator cv::Mat();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// size()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3596
	// ("cv::MatExpr::size", vec![(pred!(const, [], []), _)]),
	void cv_MatExpr_size_const(const cv::MatExpr* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->size();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// type()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3597
	// ("cv::MatExpr::type", vec![(pred!(const, [], []), _)]),
	void cv_MatExpr_type_const(const cv::MatExpr* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->type();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// row(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3599
	// ("cv::MatExpr::row", vec![(pred!(const, ["y"], ["int"]), _)]),
	void cv_MatExpr_row_const_int(const cv::MatExpr* instance, int y, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = instance->row(y);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// col(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3600
	// ("cv::MatExpr::col", vec![(pred!(const, ["x"], ["int"]), _)]),
	void cv_MatExpr_col_const_int(const cv::MatExpr* instance, int x, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = instance->col(x);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// diag(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3601
	// ("cv::MatExpr::diag", vec![(pred!(const, ["d"], ["int"]), _)]),
	void cv_MatExpr_diag_const_int(const cv::MatExpr* instance, int d, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = instance->diag(d);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::MatExpr::diag() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3601
	// ("cv::MatExpr::diag", vec![(pred!(const, [], []), _)]),
	void cv_MatExpr_diag_const(const cv::MatExpr* instance, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = instance->diag();
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(const Range &, const Range &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3602
	// ("cv::MatExpr::operator()", vec![(pred!(const, ["rowRange", "colRange"], ["const cv::Range*", "const cv::Range*"]), _)]),
	void cv_MatExpr_operator___const_const_RangeR_const_RangeR(const cv::MatExpr* instance, const cv::Range* rowRange, const cv::Range* colRange, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = instance->operator()(*rowRange, *colRange);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(const Rect &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3603
	// ("cv::MatExpr::operator()", vec![(pred!(const, ["roi"], ["const cv::Rect*"]), _)]),
	void cv_MatExpr_operator___const_const_RectR(const cv::MatExpr* instance, const cv::Rect* roi, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = instance->operator()(*roi);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// t()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3605
	// ("cv::MatExpr::t", vec![(pred!(const, [], []), _)]),
	void cv_MatExpr_t_const(const cv::MatExpr* instance, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = instance->t();
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// inv(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3606
	// ("cv::MatExpr::inv", vec![(pred!(const, ["method"], ["int"]), _)]),
	void cv_MatExpr_inv_const_int(const cv::MatExpr* instance, int method, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = instance->inv(method);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::MatExpr::inv() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3606
	// ("cv::MatExpr::inv", vec![(pred!(const, [], []), _)]),
	void cv_MatExpr_inv_const(const cv::MatExpr* instance, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = instance->inv();
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mul(const MatExpr &, double)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3607
	// ("cv::MatExpr::mul", vec![(pred!(const, ["e", "scale"], ["const cv::MatExpr*", "double"]), _)]),
	void cv_MatExpr_mul_const_const_MatExprR_double(const cv::MatExpr* instance, const cv::MatExpr* e, double scale, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = instance->mul(*e, scale);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::MatExpr::mul(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3607
	// ("cv::MatExpr::mul", vec![(pred!(const, ["e"], ["const cv::MatExpr*"]), _)]),
	void cv_MatExpr_mul_const_const_MatExprR(const cv::MatExpr* instance, const cv::MatExpr* e, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = instance->mul(*e);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mul(const Mat &, double)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3608
	// ("cv::MatExpr::mul", vec![(pred!(const, ["m", "scale"], ["const cv::Mat*", "double"]), _)]),
	void cv_MatExpr_mul_const_const_MatR_double(const cv::MatExpr* instance, const cv::Mat* m, double scale, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = instance->mul(*m, scale);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::MatExpr::mul(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3608
	// ("cv::MatExpr::mul", vec![(pred!(const, ["m"], ["const cv::Mat*"]), _)]),
	void cv_MatExpr_mul_const_const_MatR(const cv::MatExpr* instance, const cv::Mat* m, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = instance->mul(*m);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cross(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3610
	// ("cv::MatExpr::cross", vec![(pred!(const, ["m"], ["const cv::Mat*"]), _)]),
	void cv_MatExpr_cross_const_const_MatR(const cv::MatExpr* instance, const cv::Mat* m, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->cross(*m);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dot(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3611
	// ("cv::MatExpr::dot", vec![(pred!(const, ["m"], ["const cv::Mat*"]), _)]),
	void cv_MatExpr_dot_const_const_MatR(const cv::MatExpr* instance, const cv::Mat* m, Result<double>* ocvrs_return) {
		try {
			double ret = instance->dot(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// swap(MatExpr &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3613
	// ("cv::MatExpr::swap", vec![(pred!(mut, ["b"], ["cv::MatExpr*"]), _)]),
	void cv_MatExpr_swap_MatExprR(cv::MatExpr* instance, cv::MatExpr* b, ResultVoid* ocvrs_return) {
		try {
			instance->swap(*b);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::MatExpr::flags() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3616
	// ("cv::MatExpr::flags", vec![(pred!(const, [], []), _)]),
	int cv_MatExpr_propFlags_const(const cv::MatExpr* instance) {
			int ret = instance->flags;
			return ret;
	}

	// cv::MatExpr::setFlags(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3616
	// ("cv::MatExpr::setFlags", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_MatExpr_propFlags_const_int(cv::MatExpr* instance, const int val) {
			instance->flags = val;
	}

	// cv::MatExpr::a() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3618
	// ("cv::MatExpr::a", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_MatExpr_propA_const(const cv::MatExpr* instance) {
			cv::Mat ret = instance->a;
			return new cv::Mat(ret);
	}

	// cv::MatExpr::setA(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3618
	// ("cv::MatExpr::setA", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_MatExpr_propA_const_Mat(cv::MatExpr* instance, const cv::Mat* val) {
			instance->a = *val;
	}

	// cv::MatExpr::b() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3618
	// ("cv::MatExpr::b", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_MatExpr_propB_const(const cv::MatExpr* instance) {
			cv::Mat ret = instance->b;
			return new cv::Mat(ret);
	}

	// cv::MatExpr::setB(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3618
	// ("cv::MatExpr::setB", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_MatExpr_propB_const_Mat(cv::MatExpr* instance, const cv::Mat* val) {
			instance->b = *val;
	}

	// cv::MatExpr::c() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3618
	// ("cv::MatExpr::c", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_MatExpr_propC_const(const cv::MatExpr* instance) {
			cv::Mat ret = instance->c;
			return new cv::Mat(ret);
	}

	// cv::MatExpr::setC(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3618
	// ("cv::MatExpr::setC", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_MatExpr_propC_const_Mat(cv::MatExpr* instance, const cv::Mat* val) {
			instance->c = *val;
	}

	// cv::MatExpr::alpha() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3619
	// ("cv::MatExpr::alpha", vec![(pred!(const, [], []), _)]),
	double cv_MatExpr_propAlpha_const(const cv::MatExpr* instance) {
			double ret = instance->alpha;
			return ret;
	}

	// cv::MatExpr::setAlpha(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3619
	// ("cv::MatExpr::setAlpha", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_MatExpr_propAlpha_const_double(cv::MatExpr* instance, const double val) {
			instance->alpha = val;
	}

	// cv::MatExpr::beta() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3619
	// ("cv::MatExpr::beta", vec![(pred!(const, [], []), _)]),
	double cv_MatExpr_propBeta_const(const cv::MatExpr* instance) {
			double ret = instance->beta;
			return ret;
	}

	// cv::MatExpr::setBeta(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3619
	// ("cv::MatExpr::setBeta", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_MatExpr_propBeta_const_double(cv::MatExpr* instance, const double val) {
			instance->beta = val;
	}

	// cv::MatExpr::s() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3620
	// ("cv::MatExpr::s", vec![(pred!(const, [], []), _)]),
	void cv_MatExpr_propS_const(const cv::MatExpr* instance, cv::Scalar* ocvrs_return) {
			cv::Scalar ret = instance->s;
			*ocvrs_return = ret;
	}

	// cv::MatExpr::setS(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3620
	// ("cv::MatExpr::setS", vec![(pred!(mut, ["val"], ["const cv::Scalar"]), _)]),
	void cv_MatExpr_propS_const_Scalar(cv::MatExpr* instance, const cv::Scalar* val) {
			instance->s = *val;
	}

	// cv::MatExpr::delete() generated
	// ("cv::MatExpr::delete", vec![(pred!(mut, [], []), _)]),
	void cv_MatExpr_delete(cv::MatExpr* instance) {
			delete instance;
	}

	// elementWise(const MatExpr &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3503
	// ("cv::MatOp::elementWise", vec![(pred!(const, ["expr"], ["const cv::MatExpr*"]), _)]),
	void cv_MatOp_elementWise_const_const_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->elementWise(*expr);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// assign(const MatExpr &, Mat &, int)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3504
	// ("cv::MatOp::assign", vec![(pred!(const, ["expr", "m", "type"], ["const cv::MatExpr*", "cv::Mat*", "int"]), _)]),
	void cv_MatOp_assign_const_const_MatExprR_MatR_int(const cv::MatOp* instance, const cv::MatExpr* expr, cv::Mat* m, int type, ResultVoid* ocvrs_return) {
		try {
			instance->assign(*expr, *m, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::MatOp::assign(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3504
	// ("cv::MatOp::assign", vec![(pred!(const, ["expr", "m"], ["const cv::MatExpr*", "cv::Mat*"]), _)]),
	void cv_MatOp_assign_const_const_MatExprR_MatR(const cv::MatOp* instance, const cv::MatExpr* expr, cv::Mat* m, ResultVoid* ocvrs_return) {
		try {
			instance->assign(*expr, *m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// roi(const MatExpr &, const Range &, const Range &, MatExpr &)(TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3505
	// ("cv::MatOp::roi", vec![(pred!(const, ["expr", "rowRange", "colRange", "res"], ["const cv::MatExpr*", "const cv::Range*", "const cv::Range*", "cv::MatExpr*"]), _)]),
	void cv_MatOp_roi_const_const_MatExprR_const_RangeR_const_RangeR_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr, const cv::Range* rowRange, const cv::Range* colRange, cv::MatExpr* res, ResultVoid* ocvrs_return) {
		try {
			instance->roi(*expr, *rowRange, *colRange, *res);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// diag(const MatExpr &, int, MatExpr &)(TraitClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3507
	// ("cv::MatOp::diag", vec![(pred!(const, ["expr", "d", "res"], ["const cv::MatExpr*", "int", "cv::MatExpr*"]), _)]),
	void cv_MatOp_diag_const_const_MatExprR_int_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr, int d, cv::MatExpr* res, ResultVoid* ocvrs_return) {
		try {
			instance->diag(*expr, d, *res);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// augAssignAdd(const MatExpr &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3508
	// ("cv::MatOp::augAssignAdd", vec![(pred!(const, ["expr", "m"], ["const cv::MatExpr*", "cv::Mat*"]), _)]),
	void cv_MatOp_augAssignAdd_const_const_MatExprR_MatR(const cv::MatOp* instance, const cv::MatExpr* expr, cv::Mat* m, ResultVoid* ocvrs_return) {
		try {
			instance->augAssignAdd(*expr, *m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// augAssignSubtract(const MatExpr &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3509
	// ("cv::MatOp::augAssignSubtract", vec![(pred!(const, ["expr", "m"], ["const cv::MatExpr*", "cv::Mat*"]), _)]),
	void cv_MatOp_augAssignSubtract_const_const_MatExprR_MatR(const cv::MatOp* instance, const cv::MatExpr* expr, cv::Mat* m, ResultVoid* ocvrs_return) {
		try {
			instance->augAssignSubtract(*expr, *m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// augAssignMultiply(const MatExpr &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3510
	// ("cv::MatOp::augAssignMultiply", vec![(pred!(const, ["expr", "m"], ["const cv::MatExpr*", "cv::Mat*"]), _)]),
	void cv_MatOp_augAssignMultiply_const_const_MatExprR_MatR(const cv::MatOp* instance, const cv::MatExpr* expr, cv::Mat* m, ResultVoid* ocvrs_return) {
		try {
			instance->augAssignMultiply(*expr, *m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// augAssignDivide(const MatExpr &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3511
	// ("cv::MatOp::augAssignDivide", vec![(pred!(const, ["expr", "m"], ["const cv::MatExpr*", "cv::Mat*"]), _)]),
	void cv_MatOp_augAssignDivide_const_const_MatExprR_MatR(const cv::MatOp* instance, const cv::MatExpr* expr, cv::Mat* m, ResultVoid* ocvrs_return) {
		try {
			instance->augAssignDivide(*expr, *m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// augAssignAnd(const MatExpr &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3512
	// ("cv::MatOp::augAssignAnd", vec![(pred!(const, ["expr", "m"], ["const cv::MatExpr*", "cv::Mat*"]), _)]),
	void cv_MatOp_augAssignAnd_const_const_MatExprR_MatR(const cv::MatOp* instance, const cv::MatExpr* expr, cv::Mat* m, ResultVoid* ocvrs_return) {
		try {
			instance->augAssignAnd(*expr, *m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// augAssignOr(const MatExpr &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3513
	// ("cv::MatOp::augAssignOr", vec![(pred!(const, ["expr", "m"], ["const cv::MatExpr*", "cv::Mat*"]), _)]),
	void cv_MatOp_augAssignOr_const_const_MatExprR_MatR(const cv::MatOp* instance, const cv::MatExpr* expr, cv::Mat* m, ResultVoid* ocvrs_return) {
		try {
			instance->augAssignOr(*expr, *m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// augAssignXor(const MatExpr &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3514
	// ("cv::MatOp::augAssignXor", vec![(pred!(const, ["expr", "m"], ["const cv::MatExpr*", "cv::Mat*"]), _)]),
	void cv_MatOp_augAssignXor_const_const_MatExprR_MatR(const cv::MatOp* instance, const cv::MatExpr* expr, cv::Mat* m, ResultVoid* ocvrs_return) {
		try {
			instance->augAssignXor(*expr, *m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// add(const MatExpr &, const MatExpr &, MatExpr &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3516
	// ("cv::MatOp::add", vec![(pred!(const, ["expr1", "expr2", "res"], ["const cv::MatExpr*", "const cv::MatExpr*", "cv::MatExpr*"]), _)]),
	void cv_MatOp_add_const_const_MatExprR_const_MatExprR_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr1, const cv::MatExpr* expr2, cv::MatExpr* res, ResultVoid* ocvrs_return) {
		try {
			instance->add(*expr1, *expr2, *res);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// add(const MatExpr &, const Scalar &, MatExpr &)(TraitClass, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3517
	// ("cv::MatOp::add", vec![(pred!(const, ["expr1", "s", "res"], ["const cv::MatExpr*", "const cv::Scalar*", "cv::MatExpr*"]), _)]),
	void cv_MatOp_add_const_const_MatExprR_const_ScalarR_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr1, const cv::Scalar* s, cv::MatExpr* res, ResultVoid* ocvrs_return) {
		try {
			instance->add(*expr1, *s, *res);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// subtract(const MatExpr &, const MatExpr &, MatExpr &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3519
	// ("cv::MatOp::subtract", vec![(pred!(const, ["expr1", "expr2", "res"], ["const cv::MatExpr*", "const cv::MatExpr*", "cv::MatExpr*"]), _)]),
	void cv_MatOp_subtract_const_const_MatExprR_const_MatExprR_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr1, const cv::MatExpr* expr2, cv::MatExpr* res, ResultVoid* ocvrs_return) {
		try {
			instance->subtract(*expr1, *expr2, *res);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// subtract(const Scalar &, const MatExpr &, MatExpr &)(SimpleClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3520
	// ("cv::MatOp::subtract", vec![(pred!(const, ["s", "expr", "res"], ["const cv::Scalar*", "const cv::MatExpr*", "cv::MatExpr*"]), _)]),
	void cv_MatOp_subtract_const_const_ScalarR_const_MatExprR_MatExprR(const cv::MatOp* instance, const cv::Scalar* s, const cv::MatExpr* expr, cv::MatExpr* res, ResultVoid* ocvrs_return) {
		try {
			instance->subtract(*s, *expr, *res);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// multiply(const MatExpr &, const MatExpr &, MatExpr &, double)(TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3522
	// ("cv::MatOp::multiply", vec![(pred!(const, ["expr1", "expr2", "res", "scale"], ["const cv::MatExpr*", "const cv::MatExpr*", "cv::MatExpr*", "double"]), _)]),
	void cv_MatOp_multiply_const_const_MatExprR_const_MatExprR_MatExprR_double(const cv::MatOp* instance, const cv::MatExpr* expr1, const cv::MatExpr* expr2, cv::MatExpr* res, double scale, ResultVoid* ocvrs_return) {
		try {
			instance->multiply(*expr1, *expr2, *res, scale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::MatOp::multiply(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3522
	// ("cv::MatOp::multiply", vec![(pred!(const, ["expr1", "expr2", "res"], ["const cv::MatExpr*", "const cv::MatExpr*", "cv::MatExpr*"]), _)]),
	void cv_MatOp_multiply_const_const_MatExprR_const_MatExprR_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr1, const cv::MatExpr* expr2, cv::MatExpr* res, ResultVoid* ocvrs_return) {
		try {
			instance->multiply(*expr1, *expr2, *res);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// multiply(const MatExpr &, double, MatExpr &)(TraitClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3523
	// ("cv::MatOp::multiply", vec![(pred!(const, ["expr1", "s", "res"], ["const cv::MatExpr*", "double", "cv::MatExpr*"]), _)]),
	void cv_MatOp_multiply_const_const_MatExprR_double_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr1, double s, cv::MatExpr* res, ResultVoid* ocvrs_return) {
		try {
			instance->multiply(*expr1, s, *res);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// divide(const MatExpr &, const MatExpr &, MatExpr &, double)(TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3525
	// ("cv::MatOp::divide", vec![(pred!(const, ["expr1", "expr2", "res", "scale"], ["const cv::MatExpr*", "const cv::MatExpr*", "cv::MatExpr*", "double"]), _)]),
	void cv_MatOp_divide_const_const_MatExprR_const_MatExprR_MatExprR_double(const cv::MatOp* instance, const cv::MatExpr* expr1, const cv::MatExpr* expr2, cv::MatExpr* res, double scale, ResultVoid* ocvrs_return) {
		try {
			instance->divide(*expr1, *expr2, *res, scale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::MatOp::divide(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3525
	// ("cv::MatOp::divide", vec![(pred!(const, ["expr1", "expr2", "res"], ["const cv::MatExpr*", "const cv::MatExpr*", "cv::MatExpr*"]), _)]),
	void cv_MatOp_divide_const_const_MatExprR_const_MatExprR_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr1, const cv::MatExpr* expr2, cv::MatExpr* res, ResultVoid* ocvrs_return) {
		try {
			instance->divide(*expr1, *expr2, *res);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// divide(double, const MatExpr &, MatExpr &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3526
	// ("cv::MatOp::divide", vec![(pred!(const, ["s", "expr", "res"], ["double", "const cv::MatExpr*", "cv::MatExpr*"]), _)]),
	void cv_MatOp_divide_const_double_const_MatExprR_MatExprR(const cv::MatOp* instance, double s, const cv::MatExpr* expr, cv::MatExpr* res, ResultVoid* ocvrs_return) {
		try {
			instance->divide(s, *expr, *res);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// abs(const MatExpr &, MatExpr &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3528
	// ("cv::MatOp::abs", vec![(pred!(const, ["expr", "res"], ["const cv::MatExpr*", "cv::MatExpr*"]), _)]),
	void cv_MatOp_abs_const_const_MatExprR_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr, cv::MatExpr* res, ResultVoid* ocvrs_return) {
		try {
			instance->abs(*expr, *res);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// transpose(const MatExpr &, MatExpr &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3530
	// ("cv::MatOp::transpose", vec![(pred!(const, ["expr", "res"], ["const cv::MatExpr*", "cv::MatExpr*"]), _)]),
	void cv_MatOp_transpose_const_const_MatExprR_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr, cv::MatExpr* res, ResultVoid* ocvrs_return) {
		try {
			instance->transpose(*expr, *res);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// matmul(const MatExpr &, const MatExpr &, MatExpr &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3531
	// ("cv::MatOp::matmul", vec![(pred!(const, ["expr1", "expr2", "res"], ["const cv::MatExpr*", "const cv::MatExpr*", "cv::MatExpr*"]), _)]),
	void cv_MatOp_matmul_const_const_MatExprR_const_MatExprR_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr1, const cv::MatExpr* expr2, cv::MatExpr* res, ResultVoid* ocvrs_return) {
		try {
			instance->matmul(*expr1, *expr2, *res);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// invert(const MatExpr &, int, MatExpr &)(TraitClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3532
	// ("cv::MatOp::invert", vec![(pred!(const, ["expr", "method", "res"], ["const cv::MatExpr*", "int", "cv::MatExpr*"]), _)]),
	void cv_MatOp_invert_const_const_MatExprR_int_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr, int method, cv::MatExpr* res, ResultVoid* ocvrs_return) {
		try {
			instance->invert(*expr, method, *res);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// size(const MatExpr &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3534
	// ("cv::MatOp::size", vec![(pred!(const, ["expr"], ["const cv::MatExpr*"]), _)]),
	void cv_MatOp_size_const_const_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->size(*expr);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// type(const MatExpr &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3535
	// ("cv::MatOp::type", vec![(pred!(const, ["expr"], ["const cv::MatExpr*"]), _)]),
	void cv_MatOp_type_const_const_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr, Result<int>* ocvrs_return) {
		try {
			int ret = instance->type(*expr);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::MatOp::delete() generated
	// ("cv::MatOp::delete", vec![(pred!(mut, [], []), _)]),
	void cv_MatOp_delete(cv::MatOp* instance) {
			delete instance;
	}

	// MatSize(int *)(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:595
	// ("cv::MatSize::MatSize", vec![(pred!(mut, ["_p"], ["int*"]), _)]),
	cv::MatSize* cv_MatSize_MatSize_intX(int* _p) {
			cv::MatSize* ret = new cv::MatSize(_p);
			return ret;
	}

	// dims()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:596
	// ("cv::MatSize::dims", vec![(pred!(const, [], []), _)]),
	int cv_MatSize_dims_const(const cv::MatSize* instance) {
			int ret = instance->dims();
			return ret;
	}

	// operator()()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:597
	// ("cv::MatSize::operator()", vec![(pred!(const, [], []), _)]),
	void cv_MatSize_operator___const(const cv::MatSize* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->operator()();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator[](int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:598
	// ("cv::MatSize::operator[]", vec![(pred!(const, ["i"], ["int"]), _)]),
	void cv_MatSize_operator___const_int(const cv::MatSize* instance, int i, Result<int>* ocvrs_return) {
		try {
			const int ret = instance->operator[](i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator[](int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:599
	// ("cv::MatSize::operator[]", vec![(pred!(mut, ["i"], ["int"]), _)]),
	void cv_MatSize_operator___int(cv::MatSize* instance, int i, Result<int>* ocvrs_return) {
		try {
			int ret = instance->operator[](i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator const int *()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:600
	// ("cv::MatSize::operator const int*", vec![(pred!(const, [], []), _)]),
	const int* cv_MatSize_operator_const_intX_const(const cv::MatSize* instance) {
			const int* ret = instance->operator const int*();
			return ret;
	}

	// operator==(const MatSize &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:601
	// ("cv::MatSize::operator==", vec![(pred!(const, ["sz"], ["const cv::MatSize*"]), _)]),
	bool cv_MatSize_operatorEQ_const_const_MatSizeR(const cv::MatSize* instance, const cv::MatSize* sz) {
			bool ret = instance->operator==(*sz);
			return ret;
	}

	// operator!=(const MatSize &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:602
	// ("cv::MatSize::operator!=", vec![(pred!(const, ["sz"], ["const cv::MatSize*"]), _)]),
	bool cv_MatSize_operatorNE_const_const_MatSizeR(const cv::MatSize* instance, const cv::MatSize* sz) {
			bool ret = instance->operator!=(*sz);
			return ret;
	}

	// cv::MatSize::p() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:604
	// ("cv::MatSize::p", vec![(pred!(const, [], []), _)]),
	int* cv_MatSize_propP_const(const cv::MatSize* instance) {
			int* const ret = instance->p;
			return ret;
	}

	// cv::MatSize::pMut() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:604
	// ("cv::MatSize::pMut", vec![(pred!(mut, [], []), _)]),
	int* cv_MatSize_propP(cv::MatSize* instance) {
			int* ret = instance->p;
			return ret;
	}

	// cv::MatSize::setP(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:604
	// ("cv::MatSize::setP", vec![(pred!(mut, ["val"], ["int*"]), _)]),
	void cv_MatSize_propP_intX(cv::MatSize* instance, int* const val) {
			instance->p = val;
	}

	// cv::MatSize::delete() generated
	// ("cv::MatSize::delete", vec![(pred!(mut, [], []), _)]),
	void cv_MatSize_delete(cv::MatSize* instance) {
			delete instance;
	}

	// MatStep()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:609
	// ("cv::MatStep::MatStep", vec![(pred!(mut, [], []), _)]),
	cv::MatStep* cv_MatStep_MatStep() {
			cv::MatStep* ret = new cv::MatStep();
			return ret;
	}

	// MatStep(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:610
	// ("cv::MatStep::MatStep", vec![(pred!(mut, ["s"], ["size_t"]), _)]),
	cv::MatStep* cv_MatStep_MatStep_size_t(size_t s) {
			cv::MatStep* ret = new cv::MatStep(s);
			return ret;
	}

	// operator[](int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:611
	// ("cv::MatStep::operator[]", vec![(pred!(const, ["i"], ["int"]), _)]),
	const size_t cv_MatStep_operator___const_int(const cv::MatStep* instance, int i) {
			const size_t ret = instance->operator[](i);
			return ret;
	}

	// operator[](int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:612
	// ("cv::MatStep::operator[]", vec![(pred!(mut, ["i"], ["int"]), _)]),
	size_t cv_MatStep_operator___int(cv::MatStep* instance, int i) {
			size_t ret = instance->operator[](i);
			return ret;
	}

	// operator unsigned long()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:613
	// ("cv::MatStep::operator size_t", vec![(pred!(const, [], []), _)]),
	void cv_MatStep_operator_size_t_const(const cv::MatStep* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->operator size_t();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:614
	// ("cv::MatStep::operator=", vec![(pred!(mut, ["s"], ["size_t"]), _)]),
	void cv_MatStep_operatorST_size_t(cv::MatStep* instance, size_t s, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(s);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::MatStep::p() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:616
	// ("cv::MatStep::p", vec![(pred!(const, [], []), _)]),
	size_t* cv_MatStep_propP_const(const cv::MatStep* instance) {
			size_t* const ret = instance->p;
			return ret;
	}

	// cv::MatStep::pMut() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:616
	// ("cv::MatStep::pMut", vec![(pred!(mut, [], []), _)]),
	size_t* cv_MatStep_propP(cv::MatStep* instance) {
			size_t* ret = instance->p;
			return ret;
	}

	// cv::MatStep::setP(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:616
	// ("cv::MatStep::setP", vec![(pred!(mut, ["val"], ["size_t*"]), _)]),
	void cv_MatStep_propP_size_tX(cv::MatStep* instance, size_t* const val) {
			instance->p = val;
	}

	// cv::MatStep::buf() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:617
	// ("cv::MatStep::buf", vec![(pred!(const, [], []), _)]),
	const size_t** cv_MatStep_propBuf_const(const cv::MatStep* instance) {
			const size_t(*ret)[2] = &instance->buf;
			return (const size_t**)ret;
	}

	// cv::MatStep::bufMut() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:617
	// ("cv::MatStep::bufMut", vec![(pred!(mut, [], []), _)]),
	size_t** cv_MatStep_propBuf(cv::MatStep* instance) {
			size_t(*ret)[2] = &instance->buf;
			return (size_t**)ret;
	}

	// cv::MatStep::delete() generated
	// ("cv::MatStep::delete", vec![(pred!(mut, [], []), _)]),
	void cv_MatStep_delete(cv::MatStep* instance) {
			delete instance;
	}

	// Matx_AddOp()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/matx.hpp:70
	// ("cv::Matx_AddOp::Matx_AddOp", vec![(pred!(mut, [], []), _)]),
	void cv_Matx_AddOp_Matx_AddOp(Result<cv::Matx_AddOp*>* ocvrs_return) {
		try {
			cv::Matx_AddOp* ret = new cv::Matx_AddOp();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Matx_AddOp(const Matx_AddOp &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/matx.hpp:70
	// ("cv::Matx_AddOp::Matx_AddOp", vec![(pred!(mut, ["unnamed"], ["const cv::Matx_AddOp*"]), _)]),
	void cv_Matx_AddOp_Matx_AddOp_const_Matx_AddOpR(const cv::Matx_AddOp* unnamed, Result<cv::Matx_AddOp*>* ocvrs_return) {
		try {
			cv::Matx_AddOp* ret = new cv::Matx_AddOp(*unnamed);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Matx_AddOp::delete() generated
	// ("cv::Matx_AddOp::delete", vec![(pred!(mut, [], []), _)]),
	void cv_Matx_AddOp_delete(cv::Matx_AddOp* instance) {
			delete instance;
	}

	// Matx_DivOp()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/matx.hpp:74
	// ("cv::Matx_DivOp::Matx_DivOp", vec![(pred!(mut, [], []), _)]),
	void cv_Matx_DivOp_Matx_DivOp(Result<cv::Matx_DivOp*>* ocvrs_return) {
		try {
			cv::Matx_DivOp* ret = new cv::Matx_DivOp();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Matx_DivOp(const Matx_DivOp &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/matx.hpp:74
	// ("cv::Matx_DivOp::Matx_DivOp", vec![(pred!(mut, ["unnamed"], ["const cv::Matx_DivOp*"]), _)]),
	void cv_Matx_DivOp_Matx_DivOp_const_Matx_DivOpR(const cv::Matx_DivOp* unnamed, Result<cv::Matx_DivOp*>* ocvrs_return) {
		try {
			cv::Matx_DivOp* ret = new cv::Matx_DivOp(*unnamed);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Matx_DivOp::delete() generated
	// ("cv::Matx_DivOp::delete", vec![(pred!(mut, [], []), _)]),
	void cv_Matx_DivOp_delete(cv::Matx_DivOp* instance) {
			delete instance;
	}

	// Matx_MatMulOp()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/matx.hpp:75
	// ("cv::Matx_MatMulOp::Matx_MatMulOp", vec![(pred!(mut, [], []), _)]),
	void cv_Matx_MatMulOp_Matx_MatMulOp(Result<cv::Matx_MatMulOp*>* ocvrs_return) {
		try {
			cv::Matx_MatMulOp* ret = new cv::Matx_MatMulOp();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Matx_MatMulOp(const Matx_MatMulOp &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/matx.hpp:75
	// ("cv::Matx_MatMulOp::Matx_MatMulOp", vec![(pred!(mut, ["unnamed"], ["const cv::Matx_MatMulOp*"]), _)]),
	void cv_Matx_MatMulOp_Matx_MatMulOp_const_Matx_MatMulOpR(const cv::Matx_MatMulOp* unnamed, Result<cv::Matx_MatMulOp*>* ocvrs_return) {
		try {
			cv::Matx_MatMulOp* ret = new cv::Matx_MatMulOp(*unnamed);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Matx_MatMulOp::delete() generated
	// ("cv::Matx_MatMulOp::delete", vec![(pred!(mut, [], []), _)]),
	void cv_Matx_MatMulOp_delete(cv::Matx_MatMulOp* instance) {
			delete instance;
	}

	// Matx_MulOp()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/matx.hpp:73
	// ("cv::Matx_MulOp::Matx_MulOp", vec![(pred!(mut, [], []), _)]),
	void cv_Matx_MulOp_Matx_MulOp(Result<cv::Matx_MulOp*>* ocvrs_return) {
		try {
			cv::Matx_MulOp* ret = new cv::Matx_MulOp();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Matx_MulOp(const Matx_MulOp &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/matx.hpp:73
	// ("cv::Matx_MulOp::Matx_MulOp", vec![(pred!(mut, ["unnamed"], ["const cv::Matx_MulOp*"]), _)]),
	void cv_Matx_MulOp_Matx_MulOp_const_Matx_MulOpR(const cv::Matx_MulOp* unnamed, Result<cv::Matx_MulOp*>* ocvrs_return) {
		try {
			cv::Matx_MulOp* ret = new cv::Matx_MulOp(*unnamed);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Matx_MulOp::delete() generated
	// ("cv::Matx_MulOp::delete", vec![(pred!(mut, [], []), _)]),
	void cv_Matx_MulOp_delete(cv::Matx_MulOp* instance) {
			delete instance;
	}

	// Matx_ScaleOp()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/matx.hpp:72
	// ("cv::Matx_ScaleOp::Matx_ScaleOp", vec![(pred!(mut, [], []), _)]),
	void cv_Matx_ScaleOp_Matx_ScaleOp(Result<cv::Matx_ScaleOp*>* ocvrs_return) {
		try {
			cv::Matx_ScaleOp* ret = new cv::Matx_ScaleOp();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Matx_ScaleOp(const Matx_ScaleOp &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/matx.hpp:72
	// ("cv::Matx_ScaleOp::Matx_ScaleOp", vec![(pred!(mut, ["unnamed"], ["const cv::Matx_ScaleOp*"]), _)]),
	void cv_Matx_ScaleOp_Matx_ScaleOp_const_Matx_ScaleOpR(const cv::Matx_ScaleOp* unnamed, Result<cv::Matx_ScaleOp*>* ocvrs_return) {
		try {
			cv::Matx_ScaleOp* ret = new cv::Matx_ScaleOp(*unnamed);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Matx_ScaleOp::delete() generated
	// ("cv::Matx_ScaleOp::delete", vec![(pred!(mut, [], []), _)]),
	void cv_Matx_ScaleOp_delete(cv::Matx_ScaleOp* instance) {
			delete instance;
	}

	// Matx_SubOp()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/matx.hpp:71
	// ("cv::Matx_SubOp::Matx_SubOp", vec![(pred!(mut, [], []), _)]),
	void cv_Matx_SubOp_Matx_SubOp(Result<cv::Matx_SubOp*>* ocvrs_return) {
		try {
			cv::Matx_SubOp* ret = new cv::Matx_SubOp();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Matx_SubOp(const Matx_SubOp &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/matx.hpp:71
	// ("cv::Matx_SubOp::Matx_SubOp", vec![(pred!(mut, ["unnamed"], ["const cv::Matx_SubOp*"]), _)]),
	void cv_Matx_SubOp_Matx_SubOp_const_Matx_SubOpR(const cv::Matx_SubOp* unnamed, Result<cv::Matx_SubOp*>* ocvrs_return) {
		try {
			cv::Matx_SubOp* ret = new cv::Matx_SubOp(*unnamed);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Matx_SubOp::delete() generated
	// ("cv::Matx_SubOp::delete", vec![(pred!(mut, [], []), _)]),
	void cv_Matx_SubOp_delete(cv::Matx_SubOp* instance) {
			delete instance;
	}

	// Matx_TOp()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/matx.hpp:76
	// ("cv::Matx_TOp::Matx_TOp", vec![(pred!(mut, [], []), _)]),
	void cv_Matx_TOp_Matx_TOp(Result<cv::Matx_TOp*>* ocvrs_return) {
		try {
			cv::Matx_TOp* ret = new cv::Matx_TOp();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Matx_TOp(const Matx_TOp &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/matx.hpp:76
	// ("cv::Matx_TOp::Matx_TOp", vec![(pred!(mut, ["unnamed"], ["const cv::Matx_TOp*"]), _)]),
	void cv_Matx_TOp_Matx_TOp_const_Matx_TOpR(const cv::Matx_TOp* unnamed, Result<cv::Matx_TOp*>* ocvrs_return) {
		try {
			cv::Matx_TOp* ret = new cv::Matx_TOp(*unnamed);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Matx_TOp::delete() generated
	// ("cv::Matx_TOp::delete", vec![(pred!(mut, [], []), _)]),
	void cv_Matx_TOp_delete(cv::Matx_TOp* instance) {
			delete instance;
	}

	// getFunction()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/optim.hpp:81
	// ("cv::MinProblemSolver::getFunction", vec![(pred!(const, [], []), _)]),
	void cv_MinProblemSolver_getFunction_const(const cv::MinProblemSolver* instance, Result<cv::Ptr<cv::MinProblemSolver::Function>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::MinProblemSolver::Function> ret = instance->getFunction();
			Ok(new cv::Ptr<cv::MinProblemSolver::Function>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFunction(const Ptr<Function> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/optim.hpp:89
	// ("cv::MinProblemSolver::setFunction", vec![(pred!(mut, ["f"], ["const cv::Ptr<cv::MinProblemSolver::Function>*"]), _)]),
	void cv_MinProblemSolver_setFunction_const_PtrLFunctionGR(cv::MinProblemSolver* instance, const cv::Ptr<cv::MinProblemSolver::Function>* f, ResultVoid* ocvrs_return) {
		try {
			instance->setFunction(*f);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTermCriteria()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/optim.hpp:95
	// ("cv::MinProblemSolver::getTermCriteria", vec![(pred!(const, [], []), _)]),
	void cv_MinProblemSolver_getTermCriteria_const(const cv::MinProblemSolver* instance, Result<cv::TermCriteria>* ocvrs_return) {
		try {
			cv::TermCriteria ret = instance->getTermCriteria();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTermCriteria(const TermCriteria &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/optim.hpp:108
	// ("cv::MinProblemSolver::setTermCriteria", vec![(pred!(mut, ["termcrit"], ["const cv::TermCriteria*"]), _)]),
	void cv_MinProblemSolver_setTermCriteria_const_TermCriteriaR(cv::MinProblemSolver* instance, const cv::TermCriteria* termcrit, ResultVoid* ocvrs_return) {
		try {
			instance->setTermCriteria(*termcrit);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// minimize(InputOutputArray)(InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/optim.hpp:122
	// ("cv::MinProblemSolver::minimize", vec![(pred!(mut, ["x"], ["const cv::_InputOutputArray*"]), _)]),
	void cv_MinProblemSolver_minimize_const__InputOutputArrayR(cv::MinProblemSolver* instance, const cv::_InputOutputArray* x, Result<double>* ocvrs_return) {
		try {
			double ret = instance->minimize(*x);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::MinProblemSolver::to_ConjGradSolver() generated
	// ("cv::MinProblemSolver::to_ConjGradSolver", vec![(pred!(mut, [], []), _)]),
	cv::ConjGradSolver* cv_MinProblemSolver_to_ConjGradSolver(cv::MinProblemSolver* instance) {
			return dynamic_cast<cv::ConjGradSolver*>(instance);
	}

	// cv::MinProblemSolver::to_DownhillSolver() generated
	// ("cv::MinProblemSolver::to_DownhillSolver", vec![(pred!(mut, [], []), _)]),
	cv::DownhillSolver* cv_MinProblemSolver_to_DownhillSolver(cv::MinProblemSolver* instance) {
			return dynamic_cast<cv::DownhillSolver*>(instance);
	}

	// cv::MinProblemSolver::to_Algorithm() generated
	// ("cv::MinProblemSolver::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_MinProblemSolver_to_Algorithm(cv::MinProblemSolver* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::MinProblemSolver::delete() generated
	// ("cv::MinProblemSolver::delete", vec![(pred!(mut, [], []), _)]),
	void cv_MinProblemSolver_delete(cv::MinProblemSolver* instance) {
			delete instance;
	}

	// getDims()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/optim.hpp:67
	// ("cv::MinProblemSolver::Function::getDims", vec![(pred!(const, [], []), _)]),
	void cv_MinProblemSolver_Function_getDims_const(const cv::MinProblemSolver::Function* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDims();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGradientEps()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/optim.hpp:68
	// ("cv::MinProblemSolver::Function::getGradientEps", vec![(pred!(const, [], []), _)]),
	void cv_MinProblemSolver_Function_getGradientEps_const(const cv::MinProblemSolver::Function* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getGradientEps();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calc(const double *)(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/optim.hpp:69
	// ("cv::MinProblemSolver::Function::calc", vec![(pred!(const, ["x"], ["const double*"]), _)]),
	void cv_MinProblemSolver_Function_calc_const_const_doubleX(const cv::MinProblemSolver::Function* instance, const double* x, Result<double>* ocvrs_return) {
		try {
			double ret = instance->calc(x);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGradient(const double *, double *)(Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/optim.hpp:70
	// ("cv::MinProblemSolver::Function::getGradient", vec![(pred!(mut, ["x", "grad"], ["const double*", "double*"]), _)]),
	void cv_MinProblemSolver_Function_getGradient_const_doubleX_doubleX(cv::MinProblemSolver::Function* instance, const double* x, double* grad, ResultVoid* ocvrs_return) {
		try {
			instance->getGradient(x, grad);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::MinProblemSolver::Function::delete() generated
	// ("cv::MinProblemSolver::Function::delete", vec![(pred!(mut, [], []), _)]),
	void cv_MinProblemSolver_Function_delete(cv::MinProblemSolver::Function* instance) {
			delete instance;
	}

	// Moments()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:918
	// ("cv::Moments::Moments", vec![(pred!(mut, [], []), _)]),
	void cv_Moments_Moments(Result<cv::Moments>* ocvrs_return) {
		try {
			cv::Moments ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Moments(double, double, double, double, double, double, double, double, double, double)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:920
	// ("cv::Moments::Moments", vec![(pred!(mut, ["m00", "m10", "m01", "m20", "m11", "m02", "m30", "m21", "m12", "m03"], ["double", "double", "double", "double", "double", "double", "double", "double", "double", "double"]), _)]),
	void cv_Moments_Moments_double_double_double_double_double_double_double_double_double_double(double m00, double m10, double m01, double m20, double m11, double m02, double m30, double m21, double m12, double m03, Result<cv::Moments>* ocvrs_return) {
		try {
			cv::Moments ret(m00, m10, m01, m20, m11, m02, m30, m21, m12, m03);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Mutex()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:708
	// ("cv::Mutex::Mutex", vec![(pred!(mut, [], []), _)]),
	void cv_Mutex_Mutex(Result<cv::Mutex*>* ocvrs_return) {
		try {
			cv::Mutex* ret = new cv::Mutex();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Mutex(const Mutex &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:710
	// ("cv::Mutex::Mutex", vec![(pred!(mut, ["m"], ["const cv::Mutex*"]), _)]),
	void cv_Mutex_Mutex_const_MutexR(const cv::Mutex* m, Result<cv::Mutex*>* ocvrs_return) {
		try {
			cv::Mutex* ret = new cv::Mutex(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const Mutex &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:711
	// ("cv::Mutex::operator=", vec![(pred!(mut, ["m"], ["const cv::Mutex*"]), _)]),
	void cv_Mutex_operatorST_const_MutexR(cv::Mutex* instance, const cv::Mutex* m, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// lock()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:713
	// ("cv::Mutex::lock", vec![(pred!(mut, [], []), _)]),
	void cv_Mutex_lock(cv::Mutex* instance, ResultVoid* ocvrs_return) {
		try {
			instance->lock();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// trylock()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:714
	// ("cv::Mutex::trylock", vec![(pred!(mut, [], []), _)]),
	void cv_Mutex_trylock(cv::Mutex* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->trylock();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// unlock()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:715
	// ("cv::Mutex::unlock", vec![(pred!(mut, [], []), _)]),
	void cv_Mutex_unlock(cv::Mutex* instance, ResultVoid* ocvrs_return) {
		try {
			instance->unlock();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Mutex::delete() generated
	// ("cv::Mutex::delete", vec![(pred!(mut, [], []), _)]),
	void cv_Mutex_delete(cv::Mutex* instance) {
			delete instance;
	}

	// PCA()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2416
	// ("cv::PCA::PCA", vec![(pred!(mut, [], []), _)]),
	void cv_PCA_PCA(Result<cv::PCA*>* ocvrs_return) {
		try {
			cv::PCA* ret = new cv::PCA();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// PCA(InputArray, InputArray, int, int)(InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2427
	// ("cv::PCA::PCA", vec![(pred!(mut, ["data", "mean", "flags", "maxComponents"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "int"]), _)]),
	void cv_PCA_PCA_const__InputArrayR_const__InputArrayR_int_int(const cv::_InputArray* data, const cv::_InputArray* mean, int flags, int maxComponents, Result<cv::PCA*>* ocvrs_return) {
		try {
			cv::PCA* ret = new cv::PCA(*data, *mean, flags, maxComponents);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::PCA::PCA(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2427
	// ("cv::PCA::PCA", vec![(pred!(mut, ["data", "mean", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
	void cv_PCA_PCA_const__InputArrayR_const__InputArrayR_int(const cv::_InputArray* data, const cv::_InputArray* mean, int flags, Result<cv::PCA*>* ocvrs_return) {
		try {
			cv::PCA* ret = new cv::PCA(*data, *mean, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// PCA(InputArray, InputArray, int, double)(InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2439
	// ("cv::PCA::PCA", vec![(pred!(mut, ["data", "mean", "flags", "retainedVariance"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "double"]), _)]),
	void cv_PCA_PCA_const__InputArrayR_const__InputArrayR_int_double(const cv::_InputArray* data, const cv::_InputArray* mean, int flags, double retainedVariance, Result<cv::PCA*>* ocvrs_return) {
		try {
			cv::PCA* ret = new cv::PCA(*data, *mean, flags, retainedVariance);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(InputArray, InputArray, int, int)(InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2461
	// ("cv::PCA::operator()", vec![(pred!(mut, ["data", "mean", "flags", "maxComponents"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "int"]), _)]),
	void cv_PCA_operator___const__InputArrayR_const__InputArrayR_int_int(cv::PCA* instance, const cv::_InputArray* data, const cv::_InputArray* mean, int flags, int maxComponents, Result<cv::PCA*>* ocvrs_return) {
		try {
			cv::PCA ret = instance->operator()(*data, *mean, flags, maxComponents);
			Ok(new cv::PCA(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::PCA::operator()(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2461
	// ("cv::PCA::operator()", vec![(pred!(mut, ["data", "mean", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
	void cv_PCA_operator___const__InputArrayR_const__InputArrayR_int(cv::PCA* instance, const cv::_InputArray* data, const cv::_InputArray* mean, int flags, Result<cv::PCA*>* ocvrs_return) {
		try {
			cv::PCA ret = instance->operator()(*data, *mean, flags);
			Ok(new cv::PCA(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(InputArray, InputArray, int, double)(InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2474
	// ("cv::PCA::operator()", vec![(pred!(mut, ["data", "mean", "flags", "retainedVariance"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "double"]), _)]),
	void cv_PCA_operator___const__InputArrayR_const__InputArrayR_int_double(cv::PCA* instance, const cv::_InputArray* data, const cv::_InputArray* mean, int flags, double retainedVariance, Result<cv::PCA*>* ocvrs_return) {
		try {
			cv::PCA ret = instance->operator()(*data, *mean, flags, retainedVariance);
			Ok(new cv::PCA(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// project(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2490
	// ("cv::PCA::project", vec![(pred!(const, ["vec"], ["const cv::_InputArray*"]), _)]),
	void cv_PCA_project_const_const__InputArrayR(const cv::PCA* instance, const cv::_InputArray* vec, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->project(*vec);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// project(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2504
	// ("cv::PCA::project", vec![(pred!(const, ["vec", "result"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_PCA_project_const_const__InputArrayR_const__OutputArrayR(const cv::PCA* instance, const cv::_InputArray* vec, const cv::_OutputArray* result, ResultVoid* ocvrs_return) {
		try {
			instance->project(*vec, *result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// backProject(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2519
	// ("cv::PCA::backProject", vec![(pred!(const, ["vec"], ["const cv::_InputArray*"]), _)]),
	void cv_PCA_backProject_const_const__InputArrayR(const cv::PCA* instance, const cv::_InputArray* vec, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->backProject(*vec);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// backProject(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2528
	// ("cv::PCA::backProject", vec![(pred!(const, ["vec", "result"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_PCA_backProject_const_const__InputArrayR_const__OutputArrayR(const cv::PCA* instance, const cv::_InputArray* vec, const cv::_OutputArray* result, ResultVoid* ocvrs_return) {
		try {
			instance->backProject(*vec, *result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2534
	// ("cv::PCA::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_PCA_write_const_FileStorageR(const cv::PCA* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2540
	// ("cv::PCA::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_PCA_read_const_FileNodeR(cv::PCA* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::PCA::eigenvectors() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2542
	// ("cv::PCA::eigenvectors", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_PCA_propEigenvectors_const(const cv::PCA* instance) {
			cv::Mat ret = instance->eigenvectors;
			return new cv::Mat(ret);
	}

	// cv::PCA::setEigenvectors(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2542
	// ("cv::PCA::setEigenvectors", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_PCA_propEigenvectors_const_Mat(cv::PCA* instance, const cv::Mat* val) {
			instance->eigenvectors = *val;
	}

	// cv::PCA::eigenvalues() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2543
	// ("cv::PCA::eigenvalues", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_PCA_propEigenvalues_const(const cv::PCA* instance) {
			cv::Mat ret = instance->eigenvalues;
			return new cv::Mat(ret);
	}

	// cv::PCA::setEigenvalues(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2543
	// ("cv::PCA::setEigenvalues", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_PCA_propEigenvalues_const_Mat(cv::PCA* instance, const cv::Mat* val) {
			instance->eigenvalues = *val;
	}

	// cv::PCA::mean() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2544
	// ("cv::PCA::mean", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_PCA_propMean_const(const cv::PCA* instance) {
			cv::Mat ret = instance->mean;
			return new cv::Mat(ret);
	}

	// cv::PCA::setMean(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2544
	// ("cv::PCA::setMean", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_PCA_propMean_const_Mat(cv::PCA* instance, const cv::Mat* val) {
			instance->mean = *val;
	}

	// cv::PCA::delete() generated
	// ("cv::PCA::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PCA_delete(cv::PCA* instance) {
			delete instance;
	}

	// operator()(const Range &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:576
	// ("cv::ParallelLoopBody::operator()", vec![(pred!(const, ["range"], ["const cv::Range*"]), _)]),
	void cv_ParallelLoopBody_operator___const_const_RangeR(const cv::ParallelLoopBody* instance, const cv::Range* range, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*range);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ParallelLoopBody::delete() generated
	// ("cv::ParallelLoopBody::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ParallelLoopBody_delete(cv::ParallelLoopBody* instance) {
			delete instance;
	}

	// RNG()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2795
	// ("cv::RNG::RNG", vec![(pred!(mut, [], []), _)]),
	void cv_RNG_RNG(Result<cv::RNG*>* ocvrs_return) {
		try {
			cv::RNG* ret = new cv::RNG();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// RNG(uint64)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2799
	// ("cv::RNG::RNG", vec![(pred!(mut, ["state"], ["uint64_t"]), _)]),
	void cv_RNG_RNG_uint64_t(uint64_t state, Result<cv::RNG*>* ocvrs_return) {
		try {
			cv::RNG* ret = new cv::RNG(state);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// next()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2802
	// ("cv::RNG::next", vec![(pred!(mut, [], []), _)]),
	void cv_RNG_next(cv::RNG* instance, Result<unsigned int>* ocvrs_return) {
		try {
			unsigned int ret = instance->next();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator unsigned char()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2810
	// ("cv::RNG::operator unsigned char", vec![(pred!(mut, [], []), _)]),
	void cv_RNG_operator_unsigned_char(cv::RNG* instance, Result<unsigned char>* ocvrs_return) {
		try {
			unsigned char ret = instance->operator unsigned char();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator signed char()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2812
	// ("cv::RNG::operator signed char", vec![(pred!(mut, [], []), _)]),
	void cv_RNG_operator_signed_char(cv::RNG* instance, Result<signed char>* ocvrs_return) {
		try {
			signed char ret = instance->operator signed char();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator unsigned short()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2814
	// ("cv::RNG::operator unsigned short", vec![(pred!(mut, [], []), _)]),
	void cv_RNG_operator_unsigned_short(cv::RNG* instance, Result<unsigned short>* ocvrs_return) {
		try {
			unsigned short ret = instance->operator unsigned short();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator short()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2816
	// ("cv::RNG::operator short", vec![(pred!(mut, [], []), _)]),
	void cv_RNG_operator_short(cv::RNG* instance, Result<short>* ocvrs_return) {
		try {
			short ret = instance->operator short();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator unsigned int()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2818
	// ("cv::RNG::operator unsigned int", vec![(pred!(mut, [], []), _)]),
	void cv_RNG_operator_unsigned_int(cv::RNG* instance, Result<unsigned int>* ocvrs_return) {
		try {
			unsigned int ret = instance->operator unsigned int();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator int()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2820
	// ("cv::RNG::operator int", vec![(pred!(mut, [], []), _)]),
	void cv_RNG_operator_int(cv::RNG* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->operator int();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator float()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2822
	// ("cv::RNG::operator float", vec![(pred!(mut, [], []), _)]),
	void cv_RNG_operator_float(cv::RNG* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->operator float();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator double()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2824
	// ("cv::RNG::operator double", vec![(pred!(mut, [], []), _)]),
	void cv_RNG_operator_double(cv::RNG* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->operator double();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2833
	// ("cv::RNG::operator()", vec![(pred!(mut, [], []), _)]),
	void cv_RNG_operator__(cv::RNG* instance, Result<unsigned int>* ocvrs_return) {
		try {
			unsigned int ret = instance->operator()();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(unsigned int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2837
	// ("cv::RNG::operator()", vec![(pred!(mut, ["N"], ["unsigned int"]), _)]),
	void cv_RNG_operator___unsigned_int(cv::RNG* instance, unsigned int N, Result<unsigned int>* ocvrs_return) {
		try {
			unsigned int ret = instance->operator()(N);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// uniform(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2875
	// ("cv::RNG::uniform", vec![(pred!(mut, ["a", "b"], ["int", "int"]), _)]),
	void cv_RNG_uniform_int_int(cv::RNG* instance, int a, int b, Result<int>* ocvrs_return) {
		try {
			int ret = instance->uniform(a, b);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// uniform(float, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2877
	// ("cv::RNG::uniform", vec![(pred!(mut, ["a", "b"], ["float", "float"]), _)]),
	void cv_RNG_uniform_float_float(cv::RNG* instance, float a, float b, Result<float>* ocvrs_return) {
		try {
			float ret = instance->uniform(a, b);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// uniform(double, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2879
	// ("cv::RNG::uniform", vec![(pred!(mut, ["a", "b"], ["double", "double"]), _)]),
	void cv_RNG_uniform_double_double(cv::RNG* instance, double a, double b, Result<double>* ocvrs_return) {
		try {
			double ret = instance->uniform(a, b);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fill(InputOutputArray, int, InputArray, InputArray, bool)(InputOutputArray, Primitive, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2915
	// ("cv::RNG::fill", vec![(pred!(mut, ["mat", "distType", "a", "b", "saturateRange"], ["const cv::_InputOutputArray*", "int", "const cv::_InputArray*", "const cv::_InputArray*", "bool"]), _)]),
	void cv_RNG_fill_const__InputOutputArrayR_int_const__InputArrayR_const__InputArrayR_bool(cv::RNG* instance, const cv::_InputOutputArray* mat, int distType, const cv::_InputArray* a, const cv::_InputArray* b, bool saturateRange, ResultVoid* ocvrs_return) {
		try {
			instance->fill(*mat, distType, *a, *b, saturateRange);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::RNG::fill(InputOutputArray, Primitive, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2915
	// ("cv::RNG::fill", vec![(pred!(mut, ["mat", "distType", "a", "b"], ["const cv::_InputOutputArray*", "int", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_RNG_fill_const__InputOutputArrayR_int_const__InputArrayR_const__InputArrayR(cv::RNG* instance, const cv::_InputOutputArray* mat, int distType, const cv::_InputArray* a, const cv::_InputArray* b, ResultVoid* ocvrs_return) {
		try {
			instance->fill(*mat, distType, *a, *b);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// gaussian(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2925
	// ("cv::RNG::gaussian", vec![(pred!(mut, ["sigma"], ["double"]), _)]),
	void cv_RNG_gaussian_double(cv::RNG* instance, double sigma, Result<double>* ocvrs_return) {
		try {
			double ret = instance->gaussian(sigma);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator==(const RNG &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2929
	// ("cv::RNG::operator==", vec![(pred!(const, ["other"], ["const cv::RNG*"]), _)]),
	void cv_RNG_operatorEQ_const_const_RNGR(const cv::RNG* instance, const cv::RNG* other, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator==(*other);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::RNG::state() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2927
	// ("cv::RNG::state", vec![(pred!(const, [], []), _)]),
	uint64_t cv_RNG_propState_const(const cv::RNG* instance) {
			uint64_t ret = instance->state;
			return ret;
	}

	// cv::RNG::setState(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2927
	// ("cv::RNG::setState", vec![(pred!(mut, ["val"], ["const uint64_t"]), _)]),
	void cv_RNG_propState_const_uint64_t(cv::RNG* instance, const uint64_t val) {
			instance->state = val;
	}

	// cv::RNG::delete() generated
	// ("cv::RNG::delete", vec![(pred!(mut, [], []), _)]),
	void cv_RNG_delete(cv::RNG* instance) {
			delete instance;
	}

	// RNG_MT19937()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2940
	// ("cv::RNG_MT19937::RNG_MT19937", vec![(pred!(mut, [], []), _)]),
	void cv_RNG_MT19937_RNG_MT19937(Result<cv::RNG_MT19937*>* ocvrs_return) {
		try {
			cv::RNG_MT19937* ret = new cv::RNG_MT19937();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// RNG_MT19937(unsigned int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2941
	// ("cv::RNG_MT19937::RNG_MT19937", vec![(pred!(mut, ["s"], ["unsigned int"]), _)]),
	void cv_RNG_MT19937_RNG_MT19937_unsigned_int(unsigned int s, Result<cv::RNG_MT19937*>* ocvrs_return) {
		try {
			cv::RNG_MT19937* ret = new cv::RNG_MT19937(s);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// seed(unsigned int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2942
	// ("cv::RNG_MT19937::seed", vec![(pred!(mut, ["s"], ["unsigned int"]), _)]),
	void cv_RNG_MT19937_seed_unsigned_int(cv::RNG_MT19937* instance, unsigned int s, ResultVoid* ocvrs_return) {
		try {
			instance->seed(s);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// next()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2944
	// ("cv::RNG_MT19937::next", vec![(pred!(mut, [], []), _)]),
	void cv_RNG_MT19937_next(cv::RNG_MT19937* instance, Result<unsigned int>* ocvrs_return) {
		try {
			unsigned int ret = instance->next();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator int()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2946
	// ("cv::RNG_MT19937::operator int", vec![(pred!(mut, [], []), _)]),
	void cv_RNG_MT19937_operator_int(cv::RNG_MT19937* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->operator int();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator unsigned int()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2947
	// ("cv::RNG_MT19937::operator unsigned int", vec![(pred!(mut, [], []), _)]),
	void cv_RNG_MT19937_operator_unsigned_int(cv::RNG_MT19937* instance, Result<unsigned int>* ocvrs_return) {
		try {
			unsigned int ret = instance->operator unsigned int();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator float()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2948
	// ("cv::RNG_MT19937::operator float", vec![(pred!(mut, [], []), _)]),
	void cv_RNG_MT19937_operator_float(cv::RNG_MT19937* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->operator float();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator double()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2949
	// ("cv::RNG_MT19937::operator double", vec![(pred!(mut, [], []), _)]),
	void cv_RNG_MT19937_operator_double(cv::RNG_MT19937* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->operator double();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(unsigned int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2951
	// ("cv::RNG_MT19937::operator()", vec![(pred!(mut, ["N"], ["unsigned int"]), _)]),
	void cv_RNG_MT19937_operator___unsigned_int(cv::RNG_MT19937* instance, unsigned int N, Result<unsigned int>* ocvrs_return) {
		try {
			unsigned int ret = instance->operator()(N);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// uniform(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2955
	// ("cv::RNG_MT19937::uniform", vec![(pred!(mut, ["a", "b"], ["int", "int"]), _)]),
	void cv_RNG_MT19937_uniform_int_int(cv::RNG_MT19937* instance, int a, int b, Result<int>* ocvrs_return) {
		try {
			int ret = instance->uniform(a, b);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// uniform(float, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2957
	// ("cv::RNG_MT19937::uniform", vec![(pred!(mut, ["a", "b"], ["float", "float"]), _)]),
	void cv_RNG_MT19937_uniform_float_float(cv::RNG_MT19937* instance, float a, float b, Result<float>* ocvrs_return) {
		try {
			float ret = instance->uniform(a, b);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// uniform(double, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2959
	// ("cv::RNG_MT19937::uniform", vec![(pred!(mut, ["a", "b"], ["double", "double"]), _)]),
	void cv_RNG_MT19937_uniform_double_double(cv::RNG_MT19937* instance, double a, double b, Result<double>* ocvrs_return) {
		try {
			double ret = instance->uniform(a, b);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::RNG_MT19937::delete() generated
	// ("cv::RNG_MT19937::delete", vec![(pred!(mut, [], []), _)]),
	void cv_RNG_MT19937_delete(cv::RNG_MT19937* instance) {
			delete instance;
	}

	// Range()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:586
	// ("cv::Range::Range", vec![(pred!(mut, [], []), _)]),
	void cv_Range_Range(Result<cv::Range*>* ocvrs_return) {
		try {
			cv::Range* ret = new cv::Range();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Range(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:587
	// ("cv::Range::Range", vec![(pred!(mut, ["_start", "_end"], ["int", "int"]), _)]),
	void cv_Range_Range_int_int(int _start, int _end, Result<cv::Range*>* ocvrs_return) {
		try {
			cv::Range* ret = new cv::Range(_start, _end);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// size()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:588
	// ("cv::Range::size", vec![(pred!(const, [], []), _)]),
	void cv_Range_size_const(const cv::Range* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->size();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:589
	// ("cv::Range::empty", vec![(pred!(const, [], []), _)]),
	void cv_Range_empty_const(const cv::Range* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// all()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:590
	// ("cv::Range::all", vec![(pred!(mut, [], []), _)]),
	void cv_Range_all(Result<cv::Range*>* ocvrs_return) {
		try {
			cv::Range ret = cv::Range::all();
			Ok(new cv::Range(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Range::start() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:592
	// ("cv::Range::start", vec![(pred!(const, [], []), _)]),
	int cv_Range_propStart_const(const cv::Range* instance) {
			int ret = instance->start;
			return ret;
	}

	// cv::Range::setStart(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:592
	// ("cv::Range::setStart", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_Range_propStart_const_int(cv::Range* instance, const int val) {
			instance->start = val;
	}

	// cv::Range::end() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:592
	// ("cv::Range::end", vec![(pred!(const, [], []), _)]),
	int cv_Range_propEnd_const(const cv::Range* instance) {
			int ret = instance->end;
			return ret;
	}

	// cv::Range::setEnd(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:592
	// ("cv::Range::setEnd", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_Range_propEnd_const_int(cv::Range* instance, const int val) {
			instance->end = val;
	}

	// cv::Range::delete() generated
	// ("cv::Range::delete", vec![(pred!(mut, [], []), _)]),
	void cv_Range_delete(cv::Range* instance) {
			delete instance;
	}

	// RotatedRect()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:501
	// ("cv::RotatedRect::RotatedRect", vec![(pred!(mut, [], []), _)]),
	void cv_RotatedRect_RotatedRect(Result<cv::RotatedRect>* ocvrs_return) {
		try {
			cv::RotatedRect ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// RotatedRect(const Point2f &, const Size2f &, float)(SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:508
	// ("cv::RotatedRect::RotatedRect", vec![(pred!(mut, ["center", "size", "angle"], ["const cv::Point2f*", "const cv::Size2f*", "float"]), _)]),
	void cv_RotatedRect_RotatedRect_const_Point2fR_const_Size2fR_float(const cv::Point2f* center, const cv::Size2f* size, float angle, Result<cv::RotatedRect>* ocvrs_return) {
		try {
			cv::RotatedRect ret(*center, *size, angle);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// RotatedRect(const Point2f &, const Point2f &, const Point2f &)(SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:513
	// ("cv::RotatedRect::RotatedRect", vec![(pred!(mut, ["point1", "point2", "point3"], ["const cv::Point2f*", "const cv::Point2f*", "const cv::Point2f*"]), _)]),
	void cv_RotatedRect_RotatedRect_const_Point2fR_const_Point2fR_const_Point2fR(const cv::Point2f* point1, const cv::Point2f* point2, const cv::Point2f* point3, Result<cv::RotatedRect>* ocvrs_return) {
		try {
			cv::RotatedRect ret(*point1, *point2, *point3);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// points(Point2f *)(FixedArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:518
	// ("cv::RotatedRect::points", vec![(pred!(const, ["pts"], ["cv::Point2f*"]), _)]),
	void cv_RotatedRect_points_const_Point2fXX(const cv::RotatedRect* instance, cv::Point2f(*pts)[4], ResultVoid* ocvrs_return) {
		try {
			instance->points(*pts);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// boundingRect()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:520
	// ("cv::RotatedRect::boundingRect", vec![(pred!(const, [], []), _)]),
	void cv_RotatedRect_boundingRect_const(const cv::RotatedRect* instance, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->boundingRect();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// boundingRect2f()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:522
	// ("cv::RotatedRect::boundingRect2f", vec![(pred!(const, [], []), _)]),
	void cv_RotatedRect_boundingRect2f_const(const cv::RotatedRect* instance, Result<cv::Rect_<float>>* ocvrs_return) {
		try {
			cv::Rect_<float> ret = instance->boundingRect2f();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// SVD()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2661
	// ("cv::SVD::SVD", vec![(pred!(mut, [], []), _)]),
	void cv_SVD_SVD(Result<cv::SVD*>* ocvrs_return) {
		try {
			cv::SVD* ret = new cv::SVD();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// SVD(InputArray, int)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2668
	// ("cv::SVD::SVD", vec![(pred!(mut, ["src", "flags"], ["const cv::_InputArray*", "int"]), _)]),
	void cv_SVD_SVD_const__InputArrayR_int(const cv::_InputArray* src, int flags, Result<cv::SVD*>* ocvrs_return) {
		try {
			cv::SVD* ret = new cv::SVD(*src, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SVD::SVD(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2668
	// ("cv::SVD::SVD", vec![(pred!(mut, ["src"], ["const cv::_InputArray*"]), _)]),
	void cv_SVD_SVD_const__InputArrayR(const cv::_InputArray* src, Result<cv::SVD*>* ocvrs_return) {
		try {
			cv::SVD* ret = new cv::SVD(*src);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(InputArray, int)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2681
	// ("cv::SVD::operator()", vec![(pred!(mut, ["src", "flags"], ["const cv::_InputArray*", "int"]), _)]),
	void cv_SVD_operator___const__InputArrayR_int(cv::SVD* instance, const cv::_InputArray* src, int flags, Result<cv::SVD*>* ocvrs_return) {
		try {
			cv::SVD ret = instance->operator()(*src, flags);
			Ok(new cv::SVD(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SVD::operator()(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2681
	// ("cv::SVD::operator()", vec![(pred!(mut, ["src"], ["const cv::_InputArray*"]), _)]),
	void cv_SVD_operator___const__InputArrayR(cv::SVD* instance, const cv::_InputArray* src, Result<cv::SVD*>* ocvrs_return) {
		try {
			cv::SVD ret = instance->operator()(*src);
			Ok(new cv::SVD(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArray, OutputArray, OutputArray, OutputArray, int)(InputArray, OutputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2700
	// ("cv::SVD::compute", vec![(pred!(mut, ["src", "w", "u", "vt", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_SVD_compute_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* w, const cv::_OutputArray* u, const cv::_OutputArray* vt, int flags, ResultVoid* ocvrs_return) {
		try {
			cv::SVD::compute(*src, *w, *u, *vt, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SVD::compute(InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2700
	// ("cv::SVD::compute", vec![(pred!(mut, ["src", "w", "u", "vt"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_SVD_compute_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* w, const cv::_OutputArray* u, const cv::_OutputArray* vt, ResultVoid* ocvrs_return) {
		try {
			cv::SVD::compute(*src, *w, *u, *vt);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2709
	// ("cv::SVD::compute", vec![(pred!(mut, ["src", "w", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_SVD_compute_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* w, int flags, ResultVoid* ocvrs_return) {
		try {
			cv::SVD::compute(*src, *w, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SVD::compute(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2709
	// ("cv::SVD::compute", vec![(pred!(mut, ["src", "w"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_SVD_compute_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* w, ResultVoid* ocvrs_return) {
		try {
			cv::SVD::compute(*src, *w);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// backSubst(InputArray, InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2713
	// ("cv::SVD::backSubst", vec![(pred!(mut, ["w", "u", "vt", "rhs", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_SVD_backSubst_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* w, const cv::_InputArray* u, const cv::_InputArray* vt, const cv::_InputArray* rhs, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::SVD::backSubst(*w, *u, *vt, *rhs, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// solveZ(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2727
	// ("cv::SVD::solveZ", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_SVD_solveZ_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::SVD::solveZ(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// backSubst(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2751
	// ("cv::SVD::backSubst", vec![(pred!(const, ["rhs", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_SVD_backSubst_const_const__InputArrayR_const__OutputArrayR(const cv::SVD* instance, const cv::_InputArray* rhs, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->backSubst(*rhs, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SVD::u() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2765
	// ("cv::SVD::u", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_SVD_propU_const(const cv::SVD* instance) {
			cv::Mat ret = instance->u;
			return new cv::Mat(ret);
	}

	// cv::SVD::setU(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2765
	// ("cv::SVD::setU", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_SVD_propU_const_Mat(cv::SVD* instance, const cv::Mat* val) {
			instance->u = *val;
	}

	// cv::SVD::w() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2765
	// ("cv::SVD::w", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_SVD_propW_const(const cv::SVD* instance) {
			cv::Mat ret = instance->w;
			return new cv::Mat(ret);
	}

	// cv::SVD::setW(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2765
	// ("cv::SVD::setW", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_SVD_propW_const_Mat(cv::SVD* instance, const cv::Mat* val) {
			instance->w = *val;
	}

	// cv::SVD::vt() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2765
	// ("cv::SVD::vt", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_SVD_propVt_const(const cv::SVD* instance) {
			cv::Mat ret = instance->vt;
			return new cv::Mat(ret);
	}

	// cv::SVD::setVt(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core.hpp:2765
	// ("cv::SVD::setVt", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_SVD_propVt_const_Mat(cv::SVD* instance, const cv::Mat* val) {
			instance->vt = *val;
	}

	// cv::SVD::delete() generated
	// ("cv::SVD::delete", vec![(pred!(mut, [], []), _)]),
	void cv_SVD_delete(cv::SVD* instance) {
			delete instance;
	}

	// SparseMat()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2770
	// ("cv::SparseMat::SparseMat", vec![(pred!(mut, [], []), _)]),
	void cv_SparseMat_SparseMat(Result<cv::SparseMat*>* ocvrs_return) {
		try {
			cv::SparseMat* ret = new cv::SparseMat();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// SparseMat(int, const int *, int)(Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2777
	// ("cv::SparseMat::SparseMat", vec![(pred!(mut, ["dims", "_sizes", "_type"], ["int", "const int*", "int"]), _)]),
	void cv_SparseMat_SparseMat_int_const_intX_int(int dims, const int* _sizes, int _type, Result<cv::SparseMat*>* ocvrs_return) {
		try {
			cv::SparseMat* ret = new cv::SparseMat(dims, _sizes, _type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// SparseMat(const SparseMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2783
	// ("cv::SparseMat::SparseMat", vec![(pred!(mut, ["m"], ["const cv::SparseMat*"]), _)]),
	void cv_SparseMat_SparseMat_const_SparseMatR(const cv::SparseMat* m, Result<cv::SparseMat*>* ocvrs_return) {
		try {
			cv::SparseMat* ret = new cv::SparseMat(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// SparseMat(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2789
	// ("cv::SparseMat::SparseMat", vec![(pred!(mut, ["m"], ["const cv::Mat*"]), _)]),
	void cv_SparseMat_SparseMat_const_MatR(const cv::Mat* m, Result<cv::SparseMat*>* ocvrs_return) {
		try {
			cv::SparseMat* ret = new cv::SparseMat(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const SparseMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2795
	// ("cv::SparseMat::operator=", vec![(pred!(mut, ["m"], ["const cv::SparseMat*"]), _)]),
	void cv_SparseMat_operatorST_const_SparseMatR(cv::SparseMat* instance, const cv::SparseMat* m, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2797
	// ("cv::SparseMat::operator=", vec![(pred!(mut, ["m"], ["const cv::Mat*"]), _)]),
	void cv_SparseMat_operatorST_const_MatR(cv::SparseMat* instance, const cv::Mat* m, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clone()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2800
	// ("cv::SparseMat::clone", vec![(pred!(const, [], []), _)]),
	void cv_SparseMat_clone_const(const cv::SparseMat* instance, Result<cv::SparseMat*>* ocvrs_return) {
		try {
			cv::SparseMat ret = instance->clone();
			Ok(new cv::SparseMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// copyTo(SparseMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2803
	// ("cv::SparseMat::copyTo", vec![(pred!(const, ["m"], ["cv::SparseMat*"]), _)]),
	void cv_SparseMat_copyTo_const_SparseMatR(const cv::SparseMat* instance, cv::SparseMat* m, ResultVoid* ocvrs_return) {
		try {
			instance->copyTo(*m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// copyTo(Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2805
	// ("cv::SparseMat::copyTo", vec![(pred!(const, ["m"], ["cv::Mat*"]), _)]),
	void cv_SparseMat_copyTo_const_MatR(const cv::SparseMat* instance, cv::Mat* m, ResultVoid* ocvrs_return) {
		try {
			instance->copyTo(*m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertTo(SparseMat &, int, double)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2807
	// ("cv::SparseMat::convertTo", vec![(pred!(const, ["m", "rtype", "alpha"], ["cv::SparseMat*", "int", "double"]), _)]),
	void cv_SparseMat_convertTo_const_SparseMatR_int_double(const cv::SparseMat* instance, cv::SparseMat* m, int rtype, double alpha, ResultVoid* ocvrs_return) {
		try {
			instance->convertTo(*m, rtype, alpha);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SparseMat::convertTo(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2807
	// ("cv::SparseMat::convertTo", vec![(pred!(const, ["m", "rtype"], ["cv::SparseMat*", "int"]), _)]),
	void cv_SparseMat_convertTo_const_SparseMatR_int(const cv::SparseMat* instance, cv::SparseMat* m, int rtype, ResultVoid* ocvrs_return) {
		try {
			instance->convertTo(*m, rtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertTo(Mat &, int, double, double)(TraitClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2818
	// ("cv::SparseMat::convertTo", vec![(pred!(const, ["m", "rtype", "alpha", "beta"], ["cv::Mat*", "int", "double", "double"]), _)]),
	void cv_SparseMat_convertTo_const_MatR_int_double_double(const cv::SparseMat* instance, cv::Mat* m, int rtype, double alpha, double beta, ResultVoid* ocvrs_return) {
		try {
			instance->convertTo(*m, rtype, alpha, beta);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SparseMat::convertTo(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2818
	// ("cv::SparseMat::convertTo", vec![(pred!(const, ["m", "rtype"], ["cv::Mat*", "int"]), _)]),
	void cv_SparseMat_convertTo_const_MatR_int(const cv::SparseMat* instance, cv::Mat* m, int rtype, ResultVoid* ocvrs_return) {
		try {
			instance->convertTo(*m, rtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// assignTo(SparseMat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2821
	// ("cv::SparseMat::assignTo", vec![(pred!(const, ["m", "type"], ["cv::SparseMat*", "int"]), _)]),
	void cv_SparseMat_assignTo_const_SparseMatR_int(const cv::SparseMat* instance, cv::SparseMat* m, int type, ResultVoid* ocvrs_return) {
		try {
			instance->assignTo(*m, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SparseMat::assignTo(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2821
	// ("cv::SparseMat::assignTo", vec![(pred!(const, ["m"], ["cv::SparseMat*"]), _)]),
	void cv_SparseMat_assignTo_const_SparseMatR(const cv::SparseMat* instance, cv::SparseMat* m, ResultVoid* ocvrs_return) {
		try {
			instance->assignTo(*m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, const int *, int)(Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2829
	// ("cv::SparseMat::create", vec![(pred!(mut, ["dims", "_sizes", "_type"], ["int", "const int*", "int"]), _)]),
	void cv_SparseMat_create_int_const_intX_int(cv::SparseMat* instance, int dims, const int* _sizes, int _type, ResultVoid* ocvrs_return) {
		try {
			instance->create(dims, _sizes, _type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clear()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2831
	// ("cv::SparseMat::clear", vec![(pred!(mut, [], []), _)]),
	void cv_SparseMat_clear(cv::SparseMat* instance, ResultVoid* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addref()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2833
	// ("cv::SparseMat::addref", vec![(pred!(mut, [], []), _)]),
	void cv_SparseMat_addref(cv::SparseMat* instance, ResultVoid* ocvrs_return) {
		try {
			instance->addref();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// release()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2835
	// ("cv::SparseMat::release", vec![(pred!(mut, [], []), _)]),
	void cv_SparseMat_release(cv::SparseMat* instance, ResultVoid* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// elemSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2840
	// ("cv::SparseMat::elemSize", vec![(pred!(const, [], []), _)]),
	size_t cv_SparseMat_elemSize_const(const cv::SparseMat* instance) {
			size_t ret = instance->elemSize();
			return ret;
	}

	// elemSize1()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2842
	// ("cv::SparseMat::elemSize1", vec![(pred!(const, [], []), _)]),
	size_t cv_SparseMat_elemSize1_const(const cv::SparseMat* instance) {
			size_t ret = instance->elemSize1();
			return ret;
	}

	// type()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2845
	// ("cv::SparseMat::type", vec![(pred!(const, [], []), _)]),
	int cv_SparseMat_type_const(const cv::SparseMat* instance) {
			int ret = instance->type();
			return ret;
	}

	// depth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2847
	// ("cv::SparseMat::depth", vec![(pred!(const, [], []), _)]),
	int cv_SparseMat_depth_const(const cv::SparseMat* instance) {
			int ret = instance->depth();
			return ret;
	}

	// channels()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2849
	// ("cv::SparseMat::channels", vec![(pred!(const, [], []), _)]),
	int cv_SparseMat_channels_const(const cv::SparseMat* instance) {
			int ret = instance->channels();
			return ret;
	}

	// size()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2852
	// ("cv::SparseMat::size", vec![(pred!(const, [], []), _)]),
	void cv_SparseMat_size_const(const cv::SparseMat* instance, Result<const int*>* ocvrs_return) {
		try {
			const int* ret = instance->size();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// size(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2854
	// ("cv::SparseMat::size", vec![(pred!(const, ["i"], ["int"]), _)]),
	void cv_SparseMat_size_const_int(const cv::SparseMat* instance, int i, Result<int>* ocvrs_return) {
		try {
			int ret = instance->size(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dims()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2856
	// ("cv::SparseMat::dims", vec![(pred!(const, [], []), _)]),
	void cv_SparseMat_dims_const(const cv::SparseMat* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->dims();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// nzcount()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2858
	// ("cv::SparseMat::nzcount", vec![(pred!(const, [], []), _)]),
	void cv_SparseMat_nzcount_const(const cv::SparseMat* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->nzcount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// hash(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2861
	// ("cv::SparseMat::hash", vec![(pred!(const, ["i0"], ["int"]), _)]),
	void cv_SparseMat_hash_const_int(const cv::SparseMat* instance, int i0, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->hash(i0);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// hash(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2863
	// ("cv::SparseMat::hash", vec![(pred!(const, ["i0", "i1"], ["int", "int"]), _)]),
	void cv_SparseMat_hash_const_int_int(const cv::SparseMat* instance, int i0, int i1, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->hash(i0, i1);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// hash(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2865
	// ("cv::SparseMat::hash", vec![(pred!(const, ["i0", "i1", "i2"], ["int", "int", "int"]), _)]),
	void cv_SparseMat_hash_const_int_int_int(const cv::SparseMat* instance, int i0, int i1, int i2, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->hash(i0, i1, i2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// hash(const int *)(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2867
	// ("cv::SparseMat::hash", vec![(pred!(const, ["idx"], ["const int*"]), _)]),
	void cv_SparseMat_hash_const_const_intX(const cv::SparseMat* instance, const int* idx, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->hash(idx);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ptr(int, bool, size_t *)(Primitive, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2881
	// ("cv::SparseMat::ptr", vec![(pred!(mut, ["i0", "createMissing", "hashval"], ["int", "bool", "size_t*"]), _)]),
	void cv_SparseMat_ptr_int_bool_size_tX(cv::SparseMat* instance, int i0, bool createMissing, size_t* hashval, Result<unsigned char*>* ocvrs_return) {
		try {
			unsigned char* ret = instance->ptr(i0, createMissing, hashval);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SparseMat::ptr(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2881
	// ("cv::SparseMat::ptr", vec![(pred!(mut, ["i0", "createMissing"], ["int", "bool"]), _)]),
	void cv_SparseMat_ptr_int_bool(cv::SparseMat* instance, int i0, bool createMissing, Result<unsigned char*>* ocvrs_return) {
		try {
			unsigned char* ret = instance->ptr(i0, createMissing);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ptr(int, int, bool, size_t *)(Primitive, Primitive, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2883
	// ("cv::SparseMat::ptr", vec![(pred!(mut, ["i0", "i1", "createMissing", "hashval"], ["int", "int", "bool", "size_t*"]), _)]),
	void cv_SparseMat_ptr_int_int_bool_size_tX(cv::SparseMat* instance, int i0, int i1, bool createMissing, size_t* hashval, Result<unsigned char*>* ocvrs_return) {
		try {
			unsigned char* ret = instance->ptr(i0, i1, createMissing, hashval);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SparseMat::ptr(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2883
	// ("cv::SparseMat::ptr", vec![(pred!(mut, ["i0", "i1", "createMissing"], ["int", "int", "bool"]), _)]),
	void cv_SparseMat_ptr_int_int_bool(cv::SparseMat* instance, int i0, int i1, bool createMissing, Result<unsigned char*>* ocvrs_return) {
		try {
			unsigned char* ret = instance->ptr(i0, i1, createMissing);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ptr(int, int, int, bool, size_t *)(Primitive, Primitive, Primitive, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2885
	// ("cv::SparseMat::ptr", vec![(pred!(mut, ["i0", "i1", "i2", "createMissing", "hashval"], ["int", "int", "int", "bool", "size_t*"]), _)]),
	void cv_SparseMat_ptr_int_int_int_bool_size_tX(cv::SparseMat* instance, int i0, int i1, int i2, bool createMissing, size_t* hashval, Result<unsigned char*>* ocvrs_return) {
		try {
			unsigned char* ret = instance->ptr(i0, i1, i2, createMissing, hashval);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SparseMat::ptr(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2885
	// ("cv::SparseMat::ptr", vec![(pred!(mut, ["i0", "i1", "i2", "createMissing"], ["int", "int", "int", "bool"]), _)]),
	void cv_SparseMat_ptr_int_int_int_bool(cv::SparseMat* instance, int i0, int i1, int i2, bool createMissing, Result<unsigned char*>* ocvrs_return) {
		try {
			unsigned char* ret = instance->ptr(i0, i1, i2, createMissing);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ptr(const int *, bool, size_t *)(Indirect, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2887
	// ("cv::SparseMat::ptr", vec![(pred!(mut, ["idx", "createMissing", "hashval"], ["const int*", "bool", "size_t*"]), _)]),
	void cv_SparseMat_ptr_const_intX_bool_size_tX(cv::SparseMat* instance, const int* idx, bool createMissing, size_t* hashval, Result<unsigned char*>* ocvrs_return) {
		try {
			unsigned char* ret = instance->ptr(idx, createMissing, hashval);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SparseMat::ptr(Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2887
	// ("cv::SparseMat::ptr", vec![(pred!(mut, ["idx", "createMissing"], ["const int*", "bool"]), _)]),
	void cv_SparseMat_ptr_const_intX_bool(cv::SparseMat* instance, const int* idx, bool createMissing, Result<unsigned char*>* ocvrs_return) {
		try {
			unsigned char* ret = instance->ptr(idx, createMissing);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// erase(int, int, size_t *)(Primitive, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2948
	// ("cv::SparseMat::erase", vec![(pred!(mut, ["i0", "i1", "hashval"], ["int", "int", "size_t*"]), _)]),
	void cv_SparseMat_erase_int_int_size_tX(cv::SparseMat* instance, int i0, int i1, size_t* hashval, ResultVoid* ocvrs_return) {
		try {
			instance->erase(i0, i1, hashval);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SparseMat::erase(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2948
	// ("cv::SparseMat::erase", vec![(pred!(mut, ["i0", "i1"], ["int", "int"]), _)]),
	void cv_SparseMat_erase_int_int(cv::SparseMat* instance, int i0, int i1, ResultVoid* ocvrs_return) {
		try {
			instance->erase(i0, i1);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// erase(int, int, int, size_t *)(Primitive, Primitive, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2950
	// ("cv::SparseMat::erase", vec![(pred!(mut, ["i0", "i1", "i2", "hashval"], ["int", "int", "int", "size_t*"]), _)]),
	void cv_SparseMat_erase_int_int_int_size_tX(cv::SparseMat* instance, int i0, int i1, int i2, size_t* hashval, ResultVoid* ocvrs_return) {
		try {
			instance->erase(i0, i1, i2, hashval);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SparseMat::erase(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2950
	// ("cv::SparseMat::erase", vec![(pred!(mut, ["i0", "i1", "i2"], ["int", "int", "int"]), _)]),
	void cv_SparseMat_erase_int_int_int(cv::SparseMat* instance, int i0, int i1, int i2, ResultVoid* ocvrs_return) {
		try {
			instance->erase(i0, i1, i2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// erase(const int *, size_t *)(Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2952
	// ("cv::SparseMat::erase", vec![(pred!(mut, ["idx", "hashval"], ["const int*", "size_t*"]), _)]),
	void cv_SparseMat_erase_const_intX_size_tX(cv::SparseMat* instance, const int* idx, size_t* hashval, ResultVoid* ocvrs_return) {
		try {
			instance->erase(idx, hashval);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SparseMat::erase(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2952
	// ("cv::SparseMat::erase", vec![(pred!(mut, ["idx"], ["const int*"]), _)]),
	void cv_SparseMat_erase_const_intX(cv::SparseMat* instance, const int* idx, ResultVoid* ocvrs_return) {
		try {
			instance->erase(idx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// begin()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2959
	// ("cv::SparseMat::begin", vec![(pred!(mut, [], []), _)]),
	void cv_SparseMat_begin(cv::SparseMat* instance, Result<cv::SparseMatIterator*>* ocvrs_return) {
		try {
			cv::SparseMatIterator ret = instance->begin();
			Ok(new cv::SparseMatIterator(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// begin()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2963
	// ("cv::SparseMat::begin", vec![(pred!(const, [], []), _)]),
	void cv_SparseMat_begin_const(const cv::SparseMat* instance, Result<cv::SparseMatConstIterator*>* ocvrs_return) {
		try {
			cv::SparseMatConstIterator ret = instance->begin();
			Ok(new cv::SparseMatConstIterator(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// end()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2971
	// ("cv::SparseMat::end", vec![(pred!(mut, [], []), _)]),
	void cv_SparseMat_end(cv::SparseMat* instance, Result<cv::SparseMatIterator*>* ocvrs_return) {
		try {
			cv::SparseMatIterator ret = instance->end();
			Ok(new cv::SparseMatIterator(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// end()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2973
	// ("cv::SparseMat::end", vec![(pred!(const, [], []), _)]),
	void cv_SparseMat_end_const(const cv::SparseMat* instance, Result<cv::SparseMatConstIterator*>* ocvrs_return) {
		try {
			cv::SparseMatConstIterator ret = instance->end();
			Ok(new cv::SparseMatConstIterator(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// node(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2985
	// ("cv::SparseMat::node", vec![(pred!(mut, ["nidx"], ["size_t"]), _)]),
	void cv_SparseMat_node_size_t(cv::SparseMat* instance, size_t nidx, Result<cv::SparseMat::Node*>* ocvrs_return) {
		try {
			cv::SparseMat::Node* ret = instance->node(nidx);
			Ok(new cv::SparseMat::Node(*ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// node(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2986
	// ("cv::SparseMat::node", vec![(pred!(const, ["nidx"], ["size_t"]), _)]),
	void cv_SparseMat_node_const_size_t(const cv::SparseMat* instance, size_t nidx, Result<const cv::SparseMat::Node*>* ocvrs_return) {
		try {
			const cv::SparseMat::Node* ret = instance->node(nidx);
			Ok(new const cv::SparseMat::Node(*ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// newNode(const int *, size_t)(Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2988
	// ("cv::SparseMat::newNode", vec![(pred!(mut, ["idx", "hashval"], ["const int*", "size_t"]), _)]),
	void cv_SparseMat_newNode_const_intX_size_t(cv::SparseMat* instance, const int* idx, size_t hashval, Result<unsigned char*>* ocvrs_return) {
		try {
			unsigned char* ret = instance->newNode(idx, hashval);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// removeNode(size_t, size_t, size_t)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2989
	// ("cv::SparseMat::removeNode", vec![(pred!(mut, ["hidx", "nidx", "previdx"], ["size_t", "size_t", "size_t"]), _)]),
	void cv_SparseMat_removeNode_size_t_size_t_size_t(cv::SparseMat* instance, size_t hidx, size_t nidx, size_t previdx, ResultVoid* ocvrs_return) {
		try {
			instance->removeNode(hidx, nidx, previdx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// resizeHashTab(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2990
	// ("cv::SparseMat::resizeHashTab", vec![(pred!(mut, ["newsize"], ["size_t"]), _)]),
	void cv_SparseMat_resizeHashTab_size_t(cv::SparseMat* instance, size_t newsize, ResultVoid* ocvrs_return) {
		try {
			instance->resizeHashTab(newsize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SparseMat::flags() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2992
	// ("cv::SparseMat::flags", vec![(pred!(const, [], []), _)]),
	int cv_SparseMat_propFlags_const(const cv::SparseMat* instance) {
			int ret = instance->flags;
			return ret;
	}

	// cv::SparseMat::setFlags(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2992
	// ("cv::SparseMat::setFlags", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_SparseMat_propFlags_const_int(cv::SparseMat* instance, const int val) {
			instance->flags = val;
	}

	// cv::SparseMat::hdr() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2993
	// ("cv::SparseMat::hdr", vec![(pred!(mut, [], []), _)]),
	cv::SparseMat::Hdr* cv_SparseMat_propHdr(cv::SparseMat* instance) {
			cv::SparseMat::Hdr* ret = instance->hdr;
			return new cv::SparseMat::Hdr(*ret);
	}

	// cv::SparseMat::setHdr(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2993
	// ("cv::SparseMat::setHdr", vec![(pred!(mut, ["val"], ["cv::SparseMat::Hdr*"]), _)]),
	void cv_SparseMat_propHdr_HdrX(cv::SparseMat* instance, cv::SparseMat::Hdr* const val) {
			instance->hdr = val;
	}

	// cv::SparseMat::delete() generated
	// ("cv::SparseMat::delete", vec![(pred!(mut, [], []), _)]),
	void cv_SparseMat_delete(cv::SparseMat* instance) {
			delete instance;
	}

	// Hdr(int, const int *, int)(Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2744
	// ("cv::SparseMat::Hdr::Hdr", vec![(pred!(mut, ["_dims", "_sizes", "_type"], ["int", "const int*", "int"]), _)]),
	void cv_SparseMat_Hdr_Hdr_int_const_intX_int(int _dims, const int* _sizes, int _type, Result<cv::SparseMat::Hdr*>* ocvrs_return) {
		try {
			cv::SparseMat::Hdr* ret = new cv::SparseMat::Hdr(_dims, _sizes, _type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clear()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2745
	// ("cv::SparseMat::Hdr::clear", vec![(pred!(mut, [], []), _)]),
	void cv_SparseMat_Hdr_clear(cv::SparseMat::Hdr* instance, ResultVoid* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SparseMat::Hdr::refcount() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2746
	// ("cv::SparseMat::Hdr::refcount", vec![(pred!(const, [], []), _)]),
	int cv_SparseMat_Hdr_propRefcount_const(const cv::SparseMat::Hdr* instance) {
			int ret = instance->refcount;
			return ret;
	}

	// cv::SparseMat::Hdr::setRefcount(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2746
	// ("cv::SparseMat::Hdr::setRefcount", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_SparseMat_Hdr_propRefcount_const_int(cv::SparseMat::Hdr* instance, const int val) {
			instance->refcount = val;
	}

	// cv::SparseMat::Hdr::dims() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2747
	// ("cv::SparseMat::Hdr::dims", vec![(pred!(const, [], []), _)]),
	int cv_SparseMat_Hdr_propDims_const(const cv::SparseMat::Hdr* instance) {
			int ret = instance->dims;
			return ret;
	}

	// cv::SparseMat::Hdr::setDims(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2747
	// ("cv::SparseMat::Hdr::setDims", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_SparseMat_Hdr_propDims_const_int(cv::SparseMat::Hdr* instance, const int val) {
			instance->dims = val;
	}

	// cv::SparseMat::Hdr::valueOffset() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2748
	// ("cv::SparseMat::Hdr::valueOffset", vec![(pred!(const, [], []), _)]),
	int cv_SparseMat_Hdr_propValueOffset_const(const cv::SparseMat::Hdr* instance) {
			int ret = instance->valueOffset;
			return ret;
	}

	// cv::SparseMat::Hdr::setValueOffset(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2748
	// ("cv::SparseMat::Hdr::setValueOffset", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_SparseMat_Hdr_propValueOffset_const_int(cv::SparseMat::Hdr* instance, const int val) {
			instance->valueOffset = val;
	}

	// cv::SparseMat::Hdr::nodeSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2749
	// ("cv::SparseMat::Hdr::nodeSize", vec![(pred!(const, [], []), _)]),
	size_t cv_SparseMat_Hdr_propNodeSize_const(const cv::SparseMat::Hdr* instance) {
			size_t ret = instance->nodeSize;
			return ret;
	}

	// cv::SparseMat::Hdr::setNodeSize(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2749
	// ("cv::SparseMat::Hdr::setNodeSize", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
	void cv_SparseMat_Hdr_propNodeSize_const_size_t(cv::SparseMat::Hdr* instance, const size_t val) {
			instance->nodeSize = val;
	}

	// cv::SparseMat::Hdr::nodeCount() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2750
	// ("cv::SparseMat::Hdr::nodeCount", vec![(pred!(const, [], []), _)]),
	size_t cv_SparseMat_Hdr_propNodeCount_const(const cv::SparseMat::Hdr* instance) {
			size_t ret = instance->nodeCount;
			return ret;
	}

	// cv::SparseMat::Hdr::setNodeCount(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2750
	// ("cv::SparseMat::Hdr::setNodeCount", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
	void cv_SparseMat_Hdr_propNodeCount_const_size_t(cv::SparseMat::Hdr* instance, const size_t val) {
			instance->nodeCount = val;
	}

	// cv::SparseMat::Hdr::freeList() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2751
	// ("cv::SparseMat::Hdr::freeList", vec![(pred!(const, [], []), _)]),
	size_t cv_SparseMat_Hdr_propFreeList_const(const cv::SparseMat::Hdr* instance) {
			size_t ret = instance->freeList;
			return ret;
	}

	// cv::SparseMat::Hdr::setFreeList(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2751
	// ("cv::SparseMat::Hdr::setFreeList", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
	void cv_SparseMat_Hdr_propFreeList_const_size_t(cv::SparseMat::Hdr* instance, const size_t val) {
			instance->freeList = val;
	}

	// cv::SparseMat::Hdr::pool() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2752
	// ("cv::SparseMat::Hdr::pool", vec![(pred!(const, [], []), _)]),
	std::vector<unsigned char>* cv_SparseMat_Hdr_propPool_const(const cv::SparseMat::Hdr* instance) {
			std::vector<unsigned char> ret = instance->pool;
			return new std::vector<unsigned char>(ret);
	}

	// cv::SparseMat::Hdr::setPool(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2752
	// ("cv::SparseMat::Hdr::setPool", vec![(pred!(mut, ["val"], ["const std::vector<unsigned char>"]), _)]),
	void cv_SparseMat_Hdr_propPool_const_vectorLunsigned_charG(cv::SparseMat::Hdr* instance, const std::vector<unsigned char>* val) {
			instance->pool = *val;
	}

	// cv::SparseMat::Hdr::hashtab() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2753
	// ("cv::SparseMat::Hdr::hashtab", vec![(pred!(const, [], []), _)]),
	std::vector<size_t>* cv_SparseMat_Hdr_propHashtab_const(const cv::SparseMat::Hdr* instance) {
			std::vector<size_t> ret = instance->hashtab;
			return new std::vector<size_t>(ret);
	}

	// cv::SparseMat::Hdr::setHashtab(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2753
	// ("cv::SparseMat::Hdr::setHashtab", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
	void cv_SparseMat_Hdr_propHashtab_const_vectorLsize_tG(cv::SparseMat::Hdr* instance, const std::vector<size_t>* val) {
			instance->hashtab = *val;
	}

	// cv::SparseMat::Hdr::size() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2754
	// ("cv::SparseMat::Hdr::size", vec![(pred!(const, [], []), _)]),
	const int** cv_SparseMat_Hdr_propSize_const(const cv::SparseMat::Hdr* instance) {
			const int(*ret)[32] = &instance->size;
			return (const int**)ret;
	}

	// cv::SparseMat::Hdr::sizeMut() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2754
	// ("cv::SparseMat::Hdr::sizeMut", vec![(pred!(mut, [], []), _)]),
	int** cv_SparseMat_Hdr_propSize(cv::SparseMat::Hdr* instance) {
			int(*ret)[32] = &instance->size;
			return (int**)ret;
	}

	// cv::SparseMat::Hdr::delete() generated
	// ("cv::SparseMat::Hdr::delete", vec![(pred!(mut, [], []), _)]),
	void cv_SparseMat_Hdr_delete(cv::SparseMat::Hdr* instance) {
			delete instance;
	}

	// cv::SparseMat::Node::defaultNew() generated
	// ("cv::SparseMat::Node::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::SparseMat::Node* cv_SparseMat_Node_defaultNew_const() {
			cv::SparseMat::Node* ret = new cv::SparseMat::Node();
			return ret;
	}

	// cv::SparseMat::Node::hashval() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2761
	// ("cv::SparseMat::Node::hashval", vec![(pred!(const, [], []), _)]),
	size_t cv_SparseMat_Node_propHashval_const(const cv::SparseMat::Node* instance) {
			size_t ret = instance->hashval;
			return ret;
	}

	// cv::SparseMat::Node::setHashval(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2761
	// ("cv::SparseMat::Node::setHashval", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
	void cv_SparseMat_Node_propHashval_const_size_t(cv::SparseMat::Node* instance, const size_t val) {
			instance->hashval = val;
	}

	// cv::SparseMat::Node::next() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2763
	// ("cv::SparseMat::Node::next", vec![(pred!(const, [], []), _)]),
	size_t cv_SparseMat_Node_propNext_const(const cv::SparseMat::Node* instance) {
			size_t ret = instance->next;
			return ret;
	}

	// cv::SparseMat::Node::setNext(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2763
	// ("cv::SparseMat::Node::setNext", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
	void cv_SparseMat_Node_propNext_const_size_t(cv::SparseMat::Node* instance, const size_t val) {
			instance->next = val;
	}

	// cv::SparseMat::Node::idx() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2765
	// ("cv::SparseMat::Node::idx", vec![(pred!(const, [], []), _)]),
	const int** cv_SparseMat_Node_propIdx_const(const cv::SparseMat::Node* instance) {
			const int(*ret)[32] = &instance->idx;
			return (const int**)ret;
	}

	// cv::SparseMat::Node::idxMut() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2765
	// ("cv::SparseMat::Node::idxMut", vec![(pred!(mut, [], []), _)]),
	int** cv_SparseMat_Node_propIdx(cv::SparseMat::Node* instance) {
			int(*ret)[32] = &instance->idx;
			return (int**)ret;
	}

	// cv::SparseMat::Node::delete() generated
	// ("cv::SparseMat::Node::delete", vec![(pred!(mut, [], []), _)]),
	void cv_SparseMat_Node_delete(cv::SparseMat::Node* instance) {
			delete instance;
	}

	// SparseMatConstIterator()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3262
	// ("cv::SparseMatConstIterator::SparseMatConstIterator", vec![(pred!(mut, [], []), _)]),
	void cv_SparseMatConstIterator_SparseMatConstIterator(Result<cv::SparseMatConstIterator*>* ocvrs_return) {
		try {
			cv::SparseMatConstIterator* ret = new cv::SparseMatConstIterator();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// SparseMatConstIterator(const SparseMat *)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3264
	// ("cv::SparseMatConstIterator::SparseMatConstIterator", vec![(pred!(mut, ["_m"], ["const cv::SparseMat*"]), _)]),
	void cv_SparseMatConstIterator_SparseMatConstIterator_const_SparseMatX(const cv::SparseMat* _m, Result<cv::SparseMatConstIterator*>* ocvrs_return) {
		try {
			cv::SparseMatConstIterator* ret = new cv::SparseMatConstIterator(_m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// SparseMatConstIterator(const SparseMatConstIterator &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3266
	// ("cv::SparseMatConstIterator::SparseMatConstIterator", vec![(pred!(mut, ["it"], ["const cv::SparseMatConstIterator*"]), _)]),
	void cv_SparseMatConstIterator_SparseMatConstIterator_const_SparseMatConstIteratorR(const cv::SparseMatConstIterator* it, Result<cv::SparseMatConstIterator*>* ocvrs_return) {
		try {
			cv::SparseMatConstIterator* ret = new cv::SparseMatConstIterator(*it);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const SparseMatConstIterator &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3269
	// ("cv::SparseMatConstIterator::operator=", vec![(pred!(mut, ["it"], ["const cv::SparseMatConstIterator*"]), _)]),
	void cv_SparseMatConstIterator_operatorST_const_SparseMatConstIteratorR(cv::SparseMatConstIterator* instance, const cv::SparseMatConstIterator* it, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*it);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// node()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3274
	// ("cv::SparseMatConstIterator::node", vec![(pred!(const, [], []), _)]),
	void cv_SparseMatConstIterator_node_const(const cv::SparseMatConstIterator* instance, Result<const cv::SparseMat::Node*>* ocvrs_return) {
		try {
			const cv::SparseMat::Node* ret = instance->node();
			Ok(new const cv::SparseMat::Node(*ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator++()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3281
	// ("cv::SparseMatConstIterator::operator++", vec![(pred!(mut, [], []), _)]),
	void cv_SparseMatConstIterator_operatorAA(cv::SparseMatConstIterator* instance, Result<cv::SparseMatConstIterator*>* ocvrs_return) {
		try {
			cv::SparseMatConstIterator ret = instance->operator++();
			Ok(new cv::SparseMatConstIterator(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// seekEnd()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3286
	// ("cv::SparseMatConstIterator::seekEnd", vec![(pred!(mut, [], []), _)]),
	void cv_SparseMatConstIterator_seekEnd(cv::SparseMatConstIterator* instance, ResultVoid* ocvrs_return) {
		try {
			instance->seekEnd();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SparseMatConstIterator::m() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3288
	// ("cv::SparseMatConstIterator::m", vec![(pred!(const, [], []), _)]),
	const cv::SparseMat* cv_SparseMatConstIterator_propM_const(const cv::SparseMatConstIterator* instance) {
			const cv::SparseMat* ret = instance->m;
			return new const cv::SparseMat(*ret);
	}

	// cv::SparseMatConstIterator::hashidx() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3289
	// ("cv::SparseMatConstIterator::hashidx", vec![(pred!(const, [], []), _)]),
	size_t cv_SparseMatConstIterator_propHashidx_const(const cv::SparseMatConstIterator* instance) {
			size_t ret = instance->hashidx;
			return ret;
	}

	// cv::SparseMatConstIterator::setHashidx(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3289
	// ("cv::SparseMatConstIterator::setHashidx", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
	void cv_SparseMatConstIterator_propHashidx_const_size_t(cv::SparseMatConstIterator* instance, const size_t val) {
			instance->hashidx = val;
	}

	// cv::SparseMatConstIterator::ptr() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3290
	// ("cv::SparseMatConstIterator::ptr", vec![(pred!(const, [], []), _)]),
	unsigned char* cv_SparseMatConstIterator_propPtr_const(const cv::SparseMatConstIterator* instance) {
			unsigned char* const ret = instance->ptr;
			return ret;
	}

	// cv::SparseMatConstIterator::ptrMut() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3290
	// ("cv::SparseMatConstIterator::ptrMut", vec![(pred!(mut, [], []), _)]),
	unsigned char* cv_SparseMatConstIterator_propPtr(cv::SparseMatConstIterator* instance) {
			unsigned char* ret = instance->ptr;
			return ret;
	}

	// cv::SparseMatConstIterator::setPtr(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3290
	// ("cv::SparseMatConstIterator::setPtr", vec![(pred!(mut, ["val"], ["unsigned char*"]), _)]),
	void cv_SparseMatConstIterator_propPtr_unsigned_charX(cv::SparseMatConstIterator* instance, unsigned char* const val) {
			instance->ptr = val;
	}

	// cv::SparseMatConstIterator::delete() generated
	// ("cv::SparseMatConstIterator::delete", vec![(pred!(mut, [], []), _)]),
	void cv_SparseMatConstIterator_delete(cv::SparseMatConstIterator* instance) {
			delete instance;
	}

	// SparseMatIterator()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3306
	// ("cv::SparseMatIterator::SparseMatIterator", vec![(pred!(mut, [], []), _)]),
	void cv_SparseMatIterator_SparseMatIterator(Result<cv::SparseMatIterator*>* ocvrs_return) {
		try {
			cv::SparseMatIterator* ret = new cv::SparseMatIterator();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// SparseMatIterator(SparseMat *)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3308
	// ("cv::SparseMatIterator::SparseMatIterator", vec![(pred!(mut, ["_m"], ["cv::SparseMat*"]), _)]),
	void cv_SparseMatIterator_SparseMatIterator_SparseMatX(cv::SparseMat* _m, Result<cv::SparseMatIterator*>* ocvrs_return) {
		try {
			cv::SparseMatIterator* ret = new cv::SparseMatIterator(_m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// SparseMatIterator(const SparseMatIterator &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3312
	// ("cv::SparseMatIterator::SparseMatIterator", vec![(pred!(mut, ["it"], ["const cv::SparseMatIterator*"]), _)]),
	void cv_SparseMatIterator_SparseMatIterator_const_SparseMatIteratorR(const cv::SparseMatIterator* it, Result<cv::SparseMatIterator*>* ocvrs_return) {
		try {
			cv::SparseMatIterator* ret = new cv::SparseMatIterator(*it);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const SparseMatIterator &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3315
	// ("cv::SparseMatIterator::operator=", vec![(pred!(mut, ["it"], ["const cv::SparseMatIterator*"]), _)]),
	void cv_SparseMatIterator_operatorST_const_SparseMatIteratorR(cv::SparseMatIterator* instance, const cv::SparseMatIterator* it, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*it);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// node()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3319
	// ("cv::SparseMatIterator::node", vec![(pred!(const, [], []), _)]),
	void cv_SparseMatIterator_node_const(const cv::SparseMatIterator* instance, Result<cv::SparseMat::Node*>* ocvrs_return) {
		try {
			cv::SparseMat::Node* ret = instance->node();
			Ok(new cv::SparseMat::Node(*ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator++()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:3322
	// ("cv::SparseMatIterator::operator++", vec![(pred!(mut, [], []), _)]),
	void cv_SparseMatIterator_operatorAA(cv::SparseMatIterator* instance, Result<cv::SparseMatIterator*>* ocvrs_return) {
		try {
			cv::SparseMatIterator ret = instance->operator++();
			Ok(new cv::SparseMatIterator(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SparseMatIterator::to_SparseMatConstIterator() generated
	// ("cv::SparseMatIterator::to_SparseMatConstIterator", vec![(pred!(mut, [], []), _)]),
	cv::SparseMatConstIterator* cv_SparseMatIterator_to_SparseMatConstIterator(cv::SparseMatIterator* instance) {
			return dynamic_cast<cv::SparseMatConstIterator*>(instance);
	}

	// cv::SparseMatIterator::delete() generated
	// ("cv::SparseMatIterator::delete", vec![(pred!(mut, [], []), _)]),
	void cv_SparseMatIterator_delete(cv::SparseMatIterator* instance) {
			delete instance;
	}

	// TermCriteria()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:854
	// ("cv::TermCriteria::TermCriteria", vec![(pred!(mut, [], []), _)]),
	void cv_TermCriteria_TermCriteria(Result<cv::TermCriteria>* ocvrs_return) {
		try {
			cv::TermCriteria ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// TermCriteria(int, int, double)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:860
	// ("cv::TermCriteria::TermCriteria", vec![(pred!(mut, ["type", "maxCount", "epsilon"], ["int", "int", "double"]), _)]),
	void cv_TermCriteria_TermCriteria_int_int_double(int type, int maxCount, double epsilon, Result<cv::TermCriteria>* ocvrs_return) {
		try {
			cv::TermCriteria ret(type, maxCount, epsilon);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isValid()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/types.hpp:862
	// ("cv::TermCriteria::isValid", vec![(pred!(const, [], []), _)]),
	void cv_TermCriteria_isValid_const(const cv::TermCriteria* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isValid();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// TickMeter()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:296
	// ("cv::TickMeter::TickMeter", vec![(pred!(mut, [], []), _)]),
	void cv_TickMeter_TickMeter(Result<cv::TickMeter*>* ocvrs_return) {
		try {
			cv::TickMeter* ret = new cv::TickMeter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// start()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:302
	// ("cv::TickMeter::start", vec![(pred!(mut, [], []), _)]),
	void cv_TickMeter_start(cv::TickMeter* instance, ResultVoid* ocvrs_return) {
		try {
			instance->start();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stop()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:308
	// ("cv::TickMeter::stop", vec![(pred!(mut, [], []), _)]),
	void cv_TickMeter_stop(cv::TickMeter* instance, ResultVoid* ocvrs_return) {
		try {
			instance->stop();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTimeTicks()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:319
	// ("cv::TickMeter::getTimeTicks", vec![(pred!(const, [], []), _)]),
	void cv_TickMeter_getTimeTicks_const(const cv::TickMeter* instance, Result<int64_t>* ocvrs_return) {
		try {
			int64_t ret = instance->getTimeTicks();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTimeMicro()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:325
	// ("cv::TickMeter::getTimeMicro", vec![(pred!(const, [], []), _)]),
	void cv_TickMeter_getTimeMicro_const(const cv::TickMeter* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getTimeMicro();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTimeMilli()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:331
	// ("cv::TickMeter::getTimeMilli", vec![(pred!(const, [], []), _)]),
	void cv_TickMeter_getTimeMilli_const(const cv::TickMeter* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getTimeMilli();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTimeSec()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:337
	// ("cv::TickMeter::getTimeSec", vec![(pred!(const, [], []), _)]),
	void cv_TickMeter_getTimeSec_const(const cv::TickMeter* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getTimeSec();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCounter()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:343
	// ("cv::TickMeter::getCounter", vec![(pred!(const, [], []), _)]),
	void cv_TickMeter_getCounter_const(const cv::TickMeter* instance, Result<int64_t>* ocvrs_return) {
		try {
			int64_t ret = instance->getCounter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFPS()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:349
	// ("cv::TickMeter::getFPS", vec![(pred!(const, [], []), _)]),
	void cv_TickMeter_getFPS_const(const cv::TickMeter* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getFPS();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAvgTimeSec()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:358
	// ("cv::TickMeter::getAvgTimeSec", vec![(pred!(const, [], []), _)]),
	void cv_TickMeter_getAvgTimeSec_const(const cv::TickMeter* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getAvgTimeSec();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAvgTimeMilli()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:366
	// ("cv::TickMeter::getAvgTimeMilli", vec![(pred!(const, [], []), _)]),
	void cv_TickMeter_getAvgTimeMilli_const(const cv::TickMeter* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getAvgTimeMilli();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reset()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utility.hpp:372
	// ("cv::TickMeter::reset", vec![(pred!(mut, [], []), _)]),
	void cv_TickMeter_reset(cv::TickMeter* instance, ResultVoid* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TickMeter::delete() generated
	// ("cv::TickMeter::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TickMeter_delete(cv::TickMeter* instance) {
			delete instance;
	}

	// UMat(UMatUsageFlags)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2453
	// ("cv::UMat::UMat", vec![(pred!(mut, ["usageFlags"], ["cv::UMatUsageFlags"]), _)]),
	cv::UMat* cv_UMat_UMat_UMatUsageFlags(cv::UMatUsageFlags usageFlags) {
			cv::UMat* ret = new cv::UMat(usageFlags);
			return ret;
	}

	// cv::UMat::UMat() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2453
	// ("cv::UMat::UMat", vec![(pred!(mut, [], []), _)]),
	cv::UMat* cv_UMat_UMat() {
			cv::UMat* ret = new cv::UMat();
			return ret;
	}

	// UMat(int, int, int, UMatUsageFlags)(Primitive, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2456
	// ("cv::UMat::UMat", vec![(pred!(mut, ["rows", "cols", "type", "usageFlags"], ["int", "int", "int", "cv::UMatUsageFlags"]), _)]),
	void cv_UMat_UMat_int_int_int_UMatUsageFlags(int rows, int cols, int type, cv::UMatUsageFlags usageFlags, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat* ret = new cv::UMat(rows, cols, type, usageFlags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::UMat(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2456
	// ("cv::UMat::UMat", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
	void cv_UMat_UMat_int_int_int(int rows, int cols, int type, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat* ret = new cv::UMat(rows, cols, type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// UMat(Size, int, UMatUsageFlags)(SimpleClass, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2457
	// ("cv::UMat::UMat", vec![(pred!(mut, ["size", "type", "usageFlags"], ["cv::Size", "int", "cv::UMatUsageFlags"]), _)]),
	void cv_UMat_UMat_Size_int_UMatUsageFlags(cv::Size* size, int type, cv::UMatUsageFlags usageFlags, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat* ret = new cv::UMat(*size, type, usageFlags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::UMat(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2457
	// ("cv::UMat::UMat", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
	void cv_UMat_UMat_Size_int(cv::Size* size, int type, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat* ret = new cv::UMat(*size, type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// UMat(int, int, int, const Scalar &, UMatUsageFlags)(Primitive, Primitive, Primitive, SimpleClass, Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2459
	// ("cv::UMat::UMat", vec![(pred!(mut, ["rows", "cols", "type", "s", "usageFlags"], ["int", "int", "int", "const cv::Scalar*", "cv::UMatUsageFlags"]), _)]),
	void cv_UMat_UMat_int_int_int_const_ScalarR_UMatUsageFlags(int rows, int cols, int type, const cv::Scalar* s, cv::UMatUsageFlags usageFlags, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat* ret = new cv::UMat(rows, cols, type, *s, usageFlags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::UMat(Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2459
	// ("cv::UMat::UMat", vec![(pred!(mut, ["rows", "cols", "type", "s"], ["int", "int", "int", "const cv::Scalar*"]), _)]),
	void cv_UMat_UMat_int_int_int_const_ScalarR(int rows, int cols, int type, const cv::Scalar* s, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat* ret = new cv::UMat(rows, cols, type, *s);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// UMat(Size, int, const Scalar &, UMatUsageFlags)(SimpleClass, Primitive, SimpleClass, Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2460
	// ("cv::UMat::UMat", vec![(pred!(mut, ["size", "type", "s", "usageFlags"], ["cv::Size", "int", "const cv::Scalar*", "cv::UMatUsageFlags"]), _)]),
	void cv_UMat_UMat_Size_int_const_ScalarR_UMatUsageFlags(cv::Size* size, int type, const cv::Scalar* s, cv::UMatUsageFlags usageFlags, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat* ret = new cv::UMat(*size, type, *s, usageFlags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::UMat(SimpleClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2460
	// ("cv::UMat::UMat", vec![(pred!(mut, ["size", "type", "s"], ["cv::Size", "int", "const cv::Scalar*"]), _)]),
	void cv_UMat_UMat_Size_int_const_ScalarR(cv::Size* size, int type, const cv::Scalar* s, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat* ret = new cv::UMat(*size, type, *s);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// UMat(int, const int *, int, UMatUsageFlags)(Primitive, VariableArray, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2463
	// ("cv::UMat::UMat", vec![(pred!(mut, ["ndims", "sizes", "type", "usageFlags"], ["int", "const int*", "int", "cv::UMatUsageFlags"]), _)]),
	void cv_UMat_UMat_int_const_intX_int_UMatUsageFlags(int ndims, const int* sizes, int type, cv::UMatUsageFlags usageFlags, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat* ret = new cv::UMat(ndims, sizes, type, usageFlags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::UMat(Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2463
	// ("cv::UMat::UMat", vec![(pred!(mut, ["ndims", "sizes", "type"], ["int", "const int*", "int"]), _)]),
	void cv_UMat_UMat_int_const_intX_int(int ndims, const int* sizes, int type, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat* ret = new cv::UMat(ndims, sizes, type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// UMat(int, const int *, int, const Scalar &, UMatUsageFlags)(Primitive, VariableArray, Primitive, SimpleClass, Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2464
	// ("cv::UMat::UMat", vec![(pred!(mut, ["ndims", "sizes", "type", "s", "usageFlags"], ["int", "const int*", "int", "const cv::Scalar*", "cv::UMatUsageFlags"]), _)]),
	void cv_UMat_UMat_int_const_intX_int_const_ScalarR_UMatUsageFlags(int ndims, const int* sizes, int type, const cv::Scalar* s, cv::UMatUsageFlags usageFlags, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat* ret = new cv::UMat(ndims, sizes, type, *s, usageFlags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::UMat(Primitive, VariableArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2464
	// ("cv::UMat::UMat", vec![(pred!(mut, ["ndims", "sizes", "type", "s"], ["int", "const int*", "int", "const cv::Scalar*"]), _)]),
	void cv_UMat_UMat_int_const_intX_int_const_ScalarR(int ndims, const int* sizes, int type, const cv::Scalar* s, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat* ret = new cv::UMat(ndims, sizes, type, *s);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// UMat(const UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2467
	// ("cv::UMat::UMat", vec![(pred!(mut, ["m"], ["const cv::UMat*"]), _)]),
	void cv_UMat_UMat_const_UMatR(const cv::UMat* m, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat* ret = new cv::UMat(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// UMat(const UMat &, const Range &, const Range &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2470
	// ("cv::UMat::UMat", vec![(pred!(mut, ["m", "rowRange", "colRange"], ["const cv::UMat*", "const cv::Range*", "const cv::Range*"]), _)]),
	void cv_UMat_UMat_const_UMatR_const_RangeR_const_RangeR(const cv::UMat* m, const cv::Range* rowRange, const cv::Range* colRange, Result<cv::UMat*>* ocvrs_return) {
		try {
			const cv::UMat* ret = new const cv::UMat(*m, *rowRange, *colRange);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::UMat(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2470
	// ("cv::UMat::UMat", vec![(pred!(mut, ["m", "rowRange"], ["cv::UMat*", "const cv::Range*"]), _)]),
	void cv_UMat_UMat_UMatR_const_RangeR(cv::UMat* m, const cv::Range* rowRange, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat* ret = new cv::UMat(*m, *rowRange);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::UMat(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2470
	// ("cv::UMat::UMat", vec![(pred!(mut, ["m", "rowRange"], ["const cv::UMat*", "const cv::Range*"]), _)]),
	void cv_UMat_UMat_const_UMatR_const_RangeR(const cv::UMat* m, const cv::Range* rowRange, Result<cv::UMat*>* ocvrs_return) {
		try {
			const cv::UMat* ret = new const cv::UMat(*m, *rowRange);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::UMat(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2470
	// ("cv::UMat::UMat", vec![(pred!(mut, ["m", "rowRange", "colRange"], ["cv::UMat*", "const cv::Range*", "const cv::Range*"]), _)]),
	void cv_UMat_UMat_UMatR_const_RangeR_const_RangeR(cv::UMat* m, const cv::Range* rowRange, const cv::Range* colRange, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat* ret = new cv::UMat(*m, *rowRange, *colRange);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// UMat(const UMat &, const Rect &)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2471
	// ("cv::UMat::UMat", vec![(pred!(mut, ["m", "roi"], ["const cv::UMat*", "const cv::Rect*"]), _)]),
	void cv_UMat_UMat_const_UMatR_const_RectR(const cv::UMat* m, const cv::Rect* roi, Result<cv::UMat*>* ocvrs_return) {
		try {
			const cv::UMat* ret = new const cv::UMat(*m, *roi);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::UMat(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2471
	// ("cv::UMat::UMat", vec![(pred!(mut, ["m", "roi"], ["cv::UMat*", "const cv::Rect*"]), _)]),
	void cv_UMat_UMat_UMatR_const_RectR(cv::UMat* m, const cv::Rect* roi, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat* ret = new cv::UMat(*m, *roi);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// UMat(const UMat &, const std::vector<Range> &)(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2473
	// ("cv::UMat::UMat", vec![(pred!(mut, ["m", "ranges"], ["const cv::UMat*", "const std::vector<cv::Range>*"]), _)]),
	void cv_UMat_UMat_const_UMatR_const_vectorLRangeGR(const cv::UMat* m, const std::vector<cv::Range>* ranges, Result<cv::UMat*>* ocvrs_return) {
		try {
			const cv::UMat* ret = new const cv::UMat(*m, *ranges);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::UMat(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2473
	// ("cv::UMat::UMat", vec![(pred!(mut, ["m", "ranges"], ["cv::UMat*", "const std::vector<cv::Range>*"]), _)]),
	void cv_UMat_UMat_UMatR_const_vectorLRangeGR(cv::UMat* m, const std::vector<cv::Range>* ranges, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat* ret = new cv::UMat(*m, *ranges);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2482
	// ("cv::UMat::operator=", vec![(pred!(mut, ["m"], ["const cv::UMat*"]), _)]),
	void cv_UMat_operatorST_const_UMatR(cv::UMat* instance, const cv::UMat* m, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMat(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2484
	// ("cv::UMat::getMat", vec![(pred!(const, ["flags"], ["int"]), _)]),
	void cv_UMat_getMat_const_int(const cv::UMat* instance, int flags, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getMat(flags);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// row(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2487
	// ("cv::UMat::row", vec![(pred!(const, ["y"], ["int"]), _)]),
	void cv_UMat_row_const_int(const cv::UMat* instance, int y, Result<cv::UMat*>* ocvrs_return) {
		try {
			const cv::UMat ret = instance->row(y);
			Ok(new const cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::row(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2487
	// ("cv::UMat::row", vec![(pred!(mut, ["y"], ["int"]), _)]),
	void cv_UMat_row_int(cv::UMat* instance, int y, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->row(y);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// col(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2489
	// ("cv::UMat::col", vec![(pred!(const, ["x"], ["int"]), _)]),
	void cv_UMat_col_const_int(const cv::UMat* instance, int x, Result<cv::UMat*>* ocvrs_return) {
		try {
			const cv::UMat ret = instance->col(x);
			Ok(new const cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::col(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2489
	// ("cv::UMat::col", vec![(pred!(mut, ["x"], ["int"]), _)]),
	void cv_UMat_col_int(cv::UMat* instance, int x, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->col(x);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// rowRange(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2491
	// ("cv::UMat::rowRange", vec![(pred!(const, ["startrow", "endrow"], ["int", "int"]), _)]),
	void cv_UMat_rowRange_const_int_int(const cv::UMat* instance, int startrow, int endrow, Result<cv::UMat*>* ocvrs_return) {
		try {
			const cv::UMat ret = instance->rowRange(startrow, endrow);
			Ok(new const cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::rowRange(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2491
	// ("cv::UMat::rowRange", vec![(pred!(mut, ["startrow", "endrow"], ["int", "int"]), _)]),
	void cv_UMat_rowRange_int_int(cv::UMat* instance, int startrow, int endrow, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->rowRange(startrow, endrow);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// rowRange(const Range &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2492
	// ("cv::UMat::rowRange", vec![(pred!(const, ["r"], ["const cv::Range*"]), _)]),
	void cv_UMat_rowRange_const_const_RangeR(const cv::UMat* instance, const cv::Range* r, Result<cv::UMat*>* ocvrs_return) {
		try {
			const cv::UMat ret = instance->rowRange(*r);
			Ok(new const cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::rowRange(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2492
	// ("cv::UMat::rowRange", vec![(pred!(mut, ["r"], ["const cv::Range*"]), _)]),
	void cv_UMat_rowRange_const_RangeR(cv::UMat* instance, const cv::Range* r, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->rowRange(*r);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// colRange(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2494
	// ("cv::UMat::colRange", vec![(pred!(const, ["startcol", "endcol"], ["int", "int"]), _)]),
	void cv_UMat_colRange_const_int_int(const cv::UMat* instance, int startcol, int endcol, Result<cv::UMat*>* ocvrs_return) {
		try {
			const cv::UMat ret = instance->colRange(startcol, endcol);
			Ok(new const cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::colRange(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2494
	// ("cv::UMat::colRange", vec![(pred!(mut, ["startcol", "endcol"], ["int", "int"]), _)]),
	void cv_UMat_colRange_int_int(cv::UMat* instance, int startcol, int endcol, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->colRange(startcol, endcol);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// colRange(const Range &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2495
	// ("cv::UMat::colRange", vec![(pred!(const, ["r"], ["const cv::Range*"]), _)]),
	void cv_UMat_colRange_const_const_RangeR(const cv::UMat* instance, const cv::Range* r, Result<cv::UMat*>* ocvrs_return) {
		try {
			const cv::UMat ret = instance->colRange(*r);
			Ok(new const cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::colRange(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2495
	// ("cv::UMat::colRange", vec![(pred!(mut, ["r"], ["const cv::Range*"]), _)]),
	void cv_UMat_colRange_const_RangeR(cv::UMat* instance, const cv::Range* r, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->colRange(*r);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// diag(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2500
	// ("cv::UMat::diag", vec![(pred!(const, ["d"], ["int"]), _)]),
	void cv_UMat_diag_const_int(const cv::UMat* instance, int d, Result<cv::UMat*>* ocvrs_return) {
		try {
			const cv::UMat ret = instance->diag(d);
			Ok(new const cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::diag() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2500
	// ("cv::UMat::diag", vec![(pred!(mut, [], []), _)]),
	void cv_UMat_diag(cv::UMat* instance, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->diag();
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::diag() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2500
	// ("cv::UMat::diag", vec![(pred!(const, [], []), _)]),
	void cv_UMat_diag_const(const cv::UMat* instance, Result<cv::UMat*>* ocvrs_return) {
		try {
			const cv::UMat ret = instance->diag();
			Ok(new const cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::diag(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2500
	// ("cv::UMat::diag", vec![(pred!(mut, ["d"], ["int"]), _)]),
	void cv_UMat_diag_int(cv::UMat* instance, int d, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->diag(d);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// diag(const UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2502
	// ("cv::UMat::diag", vec![(pred!(mut, ["d"], ["const cv::UMat*"]), _)]),
	void cv_UMat_diag_const_UMatR(const cv::UMat* d, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = cv::UMat::diag(*d);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clone()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2505
	// ("cv::UMat::clone", vec![(pred!(const, [], []), _)]),
	void cv_UMat_clone_const(const cv::UMat* instance, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->clone();
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// copyTo(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2508
	// ("cv::UMat::copyTo", vec![(pred!(const, ["m"], ["const cv::_OutputArray*"]), _)]),
	void cv_UMat_copyTo_const_const__OutputArrayR(const cv::UMat* instance, const cv::_OutputArray* m, ResultVoid* ocvrs_return) {
		try {
			instance->copyTo(*m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// copyTo(OutputArray, InputArray)(OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2510
	// ("cv::UMat::copyTo", vec![(pred!(const, ["m", "mask"], ["const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_UMat_copyTo_const_const__OutputArrayR_const__InputArrayR(const cv::UMat* instance, const cv::_OutputArray* m, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			instance->copyTo(*m, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertTo(OutputArray, int, double, double)(OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2512
	// ("cv::UMat::convertTo", vec![(pred!(const, ["m", "rtype", "alpha", "beta"], ["const cv::_OutputArray*", "int", "double", "double"]), _)]),
	void cv_UMat_convertTo_const_const__OutputArrayR_int_double_double(const cv::UMat* instance, const cv::_OutputArray* m, int rtype, double alpha, double beta, ResultVoid* ocvrs_return) {
		try {
			instance->convertTo(*m, rtype, alpha, beta);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::convertTo(OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2512
	// ("cv::UMat::convertTo", vec![(pred!(const, ["m", "rtype"], ["const cv::_OutputArray*", "int"]), _)]),
	void cv_UMat_convertTo_const_const__OutputArrayR_int(const cv::UMat* instance, const cv::_OutputArray* m, int rtype, ResultVoid* ocvrs_return) {
		try {
			instance->convertTo(*m, rtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// assignTo(UMat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2514
	// ("cv::UMat::assignTo", vec![(pred!(const, ["m", "type"], ["cv::UMat*", "int"]), _)]),
	void cv_UMat_assignTo_const_UMatR_int(const cv::UMat* instance, cv::UMat* m, int type, ResultVoid* ocvrs_return) {
		try {
			instance->assignTo(*m, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::assignTo(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2514
	// ("cv::UMat::assignTo", vec![(pred!(const, ["m"], ["cv::UMat*"]), _)]),
	void cv_UMat_assignTo_const_UMatR(const cv::UMat* instance, cv::UMat* m, ResultVoid* ocvrs_return) {
		try {
			instance->assignTo(*m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const Scalar &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2517
	// ("cv::UMat::operator=", vec![(pred!(mut, ["s"], ["const cv::Scalar*"]), _)]),
	void cv_UMat_operatorST_const_ScalarR(cv::UMat* instance, const cv::Scalar* s, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*s);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTo(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2519
	// ("cv::UMat::setTo", vec![(pred!(mut, ["value", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_UMat_setTo_const__InputArrayR_const__InputArrayR(cv::UMat* instance, const cv::_InputArray* value, const cv::_InputArray* mask, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->setTo(*value, *mask);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::setTo(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2519
	// ("cv::UMat::setTo", vec![(pred!(mut, ["value"], ["const cv::_InputArray*"]), _)]),
	void cv_UMat_setTo_const__InputArrayR(cv::UMat* instance, const cv::_InputArray* value, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->setTo(*value);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reshape(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2522
	// ("cv::UMat::reshape", vec![(pred!(const, ["cn", "rows"], ["int", "int"]), _)]),
	void cv_UMat_reshape_const_int_int(const cv::UMat* instance, int cn, int rows, Result<cv::UMat*>* ocvrs_return) {
		try {
			const cv::UMat ret = instance->reshape(cn, rows);
			Ok(new const cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::reshape(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2522
	// ("cv::UMat::reshape", vec![(pred!(mut, ["cn"], ["int"]), _)]),
	void cv_UMat_reshape_int(cv::UMat* instance, int cn, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->reshape(cn);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::reshape(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2522
	// ("cv::UMat::reshape", vec![(pred!(const, ["cn"], ["int"]), _)]),
	void cv_UMat_reshape_const_int(const cv::UMat* instance, int cn, Result<cv::UMat*>* ocvrs_return) {
		try {
			const cv::UMat ret = instance->reshape(cn);
			Ok(new const cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::reshape(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2522
	// ("cv::UMat::reshape", vec![(pred!(mut, ["cn", "rows"], ["int", "int"]), _)]),
	void cv_UMat_reshape_int_int(cv::UMat* instance, int cn, int rows, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->reshape(cn, rows);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reshape(int, int, const int *)(Primitive, Primitive, VariableArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2523
	// ("cv::UMat::reshape", vec![(pred!(const, ["cn", "newndims", "newsz"], ["int", "int", "const int*"]), _)]),
	void cv_UMat_reshape_const_int_int_const_intX(const cv::UMat* instance, int cn, int newndims, const int* newsz, Result<cv::UMat*>* ocvrs_return) {
		try {
			const cv::UMat ret = instance->reshape(cn, newndims, newsz);
			Ok(new const cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::reshape(Primitive, Primitive, VariableArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2523
	// ("cv::UMat::reshape", vec![(pred!(mut, ["cn", "newndims", "newsz"], ["int", "int", "const int*"]), _)]),
	void cv_UMat_reshape_int_int_const_intX(cv::UMat* instance, int cn, int newndims, const int* newsz, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->reshape(cn, newndims, newsz);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// t()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2526
	// ("cv::UMat::t", vec![(pred!(const, [], []), _)]),
	void cv_UMat_t_const(const cv::UMat* instance, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->t();
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// inv(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2528
	// ("cv::UMat::inv", vec![(pred!(const, ["method"], ["int"]), _)]),
	void cv_UMat_inv_const_int(const cv::UMat* instance, int method, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->inv(method);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::inv() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2528
	// ("cv::UMat::inv", vec![(pred!(const, [], []), _)]),
	void cv_UMat_inv_const(const cv::UMat* instance, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->inv();
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mul(InputArray, double)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2530
	// ("cv::UMat::mul", vec![(pred!(const, ["m", "scale"], ["const cv::_InputArray*", "double"]), _)]),
	void cv_UMat_mul_const_const__InputArrayR_double(const cv::UMat* instance, const cv::_InputArray* m, double scale, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->mul(*m, scale);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::mul(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2530
	// ("cv::UMat::mul", vec![(pred!(const, ["m"], ["const cv::_InputArray*"]), _)]),
	void cv_UMat_mul_const_const__InputArrayR(const cv::UMat* instance, const cv::_InputArray* m, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->mul(*m);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dot(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2533
	// ("cv::UMat::dot", vec![(pred!(const, ["m"], ["const cv::_InputArray*"]), _)]),
	void cv_UMat_dot_const_const__InputArrayR(const cv::UMat* instance, const cv::_InputArray* m, Result<double>* ocvrs_return) {
		try {
			double ret = instance->dot(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// zeros(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2536
	// ("cv::UMat::zeros", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
	void cv_UMat_zeros_int_int_int(int rows, int cols, int type, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = cv::UMat::zeros(rows, cols, type);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// zeros(Size, int)(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2537
	// ("cv::UMat::zeros", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
	void cv_UMat_zeros_Size_int(cv::Size* size, int type, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = cv::UMat::zeros(*size, type);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// zeros(int, const int *, int)(Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2538
	// ("cv::UMat::zeros", vec![(pred!(mut, ["ndims", "sz", "type"], ["int", "const int*", "int"]), _)]),
	void cv_UMat_zeros_int_const_intX_int(int ndims, const int* sz, int type, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = cv::UMat::zeros(ndims, sz, type);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ones(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2539
	// ("cv::UMat::ones", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
	void cv_UMat_ones_int_int_int(int rows, int cols, int type, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = cv::UMat::ones(rows, cols, type);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ones(Size, int)(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2540
	// ("cv::UMat::ones", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
	void cv_UMat_ones_Size_int(cv::Size* size, int type, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = cv::UMat::ones(*size, type);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ones(int, const int *, int)(Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2541
	// ("cv::UMat::ones", vec![(pred!(mut, ["ndims", "sz", "type"], ["int", "const int*", "int"]), _)]),
	void cv_UMat_ones_int_const_intX_int(int ndims, const int* sz, int type, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = cv::UMat::ones(ndims, sz, type);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// eye(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2542
	// ("cv::UMat::eye", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
	void cv_UMat_eye_int_int_int(int rows, int cols, int type, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = cv::UMat::eye(rows, cols, type);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// eye(Size, int)(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2543
	// ("cv::UMat::eye", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
	void cv_UMat_eye_Size_int(cv::Size* size, int type, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = cv::UMat::eye(*size, type);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, int, int, UMatUsageFlags)(Primitive, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2547
	// ("cv::UMat::create", vec![(pred!(mut, ["rows", "cols", "type", "usageFlags"], ["int", "int", "int", "cv::UMatUsageFlags"]), _)]),
	void cv_UMat_create_int_int_int_UMatUsageFlags(cv::UMat* instance, int rows, int cols, int type, cv::UMatUsageFlags usageFlags, ResultVoid* ocvrs_return) {
		try {
			instance->create(rows, cols, type, usageFlags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::create(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2547
	// ("cv::UMat::create", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
	void cv_UMat_create_int_int_int(cv::UMat* instance, int rows, int cols, int type, ResultVoid* ocvrs_return) {
		try {
			instance->create(rows, cols, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(Size, int, UMatUsageFlags)(SimpleClass, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2548
	// ("cv::UMat::create", vec![(pred!(mut, ["size", "type", "usageFlags"], ["cv::Size", "int", "cv::UMatUsageFlags"]), _)]),
	void cv_UMat_create_Size_int_UMatUsageFlags(cv::UMat* instance, cv::Size* size, int type, cv::UMatUsageFlags usageFlags, ResultVoid* ocvrs_return) {
		try {
			instance->create(*size, type, usageFlags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::create(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2548
	// ("cv::UMat::create", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
	void cv_UMat_create_Size_int(cv::UMat* instance, cv::Size* size, int type, ResultVoid* ocvrs_return) {
		try {
			instance->create(*size, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, const int *, int, UMatUsageFlags)(Primitive, VariableArray, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2549
	// ("cv::UMat::create", vec![(pred!(mut, ["ndims", "sizes", "type", "usageFlags"], ["int", "const int*", "int", "cv::UMatUsageFlags"]), _)]),
	void cv_UMat_create_int_const_intX_int_UMatUsageFlags(cv::UMat* instance, int ndims, const int* sizes, int type, cv::UMatUsageFlags usageFlags, ResultVoid* ocvrs_return) {
		try {
			instance->create(ndims, sizes, type, usageFlags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::create(Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2549
	// ("cv::UMat::create", vec![(pred!(mut, ["ndims", "sizes", "type"], ["int", "const int*", "int"]), _)]),
	void cv_UMat_create_int_const_intX_int(cv::UMat* instance, int ndims, const int* sizes, int type, ResultVoid* ocvrs_return) {
		try {
			instance->create(ndims, sizes, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const std::vector<int> &, int, UMatUsageFlags)(CppPassByVoidPtr, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2550
	// ("cv::UMat::create", vec![(pred!(mut, ["sizes", "type", "usageFlags"], ["const std::vector<int>*", "int", "cv::UMatUsageFlags"]), _)]),
	void cv_UMat_create_const_vectorLintGR_int_UMatUsageFlags(cv::UMat* instance, const std::vector<int>* sizes, int type, cv::UMatUsageFlags usageFlags, ResultVoid* ocvrs_return) {
		try {
			instance->create(*sizes, type, usageFlags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::create(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2550
	// ("cv::UMat::create", vec![(pred!(mut, ["sizes", "type"], ["const std::vector<int>*", "int"]), _)]),
	void cv_UMat_create_const_vectorLintGR_int(cv::UMat* instance, const std::vector<int>* sizes, int type, ResultVoid* ocvrs_return) {
		try {
			instance->create(*sizes, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addref()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2553
	// ("cv::UMat::addref", vec![(pred!(mut, [], []), _)]),
	void cv_UMat_addref(cv::UMat* instance, ResultVoid* ocvrs_return) {
		try {
			instance->addref();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// release()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2556
	// ("cv::UMat::release", vec![(pred!(mut, [], []), _)]),
	void cv_UMat_release(cv::UMat* instance, ResultVoid* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// deallocate()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2559
	// ("cv::UMat::deallocate", vec![(pred!(mut, [], []), _)]),
	void cv_UMat_deallocate(cv::UMat* instance, ResultVoid* ocvrs_return) {
		try {
			instance->deallocate();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// locateROI(Size &, Point &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2564
	// ("cv::UMat::locateROI", vec![(pred!(const, ["wholeSize", "ofs"], ["cv::Size*", "cv::Point*"]), _)]),
	void cv_UMat_locateROI_const_SizeR_PointR(const cv::UMat* instance, cv::Size* wholeSize, cv::Point* ofs, ResultVoid* ocvrs_return) {
		try {
			instance->locateROI(*wholeSize, *ofs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// adjustROI(int, int, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2566
	// ("cv::UMat::adjustROI", vec![(pred!(mut, ["dtop", "dbottom", "dleft", "dright"], ["int", "int", "int", "int"]), _)]),
	void cv_UMat_adjustROI_int_int_int_int(cv::UMat* instance, int dtop, int dbottom, int dleft, int dright, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->adjustROI(dtop, dbottom, dleft, dright);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(Range, Range)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2569
	// ("cv::UMat::operator()", vec![(pred!(const, ["rowRange", "colRange"], ["cv::Range", "cv::Range"]), _)]),
	void cv_UMat_operator___const_Range_Range(const cv::UMat* instance, cv::Range* rowRange, cv::Range* colRange, Result<cv::UMat*>* ocvrs_return) {
		try {
			const cv::UMat ret = instance->operator()(*rowRange, *colRange);
			Ok(new const cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::operator()(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2569
	// ("cv::UMat::operator()", vec![(pred!(mut, ["rowRange", "colRange"], ["cv::Range", "cv::Range"]), _)]),
	void cv_UMat_operator___Range_Range(cv::UMat* instance, cv::Range* rowRange, cv::Range* colRange, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->operator()(*rowRange, *colRange);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(const Rect &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2570
	// ("cv::UMat::operator()", vec![(pred!(const, ["roi"], ["const cv::Rect*"]), _)]),
	void cv_UMat_operator___const_const_RectR(const cv::UMat* instance, const cv::Rect* roi, Result<cv::UMat*>* ocvrs_return) {
		try {
			const cv::UMat ret = instance->operator()(*roi);
			Ok(new const cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::operator()(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2570
	// ("cv::UMat::operator()", vec![(pred!(mut, ["roi"], ["const cv::Rect*"]), _)]),
	void cv_UMat_operator___const_RectR(cv::UMat* instance, const cv::Rect* roi, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->operator()(*roi);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(const std::vector<Range> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2572
	// ("cv::UMat::operator()", vec![(pred!(const, ["ranges"], ["const std::vector<cv::Range>*"]), _)]),
	void cv_UMat_operator___const_const_vectorLRangeGR(const cv::UMat* instance, const std::vector<cv::Range>* ranges, Result<cv::UMat*>* ocvrs_return) {
		try {
			const cv::UMat ret = instance->operator()(*ranges);
			Ok(new const cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::operator()(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2572
	// ("cv::UMat::operator()", vec![(pred!(mut, ["ranges"], ["const std::vector<cv::Range>*"]), _)]),
	void cv_UMat_operator___const_vectorLRangeGR(cv::UMat* instance, const std::vector<cv::Range>* ranges, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->operator()(*ranges);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isContinuous()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2577
	// ("cv::UMat::isContinuous", vec![(pred!(const, [], []), _)]),
	bool cv_UMat_isContinuous_const(const cv::UMat* instance) {
			bool ret = instance->isContinuous();
			return ret;
	}

	// isSubmatrix()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2580
	// ("cv::UMat::isSubmatrix", vec![(pred!(const, [], []), _)]),
	bool cv_UMat_isSubmatrix_const(const cv::UMat* instance) {
			bool ret = instance->isSubmatrix();
			return ret;
	}

	// elemSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2584
	// ("cv::UMat::elemSize", vec![(pred!(const, [], []), _)]),
	void cv_UMat_elemSize_const(const cv::UMat* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->elemSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// elemSize1()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2586
	// ("cv::UMat::elemSize1", vec![(pred!(const, [], []), _)]),
	size_t cv_UMat_elemSize1_const(const cv::UMat* instance) {
			size_t ret = instance->elemSize1();
			return ret;
	}

	// type()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2588
	// ("cv::UMat::type", vec![(pred!(const, [], []), _)]),
	int cv_UMat_type_const(const cv::UMat* instance) {
			int ret = instance->type();
			return ret;
	}

	// depth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2590
	// ("cv::UMat::depth", vec![(pred!(const, [], []), _)]),
	int cv_UMat_depth_const(const cv::UMat* instance) {
			int ret = instance->depth();
			return ret;
	}

	// channels()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2592
	// ("cv::UMat::channels", vec![(pred!(const, [], []), _)]),
	int cv_UMat_channels_const(const cv::UMat* instance) {
			int ret = instance->channels();
			return ret;
	}

	// step1(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2594
	// ("cv::UMat::step1", vec![(pred!(const, ["i"], ["int"]), _)]),
	void cv_UMat_step1_const_int(const cv::UMat* instance, int i, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->step1(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::step1() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2594
	// ("cv::UMat::step1", vec![(pred!(const, [], []), _)]),
	void cv_UMat_step1_const(const cv::UMat* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->step1();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2596
	// ("cv::UMat::empty", vec![(pred!(const, [], []), _)]),
	bool cv_UMat_empty_const(const cv::UMat* instance) {
			bool ret = instance->empty();
			return ret;
	}

	// total()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2598
	// ("cv::UMat::total", vec![(pred!(const, [], []), _)]),
	size_t cv_UMat_total_const(const cv::UMat* instance) {
			size_t ret = instance->total();
			return ret;
	}

	// checkVector(int, int, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2601
	// ("cv::UMat::checkVector", vec![(pred!(const, ["elemChannels", "depth", "requireContinuous"], ["int", "int", "bool"]), _)]),
	void cv_UMat_checkVector_const_int_int_bool(const cv::UMat* instance, int elemChannels, int depth, bool requireContinuous, Result<int>* ocvrs_return) {
		try {
			int ret = instance->checkVector(elemChannels, depth, requireContinuous);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::checkVector(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2601
	// ("cv::UMat::checkVector", vec![(pred!(const, ["elemChannels"], ["int"]), _)]),
	void cv_UMat_checkVector_const_int(const cv::UMat* instance, int elemChannels, Result<int>* ocvrs_return) {
		try {
			int ret = instance->checkVector(elemChannels);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// UMat(UMat &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2604
	// ("cv::UMat::UMat", vec![(pred!(mut, ["m"], ["cv::UMat*"]), _)]),
	void cv_UMat_UMat_UMatRR(cv::UMat* m, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat* ret = new cv::UMat(std::move(*m));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(UMat &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2605
	// ("cv::UMat::operator=", vec![(pred!(mut, ["m"], ["cv::UMat*"]), _)]),
	void cv_UMat_operatorST_UMatRR(cv::UMat* instance, cv::UMat* m, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(std::move(*m));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// handle(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2612
	// ("cv::UMat::handle", vec![(pred!(const, ["accessFlags"], ["int"]), _)]),
	void cv_UMat_handle_const_int(const cv::UMat* instance, int accessFlags, Result<void*>* ocvrs_return) {
		try {
			void* ret = instance->handle(accessFlags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ndoffset(size_t *)(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2613
	// ("cv::UMat::ndoffset", vec![(pred!(const, ["ofs"], ["size_t*"]), _)]),
	void cv_UMat_ndoffset_const_size_tX(const cv::UMat* instance, size_t* ofs, ResultVoid* ocvrs_return) {
		try {
			instance->ndoffset(ofs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// updateContinuityFlag()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2637
	// ("cv::UMat::updateContinuityFlag", vec![(pred!(mut, [], []), _)]),
	void cv_UMat_updateContinuityFlag(cv::UMat* instance, ResultVoid* ocvrs_return) {
		try {
			instance->updateContinuityFlag();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::size() generated
	// ("cv::UMat::size", vec![(pred!(const, [], []), _)]),
	void cv_UMat_size_const(const cv::UMat* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->size();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMat::flags() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2624
	// ("cv::UMat::flags", vec![(pred!(const, [], []), _)]),
	int cv_UMat_propFlags_const(const cv::UMat* instance) {
			int ret = instance->flags;
			return ret;
	}

	// cv::UMat::setFlags(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2624
	// ("cv::UMat::setFlags", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_UMat_propFlags_const_int(cv::UMat* instance, const int val) {
			instance->flags = val;
	}

	// cv::UMat::dims() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2626
	// ("cv::UMat::dims", vec![(pred!(const, [], []), _)]),
	int cv_UMat_propDims_const(const cv::UMat* instance) {
			int ret = instance->dims;
			return ret;
	}

	// cv::UMat::setDims(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2626
	// ("cv::UMat::setDims", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_UMat_propDims_const_int(cv::UMat* instance, const int val) {
			instance->dims = val;
	}

	// cv::UMat::rows() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2628
	// ("cv::UMat::rows", vec![(pred!(const, [], []), _)]),
	int cv_UMat_propRows_const(const cv::UMat* instance) {
			int ret = instance->rows;
			return ret;
	}

	// cv::UMat::setRows(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2628
	// ("cv::UMat::setRows", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_UMat_propRows_const_int(cv::UMat* instance, const int val) {
			instance->rows = val;
	}

	// cv::UMat::cols() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2628
	// ("cv::UMat::cols", vec![(pred!(const, [], []), _)]),
	int cv_UMat_propCols_const(const cv::UMat* instance) {
			int ret = instance->cols;
			return ret;
	}

	// cv::UMat::setCols(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2628
	// ("cv::UMat::setCols", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_UMat_propCols_const_int(cv::UMat* instance, const int val) {
			instance->cols = val;
	}

	// cv::UMat::usageFlags() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2632
	// ("cv::UMat::usageFlags", vec![(pred!(const, [], []), _)]),
	void cv_UMat_propUsageFlags_const(const cv::UMat* instance, cv::UMatUsageFlags* ocvrs_return) {
			cv::UMatUsageFlags ret = instance->usageFlags;
			*ocvrs_return = ret;
	}

	// cv::UMat::setUsageFlags(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2632
	// ("cv::UMat::setUsageFlags", vec![(pred!(mut, ["val"], ["const cv::UMatUsageFlags"]), _)]),
	void cv_UMat_propUsageFlags_const_UMatUsageFlags(cv::UMat* instance, const cv::UMatUsageFlags val) {
			instance->usageFlags = val;
	}

	// cv::UMat::u() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2640
	// ("cv::UMat::u", vec![(pred!(mut, [], []), _)]),
	cv::UMatData* cv_UMat_propU(cv::UMat* instance) {
			cv::UMatData* ret = instance->u;
			return new cv::UMatData(*ret);
	}

	// cv::UMat::setU(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2640
	// ("cv::UMat::setU", vec![(pred!(mut, ["val"], ["cv::UMatData*"]), _)]),
	void cv_UMat_propU_UMatDataX(cv::UMat* instance, cv::UMatData* const val) {
			instance->u = val;
	}

	// cv::UMat::offset() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2643
	// ("cv::UMat::offset", vec![(pred!(const, [], []), _)]),
	size_t cv_UMat_propOffset_const(const cv::UMat* instance) {
			size_t ret = instance->offset;
			return ret;
	}

	// cv::UMat::setOffset(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2643
	// ("cv::UMat::setOffset", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
	void cv_UMat_propOffset_const_size_t(cv::UMat* instance, const size_t val) {
			instance->offset = val;
	}

	// cv::UMat::size() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2645
	// ("cv::UMat::size", vec![(pred!(const, [], []), _)]),
	cv::MatSize* cv_UMat_propSize_const(const cv::UMat* instance) {
			cv::MatSize ret = instance->size;
			return new cv::MatSize(ret);
	}

	// cv::UMat::setSize(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2645
	// ("cv::UMat::setSize", vec![(pred!(mut, ["val"], ["const cv::MatSize"]), _)]),
	void cv_UMat_propSize_const_MatSize(cv::UMat* instance, const cv::MatSize* val) {
			instance->size = *val;
	}

	// cv::UMat::step() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:2646
	// ("cv::UMat::step", vec![(pred!(const, [], []), _)]),
	cv::MatStep* cv_UMat_propStep_const(const cv::UMat* instance) {
			cv::MatStep ret = instance->step;
			return new cv::MatStep(ret);
	}

	// cv::UMat::delete() generated
	// ("cv::UMat::delete", vec![(pred!(mut, [], []), _)]),
	void cv_UMat_delete(cv::UMat* instance) {
			delete instance;
	}

	// lock()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:563
	// ("cv::UMatData::lock", vec![(pred!(mut, [], []), _)]),
	void cv_UMatData_lock(cv::UMatData* instance, ResultVoid* ocvrs_return) {
		try {
			instance->lock();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// unlock()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:564
	// ("cv::UMatData::unlock", vec![(pred!(mut, [], []), _)]),
	void cv_UMatData_unlock(cv::UMatData* instance, ResultVoid* ocvrs_return) {
		try {
			instance->unlock();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// hostCopyObsolete()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:566
	// ("cv::UMatData::hostCopyObsolete", vec![(pred!(const, [], []), _)]),
	void cv_UMatData_hostCopyObsolete_const(const cv::UMatData* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->hostCopyObsolete();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// deviceCopyObsolete()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:567
	// ("cv::UMatData::deviceCopyObsolete", vec![(pred!(const, [], []), _)]),
	void cv_UMatData_deviceCopyObsolete_const(const cv::UMatData* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->deviceCopyObsolete();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// deviceMemMapped()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:568
	// ("cv::UMatData::deviceMemMapped", vec![(pred!(const, [], []), _)]),
	void cv_UMatData_deviceMemMapped_const(const cv::UMatData* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->deviceMemMapped();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// copyOnMap()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:569
	// ("cv::UMatData::copyOnMap", vec![(pred!(const, [], []), _)]),
	void cv_UMatData_copyOnMap_const(const cv::UMatData* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->copyOnMap();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// tempUMat()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:570
	// ("cv::UMatData::tempUMat", vec![(pred!(const, [], []), _)]),
	void cv_UMatData_tempUMat_const(const cv::UMatData* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->tempUMat();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// tempCopiedUMat()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:571
	// ("cv::UMatData::tempCopiedUMat", vec![(pred!(const, [], []), _)]),
	void cv_UMatData_tempCopiedUMat_const(const cv::UMatData* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->tempCopiedUMat();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// markHostCopyObsolete(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:572
	// ("cv::UMatData::markHostCopyObsolete", vec![(pred!(mut, ["flag"], ["bool"]), _)]),
	void cv_UMatData_markHostCopyObsolete_bool(cv::UMatData* instance, bool flag, ResultVoid* ocvrs_return) {
		try {
			instance->markHostCopyObsolete(flag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// markDeviceCopyObsolete(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:573
	// ("cv::UMatData::markDeviceCopyObsolete", vec![(pred!(mut, ["flag"], ["bool"]), _)]),
	void cv_UMatData_markDeviceCopyObsolete_bool(cv::UMatData* instance, bool flag, ResultVoid* ocvrs_return) {
		try {
			instance->markDeviceCopyObsolete(flag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// markDeviceMemMapped(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:574
	// ("cv::UMatData::markDeviceMemMapped", vec![(pred!(mut, ["flag"], ["bool"]), _)]),
	void cv_UMatData_markDeviceMemMapped_bool(cv::UMatData* instance, bool flag, ResultVoid* ocvrs_return) {
		try {
			instance->markDeviceMemMapped(flag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::UMatData::urefcount() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:578
	// ("cv::UMatData::urefcount", vec![(pred!(const, [], []), _)]),
	int cv_UMatData_propUrefcount_const(const cv::UMatData* instance) {
			int ret = instance->urefcount;
			return ret;
	}

	// cv::UMatData::setUrefcount(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:578
	// ("cv::UMatData::setUrefcount", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_UMatData_propUrefcount_const_int(cv::UMatData* instance, const int val) {
			instance->urefcount = val;
	}

	// cv::UMatData::refcount() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:579
	// ("cv::UMatData::refcount", vec![(pred!(const, [], []), _)]),
	int cv_UMatData_propRefcount_const(const cv::UMatData* instance) {
			int ret = instance->refcount;
			return ret;
	}

	// cv::UMatData::setRefcount(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:579
	// ("cv::UMatData::setRefcount", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_UMatData_propRefcount_const_int(cv::UMatData* instance, const int val) {
			instance->refcount = val;
	}

	// cv::UMatData::data() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:580
	// ("cv::UMatData::data", vec![(pred!(const, [], []), _)]),
	unsigned char* cv_UMatData_propData_const(const cv::UMatData* instance) {
			unsigned char* const ret = instance->data;
			return ret;
	}

	// cv::UMatData::dataMut() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:580
	// ("cv::UMatData::dataMut", vec![(pred!(mut, [], []), _)]),
	unsigned char* cv_UMatData_propData(cv::UMatData* instance) {
			unsigned char* ret = instance->data;
			return ret;
	}

	// cv::UMatData::setData(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:580
	// ("cv::UMatData::setData", vec![(pred!(mut, ["val"], ["unsigned char*"]), _)]),
	void cv_UMatData_propData_unsigned_charX(cv::UMatData* instance, unsigned char* const val) {
			instance->data = val;
	}

	// cv::UMatData::origdata() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:581
	// ("cv::UMatData::origdata", vec![(pred!(const, [], []), _)]),
	unsigned char* cv_UMatData_propOrigdata_const(const cv::UMatData* instance) {
			unsigned char* const ret = instance->origdata;
			return ret;
	}

	// cv::UMatData::origdataMut() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:581
	// ("cv::UMatData::origdataMut", vec![(pred!(mut, [], []), _)]),
	unsigned char* cv_UMatData_propOrigdata(cv::UMatData* instance) {
			unsigned char* ret = instance->origdata;
			return ret;
	}

	// cv::UMatData::setOrigdata(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:581
	// ("cv::UMatData::setOrigdata", vec![(pred!(mut, ["val"], ["unsigned char*"]), _)]),
	void cv_UMatData_propOrigdata_unsigned_charX(cv::UMatData* instance, unsigned char* const val) {
			instance->origdata = val;
	}

	// cv::UMatData::size() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:582
	// ("cv::UMatData::size", vec![(pred!(const, [], []), _)]),
	size_t cv_UMatData_propSize_const(const cv::UMatData* instance) {
			size_t ret = instance->size;
			return ret;
	}

	// cv::UMatData::setSize(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:582
	// ("cv::UMatData::setSize", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
	void cv_UMatData_propSize_const_size_t(cv::UMatData* instance, const size_t val) {
			instance->size = val;
	}

	// cv::UMatData::flags() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:584
	// ("cv::UMatData::flags", vec![(pred!(const, [], []), _)]),
	int cv_UMatData_propFlags_const(const cv::UMatData* instance) {
			int ret = instance->flags;
			return ret;
	}

	// cv::UMatData::setFlags(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:584
	// ("cv::UMatData::setFlags", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_UMatData_propFlags_const_int(cv::UMatData* instance, const int val) {
			instance->flags = val;
	}

	// cv::UMatData::handle() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:585
	// ("cv::UMatData::handle", vec![(pred!(mut, [], []), _)]),
	void* cv_UMatData_propHandle(cv::UMatData* instance) {
			void* ret = instance->handle;
			return ret;
	}

	// cv::UMatData::setHandle(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:585
	// ("cv::UMatData::setHandle", vec![(pred!(mut, ["val"], ["void*"]), _)]),
	void cv_UMatData_propHandle_voidX(cv::UMatData* instance, void* const val) {
			instance->handle = val;
	}

	// cv::UMatData::userdata() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:586
	// ("cv::UMatData::userdata", vec![(pred!(mut, [], []), _)]),
	void* cv_UMatData_propUserdata(cv::UMatData* instance) {
			void* ret = instance->userdata;
			return ret;
	}

	// cv::UMatData::setUserdata(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:586
	// ("cv::UMatData::setUserdata", vec![(pred!(mut, ["val"], ["void*"]), _)]),
	void cv_UMatData_propUserdata_voidX(cv::UMatData* instance, void* const val) {
			instance->userdata = val;
	}

	// cv::UMatData::allocatorFlags_() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:587
	// ("cv::UMatData::allocatorFlags_", vec![(pred!(const, [], []), _)]),
	int cv_UMatData_propAllocatorFlags__const(const cv::UMatData* instance) {
			int ret = instance->allocatorFlags_;
			return ret;
	}

	// cv::UMatData::setAllocatorFlags_(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:587
	// ("cv::UMatData::setAllocatorFlags_", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_UMatData_propAllocatorFlags__const_int(cv::UMatData* instance, const int val) {
			instance->allocatorFlags_ = val;
	}

	// cv::UMatData::mapcount() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:588
	// ("cv::UMatData::mapcount", vec![(pred!(const, [], []), _)]),
	int cv_UMatData_propMapcount_const(const cv::UMatData* instance) {
			int ret = instance->mapcount;
			return ret;
	}

	// cv::UMatData::setMapcount(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:588
	// ("cv::UMatData::setMapcount", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_UMatData_propMapcount_const_int(cv::UMatData* instance, const int val) {
			instance->mapcount = val;
	}

	// cv::UMatData::originalUMatData() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:589
	// ("cv::UMatData::originalUMatData", vec![(pred!(mut, [], []), _)]),
	cv::UMatData* cv_UMatData_propOriginalUMatData(cv::UMatData* instance) {
			cv::UMatData* ret = instance->originalUMatData;
			return new cv::UMatData(*ret);
	}

	// cv::UMatData::setOriginalUMatData(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:589
	// ("cv::UMatData::setOriginalUMatData", vec![(pred!(mut, ["val"], ["cv::UMatData*"]), _)]),
	void cv_UMatData_propOriginalUMatData_UMatDataX(cv::UMatData* instance, cv::UMatData* const val) {
			instance->originalUMatData = val;
	}

	// cv::UMatData::delete() generated
	// ("cv::UMatData::delete", vec![(pred!(mut, [], []), _)]),
	void cv_UMatData_delete(cv::UMatData* instance) {
			delete instance;
	}

	// _InputArray()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:189
	// ("cv::_InputArray::_InputArray", vec![(pred!(mut, [], []), _)]),
	void cv__InputArray__InputArray(Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray* ret = new cv::_InputArray();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _InputArray(int, void *)(Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:190
	// ("cv::_InputArray::_InputArray", vec![(pred!(mut, ["_flags", "_obj"], ["int", "void*"]), _)]),
	void cv__InputArray__InputArray_int_voidX(int _flags, void* _obj, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(_flags, _obj);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _InputArray(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:191
	// ("cv::_InputArray::_InputArray", vec![(pred!(mut, ["m"], ["const cv::Mat*"]), _)]),
	void cv__InputArray__InputArray_const_MatR(const cv::Mat* m, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray* ret = new const cv::_InputArray(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _InputArray(const MatExpr &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:192
	// ("cv::_InputArray::_InputArray", vec![(pred!(mut, ["expr"], ["const cv::MatExpr*"]), _)]),
	void cv__InputArray__InputArray_const_MatExprR(const cv::MatExpr* expr, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray* ret = new const cv::_InputArray(*expr);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _InputArray(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:193
	// ("cv::_InputArray::_InputArray", vec![(pred!(mut, ["vec"], ["const std::vector<cv::Mat>*"]), _)]),
	void cv__InputArray__InputArray_const_vectorLMatGR(const std::vector<cv::Mat>* vec, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray* ret = new const cv::_InputArray(*vec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _InputArray(const std::vector<bool> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:196
	// ("cv::_InputArray::_InputArray", vec![(pred!(mut, ["vec"], ["const std::vector<bool>*"]), _)]),
	void cv__InputArray__InputArray_const_vectorLboolGR(const std::vector<bool>* vec, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray* ret = new const cv::_InputArray(*vec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _InputArray(const std::vector<std::vector<bool>> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:198
	// ("cv::_InputArray::_InputArray", vec![(pred!(mut, ["unnamed"], ["const std::vector<std::vector<bool>>*"]), _)]),
	void cv__InputArray__InputArray_const_vectorLvectorLboolGGR(const std::vector<std::vector<bool>>* unnamed, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(*unnamed);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _InputArray(const double &)(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:202
	// ("cv::_InputArray::_InputArray", vec![(pred!(mut, ["val"], ["const double*"]), _)]),
	void cv__InputArray__InputArray_const_doubleR(const double* val, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray* ret = new const cv::_InputArray(*val);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _InputArray(const cuda::GpuMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:203
	// ("cv::_InputArray::_InputArray", vec![(pred!(mut, ["d_mat"], ["const cv::cuda::GpuMat*"]), _)]),
	void cv__InputArray__InputArray_const_GpuMatR(const cv::cuda::GpuMat* d_mat, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray* ret = new const cv::_InputArray(*d_mat);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _InputArray(const std::vector<cuda::GpuMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:204
	// ("cv::_InputArray::_InputArray", vec![(pred!(mut, ["d_mat_array"], ["const std::vector<cv::cuda::GpuMat>*"]), _)]),
	void cv__InputArray__InputArray_const_vectorLGpuMatGR(const std::vector<cv::cuda::GpuMat>* d_mat_array, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray* ret = new const cv::_InputArray(*d_mat_array);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _InputArray(const ogl::Buffer &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:205
	// ("cv::_InputArray::_InputArray", vec![(pred!(mut, ["buf"], ["const cv::ogl::Buffer*"]), _)]),
	void cv__InputArray__InputArray_const_BufferR(const cv::ogl::Buffer* buf, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray* ret = new const cv::_InputArray(*buf);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _InputArray(const cuda::HostMem &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:206
	// ("cv::_InputArray::_InputArray", vec![(pred!(mut, ["cuda_mem"], ["const cv::cuda::HostMem*"]), _)]),
	void cv__InputArray__InputArray_const_HostMemR(const cv::cuda::HostMem* cuda_mem, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray* ret = new const cv::_InputArray(*cuda_mem);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _InputArray(const UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:208
	// ("cv::_InputArray::_InputArray", vec![(pred!(mut, ["um"], ["const cv::UMat*"]), _)]),
	void cv__InputArray__InputArray_const_UMatR(const cv::UMat* um, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray* ret = new const cv::_InputArray(*um);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _InputArray(const std::vector<UMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:209
	// ("cv::_InputArray::_InputArray", vec![(pred!(mut, ["umv"], ["const std::vector<cv::UMat>*"]), _)]),
	void cv__InputArray__InputArray_const_vectorLUMatGR(const std::vector<cv::UMat>* umv, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray* ret = new const cv::_InputArray(*umv);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMat(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:221
	// ("cv::_InputArray::getMat", vec![(pred!(const, ["idx"], ["int"]), _)]),
	void cv__InputArray_getMat_const_int(const cv::_InputArray* instance, int idx, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getMat(idx);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::_InputArray::getMat() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:221
	// ("cv::_InputArray::getMat", vec![(pred!(const, [], []), _)]),
	void cv__InputArray_getMat_const(const cv::_InputArray* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getMat();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMat_(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:222
	// ("cv::_InputArray::getMat_", vec![(pred!(const, ["idx"], ["int"]), _)]),
	void cv__InputArray_getMat__const_int(const cv::_InputArray* instance, int idx, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getMat_(idx);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::_InputArray::getMat_() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:222
	// ("cv::_InputArray::getMat_", vec![(pred!(const, [], []), _)]),
	void cv__InputArray_getMat__const(const cv::_InputArray* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getMat_();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUMat(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:223
	// ("cv::_InputArray::getUMat", vec![(pred!(const, ["idx"], ["int"]), _)]),
	void cv__InputArray_getUMat_const_int(const cv::_InputArray* instance, int idx, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->getUMat(idx);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::_InputArray::getUMat() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:223
	// ("cv::_InputArray::getUMat", vec![(pred!(const, [], []), _)]),
	void cv__InputArray_getUMat_const(const cv::_InputArray* instance, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->getUMat();
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMatVector(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:224
	// ("cv::_InputArray::getMatVector", vec![(pred!(const, ["mv"], ["std::vector<cv::Mat>*"]), _)]),
	void cv__InputArray_getMatVector_const_vectorLMatGR(const cv::_InputArray* instance, std::vector<cv::Mat>* mv, ResultVoid* ocvrs_return) {
		try {
			instance->getMatVector(*mv);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUMatVector(std::vector<UMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:225
	// ("cv::_InputArray::getUMatVector", vec![(pred!(const, ["umv"], ["std::vector<cv::UMat>*"]), _)]),
	void cv__InputArray_getUMatVector_const_vectorLUMatGR(const cv::_InputArray* instance, std::vector<cv::UMat>* umv, ResultVoid* ocvrs_return) {
		try {
			instance->getUMatVector(*umv);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGpuMatVector(std::vector<cuda::GpuMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:226
	// ("cv::_InputArray::getGpuMatVector", vec![(pred!(const, ["gpumv"], ["std::vector<cv::cuda::GpuMat>*"]), _)]),
	void cv__InputArray_getGpuMatVector_const_vectorLGpuMatGR(const cv::_InputArray* instance, std::vector<cv::cuda::GpuMat>* gpumv, ResultVoid* ocvrs_return) {
		try {
			instance->getGpuMatVector(*gpumv);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGpuMat()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:227
	// ("cv::_InputArray::getGpuMat", vec![(pred!(const, [], []), _)]),
	void cv__InputArray_getGpuMat_const(const cv::_InputArray* instance, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->getGpuMat();
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOGlBuffer()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:228
	// ("cv::_InputArray::getOGlBuffer", vec![(pred!(const, [], []), _)]),
	void cv__InputArray_getOGlBuffer_const(const cv::_InputArray* instance, Result<cv::ogl::Buffer*>* ocvrs_return) {
		try {
			cv::ogl::Buffer ret = instance->getOGlBuffer();
			Ok(new cv::ogl::Buffer(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFlags()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:230
	// ("cv::_InputArray::getFlags", vec![(pred!(const, [], []), _)]),
	void cv__InputArray_getFlags_const(const cv::_InputArray* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFlags();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getObj()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:231
	// ("cv::_InputArray::getObj", vec![(pred!(const, [], []), _)]),
	void cv__InputArray_getObj_const(const cv::_InputArray* instance, Result<void*>* ocvrs_return) {
		try {
			void* ret = instance->getObj();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSz()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:232
	// ("cv::_InputArray::getSz", vec![(pred!(const, [], []), _)]),
	void cv__InputArray_getSz_const(const cv::_InputArray* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getSz();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// kind()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:234
	// ("cv::_InputArray::kind", vec![(pred!(const, [], []), _)]),
	void cv__InputArray_kind_const(const cv::_InputArray* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->kind();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dims(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:235
	// ("cv::_InputArray::dims", vec![(pred!(const, ["i"], ["int"]), _)]),
	void cv__InputArray_dims_const_int(const cv::_InputArray* instance, int i, Result<int>* ocvrs_return) {
		try {
			int ret = instance->dims(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::_InputArray::dims() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:235
	// ("cv::_InputArray::dims", vec![(pred!(const, [], []), _)]),
	void cv__InputArray_dims_const(const cv::_InputArray* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->dims();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cols(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:236
	// ("cv::_InputArray::cols", vec![(pred!(const, ["i"], ["int"]), _)]),
	void cv__InputArray_cols_const_int(const cv::_InputArray* instance, int i, Result<int>* ocvrs_return) {
		try {
			int ret = instance->cols(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::_InputArray::cols() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:236
	// ("cv::_InputArray::cols", vec![(pred!(const, [], []), _)]),
	void cv__InputArray_cols_const(const cv::_InputArray* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->cols();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// rows(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:237
	// ("cv::_InputArray::rows", vec![(pred!(const, ["i"], ["int"]), _)]),
	void cv__InputArray_rows_const_int(const cv::_InputArray* instance, int i, Result<int>* ocvrs_return) {
		try {
			int ret = instance->rows(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::_InputArray::rows() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:237
	// ("cv::_InputArray::rows", vec![(pred!(const, [], []), _)]),
	void cv__InputArray_rows_const(const cv::_InputArray* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->rows();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// size(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:238
	// ("cv::_InputArray::size", vec![(pred!(const, ["i"], ["int"]), _)]),
	void cv__InputArray_size_const_int(const cv::_InputArray* instance, int i, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->size(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::_InputArray::size() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:238
	// ("cv::_InputArray::size", vec![(pred!(const, [], []), _)]),
	void cv__InputArray_size_const(const cv::_InputArray* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->size();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// sizend(int *, int)(Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:239
	// ("cv::_InputArray::sizend", vec![(pred!(const, ["sz", "i"], ["int*", "int"]), _)]),
	void cv__InputArray_sizend_const_intX_int(const cv::_InputArray* instance, int* sz, int i, Result<int>* ocvrs_return) {
		try {
			int ret = instance->sizend(sz, i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::_InputArray::sizend(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:239
	// ("cv::_InputArray::sizend", vec![(pred!(const, ["sz"], ["int*"]), _)]),
	void cv__InputArray_sizend_const_intX(const cv::_InputArray* instance, int* sz, Result<int>* ocvrs_return) {
		try {
			int ret = instance->sizend(sz);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// sameSize(const _InputArray &)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:240
	// ("cv::_InputArray::sameSize", vec![(pred!(const, ["arr"], ["const cv::_InputArray*"]), _)]),
	void cv__InputArray_sameSize_const_const__InputArrayR(const cv::_InputArray* instance, const cv::_InputArray* arr, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->sameSize(*arr);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// total(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:241
	// ("cv::_InputArray::total", vec![(pred!(const, ["i"], ["int"]), _)]),
	void cv__InputArray_total_const_int(const cv::_InputArray* instance, int i, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->total(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::_InputArray::total() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:241
	// ("cv::_InputArray::total", vec![(pred!(const, [], []), _)]),
	void cv__InputArray_total_const(const cv::_InputArray* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->total();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// type(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:242
	// ("cv::_InputArray::type", vec![(pred!(const, ["i"], ["int"]), _)]),
	void cv__InputArray_type_const_int(const cv::_InputArray* instance, int i, Result<int>* ocvrs_return) {
		try {
			int ret = instance->type(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::_InputArray::type() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:242
	// ("cv::_InputArray::type", vec![(pred!(const, [], []), _)]),
	void cv__InputArray_type_const(const cv::_InputArray* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->type();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// depth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:243
	// ("cv::_InputArray::depth", vec![(pred!(const, ["i"], ["int"]), _)]),
	void cv__InputArray_depth_const_int(const cv::_InputArray* instance, int i, Result<int>* ocvrs_return) {
		try {
			int ret = instance->depth(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::_InputArray::depth() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:243
	// ("cv::_InputArray::depth", vec![(pred!(const, [], []), _)]),
	void cv__InputArray_depth_const(const cv::_InputArray* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->depth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// channels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:244
	// ("cv::_InputArray::channels", vec![(pred!(const, ["i"], ["int"]), _)]),
	void cv__InputArray_channels_const_int(const cv::_InputArray* instance, int i, Result<int>* ocvrs_return) {
		try {
			int ret = instance->channels(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::_InputArray::channels() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:244
	// ("cv::_InputArray::channels", vec![(pred!(const, [], []), _)]),
	void cv__InputArray_channels_const(const cv::_InputArray* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->channels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isContinuous(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:245
	// ("cv::_InputArray::isContinuous", vec![(pred!(const, ["i"], ["int"]), _)]),
	void cv__InputArray_isContinuous_const_int(const cv::_InputArray* instance, int i, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isContinuous(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::_InputArray::isContinuous() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:245
	// ("cv::_InputArray::isContinuous", vec![(pred!(const, [], []), _)]),
	void cv__InputArray_isContinuous_const(const cv::_InputArray* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isContinuous();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isSubmatrix(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:246
	// ("cv::_InputArray::isSubmatrix", vec![(pred!(const, ["i"], ["int"]), _)]),
	void cv__InputArray_isSubmatrix_const_int(const cv::_InputArray* instance, int i, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isSubmatrix(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::_InputArray::isSubmatrix() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:246
	// ("cv::_InputArray::isSubmatrix", vec![(pred!(const, [], []), _)]),
	void cv__InputArray_isSubmatrix_const(const cv::_InputArray* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isSubmatrix();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:247
	// ("cv::_InputArray::empty", vec![(pred!(const, [], []), _)]),
	void cv__InputArray_empty_const(const cv::_InputArray* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// copyTo(const _OutputArray &)(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:248
	// ("cv::_InputArray::copyTo", vec![(pred!(const, ["arr"], ["const cv::_OutputArray*"]), _)]),
	void cv__InputArray_copyTo_const_const__OutputArrayR(const cv::_InputArray* instance, const cv::_OutputArray* arr, ResultVoid* ocvrs_return) {
		try {
			instance->copyTo(*arr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// copyTo(const _OutputArray &, const _InputArray &)(OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:249
	// ("cv::_InputArray::copyTo", vec![(pred!(const, ["arr", "mask"], ["const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv__InputArray_copyTo_const_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* instance, const cv::_OutputArray* arr, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			instance->copyTo(*arr, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// offset(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:250
	// ("cv::_InputArray::offset", vec![(pred!(const, ["i"], ["int"]), _)]),
	void cv__InputArray_offset_const_int(const cv::_InputArray* instance, int i, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->offset(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::_InputArray::offset() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:250
	// ("cv::_InputArray::offset", vec![(pred!(const, [], []), _)]),
	void cv__InputArray_offset_const(const cv::_InputArray* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->offset();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// step(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:251
	// ("cv::_InputArray::step", vec![(pred!(const, ["i"], ["int"]), _)]),
	void cv__InputArray_step_const_int(const cv::_InputArray* instance, int i, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->step(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::_InputArray::step() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:251
	// ("cv::_InputArray::step", vec![(pred!(const, [], []), _)]),
	void cv__InputArray_step_const(const cv::_InputArray* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->step();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isMat()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:252
	// ("cv::_InputArray::isMat", vec![(pred!(const, [], []), _)]),
	void cv__InputArray_isMat_const(const cv::_InputArray* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isMat();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isUMat()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:253
	// ("cv::_InputArray::isUMat", vec![(pred!(const, [], []), _)]),
	void cv__InputArray_isUMat_const(const cv::_InputArray* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isUMat();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isMatVector()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:254
	// ("cv::_InputArray::isMatVector", vec![(pred!(const, [], []), _)]),
	void cv__InputArray_isMatVector_const(const cv::_InputArray* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isMatVector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isUMatVector()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:255
	// ("cv::_InputArray::isUMatVector", vec![(pred!(const, [], []), _)]),
	void cv__InputArray_isUMatVector_const(const cv::_InputArray* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isUMatVector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isMatx()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:256
	// ("cv::_InputArray::isMatx", vec![(pred!(const, [], []), _)]),
	void cv__InputArray_isMatx_const(const cv::_InputArray* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isMatx();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isVector()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:257
	// ("cv::_InputArray::isVector", vec![(pred!(const, [], []), _)]),
	void cv__InputArray_isVector_const(const cv::_InputArray* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isVector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isGpuMat()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:258
	// ("cv::_InputArray::isGpuMat", vec![(pred!(const, [], []), _)]),
	void cv__InputArray_isGpuMat_const(const cv::_InputArray* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isGpuMat();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isGpuMatVector()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:259
	// ("cv::_InputArray::isGpuMatVector", vec![(pred!(const, [], []), _)]),
	void cv__InputArray_isGpuMatVector_const(const cv::_InputArray* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isGpuMatVector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::_InputArray::_InputArray(VariableArray, Primitive) generated
	// ("cv::_InputArray::_InputArray", vec![(pred!(mut, ["vec", "n"], ["const unsigned char*", "int"]), _)]),
	void cv__InputArray__InputArray_const_unsigned_charX_int(const unsigned char* vec, int n, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			const cv::_InputArray* ret = new const cv::_InputArray(vec, n);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::_InputArray::delete() generated
	// ("cv::_InputArray::delete", vec![(pred!(mut, [], []), _)]),
	void cv__InputArray_delete(cv::_InputArray* instance) {
			delete instance;
	}

	// _InputOutputArray()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:393
	// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, [], []), _)]),
	void cv__InputOutputArray__InputOutputArray(Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _InputOutputArray(int, void *)(Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:394
	// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["_flags", "_obj"], ["int", "void*"]), _)]),
	void cv__InputOutputArray__InputOutputArray_int_voidX(int _flags, void* _obj, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(_flags, _obj);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _InputOutputArray(Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:395
	// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["m"], ["cv::Mat*"]), _)]),
	void cv__InputOutputArray__InputOutputArray_MatR(cv::Mat* m, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _InputOutputArray(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:396
	// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["vec"], ["std::vector<cv::Mat>*"]), _)]),
	void cv__InputOutputArray__InputOutputArray_vectorLMatGR(std::vector<cv::Mat>* vec, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*vec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _InputOutputArray(cuda::GpuMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:397
	// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["d_mat"], ["cv::cuda::GpuMat*"]), _)]),
	void cv__InputOutputArray__InputOutputArray_GpuMatR(cv::cuda::GpuMat* d_mat, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*d_mat);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _InputOutputArray(ogl::Buffer &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:398
	// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["buf"], ["cv::ogl::Buffer*"]), _)]),
	void cv__InputOutputArray__InputOutputArray_BufferR(cv::ogl::Buffer* buf, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*buf);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _InputOutputArray(cuda::HostMem &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:399
	// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["cuda_mem"], ["cv::cuda::HostMem*"]), _)]),
	void cv__InputOutputArray__InputOutputArray_HostMemR(cv::cuda::HostMem* cuda_mem, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*cuda_mem);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _InputOutputArray(std::vector<bool> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:402
	// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["vec"], ["std::vector<bool>*"]), _)]),
	void cv__InputOutputArray__InputOutputArray_vectorLboolGR(std::vector<bool>* vec, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*vec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _InputOutputArray(UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:408
	// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["m"], ["cv::UMat*"]), _)]),
	void cv__InputOutputArray__InputOutputArray_UMatR(cv::UMat* m, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _InputOutputArray(std::vector<UMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:409
	// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["vec"], ["std::vector<cv::UMat>*"]), _)]),
	void cv__InputOutputArray__InputOutputArray_vectorLUMatGR(std::vector<cv::UMat>* vec, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*vec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _InputOutputArray(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:411
	// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["m"], ["const cv::Mat*"]), _)]),
	void cv__InputOutputArray__InputOutputArray_const_MatR(const cv::Mat* m, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			const cv::_InputOutputArray* ret = new const cv::_InputOutputArray(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _InputOutputArray(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:412
	// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["vec"], ["const std::vector<cv::Mat>*"]), _)]),
	void cv__InputOutputArray__InputOutputArray_const_vectorLMatGR(const std::vector<cv::Mat>* vec, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			const cv::_InputOutputArray* ret = new const cv::_InputOutputArray(*vec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _InputOutputArray(const cuda::GpuMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:413
	// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["d_mat"], ["const cv::cuda::GpuMat*"]), _)]),
	void cv__InputOutputArray__InputOutputArray_const_GpuMatR(const cv::cuda::GpuMat* d_mat, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			const cv::_InputOutputArray* ret = new const cv::_InputOutputArray(*d_mat);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _InputOutputArray(const std::vector<cuda::GpuMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:414
	// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["d_mat"], ["const std::vector<cv::cuda::GpuMat>*"]), _)]),
	void cv__InputOutputArray__InputOutputArray_const_vectorLGpuMatGR(const std::vector<cv::cuda::GpuMat>* d_mat, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			const cv::_InputOutputArray* ret = new const cv::_InputOutputArray(*d_mat);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _InputOutputArray(const ogl::Buffer &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:415
	// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["buf"], ["const cv::ogl::Buffer*"]), _)]),
	void cv__InputOutputArray__InputOutputArray_const_BufferR(const cv::ogl::Buffer* buf, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			const cv::_InputOutputArray* ret = new const cv::_InputOutputArray(*buf);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _InputOutputArray(const cuda::HostMem &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:416
	// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["cuda_mem"], ["const cv::cuda::HostMem*"]), _)]),
	void cv__InputOutputArray__InputOutputArray_const_HostMemR(const cv::cuda::HostMem* cuda_mem, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			const cv::_InputOutputArray* ret = new const cv::_InputOutputArray(*cuda_mem);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _InputOutputArray(const UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:424
	// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["m"], ["const cv::UMat*"]), _)]),
	void cv__InputOutputArray__InputOutputArray_const_UMatR(const cv::UMat* m, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			const cv::_InputOutputArray* ret = new const cv::_InputOutputArray(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _InputOutputArray(const std::vector<UMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:425
	// ("cv::_InputOutputArray::_InputOutputArray", vec![(pred!(mut, ["vec"], ["const std::vector<cv::UMat>*"]), _)]),
	void cv__InputOutputArray__InputOutputArray_const_vectorLUMatGR(const std::vector<cv::UMat>* vec, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			const cv::_InputOutputArray* ret = new const cv::_InputOutputArray(*vec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::_InputOutputArray::to__InputArray() generated
	// ("cv::_InputOutputArray::to__InputArray", vec![(pred!(mut, [], []), _)]),
	cv::_InputArray* cv__InputOutputArray_to__InputArray(cv::_InputOutputArray* instance) {
			return dynamic_cast<cv::_InputArray*>(instance);
	}

	// cv::_InputOutputArray::to__OutputArray() generated
	// ("cv::_InputOutputArray::to__OutputArray", vec![(pred!(mut, [], []), _)]),
	cv::_OutputArray* cv__InputOutputArray_to__OutputArray(cv::_InputOutputArray* instance) {
			return dynamic_cast<cv::_OutputArray*>(instance);
	}

	// cv::_InputOutputArray::delete() generated
	// ("cv::_InputOutputArray::delete", vec![(pred!(mut, [], []), _)]),
	void cv__InputOutputArray_delete(cv::_InputOutputArray* instance) {
			delete instance;
	}

	// _OutputArray()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:314
	// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, [], []), _)]),
	void cv__OutputArray__OutputArray(Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _OutputArray(int, void *)(Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:315
	// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["_flags", "_obj"], ["int", "void*"]), _)]),
	void cv__OutputArray__OutputArray_int_voidX(int _flags, void* _obj, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(_flags, _obj);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _OutputArray(Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:316
	// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["m"], ["cv::Mat*"]), _)]),
	void cv__OutputArray__OutputArray_MatR(cv::Mat* m, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _OutputArray(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:317
	// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["vec"], ["std::vector<cv::Mat>*"]), _)]),
	void cv__OutputArray__OutputArray_vectorLMatGR(std::vector<cv::Mat>* vec, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*vec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _OutputArray(cuda::GpuMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:318
	// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["d_mat"], ["cv::cuda::GpuMat*"]), _)]),
	void cv__OutputArray__OutputArray_GpuMatR(cv::cuda::GpuMat* d_mat, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*d_mat);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _OutputArray(std::vector<cuda::GpuMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:319
	// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["d_mat"], ["std::vector<cv::cuda::GpuMat>*"]), _)]),
	void cv__OutputArray__OutputArray_vectorLGpuMatGR(std::vector<cv::cuda::GpuMat>* d_mat, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*d_mat);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _OutputArray(ogl::Buffer &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:320
	// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["buf"], ["cv::ogl::Buffer*"]), _)]),
	void cv__OutputArray__OutputArray_BufferR(cv::ogl::Buffer* buf, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*buf);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _OutputArray(cuda::HostMem &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:321
	// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["cuda_mem"], ["cv::cuda::HostMem*"]), _)]),
	void cv__OutputArray__OutputArray_HostMemR(cv::cuda::HostMem* cuda_mem, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*cuda_mem);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _OutputArray(std::vector<bool> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:324
	// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["vec"], ["std::vector<bool>*"]), _)]),
	void cv__OutputArray__OutputArray_vectorLboolGR(std::vector<bool>* vec, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*vec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _OutputArray(std::vector<std::vector<bool>> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:326
	// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["unnamed"], ["std::vector<std::vector<bool>>*"]), _)]),
	void cv__OutputArray__OutputArray_vectorLvectorLboolGGR(std::vector<std::vector<bool>>* unnamed, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*unnamed);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _OutputArray(UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:331
	// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["m"], ["cv::UMat*"]), _)]),
	void cv__OutputArray__OutputArray_UMatR(cv::UMat* m, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _OutputArray(std::vector<UMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:332
	// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["vec"], ["std::vector<cv::UMat>*"]), _)]),
	void cv__OutputArray__OutputArray_vectorLUMatGR(std::vector<cv::UMat>* vec, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*vec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _OutputArray(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:334
	// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["m"], ["const cv::Mat*"]), _)]),
	void cv__OutputArray__OutputArray_const_MatR(const cv::Mat* m, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			const cv::_OutputArray* ret = new const cv::_OutputArray(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _OutputArray(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:335
	// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["vec"], ["const std::vector<cv::Mat>*"]), _)]),
	void cv__OutputArray__OutputArray_const_vectorLMatGR(const std::vector<cv::Mat>* vec, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			const cv::_OutputArray* ret = new const cv::_OutputArray(*vec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _OutputArray(const cuda::GpuMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:336
	// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["d_mat"], ["const cv::cuda::GpuMat*"]), _)]),
	void cv__OutputArray__OutputArray_const_GpuMatR(const cv::cuda::GpuMat* d_mat, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			const cv::_OutputArray* ret = new const cv::_OutputArray(*d_mat);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _OutputArray(const ogl::Buffer &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:338
	// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["buf"], ["const cv::ogl::Buffer*"]), _)]),
	void cv__OutputArray__OutputArray_const_BufferR(const cv::ogl::Buffer* buf, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			const cv::_OutputArray* ret = new const cv::_OutputArray(*buf);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _OutputArray(const cuda::HostMem &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:339
	// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["cuda_mem"], ["const cv::cuda::HostMem*"]), _)]),
	void cv__OutputArray__OutputArray_const_HostMemR(const cv::cuda::HostMem* cuda_mem, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			const cv::_OutputArray* ret = new const cv::_OutputArray(*cuda_mem);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _OutputArray(const UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:347
	// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["m"], ["const cv::UMat*"]), _)]),
	void cv__OutputArray__OutputArray_const_UMatR(const cv::UMat* m, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			const cv::_OutputArray* ret = new const cv::_OutputArray(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _OutputArray(const std::vector<UMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:348
	// ("cv::_OutputArray::_OutputArray", vec![(pred!(mut, ["vec"], ["const std::vector<cv::UMat>*"]), _)]),
	void cv__OutputArray__OutputArray_const_vectorLUMatGR(const std::vector<cv::UMat>* vec, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			const cv::_OutputArray* ret = new const cv::_OutputArray(*vec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fixedSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:362
	// ("cv::_OutputArray::fixedSize", vec![(pred!(const, [], []), _)]),
	void cv__OutputArray_fixedSize_const(const cv::_OutputArray* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->fixedSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fixedType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:363
	// ("cv::_OutputArray::fixedType", vec![(pred!(const, [], []), _)]),
	void cv__OutputArray_fixedType_const(const cv::_OutputArray* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->fixedType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// needed()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:364
	// ("cv::_OutputArray::needed", vec![(pred!(const, [], []), _)]),
	void cv__OutputArray_needed_const(const cv::_OutputArray* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->needed();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMatRef(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:365
	// ("cv::_OutputArray::getMatRef", vec![(pred!(const, ["i"], ["int"]), _)]),
	void cv__OutputArray_getMatRef_const_int(const cv::_OutputArray* instance, int i, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getMatRef(i);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::_OutputArray::getMatRef() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:365
	// ("cv::_OutputArray::getMatRef", vec![(pred!(const, [], []), _)]),
	void cv__OutputArray_getMatRef_const(const cv::_OutputArray* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getMatRef();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUMatRef(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:366
	// ("cv::_OutputArray::getUMatRef", vec![(pred!(const, ["i"], ["int"]), _)]),
	void cv__OutputArray_getUMatRef_const_int(const cv::_OutputArray* instance, int i, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->getUMatRef(i);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::_OutputArray::getUMatRef() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:366
	// ("cv::_OutputArray::getUMatRef", vec![(pred!(const, [], []), _)]),
	void cv__OutputArray_getUMatRef_const(const cv::_OutputArray* instance, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->getUMatRef();
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGpuMatRef()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:367
	// ("cv::_OutputArray::getGpuMatRef", vec![(pred!(const, [], []), _)]),
	void cv__OutputArray_getGpuMatRef_const(const cv::_OutputArray* instance, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->getGpuMatRef();
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGpuMatVecRef()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:368
	// ("cv::_OutputArray::getGpuMatVecRef", vec![(pred!(const, [], []), _)]),
	void cv__OutputArray_getGpuMatVecRef_const(const cv::_OutputArray* instance, Result<std::vector<cv::cuda::GpuMat>*>* ocvrs_return) {
		try {
			std::vector<cv::cuda::GpuMat> ret = instance->getGpuMatVecRef();
			Ok(new std::vector<cv::cuda::GpuMat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOGlBufferRef()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:369
	// ("cv::_OutputArray::getOGlBufferRef", vec![(pred!(const, [], []), _)]),
	void cv__OutputArray_getOGlBufferRef_const(const cv::_OutputArray* instance, Result<cv::ogl::Buffer*>* ocvrs_return) {
		try {
			cv::ogl::Buffer ret = instance->getOGlBufferRef();
			Ok(new cv::ogl::Buffer(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getHostMemRef()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:370
	// ("cv::_OutputArray::getHostMemRef", vec![(pred!(const, [], []), _)]),
	void cv__OutputArray_getHostMemRef_const(const cv::_OutputArray* instance, Result<cv::cuda::HostMem*>* ocvrs_return) {
		try {
			cv::cuda::HostMem ret = instance->getHostMemRef();
			Ok(new cv::cuda::HostMem(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(Size, int, int, bool, int)(SimpleClass, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:371
	// ("cv::_OutputArray::create", vec![(pred!(const, ["sz", "type", "i", "allowTransposed", "fixedDepthMask"], ["cv::Size", "int", "int", "bool", "int"]), _)]),
	void cv__OutputArray_create_const_Size_int_int_bool_int(const cv::_OutputArray* instance, cv::Size* sz, int type, int i, bool allowTransposed, int fixedDepthMask, ResultVoid* ocvrs_return) {
		try {
			instance->create(*sz, type, i, allowTransposed, fixedDepthMask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::_OutputArray::create(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:371
	// ("cv::_OutputArray::create", vec![(pred!(const, ["sz", "type"], ["cv::Size", "int"]), _)]),
	void cv__OutputArray_create_const_Size_int(const cv::_OutputArray* instance, cv::Size* sz, int type, ResultVoid* ocvrs_return) {
		try {
			instance->create(*sz, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, int, int, int, bool, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:372
	// ("cv::_OutputArray::create", vec![(pred!(const, ["rows", "cols", "type", "i", "allowTransposed", "fixedDepthMask"], ["int", "int", "int", "int", "bool", "int"]), _)]),
	void cv__OutputArray_create_const_int_int_int_int_bool_int(const cv::_OutputArray* instance, int rows, int cols, int type, int i, bool allowTransposed, int fixedDepthMask, ResultVoid* ocvrs_return) {
		try {
			instance->create(rows, cols, type, i, allowTransposed, fixedDepthMask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::_OutputArray::create(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:372
	// ("cv::_OutputArray::create", vec![(pred!(const, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
	void cv__OutputArray_create_const_int_int_int(const cv::_OutputArray* instance, int rows, int cols, int type, ResultVoid* ocvrs_return) {
		try {
			instance->create(rows, cols, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, const int *, int, int, bool, int)(Primitive, VariableArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:373
	// ("cv::_OutputArray::create", vec![(pred!(const, ["dims", "size", "type", "i", "allowTransposed", "fixedDepthMask"], ["int", "const int*", "int", "int", "bool", "int"]), _)]),
	void cv__OutputArray_create_const_int_const_intX_int_int_bool_int(const cv::_OutputArray* instance, int dims, const int* size, int type, int i, bool allowTransposed, int fixedDepthMask, ResultVoid* ocvrs_return) {
		try {
			instance->create(dims, size, type, i, allowTransposed, fixedDepthMask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::_OutputArray::create(Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:373
	// ("cv::_OutputArray::create", vec![(pred!(const, ["dims", "size", "type"], ["int", "const int*", "int"]), _)]),
	void cv__OutputArray_create_const_int_const_intX_int(const cv::_OutputArray* instance, int dims, const int* size, int type, ResultVoid* ocvrs_return) {
		try {
			instance->create(dims, size, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createSameSize(const _InputArray &, int)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:374
	// ("cv::_OutputArray::createSameSize", vec![(pred!(const, ["arr", "mtype"], ["const cv::_InputArray*", "int"]), _)]),
	void cv__OutputArray_createSameSize_const_const__InputArrayR_int(const cv::_OutputArray* instance, const cv::_InputArray* arr, int mtype, ResultVoid* ocvrs_return) {
		try {
			instance->createSameSize(*arr, mtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// release()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:375
	// ("cv::_OutputArray::release", vec![(pred!(const, [], []), _)]),
	void cv__OutputArray_release_const(const cv::_OutputArray* instance, ResultVoid* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clear()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:376
	// ("cv::_OutputArray::clear", vec![(pred!(const, [], []), _)]),
	void cv__OutputArray_clear_const(const cv::_OutputArray* instance, ResultVoid* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTo(const _InputArray &, const _InputArray &)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:377
	// ("cv::_OutputArray::setTo", vec![(pred!(const, ["value", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv__OutputArray_setTo_const_const__InputArrayR_const__InputArrayR(const cv::_OutputArray* instance, const cv::_InputArray* value, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			instance->setTo(*value, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::_OutputArray::setTo(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:377
	// ("cv::_OutputArray::setTo", vec![(pred!(const, ["value"], ["const cv::_InputArray*"]), _)]),
	void cv__OutputArray_setTo_const_const__InputArrayR(const cv::_OutputArray* instance, const cv::_InputArray* value, ResultVoid* ocvrs_return) {
		try {
			instance->setTo(*value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// assign(const UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:379
	// ("cv::_OutputArray::assign", vec![(pred!(const, ["u"], ["const cv::UMat*"]), _)]),
	void cv__OutputArray_assign_const_const_UMatR(const cv::_OutputArray* instance, const cv::UMat* u, ResultVoid* ocvrs_return) {
		try {
			instance->assign(*u);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// assign(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:380
	// ("cv::_OutputArray::assign", vec![(pred!(const, ["m"], ["const cv::Mat*"]), _)]),
	void cv__OutputArray_assign_const_const_MatR(const cv::_OutputArray* instance, const cv::Mat* m, ResultVoid* ocvrs_return) {
		try {
			instance->assign(*m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// assign(const std::vector<UMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:382
	// ("cv::_OutputArray::assign", vec![(pred!(const, ["v"], ["const std::vector<cv::UMat>*"]), _)]),
	void cv__OutputArray_assign_const_const_vectorLUMatGR(const cv::_OutputArray* instance, const std::vector<cv::UMat>* v, ResultVoid* ocvrs_return) {
		try {
			instance->assign(*v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// assign(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:383
	// ("cv::_OutputArray::assign", vec![(pred!(const, ["v"], ["const std::vector<cv::Mat>*"]), _)]),
	void cv__OutputArray_assign_const_const_vectorLMatGR(const cv::_OutputArray* instance, const std::vector<cv::Mat>* v, ResultVoid* ocvrs_return) {
		try {
			instance->assign(*v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// move(UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:385
	// ("cv::_OutputArray::move", vec![(pred!(const, ["u"], ["cv::UMat*"]), _)]),
	void cv__OutputArray_move_const_UMatR(const cv::_OutputArray* instance, cv::UMat* u, ResultVoid* ocvrs_return) {
		try {
			instance->move(*u);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// move(Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/mat.hpp:386
	// ("cv::_OutputArray::move", vec![(pred!(const, ["m"], ["cv::Mat*"]), _)]),
	void cv__OutputArray_move_const_MatR(const cv::_OutputArray* instance, cv::Mat* m, ResultVoid* ocvrs_return) {
		try {
			instance->move(*m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::_OutputArray::to__InputArray() generated
	// ("cv::_OutputArray::to__InputArray", vec![(pred!(mut, [], []), _)]),
	cv::_InputArray* cv__OutputArray_to__InputArray(cv::_OutputArray* instance) {
			return dynamic_cast<cv::_InputArray*>(instance);
	}

	// cv::_OutputArray::delete() generated
	// ("cv::_OutputArray::delete", vec![(pred!(mut, [], []), _)]),
	void cv__OutputArray_delete(cv::_OutputArray* instance) {
			delete instance;
	}

	// BufferPool(Stream &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:486
	// ("cv::cuda::BufferPool::BufferPool", vec![(pred!(mut, ["stream"], ["cv::cuda::Stream*"]), _)]),
	void cv_cuda_BufferPool_BufferPool_StreamR(cv::cuda::Stream* stream, Result<cv::cuda::BufferPool*>* ocvrs_return) {
		try {
			cv::cuda::BufferPool* ret = new cv::cuda::BufferPool(*stream);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBuffer(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:489
	// ("cv::cuda::BufferPool::getBuffer", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
	void cv_cuda_BufferPool_getBuffer_int_int_int(cv::cuda::BufferPool* instance, int rows, int cols, int type, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->getBuffer(rows, cols, type);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBuffer(Size, int)(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:497
	// ("cv::cuda::BufferPool::getBuffer", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
	void cv_cuda_BufferPool_getBuffer_Size_int(cv::cuda::BufferPool* instance, cv::Size* size, int type, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->getBuffer(*size, type);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAllocator()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:503
	// ("cv::cuda::BufferPool::getAllocator", vec![(pred!(const, [], []), _)]),
	void cv_cuda_BufferPool_getAllocator_const(const cv::cuda::BufferPool* instance, Result<cv::Ptr<cv::cuda::GpuMat::Allocator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::GpuMat::Allocator> ret = instance->getAllocator();
			Ok(new cv::Ptr<cv::cuda::GpuMat::Allocator>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::BufferPool::delete() generated
	// ("cv::cuda::BufferPool::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_BufferPool_delete(cv::cuda::BufferPool* instance) {
			delete instance;
	}

	// DeviceInfo()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:835
	// ("cv::cuda::DeviceInfo::DeviceInfo", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_DeviceInfo_DeviceInfo(Result<cv::cuda::DeviceInfo*>* ocvrs_return) {
		try {
			cv::cuda::DeviceInfo* ret = new cv::cuda::DeviceInfo();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DeviceInfo(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:844
	// ("cv::cuda::DeviceInfo::DeviceInfo", vec![(pred!(mut, ["device_id"], ["int"]), _)]),
	void cv_cuda_DeviceInfo_DeviceInfo_int(int device_id, Result<cv::cuda::DeviceInfo*>* ocvrs_return) {
		try {
			cv::cuda::DeviceInfo* ret = new cv::cuda::DeviceInfo(device_id);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// deviceID()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:848
	// ("cv::cuda::DeviceInfo::deviceID", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_deviceID_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->deviceID();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// name()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:851
	// ("cv::cuda::DeviceInfo::name", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_name_const(const cv::cuda::DeviceInfo* instance, Result<void*>* ocvrs_return) {
		try {
			const char* ret = instance->name();
			Ok(ocvrs_create_string(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// totalGlobalMem()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:854
	// ("cv::cuda::DeviceInfo::totalGlobalMem", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_totalGlobalMem_const(const cv::cuda::DeviceInfo* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->totalGlobalMem();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// sharedMemPerBlock()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:857
	// ("cv::cuda::DeviceInfo::sharedMemPerBlock", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_sharedMemPerBlock_const(const cv::cuda::DeviceInfo* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->sharedMemPerBlock();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// regsPerBlock()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:860
	// ("cv::cuda::DeviceInfo::regsPerBlock", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_regsPerBlock_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->regsPerBlock();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:863
	// ("cv::cuda::DeviceInfo::warpSize", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_warpSize_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->warpSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// memPitch()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:866
	// ("cv::cuda::DeviceInfo::memPitch", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_memPitch_const(const cv::cuda::DeviceInfo* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->memPitch();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxThreadsPerBlock()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:869
	// ("cv::cuda::DeviceInfo::maxThreadsPerBlock", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_maxThreadsPerBlock_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxThreadsPerBlock();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxThreadsDim()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:872
	// ("cv::cuda::DeviceInfo::maxThreadsDim", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_maxThreadsDim_const(const cv::cuda::DeviceInfo* instance, Result<cv::Vec3i>* ocvrs_return) {
		try {
			cv::Vec3i ret = instance->maxThreadsDim();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxGridSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:875
	// ("cv::cuda::DeviceInfo::maxGridSize", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_maxGridSize_const(const cv::cuda::DeviceInfo* instance, Result<cv::Vec3i>* ocvrs_return) {
		try {
			cv::Vec3i ret = instance->maxGridSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clockRate()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:878
	// ("cv::cuda::DeviceInfo::clockRate", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_clockRate_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->clockRate();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// totalConstMem()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:881
	// ("cv::cuda::DeviceInfo::totalConstMem", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_totalConstMem_const(const cv::cuda::DeviceInfo* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->totalConstMem();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// majorVersion()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:884
	// ("cv::cuda::DeviceInfo::majorVersion", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_majorVersion_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->majorVersion();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// minorVersion()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:887
	// ("cv::cuda::DeviceInfo::minorVersion", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_minorVersion_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->minorVersion();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// textureAlignment()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:890
	// ("cv::cuda::DeviceInfo::textureAlignment", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_textureAlignment_const(const cv::cuda::DeviceInfo* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->textureAlignment();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// texturePitchAlignment()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:893
	// ("cv::cuda::DeviceInfo::texturePitchAlignment", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_texturePitchAlignment_const(const cv::cuda::DeviceInfo* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->texturePitchAlignment();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// multiProcessorCount()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:896
	// ("cv::cuda::DeviceInfo::multiProcessorCount", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_multiProcessorCount_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->multiProcessorCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// kernelExecTimeoutEnabled()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:899
	// ("cv::cuda::DeviceInfo::kernelExecTimeoutEnabled", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_kernelExecTimeoutEnabled_const(const cv::cuda::DeviceInfo* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->kernelExecTimeoutEnabled();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// integrated()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:902
	// ("cv::cuda::DeviceInfo::integrated", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_integrated_const(const cv::cuda::DeviceInfo* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->integrated();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// canMapHostMemory()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:905
	// ("cv::cuda::DeviceInfo::canMapHostMemory", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_canMapHostMemory_const(const cv::cuda::DeviceInfo* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->canMapHostMemory();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeMode()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:916
	// ("cv::cuda::DeviceInfo::computeMode", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_computeMode_const(const cv::cuda::DeviceInfo* instance, Result<cv::cuda::DeviceInfo::ComputeMode>* ocvrs_return) {
		try {
			cv::cuda::DeviceInfo::ComputeMode ret = instance->computeMode();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxTexture1D()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:919
	// ("cv::cuda::DeviceInfo::maxTexture1D", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_maxTexture1D_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxTexture1D();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxTexture1DMipmap()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:922
	// ("cv::cuda::DeviceInfo::maxTexture1DMipmap", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_maxTexture1DMipmap_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxTexture1DMipmap();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxTexture1DLinear()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:925
	// ("cv::cuda::DeviceInfo::maxTexture1DLinear", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_maxTexture1DLinear_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxTexture1DLinear();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxTexture2D()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:928
	// ("cv::cuda::DeviceInfo::maxTexture2D", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_maxTexture2D_const(const cv::cuda::DeviceInfo* instance, Result<cv::Vec2i>* ocvrs_return) {
		try {
			cv::Vec2i ret = instance->maxTexture2D();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxTexture2DMipmap()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:931
	// ("cv::cuda::DeviceInfo::maxTexture2DMipmap", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_maxTexture2DMipmap_const(const cv::cuda::DeviceInfo* instance, Result<cv::Vec2i>* ocvrs_return) {
		try {
			cv::Vec2i ret = instance->maxTexture2DMipmap();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxTexture2DLinear()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:934
	// ("cv::cuda::DeviceInfo::maxTexture2DLinear", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_maxTexture2DLinear_const(const cv::cuda::DeviceInfo* instance, Result<cv::Vec3i>* ocvrs_return) {
		try {
			cv::Vec3i ret = instance->maxTexture2DLinear();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxTexture2DGather()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:937
	// ("cv::cuda::DeviceInfo::maxTexture2DGather", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_maxTexture2DGather_const(const cv::cuda::DeviceInfo* instance, Result<cv::Vec2i>* ocvrs_return) {
		try {
			cv::Vec2i ret = instance->maxTexture2DGather();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxTexture3D()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:940
	// ("cv::cuda::DeviceInfo::maxTexture3D", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_maxTexture3D_const(const cv::cuda::DeviceInfo* instance, Result<cv::Vec3i>* ocvrs_return) {
		try {
			cv::Vec3i ret = instance->maxTexture3D();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxTextureCubemap()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:943
	// ("cv::cuda::DeviceInfo::maxTextureCubemap", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_maxTextureCubemap_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxTextureCubemap();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxTexture1DLayered()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:946
	// ("cv::cuda::DeviceInfo::maxTexture1DLayered", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_maxTexture1DLayered_const(const cv::cuda::DeviceInfo* instance, Result<cv::Vec2i>* ocvrs_return) {
		try {
			cv::Vec2i ret = instance->maxTexture1DLayered();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxTexture2DLayered()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:949
	// ("cv::cuda::DeviceInfo::maxTexture2DLayered", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_maxTexture2DLayered_const(const cv::cuda::DeviceInfo* instance, Result<cv::Vec3i>* ocvrs_return) {
		try {
			cv::Vec3i ret = instance->maxTexture2DLayered();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxTextureCubemapLayered()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:952
	// ("cv::cuda::DeviceInfo::maxTextureCubemapLayered", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_maxTextureCubemapLayered_const(const cv::cuda::DeviceInfo* instance, Result<cv::Vec2i>* ocvrs_return) {
		try {
			cv::Vec2i ret = instance->maxTextureCubemapLayered();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxSurface1D()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:955
	// ("cv::cuda::DeviceInfo::maxSurface1D", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_maxSurface1D_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxSurface1D();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxSurface2D()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:958
	// ("cv::cuda::DeviceInfo::maxSurface2D", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_maxSurface2D_const(const cv::cuda::DeviceInfo* instance, Result<cv::Vec2i>* ocvrs_return) {
		try {
			cv::Vec2i ret = instance->maxSurface2D();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxSurface3D()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:961
	// ("cv::cuda::DeviceInfo::maxSurface3D", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_maxSurface3D_const(const cv::cuda::DeviceInfo* instance, Result<cv::Vec3i>* ocvrs_return) {
		try {
			cv::Vec3i ret = instance->maxSurface3D();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxSurface1DLayered()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:964
	// ("cv::cuda::DeviceInfo::maxSurface1DLayered", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_maxSurface1DLayered_const(const cv::cuda::DeviceInfo* instance, Result<cv::Vec2i>* ocvrs_return) {
		try {
			cv::Vec2i ret = instance->maxSurface1DLayered();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxSurface2DLayered()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:967
	// ("cv::cuda::DeviceInfo::maxSurface2DLayered", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_maxSurface2DLayered_const(const cv::cuda::DeviceInfo* instance, Result<cv::Vec3i>* ocvrs_return) {
		try {
			cv::Vec3i ret = instance->maxSurface2DLayered();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxSurfaceCubemap()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:970
	// ("cv::cuda::DeviceInfo::maxSurfaceCubemap", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_maxSurfaceCubemap_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxSurfaceCubemap();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxSurfaceCubemapLayered()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:973
	// ("cv::cuda::DeviceInfo::maxSurfaceCubemapLayered", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_maxSurfaceCubemapLayered_const(const cv::cuda::DeviceInfo* instance, Result<cv::Vec2i>* ocvrs_return) {
		try {
			cv::Vec2i ret = instance->maxSurfaceCubemapLayered();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// surfaceAlignment()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:976
	// ("cv::cuda::DeviceInfo::surfaceAlignment", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_surfaceAlignment_const(const cv::cuda::DeviceInfo* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->surfaceAlignment();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// concurrentKernels()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:979
	// ("cv::cuda::DeviceInfo::concurrentKernels", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_concurrentKernels_const(const cv::cuda::DeviceInfo* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->concurrentKernels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ECCEnabled()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:982
	// ("cv::cuda::DeviceInfo::ECCEnabled", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_ECCEnabled_const(const cv::cuda::DeviceInfo* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->ECCEnabled();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// pciBusID()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:985
	// ("cv::cuda::DeviceInfo::pciBusID", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_pciBusID_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->pciBusID();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// pciDeviceID()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:988
	// ("cv::cuda::DeviceInfo::pciDeviceID", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_pciDeviceID_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->pciDeviceID();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// pciDomainID()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:991
	// ("cv::cuda::DeviceInfo::pciDomainID", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_pciDomainID_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->pciDomainID();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// tccDriver()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:994
	// ("cv::cuda::DeviceInfo::tccDriver", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_tccDriver_const(const cv::cuda::DeviceInfo* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->tccDriver();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// asyncEngineCount()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:997
	// ("cv::cuda::DeviceInfo::asyncEngineCount", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_asyncEngineCount_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->asyncEngineCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// unifiedAddressing()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:1000
	// ("cv::cuda::DeviceInfo::unifiedAddressing", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_unifiedAddressing_const(const cv::cuda::DeviceInfo* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->unifiedAddressing();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// memoryClockRate()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:1003
	// ("cv::cuda::DeviceInfo::memoryClockRate", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_memoryClockRate_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->memoryClockRate();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// memoryBusWidth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:1006
	// ("cv::cuda::DeviceInfo::memoryBusWidth", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_memoryBusWidth_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->memoryBusWidth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// l2CacheSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:1009
	// ("cv::cuda::DeviceInfo::l2CacheSize", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_l2CacheSize_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->l2CacheSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxThreadsPerMultiProcessor()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:1012
	// ("cv::cuda::DeviceInfo::maxThreadsPerMultiProcessor", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_maxThreadsPerMultiProcessor_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxThreadsPerMultiProcessor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// queryMemory(size_t &, size_t &)(Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:1015
	// ("cv::cuda::DeviceInfo::queryMemory", vec![(pred!(const, ["totalMemory", "freeMemory"], ["size_t*", "size_t*"]), _)]),
	void cv_cuda_DeviceInfo_queryMemory_const_size_tR_size_tR(const cv::cuda::DeviceInfo* instance, size_t* totalMemory, size_t* freeMemory, ResultVoid* ocvrs_return) {
		try {
			instance->queryMemory(*totalMemory, *freeMemory);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// freeMemory()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:1016
	// ("cv::cuda::DeviceInfo::freeMemory", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_freeMemory_const(const cv::cuda::DeviceInfo* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->freeMemory();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// totalMemory()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:1017
	// ("cv::cuda::DeviceInfo::totalMemory", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_totalMemory_const(const cv::cuda::DeviceInfo* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->totalMemory();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// supports(FeatureSet)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:1025
	// ("cv::cuda::DeviceInfo::supports", vec![(pred!(const, ["feature_set"], ["cv::cuda::FeatureSet"]), _)]),
	void cv_cuda_DeviceInfo_supports_const_FeatureSet(const cv::cuda::DeviceInfo* instance, cv::cuda::FeatureSet feature_set, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->supports(feature_set);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isCompatible()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:1032
	// ("cv::cuda::DeviceInfo::isCompatible", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DeviceInfo_isCompatible_const(const cv::cuda::DeviceInfo* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isCompatible();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::DeviceInfo::delete() generated
	// ("cv::cuda::DeviceInfo::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_DeviceInfo_delete(cv::cuda::DeviceInfo* instance) {
			delete instance;
	}

	// Event(CreateFlags)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:714
	// ("cv::cuda::Event::Event", vec![(pred!(mut, ["flags"], ["cv::cuda::Event::CreateFlags"]), _)]),
	void cv_cuda_Event_Event_CreateFlags(cv::cuda::Event::CreateFlags flags, Result<cv::cuda::Event*>* ocvrs_return) {
		try {
			cv::cuda::Event* ret = new cv::cuda::Event(flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::Event::Event() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:714
	// ("cv::cuda::Event::Event", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_Event_Event(Result<cv::cuda::Event*>* ocvrs_return) {
		try {
			cv::cuda::Event* ret = new cv::cuda::Event();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// record(Stream &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:717
	// ("cv::cuda::Event::record", vec![(pred!(mut, ["stream"], ["cv::cuda::Stream*"]), _)]),
	void cv_cuda_Event_record_StreamR(cv::cuda::Event* instance, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->record(*stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::Event::record() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:717
	// ("cv::cuda::Event::record", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_Event_record(cv::cuda::Event* instance, ResultVoid* ocvrs_return) {
		try {
			instance->record();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// queryIfComplete()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:720
	// ("cv::cuda::Event::queryIfComplete", vec![(pred!(const, [], []), _)]),
	void cv_cuda_Event_queryIfComplete_const(const cv::cuda::Event* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->queryIfComplete();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// waitForCompletion()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:723
	// ("cv::cuda::Event::waitForCompletion", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_Event_waitForCompletion(cv::cuda::Event* instance, ResultVoid* ocvrs_return) {
		try {
			instance->waitForCompletion();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// elapsedTime(const Event &, const Event &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:726
	// ("cv::cuda::Event::elapsedTime", vec![(pred!(mut, ["start", "end"], ["const cv::cuda::Event*", "const cv::cuda::Event*"]), _)]),
	void cv_cuda_Event_elapsedTime_const_EventR_const_EventR(const cv::cuda::Event* start, const cv::cuda::Event* end, Result<float>* ocvrs_return) {
		try {
			float ret = cv::cuda::Event::elapsedTime(*start, *end);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::Event::delete() generated
	// ("cv::cuda::Event::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_Event_delete(cv::cuda::Event* instance) {
			delete instance;
	}

	// defaultAllocator()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:119
	// ("cv::cuda::GpuMat::defaultAllocator", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_GpuMat_defaultAllocator(Result<cv::cuda::GpuMat::Allocator*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat::Allocator* ret = cv::cuda::GpuMat::defaultAllocator();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDefaultAllocator(Allocator *)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:120
	// ("cv::cuda::GpuMat::setDefaultAllocator", vec![(pred!(mut, ["allocator"], ["cv::cuda::GpuMat::Allocator*"]), _)]),
	void cv_cuda_GpuMat_setDefaultAllocator_AllocatorX(cv::cuda::GpuMat::Allocator* allocator, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::GpuMat::setDefaultAllocator(allocator);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GpuMat(Allocator *)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:123
	// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["allocator"], ["cv::cuda::GpuMat::Allocator*"]), _)]),
	void cv_cuda_GpuMat_GpuMat_AllocatorX(cv::cuda::GpuMat::Allocator* allocator, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(allocator);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::GpuMat::GpuMat() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:123
	// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_GpuMat_GpuMat(Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GpuMat(int, int, int, Allocator *)(Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:126
	// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["rows", "cols", "type", "allocator"], ["int", "int", "int", "cv::cuda::GpuMat::Allocator*"]), _)]),
	void cv_cuda_GpuMat_GpuMat_int_int_int_AllocatorX(int rows, int cols, int type, cv::cuda::GpuMat::Allocator* allocator, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(rows, cols, type, allocator);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::GpuMat::GpuMat(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:126
	// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
	void cv_cuda_GpuMat_GpuMat_int_int_int(int rows, int cols, int type, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(rows, cols, type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GpuMat(Size, int, Allocator *)(SimpleClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:127
	// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["size", "type", "allocator"], ["cv::Size", "int", "cv::cuda::GpuMat::Allocator*"]), _)]),
	void cv_cuda_GpuMat_GpuMat_Size_int_AllocatorX(cv::Size* size, int type, cv::cuda::GpuMat::Allocator* allocator, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(*size, type, allocator);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::GpuMat::GpuMat(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:127
	// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
	void cv_cuda_GpuMat_GpuMat_Size_int(cv::Size* size, int type, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(*size, type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GpuMat(int, int, int, Scalar, Allocator *)(Primitive, Primitive, Primitive, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:130
	// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["rows", "cols", "type", "s", "allocator"], ["int", "int", "int", "cv::Scalar", "cv::cuda::GpuMat::Allocator*"]), _)]),
	void cv_cuda_GpuMat_GpuMat_int_int_int_Scalar_AllocatorX(int rows, int cols, int type, cv::Scalar* s, cv::cuda::GpuMat::Allocator* allocator, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(rows, cols, type, *s, allocator);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::GpuMat::GpuMat(Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:130
	// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["rows", "cols", "type", "s"], ["int", "int", "int", "cv::Scalar"]), _)]),
	void cv_cuda_GpuMat_GpuMat_int_int_int_Scalar(int rows, int cols, int type, cv::Scalar* s, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(rows, cols, type, *s);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GpuMat(Size, int, Scalar, Allocator *)(SimpleClass, Primitive, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:131
	// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["size", "type", "s", "allocator"], ["cv::Size", "int", "cv::Scalar", "cv::cuda::GpuMat::Allocator*"]), _)]),
	void cv_cuda_GpuMat_GpuMat_Size_int_Scalar_AllocatorX(cv::Size* size, int type, cv::Scalar* s, cv::cuda::GpuMat::Allocator* allocator, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(*size, type, *s, allocator);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::GpuMat::GpuMat(SimpleClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:131
	// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["size", "type", "s"], ["cv::Size", "int", "cv::Scalar"]), _)]),
	void cv_cuda_GpuMat_GpuMat_Size_int_Scalar(cv::Size* size, int type, cv::Scalar* s, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(*size, type, *s);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GpuMat(const GpuMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:134
	// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["m"], ["const cv::cuda::GpuMat*"]), _)]),
	void cv_cuda_GpuMat_GpuMat_const_GpuMatR(const cv::cuda::GpuMat* m, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GpuMat(int, int, int, void *, size_t)(Primitive, Primitive, Primitive, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:137
	// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["rows", "cols", "type", "data", "step"], ["int", "int", "int", "void*", "size_t"]), _)]),
	void cv_cuda_GpuMat_GpuMat_int_int_int_voidX_size_t(int rows, int cols, int type, void* data, size_t step, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(rows, cols, type, data, step);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::GpuMat::GpuMat(Primitive, Primitive, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:137
	// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["rows", "cols", "type", "data"], ["int", "int", "int", "void*"]), _)]),
	void cv_cuda_GpuMat_GpuMat_int_int_int_voidX(int rows, int cols, int type, void* data, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(rows, cols, type, data);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GpuMat(Size, int, void *, size_t)(SimpleClass, Primitive, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:138
	// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["size", "type", "data", "step"], ["cv::Size", "int", "void*", "size_t"]), _)]),
	void cv_cuda_GpuMat_GpuMat_Size_int_voidX_size_t(cv::Size* size, int type, void* data, size_t step, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(*size, type, data, step);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::GpuMat::GpuMat(SimpleClass, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:138
	// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["size", "type", "data"], ["cv::Size", "int", "void*"]), _)]),
	void cv_cuda_GpuMat_GpuMat_Size_int_voidX(cv::Size* size, int type, void* data, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(*size, type, data);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GpuMat(const GpuMat &, Range, Range)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:141
	// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["m", "rowRange", "colRange"], ["const cv::cuda::GpuMat*", "cv::Range", "cv::Range"]), _)]),
	void cv_cuda_GpuMat_GpuMat_const_GpuMatR_Range_Range(const cv::cuda::GpuMat* m, cv::Range* rowRange, cv::Range* colRange, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			const cv::cuda::GpuMat* ret = new const cv::cuda::GpuMat(*m, *rowRange, *colRange);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::GpuMat::GpuMat(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:141
	// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["m", "rowRange", "colRange"], ["cv::cuda::GpuMat*", "cv::Range", "cv::Range"]), _)]),
	void cv_cuda_GpuMat_GpuMat_GpuMatR_Range_Range(cv::cuda::GpuMat* m, cv::Range* rowRange, cv::Range* colRange, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(*m, *rowRange, *colRange);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GpuMat(const GpuMat &, Rect)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:142
	// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["m", "roi"], ["const cv::cuda::GpuMat*", "cv::Rect"]), _)]),
	void cv_cuda_GpuMat_GpuMat_const_GpuMatR_Rect(const cv::cuda::GpuMat* m, cv::Rect* roi, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			const cv::cuda::GpuMat* ret = new const cv::cuda::GpuMat(*m, *roi);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::GpuMat::GpuMat(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:142
	// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["m", "roi"], ["cv::cuda::GpuMat*", "cv::Rect"]), _)]),
	void cv_cuda_GpuMat_GpuMat_GpuMatR_Rect(cv::cuda::GpuMat* m, cv::Rect* roi, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(*m, *roi);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GpuMat(InputArray, Allocator *)(InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:145
	// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["arr", "allocator"], ["const cv::_InputArray*", "cv::cuda::GpuMat::Allocator*"]), _)]),
	void cv_cuda_GpuMat_GpuMat_const__InputArrayR_AllocatorX(const cv::_InputArray* arr, cv::cuda::GpuMat::Allocator* allocator, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(*arr, allocator);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::GpuMat::GpuMat(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:145
	// ("cv::cuda::GpuMat::GpuMat", vec![(pred!(mut, ["arr"], ["const cv::_InputArray*"]), _)]),
	void cv_cuda_GpuMat_GpuMat_const__InputArrayR(const cv::_InputArray* arr, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(*arr);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const GpuMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:151
	// ("cv::cuda::GpuMat::operator=", vec![(pred!(mut, ["m"], ["const cv::cuda::GpuMat*"]), _)]),
	void cv_cuda_GpuMat_operatorST_const_GpuMatR(cv::cuda::GpuMat* instance, const cv::cuda::GpuMat* m, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:154
	// ("cv::cuda::GpuMat::create", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
	void cv_cuda_GpuMat_create_int_int_int(cv::cuda::GpuMat* instance, int rows, int cols, int type, ResultVoid* ocvrs_return) {
		try {
			instance->create(rows, cols, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(Size, int)(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:155
	// ("cv::cuda::GpuMat::create", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
	void cv_cuda_GpuMat_create_Size_int(cv::cuda::GpuMat* instance, cv::Size* size, int type, ResultVoid* ocvrs_return) {
		try {
			instance->create(*size, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// release()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:158
	// ("cv::cuda::GpuMat::release", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_GpuMat_release(cv::cuda::GpuMat* instance, ResultVoid* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// swap(GpuMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:161
	// ("cv::cuda::GpuMat::swap", vec![(pred!(mut, ["mat"], ["cv::cuda::GpuMat*"]), _)]),
	void cv_cuda_GpuMat_swap_GpuMatR(cv::cuda::GpuMat* instance, cv::cuda::GpuMat* mat, ResultVoid* ocvrs_return) {
		try {
			instance->swap(*mat);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// upload(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:168
	// ("cv::cuda::GpuMat::upload", vec![(pred!(mut, ["arr"], ["const cv::_InputArray*"]), _)]),
	void cv_cuda_GpuMat_upload_const__InputArrayR(cv::cuda::GpuMat* instance, const cv::_InputArray* arr, ResultVoid* ocvrs_return) {
		try {
			instance->upload(*arr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// upload(InputArray, Stream &)(InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:178
	// ("cv::cuda::GpuMat::upload", vec![(pred!(mut, ["arr", "stream"], ["const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_GpuMat_upload_const__InputArrayR_StreamR(cv::cuda::GpuMat* instance, const cv::_InputArray* arr, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->upload(*arr, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// download(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:185
	// ("cv::cuda::GpuMat::download", vec![(pred!(const, ["dst"], ["const cv::_OutputArray*"]), _)]),
	void cv_cuda_GpuMat_download_const_const__OutputArrayR(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->download(*dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// download(OutputArray, Stream &)(OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:195
	// ("cv::cuda::GpuMat::download", vec![(pred!(const, ["dst", "stream"], ["const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_GpuMat_download_const_const__OutputArrayR_StreamR(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->download(*dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clone()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:198
	// ("cv::cuda::GpuMat::clone", vec![(pred!(const, [], []), _)]),
	void cv_cuda_GpuMat_clone_const(const cv::cuda::GpuMat* instance, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->clone();
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// copyTo(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:201
	// ("cv::cuda::GpuMat::copyTo", vec![(pred!(const, ["dst"], ["const cv::_OutputArray*"]), _)]),
	void cv_cuda_GpuMat_copyTo_const_const__OutputArrayR(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->copyTo(*dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// copyTo(OutputArray, Stream &)(OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:204
	// ("cv::cuda::GpuMat::copyTo", vec![(pred!(const, ["dst", "stream"], ["const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_GpuMat_copyTo_const_const__OutputArrayR_StreamR(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->copyTo(*dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// copyTo(OutputArray, InputArray)(OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:207
	// ("cv::cuda::GpuMat::copyTo", vec![(pred!(const, ["dst", "mask"], ["const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_cuda_GpuMat_copyTo_const_const__OutputArrayR_const__InputArrayR(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			instance->copyTo(*dst, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// copyTo(OutputArray, InputArray, Stream &)(OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:210
	// ("cv::cuda::GpuMat::copyTo", vec![(pred!(const, ["dst", "mask", "stream"], ["const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_GpuMat_copyTo_const_const__OutputArrayR_const__InputArrayR_StreamR(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, const cv::_InputArray* mask, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->copyTo(*dst, *mask, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTo(Scalar)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:213
	// ("cv::cuda::GpuMat::setTo", vec![(pred!(mut, ["s"], ["cv::Scalar"]), _)]),
	void cv_cuda_GpuMat_setTo_Scalar(cv::cuda::GpuMat* instance, cv::Scalar* s, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->setTo(*s);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTo(Scalar, Stream &)(SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:216
	// ("cv::cuda::GpuMat::setTo", vec![(pred!(mut, ["s", "stream"], ["cv::Scalar", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_GpuMat_setTo_Scalar_StreamR(cv::cuda::GpuMat* instance, cv::Scalar* s, cv::cuda::Stream* stream, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->setTo(*s, *stream);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTo(Scalar, InputArray)(SimpleClass, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:219
	// ("cv::cuda::GpuMat::setTo", vec![(pred!(mut, ["s", "mask"], ["cv::Scalar", "const cv::_InputArray*"]), _)]),
	void cv_cuda_GpuMat_setTo_Scalar_const__InputArrayR(cv::cuda::GpuMat* instance, cv::Scalar* s, const cv::_InputArray* mask, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->setTo(*s, *mask);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTo(Scalar, InputArray, Stream &)(SimpleClass, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:222
	// ("cv::cuda::GpuMat::setTo", vec![(pred!(mut, ["s", "mask", "stream"], ["cv::Scalar", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_GpuMat_setTo_Scalar_const__InputArrayR_StreamR(cv::cuda::GpuMat* instance, cv::Scalar* s, const cv::_InputArray* mask, cv::cuda::Stream* stream, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->setTo(*s, *mask, *stream);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertTo(OutputArray, int)(OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:225
	// ("cv::cuda::GpuMat::convertTo", vec![(pred!(const, ["dst", "rtype"], ["const cv::_OutputArray*", "int"]), _)]),
	void cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, int rtype, ResultVoid* ocvrs_return) {
		try {
			instance->convertTo(*dst, rtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertTo(OutputArray, int, Stream &)(OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:228
	// ("cv::cuda::GpuMat::convertTo", vec![(pred!(const, ["dst", "rtype", "stream"], ["const cv::_OutputArray*", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int_StreamR(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, int rtype, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->convertTo(*dst, rtype, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertTo(OutputArray, int, double, double)(OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:231
	// ("cv::cuda::GpuMat::convertTo", vec![(pred!(const, ["dst", "rtype", "alpha", "beta"], ["const cv::_OutputArray*", "int", "double", "double"]), _)]),
	void cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int_double_double(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, int rtype, double alpha, double beta, ResultVoid* ocvrs_return) {
		try {
			instance->convertTo(*dst, rtype, alpha, beta);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::GpuMat::convertTo(OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:231
	// ("cv::cuda::GpuMat::convertTo", vec![(pred!(const, ["dst", "rtype", "alpha"], ["const cv::_OutputArray*", "int", "double"]), _)]),
	void cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int_double(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, int rtype, double alpha, ResultVoid* ocvrs_return) {
		try {
			instance->convertTo(*dst, rtype, alpha);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertTo(OutputArray, int, double, Stream &)(OutputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:234
	// ("cv::cuda::GpuMat::convertTo", vec![(pred!(const, ["dst", "rtype", "alpha", "stream"], ["const cv::_OutputArray*", "int", "double", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int_double_StreamR(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, int rtype, double alpha, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->convertTo(*dst, rtype, alpha, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertTo(OutputArray, int, double, double, Stream &)(OutputArray, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:237
	// ("cv::cuda::GpuMat::convertTo", vec![(pred!(const, ["dst", "rtype", "alpha", "beta", "stream"], ["const cv::_OutputArray*", "int", "double", "double", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int_double_double_StreamR(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, int rtype, double alpha, double beta, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->convertTo(*dst, rtype, alpha, beta, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// assignTo(GpuMat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:239
	// ("cv::cuda::GpuMat::assignTo", vec![(pred!(const, ["m", "type"], ["cv::cuda::GpuMat*", "int"]), _)]),
	void cv_cuda_GpuMat_assignTo_const_GpuMatR_int(const cv::cuda::GpuMat* instance, cv::cuda::GpuMat* m, int type, ResultVoid* ocvrs_return) {
		try {
			instance->assignTo(*m, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::GpuMat::assignTo(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:239
	// ("cv::cuda::GpuMat::assignTo", vec![(pred!(const, ["m"], ["cv::cuda::GpuMat*"]), _)]),
	void cv_cuda_GpuMat_assignTo_const_GpuMatR(const cv::cuda::GpuMat* instance, cv::cuda::GpuMat* m, ResultVoid* ocvrs_return) {
		try {
			instance->assignTo(*m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ptr(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:242
	// ("cv::cuda::GpuMat::ptr", vec![(pred!(mut, ["y"], ["int"]), _)]),
	void cv_cuda_GpuMat_ptr_int(cv::cuda::GpuMat* instance, int y, Result<unsigned char*>* ocvrs_return) {
		try {
			unsigned char* ret = instance->ptr(y);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::GpuMat::ptr() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:242
	// ("cv::cuda::GpuMat::ptr", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_GpuMat_ptr(cv::cuda::GpuMat* instance, Result<unsigned char*>* ocvrs_return) {
		try {
			unsigned char* ret = instance->ptr();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ptr(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:243
	// ("cv::cuda::GpuMat::ptr", vec![(pred!(const, ["y"], ["int"]), _)]),
	void cv_cuda_GpuMat_ptr_const_int(const cv::cuda::GpuMat* instance, int y, Result<const unsigned char*>* ocvrs_return) {
		try {
			const unsigned char* ret = instance->ptr(y);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::GpuMat::ptr() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:243
	// ("cv::cuda::GpuMat::ptr", vec![(pred!(const, [], []), _)]),
	void cv_cuda_GpuMat_ptr_const(const cv::cuda::GpuMat* instance, Result<const unsigned char*>* ocvrs_return) {
		try {
			const unsigned char* ret = instance->ptr();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// row(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:253
	// ("cv::cuda::GpuMat::row", vec![(pred!(const, ["y"], ["int"]), _)]),
	void cv_cuda_GpuMat_row_const_int(const cv::cuda::GpuMat* instance, int y, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			const cv::cuda::GpuMat ret = instance->row(y);
			Ok(new const cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::GpuMat::row(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:253
	// ("cv::cuda::GpuMat::row", vec![(pred!(mut, ["y"], ["int"]), _)]),
	void cv_cuda_GpuMat_row_int(cv::cuda::GpuMat* instance, int y, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->row(y);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// col(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:256
	// ("cv::cuda::GpuMat::col", vec![(pred!(const, ["x"], ["int"]), _)]),
	void cv_cuda_GpuMat_col_const_int(const cv::cuda::GpuMat* instance, int x, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			const cv::cuda::GpuMat ret = instance->col(x);
			Ok(new const cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::GpuMat::col(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:256
	// ("cv::cuda::GpuMat::col", vec![(pred!(mut, ["x"], ["int"]), _)]),
	void cv_cuda_GpuMat_col_int(cv::cuda::GpuMat* instance, int x, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->col(x);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// rowRange(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:259
	// ("cv::cuda::GpuMat::rowRange", vec![(pred!(const, ["startrow", "endrow"], ["int", "int"]), _)]),
	void cv_cuda_GpuMat_rowRange_const_int_int(const cv::cuda::GpuMat* instance, int startrow, int endrow, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			const cv::cuda::GpuMat ret = instance->rowRange(startrow, endrow);
			Ok(new const cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::GpuMat::rowRange(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:259
	// ("cv::cuda::GpuMat::rowRange", vec![(pred!(mut, ["startrow", "endrow"], ["int", "int"]), _)]),
	void cv_cuda_GpuMat_rowRange_int_int(cv::cuda::GpuMat* instance, int startrow, int endrow, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->rowRange(startrow, endrow);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// rowRange(Range)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:260
	// ("cv::cuda::GpuMat::rowRange", vec![(pred!(const, ["r"], ["cv::Range"]), _)]),
	void cv_cuda_GpuMat_rowRange_const_Range(const cv::cuda::GpuMat* instance, cv::Range* r, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			const cv::cuda::GpuMat ret = instance->rowRange(*r);
			Ok(new const cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::GpuMat::rowRange(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:260
	// ("cv::cuda::GpuMat::rowRange", vec![(pred!(mut, ["r"], ["cv::Range"]), _)]),
	void cv_cuda_GpuMat_rowRange_Range(cv::cuda::GpuMat* instance, cv::Range* r, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->rowRange(*r);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// colRange(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:263
	// ("cv::cuda::GpuMat::colRange", vec![(pred!(const, ["startcol", "endcol"], ["int", "int"]), _)]),
	void cv_cuda_GpuMat_colRange_const_int_int(const cv::cuda::GpuMat* instance, int startcol, int endcol, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			const cv::cuda::GpuMat ret = instance->colRange(startcol, endcol);
			Ok(new const cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::GpuMat::colRange(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:263
	// ("cv::cuda::GpuMat::colRange", vec![(pred!(mut, ["startcol", "endcol"], ["int", "int"]), _)]),
	void cv_cuda_GpuMat_colRange_int_int(cv::cuda::GpuMat* instance, int startcol, int endcol, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->colRange(startcol, endcol);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// colRange(Range)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:264
	// ("cv::cuda::GpuMat::colRange", vec![(pred!(const, ["r"], ["cv::Range"]), _)]),
	void cv_cuda_GpuMat_colRange_const_Range(const cv::cuda::GpuMat* instance, cv::Range* r, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			const cv::cuda::GpuMat ret = instance->colRange(*r);
			Ok(new const cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::GpuMat::colRange(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:264
	// ("cv::cuda::GpuMat::colRange", vec![(pred!(mut, ["r"], ["cv::Range"]), _)]),
	void cv_cuda_GpuMat_colRange_Range(cv::cuda::GpuMat* instance, cv::Range* r, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->colRange(*r);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(Range, Range)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:267
	// ("cv::cuda::GpuMat::operator()", vec![(pred!(const, ["rowRange", "colRange"], ["cv::Range", "cv::Range"]), _)]),
	void cv_cuda_GpuMat_operator___const_Range_Range(const cv::cuda::GpuMat* instance, cv::Range* rowRange, cv::Range* colRange, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			const cv::cuda::GpuMat ret = instance->operator()(*rowRange, *colRange);
			Ok(new const cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::GpuMat::operator()(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:267
	// ("cv::cuda::GpuMat::operator()", vec![(pred!(mut, ["rowRange", "colRange"], ["cv::Range", "cv::Range"]), _)]),
	void cv_cuda_GpuMat_operator___Range_Range(cv::cuda::GpuMat* instance, cv::Range* rowRange, cv::Range* colRange, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->operator()(*rowRange, *colRange);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(Rect)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:268
	// ("cv::cuda::GpuMat::operator()", vec![(pred!(const, ["roi"], ["cv::Rect"]), _)]),
	void cv_cuda_GpuMat_operator___const_Rect(const cv::cuda::GpuMat* instance, cv::Rect* roi, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			const cv::cuda::GpuMat ret = instance->operator()(*roi);
			Ok(new const cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::GpuMat::operator()(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:268
	// ("cv::cuda::GpuMat::operator()", vec![(pred!(mut, ["roi"], ["cv::Rect"]), _)]),
	void cv_cuda_GpuMat_operator___Rect(cv::cuda::GpuMat* instance, cv::Rect* roi, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->operator()(*roi);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reshape(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:272
	// ("cv::cuda::GpuMat::reshape", vec![(pred!(const, ["cn", "rows"], ["int", "int"]), _)]),
	void cv_cuda_GpuMat_reshape_const_int_int(const cv::cuda::GpuMat* instance, int cn, int rows, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			const cv::cuda::GpuMat ret = instance->reshape(cn, rows);
			Ok(new const cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::GpuMat::reshape(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:272
	// ("cv::cuda::GpuMat::reshape", vec![(pred!(mut, ["cn"], ["int"]), _)]),
	void cv_cuda_GpuMat_reshape_int(cv::cuda::GpuMat* instance, int cn, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->reshape(cn);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::GpuMat::reshape(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:272
	// ("cv::cuda::GpuMat::reshape", vec![(pred!(const, ["cn"], ["int"]), _)]),
	void cv_cuda_GpuMat_reshape_const_int(const cv::cuda::GpuMat* instance, int cn, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			const cv::cuda::GpuMat ret = instance->reshape(cn);
			Ok(new const cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::GpuMat::reshape(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:272
	// ("cv::cuda::GpuMat::reshape", vec![(pred!(mut, ["cn", "rows"], ["int", "int"]), _)]),
	void cv_cuda_GpuMat_reshape_int_int(cv::cuda::GpuMat* instance, int cn, int rows, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->reshape(cn, rows);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// locateROI(Size &, Point &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:275
	// ("cv::cuda::GpuMat::locateROI", vec![(pred!(const, ["wholeSize", "ofs"], ["cv::Size*", "cv::Point*"]), _)]),
	void cv_cuda_GpuMat_locateROI_const_SizeR_PointR(const cv::cuda::GpuMat* instance, cv::Size* wholeSize, cv::Point* ofs, ResultVoid* ocvrs_return) {
		try {
			instance->locateROI(*wholeSize, *ofs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// adjustROI(int, int, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:278
	// ("cv::cuda::GpuMat::adjustROI", vec![(pred!(mut, ["dtop", "dbottom", "dleft", "dright"], ["int", "int", "int", "int"]), _)]),
	void cv_cuda_GpuMat_adjustROI_int_int_int_int(cv::cuda::GpuMat* instance, int dtop, int dbottom, int dleft, int dright, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->adjustROI(dtop, dbottom, dleft, dright);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isContinuous()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:282
	// ("cv::cuda::GpuMat::isContinuous", vec![(pred!(const, [], []), _)]),
	void cv_cuda_GpuMat_isContinuous_const(const cv::cuda::GpuMat* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isContinuous();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// elemSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:285
	// ("cv::cuda::GpuMat::elemSize", vec![(pred!(const, [], []), _)]),
	void cv_cuda_GpuMat_elemSize_const(const cv::cuda::GpuMat* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->elemSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// elemSize1()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:288
	// ("cv::cuda::GpuMat::elemSize1", vec![(pred!(const, [], []), _)]),
	void cv_cuda_GpuMat_elemSize1_const(const cv::cuda::GpuMat* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->elemSize1();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// type()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:291
	// ("cv::cuda::GpuMat::type", vec![(pred!(const, [], []), _)]),
	void cv_cuda_GpuMat_type_const(const cv::cuda::GpuMat* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->type();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// depth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:294
	// ("cv::cuda::GpuMat::depth", vec![(pred!(const, [], []), _)]),
	void cv_cuda_GpuMat_depth_const(const cv::cuda::GpuMat* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->depth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// channels()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:297
	// ("cv::cuda::GpuMat::channels", vec![(pred!(const, [], []), _)]),
	void cv_cuda_GpuMat_channels_const(const cv::cuda::GpuMat* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->channels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// step1()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:300
	// ("cv::cuda::GpuMat::step1", vec![(pred!(const, [], []), _)]),
	void cv_cuda_GpuMat_step1_const(const cv::cuda::GpuMat* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->step1();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// size()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:303
	// ("cv::cuda::GpuMat::size", vec![(pred!(const, [], []), _)]),
	void cv_cuda_GpuMat_size_const(const cv::cuda::GpuMat* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->size();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:306
	// ("cv::cuda::GpuMat::empty", vec![(pred!(const, [], []), _)]),
	void cv_cuda_GpuMat_empty_const(const cv::cuda::GpuMat* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// updateContinuityFlag()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:309
	// ("cv::cuda::GpuMat::updateContinuityFlag", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_GpuMat_updateContinuityFlag(cv::cuda::GpuMat* instance, ResultVoid* ocvrs_return) {
		try {
			instance->updateContinuityFlag();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::GpuMat::flags() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:317
	// ("cv::cuda::GpuMat::flags", vec![(pred!(const, [], []), _)]),
	int cv_cuda_GpuMat_propFlags_const(const cv::cuda::GpuMat* instance) {
			int ret = instance->flags;
			return ret;
	}

	// cv::cuda::GpuMat::setFlags(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:317
	// ("cv::cuda::GpuMat::setFlags", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_cuda_GpuMat_propFlags_const_int(cv::cuda::GpuMat* instance, const int val) {
			instance->flags = val;
	}

	// cv::cuda::GpuMat::rows() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:320
	// ("cv::cuda::GpuMat::rows", vec![(pred!(const, [], []), _)]),
	int cv_cuda_GpuMat_propRows_const(const cv::cuda::GpuMat* instance) {
			int ret = instance->rows;
			return ret;
	}

	// cv::cuda::GpuMat::setRows(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:320
	// ("cv::cuda::GpuMat::setRows", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_cuda_GpuMat_propRows_const_int(cv::cuda::GpuMat* instance, const int val) {
			instance->rows = val;
	}

	// cv::cuda::GpuMat::cols() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:320
	// ("cv::cuda::GpuMat::cols", vec![(pred!(const, [], []), _)]),
	int cv_cuda_GpuMat_propCols_const(const cv::cuda::GpuMat* instance) {
			int ret = instance->cols;
			return ret;
	}

	// cv::cuda::GpuMat::setCols(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:320
	// ("cv::cuda::GpuMat::setCols", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_cuda_GpuMat_propCols_const_int(cv::cuda::GpuMat* instance, const int val) {
			instance->cols = val;
	}

	// cv::cuda::GpuMat::step() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:323
	// ("cv::cuda::GpuMat::step", vec![(pred!(const, [], []), _)]),
	size_t cv_cuda_GpuMat_propStep_const(const cv::cuda::GpuMat* instance) {
			size_t ret = instance->step;
			return ret;
	}

	// cv::cuda::GpuMat::setStep(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:323
	// ("cv::cuda::GpuMat::setStep", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
	void cv_cuda_GpuMat_propStep_const_size_t(cv::cuda::GpuMat* instance, const size_t val) {
			instance->step = val;
	}

	// cv::cuda::GpuMat::data() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:326
	// ("cv::cuda::GpuMat::data", vec![(pred!(const, [], []), _)]),
	unsigned char* cv_cuda_GpuMat_propData_const(const cv::cuda::GpuMat* instance) {
			unsigned char* const ret = instance->data;
			return ret;
	}

	// cv::cuda::GpuMat::dataMut() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:326
	// ("cv::cuda::GpuMat::dataMut", vec![(pred!(mut, [], []), _)]),
	unsigned char* cv_cuda_GpuMat_propData(cv::cuda::GpuMat* instance) {
			unsigned char* ret = instance->data;
			return ret;
	}

	// cv::cuda::GpuMat::setData(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:326
	// ("cv::cuda::GpuMat::setData", vec![(pred!(mut, ["val"], ["unsigned char*"]), _)]),
	void cv_cuda_GpuMat_propData_unsigned_charX(cv::cuda::GpuMat* instance, unsigned char* const val) {
			instance->data = val;
	}

	// cv::cuda::GpuMat::refcount() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:330
	// ("cv::cuda::GpuMat::refcount", vec![(pred!(const, [], []), _)]),
	int* cv_cuda_GpuMat_propRefcount_const(const cv::cuda::GpuMat* instance) {
			int* const ret = instance->refcount;
			return ret;
	}

	// cv::cuda::GpuMat::refcountMut() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:330
	// ("cv::cuda::GpuMat::refcountMut", vec![(pred!(mut, [], []), _)]),
	int* cv_cuda_GpuMat_propRefcount(cv::cuda::GpuMat* instance) {
			int* ret = instance->refcount;
			return ret;
	}

	// cv::cuda::GpuMat::setRefcount(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:330
	// ("cv::cuda::GpuMat::setRefcount", vec![(pred!(mut, ["val"], ["int*"]), _)]),
	void cv_cuda_GpuMat_propRefcount_intX(cv::cuda::GpuMat* instance, int* const val) {
			instance->refcount = val;
	}

	// cv::cuda::GpuMat::datastart() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:333
	// ("cv::cuda::GpuMat::datastart", vec![(pred!(const, [], []), _)]),
	unsigned char* cv_cuda_GpuMat_propDatastart_const(const cv::cuda::GpuMat* instance) {
			unsigned char* const ret = instance->datastart;
			return ret;
	}

	// cv::cuda::GpuMat::datastartMut() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:333
	// ("cv::cuda::GpuMat::datastartMut", vec![(pred!(mut, [], []), _)]),
	unsigned char* cv_cuda_GpuMat_propDatastart(cv::cuda::GpuMat* instance) {
			unsigned char* ret = instance->datastart;
			return ret;
	}

	// cv::cuda::GpuMat::setDatastart(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:333
	// ("cv::cuda::GpuMat::setDatastart", vec![(pred!(mut, ["val"], ["unsigned char*"]), _)]),
	void cv_cuda_GpuMat_propDatastart_unsigned_charX(cv::cuda::GpuMat* instance, unsigned char* const val) {
			instance->datastart = val;
	}

	// cv::cuda::GpuMat::dataend() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:334
	// ("cv::cuda::GpuMat::dataend", vec![(pred!(const, [], []), _)]),
	const unsigned char* cv_cuda_GpuMat_propDataend_const(const cv::cuda::GpuMat* instance) {
			const unsigned char* ret = instance->dataend;
			return ret;
	}

	// cv::cuda::GpuMat::allocator() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:337
	// ("cv::cuda::GpuMat::allocator", vec![(pred!(mut, [], []), _)]),
	cv::cuda::GpuMat::Allocator* cv_cuda_GpuMat_propAllocator(cv::cuda::GpuMat* instance) {
			cv::cuda::GpuMat::Allocator* ret = instance->allocator;
			return ret;
	}

	// cv::cuda::GpuMat::setAllocator(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:337
	// ("cv::cuda::GpuMat::setAllocator", vec![(pred!(mut, ["val"], ["cv::cuda::GpuMat::Allocator*"]), _)]),
	void cv_cuda_GpuMat_propAllocator_AllocatorX(cv::cuda::GpuMat* instance, cv::cuda::GpuMat::Allocator* const val) {
			instance->allocator = val;
	}

	// cv::cuda::GpuMat::delete() generated
	// ("cv::cuda::GpuMat::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_GpuMat_delete(cv::cuda::GpuMat* instance) {
			delete instance;
	}

	// allocate(GpuMat *, int, int, size_t)(TraitClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:114
	// ("cv::cuda::GpuMat::Allocator::allocate", vec![(pred!(mut, ["mat", "rows", "cols", "elemSize"], ["cv::cuda::GpuMat*", "int", "int", "size_t"]), _)]),
	void cv_cuda_GpuMat_Allocator_allocate_GpuMatX_int_int_size_t(cv::cuda::GpuMat::Allocator* instance, cv::cuda::GpuMat* mat, int rows, int cols, size_t elemSize, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->allocate(mat, rows, cols, elemSize);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// free(GpuMat *)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:115
	// ("cv::cuda::GpuMat::Allocator::free", vec![(pred!(mut, ["mat"], ["cv::cuda::GpuMat*"]), _)]),
	void cv_cuda_GpuMat_Allocator_free_GpuMatX(cv::cuda::GpuMat::Allocator* instance, cv::cuda::GpuMat* mat, ResultVoid* ocvrs_return) {
		try {
			instance->free(mat);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::GpuMat::Allocator::delete() generated
	// ("cv::cuda::GpuMat::Allocator::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_GpuMat_Allocator_delete(cv::cuda::GpuMat::Allocator* instance) {
			delete instance;
	}

	// HostMem(AllocType)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:539
	// ("cv::cuda::HostMem::HostMem", vec![(pred!(mut, ["alloc_type"], ["cv::cuda::HostMem::AllocType"]), _)]),
	void cv_cuda_HostMem_HostMem_AllocType(cv::cuda::HostMem::AllocType alloc_type, Result<cv::cuda::HostMem*>* ocvrs_return) {
		try {
			cv::cuda::HostMem* ret = new cv::cuda::HostMem(alloc_type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::HostMem::HostMem() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:539
	// ("cv::cuda::HostMem::HostMem", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_HostMem_HostMem(Result<cv::cuda::HostMem*>* ocvrs_return) {
		try {
			cv::cuda::HostMem* ret = new cv::cuda::HostMem();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// HostMem(const HostMem &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:541
	// ("cv::cuda::HostMem::HostMem", vec![(pred!(mut, ["m"], ["const cv::cuda::HostMem*"]), _)]),
	void cv_cuda_HostMem_HostMem_const_HostMemR(const cv::cuda::HostMem* m, Result<cv::cuda::HostMem*>* ocvrs_return) {
		try {
			cv::cuda::HostMem* ret = new cv::cuda::HostMem(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// HostMem(int, int, int, AllocType)(Primitive, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:543
	// ("cv::cuda::HostMem::HostMem", vec![(pred!(mut, ["rows", "cols", "type", "alloc_type"], ["int", "int", "int", "cv::cuda::HostMem::AllocType"]), _)]),
	void cv_cuda_HostMem_HostMem_int_int_int_AllocType(int rows, int cols, int type, cv::cuda::HostMem::AllocType alloc_type, Result<cv::cuda::HostMem*>* ocvrs_return) {
		try {
			cv::cuda::HostMem* ret = new cv::cuda::HostMem(rows, cols, type, alloc_type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::HostMem::HostMem(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:543
	// ("cv::cuda::HostMem::HostMem", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
	void cv_cuda_HostMem_HostMem_int_int_int(int rows, int cols, int type, Result<cv::cuda::HostMem*>* ocvrs_return) {
		try {
			cv::cuda::HostMem* ret = new cv::cuda::HostMem(rows, cols, type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// HostMem(Size, int, AllocType)(SimpleClass, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:544
	// ("cv::cuda::HostMem::HostMem", vec![(pred!(mut, ["size", "type", "alloc_type"], ["cv::Size", "int", "cv::cuda::HostMem::AllocType"]), _)]),
	void cv_cuda_HostMem_HostMem_Size_int_AllocType(cv::Size* size, int type, cv::cuda::HostMem::AllocType alloc_type, Result<cv::cuda::HostMem*>* ocvrs_return) {
		try {
			cv::cuda::HostMem* ret = new cv::cuda::HostMem(*size, type, alloc_type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::HostMem::HostMem(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:544
	// ("cv::cuda::HostMem::HostMem", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
	void cv_cuda_HostMem_HostMem_Size_int(cv::Size* size, int type, Result<cv::cuda::HostMem*>* ocvrs_return) {
		try {
			cv::cuda::HostMem* ret = new cv::cuda::HostMem(*size, type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// HostMem(InputArray, AllocType)(InputArray, Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:547
	// ("cv::cuda::HostMem::HostMem", vec![(pred!(mut, ["arr", "alloc_type"], ["const cv::_InputArray*", "cv::cuda::HostMem::AllocType"]), _)]),
	void cv_cuda_HostMem_HostMem_const__InputArrayR_AllocType(const cv::_InputArray* arr, cv::cuda::HostMem::AllocType alloc_type, Result<cv::cuda::HostMem*>* ocvrs_return) {
		try {
			cv::cuda::HostMem* ret = new cv::cuda::HostMem(*arr, alloc_type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::HostMem::HostMem(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:547
	// ("cv::cuda::HostMem::HostMem", vec![(pred!(mut, ["arr"], ["const cv::_InputArray*"]), _)]),
	void cv_cuda_HostMem_HostMem_const__InputArrayR(const cv::_InputArray* arr, Result<cv::cuda::HostMem*>* ocvrs_return) {
		try {
			cv::cuda::HostMem* ret = new cv::cuda::HostMem(*arr);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const HostMem &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:551
	// ("cv::cuda::HostMem::operator=", vec![(pred!(mut, ["m"], ["const cv::cuda::HostMem*"]), _)]),
	void cv_cuda_HostMem_operatorST_const_HostMemR(cv::cuda::HostMem* instance, const cv::cuda::HostMem* m, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// swap(HostMem &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:554
	// ("cv::cuda::HostMem::swap", vec![(pred!(mut, ["b"], ["cv::cuda::HostMem*"]), _)]),
	void cv_cuda_HostMem_swap_HostMemR(cv::cuda::HostMem* instance, cv::cuda::HostMem* b, ResultVoid* ocvrs_return) {
		try {
			instance->swap(*b);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clone()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:557
	// ("cv::cuda::HostMem::clone", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HostMem_clone_const(const cv::cuda::HostMem* instance, Result<cv::cuda::HostMem*>* ocvrs_return) {
		try {
			cv::cuda::HostMem ret = instance->clone();
			Ok(new cv::cuda::HostMem(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:560
	// ("cv::cuda::HostMem::create", vec![(pred!(mut, ["rows", "cols", "type"], ["int", "int", "int"]), _)]),
	void cv_cuda_HostMem_create_int_int_int(cv::cuda::HostMem* instance, int rows, int cols, int type, ResultVoid* ocvrs_return) {
		try {
			instance->create(rows, cols, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(Size, int)(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:561
	// ("cv::cuda::HostMem::create", vec![(pred!(mut, ["size", "type"], ["cv::Size", "int"]), _)]),
	void cv_cuda_HostMem_create_Size_int(cv::cuda::HostMem* instance, cv::Size* size, int type, ResultVoid* ocvrs_return) {
		try {
			instance->create(*size, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reshape(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:565
	// ("cv::cuda::HostMem::reshape", vec![(pred!(const, ["cn", "rows"], ["int", "int"]), _)]),
	void cv_cuda_HostMem_reshape_const_int_int(const cv::cuda::HostMem* instance, int cn, int rows, Result<cv::cuda::HostMem*>* ocvrs_return) {
		try {
			cv::cuda::HostMem ret = instance->reshape(cn, rows);
			Ok(new cv::cuda::HostMem(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::HostMem::reshape(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:565
	// ("cv::cuda::HostMem::reshape", vec![(pred!(const, ["cn"], ["int"]), _)]),
	void cv_cuda_HostMem_reshape_const_int(const cv::cuda::HostMem* instance, int cn, Result<cv::cuda::HostMem*>* ocvrs_return) {
		try {
			cv::cuda::HostMem ret = instance->reshape(cn);
			Ok(new cv::cuda::HostMem(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// release()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:568
	// ("cv::cuda::HostMem::release", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_HostMem_release(cv::cuda::HostMem* instance, ResultVoid* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createMatHeader()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:571
	// ("cv::cuda::HostMem::createMatHeader", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HostMem_createMatHeader_const(const cv::cuda::HostMem* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->createMatHeader();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createGpuMatHeader()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:580
	// ("cv::cuda::HostMem::createGpuMatHeader", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HostMem_createGpuMatHeader_const(const cv::cuda::HostMem* instance, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->createGpuMatHeader();
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isContinuous()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:583
	// ("cv::cuda::HostMem::isContinuous", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HostMem_isContinuous_const(const cv::cuda::HostMem* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isContinuous();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// elemSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:584
	// ("cv::cuda::HostMem::elemSize", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HostMem_elemSize_const(const cv::cuda::HostMem* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->elemSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// elemSize1()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:585
	// ("cv::cuda::HostMem::elemSize1", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HostMem_elemSize1_const(const cv::cuda::HostMem* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->elemSize1();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// type()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:586
	// ("cv::cuda::HostMem::type", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HostMem_type_const(const cv::cuda::HostMem* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->type();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// depth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:587
	// ("cv::cuda::HostMem::depth", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HostMem_depth_const(const cv::cuda::HostMem* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->depth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// channels()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:588
	// ("cv::cuda::HostMem::channels", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HostMem_channels_const(const cv::cuda::HostMem* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->channels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// step1()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:589
	// ("cv::cuda::HostMem::step1", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HostMem_step1_const(const cv::cuda::HostMem* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->step1();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// size()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:590
	// ("cv::cuda::HostMem::size", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HostMem_size_const(const cv::cuda::HostMem* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->size();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:591
	// ("cv::cuda::HostMem::empty", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HostMem_empty_const(const cv::cuda::HostMem* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::HostMem::flags() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:594
	// ("cv::cuda::HostMem::flags", vec![(pred!(const, [], []), _)]),
	int cv_cuda_HostMem_propFlags_const(const cv::cuda::HostMem* instance) {
			int ret = instance->flags;
			return ret;
	}

	// cv::cuda::HostMem::setFlags(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:594
	// ("cv::cuda::HostMem::setFlags", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_cuda_HostMem_propFlags_const_int(cv::cuda::HostMem* instance, const int val) {
			instance->flags = val;
	}

	// cv::cuda::HostMem::rows() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:595
	// ("cv::cuda::HostMem::rows", vec![(pred!(const, [], []), _)]),
	int cv_cuda_HostMem_propRows_const(const cv::cuda::HostMem* instance) {
			int ret = instance->rows;
			return ret;
	}

	// cv::cuda::HostMem::setRows(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:595
	// ("cv::cuda::HostMem::setRows", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_cuda_HostMem_propRows_const_int(cv::cuda::HostMem* instance, const int val) {
			instance->rows = val;
	}

	// cv::cuda::HostMem::cols() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:595
	// ("cv::cuda::HostMem::cols", vec![(pred!(const, [], []), _)]),
	int cv_cuda_HostMem_propCols_const(const cv::cuda::HostMem* instance) {
			int ret = instance->cols;
			return ret;
	}

	// cv::cuda::HostMem::setCols(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:595
	// ("cv::cuda::HostMem::setCols", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_cuda_HostMem_propCols_const_int(cv::cuda::HostMem* instance, const int val) {
			instance->cols = val;
	}

	// cv::cuda::HostMem::step() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:596
	// ("cv::cuda::HostMem::step", vec![(pred!(const, [], []), _)]),
	size_t cv_cuda_HostMem_propStep_const(const cv::cuda::HostMem* instance) {
			size_t ret = instance->step;
			return ret;
	}

	// cv::cuda::HostMem::setStep(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:596
	// ("cv::cuda::HostMem::setStep", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
	void cv_cuda_HostMem_propStep_const_size_t(cv::cuda::HostMem* instance, const size_t val) {
			instance->step = val;
	}

	// cv::cuda::HostMem::data() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:598
	// ("cv::cuda::HostMem::data", vec![(pred!(const, [], []), _)]),
	unsigned char* cv_cuda_HostMem_propData_const(const cv::cuda::HostMem* instance) {
			unsigned char* const ret = instance->data;
			return ret;
	}

	// cv::cuda::HostMem::dataMut() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:598
	// ("cv::cuda::HostMem::dataMut", vec![(pred!(mut, [], []), _)]),
	unsigned char* cv_cuda_HostMem_propData(cv::cuda::HostMem* instance) {
			unsigned char* ret = instance->data;
			return ret;
	}

	// cv::cuda::HostMem::setData(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:598
	// ("cv::cuda::HostMem::setData", vec![(pred!(mut, ["val"], ["unsigned char*"]), _)]),
	void cv_cuda_HostMem_propData_unsigned_charX(cv::cuda::HostMem* instance, unsigned char* const val) {
			instance->data = val;
	}

	// cv::cuda::HostMem::refcount() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:599
	// ("cv::cuda::HostMem::refcount", vec![(pred!(const, [], []), _)]),
	int* cv_cuda_HostMem_propRefcount_const(const cv::cuda::HostMem* instance) {
			int* const ret = instance->refcount;
			return ret;
	}

	// cv::cuda::HostMem::refcountMut() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:599
	// ("cv::cuda::HostMem::refcountMut", vec![(pred!(mut, [], []), _)]),
	int* cv_cuda_HostMem_propRefcount(cv::cuda::HostMem* instance) {
			int* ret = instance->refcount;
			return ret;
	}

	// cv::cuda::HostMem::setRefcount(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:599
	// ("cv::cuda::HostMem::setRefcount", vec![(pred!(mut, ["val"], ["int*"]), _)]),
	void cv_cuda_HostMem_propRefcount_intX(cv::cuda::HostMem* instance, int* const val) {
			instance->refcount = val;
	}

	// cv::cuda::HostMem::datastart() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:601
	// ("cv::cuda::HostMem::datastart", vec![(pred!(const, [], []), _)]),
	unsigned char* cv_cuda_HostMem_propDatastart_const(const cv::cuda::HostMem* instance) {
			unsigned char* const ret = instance->datastart;
			return ret;
	}

	// cv::cuda::HostMem::datastartMut() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:601
	// ("cv::cuda::HostMem::datastartMut", vec![(pred!(mut, [], []), _)]),
	unsigned char* cv_cuda_HostMem_propDatastart(cv::cuda::HostMem* instance) {
			unsigned char* ret = instance->datastart;
			return ret;
	}

	// cv::cuda::HostMem::setDatastart(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:601
	// ("cv::cuda::HostMem::setDatastart", vec![(pred!(mut, ["val"], ["unsigned char*"]), _)]),
	void cv_cuda_HostMem_propDatastart_unsigned_charX(cv::cuda::HostMem* instance, unsigned char* const val) {
			instance->datastart = val;
	}

	// cv::cuda::HostMem::dataend() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:602
	// ("cv::cuda::HostMem::dataend", vec![(pred!(const, [], []), _)]),
	const unsigned char* cv_cuda_HostMem_propDataend_const(const cv::cuda::HostMem* instance) {
			const unsigned char* ret = instance->dataend;
			return ret;
	}

	// cv::cuda::HostMem::alloc_type() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:604
	// ("cv::cuda::HostMem::alloc_type", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HostMem_propAlloc_type_const(const cv::cuda::HostMem* instance, cv::cuda::HostMem::AllocType* ocvrs_return) {
			cv::cuda::HostMem::AllocType ret = instance->alloc_type;
			*ocvrs_return = ret;
	}

	// cv::cuda::HostMem::setAlloc_type(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:604
	// ("cv::cuda::HostMem::setAlloc_type", vec![(pred!(mut, ["val"], ["const cv::cuda::HostMem::AllocType"]), _)]),
	void cv_cuda_HostMem_propAlloc_type_const_AllocType(cv::cuda::HostMem* instance, const cv::cuda::HostMem::AllocType val) {
			instance->alloc_type = val;
	}

	// cv::cuda::HostMem::delete() generated
	// ("cv::cuda::HostMem::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_HostMem_delete(cv::cuda::HostMem* instance) {
			delete instance;
	}

	// Stream()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:659
	// ("cv::cuda::Stream::Stream", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_Stream_Stream(Result<cv::cuda::Stream*>* ocvrs_return) {
		try {
			cv::cuda::Stream* ret = new cv::cuda::Stream();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Stream(const Ptr<GpuMat::Allocator> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:662
	// ("cv::cuda::Stream::Stream", vec![(pred!(mut, ["allocator"], ["const cv::Ptr<cv::cuda::GpuMat::Allocator>*"]), _)]),
	void cv_cuda_Stream_Stream_const_PtrLAllocatorGR(const cv::Ptr<cv::cuda::GpuMat::Allocator>* allocator, Result<cv::cuda::Stream*>* ocvrs_return) {
		try {
			cv::cuda::Stream* ret = new cv::cuda::Stream(*allocator);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// queryIfComplete()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:666
	// ("cv::cuda::Stream::queryIfComplete", vec![(pred!(const, [], []), _)]),
	void cv_cuda_Stream_queryIfComplete_const(const cv::cuda::Stream* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->queryIfComplete();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// waitForCompletion()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:670
	// ("cv::cuda::Stream::waitForCompletion", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_Stream_waitForCompletion(cv::cuda::Stream* instance, ResultVoid* ocvrs_return) {
		try {
			instance->waitForCompletion();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// waitEvent(const Event &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:674
	// ("cv::cuda::Stream::waitEvent", vec![(pred!(mut, ["event"], ["const cv::cuda::Event*"]), _)]),
	void cv_cuda_Stream_waitEvent_const_EventR(cv::cuda::Stream* instance, const cv::cuda::Event* event, ResultVoid* ocvrs_return) {
		try {
			instance->waitEvent(*event);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// enqueueHostCallback(StreamCallback, void *)(Function, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:684
	// ("cv::cuda::Stream::enqueueHostCallback", vec![(pred!(mut, ["callback", "userData"], ["cv::cuda::Stream::StreamCallback", "void*"]), _)]),
	void cv_cuda_Stream_enqueueHostCallback_StreamCallback_voidX(cv::cuda::Stream* instance, cv::cuda::Stream::StreamCallback callback, void* userData, ResultVoid* ocvrs_return) {
		try {
			instance->enqueueHostCallback(callback, userData);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Null()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:687
	// ("cv::cuda::Stream::Null", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_Stream_Null(Result<cv::cuda::Stream*>* ocvrs_return) {
		try {
			cv::cuda::Stream ret = cv::cuda::Stream::Null();
			Ok(new cv::cuda::Stream(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::Stream::delete() generated
	// ("cv::cuda::Stream::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_Stream_delete(cv::cuda::Stream* instance) {
			delete instance;
	}

	// builtWith(FeatureSet)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:811
	// ("cv::cuda::TargetArchs::builtWith", vec![(pred!(mut, ["feature_set"], ["cv::cuda::FeatureSet"]), _)]),
	void cv_cuda_TargetArchs_builtWith_FeatureSet(cv::cuda::FeatureSet feature_set, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::cuda::TargetArchs::builtWith(feature_set);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// has(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:819
	// ("cv::cuda::TargetArchs::has", vec![(pred!(mut, ["major", "minor"], ["int", "int"]), _)]),
	void cv_cuda_TargetArchs_has_int_int(int major, int minor, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::cuda::TargetArchs::has(major, minor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// hasPtx(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:820
	// ("cv::cuda::TargetArchs::hasPtx", vec![(pred!(mut, ["major", "minor"], ["int", "int"]), _)]),
	void cv_cuda_TargetArchs_hasPtx_int_int(int major, int minor, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::cuda::TargetArchs::hasPtx(major, minor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// hasBin(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:821
	// ("cv::cuda::TargetArchs::hasBin", vec![(pred!(mut, ["major", "minor"], ["int", "int"]), _)]),
	void cv_cuda_TargetArchs_hasBin_int_int(int major, int minor, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::cuda::TargetArchs::hasBin(major, minor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// hasEqualOrLessPtx(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:823
	// ("cv::cuda::TargetArchs::hasEqualOrLessPtx", vec![(pred!(mut, ["major", "minor"], ["int", "int"]), _)]),
	void cv_cuda_TargetArchs_hasEqualOrLessPtx_int_int(int major, int minor, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::cuda::TargetArchs::hasEqualOrLessPtx(major, minor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// hasEqualOrGreater(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:824
	// ("cv::cuda::TargetArchs::hasEqualOrGreater", vec![(pred!(mut, ["major", "minor"], ["int", "int"]), _)]),
	void cv_cuda_TargetArchs_hasEqualOrGreater_int_int(int major, int minor, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::cuda::TargetArchs::hasEqualOrGreater(major, minor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// hasEqualOrGreaterPtx(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:825
	// ("cv::cuda::TargetArchs::hasEqualOrGreaterPtx", vec![(pred!(mut, ["major", "minor"], ["int", "int"]), _)]),
	void cv_cuda_TargetArchs_hasEqualOrGreaterPtx_int_int(int major, int minor, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::cuda::TargetArchs::hasEqualOrGreaterPtx(major, minor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// hasEqualOrGreaterBin(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cuda.hpp:826
	// ("cv::cuda::TargetArchs::hasEqualOrGreaterBin", vec![(pred!(mut, ["major", "minor"], ["int", "int"]), _)]),
	void cv_cuda_TargetArchs_hasEqualOrGreaterBin_int_int(int major, int minor, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::cuda::TargetArchs::hasEqualOrGreaterBin(major, minor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::TargetArchs::defaultNew() generated
	// ("cv::cuda::TargetArchs::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::cuda::TargetArchs* cv_cuda_TargetArchs_defaultNew_const() {
			cv::cuda::TargetArchs* ret = new cv::cuda::TargetArchs();
			return ret;
	}

	// cv::cuda::TargetArchs::delete() generated
	// ("cv::cuda::TargetArchs::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_TargetArchs_delete(cv::cuda::TargetArchs* instance) {
			delete instance;
	}

	// cv::detail::CheckContext::implicitClone() generated
	// ("cv::detail::CheckContext::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::detail::CheckContext* cv_detail_CheckContext_implicitClone_const(const cv::detail::CheckContext* instance) {
			return new cv::detail::CheckContext(*instance);
	}

	// cv::detail::CheckContext::defaultNew() generated
	// ("cv::detail::CheckContext::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::detail::CheckContext* cv_detail_CheckContext_defaultNew_const() {
			cv::detail::CheckContext* ret = new cv::detail::CheckContext();
			return ret;
	}

	// cv::detail::CheckContext::func() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/check.hpp:40
	// ("cv::detail::CheckContext::func", vec![(pred!(const, [], []), _)]),
	void* cv_detail_CheckContext_propFunc_const(const cv::detail::CheckContext* instance) {
			const char* ret = instance->func;
			return ocvrs_create_string(ret);
	}

	// cv::detail::CheckContext::file() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/check.hpp:41
	// ("cv::detail::CheckContext::file", vec![(pred!(const, [], []), _)]),
	void* cv_detail_CheckContext_propFile_const(const cv::detail::CheckContext* instance) {
			const char* ret = instance->file;
			return ocvrs_create_string(ret);
	}

	// cv::detail::CheckContext::line() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/check.hpp:42
	// ("cv::detail::CheckContext::line", vec![(pred!(const, [], []), _)]),
	int cv_detail_CheckContext_propLine_const(const cv::detail::CheckContext* instance) {
			int ret = instance->line;
			return ret;
	}

	// cv::detail::CheckContext::setLine(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/check.hpp:42
	// ("cv::detail::CheckContext::setLine", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_detail_CheckContext_propLine_const_int(cv::detail::CheckContext* instance, const int val) {
			instance->line = val;
	}

	// cv::detail::CheckContext::testOp() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/check.hpp:43
	// ("cv::detail::CheckContext::testOp", vec![(pred!(const, [], []), _)]),
	void cv_detail_CheckContext_propTestOp_const(const cv::detail::CheckContext* instance, cv::detail::TestOp* ocvrs_return) {
			cv::detail::TestOp ret = instance->testOp;
			*ocvrs_return = ret;
	}

	// cv::detail::CheckContext::setTestOp(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/check.hpp:43
	// ("cv::detail::CheckContext::setTestOp", vec![(pred!(mut, ["val"], ["const cv::detail::TestOp"]), _)]),
	void cv_detail_CheckContext_propTestOp_const_TestOp(cv::detail::CheckContext* instance, const cv::detail::TestOp val) {
			instance->testOp = val;
	}

	// cv::detail::CheckContext::message() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/check.hpp:44
	// ("cv::detail::CheckContext::message", vec![(pred!(const, [], []), _)]),
	void* cv_detail_CheckContext_propMessage_const(const cv::detail::CheckContext* instance) {
			const char* ret = instance->message;
			return ocvrs_create_string(ret);
	}

	// cv::detail::CheckContext::p1_str() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/check.hpp:45
	// ("cv::detail::CheckContext::p1_str", vec![(pred!(const, [], []), _)]),
	void* cv_detail_CheckContext_propP1_str_const(const cv::detail::CheckContext* instance) {
			const char* ret = instance->p1_str;
			return ocvrs_create_string(ret);
	}

	// cv::detail::CheckContext::p2_str() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/check.hpp:46
	// ("cv::detail::CheckContext::p2_str", vec![(pred!(const, [], []), _)]),
	void* cv_detail_CheckContext_propP2_str_const(const cv::detail::CheckContext* instance) {
			const char* ret = instance->p2_str;
			return ocvrs_create_string(ret);
	}

	// cv::detail::CheckContext::delete() generated
	// ("cv::detail::CheckContext::delete", vec![(pred!(mut, [], []), _)]),
	void cv_detail_CheckContext_delete(cv::detail::CheckContext* instance) {
			delete instance;
	}

	// float16_t()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cvdef.h:779
	// ("cv::float16_t::float16_t", vec![(pred!(mut, [], []), _)]),
	void cv_float16_t_float16_t(Result<cv::float16_t>* ocvrs_return) {
		try {
			cv::float16_t ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// float16_t(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cvdef.h:780
	// ("cv::float16_t::float16_t", vec![(pred!(mut, ["x"], ["float"]), _)]),
	void cv_float16_t_float16_t_float(float x, Result<cv::float16_t>* ocvrs_return) {
		try {
			cv::float16_t ret(x);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator float()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cvdef.h:781
	// ("cv::float16_t::operator float", vec![(pred!(const, [], []), _)]),
	void cv_float16_t_operator_float_const(const cv::float16_t* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->operator float();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fromBits(ushort)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cvdef.h:782
	// ("cv::float16_t::fromBits", vec![(pred!(mut, ["w"], ["unsigned short"]), _)]),
	void cv_float16_t_fromBits_unsigned_short(unsigned short w, Result<cv::float16_t>* ocvrs_return) {
		try {
			cv::float16_t ret = cv::float16_t::fromBits(w);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// zero()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cvdef.h:790
	// ("cv::float16_t::zero", vec![(pred!(mut, [], []), _)]),
	void cv_float16_t_zero(Result<cv::float16_t>* ocvrs_return) {
		try {
			cv::float16_t ret = cv::float16_t::zero();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bits()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/cvdef.h:796
	// ("cv::float16_t::bits", vec![(pred!(const, [], []), _)]),
	void cv_float16_t_bits_const(const cv::float16_t* instance, Result<unsigned short>* ocvrs_return) {
		try {
			unsigned short ret = instance->bits();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// NodeData(const char *, const char *, int, void *, bool, cv::instr::TYPE, cv::instr::IMPL)(InString, InString, Primitive, Indirect, Primitive, Enum, Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:72
	// ("cv::instr::NodeData::NodeData", vec![(pred!(mut, ["funName", "fileName", "lineNum", "retAddress", "alwaysExpand", "instrType", "implType"], ["const char*", "const char*", "int", "void*", "bool", "cv::instr::TYPE", "cv::instr::IMPL"]), _)]),
	void cv_instr_NodeData_NodeData_const_charX_const_charX_int_voidX_bool_TYPE_IMPL(const char* funName, const char* fileName, int lineNum, void* retAddress, bool alwaysExpand, cv::instr::TYPE instrType, cv::instr::IMPL implType, Result<cv::instr::NodeData*>* ocvrs_return) {
		try {
			cv::instr::NodeData* ret = new cv::instr::NodeData(funName, fileName, lineNum, retAddress, alwaysExpand, instrType, implType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::instr::NodeData::NodeData() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:72
	// ("cv::instr::NodeData::NodeData", vec![(pred!(mut, [], []), _)]),
	void cv_instr_NodeData_NodeData(Result<cv::instr::NodeData*>* ocvrs_return) {
		try {
			cv::instr::NodeData* ret = new cv::instr::NodeData();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// NodeData(NodeData &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:73
	// ("cv::instr::NodeData::NodeData", vec![(pred!(mut, ["ref"], ["cv::instr::NodeData*"]), _)]),
	void cv_instr_NodeData_NodeData_NodeDataR(cv::instr::NodeData* ref, Result<cv::instr::NodeData*>* ocvrs_return) {
		try {
			cv::instr::NodeData* ret = new cv::instr::NodeData(*ref);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const NodeData &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:75
	// ("cv::instr::NodeData::operator=", vec![(pred!(mut, ["unnamed"], ["const cv::instr::NodeData*"]), _)]),
	void cv_instr_NodeData_operatorST_const_NodeDataR(cv::instr::NodeData* instance, const cv::instr::NodeData* unnamed, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTotalMs()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:92
	// ("cv::instr::NodeData::getTotalMs", vec![(pred!(const, [], []), _)]),
	void cv_instr_NodeData_getTotalMs_const(const cv::instr::NodeData* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getTotalMs();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMeanMs()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:93
	// ("cv::instr::NodeData::getMeanMs", vec![(pred!(const, [], []), _)]),
	void cv_instr_NodeData_getMeanMs_const(const cv::instr::NodeData* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMeanMs();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::instr::NodeData::m_funName() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:77
	// ("cv::instr::NodeData::m_funName", vec![(pred!(const, [], []), _)]),
	void* cv_instr_NodeData_propM_funName_const(const cv::instr::NodeData* instance) {
			cv::String ret = instance->m_funName;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::instr::NodeData::setM_funName(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:77
	// ("cv::instr::NodeData::setM_funName", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	void cv_instr_NodeData_propM_funName_const_String(cv::instr::NodeData* instance, const char* val) {
			instance->m_funName = cv::String(val);
	}

	// cv::instr::NodeData::m_instrType() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:78
	// ("cv::instr::NodeData::m_instrType", vec![(pred!(const, [], []), _)]),
	void cv_instr_NodeData_propM_instrType_const(const cv::instr::NodeData* instance, cv::instr::TYPE* ocvrs_return) {
			cv::instr::TYPE ret = instance->m_instrType;
			*ocvrs_return = ret;
	}

	// cv::instr::NodeData::setM_instrType(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:78
	// ("cv::instr::NodeData::setM_instrType", vec![(pred!(mut, ["val"], ["const cv::instr::TYPE"]), _)]),
	void cv_instr_NodeData_propM_instrType_const_TYPE(cv::instr::NodeData* instance, const cv::instr::TYPE val) {
			instance->m_instrType = val;
	}

	// cv::instr::NodeData::m_implType() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:79
	// ("cv::instr::NodeData::m_implType", vec![(pred!(const, [], []), _)]),
	void cv_instr_NodeData_propM_implType_const(const cv::instr::NodeData* instance, cv::instr::IMPL* ocvrs_return) {
			cv::instr::IMPL ret = instance->m_implType;
			*ocvrs_return = ret;
	}

	// cv::instr::NodeData::setM_implType(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:79
	// ("cv::instr::NodeData::setM_implType", vec![(pred!(mut, ["val"], ["const cv::instr::IMPL"]), _)]),
	void cv_instr_NodeData_propM_implType_const_IMPL(cv::instr::NodeData* instance, const cv::instr::IMPL val) {
			instance->m_implType = val;
	}

	// cv::instr::NodeData::m_fileName() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:80
	// ("cv::instr::NodeData::m_fileName", vec![(pred!(const, [], []), _)]),
	void* cv_instr_NodeData_propM_fileName_const(const cv::instr::NodeData* instance) {
			const char* ret = instance->m_fileName;
			return ocvrs_create_string(ret);
	}

	// cv::instr::NodeData::m_lineNum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:81
	// ("cv::instr::NodeData::m_lineNum", vec![(pred!(const, [], []), _)]),
	int cv_instr_NodeData_propM_lineNum_const(const cv::instr::NodeData* instance) {
			int ret = instance->m_lineNum;
			return ret;
	}

	// cv::instr::NodeData::setM_lineNum(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:81
	// ("cv::instr::NodeData::setM_lineNum", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_instr_NodeData_propM_lineNum_const_int(cv::instr::NodeData* instance, const int val) {
			instance->m_lineNum = val;
	}

	// cv::instr::NodeData::m_retAddress() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:82
	// ("cv::instr::NodeData::m_retAddress", vec![(pred!(mut, [], []), _)]),
	void* cv_instr_NodeData_propM_retAddress(cv::instr::NodeData* instance) {
			void* ret = instance->m_retAddress;
			return ret;
	}

	// cv::instr::NodeData::setM_retAddress(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:82
	// ("cv::instr::NodeData::setM_retAddress", vec![(pred!(mut, ["val"], ["void*"]), _)]),
	void cv_instr_NodeData_propM_retAddress_voidX(cv::instr::NodeData* instance, void* const val) {
			instance->m_retAddress = val;
	}

	// cv::instr::NodeData::m_alwaysExpand() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:83
	// ("cv::instr::NodeData::m_alwaysExpand", vec![(pred!(const, [], []), _)]),
	bool cv_instr_NodeData_propM_alwaysExpand_const(const cv::instr::NodeData* instance) {
			bool ret = instance->m_alwaysExpand;
			return ret;
	}

	// cv::instr::NodeData::setM_alwaysExpand(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:83
	// ("cv::instr::NodeData::setM_alwaysExpand", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_instr_NodeData_propM_alwaysExpand_const_bool(cv::instr::NodeData* instance, const bool val) {
			instance->m_alwaysExpand = val;
	}

	// cv::instr::NodeData::m_funError() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:84
	// ("cv::instr::NodeData::m_funError", vec![(pred!(const, [], []), _)]),
	bool cv_instr_NodeData_propM_funError_const(const cv::instr::NodeData* instance) {
			bool ret = instance->m_funError;
			return ret;
	}

	// cv::instr::NodeData::setM_funError(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:84
	// ("cv::instr::NodeData::setM_funError", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_instr_NodeData_propM_funError_const_bool(cv::instr::NodeData* instance, const bool val) {
			instance->m_funError = val;
	}

	// cv::instr::NodeData::m_counter() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:86
	// ("cv::instr::NodeData::m_counter", vec![(pred!(const, [], []), _)]),
	int cv_instr_NodeData_propM_counter_const(const cv::instr::NodeData* instance) {
			int ret = instance->m_counter;
			return ret;
	}

	// cv::instr::NodeData::setM_counter(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:86
	// ("cv::instr::NodeData::setM_counter", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_instr_NodeData_propM_counter_const_int(cv::instr::NodeData* instance, const int val) {
			instance->m_counter = val;
	}

	// cv::instr::NodeData::m_ticksTotal() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:87
	// ("cv::instr::NodeData::m_ticksTotal", vec![(pred!(const, [], []), _)]),
	uint64_t cv_instr_NodeData_propM_ticksTotal_const(const cv::instr::NodeData* instance) {
			uint64_t ret = instance->m_ticksTotal;
			return ret;
	}

	// cv::instr::NodeData::setM_ticksTotal(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:87
	// ("cv::instr::NodeData::setM_ticksTotal", vec![(pred!(mut, ["val"], ["const uint64_t"]), _)]),
	void cv_instr_NodeData_propM_ticksTotal_const_uint64_t(cv::instr::NodeData* instance, const uint64_t val) {
			instance->m_ticksTotal = val;
	}

	// cv::instr::NodeData::m_threads() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:89
	// ("cv::instr::NodeData::m_threads", vec![(pred!(const, [], []), _)]),
	int cv_instr_NodeData_propM_threads_const(const cv::instr::NodeData* instance) {
			int ret = instance->m_threads;
			return ret;
	}

	// cv::instr::NodeData::setM_threads(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/utils/instrumentation.hpp:89
	// ("cv::instr::NodeData::setM_threads", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_instr_NodeData_propM_threads_const_int(cv::instr::NodeData* instance, const int val) {
			instance->m_threads = val;
	}

	// cv::instr::NodeData::delete() generated
	// ("cv::instr::NodeData::delete", vec![(pred!(mut, [], []), _)]),
	void cv_instr_NodeData_delete(cv::instr::NodeData* instance) {
			delete instance;
	}

	// WriteStructContext(FileStorage &, const String &, int, const String &)(TraitClass, InString, Primitive, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:819
	// ("cv::internal::WriteStructContext::WriteStructContext", vec![(pred!(mut, ["_fs", "name", "flags", "typeName"], ["cv::FileStorage*", "const cv::String*", "int", "const cv::String*"]), _)]),
	void cv_internal_WriteStructContext_WriteStructContext_FileStorageR_const_StringR_int_const_StringR(cv::FileStorage* _fs, const char* name, int flags, const char* typeName, Result<cv::internal::WriteStructContext*>* ocvrs_return) {
		try {
			cv::internal::WriteStructContext* ret = new cv::internal::WriteStructContext(*_fs, cv::String(name), flags, cv::String(typeName));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::internal::WriteStructContext::WriteStructContext(TraitClass, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/persistence.hpp:819
	// ("cv::internal::WriteStructContext::WriteStructContext", vec![(pred!(mut, ["_fs", "name", "flags"], ["cv::FileStorage*", "const cv::String*", "int"]), _)]),
	void cv_internal_WriteStructContext_WriteStructContext_FileStorageR_const_StringR_int(cv::FileStorage* _fs, const char* name, int flags, Result<cv::internal::WriteStructContext*>* ocvrs_return) {
		try {
			cv::internal::WriteStructContext* ret = new cv::internal::WriteStructContext(*_fs, cv::String(name), flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::internal::WriteStructContext::delete() generated
	// ("cv::internal::WriteStructContext::delete", vec![(pred!(mut, [], []), _)]),
	void cv_internal_WriteStructContext_delete(cv::internal::WriteStructContext* instance) {
			delete instance;
	}

	// Context()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:241
	// ("cv::ocl::Context::Context", vec![(pred!(mut, [], []), _)]),
	cv::ocl::Context* cv_ocl_Context_Context() {
			cv::ocl::Context* ret = new cv::ocl::Context();
			return ret;
	}

	// Context(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:242
	// ("cv::ocl::Context::Context", vec![(pred!(mut, ["dtype"], ["int"]), _)]),
	void cv_ocl_Context_Context_int(int dtype, Result<cv::ocl::Context*>* ocvrs_return) {
		try {
			cv::ocl::Context* ret = new cv::ocl::Context(dtype);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Context(const Context &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:244
	// ("cv::ocl::Context::Context", vec![(pred!(mut, ["c"], ["const cv::ocl::Context*"]), _)]),
	void cv_ocl_Context_Context_const_ContextR(const cv::ocl::Context* c, Result<cv::ocl::Context*>* ocvrs_return) {
		try {
			cv::ocl::Context* ret = new cv::ocl::Context(*c);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const Context &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:245
	// ("cv::ocl::Context::operator=", vec![(pred!(mut, ["c"], ["const cv::ocl::Context*"]), _)]),
	void cv_ocl_Context_operatorST_const_ContextR(cv::ocl::Context* instance, const cv::ocl::Context* c, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*c);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:247
	// ("cv::ocl::Context::create", vec![(pred!(mut, [], []), _)]),
	void cv_ocl_Context_create(cv::ocl::Context* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->create();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:248
	// ("cv::ocl::Context::create", vec![(pred!(mut, ["dtype"], ["int"]), _)]),
	void cv_ocl_Context_create_int(cv::ocl::Context* instance, int dtype, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->create(dtype);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ndevices()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:249
	// ("cv::ocl::Context::ndevices", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Context_ndevices_const(const cv::ocl::Context* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->ndevices();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// device(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:250
	// ("cv::ocl::Context::device", vec![(pred!(const, ["idx"], ["size_t"]), _)]),
	void cv_ocl_Context_device_const_size_t(const cv::ocl::Context* instance, size_t idx, Result<cv::ocl::Device*>* ocvrs_return) {
		try {
			const cv::ocl::Device ret = instance->device(idx);
			Ok(new const cv::ocl::Device(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getProg(const ProgramSource &, const String &, String &)(TraitClass, InString, OutString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:251
	// ("cv::ocl::Context::getProg", vec![(pred!(mut, ["prog", "buildopt", "errmsg"], ["const cv::ocl::ProgramSource*", "const cv::String*", "cv::String*"]), _)]),
	void cv_ocl_Context_getProg_const_ProgramSourceR_const_StringR_StringR(cv::ocl::Context* instance, const cv::ocl::ProgramSource* prog, const char* buildopt, void** errmsg, Result<cv::ocl::Program*>* ocvrs_return) {
		try {
			cv::String errmsg_out;
			cv::ocl::Program ret = instance->getProg(*prog, cv::String(buildopt), errmsg_out);
			*errmsg = ocvrs_create_string(errmsg_out.c_str());
			Ok(new cv::ocl::Program(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// unloadProg(Program &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:253
	// ("cv::ocl::Context::unloadProg", vec![(pred!(mut, ["prog"], ["cv::ocl::Program*"]), _)]),
	void cv_ocl_Context_unloadProg_ProgramR(cv::ocl::Context* instance, cv::ocl::Program* prog, ResultVoid* ocvrs_return) {
		try {
			instance->unloadProg(*prog);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefault(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:255
	// ("cv::ocl::Context::getDefault", vec![(pred!(mut, ["initialize"], ["bool"]), _)]),
	void cv_ocl_Context_getDefault_bool(bool initialize, Result<cv::ocl::Context*>* ocvrs_return) {
		try {
			cv::ocl::Context ret = cv::ocl::Context::getDefault(initialize);
			Ok(new cv::ocl::Context(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::Context::getDefault() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:255
	// ("cv::ocl::Context::getDefault", vec![(pred!(mut, [], []), _)]),
	void cv_ocl_Context_getDefault(Result<cv::ocl::Context*>* ocvrs_return) {
		try {
			cv::ocl::Context ret = cv::ocl::Context::getDefault();
			Ok(new cv::ocl::Context(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ptr()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:256
	// ("cv::ocl::Context::ptr", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Context_ptr_const(const cv::ocl::Context* instance, Result<void*>* ocvrs_return) {
		try {
			void* ret = instance->ptr();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// useSVM()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:260
	// ("cv::ocl::Context::useSVM", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Context_useSVM_const(const cv::ocl::Context* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->useSVM();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseSVM(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:261
	// ("cv::ocl::Context::setUseSVM", vec![(pred!(mut, ["enabled"], ["bool"]), _)]),
	void cv_ocl_Context_setUseSVM_bool(cv::ocl::Context* instance, bool enabled, ResultVoid* ocvrs_return) {
		try {
			instance->setUseSVM(enabled);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::Context::delete() generated
	// ("cv::ocl::Context::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ocl_Context_delete(cv::ocl::Context* instance) {
			delete instance;
	}

	// Device()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:73
	// ("cv::ocl::Device::Device", vec![(pred!(mut, [], []), _)]),
	cv::ocl::Device* cv_ocl_Device_Device() {
			cv::ocl::Device* ret = new cv::ocl::Device();
			return ret;
	}

	// Device(void *)(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:74
	// ("cv::ocl::Device::Device", vec![(pred!(mut, ["d"], ["void*"]), _)]),
	void cv_ocl_Device_Device_voidX(void* d, Result<cv::ocl::Device*>* ocvrs_return) {
		try {
			cv::ocl::Device* ret = new cv::ocl::Device(d);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Device(const Device &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:75
	// ("cv::ocl::Device::Device", vec![(pred!(mut, ["d"], ["const cv::ocl::Device*"]), _)]),
	void cv_ocl_Device_Device_const_DeviceR(const cv::ocl::Device* d, Result<cv::ocl::Device*>* ocvrs_return) {
		try {
			cv::ocl::Device* ret = new cv::ocl::Device(*d);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const Device &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:76
	// ("cv::ocl::Device::operator=", vec![(pred!(mut, ["d"], ["const cv::ocl::Device*"]), _)]),
	void cv_ocl_Device_operatorST_const_DeviceR(cv::ocl::Device* instance, const cv::ocl::Device* d, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// set(void *)(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:79
	// ("cv::ocl::Device::set", vec![(pred!(mut, ["d"], ["void*"]), _)]),
	void cv_ocl_Device_set_voidX(cv::ocl::Device* instance, void* d, ResultVoid* ocvrs_return) {
		try {
			instance->set(d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// name()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:92
	// ("cv::ocl::Device::name", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_name_const(const cv::ocl::Device* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->name();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// extensions()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:93
	// ("cv::ocl::Device::extensions", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_extensions_const(const cv::ocl::Device* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->extensions();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isExtensionSupported(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:94
	// ("cv::ocl::Device::isExtensionSupported", vec![(pred!(const, ["extensionName"], ["const cv::String*"]), _)]),
	void cv_ocl_Device_isExtensionSupported_const_const_StringR(const cv::ocl::Device* instance, const char* extensionName, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isExtensionSupported(cv::String(extensionName));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// version()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:95
	// ("cv::ocl::Device::version", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_version_const(const cv::ocl::Device* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->version();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// vendorName()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:96
	// ("cv::ocl::Device::vendorName", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_vendorName_const(const cv::ocl::Device* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->vendorName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// OpenCL_C_Version()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:97
	// ("cv::ocl::Device::OpenCL_C_Version", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_OpenCL_C_Version_const(const cv::ocl::Device* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->OpenCL_C_Version();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// OpenCLVersion()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:98
	// ("cv::ocl::Device::OpenCLVersion", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_OpenCLVersion_const(const cv::ocl::Device* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->OpenCLVersion();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// deviceVersionMajor()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:99
	// ("cv::ocl::Device::deviceVersionMajor", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_deviceVersionMajor_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->deviceVersionMajor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// deviceVersionMinor()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:100
	// ("cv::ocl::Device::deviceVersionMinor", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_deviceVersionMinor_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->deviceVersionMinor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// driverVersion()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:101
	// ("cv::ocl::Device::driverVersion", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_driverVersion_const(const cv::ocl::Device* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->driverVersion();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ptr()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:102
	// ("cv::ocl::Device::ptr", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_ptr_const(const cv::ocl::Device* instance, Result<void*>* ocvrs_return) {
		try {
			void* ret = instance->ptr();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// type()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:104
	// ("cv::ocl::Device::type", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_type_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->type();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addressBits()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:106
	// ("cv::ocl::Device::addressBits", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_addressBits_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->addressBits();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// available()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:107
	// ("cv::ocl::Device::available", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_available_const(const cv::ocl::Device* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->available();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compilerAvailable()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:108
	// ("cv::ocl::Device::compilerAvailable", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_compilerAvailable_const(const cv::ocl::Device* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->compilerAvailable();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// linkerAvailable()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:109
	// ("cv::ocl::Device::linkerAvailable", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_linkerAvailable_const(const cv::ocl::Device* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->linkerAvailable();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// doubleFPConfig()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:122
	// ("cv::ocl::Device::doubleFPConfig", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_doubleFPConfig_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->doubleFPConfig();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// singleFPConfig()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:123
	// ("cv::ocl::Device::singleFPConfig", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_singleFPConfig_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->singleFPConfig();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// halfFPConfig()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:124
	// ("cv::ocl::Device::halfFPConfig", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_halfFPConfig_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->halfFPConfig();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// endianLittle()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:126
	// ("cv::ocl::Device::endianLittle", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_endianLittle_const(const cv::ocl::Device* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->endianLittle();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// errorCorrectionSupport()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:127
	// ("cv::ocl::Device::errorCorrectionSupport", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_errorCorrectionSupport_const(const cv::ocl::Device* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->errorCorrectionSupport();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// executionCapabilities()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:134
	// ("cv::ocl::Device::executionCapabilities", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_executionCapabilities_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->executionCapabilities();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// globalMemCacheSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:136
	// ("cv::ocl::Device::globalMemCacheSize", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_globalMemCacheSize_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->globalMemCacheSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// globalMemCacheType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:144
	// ("cv::ocl::Device::globalMemCacheType", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_globalMemCacheType_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->globalMemCacheType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// globalMemCacheLineSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:145
	// ("cv::ocl::Device::globalMemCacheLineSize", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_globalMemCacheLineSize_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->globalMemCacheLineSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// globalMemSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:146
	// ("cv::ocl::Device::globalMemSize", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_globalMemSize_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->globalMemSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// localMemSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:148
	// ("cv::ocl::Device::localMemSize", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_localMemSize_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->localMemSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// localMemType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:155
	// ("cv::ocl::Device::localMemType", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_localMemType_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->localMemType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// hostUnifiedMemory()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:156
	// ("cv::ocl::Device::hostUnifiedMemory", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_hostUnifiedMemory_const(const cv::ocl::Device* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->hostUnifiedMemory();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imageSupport()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:158
	// ("cv::ocl::Device::imageSupport", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_imageSupport_const(const cv::ocl::Device* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->imageSupport();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imageFromBufferSupport()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:160
	// ("cv::ocl::Device::imageFromBufferSupport", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_imageFromBufferSupport_const(const cv::ocl::Device* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->imageFromBufferSupport();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imagePitchAlignment()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:161
	// ("cv::ocl::Device::imagePitchAlignment", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_imagePitchAlignment_const(const cv::ocl::Device* instance, Result<unsigned int>* ocvrs_return) {
		try {
			unsigned int ret = instance->imagePitchAlignment();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imageBaseAddressAlignment()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:162
	// ("cv::ocl::Device::imageBaseAddressAlignment", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_imageBaseAddressAlignment_const(const cv::ocl::Device* instance, Result<unsigned int>* ocvrs_return) {
		try {
			unsigned int ret = instance->imageBaseAddressAlignment();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// intelSubgroupsSupport()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:165
	// ("cv::ocl::Device::intelSubgroupsSupport", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_intelSubgroupsSupport_const(const cv::ocl::Device* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->intelSubgroupsSupport();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// image2DMaxWidth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:167
	// ("cv::ocl::Device::image2DMaxWidth", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_image2DMaxWidth_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->image2DMaxWidth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// image2DMaxHeight()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:168
	// ("cv::ocl::Device::image2DMaxHeight", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_image2DMaxHeight_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->image2DMaxHeight();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// image3DMaxWidth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:170
	// ("cv::ocl::Device::image3DMaxWidth", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_image3DMaxWidth_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->image3DMaxWidth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// image3DMaxHeight()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:171
	// ("cv::ocl::Device::image3DMaxHeight", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_image3DMaxHeight_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->image3DMaxHeight();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// image3DMaxDepth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:172
	// ("cv::ocl::Device::image3DMaxDepth", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_image3DMaxDepth_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->image3DMaxDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imageMaxBufferSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:174
	// ("cv::ocl::Device::imageMaxBufferSize", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_imageMaxBufferSize_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->imageMaxBufferSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imageMaxArraySize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:175
	// ("cv::ocl::Device::imageMaxArraySize", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_imageMaxArraySize_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->imageMaxArraySize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// vendorID()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:184
	// ("cv::ocl::Device::vendorID", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_vendorID_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->vendorID();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isAMD()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:189
	// ("cv::ocl::Device::isAMD", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_isAMD_const(const cv::ocl::Device* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isAMD();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isIntel()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:190
	// ("cv::ocl::Device::isIntel", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_isIntel_const(const cv::ocl::Device* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isIntel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isNVidia()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:191
	// ("cv::ocl::Device::isNVidia", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_isNVidia_const(const cv::ocl::Device* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isNVidia();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxClockFrequency()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:193
	// ("cv::ocl::Device::maxClockFrequency", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_maxClockFrequency_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxClockFrequency();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxComputeUnits()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:194
	// ("cv::ocl::Device::maxComputeUnits", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_maxComputeUnits_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxComputeUnits();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxConstantArgs()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:195
	// ("cv::ocl::Device::maxConstantArgs", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_maxConstantArgs_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxConstantArgs();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxConstantBufferSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:196
	// ("cv::ocl::Device::maxConstantBufferSize", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_maxConstantBufferSize_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->maxConstantBufferSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxMemAllocSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:198
	// ("cv::ocl::Device::maxMemAllocSize", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_maxMemAllocSize_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->maxMemAllocSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxParameterSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:199
	// ("cv::ocl::Device::maxParameterSize", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_maxParameterSize_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->maxParameterSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxReadImageArgs()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:201
	// ("cv::ocl::Device::maxReadImageArgs", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_maxReadImageArgs_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxReadImageArgs();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxWriteImageArgs()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:202
	// ("cv::ocl::Device::maxWriteImageArgs", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_maxWriteImageArgs_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxWriteImageArgs();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxSamplers()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:203
	// ("cv::ocl::Device::maxSamplers", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_maxSamplers_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxSamplers();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxWorkGroupSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:205
	// ("cv::ocl::Device::maxWorkGroupSize", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_maxWorkGroupSize_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->maxWorkGroupSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxWorkItemDims()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:206
	// ("cv::ocl::Device::maxWorkItemDims", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_maxWorkItemDims_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxWorkItemDims();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxWorkItemSizes(size_t *)(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:207
	// ("cv::ocl::Device::maxWorkItemSizes", vec![(pred!(const, ["unnamed"], ["size_t*"]), _)]),
	void cv_ocl_Device_maxWorkItemSizes_const_size_tX(const cv::ocl::Device* instance, size_t* unnamed, ResultVoid* ocvrs_return) {
		try {
			instance->maxWorkItemSizes(unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// memBaseAddrAlign()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:209
	// ("cv::ocl::Device::memBaseAddrAlign", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_memBaseAddrAlign_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->memBaseAddrAlign();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// nativeVectorWidthChar()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:211
	// ("cv::ocl::Device::nativeVectorWidthChar", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_nativeVectorWidthChar_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->nativeVectorWidthChar();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// nativeVectorWidthShort()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:212
	// ("cv::ocl::Device::nativeVectorWidthShort", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_nativeVectorWidthShort_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->nativeVectorWidthShort();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// nativeVectorWidthInt()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:213
	// ("cv::ocl::Device::nativeVectorWidthInt", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_nativeVectorWidthInt_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->nativeVectorWidthInt();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// nativeVectorWidthLong()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:214
	// ("cv::ocl::Device::nativeVectorWidthLong", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_nativeVectorWidthLong_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->nativeVectorWidthLong();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// nativeVectorWidthFloat()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:215
	// ("cv::ocl::Device::nativeVectorWidthFloat", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_nativeVectorWidthFloat_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->nativeVectorWidthFloat();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// nativeVectorWidthDouble()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:216
	// ("cv::ocl::Device::nativeVectorWidthDouble", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_nativeVectorWidthDouble_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->nativeVectorWidthDouble();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// nativeVectorWidthHalf()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:217
	// ("cv::ocl::Device::nativeVectorWidthHalf", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_nativeVectorWidthHalf_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->nativeVectorWidthHalf();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// preferredVectorWidthChar()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:219
	// ("cv::ocl::Device::preferredVectorWidthChar", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_preferredVectorWidthChar_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->preferredVectorWidthChar();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// preferredVectorWidthShort()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:220
	// ("cv::ocl::Device::preferredVectorWidthShort", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_preferredVectorWidthShort_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->preferredVectorWidthShort();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// preferredVectorWidthInt()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:221
	// ("cv::ocl::Device::preferredVectorWidthInt", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_preferredVectorWidthInt_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->preferredVectorWidthInt();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// preferredVectorWidthLong()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:222
	// ("cv::ocl::Device::preferredVectorWidthLong", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_preferredVectorWidthLong_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->preferredVectorWidthLong();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// preferredVectorWidthFloat()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:223
	// ("cv::ocl::Device::preferredVectorWidthFloat", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_preferredVectorWidthFloat_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->preferredVectorWidthFloat();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// preferredVectorWidthDouble()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:224
	// ("cv::ocl::Device::preferredVectorWidthDouble", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_preferredVectorWidthDouble_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->preferredVectorWidthDouble();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// preferredVectorWidthHalf()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:225
	// ("cv::ocl::Device::preferredVectorWidthHalf", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_preferredVectorWidthHalf_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->preferredVectorWidthHalf();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// printfBufferSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:227
	// ("cv::ocl::Device::printfBufferSize", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_printfBufferSize_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->printfBufferSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// profilingTimerResolution()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:228
	// ("cv::ocl::Device::profilingTimerResolution", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Device_profilingTimerResolution_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->profilingTimerResolution();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefault()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:230
	// ("cv::ocl::Device::getDefault", vec![(pred!(mut, [], []), _)]),
	void cv_ocl_Device_getDefault(Result<cv::ocl::Device*>* ocvrs_return) {
		try {
			const cv::ocl::Device ret = cv::ocl::Device::getDefault();
			Ok(new const cv::ocl::Device(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::Device::implicitClone() generated
	// ("cv::ocl::Device::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::ocl::Device* cv_ocl_Device_implicitClone_const(const cv::ocl::Device* instance) {
			return new cv::ocl::Device(*instance);
	}

	// cv::ocl::Device::delete() generated
	// ("cv::ocl::Device::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ocl_Device_delete(cv::ocl::Device* instance) {
			delete instance;
	}

	// Image2D()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:795
	// ("cv::ocl::Image2D::Image2D", vec![(pred!(mut, [], []), _)]),
	cv::ocl::Image2D* cv_ocl_Image2D_Image2D() {
			cv::ocl::Image2D* ret = new cv::ocl::Image2D();
			return ret;
	}

	// Image2D(const UMat &, bool, bool)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:803
	// ("cv::ocl::Image2D::Image2D", vec![(pred!(mut, ["src", "norm", "alias"], ["const cv::UMat*", "bool", "bool"]), _)]),
	void cv_ocl_Image2D_Image2D_const_UMatR_bool_bool(const cv::UMat* src, bool norm, bool alias, Result<cv::ocl::Image2D*>* ocvrs_return) {
		try {
			cv::ocl::Image2D* ret = new cv::ocl::Image2D(*src, norm, alias);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::Image2D::Image2D(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:803
	// ("cv::ocl::Image2D::Image2D", vec![(pred!(mut, ["src"], ["const cv::UMat*"]), _)]),
	void cv_ocl_Image2D_Image2D_const_UMatR(const cv::UMat* src, Result<cv::ocl::Image2D*>* ocvrs_return) {
		try {
			cv::ocl::Image2D* ret = new cv::ocl::Image2D(*src);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Image2D(const Image2D &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:804
	// ("cv::ocl::Image2D::Image2D", vec![(pred!(mut, ["i"], ["const cv::ocl::Image2D*"]), _)]),
	void cv_ocl_Image2D_Image2D_const_Image2DR(const cv::ocl::Image2D* i, Result<cv::ocl::Image2D*>* ocvrs_return) {
		try {
			cv::ocl::Image2D* ret = new cv::ocl::Image2D(*i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const Image2D &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:807
	// ("cv::ocl::Image2D::operator=", vec![(pred!(mut, ["i"], ["const cv::ocl::Image2D*"]), _)]),
	void cv_ocl_Image2D_operatorST_const_Image2DR(cv::ocl::Image2D* instance, const cv::ocl::Image2D* i, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*i);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// canCreateAlias(const UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:812
	// ("cv::ocl::Image2D::canCreateAlias", vec![(pred!(mut, ["u"], ["const cv::UMat*"]), _)]),
	void cv_ocl_Image2D_canCreateAlias_const_UMatR(const cv::UMat* u, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::ocl::Image2D::canCreateAlias(*u);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isFormatSupported(int, int, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:816
	// ("cv::ocl::Image2D::isFormatSupported", vec![(pred!(mut, ["depth", "cn", "norm"], ["int", "int", "bool"]), _)]),
	void cv_ocl_Image2D_isFormatSupported_int_int_bool(int depth, int cn, bool norm, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::ocl::Image2D::isFormatSupported(depth, cn, norm);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ptr()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:818
	// ("cv::ocl::Image2D::ptr", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Image2D_ptr_const(const cv::ocl::Image2D* instance, Result<void*>* ocvrs_return) {
		try {
			void* ret = instance->ptr();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::Image2D::delete() generated
	// ("cv::ocl::Image2D::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ocl_Image2D_delete(cv::ocl::Image2D* instance) {
			delete instance;
	}

	// Kernel()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:390
	// ("cv::ocl::Kernel::Kernel", vec![(pred!(mut, [], []), _)]),
	cv::ocl::Kernel* cv_ocl_Kernel_Kernel() {
			cv::ocl::Kernel* ret = new cv::ocl::Kernel();
			return ret;
	}

	// Kernel(const char *, const Program &)(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:391
	// ("cv::ocl::Kernel::Kernel", vec![(pred!(mut, ["kname", "prog"], ["const char*", "const cv::ocl::Program*"]), _)]),
	void cv_ocl_Kernel_Kernel_const_charX_const_ProgramR(const char* kname, const cv::ocl::Program* prog, Result<cv::ocl::Kernel*>* ocvrs_return) {
		try {
			cv::ocl::Kernel* ret = new cv::ocl::Kernel(kname, *prog);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Kernel(const char *, const ProgramSource &, const String &, String *)(InString, TraitClass, InString, OutString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:392
	// ("cv::ocl::Kernel::Kernel", vec![(pred!(mut, ["kname", "prog", "buildopts", "errmsg"], ["const char*", "const cv::ocl::ProgramSource*", "const cv::String*", "cv::String*"]), _)]),
	void cv_ocl_Kernel_Kernel_const_charX_const_ProgramSourceR_const_StringR_StringX(const char* kname, const cv::ocl::ProgramSource* prog, const char* buildopts, void** errmsg, Result<cv::ocl::Kernel*>* ocvrs_return) {
		try {
			cv::String errmsg_out;
			cv::ocl::Kernel* ret = new cv::ocl::Kernel(kname, *prog, cv::String(buildopts), &errmsg_out);
			*errmsg = ocvrs_create_string(errmsg_out.c_str());
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::Kernel::Kernel(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:392
	// ("cv::ocl::Kernel::Kernel", vec![(pred!(mut, ["kname", "prog"], ["const char*", "const cv::ocl::ProgramSource*"]), _)]),
	void cv_ocl_Kernel_Kernel_const_charX_const_ProgramSourceR(const char* kname, const cv::ocl::ProgramSource* prog, Result<cv::ocl::Kernel*>* ocvrs_return) {
		try {
			cv::ocl::Kernel* ret = new cv::ocl::Kernel(kname, *prog);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Kernel(const Kernel &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:395
	// ("cv::ocl::Kernel::Kernel", vec![(pred!(mut, ["k"], ["const cv::ocl::Kernel*"]), _)]),
	void cv_ocl_Kernel_Kernel_const_KernelR(const cv::ocl::Kernel* k, Result<cv::ocl::Kernel*>* ocvrs_return) {
		try {
			cv::ocl::Kernel* ret = new cv::ocl::Kernel(*k);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const Kernel &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:396
	// ("cv::ocl::Kernel::operator=", vec![(pred!(mut, ["k"], ["const cv::ocl::Kernel*"]), _)]),
	void cv_ocl_Kernel_operatorST_const_KernelR(cv::ocl::Kernel* instance, const cv::ocl::Kernel* k, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:398
	// ("cv::ocl::Kernel::empty", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Kernel_empty_const(const cv::ocl::Kernel* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const char *, const Program &)(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:399
	// ("cv::ocl::Kernel::create", vec![(pred!(mut, ["kname", "prog"], ["const char*", "const cv::ocl::Program*"]), _)]),
	void cv_ocl_Kernel_create_const_charX_const_ProgramR(cv::ocl::Kernel* instance, const char* kname, const cv::ocl::Program* prog, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->create(kname, *prog);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const char *, const ProgramSource &, const String &, String *)(InString, TraitClass, InString, OutString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:400
	// ("cv::ocl::Kernel::create", vec![(pred!(mut, ["kname", "prog", "buildopts", "errmsg"], ["const char*", "const cv::ocl::ProgramSource*", "const cv::String*", "cv::String*"]), _)]),
	void cv_ocl_Kernel_create_const_charX_const_ProgramSourceR_const_StringR_StringX(cv::ocl::Kernel* instance, const char* kname, const cv::ocl::ProgramSource* prog, const char* buildopts, void** errmsg, Result<bool>* ocvrs_return) {
		try {
			cv::String errmsg_out;
			bool ret = instance->create(kname, *prog, cv::String(buildopts), &errmsg_out);
			*errmsg = ocvrs_create_string(errmsg_out.c_str());
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::Kernel::create(InString, TraitClass, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:400
	// ("cv::ocl::Kernel::create", vec![(pred!(mut, ["kname", "prog", "buildopts"], ["const char*", "const cv::ocl::ProgramSource*", "const cv::String*"]), _)]),
	void cv_ocl_Kernel_create_const_charX_const_ProgramSourceR_const_StringR(cv::ocl::Kernel* instance, const char* kname, const cv::ocl::ProgramSource* prog, const char* buildopts, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->create(kname, *prog, cv::String(buildopts));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// set(int, const void *, size_t)(Primitive, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:403
	// ("cv::ocl::Kernel::set", vec![(pred!(mut, ["i", "value", "sz"], ["int", "const void*", "size_t"]), _)]),
	void cv_ocl_Kernel_set_int_const_voidX_size_t(cv::ocl::Kernel* instance, int i, const void* value, size_t sz, Result<int>* ocvrs_return) {
		try {
			int ret = instance->set(i, value, sz);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// set(int, const Image2D &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:404
	// ("cv::ocl::Kernel::set", vec![(pred!(mut, ["i", "image2D"], ["int", "const cv::ocl::Image2D*"]), _)]),
	void cv_ocl_Kernel_set_int_const_Image2DR(cv::ocl::Kernel* instance, int i, const cv::ocl::Image2D* image2D, Result<int>* ocvrs_return) {
		try {
			int ret = instance->set(i, *image2D);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// set(int, const UMat &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:405
	// ("cv::ocl::Kernel::set", vec![(pred!(mut, ["i", "m"], ["int", "const cv::UMat*"]), _)]),
	void cv_ocl_Kernel_set_int_const_UMatR(cv::ocl::Kernel* instance, int i, const cv::UMat* m, Result<int>* ocvrs_return) {
		try {
			int ret = instance->set(i, *m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// set(int, const KernelArg &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:406
	// ("cv::ocl::Kernel::set", vec![(pred!(mut, ["i", "arg"], ["int", "const cv::ocl::KernelArg*"]), _)]),
	void cv_ocl_Kernel_set_int_const_KernelArgR(cv::ocl::Kernel* instance, int i, const cv::ocl::KernelArg* arg, Result<int>* ocvrs_return) {
		try {
			int ret = instance->set(i, *arg);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// run(int, size_t *, size_t *, bool, const Queue &)(Primitive, VariableArray, VariableArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:579
	// ("cv::ocl::Kernel::run", vec![(pred!(mut, ["dims", "globalsize", "localsize", "sync", "q"], ["int", "size_t*", "size_t*", "bool", "const cv::ocl::Queue*"]), _)]),
	void cv_ocl_Kernel_run_int_size_tX_size_tX_bool_const_QueueR(cv::ocl::Kernel* instance, int dims, size_t* globalsize, size_t* localsize, bool sync, const cv::ocl::Queue* q, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->run(dims, globalsize, localsize, sync, *q);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::Kernel::run(Primitive, VariableArray, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:579
	// ("cv::ocl::Kernel::run", vec![(pred!(mut, ["dims", "globalsize", "localsize", "sync"], ["int", "size_t*", "size_t*", "bool"]), _)]),
	void cv_ocl_Kernel_run_int_size_tX_size_tX_bool(cv::ocl::Kernel* instance, int dims, size_t* globalsize, size_t* localsize, bool sync, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->run(dims, globalsize, localsize, sync);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// run_(int, size_t *, size_t *, bool, const Queue &)(Primitive, VariableArray, VariableArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:590
	// ("cv::ocl::Kernel::run_", vec![(pred!(mut, ["dims", "globalsize", "localsize", "sync", "q"], ["int", "size_t*", "size_t*", "bool", "const cv::ocl::Queue*"]), _)]),
	void cv_ocl_Kernel_run__int_size_tX_size_tX_bool_const_QueueR(cv::ocl::Kernel* instance, int dims, size_t* globalsize, size_t* localsize, bool sync, const cv::ocl::Queue* q, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->run_(dims, globalsize, localsize, sync, *q);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::Kernel::run_(Primitive, VariableArray, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:590
	// ("cv::ocl::Kernel::run_", vec![(pred!(mut, ["dims", "globalsize", "localsize", "sync"], ["int", "size_t*", "size_t*", "bool"]), _)]),
	void cv_ocl_Kernel_run__int_size_tX_size_tX_bool(cv::ocl::Kernel* instance, int dims, size_t* globalsize, size_t* localsize, bool sync, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->run_(dims, globalsize, localsize, sync);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// runTask(bool, const Queue &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:592
	// ("cv::ocl::Kernel::runTask", vec![(pred!(mut, ["sync", "q"], ["bool", "const cv::ocl::Queue*"]), _)]),
	void cv_ocl_Kernel_runTask_bool_const_QueueR(cv::ocl::Kernel* instance, bool sync, const cv::ocl::Queue* q, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->runTask(sync, *q);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::Kernel::runTask(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:592
	// ("cv::ocl::Kernel::runTask", vec![(pred!(mut, ["sync"], ["bool"]), _)]),
	void cv_ocl_Kernel_runTask_bool(cv::ocl::Kernel* instance, bool sync, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->runTask(sync);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// runProfiling(int, size_t *, size_t *, const Queue &)(Primitive, VariableArray, VariableArray, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:599
	// ("cv::ocl::Kernel::runProfiling", vec![(pred!(mut, ["dims", "globalsize", "localsize", "q"], ["int", "size_t*", "size_t*", "const cv::ocl::Queue*"]), _)]),
	void cv_ocl_Kernel_runProfiling_int_size_tX_size_tX_const_QueueR(cv::ocl::Kernel* instance, int dims, size_t* globalsize, size_t* localsize, const cv::ocl::Queue* q, Result<int64_t>* ocvrs_return) {
		try {
			int64_t ret = instance->runProfiling(dims, globalsize, localsize, *q);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::Kernel::runProfiling(Primitive, VariableArray, VariableArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:599
	// ("cv::ocl::Kernel::runProfiling", vec![(pred!(mut, ["dims", "globalsize", "localsize"], ["int", "size_t*", "size_t*"]), _)]),
	void cv_ocl_Kernel_runProfiling_int_size_tX_size_tX(cv::ocl::Kernel* instance, int dims, size_t* globalsize, size_t* localsize, Result<int64_t>* ocvrs_return) {
		try {
			int64_t ret = instance->runProfiling(dims, globalsize, localsize);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// workGroupSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:601
	// ("cv::ocl::Kernel::workGroupSize", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Kernel_workGroupSize_const(const cv::ocl::Kernel* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->workGroupSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// preferedWorkGroupSizeMultiple()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:602
	// ("cv::ocl::Kernel::preferedWorkGroupSizeMultiple", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Kernel_preferedWorkGroupSizeMultiple_const(const cv::ocl::Kernel* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->preferedWorkGroupSizeMultiple();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compileWorkGroupSize(size_t *)(VariableArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:603
	// ("cv::ocl::Kernel::compileWorkGroupSize", vec![(pred!(const, ["wsz"], ["size_t*"]), _)]),
	void cv_ocl_Kernel_compileWorkGroupSize_const_size_tX(const cv::ocl::Kernel* instance, size_t* wsz, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->compileWorkGroupSize(wsz);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// localMemSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:604
	// ("cv::ocl::Kernel::localMemSize", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Kernel_localMemSize_const(const cv::ocl::Kernel* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->localMemSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ptr()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:606
	// ("cv::ocl::Kernel::ptr", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Kernel_ptr_const(const cv::ocl::Kernel* instance, Result<void*>* ocvrs_return) {
		try {
			void* ret = instance->ptr();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::Kernel::delete() generated
	// ("cv::ocl::Kernel::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ocl_Kernel_delete(cv::ocl::Kernel* instance) {
			delete instance;
	}

	// KernelArg(int, UMat *, int, int, const void *, size_t)(Primitive, TraitClass, Primitive, Primitive, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:352
	// ("cv::ocl::KernelArg::KernelArg", vec![(pred!(mut, ["_flags", "_m", "wscale", "iwscale", "_obj", "_sz"], ["int", "cv::UMat*", "int", "int", "const void*", "size_t"]), _)]),
	void cv_ocl_KernelArg_KernelArg_int_UMatX_int_int_const_voidX_size_t(int _flags, cv::UMat* _m, int wscale, int iwscale, const void* _obj, size_t _sz, Result<cv::ocl::KernelArg*>* ocvrs_return) {
		try {
			cv::ocl::KernelArg* ret = new cv::ocl::KernelArg(_flags, _m, wscale, iwscale, _obj, _sz);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::KernelArg::KernelArg(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:352
	// ("cv::ocl::KernelArg::KernelArg", vec![(pred!(mut, ["_flags", "_m"], ["int", "cv::UMat*"]), _)]),
	void cv_ocl_KernelArg_KernelArg_int_UMatX(int _flags, cv::UMat* _m, Result<cv::ocl::KernelArg*>* ocvrs_return) {
		try {
			cv::ocl::KernelArg* ret = new cv::ocl::KernelArg(_flags, _m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// KernelArg()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:353
	// ("cv::ocl::KernelArg::KernelArg", vec![(pred!(mut, [], []), _)]),
	cv::ocl::KernelArg* cv_ocl_KernelArg_KernelArg() {
			cv::ocl::KernelArg* ret = new cv::ocl::KernelArg();
			return ret;
	}

	// Local(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:355
	// ("cv::ocl::KernelArg::Local", vec![(pred!(mut, ["localMemSize"], ["size_t"]), _)]),
	void cv_ocl_KernelArg_Local_size_t(size_t localMemSize, Result<cv::ocl::KernelArg*>* ocvrs_return) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::Local(localMemSize);
			Ok(new cv::ocl::KernelArg(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// PtrWriteOnly(const UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:357
	// ("cv::ocl::KernelArg::PtrWriteOnly", vec![(pred!(mut, ["m"], ["const cv::UMat*"]), _)]),
	void cv_ocl_KernelArg_PtrWriteOnly_const_UMatR(const cv::UMat* m, Result<cv::ocl::KernelArg*>* ocvrs_return) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::PtrWriteOnly(*m);
			Ok(new cv::ocl::KernelArg(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// PtrReadOnly(const UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:359
	// ("cv::ocl::KernelArg::PtrReadOnly", vec![(pred!(mut, ["m"], ["const cv::UMat*"]), _)]),
	void cv_ocl_KernelArg_PtrReadOnly_const_UMatR(const cv::UMat* m, Result<cv::ocl::KernelArg*>* ocvrs_return) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::PtrReadOnly(*m);
			Ok(new cv::ocl::KernelArg(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// PtrReadWrite(const UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:361
	// ("cv::ocl::KernelArg::PtrReadWrite", vec![(pred!(mut, ["m"], ["const cv::UMat*"]), _)]),
	void cv_ocl_KernelArg_PtrReadWrite_const_UMatR(const cv::UMat* m, Result<cv::ocl::KernelArg*>* ocvrs_return) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::PtrReadWrite(*m);
			Ok(new cv::ocl::KernelArg(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ReadWrite(const UMat &, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:363
	// ("cv::ocl::KernelArg::ReadWrite", vec![(pred!(mut, ["m", "wscale", "iwscale"], ["const cv::UMat*", "int", "int"]), _)]),
	void cv_ocl_KernelArg_ReadWrite_const_UMatR_int_int(const cv::UMat* m, int wscale, int iwscale, Result<cv::ocl::KernelArg*>* ocvrs_return) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::ReadWrite(*m, wscale, iwscale);
			Ok(new cv::ocl::KernelArg(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::KernelArg::ReadWrite(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:363
	// ("cv::ocl::KernelArg::ReadWrite", vec![(pred!(mut, ["m"], ["const cv::UMat*"]), _)]),
	void cv_ocl_KernelArg_ReadWrite_const_UMatR(const cv::UMat* m, Result<cv::ocl::KernelArg*>* ocvrs_return) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::ReadWrite(*m);
			Ok(new cv::ocl::KernelArg(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ReadWriteNoSize(const UMat &, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:365
	// ("cv::ocl::KernelArg::ReadWriteNoSize", vec![(pred!(mut, ["m", "wscale", "iwscale"], ["const cv::UMat*", "int", "int"]), _)]),
	void cv_ocl_KernelArg_ReadWriteNoSize_const_UMatR_int_int(const cv::UMat* m, int wscale, int iwscale, Result<cv::ocl::KernelArg*>* ocvrs_return) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::ReadWriteNoSize(*m, wscale, iwscale);
			Ok(new cv::ocl::KernelArg(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::KernelArg::ReadWriteNoSize(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:365
	// ("cv::ocl::KernelArg::ReadWriteNoSize", vec![(pred!(mut, ["m"], ["const cv::UMat*"]), _)]),
	void cv_ocl_KernelArg_ReadWriteNoSize_const_UMatR(const cv::UMat* m, Result<cv::ocl::KernelArg*>* ocvrs_return) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::ReadWriteNoSize(*m);
			Ok(new cv::ocl::KernelArg(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ReadOnly(const UMat &, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:367
	// ("cv::ocl::KernelArg::ReadOnly", vec![(pred!(mut, ["m", "wscale", "iwscale"], ["const cv::UMat*", "int", "int"]), _)]),
	void cv_ocl_KernelArg_ReadOnly_const_UMatR_int_int(const cv::UMat* m, int wscale, int iwscale, Result<cv::ocl::KernelArg*>* ocvrs_return) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::ReadOnly(*m, wscale, iwscale);
			Ok(new cv::ocl::KernelArg(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::KernelArg::ReadOnly(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:367
	// ("cv::ocl::KernelArg::ReadOnly", vec![(pred!(mut, ["m"], ["const cv::UMat*"]), _)]),
	void cv_ocl_KernelArg_ReadOnly_const_UMatR(const cv::UMat* m, Result<cv::ocl::KernelArg*>* ocvrs_return) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::ReadOnly(*m);
			Ok(new cv::ocl::KernelArg(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// WriteOnly(const UMat &, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:369
	// ("cv::ocl::KernelArg::WriteOnly", vec![(pred!(mut, ["m", "wscale", "iwscale"], ["const cv::UMat*", "int", "int"]), _)]),
	void cv_ocl_KernelArg_WriteOnly_const_UMatR_int_int(const cv::UMat* m, int wscale, int iwscale, Result<cv::ocl::KernelArg*>* ocvrs_return) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::WriteOnly(*m, wscale, iwscale);
			Ok(new cv::ocl::KernelArg(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::KernelArg::WriteOnly(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:369
	// ("cv::ocl::KernelArg::WriteOnly", vec![(pred!(mut, ["m"], ["const cv::UMat*"]), _)]),
	void cv_ocl_KernelArg_WriteOnly_const_UMatR(const cv::UMat* m, Result<cv::ocl::KernelArg*>* ocvrs_return) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::WriteOnly(*m);
			Ok(new cv::ocl::KernelArg(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ReadOnlyNoSize(const UMat &, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:371
	// ("cv::ocl::KernelArg::ReadOnlyNoSize", vec![(pred!(mut, ["m", "wscale", "iwscale"], ["const cv::UMat*", "int", "int"]), _)]),
	void cv_ocl_KernelArg_ReadOnlyNoSize_const_UMatR_int_int(const cv::UMat* m, int wscale, int iwscale, Result<cv::ocl::KernelArg*>* ocvrs_return) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::ReadOnlyNoSize(*m, wscale, iwscale);
			Ok(new cv::ocl::KernelArg(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::KernelArg::ReadOnlyNoSize(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:371
	// ("cv::ocl::KernelArg::ReadOnlyNoSize", vec![(pred!(mut, ["m"], ["const cv::UMat*"]), _)]),
	void cv_ocl_KernelArg_ReadOnlyNoSize_const_UMatR(const cv::UMat* m, Result<cv::ocl::KernelArg*>* ocvrs_return) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::ReadOnlyNoSize(*m);
			Ok(new cv::ocl::KernelArg(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// WriteOnlyNoSize(const UMat &, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:373
	// ("cv::ocl::KernelArg::WriteOnlyNoSize", vec![(pred!(mut, ["m", "wscale", "iwscale"], ["const cv::UMat*", "int", "int"]), _)]),
	void cv_ocl_KernelArg_WriteOnlyNoSize_const_UMatR_int_int(const cv::UMat* m, int wscale, int iwscale, Result<cv::ocl::KernelArg*>* ocvrs_return) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::WriteOnlyNoSize(*m, wscale, iwscale);
			Ok(new cv::ocl::KernelArg(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::KernelArg::WriteOnlyNoSize(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:373
	// ("cv::ocl::KernelArg::WriteOnlyNoSize", vec![(pred!(mut, ["m"], ["const cv::UMat*"]), _)]),
	void cv_ocl_KernelArg_WriteOnlyNoSize_const_UMatR(const cv::UMat* m, Result<cv::ocl::KernelArg*>* ocvrs_return) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::WriteOnlyNoSize(*m);
			Ok(new cv::ocl::KernelArg(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Constant(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:375
	// ("cv::ocl::KernelArg::Constant", vec![(pred!(mut, ["m"], ["const cv::Mat*"]), _)]),
	void cv_ocl_KernelArg_Constant_const_MatR(const cv::Mat* m, Result<cv::ocl::KernelArg*>* ocvrs_return) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::Constant(*m);
			Ok(new cv::ocl::KernelArg(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::KernelArg::flags() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:379
	// ("cv::ocl::KernelArg::flags", vec![(pred!(const, [], []), _)]),
	int cv_ocl_KernelArg_propFlags_const(const cv::ocl::KernelArg* instance) {
			int ret = instance->flags;
			return ret;
	}

	// cv::ocl::KernelArg::setFlags(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:379
	// ("cv::ocl::KernelArg::setFlags", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_ocl_KernelArg_propFlags_const_int(cv::ocl::KernelArg* instance, const int val) {
			instance->flags = val;
	}

	// cv::ocl::KernelArg::m() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:380
	// ("cv::ocl::KernelArg::m", vec![(pred!(mut, [], []), _)]),
	cv::UMat* cv_ocl_KernelArg_propM(cv::ocl::KernelArg* instance) {
			cv::UMat* ret = instance->m;
			return new cv::UMat(*ret);
	}

	// cv::ocl::KernelArg::setM(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:380
	// ("cv::ocl::KernelArg::setM", vec![(pred!(mut, ["val"], ["cv::UMat*"]), _)]),
	void cv_ocl_KernelArg_propM_UMatX(cv::ocl::KernelArg* instance, cv::UMat* const val) {
			instance->m = val;
	}

	// cv::ocl::KernelArg::obj() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:381
	// ("cv::ocl::KernelArg::obj", vec![(pred!(const, [], []), _)]),
	const void* cv_ocl_KernelArg_propObj_const(const cv::ocl::KernelArg* instance) {
			const void* ret = instance->obj;
			return ret;
	}

	// cv::ocl::KernelArg::sz() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:382
	// ("cv::ocl::KernelArg::sz", vec![(pred!(const, [], []), _)]),
	size_t cv_ocl_KernelArg_propSz_const(const cv::ocl::KernelArg* instance) {
			size_t ret = instance->sz;
			return ret;
	}

	// cv::ocl::KernelArg::setSz(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:382
	// ("cv::ocl::KernelArg::setSz", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
	void cv_ocl_KernelArg_propSz_const_size_t(cv::ocl::KernelArg* instance, const size_t val) {
			instance->sz = val;
	}

	// cv::ocl::KernelArg::wscale() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:383
	// ("cv::ocl::KernelArg::wscale", vec![(pred!(const, [], []), _)]),
	int cv_ocl_KernelArg_propWscale_const(const cv::ocl::KernelArg* instance) {
			int ret = instance->wscale;
			return ret;
	}

	// cv::ocl::KernelArg::setWscale(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:383
	// ("cv::ocl::KernelArg::setWscale", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_ocl_KernelArg_propWscale_const_int(cv::ocl::KernelArg* instance, const int val) {
			instance->wscale = val;
	}

	// cv::ocl::KernelArg::iwscale() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:383
	// ("cv::ocl::KernelArg::iwscale", vec![(pred!(const, [], []), _)]),
	int cv_ocl_KernelArg_propIwscale_const(const cv::ocl::KernelArg* instance) {
			int ret = instance->iwscale;
			return ret;
	}

	// cv::ocl::KernelArg::setIwscale(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:383
	// ("cv::ocl::KernelArg::setIwscale", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_ocl_KernelArg_propIwscale_const_int(cv::ocl::KernelArg* instance, const int val) {
			instance->iwscale = val;
	}

	// cv::ocl::KernelArg::delete() generated
	// ("cv::ocl::KernelArg::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ocl_KernelArg_delete(cv::ocl::KernelArg* instance) {
			delete instance;
	}

	// Platform()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:272
	// ("cv::ocl::Platform::Platform", vec![(pred!(mut, [], []), _)]),
	cv::ocl::Platform* cv_ocl_Platform_Platform() {
			cv::ocl::Platform* ret = new cv::ocl::Platform();
			return ret;
	}

	// Platform(const Platform &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:274
	// ("cv::ocl::Platform::Platform", vec![(pred!(mut, ["p"], ["const cv::ocl::Platform*"]), _)]),
	void cv_ocl_Platform_Platform_const_PlatformR(const cv::ocl::Platform* p, Result<cv::ocl::Platform*>* ocvrs_return) {
		try {
			cv::ocl::Platform* ret = new cv::ocl::Platform(*p);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const Platform &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:275
	// ("cv::ocl::Platform::operator=", vec![(pred!(mut, ["p"], ["const cv::ocl::Platform*"]), _)]),
	void cv_ocl_Platform_operatorST_const_PlatformR(cv::ocl::Platform* instance, const cv::ocl::Platform* p, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*p);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ptr()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:277
	// ("cv::ocl::Platform::ptr", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Platform_ptr_const(const cv::ocl::Platform* instance, Result<void*>* ocvrs_return) {
		try {
			void* ret = instance->ptr();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefault()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:278
	// ("cv::ocl::Platform::getDefault", vec![(pred!(mut, [], []), _)]),
	void cv_ocl_Platform_getDefault(Result<cv::ocl::Platform*>* ocvrs_return) {
		try {
			cv::ocl::Platform ret = cv::ocl::Platform::getDefault();
			Ok(new cv::ocl::Platform(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::Platform::delete() generated
	// ("cv::ocl::Platform::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ocl_Platform_delete(cv::ocl::Platform* instance) {
			delete instance;
	}

	// PlatformInfo()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:730
	// ("cv::ocl::PlatformInfo::PlatformInfo", vec![(pred!(mut, [], []), _)]),
	cv::ocl::PlatformInfo* cv_ocl_PlatformInfo_PlatformInfo() {
			cv::ocl::PlatformInfo* ret = new cv::ocl::PlatformInfo();
			return ret;
	}

	// PlatformInfo(void *)(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:731
	// ("cv::ocl::PlatformInfo::PlatformInfo", vec![(pred!(mut, ["id"], ["void*"]), _)]),
	void cv_ocl_PlatformInfo_PlatformInfo_voidX(void* id, Result<cv::ocl::PlatformInfo*>* ocvrs_return) {
		try {
			cv::ocl::PlatformInfo* ret = new cv::ocl::PlatformInfo(id);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// PlatformInfo(const PlatformInfo &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:734
	// ("cv::ocl::PlatformInfo::PlatformInfo", vec![(pred!(mut, ["i"], ["const cv::ocl::PlatformInfo*"]), _)]),
	void cv_ocl_PlatformInfo_PlatformInfo_const_PlatformInfoR(const cv::ocl::PlatformInfo* i, Result<cv::ocl::PlatformInfo*>* ocvrs_return) {
		try {
			cv::ocl::PlatformInfo* ret = new cv::ocl::PlatformInfo(*i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const PlatformInfo &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:735
	// ("cv::ocl::PlatformInfo::operator=", vec![(pred!(mut, ["i"], ["const cv::ocl::PlatformInfo*"]), _)]),
	void cv_ocl_PlatformInfo_operatorST_const_PlatformInfoR(cv::ocl::PlatformInfo* instance, const cv::ocl::PlatformInfo* i, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*i);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// name()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:737
	// ("cv::ocl::PlatformInfo::name", vec![(pred!(const, [], []), _)]),
	void cv_ocl_PlatformInfo_name_const(const cv::ocl::PlatformInfo* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->name();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// vendor()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:738
	// ("cv::ocl::PlatformInfo::vendor", vec![(pred!(const, [], []), _)]),
	void cv_ocl_PlatformInfo_vendor_const(const cv::ocl::PlatformInfo* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->vendor();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// version()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:741
	// ("cv::ocl::PlatformInfo::version", vec![(pred!(const, [], []), _)]),
	void cv_ocl_PlatformInfo_version_const(const cv::ocl::PlatformInfo* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->version();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// versionMajor()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:742
	// ("cv::ocl::PlatformInfo::versionMajor", vec![(pred!(const, [], []), _)]),
	void cv_ocl_PlatformInfo_versionMajor_const(const cv::ocl::PlatformInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->versionMajor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// versionMinor()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:743
	// ("cv::ocl::PlatformInfo::versionMinor", vec![(pred!(const, [], []), _)]),
	void cv_ocl_PlatformInfo_versionMinor_const(const cv::ocl::PlatformInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->versionMinor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// deviceNumber()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:745
	// ("cv::ocl::PlatformInfo::deviceNumber", vec![(pred!(const, [], []), _)]),
	void cv_ocl_PlatformInfo_deviceNumber_const(const cv::ocl::PlatformInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->deviceNumber();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDevice(Device &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:746
	// ("cv::ocl::PlatformInfo::getDevice", vec![(pred!(const, ["device", "d"], ["cv::ocl::Device*", "int"]), _)]),
	void cv_ocl_PlatformInfo_getDevice_const_DeviceR_int(const cv::ocl::PlatformInfo* instance, cv::ocl::Device* device, int d, ResultVoid* ocvrs_return) {
		try {
			instance->getDevice(*device, d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::PlatformInfo::delete() generated
	// ("cv::ocl::PlatformInfo::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ocl_PlatformInfo_delete(cv::ocl::PlatformInfo* instance) {
			delete instance;
	}

	// Program()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:616
	// ("cv::ocl::Program::Program", vec![(pred!(mut, [], []), _)]),
	cv::ocl::Program* cv_ocl_Program_Program() {
			cv::ocl::Program* ret = new cv::ocl::Program();
			return ret;
	}

	// Program(const ProgramSource &, const String &, String &)(TraitClass, InString, OutString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:617
	// ("cv::ocl::Program::Program", vec![(pred!(mut, ["src", "buildflags", "errmsg"], ["const cv::ocl::ProgramSource*", "const cv::String*", "cv::String*"]), _)]),
	void cv_ocl_Program_Program_const_ProgramSourceR_const_StringR_StringR(const cv::ocl::ProgramSource* src, const char* buildflags, void** errmsg, Result<cv::ocl::Program*>* ocvrs_return) {
		try {
			cv::String errmsg_out;
			cv::ocl::Program* ret = new cv::ocl::Program(*src, cv::String(buildflags), errmsg_out);
			*errmsg = ocvrs_create_string(errmsg_out.c_str());
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Program(const Program &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:619
	// ("cv::ocl::Program::Program", vec![(pred!(mut, ["prog"], ["const cv::ocl::Program*"]), _)]),
	void cv_ocl_Program_Program_const_ProgramR(const cv::ocl::Program* prog, Result<cv::ocl::Program*>* ocvrs_return) {
		try {
			cv::ocl::Program* ret = new cv::ocl::Program(*prog);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const Program &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:621
	// ("cv::ocl::Program::operator=", vec![(pred!(mut, ["prog"], ["const cv::ocl::Program*"]), _)]),
	void cv_ocl_Program_operatorST_const_ProgramR(cv::ocl::Program* instance, const cv::ocl::Program* prog, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*prog);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const ProgramSource &, const String &, String &)(TraitClass, InString, OutString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:624
	// ("cv::ocl::Program::create", vec![(pred!(mut, ["src", "buildflags", "errmsg"], ["const cv::ocl::ProgramSource*", "const cv::String*", "cv::String*"]), _)]),
	void cv_ocl_Program_create_const_ProgramSourceR_const_StringR_StringR(cv::ocl::Program* instance, const cv::ocl::ProgramSource* src, const char* buildflags, void** errmsg, Result<bool>* ocvrs_return) {
		try {
			cv::String errmsg_out;
			bool ret = instance->create(*src, cv::String(buildflags), errmsg_out);
			*errmsg = ocvrs_create_string(errmsg_out.c_str());
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ptr()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:627
	// ("cv::ocl::Program::ptr", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Program_ptr_const(const cv::ocl::Program* instance, Result<void*>* ocvrs_return) {
		try {
			void* ret = instance->ptr();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBinary(std::vector<char> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:638
	// ("cv::ocl::Program::getBinary", vec![(pred!(const, ["binary"], ["std::vector<char>*"]), _)]),
	void cv_ocl_Program_getBinary_const_vectorLcharGR(const cv::ocl::Program* instance, std::vector<char>* binary, ResultVoid* ocvrs_return) {
		try {
			instance->getBinary(*binary);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:647
	// ("cv::ocl::Program::read", vec![(pred!(mut, ["buf", "buildflags"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_ocl_Program_read_const_StringR_const_StringR(cv::ocl::Program* instance, const char* buf, const char* buildflags, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->read(cv::String(buf), cv::String(buildflags));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(String &)(OutString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:648
	// ("cv::ocl::Program::write", vec![(pred!(const, ["buf"], ["cv::String*"]), _)]),
	void cv_ocl_Program_write_const_StringR(const cv::ocl::Program* instance, void** buf, Result<bool>* ocvrs_return) {
		try {
			cv::String buf_out;
			bool ret = instance->write(buf_out);
			*buf = ocvrs_create_string(buf_out.c_str());
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// source()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:649
	// ("cv::ocl::Program::source", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Program_source_const(const cv::ocl::Program* instance, Result<cv::ocl::ProgramSource*>* ocvrs_return) {
		try {
			const cv::ocl::ProgramSource ret = instance->source();
			Ok(new const cv::ocl::ProgramSource(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPrefix()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:650
	// ("cv::ocl::Program::getPrefix", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Program_getPrefix_const(const cv::ocl::Program* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getPrefix();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPrefix(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:651
	// ("cv::ocl::Program::getPrefix", vec![(pred!(mut, ["buildflags"], ["const cv::String*"]), _)]),
	void cv_ocl_Program_getPrefix_const_StringR(const char* buildflags, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::ocl::Program::getPrefix(cv::String(buildflags));
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::Program::delete() generated
	// ("cv::ocl::Program::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ocl_Program_delete(cv::ocl::Program* instance) {
			delete instance;
	}

	// ProgramSource()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:661
	// ("cv::ocl::ProgramSource::ProgramSource", vec![(pred!(mut, [], []), _)]),
	cv::ocl::ProgramSource* cv_ocl_ProgramSource_ProgramSource() {
			cv::ocl::ProgramSource* ret = new cv::ocl::ProgramSource();
			return ret;
	}

	// ProgramSource(const String &, const String &, const String &, const String &)(InString, InString, InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:662
	// ("cv::ocl::ProgramSource::ProgramSource", vec![(pred!(mut, ["module", "name", "codeStr", "codeHash"], ["const cv::String*", "const cv::String*", "const cv::String*", "const cv::String*"]), _)]),
	void cv_ocl_ProgramSource_ProgramSource_const_StringR_const_StringR_const_StringR_const_StringR(const char* module, const char* name, const char* codeStr, const char* codeHash, Result<cv::ocl::ProgramSource*>* ocvrs_return) {
		try {
			cv::ocl::ProgramSource* ret = new cv::ocl::ProgramSource(cv::String(module), cv::String(name), cv::String(codeStr), cv::String(codeHash));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ProgramSource(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:663
	// ("cv::ocl::ProgramSource::ProgramSource", vec![(pred!(mut, ["prog"], ["const cv::String*"]), _)]),
	void cv_ocl_ProgramSource_ProgramSource_const_StringR(const char* prog, Result<cv::ocl::ProgramSource*>* ocvrs_return) {
		try {
			cv::ocl::ProgramSource* ret = new cv::ocl::ProgramSource(cv::String(prog));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ProgramSource(const ProgramSource &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:666
	// ("cv::ocl::ProgramSource::ProgramSource", vec![(pred!(mut, ["prog"], ["const cv::ocl::ProgramSource*"]), _)]),
	void cv_ocl_ProgramSource_ProgramSource_const_ProgramSourceR(const cv::ocl::ProgramSource* prog, Result<cv::ocl::ProgramSource*>* ocvrs_return) {
		try {
			cv::ocl::ProgramSource* ret = new cv::ocl::ProgramSource(*prog);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const ProgramSource &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:667
	// ("cv::ocl::ProgramSource::operator=", vec![(pred!(mut, ["prog"], ["const cv::ocl::ProgramSource*"]), _)]),
	void cv_ocl_ProgramSource_operatorST_const_ProgramSourceR(cv::ocl::ProgramSource* instance, const cv::ocl::ProgramSource* prog, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*prog);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// source()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:669
	// ("cv::ocl::ProgramSource::source", vec![(pred!(const, [], []), _)]),
	void cv_ocl_ProgramSource_source_const(const cv::ocl::ProgramSource* instance, Result<void*>* ocvrs_return) {
		try {
			const cv::String ret = instance->source();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// hash()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:670
	// ("cv::ocl::ProgramSource::hash", vec![(pred!(const, [], []), _)]),
	void cv_ocl_ProgramSource_hash_const(const cv::ocl::ProgramSource* instance, Result<cv::ocl::ProgramSource::hash_t>* ocvrs_return) {
		try {
			cv::ocl::ProgramSource::hash_t ret = instance->hash();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fromBinary(const String &, const String &, const unsigned char *, const size_t, const cv::String &)(InString, InString, VariableArray, Primitive, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:687
	// ("cv::ocl::ProgramSource::fromBinary", vec![(pred!(mut, ["module", "name", "binary", "size", "buildOptions"], ["const cv::String*", "const cv::String*", "const unsigned char*", "const size_t", "const cv::String*"]), _)]),
	void cv_ocl_ProgramSource_fromBinary_const_StringR_const_StringR_const_unsigned_charX_const_size_t_const_StringR(const char* module, const char* name, const unsigned char* binary, const size_t size, const char* buildOptions, Result<cv::ocl::ProgramSource*>* ocvrs_return) {
		try {
			cv::ocl::ProgramSource ret = cv::ocl::ProgramSource::fromBinary(cv::String(module), cv::String(name), binary, size, cv::String(buildOptions));
			Ok(new cv::ocl::ProgramSource(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::ProgramSource::fromBinary(InString, InString, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:687
	// ("cv::ocl::ProgramSource::fromBinary", vec![(pred!(mut, ["module", "name", "binary", "size"], ["const cv::String*", "const cv::String*", "const unsigned char*", "const size_t"]), _)]),
	void cv_ocl_ProgramSource_fromBinary_const_StringR_const_StringR_const_unsigned_charX_const_size_t(const char* module, const char* name, const unsigned char* binary, const size_t size, Result<cv::ocl::ProgramSource*>* ocvrs_return) {
		try {
			cv::ocl::ProgramSource ret = cv::ocl::ProgramSource::fromBinary(cv::String(module), cv::String(name), binary, size);
			Ok(new cv::ocl::ProgramSource(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fromSPIR(const String &, const String &, const unsigned char *, const size_t, const cv::String &)(InString, InString, VariableArray, Primitive, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:712
	// ("cv::ocl::ProgramSource::fromSPIR", vec![(pred!(mut, ["module", "name", "binary", "size", "buildOptions"], ["const cv::String*", "const cv::String*", "const unsigned char*", "const size_t", "const cv::String*"]), _)]),
	void cv_ocl_ProgramSource_fromSPIR_const_StringR_const_StringR_const_unsigned_charX_const_size_t_const_StringR(const char* module, const char* name, const unsigned char* binary, const size_t size, const char* buildOptions, Result<cv::ocl::ProgramSource*>* ocvrs_return) {
		try {
			cv::ocl::ProgramSource ret = cv::ocl::ProgramSource::fromSPIR(cv::String(module), cv::String(name), binary, size, cv::String(buildOptions));
			Ok(new cv::ocl::ProgramSource(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::ProgramSource::fromSPIR(InString, InString, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:712
	// ("cv::ocl::ProgramSource::fromSPIR", vec![(pred!(mut, ["module", "name", "binary", "size"], ["const cv::String*", "const cv::String*", "const unsigned char*", "const size_t"]), _)]),
	void cv_ocl_ProgramSource_fromSPIR_const_StringR_const_StringR_const_unsigned_charX_const_size_t(const char* module, const char* name, const unsigned char* binary, const size_t size, Result<cv::ocl::ProgramSource*>* ocvrs_return) {
		try {
			cv::ocl::ProgramSource ret = cv::ocl::ProgramSource::fromSPIR(cv::String(module), cv::String(name), binary, size);
			Ok(new cv::ocl::ProgramSource(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::ProgramSource::delete() generated
	// ("cv::ocl::ProgramSource::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ocl_ProgramSource_delete(cv::ocl::ProgramSource* instance) {
			delete instance;
	}

	// Queue()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:327
	// ("cv::ocl::Queue::Queue", vec![(pred!(mut, [], []), _)]),
	cv::ocl::Queue* cv_ocl_Queue_Queue() {
			cv::ocl::Queue* ret = new cv::ocl::Queue();
			return ret;
	}

	// Queue(const Context &, const Device &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:328
	// ("cv::ocl::Queue::Queue", vec![(pred!(mut, ["c", "d"], ["const cv::ocl::Context*", "const cv::ocl::Device*"]), _)]),
	void cv_ocl_Queue_Queue_const_ContextR_const_DeviceR(const cv::ocl::Context* c, const cv::ocl::Device* d, Result<cv::ocl::Queue*>* ocvrs_return) {
		try {
			cv::ocl::Queue* ret = new cv::ocl::Queue(*c, *d);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::Queue::Queue(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:328
	// ("cv::ocl::Queue::Queue", vec![(pred!(mut, ["c"], ["const cv::ocl::Context*"]), _)]),
	void cv_ocl_Queue_Queue_const_ContextR(const cv::ocl::Context* c, Result<cv::ocl::Queue*>* ocvrs_return) {
		try {
			cv::ocl::Queue* ret = new cv::ocl::Queue(*c);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Queue(const Queue &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:330
	// ("cv::ocl::Queue::Queue", vec![(pred!(mut, ["q"], ["const cv::ocl::Queue*"]), _)]),
	void cv_ocl_Queue_Queue_const_QueueR(const cv::ocl::Queue* q, Result<cv::ocl::Queue*>* ocvrs_return) {
		try {
			cv::ocl::Queue* ret = new cv::ocl::Queue(*q);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const Queue &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:331
	// ("cv::ocl::Queue::operator=", vec![(pred!(mut, ["q"], ["const cv::ocl::Queue*"]), _)]),
	void cv_ocl_Queue_operatorST_const_QueueR(cv::ocl::Queue* instance, const cv::ocl::Queue* q, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*q);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const Context &, const Device &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:333
	// ("cv::ocl::Queue::create", vec![(pred!(mut, ["c", "d"], ["const cv::ocl::Context*", "const cv::ocl::Device*"]), _)]),
	void cv_ocl_Queue_create_const_ContextR_const_DeviceR(cv::ocl::Queue* instance, const cv::ocl::Context* c, const cv::ocl::Device* d, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->create(*c, *d);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::Queue::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:333
	// ("cv::ocl::Queue::create", vec![(pred!(mut, [], []), _)]),
	void cv_ocl_Queue_create(cv::ocl::Queue* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->create();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// finish()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:334
	// ("cv::ocl::Queue::finish", vec![(pred!(mut, [], []), _)]),
	void cv_ocl_Queue_finish(cv::ocl::Queue* instance, ResultVoid* ocvrs_return) {
		try {
			instance->finish();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ptr()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:335
	// ("cv::ocl::Queue::ptr", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Queue_ptr_const(const cv::ocl::Queue* instance, Result<void*>* ocvrs_return) {
		try {
			void* ret = instance->ptr();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefault()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:336
	// ("cv::ocl::Queue::getDefault", vec![(pred!(mut, [], []), _)]),
	void cv_ocl_Queue_getDefault(Result<cv::ocl::Queue*>* ocvrs_return) {
		try {
			cv::ocl::Queue ret = cv::ocl::Queue::getDefault();
			Ok(new cv::ocl::Queue(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getProfilingQueue()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:339
	// ("cv::ocl::Queue::getProfilingQueue", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Queue_getProfilingQueue_const(const cv::ocl::Queue* instance, Result<cv::ocl::Queue*>* ocvrs_return) {
		try {
			const cv::ocl::Queue ret = instance->getProfilingQueue();
			Ok(new const cv::ocl::Queue(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::Queue::delete() generated
	// ("cv::ocl::Queue::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ocl_Queue_delete(cv::ocl::Queue* instance) {
			delete instance;
	}

	// Timer(const Queue &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:827
	// ("cv::ocl::Timer::Timer", vec![(pred!(mut, ["q"], ["const cv::ocl::Queue*"]), _)]),
	void cv_ocl_Timer_Timer_const_QueueR(const cv::ocl::Queue* q, Result<cv::ocl::Timer*>* ocvrs_return) {
		try {
			cv::ocl::Timer* ret = new cv::ocl::Timer(*q);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// start()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:829
	// ("cv::ocl::Timer::start", vec![(pred!(mut, [], []), _)]),
	void cv_ocl_Timer_start(cv::ocl::Timer* instance, ResultVoid* ocvrs_return) {
		try {
			instance->start();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stop()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:830
	// ("cv::ocl::Timer::stop", vec![(pred!(mut, [], []), _)]),
	void cv_ocl_Timer_stop(cv::ocl::Timer* instance, ResultVoid* ocvrs_return) {
		try {
			instance->stop();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// durationNS()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/ocl.hpp:832
	// ("cv::ocl::Timer::durationNS", vec![(pred!(const, [], []), _)]),
	void cv_ocl_Timer_durationNS_const(const cv::ocl::Timer* instance, Result<uint64_t>* ocvrs_return) {
		try {
			uint64_t ret = instance->durationNS();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ocl::Timer::delete() generated
	// ("cv::ocl::Timer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ocl_Timer_delete(cv::ocl::Timer* instance) {
			delete instance;
	}

	// Arrays()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:411
	// ("cv::ogl::Arrays::Arrays", vec![(pred!(mut, [], []), _)]),
	void cv_ogl_Arrays_Arrays(Result<cv::ogl::Arrays*>* ocvrs_return) {
		try {
			cv::ogl::Arrays* ret = new cv::ogl::Arrays();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setVertexArray(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:416
	// ("cv::ogl::Arrays::setVertexArray", vec![(pred!(mut, ["vertex"], ["const cv::_InputArray*"]), _)]),
	void cv_ogl_Arrays_setVertexArray_const__InputArrayR(cv::ogl::Arrays* instance, const cv::_InputArray* vertex, ResultVoid* ocvrs_return) {
		try {
			instance->setVertexArray(*vertex);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// resetVertexArray()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:420
	// ("cv::ogl::Arrays::resetVertexArray", vec![(pred!(mut, [], []), _)]),
	void cv_ogl_Arrays_resetVertexArray(cv::ogl::Arrays* instance, ResultVoid* ocvrs_return) {
		try {
			instance->resetVertexArray();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setColorArray(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:425
	// ("cv::ogl::Arrays::setColorArray", vec![(pred!(mut, ["color"], ["const cv::_InputArray*"]), _)]),
	void cv_ogl_Arrays_setColorArray_const__InputArrayR(cv::ogl::Arrays* instance, const cv::_InputArray* color, ResultVoid* ocvrs_return) {
		try {
			instance->setColorArray(*color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// resetColorArray()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:429
	// ("cv::ogl::Arrays::resetColorArray", vec![(pred!(mut, [], []), _)]),
	void cv_ogl_Arrays_resetColorArray(cv::ogl::Arrays* instance, ResultVoid* ocvrs_return) {
		try {
			instance->resetColorArray();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNormalArray(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:434
	// ("cv::ogl::Arrays::setNormalArray", vec![(pred!(mut, ["normal"], ["const cv::_InputArray*"]), _)]),
	void cv_ogl_Arrays_setNormalArray_const__InputArrayR(cv::ogl::Arrays* instance, const cv::_InputArray* normal, ResultVoid* ocvrs_return) {
		try {
			instance->setNormalArray(*normal);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// resetNormalArray()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:438
	// ("cv::ogl::Arrays::resetNormalArray", vec![(pred!(mut, [], []), _)]),
	void cv_ogl_Arrays_resetNormalArray(cv::ogl::Arrays* instance, ResultVoid* ocvrs_return) {
		try {
			instance->resetNormalArray();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTexCoordArray(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:443
	// ("cv::ogl::Arrays::setTexCoordArray", vec![(pred!(mut, ["texCoord"], ["const cv::_InputArray*"]), _)]),
	void cv_ogl_Arrays_setTexCoordArray_const__InputArrayR(cv::ogl::Arrays* instance, const cv::_InputArray* texCoord, ResultVoid* ocvrs_return) {
		try {
			instance->setTexCoordArray(*texCoord);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// resetTexCoordArray()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:447
	// ("cv::ogl::Arrays::resetTexCoordArray", vec![(pred!(mut, [], []), _)]),
	void cv_ogl_Arrays_resetTexCoordArray(cv::ogl::Arrays* instance, ResultVoid* ocvrs_return) {
		try {
			instance->resetTexCoordArray();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// release()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:451
	// ("cv::ogl::Arrays::release", vec![(pred!(mut, [], []), _)]),
	void cv_ogl_Arrays_release(cv::ogl::Arrays* instance, ResultVoid* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setAutoRelease(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:456
	// ("cv::ogl::Arrays::setAutoRelease", vec![(pred!(mut, ["flag"], ["bool"]), _)]),
	void cv_ogl_Arrays_setAutoRelease_bool(cv::ogl::Arrays* instance, bool flag, ResultVoid* ocvrs_return) {
		try {
			instance->setAutoRelease(flag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bind()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:460
	// ("cv::ogl::Arrays::bind", vec![(pred!(const, [], []), _)]),
	void cv_ogl_Arrays_bind_const(const cv::ogl::Arrays* instance, ResultVoid* ocvrs_return) {
		try {
			instance->bind();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// size()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:464
	// ("cv::ogl::Arrays::size", vec![(pred!(const, [], []), _)]),
	void cv_ogl_Arrays_size_const(const cv::ogl::Arrays* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->size();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:465
	// ("cv::ogl::Arrays::empty", vec![(pred!(const, [], []), _)]),
	void cv_ogl_Arrays_empty_const(const cv::ogl::Arrays* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ogl::Arrays::delete() generated
	// ("cv::ogl::Arrays::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ogl_Arrays_delete(cv::ogl::Arrays* instance) {
			delete instance;
	}

	// Buffer()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:104
	// ("cv::ogl::Buffer::Buffer", vec![(pred!(mut, [], []), _)]),
	void cv_ogl_Buffer_Buffer(Result<cv::ogl::Buffer*>* ocvrs_return) {
		try {
			cv::ogl::Buffer* ret = new cv::ogl::Buffer();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Buffer(int, int, int, unsigned int, bool)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:113
	// ("cv::ogl::Buffer::Buffer", vec![(pred!(mut, ["arows", "acols", "atype", "abufId", "autoRelease"], ["int", "int", "int", "unsigned int", "bool"]), _)]),
	void cv_ogl_Buffer_Buffer_int_int_int_unsigned_int_bool(int arows, int acols, int atype, unsigned int abufId, bool autoRelease, Result<cv::ogl::Buffer*>* ocvrs_return) {
		try {
			cv::ogl::Buffer* ret = new cv::ogl::Buffer(arows, acols, atype, abufId, autoRelease);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ogl::Buffer::Buffer(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:113
	// ("cv::ogl::Buffer::Buffer", vec![(pred!(mut, ["arows", "acols", "atype", "abufId"], ["int", "int", "int", "unsigned int"]), _)]),
	void cv_ogl_Buffer_Buffer_int_int_int_unsigned_int(int arows, int acols, int atype, unsigned int abufId, Result<cv::ogl::Buffer*>* ocvrs_return) {
		try {
			cv::ogl::Buffer* ret = new cv::ogl::Buffer(arows, acols, atype, abufId);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Buffer(Size, int, unsigned int, bool)(SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:121
	// ("cv::ogl::Buffer::Buffer", vec![(pred!(mut, ["asize", "atype", "abufId", "autoRelease"], ["cv::Size", "int", "unsigned int", "bool"]), _)]),
	void cv_ogl_Buffer_Buffer_Size_int_unsigned_int_bool(cv::Size* asize, int atype, unsigned int abufId, bool autoRelease, Result<cv::ogl::Buffer*>* ocvrs_return) {
		try {
			cv::ogl::Buffer* ret = new cv::ogl::Buffer(*asize, atype, abufId, autoRelease);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ogl::Buffer::Buffer(SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:121
	// ("cv::ogl::Buffer::Buffer", vec![(pred!(mut, ["asize", "atype", "abufId"], ["cv::Size", "int", "unsigned int"]), _)]),
	void cv_ogl_Buffer_Buffer_Size_int_unsigned_int(cv::Size* asize, int atype, unsigned int abufId, Result<cv::ogl::Buffer*>* ocvrs_return) {
		try {
			cv::ogl::Buffer* ret = new cv::ogl::Buffer(*asize, atype, abufId);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Buffer(int, int, int, Target, bool)(Primitive, Primitive, Primitive, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:130
	// ("cv::ogl::Buffer::Buffer", vec![(pred!(mut, ["arows", "acols", "atype", "target", "autoRelease"], ["int", "int", "int", "cv::ogl::Buffer::Target", "bool"]), _)]),
	void cv_ogl_Buffer_Buffer_int_int_int_Target_bool(int arows, int acols, int atype, cv::ogl::Buffer::Target target, bool autoRelease, Result<cv::ogl::Buffer*>* ocvrs_return) {
		try {
			cv::ogl::Buffer* ret = new cv::ogl::Buffer(arows, acols, atype, target, autoRelease);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ogl::Buffer::Buffer(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:130
	// ("cv::ogl::Buffer::Buffer", vec![(pred!(mut, ["arows", "acols", "atype"], ["int", "int", "int"]), _)]),
	void cv_ogl_Buffer_Buffer_int_int_int(int arows, int acols, int atype, Result<cv::ogl::Buffer*>* ocvrs_return) {
		try {
			cv::ogl::Buffer* ret = new cv::ogl::Buffer(arows, acols, atype);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Buffer(Size, int, Target, bool)(SimpleClass, Primitive, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:138
	// ("cv::ogl::Buffer::Buffer", vec![(pred!(mut, ["asize", "atype", "target", "autoRelease"], ["cv::Size", "int", "cv::ogl::Buffer::Target", "bool"]), _)]),
	void cv_ogl_Buffer_Buffer_Size_int_Target_bool(cv::Size* asize, int atype, cv::ogl::Buffer::Target target, bool autoRelease, Result<cv::ogl::Buffer*>* ocvrs_return) {
		try {
			cv::ogl::Buffer* ret = new cv::ogl::Buffer(*asize, atype, target, autoRelease);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ogl::Buffer::Buffer(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:138
	// ("cv::ogl::Buffer::Buffer", vec![(pred!(mut, ["asize", "atype"], ["cv::Size", "int"]), _)]),
	void cv_ogl_Buffer_Buffer_Size_int(cv::Size* asize, int atype, Result<cv::ogl::Buffer*>* ocvrs_return) {
		try {
			cv::ogl::Buffer* ret = new cv::ogl::Buffer(*asize, atype);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Buffer(InputArray, Target, bool)(InputArray, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:145
	// ("cv::ogl::Buffer::Buffer", vec![(pred!(mut, ["arr", "target", "autoRelease"], ["const cv::_InputArray*", "cv::ogl::Buffer::Target", "bool"]), _)]),
	void cv_ogl_Buffer_Buffer_const__InputArrayR_Target_bool(const cv::_InputArray* arr, cv::ogl::Buffer::Target target, bool autoRelease, Result<cv::ogl::Buffer*>* ocvrs_return) {
		try {
			cv::ogl::Buffer* ret = new cv::ogl::Buffer(*arr, target, autoRelease);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ogl::Buffer::Buffer(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:145
	// ("cv::ogl::Buffer::Buffer", vec![(pred!(mut, ["arr"], ["const cv::_InputArray*"]), _)]),
	void cv_ogl_Buffer_Buffer_const__InputArrayR(const cv::_InputArray* arr, Result<cv::ogl::Buffer*>* ocvrs_return) {
		try {
			cv::ogl::Buffer* ret = new cv::ogl::Buffer(*arr);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, int, int, Target, bool)(Primitive, Primitive, Primitive, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:155
	// ("cv::ogl::Buffer::create", vec![(pred!(mut, ["arows", "acols", "atype", "target", "autoRelease"], ["int", "int", "int", "cv::ogl::Buffer::Target", "bool"]), _)]),
	void cv_ogl_Buffer_create_int_int_int_Target_bool(cv::ogl::Buffer* instance, int arows, int acols, int atype, cv::ogl::Buffer::Target target, bool autoRelease, ResultVoid* ocvrs_return) {
		try {
			instance->create(arows, acols, atype, target, autoRelease);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ogl::Buffer::create(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:155
	// ("cv::ogl::Buffer::create", vec![(pred!(mut, ["arows", "acols", "atype"], ["int", "int", "int"]), _)]),
	void cv_ogl_Buffer_create_int_int_int(cv::ogl::Buffer* instance, int arows, int acols, int atype, ResultVoid* ocvrs_return) {
		try {
			instance->create(arows, acols, atype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(Size, int, Target, bool)(SimpleClass, Primitive, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:163
	// ("cv::ogl::Buffer::create", vec![(pred!(mut, ["asize", "atype", "target", "autoRelease"], ["cv::Size", "int", "cv::ogl::Buffer::Target", "bool"]), _)]),
	void cv_ogl_Buffer_create_Size_int_Target_bool(cv::ogl::Buffer* instance, cv::Size* asize, int atype, cv::ogl::Buffer::Target target, bool autoRelease, ResultVoid* ocvrs_return) {
		try {
			instance->create(*asize, atype, target, autoRelease);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ogl::Buffer::create(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:163
	// ("cv::ogl::Buffer::create", vec![(pred!(mut, ["asize", "atype"], ["cv::Size", "int"]), _)]),
	void cv_ogl_Buffer_create_Size_int(cv::ogl::Buffer* instance, cv::Size* asize, int atype, ResultVoid* ocvrs_return) {
		try {
			instance->create(*asize, atype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// release()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:169
	// ("cv::ogl::Buffer::release", vec![(pred!(mut, [], []), _)]),
	void cv_ogl_Buffer_release(cv::ogl::Buffer* instance, ResultVoid* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setAutoRelease(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:180
	// ("cv::ogl::Buffer::setAutoRelease", vec![(pred!(mut, ["flag"], ["bool"]), _)]),
	void cv_ogl_Buffer_setAutoRelease_bool(cv::ogl::Buffer* instance, bool flag, ResultVoid* ocvrs_return) {
		try {
			instance->setAutoRelease(flag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// copyFrom(InputArray, Target, bool)(InputArray, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:187
	// ("cv::ogl::Buffer::copyFrom", vec![(pred!(mut, ["arr", "target", "autoRelease"], ["const cv::_InputArray*", "cv::ogl::Buffer::Target", "bool"]), _)]),
	void cv_ogl_Buffer_copyFrom_const__InputArrayR_Target_bool(cv::ogl::Buffer* instance, const cv::_InputArray* arr, cv::ogl::Buffer::Target target, bool autoRelease, ResultVoid* ocvrs_return) {
		try {
			instance->copyFrom(*arr, target, autoRelease);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ogl::Buffer::copyFrom(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:187
	// ("cv::ogl::Buffer::copyFrom", vec![(pred!(mut, ["arr"], ["const cv::_InputArray*"]), _)]),
	void cv_ogl_Buffer_copyFrom_const__InputArrayR(cv::ogl::Buffer* instance, const cv::_InputArray* arr, ResultVoid* ocvrs_return) {
		try {
			instance->copyFrom(*arr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// copyFrom(InputArray, cuda::Stream &, Target, bool)(InputArray, TraitClass, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:190
	// ("cv::ogl::Buffer::copyFrom", vec![(pred!(mut, ["arr", "stream", "target", "autoRelease"], ["const cv::_InputArray*", "cv::cuda::Stream*", "cv::ogl::Buffer::Target", "bool"]), _)]),
	void cv_ogl_Buffer_copyFrom_const__InputArrayR_StreamR_Target_bool(cv::ogl::Buffer* instance, const cv::_InputArray* arr, cv::cuda::Stream* stream, cv::ogl::Buffer::Target target, bool autoRelease, ResultVoid* ocvrs_return) {
		try {
			instance->copyFrom(*arr, *stream, target, autoRelease);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ogl::Buffer::copyFrom(InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:190
	// ("cv::ogl::Buffer::copyFrom", vec![(pred!(mut, ["arr", "stream"], ["const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_ogl_Buffer_copyFrom_const__InputArrayR_StreamR(cv::ogl::Buffer* instance, const cv::_InputArray* arr, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->copyFrom(*arr, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// copyTo(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:197
	// ("cv::ogl::Buffer::copyTo", vec![(pred!(const, ["arr"], ["const cv::_OutputArray*"]), _)]),
	void cv_ogl_Buffer_copyTo_const_const__OutputArrayR(const cv::ogl::Buffer* instance, const cv::_OutputArray* arr, ResultVoid* ocvrs_return) {
		try {
			instance->copyTo(*arr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// copyTo(OutputArray, cuda::Stream &)(OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:200
	// ("cv::ogl::Buffer::copyTo", vec![(pred!(const, ["arr", "stream"], ["const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_ogl_Buffer_copyTo_const_const__OutputArrayR_StreamR(const cv::ogl::Buffer* instance, const cv::_OutputArray* arr, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->copyTo(*arr, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clone(Target, bool)(Enum, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:207
	// ("cv::ogl::Buffer::clone", vec![(pred!(const, ["target", "autoRelease"], ["cv::ogl::Buffer::Target", "bool"]), _)]),
	void cv_ogl_Buffer_clone_const_Target_bool(const cv::ogl::Buffer* instance, cv::ogl::Buffer::Target target, bool autoRelease, Result<cv::ogl::Buffer*>* ocvrs_return) {
		try {
			cv::ogl::Buffer ret = instance->clone(target, autoRelease);
			Ok(new cv::ogl::Buffer(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ogl::Buffer::clone() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:207
	// ("cv::ogl::Buffer::clone", vec![(pred!(const, [], []), _)]),
	void cv_ogl_Buffer_clone_const(const cv::ogl::Buffer* instance, Result<cv::ogl::Buffer*>* ocvrs_return) {
		try {
			cv::ogl::Buffer ret = instance->clone();
			Ok(new cv::ogl::Buffer(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bind(Target)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:213
	// ("cv::ogl::Buffer::bind", vec![(pred!(const, ["target"], ["cv::ogl::Buffer::Target"]), _)]),
	void cv_ogl_Buffer_bind_const_Target(const cv::ogl::Buffer* instance, cv::ogl::Buffer::Target target, ResultVoid* ocvrs_return) {
		try {
			instance->bind(target);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// unbind(Target)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:219
	// ("cv::ogl::Buffer::unbind", vec![(pred!(mut, ["target"], ["cv::ogl::Buffer::Target"]), _)]),
	void cv_ogl_Buffer_unbind_Target(cv::ogl::Buffer::Target target, ResultVoid* ocvrs_return) {
		try {
			cv::ogl::Buffer::unbind(target);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mapHost(Access)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:236
	// ("cv::ogl::Buffer::mapHost", vec![(pred!(mut, ["access"], ["cv::ogl::Buffer::Access"]), _)]),
	void cv_ogl_Buffer_mapHost_Access(cv::ogl::Buffer* instance, cv::ogl::Buffer::Access access, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->mapHost(access);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// unmapHost()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:240
	// ("cv::ogl::Buffer::unmapHost", vec![(pred!(mut, [], []), _)]),
	void cv_ogl_Buffer_unmapHost(cv::ogl::Buffer* instance, ResultVoid* ocvrs_return) {
		try {
			instance->unmapHost();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mapDevice()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:243
	// ("cv::ogl::Buffer::mapDevice", vec![(pred!(mut, [], []), _)]),
	void cv_ogl_Buffer_mapDevice(cv::ogl::Buffer* instance, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->mapDevice();
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// unmapDevice()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:244
	// ("cv::ogl::Buffer::unmapDevice", vec![(pred!(mut, [], []), _)]),
	void cv_ogl_Buffer_unmapDevice(cv::ogl::Buffer* instance, ResultVoid* ocvrs_return) {
		try {
			instance->unmapDevice();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mapDevice(cuda::Stream &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:252
	// ("cv::ogl::Buffer::mapDevice", vec![(pred!(mut, ["stream"], ["cv::cuda::Stream*"]), _)]),
	void cv_ogl_Buffer_mapDevice_StreamR(cv::ogl::Buffer* instance, cv::cuda::Stream* stream, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->mapDevice(*stream);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// unmapDevice(cuda::Stream &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:256
	// ("cv::ogl::Buffer::unmapDevice", vec![(pred!(mut, ["stream"], ["cv::cuda::Stream*"]), _)]),
	void cv_ogl_Buffer_unmapDevice_StreamR(cv::ogl::Buffer* instance, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->unmapDevice(*stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// rows()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:258
	// ("cv::ogl::Buffer::rows", vec![(pred!(const, [], []), _)]),
	void cv_ogl_Buffer_rows_const(const cv::ogl::Buffer* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->rows();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cols()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:259
	// ("cv::ogl::Buffer::cols", vec![(pred!(const, [], []), _)]),
	void cv_ogl_Buffer_cols_const(const cv::ogl::Buffer* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->cols();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// size()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:260
	// ("cv::ogl::Buffer::size", vec![(pred!(const, [], []), _)]),
	void cv_ogl_Buffer_size_const(const cv::ogl::Buffer* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->size();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:261
	// ("cv::ogl::Buffer::empty", vec![(pred!(const, [], []), _)]),
	void cv_ogl_Buffer_empty_const(const cv::ogl::Buffer* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// type()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:263
	// ("cv::ogl::Buffer::type", vec![(pred!(const, [], []), _)]),
	void cv_ogl_Buffer_type_const(const cv::ogl::Buffer* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->type();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// depth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:264
	// ("cv::ogl::Buffer::depth", vec![(pred!(const, [], []), _)]),
	void cv_ogl_Buffer_depth_const(const cv::ogl::Buffer* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->depth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// channels()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:265
	// ("cv::ogl::Buffer::channels", vec![(pred!(const, [], []), _)]),
	void cv_ogl_Buffer_channels_const(const cv::ogl::Buffer* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->channels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// elemSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:266
	// ("cv::ogl::Buffer::elemSize", vec![(pred!(const, [], []), _)]),
	void cv_ogl_Buffer_elemSize_const(const cv::ogl::Buffer* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->elemSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// elemSize1()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:267
	// ("cv::ogl::Buffer::elemSize1", vec![(pred!(const, [], []), _)]),
	void cv_ogl_Buffer_elemSize1_const(const cv::ogl::Buffer* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->elemSize1();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bufId()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:270
	// ("cv::ogl::Buffer::bufId", vec![(pred!(const, [], []), _)]),
	void cv_ogl_Buffer_bufId_const(const cv::ogl::Buffer* instance, Result<unsigned int>* ocvrs_return) {
		try {
			unsigned int ret = instance->bufId();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ogl::Buffer::delete() generated
	// ("cv::ogl::Buffer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ogl_Buffer_delete(cv::ogl::Buffer* instance) {
			delete instance;
	}

	// Texture2D()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:301
	// ("cv::ogl::Texture2D::Texture2D", vec![(pred!(mut, [], []), _)]),
	void cv_ogl_Texture2D_Texture2D(Result<cv::ogl::Texture2D*>* ocvrs_return) {
		try {
			cv::ogl::Texture2D* ret = new cv::ogl::Texture2D();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Texture2D(int, int, Format, unsigned int, bool)(Primitive, Primitive, Enum, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:304
	// ("cv::ogl::Texture2D::Texture2D", vec![(pred!(mut, ["arows", "acols", "aformat", "atexId", "autoRelease"], ["int", "int", "cv::ogl::Texture2D::Format", "unsigned int", "bool"]), _)]),
	void cv_ogl_Texture2D_Texture2D_int_int_Format_unsigned_int_bool(int arows, int acols, cv::ogl::Texture2D::Format aformat, unsigned int atexId, bool autoRelease, Result<cv::ogl::Texture2D*>* ocvrs_return) {
		try {
			cv::ogl::Texture2D* ret = new cv::ogl::Texture2D(arows, acols, aformat, atexId, autoRelease);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ogl::Texture2D::Texture2D(Primitive, Primitive, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:304
	// ("cv::ogl::Texture2D::Texture2D", vec![(pred!(mut, ["arows", "acols", "aformat", "atexId"], ["int", "int", "cv::ogl::Texture2D::Format", "unsigned int"]), _)]),
	void cv_ogl_Texture2D_Texture2D_int_int_Format_unsigned_int(int arows, int acols, cv::ogl::Texture2D::Format aformat, unsigned int atexId, Result<cv::ogl::Texture2D*>* ocvrs_return) {
		try {
			cv::ogl::Texture2D* ret = new cv::ogl::Texture2D(arows, acols, aformat, atexId);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Texture2D(Size, Format, unsigned int, bool)(SimpleClass, Enum, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:307
	// ("cv::ogl::Texture2D::Texture2D", vec![(pred!(mut, ["asize", "aformat", "atexId", "autoRelease"], ["cv::Size", "cv::ogl::Texture2D::Format", "unsigned int", "bool"]), _)]),
	void cv_ogl_Texture2D_Texture2D_Size_Format_unsigned_int_bool(cv::Size* asize, cv::ogl::Texture2D::Format aformat, unsigned int atexId, bool autoRelease, Result<cv::ogl::Texture2D*>* ocvrs_return) {
		try {
			cv::ogl::Texture2D* ret = new cv::ogl::Texture2D(*asize, aformat, atexId, autoRelease);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ogl::Texture2D::Texture2D(SimpleClass, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:307
	// ("cv::ogl::Texture2D::Texture2D", vec![(pred!(mut, ["asize", "aformat", "atexId"], ["cv::Size", "cv::ogl::Texture2D::Format", "unsigned int"]), _)]),
	void cv_ogl_Texture2D_Texture2D_Size_Format_unsigned_int(cv::Size* asize, cv::ogl::Texture2D::Format aformat, unsigned int atexId, Result<cv::ogl::Texture2D*>* ocvrs_return) {
		try {
			cv::ogl::Texture2D* ret = new cv::ogl::Texture2D(*asize, aformat, atexId);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Texture2D(int, int, Format, bool)(Primitive, Primitive, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:315
	// ("cv::ogl::Texture2D::Texture2D", vec![(pred!(mut, ["arows", "acols", "aformat", "autoRelease"], ["int", "int", "cv::ogl::Texture2D::Format", "bool"]), _)]),
	void cv_ogl_Texture2D_Texture2D_int_int_Format_bool(int arows, int acols, cv::ogl::Texture2D::Format aformat, bool autoRelease, Result<cv::ogl::Texture2D*>* ocvrs_return) {
		try {
			cv::ogl::Texture2D* ret = new cv::ogl::Texture2D(arows, acols, aformat, autoRelease);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ogl::Texture2D::Texture2D(Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:315
	// ("cv::ogl::Texture2D::Texture2D", vec![(pred!(mut, ["arows", "acols", "aformat"], ["int", "int", "cv::ogl::Texture2D::Format"]), _)]),
	void cv_ogl_Texture2D_Texture2D_int_int_Format(int arows, int acols, cv::ogl::Texture2D::Format aformat, Result<cv::ogl::Texture2D*>* ocvrs_return) {
		try {
			cv::ogl::Texture2D* ret = new cv::ogl::Texture2D(arows, acols, aformat);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Texture2D(Size, Format, bool)(SimpleClass, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:322
	// ("cv::ogl::Texture2D::Texture2D", vec![(pred!(mut, ["asize", "aformat", "autoRelease"], ["cv::Size", "cv::ogl::Texture2D::Format", "bool"]), _)]),
	void cv_ogl_Texture2D_Texture2D_Size_Format_bool(cv::Size* asize, cv::ogl::Texture2D::Format aformat, bool autoRelease, Result<cv::ogl::Texture2D*>* ocvrs_return) {
		try {
			cv::ogl::Texture2D* ret = new cv::ogl::Texture2D(*asize, aformat, autoRelease);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ogl::Texture2D::Texture2D(SimpleClass, Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:322
	// ("cv::ogl::Texture2D::Texture2D", vec![(pred!(mut, ["asize", "aformat"], ["cv::Size", "cv::ogl::Texture2D::Format"]), _)]),
	void cv_ogl_Texture2D_Texture2D_Size_Format(cv::Size* asize, cv::ogl::Texture2D::Format aformat, Result<cv::ogl::Texture2D*>* ocvrs_return) {
		try {
			cv::ogl::Texture2D* ret = new cv::ogl::Texture2D(*asize, aformat);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Texture2D(InputArray, bool)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:328
	// ("cv::ogl::Texture2D::Texture2D", vec![(pred!(mut, ["arr", "autoRelease"], ["const cv::_InputArray*", "bool"]), _)]),
	void cv_ogl_Texture2D_Texture2D_const__InputArrayR_bool(const cv::_InputArray* arr, bool autoRelease, Result<cv::ogl::Texture2D*>* ocvrs_return) {
		try {
			cv::ogl::Texture2D* ret = new cv::ogl::Texture2D(*arr, autoRelease);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ogl::Texture2D::Texture2D(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:328
	// ("cv::ogl::Texture2D::Texture2D", vec![(pred!(mut, ["arr"], ["const cv::_InputArray*"]), _)]),
	void cv_ogl_Texture2D_Texture2D_const__InputArrayR(const cv::_InputArray* arr, Result<cv::ogl::Texture2D*>* ocvrs_return) {
		try {
			cv::ogl::Texture2D* ret = new cv::ogl::Texture2D(*arr);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, int, Format, bool)(Primitive, Primitive, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:337
	// ("cv::ogl::Texture2D::create", vec![(pred!(mut, ["arows", "acols", "aformat", "autoRelease"], ["int", "int", "cv::ogl::Texture2D::Format", "bool"]), _)]),
	void cv_ogl_Texture2D_create_int_int_Format_bool(cv::ogl::Texture2D* instance, int arows, int acols, cv::ogl::Texture2D::Format aformat, bool autoRelease, ResultVoid* ocvrs_return) {
		try {
			instance->create(arows, acols, aformat, autoRelease);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ogl::Texture2D::create(Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:337
	// ("cv::ogl::Texture2D::create", vec![(pred!(mut, ["arows", "acols", "aformat"], ["int", "int", "cv::ogl::Texture2D::Format"]), _)]),
	void cv_ogl_Texture2D_create_int_int_Format(cv::ogl::Texture2D* instance, int arows, int acols, cv::ogl::Texture2D::Format aformat, ResultVoid* ocvrs_return) {
		try {
			instance->create(arows, acols, aformat);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(Size, Format, bool)(SimpleClass, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:343
	// ("cv::ogl::Texture2D::create", vec![(pred!(mut, ["asize", "aformat", "autoRelease"], ["cv::Size", "cv::ogl::Texture2D::Format", "bool"]), _)]),
	void cv_ogl_Texture2D_create_Size_Format_bool(cv::ogl::Texture2D* instance, cv::Size* asize, cv::ogl::Texture2D::Format aformat, bool autoRelease, ResultVoid* ocvrs_return) {
		try {
			instance->create(*asize, aformat, autoRelease);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ogl::Texture2D::create(SimpleClass, Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:343
	// ("cv::ogl::Texture2D::create", vec![(pred!(mut, ["asize", "aformat"], ["cv::Size", "cv::ogl::Texture2D::Format"]), _)]),
	void cv_ogl_Texture2D_create_Size_Format(cv::ogl::Texture2D* instance, cv::Size* asize, cv::ogl::Texture2D::Format aformat, ResultVoid* ocvrs_return) {
		try {
			instance->create(*asize, aformat);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// release()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:349
	// ("cv::ogl::Texture2D::release", vec![(pred!(mut, [], []), _)]),
	void cv_ogl_Texture2D_release(cv::ogl::Texture2D* instance, ResultVoid* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setAutoRelease(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:361
	// ("cv::ogl::Texture2D::setAutoRelease", vec![(pred!(mut, ["flag"], ["bool"]), _)]),
	void cv_ogl_Texture2D_setAutoRelease_bool(cv::ogl::Texture2D* instance, bool flag, ResultVoid* ocvrs_return) {
		try {
			instance->setAutoRelease(flag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// copyFrom(InputArray, bool)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:368
	// ("cv::ogl::Texture2D::copyFrom", vec![(pred!(mut, ["arr", "autoRelease"], ["const cv::_InputArray*", "bool"]), _)]),
	void cv_ogl_Texture2D_copyFrom_const__InputArrayR_bool(cv::ogl::Texture2D* instance, const cv::_InputArray* arr, bool autoRelease, ResultVoid* ocvrs_return) {
		try {
			instance->copyFrom(*arr, autoRelease);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ogl::Texture2D::copyFrom(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:368
	// ("cv::ogl::Texture2D::copyFrom", vec![(pred!(mut, ["arr"], ["const cv::_InputArray*"]), _)]),
	void cv_ogl_Texture2D_copyFrom_const__InputArrayR(cv::ogl::Texture2D* instance, const cv::_InputArray* arr, ResultVoid* ocvrs_return) {
		try {
			instance->copyFrom(*arr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// copyTo(OutputArray, int, bool)(OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:377
	// ("cv::ogl::Texture2D::copyTo", vec![(pred!(const, ["arr", "ddepth", "autoRelease"], ["const cv::_OutputArray*", "int", "bool"]), _)]),
	void cv_ogl_Texture2D_copyTo_const_const__OutputArrayR_int_bool(const cv::ogl::Texture2D* instance, const cv::_OutputArray* arr, int ddepth, bool autoRelease, ResultVoid* ocvrs_return) {
		try {
			instance->copyTo(*arr, ddepth, autoRelease);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ogl::Texture2D::copyTo(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:377
	// ("cv::ogl::Texture2D::copyTo", vec![(pred!(const, ["arr"], ["const cv::_OutputArray*"]), _)]),
	void cv_ogl_Texture2D_copyTo_const_const__OutputArrayR(const cv::ogl::Texture2D* instance, const cv::_OutputArray* arr, ResultVoid* ocvrs_return) {
		try {
			instance->copyTo(*arr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bind()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:381
	// ("cv::ogl::Texture2D::bind", vec![(pred!(const, [], []), _)]),
	void cv_ogl_Texture2D_bind_const(const cv::ogl::Texture2D* instance, ResultVoid* ocvrs_return) {
		try {
			instance->bind();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// rows()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:383
	// ("cv::ogl::Texture2D::rows", vec![(pred!(const, [], []), _)]),
	void cv_ogl_Texture2D_rows_const(const cv::ogl::Texture2D* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->rows();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cols()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:384
	// ("cv::ogl::Texture2D::cols", vec![(pred!(const, [], []), _)]),
	void cv_ogl_Texture2D_cols_const(const cv::ogl::Texture2D* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->cols();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// size()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:385
	// ("cv::ogl::Texture2D::size", vec![(pred!(const, [], []), _)]),
	void cv_ogl_Texture2D_size_const(const cv::ogl::Texture2D* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->size();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:386
	// ("cv::ogl::Texture2D::empty", vec![(pred!(const, [], []), _)]),
	void cv_ogl_Texture2D_empty_const(const cv::ogl::Texture2D* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// format()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:388
	// ("cv::ogl::Texture2D::format", vec![(pred!(const, [], []), _)]),
	void cv_ogl_Texture2D_format_const(const cv::ogl::Texture2D* instance, Result<cv::ogl::Texture2D::Format>* ocvrs_return) {
		try {
			cv::ogl::Texture2D::Format ret = instance->format();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// texId()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/opengl.hpp:391
	// ("cv::ogl::Texture2D::texId", vec![(pred!(const, [], []), _)]),
	void cv_ogl_Texture2D_texId_const(const cv::ogl::Texture2D* instance, Result<unsigned int>* ocvrs_return) {
		try {
			unsigned int ret = instance->texId();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ogl::Texture2D::delete() generated
	// ("cv::ogl::Texture2D::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ogl_Texture2D_delete(cv::ogl::Texture2D* instance) {
			delete instance;
	}

	// ClassWithKeywordProperties(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:220
	// ("cv::utils::ClassWithKeywordProperties::ClassWithKeywordProperties", vec![(pred!(mut, ["lambda_arg", "except_arg"], ["int", "int"]), _)]),
	void cv_utils_ClassWithKeywordProperties_ClassWithKeywordProperties_int_int(int lambda_arg, int except_arg, Result<cv::utils::ClassWithKeywordProperties>* ocvrs_return) {
		try {
			cv::utils::ClassWithKeywordProperties ret(lambda_arg, except_arg);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::utils::ClassWithKeywordProperties::ClassWithKeywordProperties() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:220
	// ("cv::utils::ClassWithKeywordProperties::ClassWithKeywordProperties", vec![(pred!(mut, [], []), _)]),
	void cv_utils_ClassWithKeywordProperties_ClassWithKeywordProperties(Result<cv::utils::ClassWithKeywordProperties>* ocvrs_return) {
		try {
			cv::utils::ClassWithKeywordProperties ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// OriginalClassName(const OriginalClassName::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:247
	// ("cv::utils::nested::OriginalClassName::OriginalClassName", vec![(pred!(mut, ["params"], ["const cv::utils::nested::OriginalClassName::Params*"]), _)]),
	void cv_utils_nested_OriginalClassName_OriginalClassName_const_ParamsR(const cv::utils::nested::OriginalClassName::Params* params, Result<cv::utils::nested::OriginalClassName*>* ocvrs_return) {
		try {
			cv::utils::nested::OriginalClassName* ret = new cv::utils::nested::OriginalClassName(*params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::utils::nested::OriginalClassName::OriginalClassName() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:247
	// ("cv::utils::nested::OriginalClassName::OriginalClassName", vec![(pred!(mut, [], []), _)]),
	void cv_utils_nested_OriginalClassName_OriginalClassName(Result<cv::utils::nested::OriginalClassName*>* ocvrs_return) {
		try {
			cv::utils::nested::OriginalClassName* ret = new cv::utils::nested::OriginalClassName();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getIntParam()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:252
	// ("cv::utils::nested::OriginalClassName::getIntParam", vec![(pred!(const, [], []), _)]),
	void cv_utils_nested_OriginalClassName_getIntParam_const(const cv::utils::nested::OriginalClassName* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getIntParam();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFloatParam()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:257
	// ("cv::utils::nested::OriginalClassName::getFloatParam", vec![(pred!(const, [], []), _)]),
	void cv_utils_nested_OriginalClassName_getFloatParam_const(const cv::utils::nested::OriginalClassName* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getFloatParam();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// originalName()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:262
	// ("cv::utils::nested::OriginalClassName::originalName", vec![(pred!(mut, [], []), _)]),
	void cv_utils_nested_OriginalClassName_originalName(Result<void*>* ocvrs_return) {
		try {
			std::string ret = cv::utils::nested::OriginalClassName::originalName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const OriginalClassName::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:268
	// ("cv::utils::nested::OriginalClassName::create", vec![(pred!(mut, ["params"], ["const cv::utils::nested::OriginalClassName::Params*"]), _)]),
	void cv_utils_nested_OriginalClassName_create_const_ParamsR(const cv::utils::nested::OriginalClassName::Params* params, Result<cv::Ptr<cv::utils::nested::OriginalClassName>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::utils::nested::OriginalClassName> ret = cv::utils::nested::OriginalClassName::create(*params);
			Ok(new cv::Ptr<cv::utils::nested::OriginalClassName>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::utils::nested::OriginalClassName::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:268
	// ("cv::utils::nested::OriginalClassName::create", vec![(pred!(mut, [], []), _)]),
	void cv_utils_nested_OriginalClassName_create(Result<cv::Ptr<cv::utils::nested::OriginalClassName>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::utils::nested::OriginalClassName> ret = cv::utils::nested::OriginalClassName::create();
			Ok(new cv::Ptr<cv::utils::nested::OriginalClassName>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::utils::nested::OriginalClassName::delete() generated
	// ("cv::utils::nested::OriginalClassName::delete", vec![(pred!(mut, [], []), _)]),
	void cv_utils_nested_OriginalClassName_delete(cv::utils::nested::OriginalClassName* instance) {
			delete instance;
	}

	// Params(int, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:240
	// ("cv::utils::nested::OriginalClassName::Params::Params", vec![(pred!(mut, ["int_param", "float_param"], ["int", "float"]), _)]),
	void cv_utils_nested_OriginalClassName_Params_Params_int_float(int int_param, float float_param, Result<cv::utils::nested::OriginalClassName::Params>* ocvrs_return) {
		try {
			cv::utils::nested::OriginalClassName::Params ret(int_param, float_param);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::utils::nested::OriginalClassName::Params::Params() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/core/bindings_utils.hpp:240
	// ("cv::utils::nested::OriginalClassName::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_utils_nested_OriginalClassName_Params_Params(Result<cv::utils::nested::OriginalClassName::Params>* ocvrs_return) {
		try {
			cv::utils::nested::OriginalClassName::Params ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}
