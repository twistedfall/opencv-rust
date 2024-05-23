#include "ocvrs_common.hpp"
#include <opencv2/cudafilters.hpp>
#include "cudafilters_types.hpp"

extern "C" {
	// cv::cuda::createBoxFilter(Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafilters.hpp:102
	// ("cv::cuda::createBoxFilter", vec![(pred!(mut, ["srcType", "dstType", "ksize"], ["int", "int", "cv::Size"]), _)]),
	void cv_cuda_createBoxFilter_int_int_Size(int srcType, int dstType, cv::Size* ksize, Result<cv::Ptr<cv::cuda::Filter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createBoxFilter(srcType, dstType, *ksize);
			Ok(new cv::Ptr<cv::cuda::Filter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createBoxFilter(int, int, Size, Point, int, Scalar)(Primitive, Primitive, SimpleClass, SimpleClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafilters.hpp:102
	// ("cv::cuda::createBoxFilter", vec![(pred!(mut, ["srcType", "dstType", "ksize", "anchor", "borderMode", "borderVal"], ["int", "int", "cv::Size", "cv::Point", "int", "cv::Scalar"]), _)]),
	void cv_cuda_createBoxFilter_int_int_Size_Point_int_Scalar(int srcType, int dstType, cv::Size* ksize, cv::Point* anchor, int borderMode, cv::Scalar* borderVal, Result<cv::Ptr<cv::cuda::Filter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createBoxFilter(srcType, dstType, *ksize, *anchor, borderMode, *borderVal);
			Ok(new cv::Ptr<cv::cuda::Filter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::createBoxMaxFilter(Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafilters.hpp:276
	// ("cv::cuda::createBoxMaxFilter", vec![(pred!(mut, ["srcType", "ksize"], ["int", "cv::Size"]), _)]),
	void cv_cuda_createBoxMaxFilter_int_Size(int srcType, cv::Size* ksize, Result<cv::Ptr<cv::cuda::Filter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createBoxMaxFilter(srcType, *ksize);
			Ok(new cv::Ptr<cv::cuda::Filter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createBoxMaxFilter(int, Size, Point, int, Scalar)(Primitive, SimpleClass, SimpleClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafilters.hpp:276
	// ("cv::cuda::createBoxMaxFilter", vec![(pred!(mut, ["srcType", "ksize", "anchor", "borderMode", "borderVal"], ["int", "cv::Size", "cv::Point", "int", "cv::Scalar"]), _)]),
	void cv_cuda_createBoxMaxFilter_int_Size_Point_int_Scalar(int srcType, cv::Size* ksize, cv::Point* anchor, int borderMode, cv::Scalar* borderVal, Result<cv::Ptr<cv::cuda::Filter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createBoxMaxFilter(srcType, *ksize, *anchor, borderMode, *borderVal);
			Ok(new cv::Ptr<cv::cuda::Filter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::createBoxMinFilter(Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafilters.hpp:288
	// ("cv::cuda::createBoxMinFilter", vec![(pred!(mut, ["srcType", "ksize"], ["int", "cv::Size"]), _)]),
	void cv_cuda_createBoxMinFilter_int_Size(int srcType, cv::Size* ksize, Result<cv::Ptr<cv::cuda::Filter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createBoxMinFilter(srcType, *ksize);
			Ok(new cv::Ptr<cv::cuda::Filter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createBoxMinFilter(int, Size, Point, int, Scalar)(Primitive, SimpleClass, SimpleClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafilters.hpp:288
	// ("cv::cuda::createBoxMinFilter", vec![(pred!(mut, ["srcType", "ksize", "anchor", "borderMode", "borderVal"], ["int", "cv::Size", "cv::Point", "int", "cv::Scalar"]), _)]),
	void cv_cuda_createBoxMinFilter_int_Size_Point_int_Scalar(int srcType, cv::Size* ksize, cv::Point* anchor, int borderMode, cv::Scalar* borderVal, Result<cv::Ptr<cv::cuda::Filter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createBoxMinFilter(srcType, *ksize, *anchor, borderMode, *borderVal);
			Ok(new cv::Ptr<cv::cuda::Filter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::createColumnSumFilter(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafilters.hpp:315
	// ("cv::cuda::createColumnSumFilter", vec![(pred!(mut, ["srcType", "dstType", "ksize"], ["int", "int", "int"]), _)]),
	void cv_cuda_createColumnSumFilter_int_int_int(int srcType, int dstType, int ksize, Result<cv::Ptr<cv::cuda::Filter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createColumnSumFilter(srcType, dstType, ksize);
			Ok(new cv::Ptr<cv::cuda::Filter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createColumnSumFilter(int, int, int, int, int, Scalar)(Primitive, Primitive, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafilters.hpp:315
	// ("cv::cuda::createColumnSumFilter", vec![(pred!(mut, ["srcType", "dstType", "ksize", "anchor", "borderMode", "borderVal"], ["int", "int", "int", "int", "int", "cv::Scalar"]), _)]),
	void cv_cuda_createColumnSumFilter_int_int_int_int_int_Scalar(int srcType, int dstType, int ksize, int anchor, int borderMode, cv::Scalar* borderVal, Result<cv::Ptr<cv::cuda::Filter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createColumnSumFilter(srcType, dstType, ksize, anchor, borderMode, *borderVal);
			Ok(new cv::Ptr<cv::cuda::Filter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::createDerivFilter(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafilters.hpp:182
	// ("cv::cuda::createDerivFilter", vec![(pred!(mut, ["srcType", "dstType", "dx", "dy", "ksize"], ["int", "int", "int", "int", "int"]), _)]),
	void cv_cuda_createDerivFilter_int_int_int_int_int(int srcType, int dstType, int dx, int dy, int ksize, Result<cv::Ptr<cv::cuda::Filter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createDerivFilter(srcType, dstType, dx, dy, ksize);
			Ok(new cv::Ptr<cv::cuda::Filter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createDerivFilter(int, int, int, int, int, bool, double, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafilters.hpp:182
	// ("cv::cuda::createDerivFilter", vec![(pred!(mut, ["srcType", "dstType", "dx", "dy", "ksize", "normalize", "scale", "rowBorderMode", "columnBorderMode"], ["int", "int", "int", "int", "int", "bool", "double", "int", "int"]), _)]),
	void cv_cuda_createDerivFilter_int_int_int_int_int_bool_double_int_int(int srcType, int dstType, int dx, int dy, int ksize, bool normalize, double scale, int rowBorderMode, int columnBorderMode, Result<cv::Ptr<cv::cuda::Filter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createDerivFilter(srcType, dstType, dx, dy, ksize, normalize, scale, rowBorderMode, columnBorderMode);
			Ok(new cv::Ptr<cv::cuda::Filter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::createGaussianFilter(Primitive, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafilters.hpp:238
	// ("cv::cuda::createGaussianFilter", vec![(pred!(mut, ["srcType", "dstType", "ksize", "sigma1"], ["int", "int", "cv::Size", "double"]), _)]),
	void cv_cuda_createGaussianFilter_int_int_Size_double(int srcType, int dstType, cv::Size* ksize, double sigma1, Result<cv::Ptr<cv::cuda::Filter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createGaussianFilter(srcType, dstType, *ksize, sigma1);
			Ok(new cv::Ptr<cv::cuda::Filter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createGaussianFilter(int, int, Size, double, double, int, int)(Primitive, Primitive, SimpleClass, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafilters.hpp:238
	// ("cv::cuda::createGaussianFilter", vec![(pred!(mut, ["srcType", "dstType", "ksize", "sigma1", "sigma2", "rowBorderMode", "columnBorderMode"], ["int", "int", "cv::Size", "double", "double", "int", "int"]), _)]),
	void cv_cuda_createGaussianFilter_int_int_Size_double_double_int_int(int srcType, int dstType, cv::Size* ksize, double sigma1, double sigma2, int rowBorderMode, int columnBorderMode, Result<cv::Ptr<cv::cuda::Filter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createGaussianFilter(srcType, dstType, *ksize, sigma1, sigma2, rowBorderMode, columnBorderMode);
			Ok(new cv::Ptr<cv::cuda::Filter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::createLaplacianFilter(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafilters.hpp:139
	// ("cv::cuda::createLaplacianFilter", vec![(pred!(mut, ["srcType", "dstType"], ["int", "int"]), _)]),
	void cv_cuda_createLaplacianFilter_int_int(int srcType, int dstType, Result<cv::Ptr<cv::cuda::Filter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createLaplacianFilter(srcType, dstType);
			Ok(new cv::Ptr<cv::cuda::Filter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createLaplacianFilter(int, int, int, double, int, Scalar)(Primitive, Primitive, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafilters.hpp:139
	// ("cv::cuda::createLaplacianFilter", vec![(pred!(mut, ["srcType", "dstType", "ksize", "scale", "borderMode", "borderVal"], ["int", "int", "int", "double", "int", "cv::Scalar"]), _)]),
	void cv_cuda_createLaplacianFilter_int_int_int_double_int_Scalar(int srcType, int dstType, int ksize, double scale, int borderMode, cv::Scalar* borderVal, Result<cv::Ptr<cv::cuda::Filter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createLaplacianFilter(srcType, dstType, ksize, scale, borderMode, *borderVal);
			Ok(new cv::Ptr<cv::cuda::Filter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::createLinearFilter(Primitive, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafilters.hpp:120
	// ("cv::cuda::createLinearFilter", vec![(pred!(mut, ["srcType", "dstType", "kernel"], ["int", "int", "const cv::_InputArray*"]), _)]),
	void cv_cuda_createLinearFilter_int_int_const__InputArrayR(int srcType, int dstType, const cv::_InputArray* kernel, Result<cv::Ptr<cv::cuda::Filter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createLinearFilter(srcType, dstType, *kernel);
			Ok(new cv::Ptr<cv::cuda::Filter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createLinearFilter(int, int, InputArray, Point, int, Scalar)(Primitive, Primitive, InputArray, SimpleClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafilters.hpp:120
	// ("cv::cuda::createLinearFilter", vec![(pred!(mut, ["srcType", "dstType", "kernel", "anchor", "borderMode", "borderVal"], ["int", "int", "const cv::_InputArray*", "cv::Point", "int", "cv::Scalar"]), _)]),
	void cv_cuda_createLinearFilter_int_int_const__InputArrayR_Point_int_Scalar(int srcType, int dstType, const cv::_InputArray* kernel, cv::Point* anchor, int borderMode, cv::Scalar* borderVal, Result<cv::Ptr<cv::cuda::Filter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createLinearFilter(srcType, dstType, *kernel, *anchor, borderMode, *borderVal);
			Ok(new cv::Ptr<cv::cuda::Filter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::createMedianFilter(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafilters.hpp:333
	// ("cv::cuda::createMedianFilter", vec![(pred!(mut, ["srcType", "windowSize"], ["int", "int"]), _)]),
	void cv_cuda_createMedianFilter_int_int(int srcType, int windowSize, Result<cv::Ptr<cv::cuda::Filter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createMedianFilter(srcType, windowSize);
			Ok(new cv::Ptr<cv::cuda::Filter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createMedianFilter(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafilters.hpp:333
	// ("cv::cuda::createMedianFilter", vec![(pred!(mut, ["srcType", "windowSize", "partition"], ["int", "int", "int"]), _)]),
	void cv_cuda_createMedianFilter_int_int_int(int srcType, int windowSize, int partition, Result<cv::Ptr<cv::cuda::Filter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createMedianFilter(srcType, windowSize, partition);
			Ok(new cv::Ptr<cv::cuda::Filter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::createMorphologyFilter(Primitive, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafilters.hpp:263
	// ("cv::cuda::createMorphologyFilter", vec![(pred!(mut, ["op", "srcType", "kernel"], ["int", "int", "const cv::_InputArray*"]), _)]),
	void cv_cuda_createMorphologyFilter_int_int_const__InputArrayR(int op, int srcType, const cv::_InputArray* kernel, Result<cv::Ptr<cv::cuda::Filter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createMorphologyFilter(op, srcType, *kernel);
			Ok(new cv::Ptr<cv::cuda::Filter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createMorphologyFilter(int, int, InputArray, Point, int)(Primitive, Primitive, InputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafilters.hpp:263
	// ("cv::cuda::createMorphologyFilter", vec![(pred!(mut, ["op", "srcType", "kernel", "anchor", "iterations"], ["int", "int", "const cv::_InputArray*", "cv::Point", "int"]), _)]),
	void cv_cuda_createMorphologyFilter_int_int_const__InputArrayR_Point_int(int op, int srcType, const cv::_InputArray* kernel, cv::Point* anchor, int iterations, Result<cv::Ptr<cv::cuda::Filter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createMorphologyFilter(op, srcType, *kernel, *anchor, iterations);
			Ok(new cv::Ptr<cv::cuda::Filter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::createRowSumFilter(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafilters.hpp:304
	// ("cv::cuda::createRowSumFilter", vec![(pred!(mut, ["srcType", "dstType", "ksize"], ["int", "int", "int"]), _)]),
	void cv_cuda_createRowSumFilter_int_int_int(int srcType, int dstType, int ksize, Result<cv::Ptr<cv::cuda::Filter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createRowSumFilter(srcType, dstType, ksize);
			Ok(new cv::Ptr<cv::cuda::Filter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createRowSumFilter(int, int, int, int, int, Scalar)(Primitive, Primitive, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafilters.hpp:304
	// ("cv::cuda::createRowSumFilter", vec![(pred!(mut, ["srcType", "dstType", "ksize", "anchor", "borderMode", "borderVal"], ["int", "int", "int", "int", "int", "cv::Scalar"]), _)]),
	void cv_cuda_createRowSumFilter_int_int_int_int_int_Scalar(int srcType, int dstType, int ksize, int anchor, int borderMode, cv::Scalar* borderVal, Result<cv::Ptr<cv::cuda::Filter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createRowSumFilter(srcType, dstType, ksize, anchor, borderMode, *borderVal);
			Ok(new cv::Ptr<cv::cuda::Filter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::createScharrFilter(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafilters.hpp:218
	// ("cv::cuda::createScharrFilter", vec![(pred!(mut, ["srcType", "dstType", "dx", "dy"], ["int", "int", "int", "int"]), _)]),
	void cv_cuda_createScharrFilter_int_int_int_int(int srcType, int dstType, int dx, int dy, Result<cv::Ptr<cv::cuda::Filter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createScharrFilter(srcType, dstType, dx, dy);
			Ok(new cv::Ptr<cv::cuda::Filter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createScharrFilter(int, int, int, int, double, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafilters.hpp:218
	// ("cv::cuda::createScharrFilter", vec![(pred!(mut, ["srcType", "dstType", "dx", "dy", "scale", "rowBorderMode", "columnBorderMode"], ["int", "int", "int", "int", "double", "int", "int"]), _)]),
	void cv_cuda_createScharrFilter_int_int_int_int_double_int_int(int srcType, int dstType, int dx, int dy, double scale, int rowBorderMode, int columnBorderMode, Result<cv::Ptr<cv::cuda::Filter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createScharrFilter(srcType, dstType, dx, dy, scale, rowBorderMode, columnBorderMode);
			Ok(new cv::Ptr<cv::cuda::Filter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::createSeparableLinearFilter(Primitive, Primitive, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafilters.hpp:161
	// ("cv::cuda::createSeparableLinearFilter", vec![(pred!(mut, ["srcType", "dstType", "rowKernel", "columnKernel"], ["int", "int", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_cuda_createSeparableLinearFilter_int_int_const__InputArrayR_const__InputArrayR(int srcType, int dstType, const cv::_InputArray* rowKernel, const cv::_InputArray* columnKernel, Result<cv::Ptr<cv::cuda::Filter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createSeparableLinearFilter(srcType, dstType, *rowKernel, *columnKernel);
			Ok(new cv::Ptr<cv::cuda::Filter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createSeparableLinearFilter(int, int, InputArray, InputArray, Point, int, int)(Primitive, Primitive, InputArray, InputArray, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafilters.hpp:161
	// ("cv::cuda::createSeparableLinearFilter", vec![(pred!(mut, ["srcType", "dstType", "rowKernel", "columnKernel", "anchor", "rowBorderMode", "columnBorderMode"], ["int", "int", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Point", "int", "int"]), _)]),
	void cv_cuda_createSeparableLinearFilter_int_int_const__InputArrayR_const__InputArrayR_Point_int_int(int srcType, int dstType, const cv::_InputArray* rowKernel, const cv::_InputArray* columnKernel, cv::Point* anchor, int rowBorderMode, int columnBorderMode, Result<cv::Ptr<cv::cuda::Filter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createSeparableLinearFilter(srcType, dstType, *rowKernel, *columnKernel, *anchor, rowBorderMode, columnBorderMode);
			Ok(new cv::Ptr<cv::cuda::Filter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::createSobelFilter(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafilters.hpp:201
	// ("cv::cuda::createSobelFilter", vec![(pred!(mut, ["srcType", "dstType", "dx", "dy"], ["int", "int", "int", "int"]), _)]),
	void cv_cuda_createSobelFilter_int_int_int_int(int srcType, int dstType, int dx, int dy, Result<cv::Ptr<cv::cuda::Filter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createSobelFilter(srcType, dstType, dx, dy);
			Ok(new cv::Ptr<cv::cuda::Filter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createSobelFilter(int, int, int, int, int, double, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafilters.hpp:201
	// ("cv::cuda::createSobelFilter", vec![(pred!(mut, ["srcType", "dstType", "dx", "dy", "ksize", "scale", "rowBorderMode", "columnBorderMode"], ["int", "int", "int", "int", "int", "double", "int", "int"]), _)]),
	void cv_cuda_createSobelFilter_int_int_int_int_int_double_int_int(int srcType, int dstType, int dx, int dy, int ksize, double scale, int rowBorderMode, int columnBorderMode, Result<cv::Ptr<cv::cuda::Filter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::Filter> ret = cv::cuda::createSobelFilter(srcType, dstType, dx, dy, ksize, scale, rowBorderMode, columnBorderMode);
			Ok(new cv::Ptr<cv::cuda::Filter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// apply(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafilters.hpp:84
	// ("cv::cuda::Filter::apply", vec![(pred!(mut, ["src", "dst", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_Filter_apply_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::Filter* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*src, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::Filter::apply(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafilters.hpp:84
	// ("cv::cuda::Filter::apply", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_Filter_apply_const__InputArrayR_const__OutputArrayR(cv::cuda::Filter* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::Filter::to_Algorithm() generated
	// ("cv::cuda::Filter::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_Filter_to_Algorithm(cv::cuda::Filter* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::Filter::delete() generated
	// ("cv::cuda::Filter::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_Filter_delete(cv::cuda::Filter* instance) {
			delete instance;
	}

}
