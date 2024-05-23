#include "ocvrs_common.hpp"
#include <opencv2/cudaarithm.hpp>
#include "cudaarithm_types.hpp"

extern "C" {
	// cv::cuda::absSum(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:852
	// ("cv::cuda::absSum", vec![(pred!(mut, ["src"], ["const cv::_InputArray*"]), _)]),
	void cv_cuda_absSum_const__InputArrayR(const cv::_InputArray* src, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = cv::cuda::absSum(*src);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// absSum(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:852
	// ("cv::cuda::absSum", vec![(pred!(mut, ["src", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_cuda_absSum_const__InputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_InputArray* mask, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = cv::cuda::absSum(*src, *mask);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::abs(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:243
	// ("cv::cuda::abs", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_abs_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::abs(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// abs(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:243
	// ("cv::cuda::abs", vec![(pred!(mut, ["src", "dst", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_abs_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::abs(*src, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::absdiffWithScalar(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:231
	// ("cv::cuda::absdiffWithScalar", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_absdiffWithScalar_const__InputArrayR_Scalar_const__OutputArrayR(const cv::_InputArray* src1, cv::Scalar* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::absdiffWithScalar(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// absdiffWithScalar(InputArray, Scalar, OutputArray, Stream &)(InputArray, SimpleClass, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:231
	// ("cv::cuda::absdiffWithScalar", vec![(pred!(mut, ["src1", "src2", "dst", "stream"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_absdiffWithScalar_const__InputArrayR_Scalar_const__OutputArrayR_StreamR(const cv::_InputArray* src1, cv::Scalar* src2, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::absdiffWithScalar(*src1, *src2, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::absdiff(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:220
	// ("cv::cuda::absdiff", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_absdiff_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::absdiff(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// absdiff(InputArray, InputArray, OutputArray, Stream &)(InputArray, InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:220
	// ("cv::cuda::absdiff", vec![(pred!(mut, ["src1", "src2", "dst", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_absdiff_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::absdiff(*src1, *src2, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::addWeighted(InputArray, Primitive, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:538
	// ("cv::cuda::addWeighted", vec![(pred!(mut, ["src1", "alpha", "src2", "beta", "gamma", "dst"], ["const cv::_InputArray*", "double", "const cv::_InputArray*", "double", "double", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_addWeighted_const__InputArrayR_double_const__InputArrayR_double_double_const__OutputArrayR(const cv::_InputArray* src1, double alpha, const cv::_InputArray* src2, double beta, double gamma, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::addWeighted(*src1, alpha, *src2, beta, gamma, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addWeighted(InputArray, double, InputArray, double, double, OutputArray, int, Stream &)(InputArray, Primitive, InputArray, Primitive, Primitive, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:538
	// ("cv::cuda::addWeighted", vec![(pred!(mut, ["src1", "alpha", "src2", "beta", "gamma", "dst", "dtype", "stream"], ["const cv::_InputArray*", "double", "const cv::_InputArray*", "double", "double", "const cv::_OutputArray*", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_addWeighted_const__InputArrayR_double_const__InputArrayR_double_double_const__OutputArrayR_int_StreamR(const cv::_InputArray* src1, double alpha, const cv::_InputArray* src2, double beta, double gamma, const cv::_OutputArray* dst, int dtype, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::addWeighted(*src1, alpha, *src2, beta, gamma, *dst, dtype, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::addWithScalar(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:103
	// ("cv::cuda::addWithScalar", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_addWithScalar_const__InputArrayR_Scalar_const__OutputArrayR(const cv::_InputArray* src1, cv::Scalar* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::addWithScalar(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addWithScalar(InputArray, Scalar, OutputArray, InputArray, int, Stream &)(InputArray, SimpleClass, OutputArray, InputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:103
	// ("cv::cuda::addWithScalar", vec![(pred!(mut, ["src1", "src2", "dst", "mask", "dtype", "stream"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*", "const cv::_InputArray*", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_addWithScalar_const__InputArrayR_Scalar_const__OutputArrayR_const__InputArrayR_int_StreamR(const cv::_InputArray* src1, cv::Scalar* src2, const cv::_OutputArray* dst, const cv::_InputArray* mask, int dtype, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::addWithScalar(*src1, *src2, *dst, *mask, dtype, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::add(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:88
	// ("cv::cuda::add", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_add_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::add(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// add(InputArray, InputArray, OutputArray, InputArray, int, Stream &)(InputArray, InputArray, OutputArray, InputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:88
	// ("cv::cuda::add", vec![(pred!(mut, ["src1", "src2", "dst", "mask", "dtype", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_add_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_int_StreamR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, const cv::_InputArray* mask, int dtype, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::add(*src1, *src2, *dst, *mask, dtype, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::bitwise_and(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:391
	// ("cv::cuda::bitwise_and", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_bitwise_and_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::bitwise_and(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bitwise_and(InputArray, InputArray, OutputArray, InputArray, Stream &)(InputArray, InputArray, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:391
	// ("cv::cuda::bitwise_and", vec![(pred!(mut, ["src1", "src2", "dst", "mask", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_bitwise_and_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, const cv::_InputArray* mask, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::bitwise_and(*src1, *src2, *dst, *mask, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::bitwise_and_with_scalar(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:404
	// ("cv::cuda::bitwise_and_with_scalar", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_bitwise_and_with_scalar_const__InputArrayR_Scalar_const__OutputArrayR(const cv::_InputArray* src1, cv::Scalar* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::bitwise_and_with_scalar(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bitwise_and_with_scalar(InputArray, Scalar, OutputArray, InputArray, Stream &)(InputArray, SimpleClass, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:404
	// ("cv::cuda::bitwise_and_with_scalar", vec![(pred!(mut, ["src1", "src2", "dst", "mask", "stream"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_bitwise_and_with_scalar_const__InputArrayR_Scalar_const__OutputArrayR_const__InputArrayR_StreamR(const cv::_InputArray* src1, cv::Scalar* src2, const cv::_OutputArray* dst, const cv::_InputArray* mask, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::bitwise_and_with_scalar(*src1, *src2, *dst, *mask, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::bitwise_not(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:346
	// ("cv::cuda::bitwise_not", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_bitwise_not_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::bitwise_not(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bitwise_not(InputArray, OutputArray, InputArray, Stream &)(InputArray, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:346
	// ("cv::cuda::bitwise_not", vec![(pred!(mut, ["src", "dst", "mask", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_bitwise_not_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* mask, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::bitwise_not(*src, *dst, *mask, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::bitwise_or(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:361
	// ("cv::cuda::bitwise_or", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_bitwise_or_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::bitwise_or(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bitwise_or(InputArray, InputArray, OutputArray, InputArray, Stream &)(InputArray, InputArray, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:361
	// ("cv::cuda::bitwise_or", vec![(pred!(mut, ["src1", "src2", "dst", "mask", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_bitwise_or_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, const cv::_InputArray* mask, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::bitwise_or(*src1, *src2, *dst, *mask, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::bitwise_or_with_scalar(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:374
	// ("cv::cuda::bitwise_or_with_scalar", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_bitwise_or_with_scalar_const__InputArrayR_Scalar_const__OutputArrayR(const cv::_InputArray* src1, cv::Scalar* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::bitwise_or_with_scalar(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bitwise_or_with_scalar(InputArray, Scalar, OutputArray, InputArray, Stream &)(InputArray, SimpleClass, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:374
	// ("cv::cuda::bitwise_or_with_scalar", vec![(pred!(mut, ["src1", "src2", "dst", "mask", "stream"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_bitwise_or_with_scalar_const__InputArrayR_Scalar_const__OutputArrayR_const__InputArrayR_StreamR(const cv::_InputArray* src1, cv::Scalar* src2, const cv::_OutputArray* dst, const cv::_InputArray* mask, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::bitwise_or_with_scalar(*src1, *src2, *dst, *mask, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::bitwise_xor(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:421
	// ("cv::cuda::bitwise_xor", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_bitwise_xor_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::bitwise_xor(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bitwise_xor(InputArray, InputArray, OutputArray, InputArray, Stream &)(InputArray, InputArray, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:421
	// ("cv::cuda::bitwise_xor", vec![(pred!(mut, ["src1", "src2", "dst", "mask", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_bitwise_xor_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, const cv::_InputArray* mask, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::bitwise_xor(*src1, *src2, *dst, *mask, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::bitwise_xor_with_scalar(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:434
	// ("cv::cuda::bitwise_xor_with_scalar", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_bitwise_xor_with_scalar_const__InputArrayR_Scalar_const__OutputArrayR(const cv::_InputArray* src1, cv::Scalar* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::bitwise_xor_with_scalar(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bitwise_xor_with_scalar(InputArray, Scalar, OutputArray, InputArray, Stream &)(InputArray, SimpleClass, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:434
	// ("cv::cuda::bitwise_xor_with_scalar", vec![(pred!(mut, ["src1", "src2", "dst", "mask", "stream"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_bitwise_xor_with_scalar_const__InputArrayR_Scalar_const__OutputArrayR_const__InputArrayR_StreamR(const cv::_InputArray* src1, cv::Scalar* src2, const cv::_OutputArray* dst, const cv::_InputArray* mask, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::bitwise_xor_with_scalar(*src1, *src2, *dst, *mask, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::calcAbsSum(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:854
	// ("cv::cuda::calcAbsSum", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_calcAbsSum_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::calcAbsSum(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calcAbsSum(InputArray, OutputArray, InputArray, Stream &)(InputArray, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:854
	// ("cv::cuda::calcAbsSum", vec![(pred!(mut, ["src", "dst", "mask", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_calcAbsSum_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* mask, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::calcAbsSum(*src, *dst, *mask, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::calcNormDiff(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:834
	// ("cv::cuda::calcNormDiff", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_calcNormDiff_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::calcNormDiff(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calcNormDiff(InputArray, InputArray, OutputArray, int, Stream &)(InputArray, InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:834
	// ("cv::cuda::calcNormDiff", vec![(pred!(mut, ["src1", "src2", "dst", "normType", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_calcNormDiff_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_StreamR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, int normType, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::calcNormDiff(*src1, *src2, *dst, normType, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::calcNorm(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:822
	// ("cv::cuda::calcNorm", vec![(pred!(mut, ["src", "dst", "normType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_cuda_calcNorm_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int normType, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::calcNorm(*src, *dst, normType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calcNorm(InputArray, OutputArray, int, InputArray, Stream &)(InputArray, OutputArray, Primitive, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:822
	// ("cv::cuda::calcNorm", vec![(pred!(mut, ["src", "dst", "normType", "mask", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_calcNorm_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, int normType, const cv::_InputArray* mask, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::calcNorm(*src, *dst, normType, *mask, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::calcSqrSum(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:863
	// ("cv::cuda::calcSqrSum", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_calcSqrSum_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::calcSqrSum(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calcSqrSum(InputArray, OutputArray, InputArray, Stream &)(InputArray, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:863
	// ("cv::cuda::calcSqrSum", vec![(pred!(mut, ["src", "dst", "mask", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_calcSqrSum_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* mask, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::calcSqrSum(*src, *dst, *mask, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::calcSum(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:845
	// ("cv::cuda::calcSum", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_calcSum_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::calcSum(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calcSum(InputArray, OutputArray, InputArray, Stream &)(InputArray, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:845
	// ("cv::cuda::calcSum", vec![(pred!(mut, ["src", "dst", "mask", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_calcSum_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* mask, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::calcSum(*src, *dst, *mask, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::cartToPolar(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:658
	// ("cv::cuda::cartToPolar", vec![(pred!(mut, ["x", "y", "magnitude", "angle"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_cartToPolar_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* x, const cv::_InputArray* y, const cv::_OutputArray* magnitude, const cv::_OutputArray* angle, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::cartToPolar(*x, *y, *magnitude, *angle);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cartToPolar(InputArray, InputArray, OutputArray, OutputArray, bool, Stream &)(InputArray, InputArray, OutputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:658
	// ("cv::cuda::cartToPolar", vec![(pred!(mut, ["x", "y", "magnitude", "angle", "angleInDegrees", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_cartToPolar_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_StreamR(const cv::_InputArray* x, const cv::_InputArray* y, const cv::_OutputArray* magnitude, const cv::_OutputArray* angle, bool angleInDegrees, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::cartToPolar(*x, *y, *magnitude, *angle, angleInDegrees, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::cartToPolar(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:681
	// ("cv::cuda::cartToPolar", vec![(pred!(mut, ["xy", "magnitudeAngle"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_cartToPolar_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* xy, const cv::_OutputArray* magnitudeAngle, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::cartToPolar(*xy, *magnitudeAngle);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cartToPolar(InputArray, OutputArray, bool, Stream &)(InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:681
	// ("cv::cuda::cartToPolar", vec![(pred!(mut, ["xy", "magnitudeAngle", "angleInDegrees", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "bool", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_cartToPolar_const__InputArrayR_const__OutputArrayR_bool_StreamR(const cv::_InputArray* xy, const cv::_OutputArray* magnitudeAngle, bool angleInDegrees, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::cartToPolar(*xy, *magnitudeAngle, angleInDegrees, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::cartToPolar(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:670
	// ("cv::cuda::cartToPolar", vec![(pred!(mut, ["xy", "magnitude", "angle"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_cartToPolar_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* xy, const cv::_OutputArray* magnitude, const cv::_OutputArray* angle, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::cartToPolar(*xy, *magnitude, *angle);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cartToPolar(InputArray, OutputArray, OutputArray, bool, Stream &)(InputArray, OutputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:670
	// ("cv::cuda::cartToPolar", vec![(pred!(mut, ["xy", "magnitude", "angle", "angleInDegrees", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_cartToPolar_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_StreamR(const cv::_InputArray* xy, const cv::_OutputArray* magnitude, const cv::_OutputArray* angle, bool angleInDegrees, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::cartToPolar(*xy, *magnitude, *angle, angleInDegrees, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::compareWithScalar(InputArray, SimpleClass, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:334
	// ("cv::cuda::compareWithScalar", vec![(pred!(mut, ["src1", "src2", "dst", "cmpop"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*", "int"]), _)]),
	void cv_cuda_compareWithScalar_const__InputArrayR_Scalar_const__OutputArrayR_int(const cv::_InputArray* src1, cv::Scalar* src2, const cv::_OutputArray* dst, int cmpop, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::compareWithScalar(*src1, *src2, *dst, cmpop);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compareWithScalar(InputArray, Scalar, OutputArray, int, Stream &)(InputArray, SimpleClass, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:334
	// ("cv::cuda::compareWithScalar", vec![(pred!(mut, ["src1", "src2", "dst", "cmpop", "stream"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_compareWithScalar_const__InputArrayR_Scalar_const__OutputArrayR_int_StreamR(const cv::_InputArray* src1, cv::Scalar* src2, const cv::_OutputArray* dst, int cmpop, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::compareWithScalar(*src1, *src2, *dst, cmpop, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::compare(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:316
	// ("cv::cuda::compare", vec![(pred!(mut, ["src1", "src2", "dst", "cmpop"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_cuda_compare_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, int cmpop, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::compare(*src1, *src2, *dst, cmpop);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compare(InputArray, InputArray, OutputArray, int, Stream &)(InputArray, InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:316
	// ("cv::cuda::compare", vec![(pred!(mut, ["src1", "src2", "dst", "cmpop", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_compare_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_StreamR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, int cmpop, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::compare(*src1, *src2, *dst, cmpop, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::copyMakeBorder(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:804
	// ("cv::cuda::copyMakeBorder", vec![(pred!(mut, ["src", "dst", "top", "bottom", "left", "right", "borderType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int", "int", "int"]), _)]),
	void cv_cuda_copyMakeBorder_const__InputArrayR_const__OutputArrayR_int_int_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int top, int bottom, int left, int right, int borderType, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::copyMakeBorder(*src, *dst, top, bottom, left, right, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// copyMakeBorder(InputArray, OutputArray, int, int, int, int, int, Scalar, Stream &)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:804
	// ("cv::cuda::copyMakeBorder", vec![(pred!(mut, ["src", "dst", "top", "bottom", "left", "right", "borderType", "value", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int", "int", "int", "cv::Scalar", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_copyMakeBorder_const__InputArrayR_const__OutputArrayR_int_int_int_int_int_Scalar_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, int top, int bottom, int left, int right, int borderType, cv::Scalar* value, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::copyMakeBorder(*src, *dst, top, bottom, left, right, borderType, *value, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// countNonZero(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:907
	// ("cv::cuda::countNonZero", vec![(pred!(mut, ["src"], ["const cv::_InputArray*"]), _)]),
	void cv_cuda_countNonZero_const__InputArrayR(const cv::_InputArray* src, Result<int>* ocvrs_return) {
		try {
			int ret = cv::cuda::countNonZero(*src);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::countNonZero(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:909
	// ("cv::cuda::countNonZero", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_countNonZero_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::countNonZero(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// countNonZero(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:909
	// ("cv::cuda::countNonZero", vec![(pred!(mut, ["src", "dst", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_countNonZero_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::countNonZero(*src, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::createConvolution() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:1174
	// ("cv::cuda::createConvolution", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_createConvolution(Result<cv::Ptr<cv::cuda::Convolution>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::Convolution> ret = cv::cuda::createConvolution();
			Ok(new cv::Ptr<cv::cuda::Convolution>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createConvolution(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:1174
	// ("cv::cuda::createConvolution", vec![(pred!(mut, ["user_block_size"], ["cv::Size"]), _)]),
	void cv_cuda_createConvolution_Size(cv::Size* user_block_size, Result<cv::Ptr<cv::cuda::Convolution>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::Convolution> ret = cv::cuda::createConvolution(*user_block_size);
			Ok(new cv::Ptr<cv::cuda::Convolution>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createDFT(Size, int)(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:1148
	// ("cv::cuda::createDFT", vec![(pred!(mut, ["dft_size", "flags"], ["cv::Size", "int"]), _)]),
	void cv_cuda_createDFT_Size_int(cv::Size* dft_size, int flags, Result<cv::Ptr<cv::cuda::DFT>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::DFT> ret = cv::cuda::createDFT(*dft_size, flags);
			Ok(new cv::Ptr<cv::cuda::DFT>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createLookUpTable(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:787
	// ("cv::cuda::createLookUpTable", vec![(pred!(mut, ["lut"], ["const cv::_InputArray*"]), _)]),
	void cv_cuda_createLookUpTable_const__InputArrayR(const cv::_InputArray* lut, Result<cv::Ptr<cv::cuda::LookUpTable>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::LookUpTable> ret = cv::cuda::createLookUpTable(*lut);
			Ok(new cv::Ptr<cv::cuda::LookUpTable>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::dft(InputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:1119
	// ("cv::cuda::dft", vec![(pred!(mut, ["src", "dst", "dft_size"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size"]), _)]),
	void cv_cuda_dft_const__InputArrayR_const__OutputArrayR_Size(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Size* dft_size, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::dft(*src, *dst, *dft_size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dft(InputArray, OutputArray, Size, int, Stream &)(InputArray, OutputArray, SimpleClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:1119
	// ("cv::cuda::dft", vec![(pred!(mut, ["src", "dst", "dft_size", "flags", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_dft_const__InputArrayR_const__OutputArrayR_Size_int_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Size* dft_size, int flags, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::dft(*src, *dst, *dft_size, flags, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::divideWithScalar(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:205
	// ("cv::cuda::divideWithScalar", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_divideWithScalar_const__InputArrayR_Scalar_const__OutputArrayR(const cv::_InputArray* src1, cv::Scalar* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::divideWithScalar(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// divideWithScalar(InputArray, Scalar, OutputArray, double, int, Stream &)(InputArray, SimpleClass, OutputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:205
	// ("cv::cuda::divideWithScalar", vec![(pred!(mut, ["src1", "src2", "dst", "scale", "dtype", "stream"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*", "double", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_divideWithScalar_const__InputArrayR_Scalar_const__OutputArrayR_double_int_StreamR(const cv::_InputArray* src1, cv::Scalar* src2, const cv::_OutputArray* dst, double scale, int dtype, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::divideWithScalar(*src1, *src2, *dst, scale, dtype, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::divide(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:189
	// ("cv::cuda::divide", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_divide_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::divide(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// divide(InputArray, InputArray, OutputArray, double, int, Stream &)(InputArray, InputArray, OutputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:189
	// ("cv::cuda::divide", vec![(pred!(mut, ["src1", "src2", "dst", "scale", "dtype", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_divide_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_int_StreamR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, double scale, int dtype, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::divide(*src1, *src2, *dst, scale, dtype, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::exp(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:271
	// ("cv::cuda::exp", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_exp_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::exp(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// exp(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:271
	// ("cv::cuda::exp", vec![(pred!(mut, ["src", "dst", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_exp_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::exp(*src, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::findMinMaxLoc(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:896
	// ("cv::cuda::findMinMaxLoc", vec![(pred!(mut, ["src", "minMaxVals", "loc"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_findMinMaxLoc_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* minMaxVals, const cv::_OutputArray* loc, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::findMinMaxLoc(*src, *minMaxVals, *loc);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findMinMaxLoc(InputArray, OutputArray, OutputArray, InputArray, Stream &)(InputArray, OutputArray, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:896
	// ("cv::cuda::findMinMaxLoc", vec![(pred!(mut, ["src", "minMaxVals", "loc", "mask", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_findMinMaxLoc_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* minMaxVals, const cv::_OutputArray* loc, const cv::_InputArray* mask, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::findMinMaxLoc(*src, *minMaxVals, *loc, *mask, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::findMinMax(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:878
	// ("cv::cuda::findMinMax", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_findMinMax_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::findMinMax(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findMinMax(InputArray, OutputArray, InputArray, Stream &)(InputArray, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:878
	// ("cv::cuda::findMinMax", vec![(pred!(mut, ["src", "dst", "mask", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_findMinMax_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* mask, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::findMinMax(*src, *dst, *mask, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::flip(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:766
	// ("cv::cuda::flip", vec![(pred!(mut, ["src", "dst", "flipCode"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_cuda_flip_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int flipCode, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::flip(*src, *dst, flipCode);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// flip(InputArray, OutputArray, int, Stream &)(InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:766
	// ("cv::cuda::flip", vec![(pred!(mut, ["src", "dst", "flipCode", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_flip_const__InputArrayR_const__OutputArrayR_int_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, int flipCode, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::flip(*src, *dst, flipCode, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::gemm(InputArray, InputArray, Primitive, InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:1048
	// ("cv::cuda::gemm", vec![(pred!(mut, ["src1", "src2", "alpha", "src3", "beta", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_InputArray*", "double", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_gemm_const__InputArrayR_const__InputArrayR_double_const__InputArrayR_double_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, double alpha, const cv::_InputArray* src3, double beta, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::gemm(*src1, *src2, alpha, *src3, beta, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// gemm(InputArray, InputArray, double, InputArray, double, OutputArray, int, Stream &)(InputArray, InputArray, Primitive, InputArray, Primitive, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:1048
	// ("cv::cuda::gemm", vec![(pred!(mut, ["src1", "src2", "alpha", "src3", "beta", "dst", "flags", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_InputArray*", "double", "const cv::_OutputArray*", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_gemm_const__InputArrayR_const__InputArrayR_double_const__InputArrayR_double_const__OutputArrayR_int_StreamR(const cv::_InputArray* src1, const cv::_InputArray* src2, double alpha, const cv::_InputArray* src3, double beta, const cv::_OutputArray* dst, int flags, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::gemm(*src1, *src2, alpha, *src3, beta, *dst, flags, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::inRange(InputArray, SimpleClass, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:584
	// ("cv::cuda::inRange", vec![(pred!(mut, ["src", "lowerb", "upperb", "dst"], ["const cv::_InputArray*", "const cv::Scalar*", "const cv::Scalar*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_inRange_const__InputArrayR_const_ScalarR_const_ScalarR_const__OutputArrayR(const cv::_InputArray* src, const cv::Scalar* lowerb, const cv::Scalar* upperb, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::inRange(*src, *lowerb, *upperb, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// inRange(InputArray, const Scalar &, const Scalar &, OutputArray, Stream &)(InputArray, SimpleClass, SimpleClass, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:584
	// ("cv::cuda::inRange", vec![(pred!(mut, ["src", "lowerb", "upperb", "dst", "stream"], ["const cv::_InputArray*", "const cv::Scalar*", "const cv::Scalar*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_inRange_const__InputArrayR_const_ScalarR_const_ScalarR_const__OutputArrayR_StreamR(const cv::_InputArray* src, const cv::Scalar* lowerb, const cv::Scalar* upperb, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::inRange(*src, *lowerb, *upperb, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::integral(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:1007
	// ("cv::cuda::integral", vec![(pred!(mut, ["src", "sum"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_integral_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* sum, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::integral(*src, *sum);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// integral(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:1007
	// ("cv::cuda::integral", vec![(pred!(mut, ["src", "sum", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_integral_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* sum, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::integral(*src, *sum, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::log(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:281
	// ("cv::cuda::log", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_log_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::log(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// log(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:281
	// ("cv::cuda::log", vec![(pred!(mut, ["src", "dst", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_log_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::log(*src, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::lshift(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:459
	// ("cv::cuda::lshift", vec![(pred!(mut, ["src", "val", "dst"], ["const cv::_InputArray*", "cv::Scalar_<int>", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_lshift_const__InputArrayR_Scalar_LintG_const__OutputArrayR(const cv::_InputArray* src, cv::Scalar_<int>* val, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::lshift(*src, *val, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// lshift(InputArray, Scalar_<int>, OutputArray, Stream &)(InputArray, SimpleClass, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:459
	// ("cv::cuda::lshift", vec![(pred!(mut, ["src", "val", "dst", "stream"], ["const cv::_InputArray*", "cv::Scalar_<int>", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_lshift_const__InputArrayR_Scalar_LintG_const__OutputArrayR_StreamR(const cv::_InputArray* src, cv::Scalar_<int>* val, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::lshift(*src, *val, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::lshift(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:461
	// ("cv::cuda::lshift", vec![(pred!(mut, ["src", "val", "dst"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_lshift_const__InputArrayR_Scalar_const__OutputArrayR(const cv::_InputArray* src, cv::Scalar* val, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::lshift(*src, *val, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// lshift(InputArray, Scalar, OutputArray, Stream &)(InputArray, SimpleClass, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:461
	// ("cv::cuda::lshift", vec![(pred!(mut, ["src", "val", "dst", "stream"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_lshift_const__InputArrayR_Scalar_const__OutputArrayR_StreamR(const cv::_InputArray* src, cv::Scalar* val, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::lshift(*src, *val, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::magnitudeSqr(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:622
	// ("cv::cuda::magnitudeSqr", vec![(pred!(mut, ["x", "y", "magnitude"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_magnitudeSqr_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* x, const cv::_InputArray* y, const cv::_OutputArray* magnitude, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::magnitudeSqr(*x, *y, *magnitude);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// magnitudeSqr(InputArray, InputArray, OutputArray, Stream &)(InputArray, InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:622
	// ("cv::cuda::magnitudeSqr", vec![(pred!(mut, ["x", "y", "magnitude", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_magnitudeSqr_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* x, const cv::_InputArray* y, const cv::_OutputArray* magnitude, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::magnitudeSqr(*x, *y, *magnitude, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::magnitudeSqr(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:602
	// ("cv::cuda::magnitudeSqr", vec![(pred!(mut, ["xy", "magnitude"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_magnitudeSqr_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* xy, const cv::_OutputArray* magnitude, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::magnitudeSqr(*xy, *magnitude);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// magnitudeSqr(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:602
	// ("cv::cuda::magnitudeSqr", vec![(pred!(mut, ["xy", "magnitude", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_magnitudeSqr_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* xy, const cv::_OutputArray* magnitude, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::magnitudeSqr(*xy, *magnitude, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::magnitude(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:612
	// ("cv::cuda::magnitude", vec![(pred!(mut, ["x", "y", "magnitude"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_magnitude_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* x, const cv::_InputArray* y, const cv::_OutputArray* magnitude, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::magnitude(*x, *y, *magnitude);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// magnitude(InputArray, InputArray, OutputArray, Stream &)(InputArray, InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:612
	// ("cv::cuda::magnitude", vec![(pred!(mut, ["x", "y", "magnitude", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_magnitude_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* x, const cv::_InputArray* y, const cv::_OutputArray* magnitude, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::magnitude(*x, *y, *magnitude, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::magnitude(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:594
	// ("cv::cuda::magnitude", vec![(pred!(mut, ["xy", "magnitude"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_magnitude_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* xy, const cv::_OutputArray* magnitude, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::magnitude(*xy, *magnitude);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// magnitude(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:594
	// ("cv::cuda::magnitude", vec![(pred!(mut, ["xy", "magnitude", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_magnitude_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* xy, const cv::_OutputArray* magnitude, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::magnitude(*xy, *magnitude, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::maxWithScalar(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:513
	// ("cv::cuda::maxWithScalar", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_maxWithScalar_const__InputArrayR_Scalar_const__OutputArrayR(const cv::_InputArray* src1, cv::Scalar* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::maxWithScalar(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxWithScalar(InputArray, Scalar, OutputArray, Stream &)(InputArray, SimpleClass, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:513
	// ("cv::cuda::maxWithScalar", vec![(pred!(mut, ["src1", "src2", "dst", "stream"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_maxWithScalar_const__InputArrayR_Scalar_const__OutputArrayR_StreamR(const cv::_InputArray* src1, cv::Scalar* src2, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::maxWithScalar(*src1, *src2, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::max(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:502
	// ("cv::cuda::max", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_max_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::max(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// max(InputArray, InputArray, OutputArray, Stream &)(InputArray, InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:502
	// ("cv::cuda::max", vec![(pred!(mut, ["src1", "src2", "dst", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_max_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::max(*src1, *src2, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// meanStdDev(InputArray, Scalar &, Scalar &)(InputArray, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:967
	// ("cv::cuda::meanStdDev", vec![(pred!(mut, ["mtx", "mean", "stddev"], ["const cv::_InputArray*", "cv::Scalar*", "cv::Scalar*"]), _)]),
	void cv_cuda_meanStdDev_const__InputArrayR_ScalarR_ScalarR(const cv::_InputArray* mtx, cv::Scalar* mean, cv::Scalar* stddev, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::meanStdDev(*mtx, *mean, *stddev);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// meanStdDev(InputArray, Scalar &, Scalar &, InputArray)(InputArray, SimpleClass, SimpleClass, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:961
	// ("cv::cuda::meanStdDev", vec![(pred!(mut, ["src", "mean", "stddev", "mask"], ["const cv::_InputArray*", "cv::Scalar*", "cv::Scalar*", "const cv::_InputArray*"]), _)]),
	void cv_cuda_meanStdDev_const__InputArrayR_ScalarR_ScalarR_const__InputArrayR(const cv::_InputArray* src, cv::Scalar* mean, cv::Scalar* stddev, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::meanStdDev(*src, *mean, *stddev, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::meanStdDev(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:954
	// ("cv::cuda::meanStdDev", vec![(pred!(mut, ["mtx", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_meanStdDev_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* mtx, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::meanStdDev(*mtx, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// meanStdDev(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:954
	// ("cv::cuda::meanStdDev", vec![(pred!(mut, ["mtx", "dst", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_meanStdDev_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* mtx, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::meanStdDev(*mtx, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::meanStdDev(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:948
	// ("cv::cuda::meanStdDev", vec![(pred!(mut, ["src", "dst", "mask"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_cuda_meanStdDev_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::meanStdDev(*src, *dst, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// meanStdDev(InputArray, OutputArray, InputArray, Stream &)(InputArray, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:948
	// ("cv::cuda::meanStdDev", vec![(pred!(mut, ["src", "dst", "mask", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_meanStdDev_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* mask, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::meanStdDev(*src, *dst, *mask, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::merge(TraitClass, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:727
	// ("cv::cuda::merge", vec![(pred!(mut, ["src", "n", "dst"], ["const cv::cuda::GpuMat*", "size_t", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_merge_const_GpuMatX_size_t_const__OutputArrayR(const cv::cuda::GpuMat* src, size_t n, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::merge(src, n, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// merge(const GpuMat *, size_t, OutputArray, Stream &)(TraitClass, Primitive, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:727
	// ("cv::cuda::merge", vec![(pred!(mut, ["src", "n", "dst", "stream"], ["const cv::cuda::GpuMat*", "size_t", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_merge_const_GpuMatX_size_t_const__OutputArrayR_StreamR(const cv::cuda::GpuMat* src, size_t n, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::merge(src, n, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::merge(CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:729
	// ("cv::cuda::merge", vec![(pred!(mut, ["src", "dst"], ["const std::vector<cv::cuda::GpuMat>*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_merge_const_vectorLGpuMatGR_const__OutputArrayR(const std::vector<cv::cuda::GpuMat>* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::merge(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// merge(const std::vector<GpuMat> &, OutputArray, Stream &)(CppPassByVoidPtr, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:729
	// ("cv::cuda::merge", vec![(pred!(mut, ["src", "dst", "stream"], ["const std::vector<cv::cuda::GpuMat>*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_merge_const_vectorLGpuMatGR_const__OutputArrayR_StreamR(const std::vector<cv::cuda::GpuMat>* src, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::merge(*src, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::minMaxLoc(InputArray, Indirect, Indirect, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:893
	// ("cv::cuda::minMaxLoc", vec![(pred!(mut, ["src", "minVal", "maxVal", "minLoc", "maxLoc"], ["const cv::_InputArray*", "double*", "double*", "cv::Point*", "cv::Point*"]), _)]),
	void cv_cuda_minMaxLoc_const__InputArrayR_doubleX_doubleX_PointX_PointX(const cv::_InputArray* src, double* minVal, double* maxVal, cv::Point* minLoc, cv::Point* maxLoc, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::minMaxLoc(*src, minVal, maxVal, minLoc, maxLoc);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// minMaxLoc(InputArray, double *, double *, Point *, Point *, InputArray)(InputArray, Indirect, Indirect, SimpleClass, SimpleClass, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:893
	// ("cv::cuda::minMaxLoc", vec![(pred!(mut, ["src", "minVal", "maxVal", "minLoc", "maxLoc", "mask"], ["const cv::_InputArray*", "double*", "double*", "cv::Point*", "cv::Point*", "const cv::_InputArray*"]), _)]),
	void cv_cuda_minMaxLoc_const__InputArrayR_doubleX_doubleX_PointX_PointX_const__InputArrayR(const cv::_InputArray* src, double* minVal, double* maxVal, cv::Point* minLoc, cv::Point* maxLoc, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::minMaxLoc(*src, minVal, maxVal, minLoc, maxLoc, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::minMax(InputArray, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:876
	// ("cv::cuda::minMax", vec![(pred!(mut, ["src", "minVal", "maxVal"], ["const cv::_InputArray*", "double*", "double*"]), _)]),
	void cv_cuda_minMax_const__InputArrayR_doubleX_doubleX(const cv::_InputArray* src, double* minVal, double* maxVal, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::minMax(*src, minVal, maxVal);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// minMax(InputArray, double *, double *, InputArray)(InputArray, Indirect, Indirect, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:876
	// ("cv::cuda::minMax", vec![(pred!(mut, ["src", "minVal", "maxVal", "mask"], ["const cv::_InputArray*", "double*", "double*", "const cv::_InputArray*"]), _)]),
	void cv_cuda_minMax_const__InputArrayR_doubleX_doubleX_const__InputArrayR(const cv::_InputArray* src, double* minVal, double* maxVal, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::minMax(*src, minVal, maxVal, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::minWithScalar(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:487
	// ("cv::cuda::minWithScalar", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_minWithScalar_const__InputArrayR_Scalar_const__OutputArrayR(const cv::_InputArray* src1, cv::Scalar* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::minWithScalar(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// minWithScalar(InputArray, Scalar, OutputArray, Stream &)(InputArray, SimpleClass, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:487
	// ("cv::cuda::minWithScalar", vec![(pred!(mut, ["src1", "src2", "dst", "stream"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_minWithScalar_const__InputArrayR_Scalar_const__OutputArrayR_StreamR(const cv::_InputArray* src1, cv::Scalar* src2, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::minWithScalar(*src1, *src2, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::min(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:476
	// ("cv::cuda::min", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_min_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::min(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// min(InputArray, InputArray, OutputArray, Stream &)(InputArray, InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:476
	// ("cv::cuda::min", vec![(pred!(mut, ["src1", "src2", "dst", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_min_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::min(*src1, *src2, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::mulAndScaleSpectrums(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:1082
	// ("cv::cuda::mulAndScaleSpectrums", vec![(pred!(mut, ["src1", "src2", "dst", "flags", "scale"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "float"]), _)]),
	void cv_cuda_mulAndScaleSpectrums_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_float(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, int flags, float scale, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::mulAndScaleSpectrums(*src1, *src2, *dst, flags, scale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mulAndScaleSpectrums(InputArray, InputArray, OutputArray, int, float, bool, Stream &)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:1082
	// ("cv::cuda::mulAndScaleSpectrums", vec![(pred!(mut, ["src1", "src2", "dst", "flags", "scale", "conjB", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "float", "bool", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_mulAndScaleSpectrums_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_float_bool_StreamR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, int flags, float scale, bool conjB, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::mulAndScaleSpectrums(*src1, *src2, *dst, flags, scale, conjB, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::mulSpectrums(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:1065
	// ("cv::cuda::mulSpectrums", vec![(pred!(mut, ["src1", "src2", "dst", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_cuda_mulSpectrums_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, int flags, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::mulSpectrums(*src1, *src2, *dst, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mulSpectrums(InputArray, InputArray, OutputArray, int, bool, Stream &)(InputArray, InputArray, OutputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:1065
	// ("cv::cuda::mulSpectrums", vec![(pred!(mut, ["src1", "src2", "dst", "flags", "conjB", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "bool", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_mulSpectrums_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_bool_StreamR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, int flags, bool conjB, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::mulSpectrums(*src1, *src2, *dst, flags, conjB, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::multiplyWithScalar(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:169
	// ("cv::cuda::multiplyWithScalar", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_multiplyWithScalar_const__InputArrayR_Scalar_const__OutputArrayR(const cv::_InputArray* src1, cv::Scalar* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::multiplyWithScalar(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// multiplyWithScalar(InputArray, Scalar, OutputArray, double, int, Stream &)(InputArray, SimpleClass, OutputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:169
	// ("cv::cuda::multiplyWithScalar", vec![(pred!(mut, ["src1", "src2", "dst", "scale", "dtype", "stream"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*", "double", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_multiplyWithScalar_const__InputArrayR_Scalar_const__OutputArrayR_double_int_StreamR(const cv::_InputArray* src1, cv::Scalar* src2, const cv::_OutputArray* dst, double scale, int dtype, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::multiplyWithScalar(*src1, *src2, *dst, scale, dtype, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::multiply(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:155
	// ("cv::cuda::multiply", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_multiply_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::multiply(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// multiply(InputArray, InputArray, OutputArray, double, int, Stream &)(InputArray, InputArray, OutputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:155
	// ("cv::cuda::multiply", vec![(pred!(mut, ["src1", "src2", "dst", "scale", "dtype", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_multiply_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_int_StreamR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, double scale, int dtype, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::multiply(*src1, *src2, *dst, scale, dtype, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::norm(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:832
	// ("cv::cuda::norm", vec![(pred!(mut, ["src1", "src2"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_cuda_norm_const__InputArrayR_const__InputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, Result<double>* ocvrs_return) {
		try {
			double ret = cv::cuda::norm(*src1, *src2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// norm(InputArray, InputArray, int)(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:832
	// ("cv::cuda::norm", vec![(pred!(mut, ["src1", "src2", "normType"], ["const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
	void cv_cuda_norm_const__InputArrayR_const__InputArrayR_int(const cv::_InputArray* src1, const cv::_InputArray* src2, int normType, Result<double>* ocvrs_return) {
		try {
			double ret = cv::cuda::norm(*src1, *src2, normType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::norm(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:820
	// ("cv::cuda::norm", vec![(pred!(mut, ["src1", "normType"], ["const cv::_InputArray*", "int"]), _)]),
	void cv_cuda_norm_const__InputArrayR_int(const cv::_InputArray* src1, int normType, Result<double>* ocvrs_return) {
		try {
			double ret = cv::cuda::norm(*src1, normType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// norm(InputArray, int, InputArray)(InputArray, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:820
	// ("cv::cuda::norm", vec![(pred!(mut, ["src1", "normType", "mask"], ["const cv::_InputArray*", "int", "const cv::_InputArray*"]), _)]),
	void cv_cuda_norm_const__InputArrayR_int_const__InputArrayR(const cv::_InputArray* src1, int normType, const cv::_InputArray* mask, Result<double>* ocvrs_return) {
		try {
			double ret = cv::cuda::norm(*src1, normType, *mask);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::normalize(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:995
	// ("cv::cuda::normalize", vec![(pred!(mut, ["src", "dst", "alpha", "beta", "norm_type", "dtype"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "int", "int"]), _)]),
	void cv_cuda_normalize_const__InputArrayR_const__OutputArrayR_double_double_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, double alpha, double beta, int norm_type, int dtype, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::normalize(*src, *dst, alpha, beta, norm_type, dtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// normalize(InputArray, OutputArray, double, double, int, int, InputArray, Stream &)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:995
	// ("cv::cuda::normalize", vec![(pred!(mut, ["src", "dst", "alpha", "beta", "norm_type", "dtype", "mask", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "int", "int", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_normalize_const__InputArrayR_const__OutputArrayR_double_double_int_int_const__InputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, double alpha, double beta, int norm_type, int dtype, const cv::_InputArray* mask, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::normalize(*src, *dst, alpha, beta, norm_type, dtype, *mask, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::phase(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:634
	// ("cv::cuda::phase", vec![(pred!(mut, ["x", "y", "angle"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_phase_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* x, const cv::_InputArray* y, const cv::_OutputArray* angle, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::phase(*x, *y, *angle);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// phase(InputArray, InputArray, OutputArray, bool, Stream &)(InputArray, InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:634
	// ("cv::cuda::phase", vec![(pred!(mut, ["x", "y", "angle", "angleInDegrees", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "bool", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_phase_const__InputArrayR_const__InputArrayR_const__OutputArrayR_bool_StreamR(const cv::_InputArray* x, const cv::_InputArray* y, const cv::_OutputArray* angle, bool angleInDegrees, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::phase(*x, *y, *angle, angleInDegrees, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::phase(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:645
	// ("cv::cuda::phase", vec![(pred!(mut, ["xy", "angle"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_phase_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* xy, const cv::_OutputArray* angle, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::phase(*xy, *angle);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// phase(InputArray, OutputArray, bool, Stream &)(InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:645
	// ("cv::cuda::phase", vec![(pred!(mut, ["xy", "angle", "angleInDegrees", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "bool", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_phase_const__InputArrayR_const__OutputArrayR_bool_StreamR(const cv::_InputArray* xy, const cv::_OutputArray* angle, bool angleInDegrees, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::phase(*xy, *angle, angleInDegrees, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::polarToCart(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:702
	// ("cv::cuda::polarToCart", vec![(pred!(mut, ["magnitude", "angle", "xy"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_polarToCart_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* magnitude, const cv::_InputArray* angle, const cv::_OutputArray* xy, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::polarToCart(*magnitude, *angle, *xy);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// polarToCart(InputArray, InputArray, OutputArray, bool, Stream &)(InputArray, InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:702
	// ("cv::cuda::polarToCart", vec![(pred!(mut, ["magnitude", "angle", "xy", "angleInDegrees", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "bool", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_polarToCart_const__InputArrayR_const__InputArrayR_const__OutputArrayR_bool_StreamR(const cv::_InputArray* magnitude, const cv::_InputArray* angle, const cv::_OutputArray* xy, bool angleInDegrees, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::polarToCart(*magnitude, *angle, *xy, angleInDegrees, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::polarToCart(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:692
	// ("cv::cuda::polarToCart", vec![(pred!(mut, ["magnitude", "angle", "x", "y"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_polarToCart_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* magnitude, const cv::_InputArray* angle, const cv::_OutputArray* x, const cv::_OutputArray* y, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::polarToCart(*magnitude, *angle, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// polarToCart(InputArray, InputArray, OutputArray, OutputArray, bool, Stream &)(InputArray, InputArray, OutputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:692
	// ("cv::cuda::polarToCart", vec![(pred!(mut, ["magnitude", "angle", "x", "y", "angleInDegrees", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_polarToCart_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_StreamR(const cv::_InputArray* magnitude, const cv::_InputArray* angle, const cv::_OutputArray* x, const cv::_OutputArray* y, bool angleInDegrees, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::polarToCart(*magnitude, *angle, *x, *y, angleInDegrees, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::polarToCart(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:711
	// ("cv::cuda::polarToCart", vec![(pred!(mut, ["magnitudeAngle", "xy"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_polarToCart_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* magnitudeAngle, const cv::_OutputArray* xy, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::polarToCart(*magnitudeAngle, *xy);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// polarToCart(InputArray, OutputArray, bool, Stream &)(InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:711
	// ("cv::cuda::polarToCart", vec![(pred!(mut, ["magnitudeAngle", "xy", "angleInDegrees", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "bool", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_polarToCart_const__InputArrayR_const__OutputArrayR_bool_StreamR(const cv::_InputArray* magnitudeAngle, const cv::_OutputArray* xy, bool angleInDegrees, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::polarToCart(*magnitudeAngle, *xy, angleInDegrees, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::pow(InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:296
	// ("cv::cuda::pow", vec![(pred!(mut, ["src", "power", "dst"], ["const cv::_InputArray*", "double", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_pow_const__InputArrayR_double_const__OutputArrayR(const cv::_InputArray* src, double power, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::pow(*src, power, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// pow(InputArray, double, OutputArray, Stream &)(InputArray, Primitive, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:296
	// ("cv::cuda::pow", vec![(pred!(mut, ["src", "power", "dst", "stream"], ["const cv::_InputArray*", "double", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_pow_const__InputArrayR_double_const__OutputArrayR_StreamR(const cv::_InputArray* src, double power, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::pow(*src, power, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::rectStdDev(InputArray, InputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:977
	// ("cv::cuda::rectStdDev", vec![(pred!(mut, ["src", "sqr", "dst", "rect"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "cv::Rect"]), _)]),
	void cv_cuda_rectStdDev_const__InputArrayR_const__InputArrayR_const__OutputArrayR_Rect(const cv::_InputArray* src, const cv::_InputArray* sqr, const cv::_OutputArray* dst, cv::Rect* rect, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::rectStdDev(*src, *sqr, *dst, *rect);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// rectStdDev(InputArray, InputArray, OutputArray, Rect, Stream &)(InputArray, InputArray, OutputArray, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:977
	// ("cv::cuda::rectStdDev", vec![(pred!(mut, ["src", "sqr", "dst", "rect", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "cv::Rect", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_rectStdDev_const__InputArrayR_const__InputArrayR_const__OutputArrayR_Rect_StreamR(const cv::_InputArray* src, const cv::_InputArray* sqr, const cv::_OutputArray* dst, cv::Rect* rect, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::rectStdDev(*src, *sqr, *dst, *rect, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::reduce(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:937
	// ("cv::cuda::reduce", vec![(pred!(mut, ["mtx", "vec", "dim", "reduceOp"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_cuda_reduce_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* mtx, const cv::_OutputArray* vec, int dim, int reduceOp, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::reduce(*mtx, *vec, dim, reduceOp);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reduce(InputArray, OutputArray, int, int, int, Stream &)(InputArray, OutputArray, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:937
	// ("cv::cuda::reduce", vec![(pred!(mut, ["mtx", "vec", "dim", "reduceOp", "dtype", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_reduce_const__InputArrayR_const__OutputArrayR_int_int_int_StreamR(const cv::_InputArray* mtx, const cv::_OutputArray* vec, int dim, int reduceOp, int dtype, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::reduce(*mtx, *vec, dim, reduceOp, dtype, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::rshift(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:445
	// ("cv::cuda::rshift", vec![(pred!(mut, ["src", "val", "dst"], ["const cv::_InputArray*", "cv::Scalar_<int>", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_rshift_const__InputArrayR_Scalar_LintG_const__OutputArrayR(const cv::_InputArray* src, cv::Scalar_<int>* val, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::rshift(*src, *val, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// rshift(InputArray, Scalar_<int>, OutputArray, Stream &)(InputArray, SimpleClass, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:445
	// ("cv::cuda::rshift", vec![(pred!(mut, ["src", "val", "dst", "stream"], ["const cv::_InputArray*", "cv::Scalar_<int>", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_rshift_const__InputArrayR_Scalar_LintG_const__OutputArrayR_StreamR(const cv::_InputArray* src, cv::Scalar_<int>* val, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::rshift(*src, *val, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::rshift(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:447
	// ("cv::cuda::rshift", vec![(pred!(mut, ["src", "val", "dst"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_rshift_const__InputArrayR_Scalar_const__OutputArrayR(const cv::_InputArray* src, cv::Scalar* val, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::rshift(*src, *val, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// rshift(InputArray, Scalar, OutputArray, Stream &)(InputArray, SimpleClass, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:447
	// ("cv::cuda::rshift", vec![(pred!(mut, ["src", "val", "dst", "stream"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_rshift_const__InputArrayR_Scalar_const__OutputArrayR_StreamR(const cv::_InputArray* src, cv::Scalar* val, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::rshift(*src, *val, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::split(InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:739
	// ("cv::cuda::split", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "cv::cuda::GpuMat*"]), _)]),
	void cv_cuda_split_const__InputArrayR_GpuMatX(const cv::_InputArray* src, cv::cuda::GpuMat* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::split(*src, dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// split(InputArray, GpuMat *, Stream &)(InputArray, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:739
	// ("cv::cuda::split", vec![(pred!(mut, ["src", "dst", "stream"], ["const cv::_InputArray*", "cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_split_const__InputArrayR_GpuMatX_StreamR(const cv::_InputArray* src, cv::cuda::GpuMat* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::split(*src, dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::split(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:741
	// ("cv::cuda::split", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "std::vector<cv::cuda::GpuMat>*"]), _)]),
	void cv_cuda_split_const__InputArrayR_vectorLGpuMatGR(const cv::_InputArray* src, std::vector<cv::cuda::GpuMat>* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::split(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// split(InputArray, std::vector<GpuMat> &, Stream &)(InputArray, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:741
	// ("cv::cuda::split", vec![(pred!(mut, ["src", "dst", "stream"], ["const cv::_InputArray*", "std::vector<cv::cuda::GpuMat>*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_split_const__InputArrayR_vectorLGpuMatGR_StreamR(const cv::_InputArray* src, std::vector<cv::cuda::GpuMat>* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::split(*src, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::sqrIntegral(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:1016
	// ("cv::cuda::sqrIntegral", vec![(pred!(mut, ["src", "sqsum"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_sqrIntegral_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* sqsum, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::sqrIntegral(*src, *sqsum);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// sqrIntegral(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:1016
	// ("cv::cuda::sqrIntegral", vec![(pred!(mut, ["src", "sqsum", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_sqrIntegral_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* sqsum, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::sqrIntegral(*src, *sqsum, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::sqrSum(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:861
	// ("cv::cuda::sqrSum", vec![(pred!(mut, ["src"], ["const cv::_InputArray*"]), _)]),
	void cv_cuda_sqrSum_const__InputArrayR(const cv::_InputArray* src, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = cv::cuda::sqrSum(*src);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// sqrSum(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:861
	// ("cv::cuda::sqrSum", vec![(pred!(mut, ["src", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_cuda_sqrSum_const__InputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_InputArray* mask, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = cv::cuda::sqrSum(*src, *mask);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::sqr(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:251
	// ("cv::cuda::sqr", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_sqr_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::sqr(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// sqr(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:251
	// ("cv::cuda::sqr", vec![(pred!(mut, ["src", "dst", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_sqr_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::sqr(*src, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::sqrt(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:261
	// ("cv::cuda::sqrt", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_sqrt_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::sqrt(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// sqrt(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:261
	// ("cv::cuda::sqrt", vec![(pred!(mut, ["src", "dst", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_sqrt_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::sqrt(*src, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::subtractWithScalar(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:137
	// ("cv::cuda::subtractWithScalar", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_subtractWithScalar_const__InputArrayR_Scalar_const__OutputArrayR(const cv::_InputArray* src1, cv::Scalar* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::subtractWithScalar(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// subtractWithScalar(InputArray, Scalar, OutputArray, InputArray, int, Stream &)(InputArray, SimpleClass, OutputArray, InputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:137
	// ("cv::cuda::subtractWithScalar", vec![(pred!(mut, ["src1", "src2", "dst", "mask", "dtype", "stream"], ["const cv::_InputArray*", "cv::Scalar", "const cv::_OutputArray*", "const cv::_InputArray*", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_subtractWithScalar_const__InputArrayR_Scalar_const__OutputArrayR_const__InputArrayR_int_StreamR(const cv::_InputArray* src1, cv::Scalar* src2, const cv::_OutputArray* dst, const cv::_InputArray* mask, int dtype, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::subtractWithScalar(*src1, *src2, *dst, *mask, dtype, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::subtract(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:122
	// ("cv::cuda::subtract", vec![(pred!(mut, ["src1", "src2", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_subtract_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::subtract(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// subtract(InputArray, InputArray, OutputArray, InputArray, int, Stream &)(InputArray, InputArray, OutputArray, InputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:122
	// ("cv::cuda::subtract", vec![(pred!(mut, ["src1", "src2", "dst", "mask", "dtype", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_subtract_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_int_StreamR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, const cv::_InputArray* mask, int dtype, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::subtract(*src1, *src2, *dst, *mask, dtype, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::sum(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:843
	// ("cv::cuda::sum", vec![(pred!(mut, ["src"], ["const cv::_InputArray*"]), _)]),
	void cv_cuda_sum_const__InputArrayR(const cv::_InputArray* src, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = cv::cuda::sum(*src);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// sum(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:843
	// ("cv::cuda::sum", vec![(pred!(mut, ["src", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_cuda_sum_const__InputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_InputArray* mask, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = cv::cuda::sum(*src, *mask);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::threshold(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:559
	// ("cv::cuda::threshold", vec![(pred!(mut, ["src", "dst", "thresh", "maxval", "type"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "int"]), _)]),
	void cv_cuda_threshold_const__InputArrayR_const__OutputArrayR_double_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, double thresh, double maxval, int type, Result<double>* ocvrs_return) {
		try {
			double ret = cv::cuda::threshold(*src, *dst, thresh, maxval, type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// threshold(InputArray, OutputArray, double, double, int, Stream &)(InputArray, OutputArray, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:559
	// ("cv::cuda::threshold", vec![(pred!(mut, ["src", "dst", "thresh", "maxval", "type", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "double", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_threshold_const__InputArrayR_const__OutputArrayR_double_double_int_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, double thresh, double maxval, int type, cv::cuda::Stream* stream, Result<double>* ocvrs_return) {
		try {
			double ret = cv::cuda::threshold(*src, *dst, thresh, maxval, type, *stream);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::transpose(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:751
	// ("cv::cuda::transpose", vec![(pred!(mut, ["src1", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_transpose_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::transpose(*src1, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// transpose(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:751
	// ("cv::cuda::transpose", vec![(pred!(mut, ["src1", "dst", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_transpose_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src1, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::transpose(*src1, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convolve(InputArray, InputArray, OutputArray, bool, Stream &)(InputArray, InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:1165
	// ("cv::cuda::Convolution::convolve", vec![(pred!(mut, ["image", "templ", "result", "ccorr", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "bool", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_Convolution_convolve_const__InputArrayR_const__InputArrayR_const__OutputArrayR_bool_StreamR(cv::cuda::Convolution* instance, const cv::_InputArray* image, const cv::_InputArray* templ, const cv::_OutputArray* result, bool ccorr, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->convolve(*image, *templ, *result, ccorr, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::Convolution::convolve(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:1165
	// ("cv::cuda::Convolution::convolve", vec![(pred!(mut, ["image", "templ", "result"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_Convolution_convolve_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::cuda::Convolution* instance, const cv::_InputArray* image, const cv::_InputArray* templ, const cv::_OutputArray* result, ResultVoid* ocvrs_return) {
		try {
			instance->convolve(*image, *templ, *result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::Convolution::to_Algorithm() generated
	// ("cv::cuda::Convolution::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_Convolution_to_Algorithm(cv::cuda::Convolution* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::Convolution::delete() generated
	// ("cv::cuda::Convolution::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_Convolution_delete(cv::cuda::Convolution* instance) {
			delete instance;
	}

	// compute(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:1132
	// ("cv::cuda::DFT::compute", vec![(pred!(mut, ["image", "result", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_DFT_compute_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::DFT* instance, const cv::_InputArray* image, const cv::_OutputArray* result, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*image, *result, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::DFT::compute(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:1132
	// ("cv::cuda::DFT::compute", vec![(pred!(mut, ["image", "result"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_DFT_compute_const__InputArrayR_const__OutputArrayR(cv::cuda::DFT* instance, const cv::_InputArray* image, const cv::_OutputArray* result, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*image, *result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::DFT::to_Algorithm() generated
	// ("cv::cuda::DFT::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_DFT_to_Algorithm(cv::cuda::DFT* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::DFT::delete() generated
	// ("cv::cuda::DFT::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_DFT_delete(cv::cuda::DFT* instance) {
			delete instance;
	}

	// transform(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:780
	// ("cv::cuda::LookUpTable::transform", vec![(pred!(mut, ["src", "dst", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_LookUpTable_transform_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::LookUpTable* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->transform(*src, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::LookUpTable::transform(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaarithm.hpp:780
	// ("cv::cuda::LookUpTable::transform", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_LookUpTable_transform_const__InputArrayR_const__OutputArrayR(cv::cuda::LookUpTable* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->transform(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::LookUpTable::to_Algorithm() generated
	// ("cv::cuda::LookUpTable::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_LookUpTable_to_Algorithm(cv::cuda::LookUpTable* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::LookUpTable::delete() generated
	// ("cv::cuda::LookUpTable::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_LookUpTable_delete(cv::cuda::LookUpTable* instance) {
			delete instance;
	}

}
