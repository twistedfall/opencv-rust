#include "dnn.hpp"
#include "dnn_types.hpp"

extern "C" {
	// cv::dnn::NMSBoxesBatched(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1352
	// ("cv::dnn::NMSBoxesBatched", vec![(pred!(mut, ["bboxes", "scores", "class_ids", "score_threshold", "nms_threshold", "indices"], ["const std::vector<cv::Rect2d>*", "const std::vector<float>*", "const std::vector<int>*", "const float", "const float", "std::vector<int>*"]), _)]),
	void cv_dnn_NMSBoxesBatched_const_vectorLRect2dGR_const_vectorLfloatGR_const_vectorLintGR_const_float_const_float_vectorLintGR(const std::vector<cv::Rect2d>* bboxes, const std::vector<float>* scores, const std::vector<int>* class_ids, const float score_threshold, const float nms_threshold, std::vector<int>* indices, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::NMSBoxesBatched(*bboxes, *scores, *class_ids, score_threshold, nms_threshold, *indices);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// NMSBoxesBatched(const std::vector<Rect2d> &, const std::vector<float> &, const std::vector<int> &, const float, const float, std::vector<int> &, const float, const int)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1352
	// ("cv::dnn::NMSBoxesBatched", vec![(pred!(mut, ["bboxes", "scores", "class_ids", "score_threshold", "nms_threshold", "indices", "eta", "top_k"], ["const std::vector<cv::Rect2d>*", "const std::vector<float>*", "const std::vector<int>*", "const float", "const float", "std::vector<int>*", "const float", "const int"]), _)]),
	void cv_dnn_NMSBoxesBatched_const_vectorLRect2dGR_const_vectorLfloatGR_const_vectorLintGR_const_float_const_float_vectorLintGR_const_float_const_int(const std::vector<cv::Rect2d>* bboxes, const std::vector<float>* scores, const std::vector<int>* class_ids, const float score_threshold, const float nms_threshold, std::vector<int>* indices, const float eta, const int top_k, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::NMSBoxesBatched(*bboxes, *scores, *class_ids, score_threshold, nms_threshold, *indices, eta, top_k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::NMSBoxesBatched(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1347
	// ("cv::dnn::NMSBoxesBatched", vec![(pred!(mut, ["bboxes", "scores", "class_ids", "score_threshold", "nms_threshold", "indices"], ["const std::vector<cv::Rect>*", "const std::vector<float>*", "const std::vector<int>*", "const float", "const float", "std::vector<int>*"]), _)]),
	void cv_dnn_NMSBoxesBatched_const_vectorLRectGR_const_vectorLfloatGR_const_vectorLintGR_const_float_const_float_vectorLintGR(const std::vector<cv::Rect>* bboxes, const std::vector<float>* scores, const std::vector<int>* class_ids, const float score_threshold, const float nms_threshold, std::vector<int>* indices, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::NMSBoxesBatched(*bboxes, *scores, *class_ids, score_threshold, nms_threshold, *indices);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// NMSBoxesBatched(const std::vector<Rect> &, const std::vector<float> &, const std::vector<int> &, const float, const float, std::vector<int> &, const float, const int)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1347
	// ("cv::dnn::NMSBoxesBatched", vec![(pred!(mut, ["bboxes", "scores", "class_ids", "score_threshold", "nms_threshold", "indices", "eta", "top_k"], ["const std::vector<cv::Rect>*", "const std::vector<float>*", "const std::vector<int>*", "const float", "const float", "std::vector<int>*", "const float", "const int"]), _)]),
	void cv_dnn_NMSBoxesBatched_const_vectorLRectGR_const_vectorLfloatGR_const_vectorLintGR_const_float_const_float_vectorLintGR_const_float_const_int(const std::vector<cv::Rect>* bboxes, const std::vector<float>* scores, const std::vector<int>* class_ids, const float score_threshold, const float nms_threshold, std::vector<int>* indices, const float eta, const int top_k, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::NMSBoxesBatched(*bboxes, *scores, *class_ids, score_threshold, nms_threshold, *indices, eta, top_k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::NMSBoxes(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1326
	// ("cv::dnn::NMSBoxes", vec![(pred!(mut, ["bboxes", "scores", "score_threshold", "nms_threshold", "indices"], ["const std::vector<cv::Rect2d>*", "const std::vector<float>*", "const float", "const float", "std::vector<int>*"]), _)]),
	void cv_dnn_NMSBoxes_const_vectorLRect2dGR_const_vectorLfloatGR_const_float_const_float_vectorLintGR(const std::vector<cv::Rect2d>* bboxes, const std::vector<float>* scores, const float score_threshold, const float nms_threshold, std::vector<int>* indices, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::NMSBoxes(*bboxes, *scores, score_threshold, nms_threshold, *indices);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// NMSBoxes(const std::vector<Rect2d> &, const std::vector<float> &, const float, const float, std::vector<int> &, const float, const int)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1326
	// ("cv::dnn::NMSBoxes", vec![(pred!(mut, ["bboxes", "scores", "score_threshold", "nms_threshold", "indices", "eta", "top_k"], ["const std::vector<cv::Rect2d>*", "const std::vector<float>*", "const float", "const float", "std::vector<int>*", "const float", "const int"]), _)]),
	void cv_dnn_NMSBoxes_const_vectorLRect2dGR_const_vectorLfloatGR_const_float_const_float_vectorLintGR_const_float_const_int(const std::vector<cv::Rect2d>* bboxes, const std::vector<float>* scores, const float score_threshold, const float nms_threshold, std::vector<int>* indices, const float eta, const int top_k, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::NMSBoxes(*bboxes, *scores, score_threshold, nms_threshold, *indices, eta, top_k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::NMSBoxes(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1321
	// ("cv::dnn::NMSBoxes", vec![(pred!(mut, ["bboxes", "scores", "score_threshold", "nms_threshold", "indices"], ["const std::vector<cv::Rect>*", "const std::vector<float>*", "const float", "const float", "std::vector<int>*"]), _)]),
	void cv_dnn_NMSBoxes_const_vectorLRectGR_const_vectorLfloatGR_const_float_const_float_vectorLintGR(const std::vector<cv::Rect>* bboxes, const std::vector<float>* scores, const float score_threshold, const float nms_threshold, std::vector<int>* indices, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::NMSBoxes(*bboxes, *scores, score_threshold, nms_threshold, *indices);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// NMSBoxes(const std::vector<Rect> &, const std::vector<float> &, const float, const float, std::vector<int> &, const float, const int)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1321
	// ("cv::dnn::NMSBoxes", vec![(pred!(mut, ["bboxes", "scores", "score_threshold", "nms_threshold", "indices", "eta", "top_k"], ["const std::vector<cv::Rect>*", "const std::vector<float>*", "const float", "const float", "std::vector<int>*", "const float", "const int"]), _)]),
	void cv_dnn_NMSBoxes_const_vectorLRectGR_const_vectorLfloatGR_const_float_const_float_vectorLintGR_const_float_const_int(const std::vector<cv::Rect>* bboxes, const std::vector<float>* scores, const float score_threshold, const float nms_threshold, std::vector<int>* indices, const float eta, const int top_k, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::NMSBoxes(*bboxes, *scores, score_threshold, nms_threshold, *indices, eta, top_k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::NMSBoxes(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1331
	// ("cv::dnn::NMSBoxes", vec![(pred!(mut, ["bboxes", "scores", "score_threshold", "nms_threshold", "indices"], ["const std::vector<cv::RotatedRect>*", "const std::vector<float>*", "const float", "const float", "std::vector<int>*"]), _)]),
	void cv_dnn_NMSBoxes_const_vectorLRotatedRectGR_const_vectorLfloatGR_const_float_const_float_vectorLintGR(const std::vector<cv::RotatedRect>* bboxes, const std::vector<float>* scores, const float score_threshold, const float nms_threshold, std::vector<int>* indices, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::NMSBoxes(*bboxes, *scores, score_threshold, nms_threshold, *indices);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// NMSBoxes(const std::vector<RotatedRect> &, const std::vector<float> &, const float, const float, std::vector<int> &, const float, const int)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1331
	// ("cv::dnn::NMSBoxes", vec![(pred!(mut, ["bboxes", "scores", "score_threshold", "nms_threshold", "indices", "eta", "top_k"], ["const std::vector<cv::RotatedRect>*", "const std::vector<float>*", "const float", "const float", "std::vector<int>*", "const float", "const int"]), _)]),
	void cv_dnn_NMSBoxes_const_vectorLRotatedRectGR_const_vectorLfloatGR_const_float_const_float_vectorLintGR_const_float_const_int(const std::vector<cv::RotatedRect>* bboxes, const std::vector<float>* scores, const float score_threshold, const float nms_threshold, std::vector<int>* indices, const float eta, const int top_k, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::NMSBoxes(*bboxes, *scores, score_threshold, nms_threshold, *indices, eta, top_k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::blobFromImageWithParams(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1258
	// ("cv::dnn::blobFromImageWithParams", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
	void cv_dnn_blobFromImageWithParams_const__InputArrayR(const cv::_InputArray* image, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::dnn::blobFromImageWithParams(*image);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// blobFromImageWithParams(InputArray, const Image2BlobParams &)(InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1258
	// ("cv::dnn::blobFromImageWithParams", vec![(pred!(mut, ["image", "param"], ["const cv::_InputArray*", "const cv::dnn::Image2BlobParams*"]), _)]),
	void cv_dnn_blobFromImageWithParams_const__InputArrayR_const_Image2BlobParamsR(const cv::_InputArray* image, const cv::dnn::Image2BlobParams* param, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::dnn::blobFromImageWithParams(*image, *param);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::blobFromImageWithParams(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1261
	// ("cv::dnn::blobFromImageWithParams", vec![(pred!(mut, ["image", "blob"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_dnn_blobFromImageWithParams_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* image, const cv::_OutputArray* blob, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::blobFromImageWithParams(*image, *blob);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// blobFromImageWithParams(InputArray, OutputArray, const Image2BlobParams &)(InputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1261
	// ("cv::dnn::blobFromImageWithParams", vec![(pred!(mut, ["image", "blob", "param"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::dnn::Image2BlobParams*"]), _)]),
	void cv_dnn_blobFromImageWithParams_const__InputArrayR_const__OutputArrayR_const_Image2BlobParamsR(const cv::_InputArray* image, const cv::_OutputArray* blob, const cv::dnn::Image2BlobParams* param, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::blobFromImageWithParams(*image, *blob, *param);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::blobFromImage(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1148
	// ("cv::dnn::blobFromImage", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
	void cv_dnn_blobFromImage_const__InputArrayR(const cv::_InputArray* image, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::dnn::blobFromImage(*image);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::blobFromImage(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1156
	// ("cv::dnn::blobFromImage", vec![(pred!(mut, ["image", "blob"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_dnn_blobFromImage_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* image, const cv::_OutputArray* blob, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::blobFromImage(*image, *blob);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// blobFromImage(InputArray, OutputArray, double, const Size &, const Scalar &, bool, bool, int)(InputArray, OutputArray, Primitive, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1156
	// ("cv::dnn::blobFromImage", vec![(pred!(mut, ["image", "blob", "scalefactor", "size", "mean", "swapRB", "crop", "ddepth"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "const cv::Size*", "const cv::Scalar*", "bool", "bool", "int"]), _)]),
	void cv_dnn_blobFromImage_const__InputArrayR_const__OutputArrayR_double_const_SizeR_const_ScalarR_bool_bool_int(const cv::_InputArray* image, const cv::_OutputArray* blob, double scalefactor, const cv::Size* size, const cv::Scalar* mean, bool swapRB, bool crop, int ddepth, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::blobFromImage(*image, *blob, scalefactor, *size, *mean, swapRB, crop, ddepth);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// blobFromImage(InputArray, double, const Size &, const Scalar &, bool, bool, int)(InputArray, Primitive, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1148
	// ("cv::dnn::blobFromImage", vec![(pred!(mut, ["image", "scalefactor", "size", "mean", "swapRB", "crop", "ddepth"], ["const cv::_InputArray*", "double", "const cv::Size*", "const cv::Scalar*", "bool", "bool", "int"]), _)]),
	void cv_dnn_blobFromImage_const__InputArrayR_double_const_SizeR_const_ScalarR_bool_bool_int(const cv::_InputArray* image, double scalefactor, const cv::Size* size, const cv::Scalar* mean, bool swapRB, bool crop, int ddepth, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::dnn::blobFromImage(*image, scalefactor, *size, *mean, swapRB, crop, ddepth);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::blobFromImagesWithParams(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1272
	// ("cv::dnn::blobFromImagesWithParams", vec![(pred!(mut, ["images"], ["const cv::_InputArray*"]), _)]),
	void cv_dnn_blobFromImagesWithParams_const__InputArrayR(const cv::_InputArray* images, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::dnn::blobFromImagesWithParams(*images);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// blobFromImagesWithParams(InputArrayOfArrays, const Image2BlobParams &)(InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1272
	// ("cv::dnn::blobFromImagesWithParams", vec![(pred!(mut, ["images", "param"], ["const cv::_InputArray*", "const cv::dnn::Image2BlobParams*"]), _)]),
	void cv_dnn_blobFromImagesWithParams_const__InputArrayR_const_Image2BlobParamsR(const cv::_InputArray* images, const cv::dnn::Image2BlobParams* param, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::dnn::blobFromImagesWithParams(*images, *param);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::blobFromImagesWithParams(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1275
	// ("cv::dnn::blobFromImagesWithParams", vec![(pred!(mut, ["images", "blob"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_dnn_blobFromImagesWithParams_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* images, const cv::_OutputArray* blob, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::blobFromImagesWithParams(*images, *blob);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// blobFromImagesWithParams(InputArrayOfArrays, OutputArray, const Image2BlobParams &)(InputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1275
	// ("cv::dnn::blobFromImagesWithParams", vec![(pred!(mut, ["images", "blob", "param"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::dnn::Image2BlobParams*"]), _)]),
	void cv_dnn_blobFromImagesWithParams_const__InputArrayR_const__OutputArrayR_const_Image2BlobParamsR(const cv::_InputArray* images, const cv::_OutputArray* blob, const cv::dnn::Image2BlobParams* param, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::blobFromImagesWithParams(*images, *blob, *param);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::blobFromImages(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1181
	// ("cv::dnn::blobFromImages", vec![(pred!(mut, ["images"], ["const cv::_InputArray*"]), _)]),
	void cv_dnn_blobFromImages_const__InputArrayR(const cv::_InputArray* images, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::dnn::blobFromImages(*images);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::blobFromImages(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1189
	// ("cv::dnn::blobFromImages", vec![(pred!(mut, ["images", "blob"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_dnn_blobFromImages_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* images, const cv::_OutputArray* blob, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::blobFromImages(*images, *blob);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// blobFromImages(InputArrayOfArrays, OutputArray, double, Size, const Scalar &, bool, bool, int)(InputArray, OutputArray, Primitive, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1189
	// ("cv::dnn::blobFromImages", vec![(pred!(mut, ["images", "blob", "scalefactor", "size", "mean", "swapRB", "crop", "ddepth"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "cv::Size", "const cv::Scalar*", "bool", "bool", "int"]), _)]),
	void cv_dnn_blobFromImages_const__InputArrayR_const__OutputArrayR_double_Size_const_ScalarR_bool_bool_int(const cv::_InputArray* images, const cv::_OutputArray* blob, double scalefactor, cv::Size* size, const cv::Scalar* mean, bool swapRB, bool crop, int ddepth, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::blobFromImages(*images, *blob, scalefactor, *size, *mean, swapRB, crop, ddepth);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// blobFromImages(InputArrayOfArrays, double, Size, const Scalar &, bool, bool, int)(InputArray, Primitive, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1181
	// ("cv::dnn::blobFromImages", vec![(pred!(mut, ["images", "scalefactor", "size", "mean", "swapRB", "crop", "ddepth"], ["const cv::_InputArray*", "double", "cv::Size", "const cv::Scalar*", "bool", "bool", "int"]), _)]),
	void cv_dnn_blobFromImages_const__InputArrayR_double_Size_const_ScalarR_bool_bool_int(const cv::_InputArray* images, double scalefactor, cv::Size* size, const cv::Scalar* mean, bool swapRB, bool crop, int ddepth, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::dnn::blobFromImages(*images, scalefactor, *size, *mean, swapRB, crop, ddepth);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// concat(const MatShape &, const MatShape &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/shape_utils.hpp:206
	// ("cv::dnn::concat", vec![(pred!(mut, ["a", "b"], ["const cv::dnn::MatShape*", "const cv::dnn::MatShape*"]), _)]),
	void cv_dnn_concat_const_MatShapeR_const_MatShapeR(const cv::dnn::MatShape* a, const cv::dnn::MatShape* b, Result<cv::dnn::MatShape*>* ocvrs_return) {
		try {
			cv::dnn::MatShape ret = cv::dnn::concat(*a, *b);
			Ok(new cv::dnn::MatShape(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// enableModelDiagnostics(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:138
	// ("cv::dnn::enableModelDiagnostics", vec![(pred!(mut, ["isDiagnosticsMode"], ["bool"]), _)]),
	void cv_dnn_enableModelDiagnostics_bool(bool isDiagnosticsMode, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::enableModelDiagnostics(isDiagnosticsMode);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAvailableBackends()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:125
	// ("cv::dnn::getAvailableBackends", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_getAvailableBackends(Result<std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>*>* ocvrs_return) {
		try {
			std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>> ret = cv::dnn::getAvailableBackends();
			Ok(new std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAvailableTargets(dnn::Backend)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:126
	// ("cv::dnn::getAvailableTargets", vec![(pred!(mut, ["be"], ["cv::dnn::Backend"]), _)]),
	void cv_dnn_getAvailableTargets_Backend(cv::dnn::Backend be, Result<std::vector<cv::dnn::Target>*>* ocvrs_return) {
		try {
			std::vector<cv::dnn::Target> ret = cv::dnn::getAvailableTargets(be);
			Ok(new std::vector<cv::dnn::Target>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getInferenceEngineBackendType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/utils/inference_engine.hpp:31
	// ("cv::dnn::getInferenceEngineBackendType", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_getInferenceEngineBackendType(Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::dnn::getInferenceEngineBackendType();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getInferenceEngineCPUType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/utils/inference_engine.hpp:72
	// ("cv::dnn::getInferenceEngineCPUType", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_getInferenceEngineCPUType(Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::dnn::getInferenceEngineCPUType();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getInferenceEngineVPUType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/utils/inference_engine.hpp:66
	// ("cv::dnn::getInferenceEngineVPUType", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_getInferenceEngineVPUType(Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::dnn::getInferenceEngineVPUType();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPlane(const Mat &, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/shape_utils.hpp:108
	// ("cv::dnn::getPlane", vec![(pred!(mut, ["m", "n", "cn"], ["const cv::Mat*", "int", "int"]), _)]),
	void cv_dnn_getPlane_const_MatR_int_int(const cv::Mat* m, int n, int cn, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::dnn::getPlane(*m, n, cn);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imagesFromBlob(const cv::Mat &, OutputArrayOfArrays)(TraitClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1285
	// ("cv::dnn::imagesFromBlob", vec![(pred!(mut, ["blob_", "images_"], ["const cv::Mat*", "const cv::_OutputArray*"]), _)]),
	void cv_dnn_imagesFromBlob_const_MatR_const__OutputArrayR(const cv::Mat* blob_, const cv::_OutputArray* images_, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::imagesFromBlob(*blob_, *images_);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::readNetFromCaffe(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:926
	// ("cv::dnn::readNetFromCaffe", vec![(pred!(mut, ["prototxt"], ["const cv::String*"]), _)]),
	void cv_dnn_readNetFromCaffe_const_StringR(const char* prototxt, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromCaffe(std::string(prototxt));
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromCaffe(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:926
	// ("cv::dnn::readNetFromCaffe", vec![(pred!(mut, ["prototxt", "caffeModel"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_dnn_readNetFromCaffe_const_StringR_const_StringR(const char* prototxt, const char* caffeModel, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromCaffe(std::string(prototxt), std::string(caffeModel));
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::readNetFromCaffe(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:945
	// ("cv::dnn::readNetFromCaffe", vec![(pred!(mut, ["bufferProto", "lenProto"], ["const char*", "size_t"]), _)]),
	void cv_dnn_readNetFromCaffe_const_charX_size_t(const char* bufferProto, size_t lenProto, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromCaffe(bufferProto, lenProto);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromCaffe(const char *, size_t, const char *, size_t)(InString, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:945
	// ("cv::dnn::readNetFromCaffe", vec![(pred!(mut, ["bufferProto", "lenProto", "bufferModel", "lenModel"], ["const char*", "size_t", "const char*", "size_t"]), _)]),
	void cv_dnn_readNetFromCaffe_const_charX_size_t_const_charX_size_t(const char* bufferProto, size_t lenProto, const char* bufferModel, size_t lenModel, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromCaffe(bufferProto, lenProto, bufferModel, lenModel);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::readNetFromCaffe(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:933
	// ("cv::dnn::readNetFromCaffe", vec![(pred!(mut, ["bufferProto"], ["const std::vector<unsigned char>*"]), _)]),
	void cv_dnn_readNetFromCaffe_const_vectorLunsigned_charGR(const std::vector<unsigned char>* bufferProto, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromCaffe(*bufferProto);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromCaffe(const std::vector<uchar> &, const std::vector<uchar> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:933
	// ("cv::dnn::readNetFromCaffe", vec![(pred!(mut, ["bufferProto", "bufferModel"], ["const std::vector<unsigned char>*", "const std::vector<unsigned char>*"]), _)]),
	void cv_dnn_readNetFromCaffe_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR(const std::vector<unsigned char>* bufferProto, const std::vector<unsigned char>* bufferModel, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromCaffe(*bufferProto, *bufferModel);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::readNetFromDarknet(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:901
	// ("cv::dnn::readNetFromDarknet", vec![(pred!(mut, ["cfgFile"], ["const cv::String*"]), _)]),
	void cv_dnn_readNetFromDarknet_const_StringR(const char* cfgFile, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromDarknet(std::string(cfgFile));
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromDarknet(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:901
	// ("cv::dnn::readNetFromDarknet", vec![(pred!(mut, ["cfgFile", "darknetModel"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_dnn_readNetFromDarknet_const_StringR_const_StringR(const char* cfgFile, const char* darknetModel, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromDarknet(std::string(cfgFile), std::string(darknetModel));
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::readNetFromDarknet(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:918
	// ("cv::dnn::readNetFromDarknet", vec![(pred!(mut, ["bufferCfg", "lenCfg"], ["const char*", "size_t"]), _)]),
	void cv_dnn_readNetFromDarknet_const_charX_size_t(const char* bufferCfg, size_t lenCfg, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromDarknet(bufferCfg, lenCfg);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromDarknet(const char *, size_t, const char *, size_t)(InString, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:918
	// ("cv::dnn::readNetFromDarknet", vec![(pred!(mut, ["bufferCfg", "lenCfg", "bufferModel", "lenModel"], ["const char*", "size_t", "const char*", "size_t"]), _)]),
	void cv_dnn_readNetFromDarknet_const_charX_size_t_const_charX_size_t(const char* bufferCfg, size_t lenCfg, const char* bufferModel, size_t lenModel, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromDarknet(bufferCfg, lenCfg, bufferModel, lenModel);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::readNetFromDarknet(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:908
	// ("cv::dnn::readNetFromDarknet", vec![(pred!(mut, ["bufferCfg"], ["const std::vector<unsigned char>*"]), _)]),
	void cv_dnn_readNetFromDarknet_const_vectorLunsigned_charGR(const std::vector<unsigned char>* bufferCfg, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromDarknet(*bufferCfg);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromDarknet(const std::vector<uchar> &, const std::vector<uchar> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:908
	// ("cv::dnn::readNetFromDarknet", vec![(pred!(mut, ["bufferCfg", "bufferModel"], ["const std::vector<unsigned char>*", "const std::vector<unsigned char>*"]), _)]),
	void cv_dnn_readNetFromDarknet_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR(const std::vector<unsigned char>* bufferCfg, const std::vector<unsigned char>* bufferModel, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromDarknet(*bufferCfg, *bufferModel);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::readNetFromModelOptimizer(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1075
	// ("cv::dnn::readNetFromModelOptimizer", vec![(pred!(mut, ["xml"], ["const cv::String*"]), _)]),
	void cv_dnn_readNetFromModelOptimizer_const_StringR(const char* xml, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromModelOptimizer(std::string(xml));
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromModelOptimizer(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1075
	// ("cv::dnn::readNetFromModelOptimizer", vec![(pred!(mut, ["xml", "bin"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_dnn_readNetFromModelOptimizer_const_StringR_const_StringR(const char* xml, const char* bin, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromModelOptimizer(std::string(xml), std::string(bin));
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromModelOptimizer(const uchar *, size_t, const uchar *, size_t)(VariableArray, Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1097
	// ("cv::dnn::readNetFromModelOptimizer", vec![(pred!(mut, ["bufferModelConfigPtr", "bufferModelConfigSize", "bufferWeightsPtr", "bufferWeightsSize"], ["const unsigned char*", "size_t", "const unsigned char*", "size_t"]), _)]),
	void cv_dnn_readNetFromModelOptimizer_const_unsigned_charX_size_t_const_unsigned_charX_size_t(const unsigned char* bufferModelConfigPtr, size_t bufferModelConfigSize, const unsigned char* bufferWeightsPtr, size_t bufferWeightsSize, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromModelOptimizer(bufferModelConfigPtr, bufferModelConfigSize, bufferWeightsPtr, bufferWeightsSize);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromModelOptimizer(const std::vector<uchar> &, const std::vector<uchar> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1085
	// ("cv::dnn::readNetFromModelOptimizer", vec![(pred!(mut, ["bufferModelConfig", "bufferWeights"], ["const std::vector<unsigned char>*", "const std::vector<unsigned char>*"]), _)]),
	void cv_dnn_readNetFromModelOptimizer_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR(const std::vector<unsigned char>* bufferModelConfig, const std::vector<unsigned char>* bufferWeights, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromModelOptimizer(*bufferModelConfig, *bufferWeights);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromONNX(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1104
	// ("cv::dnn::readNetFromONNX", vec![(pred!(mut, ["onnxFile"], ["const cv::String*"]), _)]),
	void cv_dnn_readNetFromONNX_const_StringR(const char* onnxFile, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromONNX(std::string(onnxFile));
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromONNX(const char *, size_t)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1113
	// ("cv::dnn::readNetFromONNX", vec![(pred!(mut, ["buffer", "sizeBuffer"], ["const char*", "size_t"]), _)]),
	void cv_dnn_readNetFromONNX_const_charX_size_t(const char* buffer, size_t sizeBuffer, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromONNX(buffer, sizeBuffer);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromONNX(const std::vector<uchar> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1121
	// ("cv::dnn::readNetFromONNX", vec![(pred!(mut, ["buffer"], ["const std::vector<unsigned char>*"]), _)]),
	void cv_dnn_readNetFromONNX_const_vectorLunsigned_charGR(const std::vector<unsigned char>* buffer, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromONNX(*buffer);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromTFLite(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:980
	// ("cv::dnn::readNetFromTFLite", vec![(pred!(mut, ["model"], ["const cv::String*"]), _)]),
	void cv_dnn_readNetFromTFLite_const_StringR(const char* model, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromTFLite(std::string(model));
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromTFLite(const char *, size_t)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:994
	// ("cv::dnn::readNetFromTFLite", vec![(pred!(mut, ["bufferModel", "lenModel"], ["const char*", "size_t"]), _)]),
	void cv_dnn_readNetFromTFLite_const_charX_size_t(const char* bufferModel, size_t lenModel, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromTFLite(bufferModel, lenModel);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromTFLite(const std::vector<uchar> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:986
	// ("cv::dnn::readNetFromTFLite", vec![(pred!(mut, ["bufferModel"], ["const std::vector<unsigned char>*"]), _)]),
	void cv_dnn_readNetFromTFLite_const_vectorLunsigned_charGR(const std::vector<unsigned char>* bufferModel, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromTFLite(*bufferModel);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::readNetFromTensorflow(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:955
	// ("cv::dnn::readNetFromTensorflow", vec![(pred!(mut, ["model"], ["const cv::String*"]), _)]),
	void cv_dnn_readNetFromTensorflow_const_StringR(const char* model, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromTensorflow(std::string(model));
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromTensorflow(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:955
	// ("cv::dnn::readNetFromTensorflow", vec![(pred!(mut, ["model", "config"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_dnn_readNetFromTensorflow_const_StringR_const_StringR(const char* model, const char* config, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromTensorflow(std::string(model), std::string(config));
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::readNetFromTensorflow(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:973
	// ("cv::dnn::readNetFromTensorflow", vec![(pred!(mut, ["bufferModel", "lenModel"], ["const char*", "size_t"]), _)]),
	void cv_dnn_readNetFromTensorflow_const_charX_size_t(const char* bufferModel, size_t lenModel, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromTensorflow(bufferModel, lenModel);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromTensorflow(const char *, size_t, const char *, size_t)(InString, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:973
	// ("cv::dnn::readNetFromTensorflow", vec![(pred!(mut, ["bufferModel", "lenModel", "bufferConfig", "lenConfig"], ["const char*", "size_t", "const char*", "size_t"]), _)]),
	void cv_dnn_readNetFromTensorflow_const_charX_size_t_const_charX_size_t(const char* bufferModel, size_t lenModel, const char* bufferConfig, size_t lenConfig, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromTensorflow(bufferModel, lenModel, bufferConfig, lenConfig);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::readNetFromTensorflow(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:962
	// ("cv::dnn::readNetFromTensorflow", vec![(pred!(mut, ["bufferModel"], ["const std::vector<unsigned char>*"]), _)]),
	void cv_dnn_readNetFromTensorflow_const_vectorLunsigned_charGR(const std::vector<unsigned char>* bufferModel, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromTensorflow(*bufferModel);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromTensorflow(const std::vector<uchar> &, const std::vector<uchar> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:962
	// ("cv::dnn::readNetFromTensorflow", vec![(pred!(mut, ["bufferModel", "bufferConfig"], ["const std::vector<unsigned char>*", "const std::vector<unsigned char>*"]), _)]),
	void cv_dnn_readNetFromTensorflow_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR(const std::vector<unsigned char>* bufferModel, const std::vector<unsigned char>* bufferConfig, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromTensorflow(*bufferModel, *bufferConfig);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::readNetFromTorch(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1022
	// ("cv::dnn::readNetFromTorch", vec![(pred!(mut, ["model"], ["const cv::String*"]), _)]),
	void cv_dnn_readNetFromTorch_const_StringR(const char* model, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromTorch(std::string(model));
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromTorch(const String &, bool, bool)(InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1022
	// ("cv::dnn::readNetFromTorch", vec![(pred!(mut, ["model", "isBinary", "evaluate"], ["const cv::String*", "bool", "bool"]), _)]),
	void cv_dnn_readNetFromTorch_const_StringR_bool_bool(const char* model, bool isBinary, bool evaluate, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromTorch(std::string(model), isBinary, evaluate);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::readNet(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1048
	// ("cv::dnn::readNet", vec![(pred!(mut, ["model"], ["const cv::String*"]), _)]),
	void cv_dnn_readNet_const_StringR(const char* model, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNet(std::string(model));
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNet(const String &, const String &, const String &)(InString, InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1048
	// ("cv::dnn::readNet", vec![(pred!(mut, ["model", "config", "framework"], ["const cv::String*", "const cv::String*", "const cv::String*"]), _)]),
	void cv_dnn_readNet_const_StringR_const_StringR_const_StringR(const char* model, const char* config, const char* framework, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNet(std::string(model), std::string(config), std::string(framework));
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::readNet(InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1059
	// ("cv::dnn::readNet", vec![(pred!(mut, ["framework", "bufferModel"], ["const cv::String*", "const std::vector<unsigned char>*"]), _)]),
	void cv_dnn_readNet_const_StringR_const_vectorLunsigned_charGR(const char* framework, const std::vector<unsigned char>* bufferModel, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNet(std::string(framework), *bufferModel);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNet(const String &, const std::vector<uchar> &, const std::vector<uchar> &)(InString, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1059
	// ("cv::dnn::readNet", vec![(pred!(mut, ["framework", "bufferModel", "bufferConfig"], ["const cv::String*", "const std::vector<unsigned char>*", "const std::vector<unsigned char>*"]), _)]),
	void cv_dnn_readNet_const_StringR_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR(const char* framework, const std::vector<unsigned char>* bufferModel, const std::vector<unsigned char>* bufferConfig, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNet(std::string(framework), *bufferModel, *bufferConfig);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readTensorFromONNX(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1127
	// ("cv::dnn::readTensorFromONNX", vec![(pred!(mut, ["path"], ["const cv::String*"]), _)]),
	void cv_dnn_readTensorFromONNX_const_StringR(const char* path, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::dnn::readTensorFromONNX(std::string(path));
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::readTorchBlob(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1065
	// ("cv::dnn::readTorchBlob", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_dnn_readTorchBlob_const_StringR(const char* filename, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::dnn::readTorchBlob(std::string(filename));
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readTorchBlob(const String &, bool)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1065
	// ("cv::dnn::readTorchBlob", vec![(pred!(mut, ["filename", "isBinary"], ["const cv::String*", "bool"]), _)]),
	void cv_dnn_readTorchBlob_const_StringR_bool(const char* filename, bool isBinary, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::dnn::readTorchBlob(std::string(filename), isBinary);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// releaseHDDLPlugin()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/utils/inference_engine.hpp:76
	// ("cv::dnn::releaseHDDLPlugin", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_releaseHDDLPlugin(ResultVoid* ocvrs_return) {
		try {
			cv::dnn::releaseHDDLPlugin();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// resetMyriadDevice()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/utils/inference_engine.hpp:49
	// ("cv::dnn::resetMyriadDevice", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_resetMyriadDevice(ResultVoid* ocvrs_return) {
		try {
			cv::dnn::resetMyriadDevice();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInferenceEngineBackendType(const cv::String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/utils/inference_engine.hpp:41
	// ("cv::dnn::setInferenceEngineBackendType", vec![(pred!(mut, ["newBackendType"], ["const cv::String*"]), _)]),
	void cv_dnn_setInferenceEngineBackendType_const_StringR(const char* newBackendType, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::dnn::setInferenceEngineBackendType(std::string(newBackendType));
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// shape(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/shape_utils.hpp:126
	// ("cv::dnn::shape", vec![(pred!(mut, ["mat"], ["const cv::Mat*"]), _)]),
	void cv_dnn_shape_const_MatR(const cv::Mat* mat, Result<cv::dnn::MatShape*>* ocvrs_return) {
		try {
			cv::dnn::MatShape ret = cv::dnn::shape(*mat);
			Ok(new cv::dnn::MatShape(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// shape(const MatSize &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/shape_utils.hpp:131
	// ("cv::dnn::shape", vec![(pred!(mut, ["sz"], ["const cv::MatSize*"]), _)]),
	void cv_dnn_shape_const_MatSizeR(const cv::MatSize* sz, Result<cv::dnn::MatShape*>* ocvrs_return) {
		try {
			cv::dnn::MatShape ret = cv::dnn::shape(*sz);
			Ok(new cv::dnn::MatShape(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// shape(const UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/shape_utils.hpp:136
	// ("cv::dnn::shape", vec![(pred!(mut, ["mat"], ["const cv::UMat*"]), _)]),
	void cv_dnn_shape_const_UMatR(const cv::UMat* mat, Result<cv::dnn::MatShape*>* ocvrs_return) {
		try {
			cv::dnn::MatShape ret = cv::dnn::shape(*mat);
			Ok(new cv::dnn::MatShape(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// shape(const int *, const int)(Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/shape_utils.hpp:119
	// ("cv::dnn::shape", vec![(pred!(mut, ["dims", "n"], ["const int*", "const int"]), _)]),
	void cv_dnn_shape_const_intX_const_int(const int* dims, const int n, Result<cv::dnn::MatShape*>* ocvrs_return) {
		try {
			cv::dnn::MatShape ret = cv::dnn::shape(dims, n);
			Ok(new cv::dnn::MatShape(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::shape(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/shape_utils.hpp:153
	// ("cv::dnn::shape", vec![(pred!(mut, ["a0"], ["int"]), _)]),
	void cv_dnn_shape_int(int a0, Result<cv::dnn::MatShape*>* ocvrs_return) {
		try {
			cv::dnn::MatShape ret = cv::dnn::shape(a0);
			Ok(new cv::dnn::MatShape(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// shape(int, int, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/shape_utils.hpp:153
	// ("cv::dnn::shape", vec![(pred!(mut, ["a0", "a1", "a2", "a3"], ["int", "int", "int", "int"]), _)]),
	void cv_dnn_shape_int_int_int_int(int a0, int a1, int a2, int a3, Result<cv::dnn::MatShape*>* ocvrs_return) {
		try {
			cv::dnn::MatShape ret = cv::dnn::shape(a0, a1, a2, a3);
			Ok(new cv::dnn::MatShape(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::shrinkCaffeModel(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1300
	// ("cv::dnn::shrinkCaffeModel", vec![(pred!(mut, ["src", "dst"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_dnn_shrinkCaffeModel_const_StringR_const_StringR(const char* src, const char* dst, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::shrinkCaffeModel(std::string(src), std::string(dst));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// shrinkCaffeModel(const String &, const String &, const std::vector<String> &)(InString, InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1300
	// ("cv::dnn::shrinkCaffeModel", vec![(pred!(mut, ["src", "dst", "layersTypes"], ["const cv::String*", "const cv::String*", "const std::vector<cv::String>*"]), _)]),
	void cv_dnn_shrinkCaffeModel_const_StringR_const_StringR_const_vectorLStringGR(const char* src, const char* dst, const std::vector<cv::String>* layersTypes, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::shrinkCaffeModel(std::string(src), std::string(dst), *layersTypes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// slice(const Mat &, const _Range &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/shape_utils.hpp:63
	// ("cv::dnn::slice", vec![(pred!(mut, ["m", "r0"], ["const cv::Mat*", "const cv::dnn::_Range*"]), _)]),
	void cv_dnn_slice_const_MatR_const__RangeR(const cv::Mat* m, const cv::dnn::_Range* r0, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::dnn::slice(*m, *r0);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// slice(const Mat &, const _Range &, const _Range &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/shape_utils.hpp:72
	// ("cv::dnn::slice", vec![(pred!(mut, ["m", "r0", "r1"], ["const cv::Mat*", "const cv::dnn::_Range*", "const cv::dnn::_Range*"]), _)]),
	void cv_dnn_slice_const_MatR_const__RangeR_const__RangeR(const cv::Mat* m, const cv::dnn::_Range* r0, const cv::dnn::_Range* r1, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::dnn::slice(*m, *r0, *r1);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// slice(const Mat &, const _Range &, const _Range &, const _Range &)(TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/shape_utils.hpp:83
	// ("cv::dnn::slice", vec![(pred!(mut, ["m", "r0", "r1", "r2"], ["const cv::Mat*", "const cv::dnn::_Range*", "const cv::dnn::_Range*", "const cv::dnn::_Range*"]), _)]),
	void cv_dnn_slice_const_MatR_const__RangeR_const__RangeR_const__RangeR(const cv::Mat* m, const cv::dnn::_Range* r0, const cv::dnn::_Range* r1, const cv::dnn::_Range* r2, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::dnn::slice(*m, *r0, *r1, *r2);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// slice(const Mat &, const _Range &, const _Range &, const _Range &, const _Range &)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/shape_utils.hpp:95
	// ("cv::dnn::slice", vec![(pred!(mut, ["m", "r0", "r1", "r2", "r3"], ["const cv::Mat*", "const cv::dnn::_Range*", "const cv::dnn::_Range*", "const cv::dnn::_Range*", "const cv::dnn::_Range*"]), _)]),
	void cv_dnn_slice_const_MatR_const__RangeR_const__RangeR_const__RangeR_const__RangeR(const cv::Mat* m, const cv::dnn::_Range* r0, const cv::dnn::_Range* r1, const cv::dnn::_Range* r2, const cv::dnn::_Range* r3, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::dnn::slice(*m, *r0, *r1, *r2, *r3);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::softNMSBoxes(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1380
	// ("cv::dnn::softNMSBoxes", vec![(pred!(mut, ["bboxes", "scores", "updated_scores", "score_threshold", "nms_threshold", "indices"], ["const std::vector<cv::Rect>*", "const std::vector<float>*", "std::vector<float>*", "const float", "const float", "std::vector<int>*"]), _)]),
	void cv_dnn_softNMSBoxes_const_vectorLRectGR_const_vectorLfloatGR_vectorLfloatGR_const_float_const_float_vectorLintGR(const std::vector<cv::Rect>* bboxes, const std::vector<float>* scores, std::vector<float>* updated_scores, const float score_threshold, const float nms_threshold, std::vector<int>* indices, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::softNMSBoxes(*bboxes, *scores, *updated_scores, score_threshold, nms_threshold, *indices);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// softNMSBoxes(const std::vector<Rect> &, const std::vector<float> &, std::vector<float> &, const float, const float, std::vector<int> &, size_t, const float, SoftNMSMethod)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1380
	// ("cv::dnn::softNMSBoxes", vec![(pred!(mut, ["bboxes", "scores", "updated_scores", "score_threshold", "nms_threshold", "indices", "top_k", "sigma", "method"], ["const std::vector<cv::Rect>*", "const std::vector<float>*", "std::vector<float>*", "const float", "const float", "std::vector<int>*", "size_t", "const float", "cv::dnn::SoftNMSMethod"]), _)]),
	void cv_dnn_softNMSBoxes_const_vectorLRectGR_const_vectorLfloatGR_vectorLfloatGR_const_float_const_float_vectorLintGR_size_t_const_float_SoftNMSMethod(const std::vector<cv::Rect>* bboxes, const std::vector<float>* scores, std::vector<float>* updated_scores, const float score_threshold, const float nms_threshold, std::vector<int>* indices, size_t top_k, const float sigma, cv::dnn::SoftNMSMethod method, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::softNMSBoxes(*bboxes, *scores, *updated_scores, score_threshold, nms_threshold, *indices, top_k, sigma, method);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::total(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/shape_utils.hpp:184
	// ("cv::dnn::total", vec![(pred!(mut, ["mat"], ["const cv::Mat*"]), _)]),
	void cv_dnn_total_const_MatR(const cv::Mat* mat, Result<int>* ocvrs_return) {
		try {
			int ret = cv::dnn::total(*mat);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// total(const Mat &, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/shape_utils.hpp:184
	// ("cv::dnn::total", vec![(pred!(mut, ["mat", "start", "end"], ["const cv::Mat*", "int", "int"]), _)]),
	void cv_dnn_total_const_MatR_int_int(const cv::Mat* mat, int start, int end, Result<int>* ocvrs_return) {
		try {
			int ret = cv::dnn::total(*mat, start, end);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::total(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/shape_utils.hpp:161
	// ("cv::dnn::total", vec![(pred!(mut, ["shape"], ["const cv::dnn::MatShape*"]), _)]),
	void cv_dnn_total_const_MatShapeR(const cv::dnn::MatShape* shape, Result<int>* ocvrs_return) {
		try {
			int ret = cv::dnn::total(*shape);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// total(const MatShape &, int, int)(CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/shape_utils.hpp:161
	// ("cv::dnn::total", vec![(pred!(mut, ["shape", "start", "end"], ["const cv::dnn::MatShape*", "int", "int"]), _)]),
	void cv_dnn_total_const_MatShapeR_int_int(const cv::dnn::MatShape* shape, int start, int end, Result<int>* ocvrs_return) {
		try {
			int ret = cv::dnn::total(*shape, start, end);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writeTextGraph(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1309
	// ("cv::dnn::writeTextGraph", vec![(pred!(mut, ["model", "output"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_dnn_writeTextGraph_const_StringR_const_StringR(const char* model, const char* output, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::writeTextGraph(std::string(model), std::string(output));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:677
	// ("cv::dnn::AbsLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_AbsLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::AbsLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::AbsLayer> ret = cv::dnn::AbsLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::AbsLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::AbsLayer::defaultNew() generated
	// ("cv::dnn::AbsLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::AbsLayer* cv_dnn_AbsLayer_defaultNew_const() {
			cv::dnn::AbsLayer* ret = new cv::dnn::AbsLayer();
			return ret;
	}

	// cv::dnn::AbsLayer::to_ActivationLayer() generated
	// ("cv::dnn::AbsLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_AbsLayer_to_ActivationLayer(cv::dnn::AbsLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::AbsLayer::to_Algorithm() generated
	// ("cv::dnn::AbsLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_AbsLayer_to_Algorithm(cv::dnn::AbsLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::AbsLayer::to_Layer() generated
	// ("cv::dnn::AbsLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_AbsLayer_to_Layer(cv::dnn::AbsLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::AbsLayer::delete() generated
	// ("cv::dnn::AbsLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_AbsLayer_delete(cv::dnn::AbsLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1005
	// ("cv::dnn::AccumLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_AccumLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::AccumLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::AccumLayer> ret = cv::dnn::AccumLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::AccumLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::AccumLayer::defaultNew() generated
	// ("cv::dnn::AccumLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::AccumLayer* cv_dnn_AccumLayer_defaultNew_const() {
			cv::dnn::AccumLayer* ret = new cv::dnn::AccumLayer();
			return ret;
	}

	// cv::dnn::AccumLayer::to_Algorithm() generated
	// ("cv::dnn::AccumLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_AccumLayer_to_Algorithm(cv::dnn::AccumLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::AccumLayer::to_Layer() generated
	// ("cv::dnn::AccumLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_AccumLayer_to_Layer(cv::dnn::AccumLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::AccumLayer::delete() generated
	// ("cv::dnn::AccumLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_AccumLayer_delete(cv::dnn::AccumLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:735
	// ("cv::dnn::AcosLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_AcosLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::AcosLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::AcosLayer> ret = cv::dnn::AcosLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::AcosLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::AcosLayer::defaultNew() generated
	// ("cv::dnn::AcosLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::AcosLayer* cv_dnn_AcosLayer_defaultNew_const() {
			cv::dnn::AcosLayer* ret = new cv::dnn::AcosLayer();
			return ret;
	}

	// cv::dnn::AcosLayer::to_ActivationLayer() generated
	// ("cv::dnn::AcosLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_AcosLayer_to_ActivationLayer(cv::dnn::AcosLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::AcosLayer::to_Algorithm() generated
	// ("cv::dnn::AcosLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_AcosLayer_to_Algorithm(cv::dnn::AcosLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::AcosLayer::to_Layer() generated
	// ("cv::dnn::AcosLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_AcosLayer_to_Layer(cv::dnn::AcosLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::AcosLayer::delete() generated
	// ("cv::dnn::AcosLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_AcosLayer_delete(cv::dnn::AcosLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:741
	// ("cv::dnn::AcoshLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_AcoshLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::AcoshLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::AcoshLayer> ret = cv::dnn::AcoshLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::AcoshLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::AcoshLayer::defaultNew() generated
	// ("cv::dnn::AcoshLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::AcoshLayer* cv_dnn_AcoshLayer_defaultNew_const() {
			cv::dnn::AcoshLayer* ret = new cv::dnn::AcoshLayer();
			return ret;
	}

	// cv::dnn::AcoshLayer::to_ActivationLayer() generated
	// ("cv::dnn::AcoshLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_AcoshLayer_to_ActivationLayer(cv::dnn::AcoshLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::AcoshLayer::to_Algorithm() generated
	// ("cv::dnn::AcoshLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_AcoshLayer_to_Algorithm(cv::dnn::AcoshLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::AcoshLayer::to_Layer() generated
	// ("cv::dnn::AcoshLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_AcoshLayer_to_Layer(cv::dnn::AcoshLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::AcoshLayer::delete() generated
	// ("cv::dnn::AcoshLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_AcoshLayer_delete(cv::dnn::AcoshLayer* instance) {
			delete instance;
	}

	// forwardSlice(const float *, float *, int, size_t, int, int)(VariableArray, VariableArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:606
	// ("cv::dnn::ActivationLayer::forwardSlice", vec![(pred!(const, ["src", "dst", "len", "outPlaneSize", "cn0", "cn1"], ["const float*", "float*", "int", "size_t", "int", "int"]), _)]),
	void cv_dnn_ActivationLayer_forwardSlice_const_const_floatX_floatX_int_size_t_int_int(const cv::dnn::ActivationLayer* instance, const float* src, float* dst, int len, size_t outPlaneSize, int cn0, int cn1, ResultVoid* ocvrs_return) {
		try {
			instance->forwardSlice(src, dst, len, outPlaneSize, cn0, cn1);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// forwardSlice(const int *, const int *, int *, int, size_t, int, int)(VariableArray, VariableArray, VariableArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:608
	// ("cv::dnn::ActivationLayer::forwardSlice", vec![(pred!(const, ["src", "lut", "dst", "len", "outPlaneSize", "cn0", "cn1"], ["const int*", "const int*", "int*", "int", "size_t", "int", "int"]), _)]),
	void cv_dnn_ActivationLayer_forwardSlice_const_const_intX_const_intX_intX_int_size_t_int_int(const cv::dnn::ActivationLayer* instance, const int* src, const int* lut, int* dst, int len, size_t outPlaneSize, int cn0, int cn1, ResultVoid* ocvrs_return) {
		try {
			instance->forwardSlice(src, lut, dst, len, outPlaneSize, cn0, cn1);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ActivationLayer::defaultNew() generated
	// ("cv::dnn::ActivationLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_ActivationLayer_defaultNew_const() {
			cv::dnn::ActivationLayer* ret = new cv::dnn::ActivationLayer();
			return ret;
	}

	// cv::dnn::ActivationLayer::to_AbsLayer() generated
	// ("cv::dnn::ActivationLayer::to_AbsLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::AbsLayer* cv_dnn_ActivationLayer_to_AbsLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::AbsLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_AcosLayer() generated
	// ("cv::dnn::ActivationLayer::to_AcosLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::AcosLayer* cv_dnn_ActivationLayer_to_AcosLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::AcosLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_AcoshLayer() generated
	// ("cv::dnn::ActivationLayer::to_AcoshLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::AcoshLayer* cv_dnn_ActivationLayer_to_AcoshLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::AcoshLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_ActivationLayerInt8() generated
	// ("cv::dnn::ActivationLayer::to_ActivationLayerInt8", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayerInt8* cv_dnn_ActivationLayer_to_ActivationLayerInt8(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayerInt8*>(instance);
	}

	// cv::dnn::ActivationLayer::to_AsinLayer() generated
	// ("cv::dnn::ActivationLayer::to_AsinLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::AsinLayer* cv_dnn_ActivationLayer_to_AsinLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::AsinLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_AsinhLayer() generated
	// ("cv::dnn::ActivationLayer::to_AsinhLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::AsinhLayer* cv_dnn_ActivationLayer_to_AsinhLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::AsinhLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_AtanLayer() generated
	// ("cv::dnn::ActivationLayer::to_AtanLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::AtanLayer* cv_dnn_ActivationLayer_to_AtanLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::AtanLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_AtanhLayer() generated
	// ("cv::dnn::ActivationLayer::to_AtanhLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::AtanhLayer* cv_dnn_ActivationLayer_to_AtanhLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::AtanhLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_BNLLLayer() generated
	// ("cv::dnn::ActivationLayer::to_BNLLLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::BNLLLayer* cv_dnn_ActivationLayer_to_BNLLLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::BNLLLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_BatchNormLayer() generated
	// ("cv::dnn::ActivationLayer::to_BatchNormLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::BatchNormLayer* cv_dnn_ActivationLayer_to_BatchNormLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::BatchNormLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_BatchNormLayerInt8() generated
	// ("cv::dnn::ActivationLayer::to_BatchNormLayerInt8", vec![(pred!(mut, [], []), _)]),
	cv::dnn::BatchNormLayerInt8* cv_dnn_ActivationLayer_to_BatchNormLayerInt8(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::BatchNormLayerInt8*>(instance);
	}

	// cv::dnn::ActivationLayer::to_CeilLayer() generated
	// ("cv::dnn::ActivationLayer::to_CeilLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::CeilLayer* cv_dnn_ActivationLayer_to_CeilLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::CeilLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_CeluLayer() generated
	// ("cv::dnn::ActivationLayer::to_CeluLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::CeluLayer* cv_dnn_ActivationLayer_to_CeluLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::CeluLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_ChannelsPReLULayer() generated
	// ("cv::dnn::ActivationLayer::to_ChannelsPReLULayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ChannelsPReLULayer* cv_dnn_ActivationLayer_to_ChannelsPReLULayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::ChannelsPReLULayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_CosLayer() generated
	// ("cv::dnn::ActivationLayer::to_CosLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::CosLayer* cv_dnn_ActivationLayer_to_CosLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::CosLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_CoshLayer() generated
	// ("cv::dnn::ActivationLayer::to_CoshLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::CoshLayer* cv_dnn_ActivationLayer_to_CoshLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::CoshLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_ELULayer() generated
	// ("cv::dnn::ActivationLayer::to_ELULayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ELULayer* cv_dnn_ActivationLayer_to_ELULayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::ELULayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_ErfLayer() generated
	// ("cv::dnn::ActivationLayer::to_ErfLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ErfLayer* cv_dnn_ActivationLayer_to_ErfLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::ErfLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_ExpLayer() generated
	// ("cv::dnn::ActivationLayer::to_ExpLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ExpLayer* cv_dnn_ActivationLayer_to_ExpLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::ExpLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_FloorLayer() generated
	// ("cv::dnn::ActivationLayer::to_FloorLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::FloorLayer* cv_dnn_ActivationLayer_to_FloorLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::FloorLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_GeluApproximationLayer() generated
	// ("cv::dnn::ActivationLayer::to_GeluApproximationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::GeluApproximationLayer* cv_dnn_ActivationLayer_to_GeluApproximationLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::GeluApproximationLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_GeluLayer() generated
	// ("cv::dnn::ActivationLayer::to_GeluLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::GeluLayer* cv_dnn_ActivationLayer_to_GeluLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::GeluLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_HardSigmoidLayer() generated
	// ("cv::dnn::ActivationLayer::to_HardSigmoidLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::HardSigmoidLayer* cv_dnn_ActivationLayer_to_HardSigmoidLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::HardSigmoidLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_HardSwishLayer() generated
	// ("cv::dnn::ActivationLayer::to_HardSwishLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::HardSwishLayer* cv_dnn_ActivationLayer_to_HardSwishLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::HardSwishLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_LogLayer() generated
	// ("cv::dnn::ActivationLayer::to_LogLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::LogLayer* cv_dnn_ActivationLayer_to_LogLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::LogLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_MishLayer() generated
	// ("cv::dnn::ActivationLayer::to_MishLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::MishLayer* cv_dnn_ActivationLayer_to_MishLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::MishLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_NotLayer() generated
	// ("cv::dnn::ActivationLayer::to_NotLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::NotLayer* cv_dnn_ActivationLayer_to_NotLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::NotLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_PowerLayer() generated
	// ("cv::dnn::ActivationLayer::to_PowerLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::PowerLayer* cv_dnn_ActivationLayer_to_PowerLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::PowerLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_ReLU6Layer() generated
	// ("cv::dnn::ActivationLayer::to_ReLU6Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ReLU6Layer* cv_dnn_ActivationLayer_to_ReLU6Layer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::ReLU6Layer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_ReLULayer() generated
	// ("cv::dnn::ActivationLayer::to_ReLULayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ReLULayer* cv_dnn_ActivationLayer_to_ReLULayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::ReLULayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_ReciprocalLayer() generated
	// ("cv::dnn::ActivationLayer::to_ReciprocalLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ReciprocalLayer* cv_dnn_ActivationLayer_to_ReciprocalLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::ReciprocalLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_RoundLayer() generated
	// ("cv::dnn::ActivationLayer::to_RoundLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::RoundLayer* cv_dnn_ActivationLayer_to_RoundLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::RoundLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_SeluLayer() generated
	// ("cv::dnn::ActivationLayer::to_SeluLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SeluLayer* cv_dnn_ActivationLayer_to_SeluLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::SeluLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_ShrinkLayer() generated
	// ("cv::dnn::ActivationLayer::to_ShrinkLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ShrinkLayer* cv_dnn_ActivationLayer_to_ShrinkLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::ShrinkLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_SigmoidLayer() generated
	// ("cv::dnn::ActivationLayer::to_SigmoidLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SigmoidLayer* cv_dnn_ActivationLayer_to_SigmoidLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::SigmoidLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_SignLayer() generated
	// ("cv::dnn::ActivationLayer::to_SignLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SignLayer* cv_dnn_ActivationLayer_to_SignLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::SignLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_SinLayer() generated
	// ("cv::dnn::ActivationLayer::to_SinLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SinLayer* cv_dnn_ActivationLayer_to_SinLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::SinLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_SinhLayer() generated
	// ("cv::dnn::ActivationLayer::to_SinhLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SinhLayer* cv_dnn_ActivationLayer_to_SinhLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::SinhLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_SoftplusLayer() generated
	// ("cv::dnn::ActivationLayer::to_SoftplusLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SoftplusLayer* cv_dnn_ActivationLayer_to_SoftplusLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::SoftplusLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_SoftsignLayer() generated
	// ("cv::dnn::ActivationLayer::to_SoftsignLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SoftsignLayer* cv_dnn_ActivationLayer_to_SoftsignLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::SoftsignLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_SqrtLayer() generated
	// ("cv::dnn::ActivationLayer::to_SqrtLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SqrtLayer* cv_dnn_ActivationLayer_to_SqrtLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::SqrtLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_SwishLayer() generated
	// ("cv::dnn::ActivationLayer::to_SwishLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SwishLayer* cv_dnn_ActivationLayer_to_SwishLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::SwishLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_TanHLayer() generated
	// ("cv::dnn::ActivationLayer::to_TanHLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::TanHLayer* cv_dnn_ActivationLayer_to_TanHLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::TanHLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_TanLayer() generated
	// ("cv::dnn::ActivationLayer::to_TanLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::TanLayer* cv_dnn_ActivationLayer_to_TanLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::TanLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_ThresholdedReluLayer() generated
	// ("cv::dnn::ActivationLayer::to_ThresholdedReluLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ThresholdedReluLayer* cv_dnn_ActivationLayer_to_ThresholdedReluLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::ThresholdedReluLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_Algorithm() generated
	// ("cv::dnn::ActivationLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_ActivationLayer_to_Algorithm(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::ActivationLayer::to_Layer() generated
	// ("cv::dnn::ActivationLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_ActivationLayer_to_Layer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::ActivationLayer::delete() generated
	// ("cv::dnn::ActivationLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_ActivationLayer_delete(cv::dnn::ActivationLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:871
	// ("cv::dnn::ActivationLayerInt8::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ActivationLayerInt8_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::ActivationLayerInt8>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::ActivationLayerInt8> ret = cv::dnn::ActivationLayerInt8::create(*params);
			Ok(new cv::Ptr<cv::dnn::ActivationLayerInt8>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ActivationLayerInt8::defaultNew() generated
	// ("cv::dnn::ActivationLayerInt8::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::ActivationLayerInt8* cv_dnn_ActivationLayerInt8_defaultNew_const() {
			cv::dnn::ActivationLayerInt8* ret = new cv::dnn::ActivationLayerInt8();
			return ret;
	}

	// cv::dnn::ActivationLayerInt8::to_ActivationLayer() generated
	// ("cv::dnn::ActivationLayerInt8::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_ActivationLayerInt8_to_ActivationLayer(cv::dnn::ActivationLayerInt8* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::ActivationLayerInt8::to_Algorithm() generated
	// ("cv::dnn::ActivationLayerInt8::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_ActivationLayerInt8_to_Algorithm(cv::dnn::ActivationLayerInt8* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::ActivationLayerInt8::to_Layer() generated
	// ("cv::dnn::ActivationLayerInt8::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_ActivationLayerInt8_to_Layer(cv::dnn::ActivationLayerInt8* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::ActivationLayerInt8::delete() generated
	// ("cv::dnn::ActivationLayerInt8::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_ActivationLayerInt8_delete(cv::dnn::ActivationLayerInt8* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:335
	// ("cv::dnn::ArgLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ArgLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::ArgLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::ArgLayer> ret = cv::dnn::ArgLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::ArgLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ArgLayer::defaultNew() generated
	// ("cv::dnn::ArgLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::ArgLayer* cv_dnn_ArgLayer_defaultNew_const() {
			cv::dnn::ArgLayer* ret = new cv::dnn::ArgLayer();
			return ret;
	}

	// cv::dnn::ArgLayer::to_Algorithm() generated
	// ("cv::dnn::ArgLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_ArgLayer_to_Algorithm(cv::dnn::ArgLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::ArgLayer::to_Layer() generated
	// ("cv::dnn::ArgLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_ArgLayer_to_Layer(cv::dnn::ArgLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::ArgLayer::delete() generated
	// ("cv::dnn::ArgLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_ArgLayer_delete(cv::dnn::ArgLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:747
	// ("cv::dnn::AsinLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_AsinLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::AsinLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::AsinLayer> ret = cv::dnn::AsinLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::AsinLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::AsinLayer::defaultNew() generated
	// ("cv::dnn::AsinLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::AsinLayer* cv_dnn_AsinLayer_defaultNew_const() {
			cv::dnn::AsinLayer* ret = new cv::dnn::AsinLayer();
			return ret;
	}

	// cv::dnn::AsinLayer::to_ActivationLayer() generated
	// ("cv::dnn::AsinLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_AsinLayer_to_ActivationLayer(cv::dnn::AsinLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::AsinLayer::to_Algorithm() generated
	// ("cv::dnn::AsinLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_AsinLayer_to_Algorithm(cv::dnn::AsinLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::AsinLayer::to_Layer() generated
	// ("cv::dnn::AsinLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_AsinLayer_to_Layer(cv::dnn::AsinLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::AsinLayer::delete() generated
	// ("cv::dnn::AsinLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_AsinLayer_delete(cv::dnn::AsinLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:753
	// ("cv::dnn::AsinhLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_AsinhLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::AsinhLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::AsinhLayer> ret = cv::dnn::AsinhLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::AsinhLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::AsinhLayer::defaultNew() generated
	// ("cv::dnn::AsinhLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::AsinhLayer* cv_dnn_AsinhLayer_defaultNew_const() {
			cv::dnn::AsinhLayer* ret = new cv::dnn::AsinhLayer();
			return ret;
	}

	// cv::dnn::AsinhLayer::to_ActivationLayer() generated
	// ("cv::dnn::AsinhLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_AsinhLayer_to_ActivationLayer(cv::dnn::AsinhLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::AsinhLayer::to_Algorithm() generated
	// ("cv::dnn::AsinhLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_AsinhLayer_to_Algorithm(cv::dnn::AsinhLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::AsinhLayer::to_Layer() generated
	// ("cv::dnn::AsinhLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_AsinhLayer_to_Layer(cv::dnn::AsinhLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::AsinhLayer::delete() generated
	// ("cv::dnn::AsinhLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_AsinhLayer_delete(cv::dnn::AsinhLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:759
	// ("cv::dnn::AtanLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_AtanLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::AtanLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::AtanLayer> ret = cv::dnn::AtanLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::AtanLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::AtanLayer::defaultNew() generated
	// ("cv::dnn::AtanLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::AtanLayer* cv_dnn_AtanLayer_defaultNew_const() {
			cv::dnn::AtanLayer* ret = new cv::dnn::AtanLayer();
			return ret;
	}

	// cv::dnn::AtanLayer::to_ActivationLayer() generated
	// ("cv::dnn::AtanLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_AtanLayer_to_ActivationLayer(cv::dnn::AtanLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::AtanLayer::to_Algorithm() generated
	// ("cv::dnn::AtanLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_AtanLayer_to_Algorithm(cv::dnn::AtanLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::AtanLayer::to_Layer() generated
	// ("cv::dnn::AtanLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_AtanLayer_to_Layer(cv::dnn::AtanLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::AtanLayer::delete() generated
	// ("cv::dnn::AtanLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_AtanLayer_delete(cv::dnn::AtanLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:765
	// ("cv::dnn::AtanhLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_AtanhLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::AtanhLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::AtanhLayer> ret = cv::dnn::AtanhLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::AtanhLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::AtanhLayer::defaultNew() generated
	// ("cv::dnn::AtanhLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::AtanhLayer* cv_dnn_AtanhLayer_defaultNew_const() {
			cv::dnn::AtanhLayer* ret = new cv::dnn::AtanhLayer();
			return ret;
	}

	// cv::dnn::AtanhLayer::to_ActivationLayer() generated
	// ("cv::dnn::AtanhLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_AtanhLayer_to_ActivationLayer(cv::dnn::AtanhLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::AtanhLayer::to_Algorithm() generated
	// ("cv::dnn::AtanhLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_AtanhLayer_to_Algorithm(cv::dnn::AtanhLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::AtanhLayer::to_Layer() generated
	// ("cv::dnn::AtanhLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_AtanhLayer_to_Layer(cv::dnn::AtanhLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::AtanhLayer::delete() generated
	// ("cv::dnn::AtanhLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_AtanhLayer_delete(cv::dnn::AtanhLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1183
	// ("cv::dnn::AttentionLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_AttentionLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::AttentionLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::AttentionLayer> ret = cv::dnn::AttentionLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::AttentionLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::AttentionLayer::defaultNew() generated
	// ("cv::dnn::AttentionLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::AttentionLayer* cv_dnn_AttentionLayer_defaultNew_const() {
			cv::dnn::AttentionLayer* ret = new cv::dnn::AttentionLayer();
			return ret;
	}

	// cv::dnn::AttentionLayer::to_Algorithm() generated
	// ("cv::dnn::AttentionLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_AttentionLayer_to_Algorithm(cv::dnn::AttentionLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::AttentionLayer::to_Layer() generated
	// ("cv::dnn::AttentionLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_AttentionLayer_to_Layer(cv::dnn::AttentionLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::AttentionLayer::delete() generated
	// ("cv::dnn::AttentionLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_AttentionLayer_delete(cv::dnn::AttentionLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:671
	// ("cv::dnn::BNLLLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_BNLLLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::BNLLLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::BNLLLayer> ret = cv::dnn::BNLLLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::BNLLLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::BNLLLayer::defaultNew() generated
	// ("cv::dnn::BNLLLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::BNLLLayer* cv_dnn_BNLLLayer_defaultNew_const() {
			cv::dnn::BNLLLayer* ret = new cv::dnn::BNLLLayer();
			return ret;
	}

	// cv::dnn::BNLLLayer::to_ActivationLayer() generated
	// ("cv::dnn::BNLLLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_BNLLLayer_to_ActivationLayer(cv::dnn::BNLLLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::BNLLLayer::to_Algorithm() generated
	// ("cv::dnn::BNLLLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_BNLLLayer_to_Algorithm(cv::dnn::BNLLLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::BNLLLayer::to_Layer() generated
	// ("cv::dnn::BNLLLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_BNLLLayer_to_Layer(cv::dnn::BNLLLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::BNLLLayer::delete() generated
	// ("cv::dnn::BNLLLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_BNLLLayer_delete(cv::dnn::BNLLLayer* instance) {
			delete instance;
	}

	// cv::dnn::BackendNode::backendId() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:165
	// ("cv::dnn::BackendNode::backendId", vec![(pred!(const, [], []), _)]),
	int cv_dnn_BackendNode_propBackendId_const(const cv::dnn::BackendNode* instance) {
			int ret = instance->backendId;
			return ret;
	}

	// cv::dnn::BackendNode::setBackendId(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:165
	// ("cv::dnn::BackendNode::setBackendId", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_BackendNode_propBackendId_const_int(cv::dnn::BackendNode* instance, const int val) {
			instance->backendId = val;
	}

	// cv::dnn::BackendNode::delete() generated
	// ("cv::dnn::BackendNode::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_BackendNode_delete(cv::dnn::BackendNode* instance) {
			delete instance;
	}

	// copyToHost()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:202
	// ("cv::dnn::BackendWrapper::copyToHost", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_BackendWrapper_copyToHost(cv::dnn::BackendWrapper* instance, ResultVoid* ocvrs_return) {
		try {
			instance->copyToHost();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setHostDirty()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:207
	// ("cv::dnn::BackendWrapper::setHostDirty", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_BackendWrapper_setHostDirty(cv::dnn::BackendWrapper* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setHostDirty();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::BackendWrapper::backendId() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:209
	// ("cv::dnn::BackendWrapper::backendId", vec![(pred!(const, [], []), _)]),
	int cv_dnn_BackendWrapper_propBackendId_const(const cv::dnn::BackendWrapper* instance) {
			int ret = instance->backendId;
			return ret;
	}

	// cv::dnn::BackendWrapper::setBackendId(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:209
	// ("cv::dnn::BackendWrapper::setBackendId", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_BackendWrapper_propBackendId_const_int(cv::dnn::BackendWrapper* instance, const int val) {
			instance->backendId = val;
	}

	// cv::dnn::BackendWrapper::targetId() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:210
	// ("cv::dnn::BackendWrapper::targetId", vec![(pred!(const, [], []), _)]),
	int cv_dnn_BackendWrapper_propTargetId_const(const cv::dnn::BackendWrapper* instance) {
			int ret = instance->targetId;
			return ret;
	}

	// cv::dnn::BackendWrapper::setTargetId(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:210
	// ("cv::dnn::BackendWrapper::setTargetId", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_BackendWrapper_propTargetId_const_int(cv::dnn::BackendWrapper* instance, const int val) {
			instance->targetId = val;
	}

	// cv::dnn::BackendWrapper::delete() generated
	// ("cv::dnn::BackendWrapper::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_BackendWrapper_delete(cv::dnn::BackendWrapper* instance) {
			delete instance;
	}

	// cv::dnn::BaseConvolutionLayer::defaultNew() generated
	// ("cv::dnn::BaseConvolutionLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::BaseConvolutionLayer* cv_dnn_BaseConvolutionLayer_defaultNew_const() {
			cv::dnn::BaseConvolutionLayer* ret = new cv::dnn::BaseConvolutionLayer();
			return ret;
	}

	// cv::dnn::BaseConvolutionLayer::kernel() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:280
	// ("cv::dnn::BaseConvolutionLayer::kernel", vec![(pred!(const, [], []), _)]),
	void cv_dnn_BaseConvolutionLayer_propKernel_const(const cv::dnn::BaseConvolutionLayer* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->kernel;
			*ocvrs_return = ret;
	}

	// cv::dnn::BaseConvolutionLayer::setKernel(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:280
	// ("cv::dnn::BaseConvolutionLayer::setKernel", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_dnn_BaseConvolutionLayer_propKernel_const_Size(cv::dnn::BaseConvolutionLayer* instance, const cv::Size* val) {
			instance->kernel = *val;
	}

	// cv::dnn::BaseConvolutionLayer::stride() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:280
	// ("cv::dnn::BaseConvolutionLayer::stride", vec![(pred!(const, [], []), _)]),
	void cv_dnn_BaseConvolutionLayer_propStride_const(const cv::dnn::BaseConvolutionLayer* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->stride;
			*ocvrs_return = ret;
	}

	// cv::dnn::BaseConvolutionLayer::setStride(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:280
	// ("cv::dnn::BaseConvolutionLayer::setStride", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_dnn_BaseConvolutionLayer_propStride_const_Size(cv::dnn::BaseConvolutionLayer* instance, const cv::Size* val) {
			instance->stride = *val;
	}

	// cv::dnn::BaseConvolutionLayer::pad() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:280
	// ("cv::dnn::BaseConvolutionLayer::pad", vec![(pred!(const, [], []), _)]),
	void cv_dnn_BaseConvolutionLayer_propPad_const(const cv::dnn::BaseConvolutionLayer* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->pad;
			*ocvrs_return = ret;
	}

	// cv::dnn::BaseConvolutionLayer::setPad(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:280
	// ("cv::dnn::BaseConvolutionLayer::setPad", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_dnn_BaseConvolutionLayer_propPad_const_Size(cv::dnn::BaseConvolutionLayer* instance, const cv::Size* val) {
			instance->pad = *val;
	}

	// cv::dnn::BaseConvolutionLayer::dilation() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:280
	// ("cv::dnn::BaseConvolutionLayer::dilation", vec![(pred!(const, [], []), _)]),
	void cv_dnn_BaseConvolutionLayer_propDilation_const(const cv::dnn::BaseConvolutionLayer* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->dilation;
			*ocvrs_return = ret;
	}

	// cv::dnn::BaseConvolutionLayer::setDilation(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:280
	// ("cv::dnn::BaseConvolutionLayer::setDilation", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_dnn_BaseConvolutionLayer_propDilation_const_Size(cv::dnn::BaseConvolutionLayer* instance, const cv::Size* val) {
			instance->dilation = *val;
	}

	// cv::dnn::BaseConvolutionLayer::adjustPad() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:280
	// ("cv::dnn::BaseConvolutionLayer::adjustPad", vec![(pred!(const, [], []), _)]),
	void cv_dnn_BaseConvolutionLayer_propAdjustPad_const(const cv::dnn::BaseConvolutionLayer* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->adjustPad;
			*ocvrs_return = ret;
	}

	// cv::dnn::BaseConvolutionLayer::setAdjustPad(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:280
	// ("cv::dnn::BaseConvolutionLayer::setAdjustPad", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_dnn_BaseConvolutionLayer_propAdjustPad_const_Size(cv::dnn::BaseConvolutionLayer* instance, const cv::Size* val) {
			instance->adjustPad = *val;
	}

	// cv::dnn::BaseConvolutionLayer::adjust_pads() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:281
	// ("cv::dnn::BaseConvolutionLayer::adjust_pads", vec![(pred!(const, [], []), _)]),
	std::vector<size_t>* cv_dnn_BaseConvolutionLayer_propAdjust_pads_const(const cv::dnn::BaseConvolutionLayer* instance) {
			std::vector<size_t> ret = instance->adjust_pads;
			return new std::vector<size_t>(ret);
	}

	// cv::dnn::BaseConvolutionLayer::setAdjust_pads(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:281
	// ("cv::dnn::BaseConvolutionLayer::setAdjust_pads", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
	void cv_dnn_BaseConvolutionLayer_propAdjust_pads_const_vectorLsize_tG(cv::dnn::BaseConvolutionLayer* instance, const std::vector<size_t>* val) {
			instance->adjust_pads = *val;
	}

	// cv::dnn::BaseConvolutionLayer::kernel_size() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:282
	// ("cv::dnn::BaseConvolutionLayer::kernel_size", vec![(pred!(const, [], []), _)]),
	std::vector<size_t>* cv_dnn_BaseConvolutionLayer_propKernel_size_const(const cv::dnn::BaseConvolutionLayer* instance) {
			std::vector<size_t> ret = instance->kernel_size;
			return new std::vector<size_t>(ret);
	}

	// cv::dnn::BaseConvolutionLayer::setKernel_size(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:282
	// ("cv::dnn::BaseConvolutionLayer::setKernel_size", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
	void cv_dnn_BaseConvolutionLayer_propKernel_size_const_vectorLsize_tG(cv::dnn::BaseConvolutionLayer* instance, const std::vector<size_t>* val) {
			instance->kernel_size = *val;
	}

	// cv::dnn::BaseConvolutionLayer::strides() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:282
	// ("cv::dnn::BaseConvolutionLayer::strides", vec![(pred!(const, [], []), _)]),
	std::vector<size_t>* cv_dnn_BaseConvolutionLayer_propStrides_const(const cv::dnn::BaseConvolutionLayer* instance) {
			std::vector<size_t> ret = instance->strides;
			return new std::vector<size_t>(ret);
	}

	// cv::dnn::BaseConvolutionLayer::setStrides(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:282
	// ("cv::dnn::BaseConvolutionLayer::setStrides", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
	void cv_dnn_BaseConvolutionLayer_propStrides_const_vectorLsize_tG(cv::dnn::BaseConvolutionLayer* instance, const std::vector<size_t>* val) {
			instance->strides = *val;
	}

	// cv::dnn::BaseConvolutionLayer::dilations() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:282
	// ("cv::dnn::BaseConvolutionLayer::dilations", vec![(pred!(const, [], []), _)]),
	std::vector<size_t>* cv_dnn_BaseConvolutionLayer_propDilations_const(const cv::dnn::BaseConvolutionLayer* instance) {
			std::vector<size_t> ret = instance->dilations;
			return new std::vector<size_t>(ret);
	}

	// cv::dnn::BaseConvolutionLayer::setDilations(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:282
	// ("cv::dnn::BaseConvolutionLayer::setDilations", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
	void cv_dnn_BaseConvolutionLayer_propDilations_const_vectorLsize_tG(cv::dnn::BaseConvolutionLayer* instance, const std::vector<size_t>* val) {
			instance->dilations = *val;
	}

	// cv::dnn::BaseConvolutionLayer::pads_begin() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:283
	// ("cv::dnn::BaseConvolutionLayer::pads_begin", vec![(pred!(const, [], []), _)]),
	std::vector<size_t>* cv_dnn_BaseConvolutionLayer_propPads_begin_const(const cv::dnn::BaseConvolutionLayer* instance) {
			std::vector<size_t> ret = instance->pads_begin;
			return new std::vector<size_t>(ret);
	}

	// cv::dnn::BaseConvolutionLayer::setPads_begin(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:283
	// ("cv::dnn::BaseConvolutionLayer::setPads_begin", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
	void cv_dnn_BaseConvolutionLayer_propPads_begin_const_vectorLsize_tG(cv::dnn::BaseConvolutionLayer* instance, const std::vector<size_t>* val) {
			instance->pads_begin = *val;
	}

	// cv::dnn::BaseConvolutionLayer::pads_end() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:283
	// ("cv::dnn::BaseConvolutionLayer::pads_end", vec![(pred!(const, [], []), _)]),
	std::vector<size_t>* cv_dnn_BaseConvolutionLayer_propPads_end_const(const cv::dnn::BaseConvolutionLayer* instance) {
			std::vector<size_t> ret = instance->pads_end;
			return new std::vector<size_t>(ret);
	}

	// cv::dnn::BaseConvolutionLayer::setPads_end(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:283
	// ("cv::dnn::BaseConvolutionLayer::setPads_end", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
	void cv_dnn_BaseConvolutionLayer_propPads_end_const_vectorLsize_tG(cv::dnn::BaseConvolutionLayer* instance, const std::vector<size_t>* val) {
			instance->pads_end = *val;
	}

	// cv::dnn::BaseConvolutionLayer::padMode() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:284
	// ("cv::dnn::BaseConvolutionLayer::padMode", vec![(pred!(const, [], []), _)]),
	void* cv_dnn_BaseConvolutionLayer_propPadMode_const(const cv::dnn::BaseConvolutionLayer* instance) {
			cv::String ret = instance->padMode;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::dnn::BaseConvolutionLayer::setPadMode(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:284
	// ("cv::dnn::BaseConvolutionLayer::setPadMode", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	void cv_dnn_BaseConvolutionLayer_propPadMode_const_String(cv::dnn::BaseConvolutionLayer* instance, const char* val) {
			instance->padMode = std::string(val);
	}

	// cv::dnn::BaseConvolutionLayer::numOutput() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:285
	// ("cv::dnn::BaseConvolutionLayer::numOutput", vec![(pred!(const, [], []), _)]),
	int cv_dnn_BaseConvolutionLayer_propNumOutput_const(const cv::dnn::BaseConvolutionLayer* instance) {
			int ret = instance->numOutput;
			return ret;
	}

	// cv::dnn::BaseConvolutionLayer::setNumOutput(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:285
	// ("cv::dnn::BaseConvolutionLayer::setNumOutput", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_BaseConvolutionLayer_propNumOutput_const_int(cv::dnn::BaseConvolutionLayer* instance, const int val) {
			instance->numOutput = val;
	}

	// cv::dnn::BaseConvolutionLayer::to_Algorithm() generated
	// ("cv::dnn::BaseConvolutionLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_BaseConvolutionLayer_to_Algorithm(cv::dnn::BaseConvolutionLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::BaseConvolutionLayer::to_Layer() generated
	// ("cv::dnn::BaseConvolutionLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_BaseConvolutionLayer_to_Layer(cv::dnn::BaseConvolutionLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::BaseConvolutionLayer::delete() generated
	// ("cv::dnn::BaseConvolutionLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_BaseConvolutionLayer_delete(cv::dnn::BaseConvolutionLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:933
	// ("cv::dnn::BatchNormLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_BatchNormLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::BatchNormLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::BatchNormLayer> ret = cv::dnn::BatchNormLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::BatchNormLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::BatchNormLayer::defaultNew() generated
	// ("cv::dnn::BatchNormLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::BatchNormLayer* cv_dnn_BatchNormLayer_defaultNew_const() {
			cv::dnn::BatchNormLayer* ret = new cv::dnn::BatchNormLayer();
			return ret;
	}

	// cv::dnn::BatchNormLayer::hasWeights() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:930
	// ("cv::dnn::BatchNormLayer::hasWeights", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_BatchNormLayer_propHasWeights_const(const cv::dnn::BatchNormLayer* instance) {
			bool ret = instance->hasWeights;
			return ret;
	}

	// cv::dnn::BatchNormLayer::setHasWeights(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:930
	// ("cv::dnn::BatchNormLayer::setHasWeights", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_dnn_BatchNormLayer_propHasWeights_const_bool(cv::dnn::BatchNormLayer* instance, const bool val) {
			instance->hasWeights = val;
	}

	// cv::dnn::BatchNormLayer::hasBias() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:930
	// ("cv::dnn::BatchNormLayer::hasBias", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_BatchNormLayer_propHasBias_const(const cv::dnn::BatchNormLayer* instance) {
			bool ret = instance->hasBias;
			return ret;
	}

	// cv::dnn::BatchNormLayer::setHasBias(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:930
	// ("cv::dnn::BatchNormLayer::setHasBias", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_dnn_BatchNormLayer_propHasBias_const_bool(cv::dnn::BatchNormLayer* instance, const bool val) {
			instance->hasBias = val;
	}

	// cv::dnn::BatchNormLayer::epsilon() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:931
	// ("cv::dnn::BatchNormLayer::epsilon", vec![(pred!(const, [], []), _)]),
	float cv_dnn_BatchNormLayer_propEpsilon_const(const cv::dnn::BatchNormLayer* instance) {
			float ret = instance->epsilon;
			return ret;
	}

	// cv::dnn::BatchNormLayer::setEpsilon(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:931
	// ("cv::dnn::BatchNormLayer::setEpsilon", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_BatchNormLayer_propEpsilon_const_float(cv::dnn::BatchNormLayer* instance, const float val) {
			instance->epsilon = val;
	}

	// cv::dnn::BatchNormLayer::to_ActivationLayer() generated
	// ("cv::dnn::BatchNormLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_BatchNormLayer_to_ActivationLayer(cv::dnn::BatchNormLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::BatchNormLayer::to_Algorithm() generated
	// ("cv::dnn::BatchNormLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_BatchNormLayer_to_Algorithm(cv::dnn::BatchNormLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::BatchNormLayer::to_Layer() generated
	// ("cv::dnn::BatchNormLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_BatchNormLayer_to_Layer(cv::dnn::BatchNormLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::BatchNormLayer::delete() generated
	// ("cv::dnn::BatchNormLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_BatchNormLayer_delete(cv::dnn::BatchNormLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:941
	// ("cv::dnn::BatchNormLayerInt8::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_BatchNormLayerInt8_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::BatchNormLayerInt8>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::BatchNormLayerInt8> ret = cv::dnn::BatchNormLayerInt8::create(*params);
			Ok(new cv::Ptr<cv::dnn::BatchNormLayerInt8>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::BatchNormLayerInt8::defaultNew() generated
	// ("cv::dnn::BatchNormLayerInt8::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::BatchNormLayerInt8* cv_dnn_BatchNormLayerInt8_defaultNew_const() {
			cv::dnn::BatchNormLayerInt8* ret = new cv::dnn::BatchNormLayerInt8();
			return ret;
	}

	// cv::dnn::BatchNormLayerInt8::input_sc() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:939
	// ("cv::dnn::BatchNormLayerInt8::input_sc", vec![(pred!(const, [], []), _)]),
	float cv_dnn_BatchNormLayerInt8_propInput_sc_const(const cv::dnn::BatchNormLayerInt8* instance) {
			float ret = instance->input_sc;
			return ret;
	}

	// cv::dnn::BatchNormLayerInt8::setInput_sc(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:939
	// ("cv::dnn::BatchNormLayerInt8::setInput_sc", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_BatchNormLayerInt8_propInput_sc_const_float(cv::dnn::BatchNormLayerInt8* instance, const float val) {
			instance->input_sc = val;
	}

	// cv::dnn::BatchNormLayerInt8::output_sc() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:939
	// ("cv::dnn::BatchNormLayerInt8::output_sc", vec![(pred!(const, [], []), _)]),
	float cv_dnn_BatchNormLayerInt8_propOutput_sc_const(const cv::dnn::BatchNormLayerInt8* instance) {
			float ret = instance->output_sc;
			return ret;
	}

	// cv::dnn::BatchNormLayerInt8::setOutput_sc(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:939
	// ("cv::dnn::BatchNormLayerInt8::setOutput_sc", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_BatchNormLayerInt8_propOutput_sc_const_float(cv::dnn::BatchNormLayerInt8* instance, const float val) {
			instance->output_sc = val;
	}

	// cv::dnn::BatchNormLayerInt8::input_zp() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:940
	// ("cv::dnn::BatchNormLayerInt8::input_zp", vec![(pred!(const, [], []), _)]),
	int cv_dnn_BatchNormLayerInt8_propInput_zp_const(const cv::dnn::BatchNormLayerInt8* instance) {
			int ret = instance->input_zp;
			return ret;
	}

	// cv::dnn::BatchNormLayerInt8::setInput_zp(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:940
	// ("cv::dnn::BatchNormLayerInt8::setInput_zp", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_BatchNormLayerInt8_propInput_zp_const_int(cv::dnn::BatchNormLayerInt8* instance, const int val) {
			instance->input_zp = val;
	}

	// cv::dnn::BatchNormLayerInt8::output_zp() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:940
	// ("cv::dnn::BatchNormLayerInt8::output_zp", vec![(pred!(const, [], []), _)]),
	int cv_dnn_BatchNormLayerInt8_propOutput_zp_const(const cv::dnn::BatchNormLayerInt8* instance) {
			int ret = instance->output_zp;
			return ret;
	}

	// cv::dnn::BatchNormLayerInt8::setOutput_zp(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:940
	// ("cv::dnn::BatchNormLayerInt8::setOutput_zp", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_BatchNormLayerInt8_propOutput_zp_const_int(cv::dnn::BatchNormLayerInt8* instance, const int val) {
			instance->output_zp = val;
	}

	// cv::dnn::BatchNormLayerInt8::to_ActivationLayer() generated
	// ("cv::dnn::BatchNormLayerInt8::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_BatchNormLayerInt8_to_ActivationLayer(cv::dnn::BatchNormLayerInt8* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::BatchNormLayerInt8::to_Algorithm() generated
	// ("cv::dnn::BatchNormLayerInt8::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_BatchNormLayerInt8_to_Algorithm(cv::dnn::BatchNormLayerInt8* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::BatchNormLayerInt8::to_BatchNormLayer() generated
	// ("cv::dnn::BatchNormLayerInt8::to_BatchNormLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::BatchNormLayer* cv_dnn_BatchNormLayerInt8_to_BatchNormLayer(cv::dnn::BatchNormLayerInt8* instance) {
			return dynamic_cast<cv::dnn::BatchNormLayer*>(instance);
	}

	// cv::dnn::BatchNormLayerInt8::to_Layer() generated
	// ("cv::dnn::BatchNormLayerInt8::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_BatchNormLayerInt8_to_Layer(cv::dnn::BatchNormLayerInt8* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::BatchNormLayerInt8::delete() generated
	// ("cv::dnn::BatchNormLayerInt8::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_BatchNormLayerInt8_delete(cv::dnn::BatchNormLayerInt8* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:77
	// ("cv::dnn::BlankLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_BlankLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::Layer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::BlankLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::Layer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::BlankLayer::defaultNew() generated
	// ("cv::dnn::BlankLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::BlankLayer* cv_dnn_BlankLayer_defaultNew_const() {
			cv::dnn::BlankLayer* ret = new cv::dnn::BlankLayer();
			return ret;
	}

	// cv::dnn::BlankLayer::to_Algorithm() generated
	// ("cv::dnn::BlankLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_BlankLayer_to_Algorithm(cv::dnn::BlankLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::BlankLayer::to_Layer() generated
	// ("cv::dnn::BlankLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_BlankLayer_to_Layer(cv::dnn::BlankLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::BlankLayer::delete() generated
	// ("cv::dnn::BlankLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_BlankLayer_delete(cv::dnn::BlankLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:699
	// ("cv::dnn::CeilLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_CeilLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::CeilLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::CeilLayer> ret = cv::dnn::CeilLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::CeilLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::CeilLayer::defaultNew() generated
	// ("cv::dnn::CeilLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::CeilLayer* cv_dnn_CeilLayer_defaultNew_const() {
			cv::dnn::CeilLayer* ret = new cv::dnn::CeilLayer();
			return ret;
	}

	// cv::dnn::CeilLayer::to_ActivationLayer() generated
	// ("cv::dnn::CeilLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_CeilLayer_to_ActivationLayer(cv::dnn::CeilLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::CeilLayer::to_Algorithm() generated
	// ("cv::dnn::CeilLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_CeilLayer_to_Algorithm(cv::dnn::CeilLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::CeilLayer::to_Layer() generated
	// ("cv::dnn::CeilLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_CeilLayer_to_Layer(cv::dnn::CeilLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::CeilLayer::delete() generated
	// ("cv::dnn::CeilLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_CeilLayer_delete(cv::dnn::CeilLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:827
	// ("cv::dnn::CeluLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_CeluLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::CeluLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::CeluLayer> ret = cv::dnn::CeluLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::CeluLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::CeluLayer::defaultNew() generated
	// ("cv::dnn::CeluLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::CeluLayer* cv_dnn_CeluLayer_defaultNew_const() {
			cv::dnn::CeluLayer* ret = new cv::dnn::CeluLayer();
			return ret;
	}

	// cv::dnn::CeluLayer::alpha() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:825
	// ("cv::dnn::CeluLayer::alpha", vec![(pred!(const, [], []), _)]),
	float cv_dnn_CeluLayer_propAlpha_const(const cv::dnn::CeluLayer* instance) {
			float ret = instance->alpha;
			return ret;
	}

	// cv::dnn::CeluLayer::setAlpha(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:825
	// ("cv::dnn::CeluLayer::setAlpha", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_CeluLayer_propAlpha_const_float(cv::dnn::CeluLayer* instance, const float val) {
			instance->alpha = val;
	}

	// cv::dnn::CeluLayer::to_ActivationLayer() generated
	// ("cv::dnn::CeluLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_CeluLayer_to_ActivationLayer(cv::dnn::CeluLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::CeluLayer::to_Algorithm() generated
	// ("cv::dnn::CeluLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_CeluLayer_to_Algorithm(cv::dnn::CeluLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::CeluLayer::to_Layer() generated
	// ("cv::dnn::CeluLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_CeluLayer_to_Layer(cv::dnn::CeluLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::CeluLayer::delete() generated
	// ("cv::dnn::CeluLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_CeluLayer_delete(cv::dnn::CeluLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:633
	// ("cv::dnn::ChannelsPReLULayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ChannelsPReLULayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::Layer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::ChannelsPReLULayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::Layer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ChannelsPReLULayer::defaultNew() generated
	// ("cv::dnn::ChannelsPReLULayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::ChannelsPReLULayer* cv_dnn_ChannelsPReLULayer_defaultNew_const() {
			cv::dnn::ChannelsPReLULayer* ret = new cv::dnn::ChannelsPReLULayer();
			return ret;
	}

	// cv::dnn::ChannelsPReLULayer::to_ActivationLayer() generated
	// ("cv::dnn::ChannelsPReLULayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_ChannelsPReLULayer_to_ActivationLayer(cv::dnn::ChannelsPReLULayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::ChannelsPReLULayer::to_Algorithm() generated
	// ("cv::dnn::ChannelsPReLULayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_ChannelsPReLULayer_to_Algorithm(cv::dnn::ChannelsPReLULayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::ChannelsPReLULayer::to_Layer() generated
	// ("cv::dnn::ChannelsPReLULayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_ChannelsPReLULayer_to_Layer(cv::dnn::ChannelsPReLULayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::ChannelsPReLULayer::delete() generated
	// ("cv::dnn::ChannelsPReLULayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_ChannelsPReLULayer_delete(cv::dnn::ChannelsPReLULayer* instance) {
			delete instance;
	}

	// ClassificationModel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1516
	// ("cv::dnn::ClassificationModel::ClassificationModel", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_ClassificationModel_ClassificationModel(Result<cv::dnn::ClassificationModel*>* ocvrs_return) {
		try {
			cv::dnn::ClassificationModel* ret = new cv::dnn::ClassificationModel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ClassificationModel(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1524
	// ("cv::dnn::ClassificationModel::ClassificationModel", vec![(pred!(mut, ["model", "config"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_dnn_ClassificationModel_ClassificationModel_const_StringR_const_StringR(const char* model, const char* config, Result<cv::dnn::ClassificationModel*>* ocvrs_return) {
		try {
			cv::dnn::ClassificationModel* ret = new cv::dnn::ClassificationModel(std::string(model), std::string(config));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ClassificationModel::ClassificationModel(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1524
	// ("cv::dnn::ClassificationModel::ClassificationModel", vec![(pred!(mut, ["model"], ["const cv::String*"]), _)]),
	void cv_dnn_ClassificationModel_ClassificationModel_const_StringR(const char* model, Result<cv::dnn::ClassificationModel*>* ocvrs_return) {
		try {
			cv::dnn::ClassificationModel* ret = new cv::dnn::ClassificationModel(std::string(model));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ClassificationModel(const Net &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1530
	// ("cv::dnn::ClassificationModel::ClassificationModel", vec![(pred!(mut, ["network"], ["const cv::dnn::Net*"]), _)]),
	void cv_dnn_ClassificationModel_ClassificationModel_const_NetR(const cv::dnn::Net* network, Result<cv::dnn::ClassificationModel*>* ocvrs_return) {
		try {
			cv::dnn::ClassificationModel* ret = new cv::dnn::ClassificationModel(*network);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEnableSoftmaxPostProcessing(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1541
	// ("cv::dnn::ClassificationModel::setEnableSoftmaxPostProcessing", vec![(pred!(mut, ["enable"], ["bool"]), _)]),
	void cv_dnn_ClassificationModel_setEnableSoftmaxPostProcessing_bool(cv::dnn::ClassificationModel* instance, bool enable, Result<cv::dnn::ClassificationModel*>* ocvrs_return) {
		try {
			cv::dnn::ClassificationModel ret = instance->setEnableSoftmaxPostProcessing(enable);
			Ok(new cv::dnn::ClassificationModel(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEnableSoftmaxPostProcessing()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1548
	// ("cv::dnn::ClassificationModel::getEnableSoftmaxPostProcessing", vec![(pred!(const, [], []), _)]),
	void cv_dnn_ClassificationModel_getEnableSoftmaxPostProcessing_const(const cv::dnn::ClassificationModel* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getEnableSoftmaxPostProcessing();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// classify(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1553
	// ("cv::dnn::ClassificationModel::classify", vec![(pred!(mut, ["frame"], ["const cv::_InputArray*"]), _)]),
	void cv_dnn_ClassificationModel_classify_const__InputArrayR(cv::dnn::ClassificationModel* instance, const cv::_InputArray* frame, Result<std::pair<int, float>*>* ocvrs_return) {
		try {
			std::pair<int, float> ret = instance->classify(*frame);
			Ok(new std::pair<int, float>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// classify(InputArray, int &, float &)(InputArray, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1556
	// ("cv::dnn::ClassificationModel::classify", vec![(pred!(mut, ["frame", "classId", "conf"], ["const cv::_InputArray*", "int*", "float*"]), _)]),
	void cv_dnn_ClassificationModel_classify_const__InputArrayR_intR_floatR(cv::dnn::ClassificationModel* instance, const cv::_InputArray* frame, int* classId, float* conf, ResultVoid* ocvrs_return) {
		try {
			instance->classify(*frame, *classId, *conf);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ClassificationModel::implicitClone() generated
	// ("cv::dnn::ClassificationModel::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::dnn::ClassificationModel* cv_dnn_ClassificationModel_implicitClone_const(const cv::dnn::ClassificationModel* instance) {
			return new cv::dnn::ClassificationModel(*instance);
	}

	// cv::dnn::ClassificationModel::to_Model() generated
	// ("cv::dnn::ClassificationModel::to_Model", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Model* cv_dnn_ClassificationModel_to_Model(cv::dnn::ClassificationModel* instance) {
			return dynamic_cast<cv::dnn::Model*>(instance);
	}

	// cv::dnn::ClassificationModel::delete() generated
	// ("cv::dnn::ClassificationModel::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_ClassificationModel_delete(cv::dnn::ClassificationModel* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:987
	// ("cv::dnn::CompareLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_CompareLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::Layer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::CompareLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::Layer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::CompareLayer::defaultNew() generated
	// ("cv::dnn::CompareLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::CompareLayer* cv_dnn_CompareLayer_defaultNew_const() {
			cv::dnn::CompareLayer* ret = new cv::dnn::CompareLayer();
			return ret;
	}

	// cv::dnn::CompareLayer::to_Algorithm() generated
	// ("cv::dnn::CompareLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_CompareLayer_to_Algorithm(cv::dnn::CompareLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::CompareLayer::to_Layer() generated
	// ("cv::dnn::CompareLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_CompareLayer_to_Layer(cv::dnn::CompareLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::CompareLayer::delete() generated
	// ("cv::dnn::CompareLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_CompareLayer_delete(cv::dnn::CompareLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:501
	// ("cv::dnn::ConcatLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ConcatLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::ConcatLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::ConcatLayer> ret = cv::dnn::ConcatLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::ConcatLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ConcatLayer::defaultNew() generated
	// ("cv::dnn::ConcatLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::ConcatLayer* cv_dnn_ConcatLayer_defaultNew_const() {
			cv::dnn::ConcatLayer* ret = new cv::dnn::ConcatLayer();
			return ret;
	}

	// cv::dnn::ConcatLayer::axis() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:491
	// ("cv::dnn::ConcatLayer::axis", vec![(pred!(const, [], []), _)]),
	int cv_dnn_ConcatLayer_propAxis_const(const cv::dnn::ConcatLayer* instance) {
			int ret = instance->axis;
			return ret;
	}

	// cv::dnn::ConcatLayer::setAxis(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:491
	// ("cv::dnn::ConcatLayer::setAxis", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_ConcatLayer_propAxis_const_int(cv::dnn::ConcatLayer* instance, const int val) {
			instance->axis = val;
	}

	// cv::dnn::ConcatLayer::padding() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:498
	// ("cv::dnn::ConcatLayer::padding", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_ConcatLayer_propPadding_const(const cv::dnn::ConcatLayer* instance) {
			bool ret = instance->padding;
			return ret;
	}

	// cv::dnn::ConcatLayer::setPadding(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:498
	// ("cv::dnn::ConcatLayer::setPadding", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_dnn_ConcatLayer_propPadding_const_bool(cv::dnn::ConcatLayer* instance, const bool val) {
			instance->padding = val;
	}

	// cv::dnn::ConcatLayer::paddingValue() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:499
	// ("cv::dnn::ConcatLayer::paddingValue", vec![(pred!(const, [], []), _)]),
	int cv_dnn_ConcatLayer_propPaddingValue_const(const cv::dnn::ConcatLayer* instance) {
			int ret = instance->paddingValue;
			return ret;
	}

	// cv::dnn::ConcatLayer::setPaddingValue(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:499
	// ("cv::dnn::ConcatLayer::setPaddingValue", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_ConcatLayer_propPaddingValue_const_int(cv::dnn::ConcatLayer* instance, const int val) {
			instance->paddingValue = val;
	}

	// cv::dnn::ConcatLayer::to_Algorithm() generated
	// ("cv::dnn::ConcatLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_ConcatLayer_to_Algorithm(cv::dnn::ConcatLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::ConcatLayer::to_Layer() generated
	// ("cv::dnn::ConcatLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_ConcatLayer_to_Layer(cv::dnn::ConcatLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::ConcatLayer::delete() generated
	// ("cv::dnn::ConcatLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_ConcatLayer_delete(cv::dnn::ConcatLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:86
	// ("cv::dnn::ConstLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ConstLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::Layer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::ConstLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::Layer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ConstLayer::defaultNew() generated
	// ("cv::dnn::ConstLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::ConstLayer* cv_dnn_ConstLayer_defaultNew_const() {
			cv::dnn::ConstLayer* ret = new cv::dnn::ConstLayer();
			return ret;
	}

	// cv::dnn::ConstLayer::to_Algorithm() generated
	// ("cv::dnn::ConstLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_ConstLayer_to_Algorithm(cv::dnn::ConstLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::ConstLayer::to_Layer() generated
	// ("cv::dnn::ConstLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_ConstLayer_to_Layer(cv::dnn::ConstLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::ConstLayer::delete() generated
	// ("cv::dnn::ConstLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_ConstLayer_delete(cv::dnn::ConstLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:291
	// ("cv::dnn::ConvolutionLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ConvolutionLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::BaseConvolutionLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::BaseConvolutionLayer> ret = cv::dnn::ConvolutionLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::BaseConvolutionLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ConvolutionLayer::defaultNew() generated
	// ("cv::dnn::ConvolutionLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::ConvolutionLayer* cv_dnn_ConvolutionLayer_defaultNew_const() {
			cv::dnn::ConvolutionLayer* ret = new cv::dnn::ConvolutionLayer();
			return ret;
	}

	// cv::dnn::ConvolutionLayer::fusedActivation() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:292
	// ("cv::dnn::ConvolutionLayer::fusedActivation", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_ConvolutionLayer_propFusedActivation_const(const cv::dnn::ConvolutionLayer* instance) {
			bool ret = instance->fusedActivation;
			return ret;
	}

	// cv::dnn::ConvolutionLayer::setFusedActivation(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:292
	// ("cv::dnn::ConvolutionLayer::setFusedActivation", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_dnn_ConvolutionLayer_propFusedActivation_const_bool(cv::dnn::ConvolutionLayer* instance, const bool val) {
			instance->fusedActivation = val;
	}

	// cv::dnn::ConvolutionLayer::fusedAdd() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:293
	// ("cv::dnn::ConvolutionLayer::fusedAdd", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_ConvolutionLayer_propFusedAdd_const(const cv::dnn::ConvolutionLayer* instance) {
			bool ret = instance->fusedAdd;
			return ret;
	}

	// cv::dnn::ConvolutionLayer::setFusedAdd(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:293
	// ("cv::dnn::ConvolutionLayer::setFusedAdd", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_dnn_ConvolutionLayer_propFusedAdd_const_bool(cv::dnn::ConvolutionLayer* instance, const bool val) {
			instance->fusedAdd = val;
	}

	// cv::dnn::ConvolutionLayer::useWinograd() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:294
	// ("cv::dnn::ConvolutionLayer::useWinograd", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_ConvolutionLayer_propUseWinograd_const(const cv::dnn::ConvolutionLayer* instance) {
			bool ret = instance->useWinograd;
			return ret;
	}

	// cv::dnn::ConvolutionLayer::setUseWinograd(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:294
	// ("cv::dnn::ConvolutionLayer::setUseWinograd", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_dnn_ConvolutionLayer_propUseWinograd_const_bool(cv::dnn::ConvolutionLayer* instance, const bool val) {
			instance->useWinograd = val;
	}

	// cv::dnn::ConvolutionLayer::to_Algorithm() generated
	// ("cv::dnn::ConvolutionLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_ConvolutionLayer_to_Algorithm(cv::dnn::ConvolutionLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::ConvolutionLayer::to_BaseConvolutionLayer() generated
	// ("cv::dnn::ConvolutionLayer::to_BaseConvolutionLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::BaseConvolutionLayer* cv_dnn_ConvolutionLayer_to_BaseConvolutionLayer(cv::dnn::ConvolutionLayer* instance) {
			return dynamic_cast<cv::dnn::BaseConvolutionLayer*>(instance);
	}

	// cv::dnn::ConvolutionLayer::to_Layer() generated
	// ("cv::dnn::ConvolutionLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_ConvolutionLayer_to_Layer(cv::dnn::ConvolutionLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::ConvolutionLayer::delete() generated
	// ("cv::dnn::ConvolutionLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_ConvolutionLayer_delete(cv::dnn::ConvolutionLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:307
	// ("cv::dnn::ConvolutionLayerInt8::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ConvolutionLayerInt8_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::BaseConvolutionLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::BaseConvolutionLayer> ret = cv::dnn::ConvolutionLayerInt8::create(*params);
			Ok(new cv::Ptr<cv::dnn::BaseConvolutionLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ConvolutionLayerInt8::defaultNew() generated
	// ("cv::dnn::ConvolutionLayerInt8::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::ConvolutionLayerInt8* cv_dnn_ConvolutionLayerInt8_defaultNew_const() {
			cv::dnn::ConvolutionLayerInt8* ret = new cv::dnn::ConvolutionLayerInt8();
			return ret;
	}

	// cv::dnn::ConvolutionLayerInt8::input_zp() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:300
	// ("cv::dnn::ConvolutionLayerInt8::input_zp", vec![(pred!(const, [], []), _)]),
	int cv_dnn_ConvolutionLayerInt8_propInput_zp_const(const cv::dnn::ConvolutionLayerInt8* instance) {
			int ret = instance->input_zp;
			return ret;
	}

	// cv::dnn::ConvolutionLayerInt8::setInput_zp(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:300
	// ("cv::dnn::ConvolutionLayerInt8::setInput_zp", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_ConvolutionLayerInt8_propInput_zp_const_int(cv::dnn::ConvolutionLayerInt8* instance, const int val) {
			instance->input_zp = val;
	}

	// cv::dnn::ConvolutionLayerInt8::output_zp() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:300
	// ("cv::dnn::ConvolutionLayerInt8::output_zp", vec![(pred!(const, [], []), _)]),
	int cv_dnn_ConvolutionLayerInt8_propOutput_zp_const(const cv::dnn::ConvolutionLayerInt8* instance) {
			int ret = instance->output_zp;
			return ret;
	}

	// cv::dnn::ConvolutionLayerInt8::setOutput_zp(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:300
	// ("cv::dnn::ConvolutionLayerInt8::setOutput_zp", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_ConvolutionLayerInt8_propOutput_zp_const_int(cv::dnn::ConvolutionLayerInt8* instance, const int val) {
			instance->output_zp = val;
	}

	// cv::dnn::ConvolutionLayerInt8::input_sc() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:301
	// ("cv::dnn::ConvolutionLayerInt8::input_sc", vec![(pred!(const, [], []), _)]),
	float cv_dnn_ConvolutionLayerInt8_propInput_sc_const(const cv::dnn::ConvolutionLayerInt8* instance) {
			float ret = instance->input_sc;
			return ret;
	}

	// cv::dnn::ConvolutionLayerInt8::setInput_sc(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:301
	// ("cv::dnn::ConvolutionLayerInt8::setInput_sc", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_ConvolutionLayerInt8_propInput_sc_const_float(cv::dnn::ConvolutionLayerInt8* instance, const float val) {
			instance->input_sc = val;
	}

	// cv::dnn::ConvolutionLayerInt8::output_sc() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:301
	// ("cv::dnn::ConvolutionLayerInt8::output_sc", vec![(pred!(const, [], []), _)]),
	float cv_dnn_ConvolutionLayerInt8_propOutput_sc_const(const cv::dnn::ConvolutionLayerInt8* instance) {
			float ret = instance->output_sc;
			return ret;
	}

	// cv::dnn::ConvolutionLayerInt8::setOutput_sc(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:301
	// ("cv::dnn::ConvolutionLayerInt8::setOutput_sc", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_ConvolutionLayerInt8_propOutput_sc_const_float(cv::dnn::ConvolutionLayerInt8* instance, const float val) {
			instance->output_sc = val;
	}

	// cv::dnn::ConvolutionLayerInt8::per_channel() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:305
	// ("cv::dnn::ConvolutionLayerInt8::per_channel", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_ConvolutionLayerInt8_propPer_channel_const(const cv::dnn::ConvolutionLayerInt8* instance) {
			bool ret = instance->per_channel;
			return ret;
	}

	// cv::dnn::ConvolutionLayerInt8::setPer_channel(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:305
	// ("cv::dnn::ConvolutionLayerInt8::setPer_channel", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_dnn_ConvolutionLayerInt8_propPer_channel_const_bool(cv::dnn::ConvolutionLayerInt8* instance, const bool val) {
			instance->per_channel = val;
	}

	// cv::dnn::ConvolutionLayerInt8::useWinograd() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:306
	// ("cv::dnn::ConvolutionLayerInt8::useWinograd", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_ConvolutionLayerInt8_propUseWinograd_const(const cv::dnn::ConvolutionLayerInt8* instance) {
			bool ret = instance->useWinograd;
			return ret;
	}

	// cv::dnn::ConvolutionLayerInt8::setUseWinograd(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:306
	// ("cv::dnn::ConvolutionLayerInt8::setUseWinograd", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_dnn_ConvolutionLayerInt8_propUseWinograd_const_bool(cv::dnn::ConvolutionLayerInt8* instance, const bool val) {
			instance->useWinograd = val;
	}

	// cv::dnn::ConvolutionLayerInt8::to_Algorithm() generated
	// ("cv::dnn::ConvolutionLayerInt8::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_ConvolutionLayerInt8_to_Algorithm(cv::dnn::ConvolutionLayerInt8* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::ConvolutionLayerInt8::to_BaseConvolutionLayer() generated
	// ("cv::dnn::ConvolutionLayerInt8::to_BaseConvolutionLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::BaseConvolutionLayer* cv_dnn_ConvolutionLayerInt8_to_BaseConvolutionLayer(cv::dnn::ConvolutionLayerInt8* instance) {
			return dynamic_cast<cv::dnn::BaseConvolutionLayer*>(instance);
	}

	// cv::dnn::ConvolutionLayerInt8::to_Layer() generated
	// ("cv::dnn::ConvolutionLayerInt8::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_ConvolutionLayerInt8_to_Layer(cv::dnn::ConvolutionLayerInt8* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::ConvolutionLayerInt8::delete() generated
	// ("cv::dnn::ConvolutionLayerInt8::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_ConvolutionLayerInt8_delete(cv::dnn::ConvolutionLayerInt8* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:999
	// ("cv::dnn::CorrelationLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_CorrelationLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::CorrelationLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::CorrelationLayer> ret = cv::dnn::CorrelationLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::CorrelationLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::CorrelationLayer::defaultNew() generated
	// ("cv::dnn::CorrelationLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::CorrelationLayer* cv_dnn_CorrelationLayer_defaultNew_const() {
			cv::dnn::CorrelationLayer* ret = new cv::dnn::CorrelationLayer();
			return ret;
	}

	// cv::dnn::CorrelationLayer::to_Algorithm() generated
	// ("cv::dnn::CorrelationLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_CorrelationLayer_to_Algorithm(cv::dnn::CorrelationLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::CorrelationLayer::to_Layer() generated
	// ("cv::dnn::CorrelationLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_CorrelationLayer_to_Layer(cv::dnn::CorrelationLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::CorrelationLayer::delete() generated
	// ("cv::dnn::CorrelationLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_CorrelationLayer_delete(cv::dnn::CorrelationLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:771
	// ("cv::dnn::CosLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_CosLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::CosLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::CosLayer> ret = cv::dnn::CosLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::CosLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::CosLayer::defaultNew() generated
	// ("cv::dnn::CosLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::CosLayer* cv_dnn_CosLayer_defaultNew_const() {
			cv::dnn::CosLayer* ret = new cv::dnn::CosLayer();
			return ret;
	}

	// cv::dnn::CosLayer::to_ActivationLayer() generated
	// ("cv::dnn::CosLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_CosLayer_to_ActivationLayer(cv::dnn::CosLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::CosLayer::to_Algorithm() generated
	// ("cv::dnn::CosLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_CosLayer_to_Algorithm(cv::dnn::CosLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::CosLayer::to_Layer() generated
	// ("cv::dnn::CosLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_CosLayer_to_Layer(cv::dnn::CosLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::CosLayer::delete() generated
	// ("cv::dnn::CosLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_CosLayer_delete(cv::dnn::CosLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:777
	// ("cv::dnn::CoshLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_CoshLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::CoshLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::CoshLayer> ret = cv::dnn::CoshLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::CoshLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::CoshLayer::defaultNew() generated
	// ("cv::dnn::CoshLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::CoshLayer* cv_dnn_CoshLayer_defaultNew_const() {
			cv::dnn::CoshLayer* ret = new cv::dnn::CoshLayer();
			return ret;
	}

	// cv::dnn::CoshLayer::to_ActivationLayer() generated
	// ("cv::dnn::CoshLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_CoshLayer_to_ActivationLayer(cv::dnn::CoshLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::CoshLayer::to_Algorithm() generated
	// ("cv::dnn::CoshLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_CoshLayer_to_Algorithm(cv::dnn::CoshLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::CoshLayer::to_Layer() generated
	// ("cv::dnn::CoshLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_CoshLayer_to_Layer(cv::dnn::CoshLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::CoshLayer::delete() generated
	// ("cv::dnn::CoshLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_CoshLayer_delete(cv::dnn::CoshLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1113
	// ("cv::dnn::CropAndResizeLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_CropAndResizeLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::Layer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::CropAndResizeLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::Layer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::CropAndResizeLayer::defaultNew() generated
	// ("cv::dnn::CropAndResizeLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::CropAndResizeLayer* cv_dnn_CropAndResizeLayer_defaultNew_const() {
			cv::dnn::CropAndResizeLayer* ret = new cv::dnn::CropAndResizeLayer();
			return ret;
	}

	// cv::dnn::CropAndResizeLayer::to_Algorithm() generated
	// ("cv::dnn::CropAndResizeLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_CropAndResizeLayer_to_Algorithm(cv::dnn::CropAndResizeLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::CropAndResizeLayer::to_Layer() generated
	// ("cv::dnn::CropAndResizeLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_CropAndResizeLayer_to_Layer(cv::dnn::CropAndResizeLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::CropAndResizeLayer::delete() generated
	// ("cv::dnn::CropAndResizeLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_CropAndResizeLayer_delete(cv::dnn::CropAndResizeLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:899
	// ("cv::dnn::CropLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_CropLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::Layer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::CropLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::Layer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::CropLayer::defaultNew() generated
	// ("cv::dnn::CropLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::CropLayer* cv_dnn_CropLayer_defaultNew_const() {
			cv::dnn::CropLayer* ret = new cv::dnn::CropLayer();
			return ret;
	}

	// cv::dnn::CropLayer::to_Algorithm() generated
	// ("cv::dnn::CropLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_CropLayer_to_Algorithm(cv::dnn::CropLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::CropLayer::to_Layer() generated
	// ("cv::dnn::CropLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_CropLayer_to_Layer(cv::dnn::CropLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::CropLayer::delete() generated
	// ("cv::dnn::CropLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_CropLayer_delete(cv::dnn::CropLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1122
	// ("cv::dnn::CumSumLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_CumSumLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::CumSumLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::CumSumLayer> ret = cv::dnn::CumSumLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::CumSumLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::CumSumLayer::defaultNew() generated
	// ("cv::dnn::CumSumLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::CumSumLayer* cv_dnn_CumSumLayer_defaultNew_const() {
			cv::dnn::CumSumLayer* ret = new cv::dnn::CumSumLayer();
			return ret;
	}

	// cv::dnn::CumSumLayer::exclusive() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1119
	// ("cv::dnn::CumSumLayer::exclusive", vec![(pred!(const, [], []), _)]),
	int cv_dnn_CumSumLayer_propExclusive_const(const cv::dnn::CumSumLayer* instance) {
			int ret = instance->exclusive;
			return ret;
	}

	// cv::dnn::CumSumLayer::setExclusive(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1119
	// ("cv::dnn::CumSumLayer::setExclusive", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_CumSumLayer_propExclusive_const_int(cv::dnn::CumSumLayer* instance, const int val) {
			instance->exclusive = val;
	}

	// cv::dnn::CumSumLayer::reverse() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1120
	// ("cv::dnn::CumSumLayer::reverse", vec![(pred!(const, [], []), _)]),
	int cv_dnn_CumSumLayer_propReverse_const(const cv::dnn::CumSumLayer* instance) {
			int ret = instance->reverse;
			return ret;
	}

	// cv::dnn::CumSumLayer::setReverse(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1120
	// ("cv::dnn::CumSumLayer::setReverse", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_CumSumLayer_propReverse_const_int(cv::dnn::CumSumLayer* instance, const int val) {
			instance->reverse = val;
	}

	// cv::dnn::CumSumLayer::to_Algorithm() generated
	// ("cv::dnn::CumSumLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_CumSumLayer_to_Algorithm(cv::dnn::CumSumLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::CumSumLayer::to_Layer() generated
	// ("cv::dnn::CumSumLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_CumSumLayer_to_Layer(cv::dnn::CumSumLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::CumSumLayer::delete() generated
	// ("cv::dnn::CumSumLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_CumSumLayer_delete(cv::dnn::CumSumLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:993
	// ("cv::dnn::DataAugmentationLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_DataAugmentationLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::DataAugmentationLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::DataAugmentationLayer> ret = cv::dnn::DataAugmentationLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::DataAugmentationLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DataAugmentationLayer::defaultNew() generated
	// ("cv::dnn::DataAugmentationLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::DataAugmentationLayer* cv_dnn_DataAugmentationLayer_defaultNew_const() {
			cv::dnn::DataAugmentationLayer* ret = new cv::dnn::DataAugmentationLayer();
			return ret;
	}

	// cv::dnn::DataAugmentationLayer::to_Algorithm() generated
	// ("cv::dnn::DataAugmentationLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_DataAugmentationLayer_to_Algorithm(cv::dnn::DataAugmentationLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::DataAugmentationLayer::to_Layer() generated
	// ("cv::dnn::DataAugmentationLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_DataAugmentationLayer_to_Layer(cv::dnn::DataAugmentationLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::DataAugmentationLayer::delete() generated
	// ("cv::dnn::DataAugmentationLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_DataAugmentationLayer_delete(cv::dnn::DataAugmentationLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:313
	// ("cv::dnn::DeconvolutionLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_DeconvolutionLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::BaseConvolutionLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::BaseConvolutionLayer> ret = cv::dnn::DeconvolutionLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::BaseConvolutionLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DeconvolutionLayer::defaultNew() generated
	// ("cv::dnn::DeconvolutionLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::DeconvolutionLayer* cv_dnn_DeconvolutionLayer_defaultNew_const() {
			cv::dnn::DeconvolutionLayer* ret = new cv::dnn::DeconvolutionLayer();
			return ret;
	}

	// cv::dnn::DeconvolutionLayer::to_Algorithm() generated
	// ("cv::dnn::DeconvolutionLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_DeconvolutionLayer_to_Algorithm(cv::dnn::DeconvolutionLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::DeconvolutionLayer::to_BaseConvolutionLayer() generated
	// ("cv::dnn::DeconvolutionLayer::to_BaseConvolutionLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::BaseConvolutionLayer* cv_dnn_DeconvolutionLayer_to_BaseConvolutionLayer(cv::dnn::DeconvolutionLayer* instance) {
			return dynamic_cast<cv::dnn::BaseConvolutionLayer*>(instance);
	}

	// cv::dnn::DeconvolutionLayer::to_Layer() generated
	// ("cv::dnn::DeconvolutionLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_DeconvolutionLayer_to_Layer(cv::dnn::DeconvolutionLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::DeconvolutionLayer::delete() generated
	// ("cv::dnn::DeconvolutionLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_DeconvolutionLayer_delete(cv::dnn::DeconvolutionLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1193
	// ("cv::dnn::DepthToSpaceLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_DepthToSpaceLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::DepthToSpaceLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::DepthToSpaceLayer> ret = cv::dnn::DepthToSpaceLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::DepthToSpaceLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DepthToSpaceLayer::defaultNew() generated
	// ("cv::dnn::DepthToSpaceLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::DepthToSpaceLayer* cv_dnn_DepthToSpaceLayer_defaultNew_const() {
			cv::dnn::DepthToSpaceLayer* ret = new cv::dnn::DepthToSpaceLayer();
			return ret;
	}

	// cv::dnn::DepthToSpaceLayer::to_Algorithm() generated
	// ("cv::dnn::DepthToSpaceLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_DepthToSpaceLayer_to_Algorithm(cv::dnn::DepthToSpaceLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::DepthToSpaceLayer::to_Layer() generated
	// ("cv::dnn::DepthToSpaceLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_DepthToSpaceLayer_to_Layer(cv::dnn::DepthToSpaceLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::DepthToSpaceLayer::delete() generated
	// ("cv::dnn::DepthToSpaceLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_DepthToSpaceLayer_delete(cv::dnn::DepthToSpaceLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:478
	// ("cv::dnn::DequantizeLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_DequantizeLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::DequantizeLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::DequantizeLayer> ret = cv::dnn::DequantizeLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::DequantizeLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DequantizeLayer::defaultNew() generated
	// ("cv::dnn::DequantizeLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::DequantizeLayer* cv_dnn_DequantizeLayer_defaultNew_const() {
			cv::dnn::DequantizeLayer* ret = new cv::dnn::DequantizeLayer();
			return ret;
	}

	// cv::dnn::DequantizeLayer::scales() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:476
	// ("cv::dnn::DequantizeLayer::scales", vec![(pred!(const, [], []), _)]),
	std::vector<float>* cv_dnn_DequantizeLayer_propScales_const(const cv::dnn::DequantizeLayer* instance) {
			std::vector<float> ret = instance->scales;
			return new std::vector<float>(ret);
	}

	// cv::dnn::DequantizeLayer::setScales(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:476
	// ("cv::dnn::DequantizeLayer::setScales", vec![(pred!(mut, ["val"], ["const std::vector<float>"]), _)]),
	void cv_dnn_DequantizeLayer_propScales_const_vectorLfloatG(cv::dnn::DequantizeLayer* instance, const std::vector<float>* val) {
			instance->scales = *val;
	}

	// cv::dnn::DequantizeLayer::zeropoints() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:477
	// ("cv::dnn::DequantizeLayer::zeropoints", vec![(pred!(const, [], []), _)]),
	std::vector<int>* cv_dnn_DequantizeLayer_propZeropoints_const(const cv::dnn::DequantizeLayer* instance) {
			std::vector<int> ret = instance->zeropoints;
			return new std::vector<int>(ret);
	}

	// cv::dnn::DequantizeLayer::setZeropoints(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:477
	// ("cv::dnn::DequantizeLayer::setZeropoints", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
	void cv_dnn_DequantizeLayer_propZeropoints_const_vectorLintG(cv::dnn::DequantizeLayer* instance, const std::vector<int>* val) {
			instance->zeropoints = *val;
	}

	// cv::dnn::DequantizeLayer::to_Algorithm() generated
	// ("cv::dnn::DequantizeLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_DequantizeLayer_to_Algorithm(cv::dnn::DequantizeLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::DequantizeLayer::to_Layer() generated
	// ("cv::dnn::DequantizeLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_DequantizeLayer_to_Layer(cv::dnn::DequantizeLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::DequantizeLayer::delete() generated
	// ("cv::dnn::DequantizeLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_DequantizeLayer_delete(cv::dnn::DequantizeLayer* instance) {
			delete instance;
	}

	// DetectionModel(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1637
	// ("cv::dnn::DetectionModel::DetectionModel", vec![(pred!(mut, ["model", "config"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_dnn_DetectionModel_DetectionModel_const_StringR_const_StringR(const char* model, const char* config, Result<cv::dnn::DetectionModel*>* ocvrs_return) {
		try {
			cv::dnn::DetectionModel* ret = new cv::dnn::DetectionModel(std::string(model), std::string(config));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DetectionModel::DetectionModel(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1637
	// ("cv::dnn::DetectionModel::DetectionModel", vec![(pred!(mut, ["model"], ["const cv::String*"]), _)]),
	void cv_dnn_DetectionModel_DetectionModel_const_StringR(const char* model, Result<cv::dnn::DetectionModel*>* ocvrs_return) {
		try {
			cv::dnn::DetectionModel* ret = new cv::dnn::DetectionModel(std::string(model));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DetectionModel(const Net &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1643
	// ("cv::dnn::DetectionModel::DetectionModel", vec![(pred!(mut, ["network"], ["const cv::dnn::Net*"]), _)]),
	void cv_dnn_DetectionModel_DetectionModel_const_NetR(const cv::dnn::Net* network, Result<cv::dnn::DetectionModel*>* ocvrs_return) {
		try {
			cv::dnn::DetectionModel* ret = new cv::dnn::DetectionModel(*network);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DetectionModel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1646
	// ("cv::dnn::DetectionModel::DetectionModel", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_DetectionModel_DetectionModel(Result<cv::dnn::DetectionModel*>* ocvrs_return) {
		try {
			cv::dnn::DetectionModel* ret = new cv::dnn::DetectionModel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNmsAcrossClasses(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1654
	// ("cv::dnn::DetectionModel::setNmsAcrossClasses", vec![(pred!(mut, ["value"], ["bool"]), _)]),
	void cv_dnn_DetectionModel_setNmsAcrossClasses_bool(cv::dnn::DetectionModel* instance, bool value, Result<cv::dnn::DetectionModel*>* ocvrs_return) {
		try {
			cv::dnn::DetectionModel ret = instance->setNmsAcrossClasses(value);
			Ok(new cv::dnn::DetectionModel(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNmsAcrossClasses()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1660
	// ("cv::dnn::DetectionModel::getNmsAcrossClasses", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_DetectionModel_getNmsAcrossClasses(cv::dnn::DetectionModel* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getNmsAcrossClasses();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detect(InputArray, std::vector<int> &, std::vector<float> &, std::vector<Rect> &, float, float)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1670
	// ("cv::dnn::DetectionModel::detect", vec![(pred!(mut, ["frame", "classIds", "confidences", "boxes", "confThreshold", "nmsThreshold"], ["const cv::_InputArray*", "std::vector<int>*", "std::vector<float>*", "std::vector<cv::Rect>*", "float", "float"]), _)]),
	void cv_dnn_DetectionModel_detect_const__InputArrayR_vectorLintGR_vectorLfloatGR_vectorLRectGR_float_float(cv::dnn::DetectionModel* instance, const cv::_InputArray* frame, std::vector<int>* classIds, std::vector<float>* confidences, std::vector<cv::Rect>* boxes, float confThreshold, float nmsThreshold, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*frame, *classIds, *confidences, *boxes, confThreshold, nmsThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DetectionModel::detect(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1670
	// ("cv::dnn::DetectionModel::detect", vec![(pred!(mut, ["frame", "classIds", "confidences", "boxes"], ["const cv::_InputArray*", "std::vector<int>*", "std::vector<float>*", "std::vector<cv::Rect>*"]), _)]),
	void cv_dnn_DetectionModel_detect_const__InputArrayR_vectorLintGR_vectorLfloatGR_vectorLRectGR(cv::dnn::DetectionModel* instance, const cv::_InputArray* frame, std::vector<int>* classIds, std::vector<float>* confidences, std::vector<cv::Rect>* boxes, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*frame, *classIds, *confidences, *boxes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DetectionModel::implicitClone() generated
	// ("cv::dnn::DetectionModel::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::dnn::DetectionModel* cv_dnn_DetectionModel_implicitClone_const(const cv::dnn::DetectionModel* instance) {
			return new cv::dnn::DetectionModel(*instance);
	}

	// cv::dnn::DetectionModel::to_Model() generated
	// ("cv::dnn::DetectionModel::to_Model", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Model* cv_dnn_DetectionModel_to_Model(cv::dnn::DetectionModel* instance) {
			return dynamic_cast<cv::dnn::Model*>(instance);
	}

	// cv::dnn::DetectionModel::delete() generated
	// ("cv::dnn::DetectionModel::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_DetectionModel_delete(cv::dnn::DetectionModel* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1045
	// ("cv::dnn::DetectionOutputLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_DetectionOutputLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::DetectionOutputLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::DetectionOutputLayer> ret = cv::dnn::DetectionOutputLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::DetectionOutputLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DetectionOutputLayer::defaultNew() generated
	// ("cv::dnn::DetectionOutputLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::DetectionOutputLayer* cv_dnn_DetectionOutputLayer_defaultNew_const() {
			cv::dnn::DetectionOutputLayer* ret = new cv::dnn::DetectionOutputLayer();
			return ret;
	}

	// cv::dnn::DetectionOutputLayer::to_Algorithm() generated
	// ("cv::dnn::DetectionOutputLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_DetectionOutputLayer_to_Algorithm(cv::dnn::DetectionOutputLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::DetectionOutputLayer::to_Layer() generated
	// ("cv::dnn::DetectionOutputLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_DetectionOutputLayer_to_Layer(cv::dnn::DetectionOutputLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::DetectionOutputLayer::delete() generated
	// ("cv::dnn::DetectionOutputLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_DetectionOutputLayer_delete(cv::dnn::DetectionOutputLayer* instance) {
			delete instance;
	}

	// has(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:122
	// ("cv::dnn::Dict::has", vec![(pred!(const, ["key"], ["const cv::String*"]), _)]),
	void cv_dnn_Dict_has_const_const_StringR(const cv::dnn::Dict* instance, const char* key, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->has(std::string(key));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ptr(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:125
	// ("cv::dnn::Dict::ptr", vec![(pred!(mut, ["key"], ["const cv::String*"]), _)]),
	void cv_dnn_Dict_ptr_const_StringR(cv::dnn::Dict* instance, const char* key, Result<cv::dnn::DictValue*>* ocvrs_return) {
		try {
			cv::dnn::DictValue* ret = instance->ptr(std::string(key));
			Ok(new cv::dnn::DictValue(*ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ptr(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:128
	// ("cv::dnn::Dict::ptr", vec![(pred!(const, ["key"], ["const cv::String*"]), _)]),
	void cv_dnn_Dict_ptr_const_const_StringR(const cv::dnn::Dict* instance, const char* key, Result<const cv::dnn::DictValue*>* ocvrs_return) {
		try {
			const cv::dnn::DictValue* ret = instance->ptr(std::string(key));
			Ok(new const cv::dnn::DictValue(*ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// get(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:131
	// ("cv::dnn::Dict::get", vec![(pred!(const, ["key"], ["const cv::String*"]), _)]),
	void cv_dnn_Dict_get_const_const_StringR(const cv::dnn::Dict* instance, const char* key, Result<cv::dnn::DictValue*>* ocvrs_return) {
		try {
			const cv::dnn::DictValue ret = instance->get(std::string(key));
			Ok(new const cv::dnn::DictValue(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::Dict::set(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:143
	// ("cv::dnn::Dict::set", vec![(pred!(mut, ["key", "value"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_dnn_Dict_set_const_cv_String_const_StringR_const_StringR(cv::dnn::Dict* instance, const char* key, const char* value, Result<void*>* ocvrs_return) {
		try {
			const cv::String ret = instance->set<cv::String>(std::string(key), cv::String(value));
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::Dict::set(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:143
	// ("cv::dnn::Dict::set", vec![(pred!(mut, ["key", "value"], ["const cv::String*", "const cv::dnn::DictValue*"]), _)]),
	void cv_dnn_Dict_set_const_cv_dnn_DictValue_const_StringR_const_DictValueR(cv::dnn::Dict* instance, const char* key, const cv::dnn::DictValue* value, Result<cv::dnn::DictValue*>* ocvrs_return) {
		try {
			const cv::dnn::DictValue ret = instance->set<cv::dnn::DictValue>(std::string(key), *value);
			Ok(new const cv::dnn::DictValue(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::Dict::set(InString, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:143
	// ("cv::dnn::Dict::set", vec![(pred!(mut, ["key", "value"], ["const cv::String*", "const double*"]), _)]),
	void cv_dnn_Dict_set_const_double_const_StringR_const_doubleR(cv::dnn::Dict* instance, const char* key, const double* value, Result<double>* ocvrs_return) {
		try {
			const double ret = instance->set<double>(std::string(key), *value);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::Dict::set(InString, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:143
	// ("cv::dnn::Dict::set", vec![(pred!(mut, ["key", "value"], ["const cv::String*", "const int64_t*"]), _)]),
	void cv_dnn_Dict_set_const_int64_t_const_StringR_const_int64_tR(cv::dnn::Dict* instance, const char* key, const int64_t* value, Result<int64_t>* ocvrs_return) {
		try {
			const int64_t ret = instance->set<int64_t>(std::string(key), *value);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// erase(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:146
	// ("cv::dnn::Dict::erase", vec![(pred!(mut, ["key"], ["const cv::String*"]), _)]),
	void cv_dnn_Dict_erase_const_StringR(cv::dnn::Dict* instance, const char* key, ResultVoid* ocvrs_return) {
		try {
			instance->erase(std::string(key));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::Dict::defaultNew() generated
	// ("cv::dnn::Dict::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::Dict* cv_dnn_Dict_defaultNew_const() {
			cv::dnn::Dict* ret = new cv::dnn::Dict();
			return ret;
	}

	// cv::dnn::Dict::delete() generated
	// ("cv::dnn::Dict::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_Dict_delete(cv::dnn::Dict* instance) {
			delete instance;
	}

	// DictValue(const DictValue &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:62
	// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, ["r"], ["const cv::dnn::DictValue*"]), _)]),
	void cv_dnn_DictValue_DictValue_const_DictValueR(const cv::dnn::DictValue* r, Result<cv::dnn::DictValue*>* ocvrs_return) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(*r);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DictValue(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:63
	// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, ["i"], ["bool"]), _)]),
	void cv_dnn_DictValue_DictValue_bool(bool i, Result<cv::dnn::DictValue*>* ocvrs_return) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DictValue(int64)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:64
	// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, ["i"], ["int64_t"]), _)]),
	void cv_dnn_DictValue_DictValue_int64_t(int64_t i, Result<cv::dnn::DictValue*>* ocvrs_return) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DictValue::DictValue() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:64
	// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_DictValue_DictValue(Result<cv::dnn::DictValue*>* ocvrs_return) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DictValue(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:65
	// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, ["i"], ["int"]), _)]),
	void cv_dnn_DictValue_DictValue_int(int i, Result<cv::dnn::DictValue*>* ocvrs_return) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DictValue(unsigned int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:66
	// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, ["p"], ["unsigned int"]), _)]),
	void cv_dnn_DictValue_DictValue_unsigned_int(unsigned int p, Result<cv::dnn::DictValue*>* ocvrs_return) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(p);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DictValue(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:67
	// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, ["p"], ["double"]), _)]),
	void cv_dnn_DictValue_DictValue_double(double p, Result<cv::dnn::DictValue*>* ocvrs_return) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(p);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DictValue(const char *)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:69
	// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, ["s"], ["const char*"]), _)]),
	void cv_dnn_DictValue_DictValue_const_charX(const char* s, Result<cv::dnn::DictValue*>* ocvrs_return) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(s);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DictValue::get(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:79
	// ("cv::dnn::DictValue::get", vec![(pred!(const, ["idx"], ["int"]), _)]),
	void cv_dnn_DictValue_get_cv_String_const_int(const cv::dnn::DictValue* instance, int idx, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->get<cv::String>(idx);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DictValue::get() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:79
	// ("cv::dnn::DictValue::get", vec![(pred!(const, [], []), _)]),
	void cv_dnn_DictValue_get_cv_String_const(const cv::dnn::DictValue* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->get<cv::String>();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DictValue::get(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:79
	// ("cv::dnn::DictValue::get", vec![(pred!(const, ["idx"], ["int"]), _)]),
	void cv_dnn_DictValue_get_double_const_int(const cv::dnn::DictValue* instance, int idx, Result<double>* ocvrs_return) {
		try {
			double ret = instance->get<double>(idx);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DictValue::get() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:79
	// ("cv::dnn::DictValue::get", vec![(pred!(const, [], []), _)]),
	void cv_dnn_DictValue_get_double_const(const cv::dnn::DictValue* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->get<double>();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DictValue::get(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:79
	// ("cv::dnn::DictValue::get", vec![(pred!(const, ["idx"], ["int"]), _)]),
	void cv_dnn_DictValue_get_int_const_int(const cv::dnn::DictValue* instance, int idx, Result<int>* ocvrs_return) {
		try {
			int ret = instance->get<int>(idx);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DictValue::get() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:79
	// ("cv::dnn::DictValue::get", vec![(pred!(const, [], []), _)]),
	void cv_dnn_DictValue_get_int_const(const cv::dnn::DictValue* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->get<int>();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DictValue::get(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:79
	// ("cv::dnn::DictValue::get", vec![(pred!(const, ["idx"], ["int"]), _)]),
	void cv_dnn_DictValue_get_int64_t_const_int(const cv::dnn::DictValue* instance, int idx, Result<int64_t>* ocvrs_return) {
		try {
			int64_t ret = instance->get<int64_t>(idx);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DictValue::get() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:79
	// ("cv::dnn::DictValue::get", vec![(pred!(const, [], []), _)]),
	void cv_dnn_DictValue_get_int64_t_const(const cv::dnn::DictValue* instance, Result<int64_t>* ocvrs_return) {
		try {
			int64_t ret = instance->get<int64_t>();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// size()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:81
	// ("cv::dnn::DictValue::size", vec![(pred!(const, [], []), _)]),
	void cv_dnn_DictValue_size_const(const cv::dnn::DictValue* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->size();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isInt()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:83
	// ("cv::dnn::DictValue::isInt", vec![(pred!(const, [], []), _)]),
	void cv_dnn_DictValue_isInt_const(const cv::dnn::DictValue* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isInt();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isString()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:84
	// ("cv::dnn::DictValue::isString", vec![(pred!(const, [], []), _)]),
	void cv_dnn_DictValue_isString_const(const cv::dnn::DictValue* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isString();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isReal()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:85
	// ("cv::dnn::DictValue::isReal", vec![(pred!(const, [], []), _)]),
	void cv_dnn_DictValue_isReal_const(const cv::dnn::DictValue* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isReal();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getIntValue(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:87
	// ("cv::dnn::DictValue::getIntValue", vec![(pred!(const, ["idx"], ["int"]), _)]),
	void cv_dnn_DictValue_getIntValue_const_int(const cv::dnn::DictValue* instance, int idx, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getIntValue(idx);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DictValue::getIntValue() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:87
	// ("cv::dnn::DictValue::getIntValue", vec![(pred!(const, [], []), _)]),
	void cv_dnn_DictValue_getIntValue_const(const cv::dnn::DictValue* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getIntValue();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRealValue(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:88
	// ("cv::dnn::DictValue::getRealValue", vec![(pred!(const, ["idx"], ["int"]), _)]),
	void cv_dnn_DictValue_getRealValue_const_int(const cv::dnn::DictValue* instance, int idx, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getRealValue(idx);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DictValue::getRealValue() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:88
	// ("cv::dnn::DictValue::getRealValue", vec![(pred!(const, [], []), _)]),
	void cv_dnn_DictValue_getRealValue_const(const cv::dnn::DictValue* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getRealValue();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getStringValue(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:89
	// ("cv::dnn::DictValue::getStringValue", vec![(pred!(const, ["idx"], ["int"]), _)]),
	void cv_dnn_DictValue_getStringValue_const_int(const cv::dnn::DictValue* instance, int idx, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getStringValue(idx);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DictValue::getStringValue() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:89
	// ("cv::dnn::DictValue::getStringValue", vec![(pred!(const, [], []), _)]),
	void cv_dnn_DictValue_getStringValue_const(const cv::dnn::DictValue* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getStringValue();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const DictValue &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dict.hpp:91
	// ("cv::dnn::DictValue::operator=", vec![(pred!(mut, ["r"], ["const cv::dnn::DictValue*"]), _)]),
	void cv_dnn_DictValue_operatorST_const_DictValueR(cv::dnn::DictValue* instance, const cv::dnn::DictValue* r, ResultVoid* ocvrs_return) {
		try {
			instance->operator=(*r);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DictValue::delete() generated
	// ("cv::dnn::DictValue::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_DictValue_delete(cv::dnn::DictValue* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:641
	// ("cv::dnn::ELULayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ELULayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::ELULayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::ELULayer> ret = cv::dnn::ELULayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::ELULayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ELULayer::defaultNew() generated
	// ("cv::dnn::ELULayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::ELULayer* cv_dnn_ELULayer_defaultNew_const() {
			cv::dnn::ELULayer* ret = new cv::dnn::ELULayer();
			return ret;
	}

	// cv::dnn::ELULayer::alpha() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:639
	// ("cv::dnn::ELULayer::alpha", vec![(pred!(const, [], []), _)]),
	float cv_dnn_ELULayer_propAlpha_const(const cv::dnn::ELULayer* instance) {
			float ret = instance->alpha;
			return ret;
	}

	// cv::dnn::ELULayer::setAlpha(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:639
	// ("cv::dnn::ELULayer::setAlpha", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_ELULayer_propAlpha_const_float(cv::dnn::ELULayer* instance, const float val) {
			instance->alpha = val;
	}

	// cv::dnn::ELULayer::to_ActivationLayer() generated
	// ("cv::dnn::ELULayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_ELULayer_to_ActivationLayer(cv::dnn::ELULayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::ELULayer::to_Algorithm() generated
	// ("cv::dnn::ELULayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_ELULayer_to_Algorithm(cv::dnn::ELULayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::ELULayer::to_Layer() generated
	// ("cv::dnn::ELULayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_ELULayer_to_Layer(cv::dnn::ELULayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::ELULayer::delete() generated
	// ("cv::dnn::ELULayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_ELULayer_delete(cv::dnn::ELULayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:274
	// ("cv::dnn::EinsumLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_EinsumLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::EinsumLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::EinsumLayer> ret = cv::dnn::EinsumLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::EinsumLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::EinsumLayer::defaultNew() generated
	// ("cv::dnn::EinsumLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::EinsumLayer* cv_dnn_EinsumLayer_defaultNew_const() {
			cv::dnn::EinsumLayer* ret = new cv::dnn::EinsumLayer();
			return ret;
	}

	// cv::dnn::EinsumLayer::to_Algorithm() generated
	// ("cv::dnn::EinsumLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_EinsumLayer_to_Algorithm(cv::dnn::EinsumLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::EinsumLayer::to_Layer() generated
	// ("cv::dnn::EinsumLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_EinsumLayer_to_Layer(cv::dnn::EinsumLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::EinsumLayer::delete() generated
	// ("cv::dnn::EinsumLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_EinsumLayer_delete(cv::dnn::EinsumLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:912
	// ("cv::dnn::EltwiseLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_EltwiseLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::EltwiseLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::EltwiseLayer> ret = cv::dnn::EltwiseLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::EltwiseLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::EltwiseLayer::defaultNew() generated
	// ("cv::dnn::EltwiseLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::EltwiseLayer* cv_dnn_EltwiseLayer_defaultNew_const() {
			cv::dnn::EltwiseLayer* ret = new cv::dnn::EltwiseLayer();
			return ret;
	}

	// cv::dnn::EltwiseLayer::to_Algorithm() generated
	// ("cv::dnn::EltwiseLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_EltwiseLayer_to_Algorithm(cv::dnn::EltwiseLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::EltwiseLayer::to_Layer() generated
	// ("cv::dnn::EltwiseLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_EltwiseLayer_to_Layer(cv::dnn::EltwiseLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::EltwiseLayer::delete() generated
	// ("cv::dnn::EltwiseLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_EltwiseLayer_delete(cv::dnn::EltwiseLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:918
	// ("cv::dnn::EltwiseLayerInt8::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_EltwiseLayerInt8_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::EltwiseLayerInt8>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::EltwiseLayerInt8> ret = cv::dnn::EltwiseLayerInt8::create(*params);
			Ok(new cv::Ptr<cv::dnn::EltwiseLayerInt8>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::EltwiseLayerInt8::defaultNew() generated
	// ("cv::dnn::EltwiseLayerInt8::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::EltwiseLayerInt8* cv_dnn_EltwiseLayerInt8_defaultNew_const() {
			cv::dnn::EltwiseLayerInt8* ret = new cv::dnn::EltwiseLayerInt8();
			return ret;
	}

	// cv::dnn::EltwiseLayerInt8::to_Algorithm() generated
	// ("cv::dnn::EltwiseLayerInt8::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_EltwiseLayerInt8_to_Algorithm(cv::dnn::EltwiseLayerInt8* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::EltwiseLayerInt8::to_Layer() generated
	// ("cv::dnn::EltwiseLayerInt8::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_EltwiseLayerInt8_to_Layer(cv::dnn::EltwiseLayerInt8* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::EltwiseLayerInt8::delete() generated
	// ("cv::dnn::EltwiseLayerInt8::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_EltwiseLayerInt8_delete(cv::dnn::EltwiseLayerInt8* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:783
	// ("cv::dnn::ErfLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ErfLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::ErfLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::ErfLayer> ret = cv::dnn::ErfLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::ErfLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ErfLayer::defaultNew() generated
	// ("cv::dnn::ErfLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::ErfLayer* cv_dnn_ErfLayer_defaultNew_const() {
			cv::dnn::ErfLayer* ret = new cv::dnn::ErfLayer();
			return ret;
	}

	// cv::dnn::ErfLayer::to_ActivationLayer() generated
	// ("cv::dnn::ErfLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_ErfLayer_to_ActivationLayer(cv::dnn::ErfLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::ErfLayer::to_Algorithm() generated
	// ("cv::dnn::ErfLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_ErfLayer_to_Algorithm(cv::dnn::ErfLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::ErfLayer::to_Layer() generated
	// ("cv::dnn::ErfLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_ErfLayer_to_Layer(cv::dnn::ErfLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::ErfLayer::delete() generated
	// ("cv::dnn::ErfLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_ErfLayer_delete(cv::dnn::ErfLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:693
	// ("cv::dnn::ExpLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ExpLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::ExpLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::ExpLayer> ret = cv::dnn::ExpLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::ExpLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ExpLayer::defaultNew() generated
	// ("cv::dnn::ExpLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::ExpLayer* cv_dnn_ExpLayer_defaultNew_const() {
			cv::dnn::ExpLayer* ret = new cv::dnn::ExpLayer();
			return ret;
	}

	// cv::dnn::ExpLayer::base() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:691
	// ("cv::dnn::ExpLayer::base", vec![(pred!(const, [], []), _)]),
	float cv_dnn_ExpLayer_propBase_const(const cv::dnn::ExpLayer* instance) {
			float ret = instance->base;
			return ret;
	}

	// cv::dnn::ExpLayer::setBase(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:691
	// ("cv::dnn::ExpLayer::setBase", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_ExpLayer_propBase_const_float(cv::dnn::ExpLayer* instance, const float val) {
			instance->base = val;
	}

	// cv::dnn::ExpLayer::scale() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:691
	// ("cv::dnn::ExpLayer::scale", vec![(pred!(const, [], []), _)]),
	float cv_dnn_ExpLayer_propScale_const(const cv::dnn::ExpLayer* instance) {
			float ret = instance->scale;
			return ret;
	}

	// cv::dnn::ExpLayer::setScale(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:691
	// ("cv::dnn::ExpLayer::setScale", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_ExpLayer_propScale_const_float(cv::dnn::ExpLayer* instance, const float val) {
			instance->scale = val;
	}

	// cv::dnn::ExpLayer::shift() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:691
	// ("cv::dnn::ExpLayer::shift", vec![(pred!(const, [], []), _)]),
	float cv_dnn_ExpLayer_propShift_const(const cv::dnn::ExpLayer* instance) {
			float ret = instance->shift;
			return ret;
	}

	// cv::dnn::ExpLayer::setShift(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:691
	// ("cv::dnn::ExpLayer::setShift", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_ExpLayer_propShift_const_float(cv::dnn::ExpLayer* instance, const float val) {
			instance->shift = val;
	}

	// cv::dnn::ExpLayer::to_ActivationLayer() generated
	// ("cv::dnn::ExpLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_ExpLayer_to_ActivationLayer(cv::dnn::ExpLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::ExpLayer::to_Algorithm() generated
	// ("cv::dnn::ExpLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_ExpLayer_to_Algorithm(cv::dnn::ExpLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::ExpLayer::to_Layer() generated
	// ("cv::dnn::ExpLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_ExpLayer_to_Layer(cv::dnn::ExpLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::ExpLayer::delete() generated
	// ("cv::dnn::ExpLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_ExpLayer_delete(cv::dnn::ExpLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1171
	// ("cv::dnn::ExpandLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ExpandLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::ExpandLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::ExpandLayer> ret = cv::dnn::ExpandLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::ExpandLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ExpandLayer::defaultNew() generated
	// ("cv::dnn::ExpandLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::ExpandLayer* cv_dnn_ExpandLayer_defaultNew_const() {
			cv::dnn::ExpandLayer* ret = new cv::dnn::ExpandLayer();
			return ret;
	}

	// cv::dnn::ExpandLayer::to_Algorithm() generated
	// ("cv::dnn::ExpandLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_ExpandLayer_to_Algorithm(cv::dnn::ExpandLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::ExpandLayer::to_Layer() generated
	// ("cv::dnn::ExpandLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_ExpandLayer_to_Layer(cv::dnn::ExpandLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::ExpandLayer::delete() generated
	// ("cv::dnn::ExpandLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_ExpandLayer_delete(cv::dnn::ExpandLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:462
	// ("cv::dnn::FlattenLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_FlattenLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::FlattenLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::FlattenLayer> ret = cv::dnn::FlattenLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::FlattenLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::FlattenLayer::defaultNew() generated
	// ("cv::dnn::FlattenLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::FlattenLayer* cv_dnn_FlattenLayer_defaultNew_const() {
			cv::dnn::FlattenLayer* ret = new cv::dnn::FlattenLayer();
			return ret;
	}

	// cv::dnn::FlattenLayer::to_Algorithm() generated
	// ("cv::dnn::FlattenLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_FlattenLayer_to_Algorithm(cv::dnn::FlattenLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::FlattenLayer::to_Layer() generated
	// ("cv::dnn::FlattenLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_FlattenLayer_to_Layer(cv::dnn::FlattenLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::FlattenLayer::delete() generated
	// ("cv::dnn::FlattenLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_FlattenLayer_delete(cv::dnn::FlattenLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:705
	// ("cv::dnn::FloorLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_FloorLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::FloorLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::FloorLayer> ret = cv::dnn::FloorLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::FloorLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::FloorLayer::defaultNew() generated
	// ("cv::dnn::FloorLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::FloorLayer* cv_dnn_FloorLayer_defaultNew_const() {
			cv::dnn::FloorLayer* ret = new cv::dnn::FloorLayer();
			return ret;
	}

	// cv::dnn::FloorLayer::to_ActivationLayer() generated
	// ("cv::dnn::FloorLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_FloorLayer_to_ActivationLayer(cv::dnn::FloorLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::FloorLayer::to_Algorithm() generated
	// ("cv::dnn::FloorLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_FloorLayer_to_Algorithm(cv::dnn::FloorLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::FloorLayer::to_Layer() generated
	// ("cv::dnn::FloorLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_FloorLayer_to_Layer(cv::dnn::FloorLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::FloorLayer::delete() generated
	// ("cv::dnn::FloorLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_FloorLayer_delete(cv::dnn::FloorLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1011
	// ("cv::dnn::FlowWarpLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_FlowWarpLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::FlowWarpLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::FlowWarpLayer> ret = cv::dnn::FlowWarpLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::FlowWarpLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::FlowWarpLayer::defaultNew() generated
	// ("cv::dnn::FlowWarpLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::FlowWarpLayer* cv_dnn_FlowWarpLayer_defaultNew_const() {
			cv::dnn::FlowWarpLayer* ret = new cv::dnn::FlowWarpLayer();
			return ret;
	}

	// cv::dnn::FlowWarpLayer::to_Algorithm() generated
	// ("cv::dnn::FlowWarpLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_FlowWarpLayer_to_Algorithm(cv::dnn::FlowWarpLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::FlowWarpLayer::to_Layer() generated
	// ("cv::dnn::FlowWarpLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_FlowWarpLayer_to_Layer(cv::dnn::FlowWarpLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::FlowWarpLayer::delete() generated
	// ("cv::dnn::FlowWarpLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_FlowWarpLayer_delete(cv::dnn::FlowWarpLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:199
	// ("cv::dnn::GRULayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_GRULayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::GRULayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::GRULayer> ret = cv::dnn::GRULayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::GRULayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::GRULayer::defaultNew() generated
	// ("cv::dnn::GRULayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::GRULayer* cv_dnn_GRULayer_defaultNew_const() {
			cv::dnn::GRULayer* ret = new cv::dnn::GRULayer();
			return ret;
	}

	// cv::dnn::GRULayer::to_Algorithm() generated
	// ("cv::dnn::GRULayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_GRULayer_to_Algorithm(cv::dnn::GRULayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::GRULayer::to_Layer() generated
	// ("cv::dnn::GRULayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_GRULayer_to_Layer(cv::dnn::GRULayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::GRULayer::delete() generated
	// ("cv::dnn::GRULayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_GRULayer_delete(cv::dnn::GRULayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:359
	// ("cv::dnn::GatherElementsLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_GatherElementsLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::GatherElementsLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::GatherElementsLayer> ret = cv::dnn::GatherElementsLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::GatherElementsLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::GatherElementsLayer::defaultNew() generated
	// ("cv::dnn::GatherElementsLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::GatherElementsLayer* cv_dnn_GatherElementsLayer_defaultNew_const() {
			cv::dnn::GatherElementsLayer* ret = new cv::dnn::GatherElementsLayer();
			return ret;
	}

	// cv::dnn::GatherElementsLayer::to_Algorithm() generated
	// ("cv::dnn::GatherElementsLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_GatherElementsLayer_to_Algorithm(cv::dnn::GatherElementsLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::GatherElementsLayer::to_Layer() generated
	// ("cv::dnn::GatherElementsLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_GatherElementsLayer_to_Layer(cv::dnn::GatherElementsLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::GatherElementsLayer::delete() generated
	// ("cv::dnn::GatherElementsLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_GatherElementsLayer_delete(cv::dnn::GatherElementsLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:343
	// ("cv::dnn::GatherLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_GatherLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::GatherLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::GatherLayer> ret = cv::dnn::GatherLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::GatherLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::GatherLayer::defaultNew() generated
	// ("cv::dnn::GatherLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::GatherLayer* cv_dnn_GatherLayer_defaultNew_const() {
			cv::dnn::GatherLayer* ret = new cv::dnn::GatherLayer();
			return ret;
	}

	// cv::dnn::GatherLayer::to_Algorithm() generated
	// ("cv::dnn::GatherLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_GatherLayer_to_Algorithm(cv::dnn::GatherLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::GatherLayer::to_Layer() generated
	// ("cv::dnn::GatherLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_GatherLayer_to_Layer(cv::dnn::GatherLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::GatherLayer::delete() generated
	// ("cv::dnn::GatherLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_GatherLayer_delete(cv::dnn::GatherLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:857
	// ("cv::dnn::GeluApproximationLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_GeluApproximationLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::GeluApproximationLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::GeluApproximationLayer> ret = cv::dnn::GeluApproximationLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::GeluApproximationLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::GeluApproximationLayer::defaultNew() generated
	// ("cv::dnn::GeluApproximationLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::GeluApproximationLayer* cv_dnn_GeluApproximationLayer_defaultNew_const() {
			cv::dnn::GeluApproximationLayer* ret = new cv::dnn::GeluApproximationLayer();
			return ret;
	}

	// cv::dnn::GeluApproximationLayer::to_ActivationLayer() generated
	// ("cv::dnn::GeluApproximationLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_GeluApproximationLayer_to_ActivationLayer(cv::dnn::GeluApproximationLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::GeluApproximationLayer::to_Algorithm() generated
	// ("cv::dnn::GeluApproximationLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_GeluApproximationLayer_to_Algorithm(cv::dnn::GeluApproximationLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::GeluApproximationLayer::to_Layer() generated
	// ("cv::dnn::GeluApproximationLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_GeluApproximationLayer_to_Layer(cv::dnn::GeluApproximationLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::GeluApproximationLayer::delete() generated
	// ("cv::dnn::GeluApproximationLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_GeluApproximationLayer_delete(cv::dnn::GeluApproximationLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:851
	// ("cv::dnn::GeluLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_GeluLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::GeluLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::GeluLayer> ret = cv::dnn::GeluLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::GeluLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::GeluLayer::defaultNew() generated
	// ("cv::dnn::GeluLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::GeluLayer* cv_dnn_GeluLayer_defaultNew_const() {
			cv::dnn::GeluLayer* ret = new cv::dnn::GeluLayer();
			return ret;
	}

	// cv::dnn::GeluLayer::to_ActivationLayer() generated
	// ("cv::dnn::GeluLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_GeluLayer_to_ActivationLayer(cv::dnn::GeluLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::GeluLayer::to_Algorithm() generated
	// ("cv::dnn::GeluLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_GeluLayer_to_Algorithm(cv::dnn::GeluLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::GeluLayer::to_Layer() generated
	// ("cv::dnn::GeluLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_GeluLayer_to_Layer(cv::dnn::GeluLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::GeluLayer::delete() generated
	// ("cv::dnn::GeluLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_GeluLayer_delete(cv::dnn::GeluLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1160
	// ("cv::dnn::GemmLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_GemmLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::GemmLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::GemmLayer> ret = cv::dnn::GemmLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::GemmLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::GemmLayer::defaultNew() generated
	// ("cv::dnn::GemmLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::GemmLayer* cv_dnn_GemmLayer_defaultNew_const() {
			cv::dnn::GemmLayer* ret = new cv::dnn::GemmLayer();
			return ret;
	}

	// cv::dnn::GemmLayer::trans_a() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1155
	// ("cv::dnn::GemmLayer::trans_a", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_GemmLayer_propTrans_a_const(const cv::dnn::GemmLayer* instance) {
			bool ret = instance->trans_a;
			return ret;
	}

	// cv::dnn::GemmLayer::setTrans_a(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1155
	// ("cv::dnn::GemmLayer::setTrans_a", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_dnn_GemmLayer_propTrans_a_const_bool(cv::dnn::GemmLayer* instance, const bool val) {
			instance->trans_a = val;
	}

	// cv::dnn::GemmLayer::trans_b() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1156
	// ("cv::dnn::GemmLayer::trans_b", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_GemmLayer_propTrans_b_const(const cv::dnn::GemmLayer* instance) {
			bool ret = instance->trans_b;
			return ret;
	}

	// cv::dnn::GemmLayer::setTrans_b(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1156
	// ("cv::dnn::GemmLayer::setTrans_b", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_dnn_GemmLayer_propTrans_b_const_bool(cv::dnn::GemmLayer* instance, const bool val) {
			instance->trans_b = val;
	}

	// cv::dnn::GemmLayer::alpha() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1157
	// ("cv::dnn::GemmLayer::alpha", vec![(pred!(const, [], []), _)]),
	float cv_dnn_GemmLayer_propAlpha_const(const cv::dnn::GemmLayer* instance) {
			float ret = instance->alpha;
			return ret;
	}

	// cv::dnn::GemmLayer::setAlpha(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1157
	// ("cv::dnn::GemmLayer::setAlpha", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_GemmLayer_propAlpha_const_float(cv::dnn::GemmLayer* instance, const float val) {
			instance->alpha = val;
	}

	// cv::dnn::GemmLayer::beta() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1158
	// ("cv::dnn::GemmLayer::beta", vec![(pred!(const, [], []), _)]),
	float cv_dnn_GemmLayer_propBeta_const(const cv::dnn::GemmLayer* instance) {
			float ret = instance->beta;
			return ret;
	}

	// cv::dnn::GemmLayer::setBeta(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1158
	// ("cv::dnn::GemmLayer::setBeta", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_GemmLayer_propBeta_const_float(cv::dnn::GemmLayer* instance, const float val) {
			instance->beta = val;
	}

	// cv::dnn::GemmLayer::to_Algorithm() generated
	// ("cv::dnn::GemmLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_GemmLayer_to_Algorithm(cv::dnn::GemmLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::GemmLayer::to_Layer() generated
	// ("cv::dnn::GemmLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_GemmLayer_to_Layer(cv::dnn::GemmLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::GemmLayer::delete() generated
	// ("cv::dnn::GemmLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_GemmLayer_delete(cv::dnn::GemmLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1188
	// ("cv::dnn::GroupNormLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_GroupNormLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::GroupNormLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::GroupNormLayer> ret = cv::dnn::GroupNormLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::GroupNormLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::GroupNormLayer::defaultNew() generated
	// ("cv::dnn::GroupNormLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::GroupNormLayer* cv_dnn_GroupNormLayer_defaultNew_const() {
			cv::dnn::GroupNormLayer* ret = new cv::dnn::GroupNormLayer();
			return ret;
	}

	// cv::dnn::GroupNormLayer::to_Algorithm() generated
	// ("cv::dnn::GroupNormLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_GroupNormLayer_to_Algorithm(cv::dnn::GroupNormLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::GroupNormLayer::to_Layer() generated
	// ("cv::dnn::GroupNormLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_GroupNormLayer_to_Layer(cv::dnn::GroupNormLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::GroupNormLayer::delete() generated
	// ("cv::dnn::GroupNormLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_GroupNormLayer_delete(cv::dnn::GroupNormLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:836
	// ("cv::dnn::HardSigmoidLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_HardSigmoidLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::HardSigmoidLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::HardSigmoidLayer> ret = cv::dnn::HardSigmoidLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::HardSigmoidLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::HardSigmoidLayer::defaultNew() generated
	// ("cv::dnn::HardSigmoidLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::HardSigmoidLayer* cv_dnn_HardSigmoidLayer_defaultNew_const() {
			cv::dnn::HardSigmoidLayer* ret = new cv::dnn::HardSigmoidLayer();
			return ret;
	}

	// cv::dnn::HardSigmoidLayer::alpha() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:833
	// ("cv::dnn::HardSigmoidLayer::alpha", vec![(pred!(const, [], []), _)]),
	float cv_dnn_HardSigmoidLayer_propAlpha_const(const cv::dnn::HardSigmoidLayer* instance) {
			float ret = instance->alpha;
			return ret;
	}

	// cv::dnn::HardSigmoidLayer::setAlpha(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:833
	// ("cv::dnn::HardSigmoidLayer::setAlpha", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_HardSigmoidLayer_propAlpha_const_float(cv::dnn::HardSigmoidLayer* instance, const float val) {
			instance->alpha = val;
	}

	// cv::dnn::HardSigmoidLayer::beta() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:834
	// ("cv::dnn::HardSigmoidLayer::beta", vec![(pred!(const, [], []), _)]),
	float cv_dnn_HardSigmoidLayer_propBeta_const(const cv::dnn::HardSigmoidLayer* instance) {
			float ret = instance->beta;
			return ret;
	}

	// cv::dnn::HardSigmoidLayer::setBeta(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:834
	// ("cv::dnn::HardSigmoidLayer::setBeta", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_HardSigmoidLayer_propBeta_const_float(cv::dnn::HardSigmoidLayer* instance, const float val) {
			instance->beta = val;
	}

	// cv::dnn::HardSigmoidLayer::to_ActivationLayer() generated
	// ("cv::dnn::HardSigmoidLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_HardSigmoidLayer_to_ActivationLayer(cv::dnn::HardSigmoidLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::HardSigmoidLayer::to_Algorithm() generated
	// ("cv::dnn::HardSigmoidLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_HardSigmoidLayer_to_Algorithm(cv::dnn::HardSigmoidLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::HardSigmoidLayer::to_Layer() generated
	// ("cv::dnn::HardSigmoidLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_HardSigmoidLayer_to_Layer(cv::dnn::HardSigmoidLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::HardSigmoidLayer::delete() generated
	// ("cv::dnn::HardSigmoidLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_HardSigmoidLayer_delete(cv::dnn::HardSigmoidLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:789
	// ("cv::dnn::HardSwishLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_HardSwishLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::HardSwishLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::HardSwishLayer> ret = cv::dnn::HardSwishLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::HardSwishLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::HardSwishLayer::defaultNew() generated
	// ("cv::dnn::HardSwishLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::HardSwishLayer* cv_dnn_HardSwishLayer_defaultNew_const() {
			cv::dnn::HardSwishLayer* ret = new cv::dnn::HardSwishLayer();
			return ret;
	}

	// cv::dnn::HardSwishLayer::to_ActivationLayer() generated
	// ("cv::dnn::HardSwishLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_HardSwishLayer_to_ActivationLayer(cv::dnn::HardSwishLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::HardSwishLayer::to_Algorithm() generated
	// ("cv::dnn::HardSwishLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_HardSwishLayer_to_Algorithm(cv::dnn::HardSwishLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::HardSwishLayer::to_Layer() generated
	// ("cv::dnn::HardSwishLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_HardSwishLayer_to_Layer(cv::dnn::HardSwishLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::HardSwishLayer::delete() generated
	// ("cv::dnn::HardSwishLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_HardSwishLayer_delete(cv::dnn::HardSwishLayer* instance) {
			delete instance;
	}

	// Image2BlobParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1220
	// ("cv::dnn::Image2BlobParams::Image2BlobParams", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_Image2BlobParams_Image2BlobParams(Result<cv::dnn::Image2BlobParams>* ocvrs_return) {
		try {
			cv::dnn::Image2BlobParams ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Image2BlobParams(const Scalar &, const Size &, const Scalar &, bool, int, DataLayout, ImagePaddingMode, Scalar)(SimpleClass, SimpleClass, SimpleClass, Primitive, Primitive, Enum, Enum, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1221
	// ("cv::dnn::Image2BlobParams::Image2BlobParams", vec![(pred!(mut, ["scalefactor", "size", "mean", "swapRB", "ddepth", "datalayout", "mode", "borderValue"], ["const cv::Scalar*", "const cv::Size*", "const cv::Scalar*", "bool", "int", "cv::dnn::DataLayout", "cv::dnn::ImagePaddingMode", "cv::Scalar"]), _)]),
	void cv_dnn_Image2BlobParams_Image2BlobParams_const_ScalarR_const_SizeR_const_ScalarR_bool_int_DataLayout_ImagePaddingMode_Scalar(const cv::Scalar* scalefactor, const cv::Size* size, const cv::Scalar* mean, bool swapRB, int ddepth, cv::dnn::DataLayout datalayout, cv::dnn::ImagePaddingMode mode, cv::Scalar* borderValue, Result<cv::dnn::Image2BlobParams>* ocvrs_return) {
		try {
			cv::dnn::Image2BlobParams ret(*scalefactor, *size, *mean, swapRB, ddepth, datalayout, mode, *borderValue);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::Image2BlobParams::Image2BlobParams(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1221
	// ("cv::dnn::Image2BlobParams::Image2BlobParams", vec![(pred!(mut, ["scalefactor"], ["const cv::Scalar*"]), _)]),
	void cv_dnn_Image2BlobParams_Image2BlobParams_const_ScalarR(const cv::Scalar* scalefactor, Result<cv::dnn::Image2BlobParams>* ocvrs_return) {
		try {
			cv::dnn::Image2BlobParams ret(*scalefactor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// blobRectToImageRect(const Rect &, const Size &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1239
	// ("cv::dnn::Image2BlobParams::blobRectToImageRect", vec![(pred!(mut, ["rBlob", "size"], ["const cv::Rect*", "const cv::Size*"]), _)]),
	void cv_dnn_Image2BlobParams_blobRectToImageRect_const_RectR_const_SizeR(cv::dnn::Image2BlobParams* instance, const cv::Rect* rBlob, const cv::Size* size, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->blobRectToImageRect(*rBlob, *size);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// blobRectsToImageRects(const std::vector<Rect> &, std::vector<Rect> &, const Size &)(CppPassByVoidPtr, CppPassByVoidPtr, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1246
	// ("cv::dnn::Image2BlobParams::blobRectsToImageRects", vec![(pred!(mut, ["rBlob", "rImg", "size"], ["const std::vector<cv::Rect>*", "std::vector<cv::Rect>*", "const cv::Size*"]), _)]),
	void cv_dnn_Image2BlobParams_blobRectsToImageRects_const_vectorLRectGR_vectorLRectGR_const_SizeR(cv::dnn::Image2BlobParams* instance, const std::vector<cv::Rect>* rBlob, std::vector<cv::Rect>* rImg, const cv::Size* size, ResultVoid* ocvrs_return) {
		try {
			instance->blobRectsToImageRects(*rBlob, *rImg, *size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:424
	// ("cv::dnn::InnerProductLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_InnerProductLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::InnerProductLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::InnerProductLayer> ret = cv::dnn::InnerProductLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::InnerProductLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::InnerProductLayer::defaultNew() generated
	// ("cv::dnn::InnerProductLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::InnerProductLayer* cv_dnn_InnerProductLayer_defaultNew_const() {
			cv::dnn::InnerProductLayer* ret = new cv::dnn::InnerProductLayer();
			return ret;
	}

	// cv::dnn::InnerProductLayer::axis() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:423
	// ("cv::dnn::InnerProductLayer::axis", vec![(pred!(const, [], []), _)]),
	int cv_dnn_InnerProductLayer_propAxis_const(const cv::dnn::InnerProductLayer* instance) {
			int ret = instance->axis;
			return ret;
	}

	// cv::dnn::InnerProductLayer::setAxis(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:423
	// ("cv::dnn::InnerProductLayer::setAxis", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_InnerProductLayer_propAxis_const_int(cv::dnn::InnerProductLayer* instance, const int val) {
			instance->axis = val;
	}

	// cv::dnn::InnerProductLayer::to_Algorithm() generated
	// ("cv::dnn::InnerProductLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_InnerProductLayer_to_Algorithm(cv::dnn::InnerProductLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::InnerProductLayer::to_Layer() generated
	// ("cv::dnn::InnerProductLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_InnerProductLayer_to_Layer(cv::dnn::InnerProductLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::InnerProductLayer::delete() generated
	// ("cv::dnn::InnerProductLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_InnerProductLayer_delete(cv::dnn::InnerProductLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:436
	// ("cv::dnn::InnerProductLayerInt8::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_InnerProductLayerInt8_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::InnerProductLayerInt8>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::InnerProductLayerInt8> ret = cv::dnn::InnerProductLayerInt8::create(*params);
			Ok(new cv::Ptr<cv::dnn::InnerProductLayerInt8>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::InnerProductLayerInt8::defaultNew() generated
	// ("cv::dnn::InnerProductLayerInt8::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::InnerProductLayerInt8* cv_dnn_InnerProductLayerInt8_defaultNew_const() {
			cv::dnn::InnerProductLayerInt8* ret = new cv::dnn::InnerProductLayerInt8();
			return ret;
	}

	// cv::dnn::InnerProductLayerInt8::input_zp() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:430
	// ("cv::dnn::InnerProductLayerInt8::input_zp", vec![(pred!(const, [], []), _)]),
	int cv_dnn_InnerProductLayerInt8_propInput_zp_const(const cv::dnn::InnerProductLayerInt8* instance) {
			int ret = instance->input_zp;
			return ret;
	}

	// cv::dnn::InnerProductLayerInt8::setInput_zp(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:430
	// ("cv::dnn::InnerProductLayerInt8::setInput_zp", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_InnerProductLayerInt8_propInput_zp_const_int(cv::dnn::InnerProductLayerInt8* instance, const int val) {
			instance->input_zp = val;
	}

	// cv::dnn::InnerProductLayerInt8::output_zp() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:430
	// ("cv::dnn::InnerProductLayerInt8::output_zp", vec![(pred!(const, [], []), _)]),
	int cv_dnn_InnerProductLayerInt8_propOutput_zp_const(const cv::dnn::InnerProductLayerInt8* instance) {
			int ret = instance->output_zp;
			return ret;
	}

	// cv::dnn::InnerProductLayerInt8::setOutput_zp(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:430
	// ("cv::dnn::InnerProductLayerInt8::setOutput_zp", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_InnerProductLayerInt8_propOutput_zp_const_int(cv::dnn::InnerProductLayerInt8* instance, const int val) {
			instance->output_zp = val;
	}

	// cv::dnn::InnerProductLayerInt8::input_sc() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:431
	// ("cv::dnn::InnerProductLayerInt8::input_sc", vec![(pred!(const, [], []), _)]),
	float cv_dnn_InnerProductLayerInt8_propInput_sc_const(const cv::dnn::InnerProductLayerInt8* instance) {
			float ret = instance->input_sc;
			return ret;
	}

	// cv::dnn::InnerProductLayerInt8::setInput_sc(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:431
	// ("cv::dnn::InnerProductLayerInt8::setInput_sc", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_InnerProductLayerInt8_propInput_sc_const_float(cv::dnn::InnerProductLayerInt8* instance, const float val) {
			instance->input_sc = val;
	}

	// cv::dnn::InnerProductLayerInt8::output_sc() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:431
	// ("cv::dnn::InnerProductLayerInt8::output_sc", vec![(pred!(const, [], []), _)]),
	float cv_dnn_InnerProductLayerInt8_propOutput_sc_const(const cv::dnn::InnerProductLayerInt8* instance) {
			float ret = instance->output_sc;
			return ret;
	}

	// cv::dnn::InnerProductLayerInt8::setOutput_sc(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:431
	// ("cv::dnn::InnerProductLayerInt8::setOutput_sc", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_InnerProductLayerInt8_propOutput_sc_const_float(cv::dnn::InnerProductLayerInt8* instance, const float val) {
			instance->output_sc = val;
	}

	// cv::dnn::InnerProductLayerInt8::per_channel() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:435
	// ("cv::dnn::InnerProductLayerInt8::per_channel", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_InnerProductLayerInt8_propPer_channel_const(const cv::dnn::InnerProductLayerInt8* instance) {
			bool ret = instance->per_channel;
			return ret;
	}

	// cv::dnn::InnerProductLayerInt8::setPer_channel(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:435
	// ("cv::dnn::InnerProductLayerInt8::setPer_channel", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_dnn_InnerProductLayerInt8_propPer_channel_const_bool(cv::dnn::InnerProductLayerInt8* instance, const bool val) {
			instance->per_channel = val;
	}

	// cv::dnn::InnerProductLayerInt8::to_Algorithm() generated
	// ("cv::dnn::InnerProductLayerInt8::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_InnerProductLayerInt8_to_Algorithm(cv::dnn::InnerProductLayerInt8* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::InnerProductLayerInt8::to_InnerProductLayer() generated
	// ("cv::dnn::InnerProductLayerInt8::to_InnerProductLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::InnerProductLayer* cv_dnn_InnerProductLayerInt8_to_InnerProductLayer(cv::dnn::InnerProductLayerInt8* instance) {
			return dynamic_cast<cv::dnn::InnerProductLayer*>(instance);
	}

	// cv::dnn::InnerProductLayerInt8::to_Layer() generated
	// ("cv::dnn::InnerProductLayerInt8::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_InnerProductLayerInt8_to_Layer(cv::dnn::InnerProductLayerInt8* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::InnerProductLayerInt8::delete() generated
	// ("cv::dnn::InnerProductLayerInt8::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_InnerProductLayerInt8_delete(cv::dnn::InnerProductLayerInt8* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1178
	// ("cv::dnn::InstanceNormLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_InstanceNormLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::InstanceNormLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::InstanceNormLayer> ret = cv::dnn::InstanceNormLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::InstanceNormLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::InstanceNormLayer::defaultNew() generated
	// ("cv::dnn::InstanceNormLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::InstanceNormLayer* cv_dnn_InstanceNormLayer_defaultNew_const() {
			cv::dnn::InstanceNormLayer* ret = new cv::dnn::InstanceNormLayer();
			return ret;
	}

	// cv::dnn::InstanceNormLayer::epsilon() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1176
	// ("cv::dnn::InstanceNormLayer::epsilon", vec![(pred!(const, [], []), _)]),
	float cv_dnn_InstanceNormLayer_propEpsilon_const(const cv::dnn::InstanceNormLayer* instance) {
			float ret = instance->epsilon;
			return ret;
	}

	// cv::dnn::InstanceNormLayer::setEpsilon(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1176
	// ("cv::dnn::InstanceNormLayer::setEpsilon", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_InstanceNormLayer_propEpsilon_const_float(cv::dnn::InstanceNormLayer* instance, const float val) {
			instance->epsilon = val;
	}

	// cv::dnn::InstanceNormLayer::to_Algorithm() generated
	// ("cv::dnn::InstanceNormLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_InstanceNormLayer_to_Algorithm(cv::dnn::InstanceNormLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::InstanceNormLayer::to_Layer() generated
	// ("cv::dnn::InstanceNormLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_InstanceNormLayer_to_Layer(cv::dnn::InstanceNormLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::InstanceNormLayer::delete() generated
	// ("cv::dnn::InstanceNormLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_InstanceNormLayer_delete(cv::dnn::InstanceNormLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1101
	// ("cv::dnn::InterpLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_InterpLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::Layer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::InterpLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::Layer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::InterpLayer::defaultNew() generated
	// ("cv::dnn::InterpLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::InterpLayer* cv_dnn_InterpLayer_defaultNew_const() {
			cv::dnn::InterpLayer* ret = new cv::dnn::InterpLayer();
			return ret;
	}

	// cv::dnn::InterpLayer::to_Algorithm() generated
	// ("cv::dnn::InterpLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_InterpLayer_to_Algorithm(cv::dnn::InterpLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::InterpLayer::to_Layer() generated
	// ("cv::dnn::InterpLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_InterpLayer_to_Layer(cv::dnn::InterpLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::InterpLayer::delete() generated
	// ("cv::dnn::InterpLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_InterpLayer_delete(cv::dnn::InterpLayer* instance) {
			delete instance;
	}

	// KeypointsModel(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1574
	// ("cv::dnn::KeypointsModel::KeypointsModel", vec![(pred!(mut, ["model", "config"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_dnn_KeypointsModel_KeypointsModel_const_StringR_const_StringR(const char* model, const char* config, Result<cv::dnn::KeypointsModel*>* ocvrs_return) {
		try {
			cv::dnn::KeypointsModel* ret = new cv::dnn::KeypointsModel(std::string(model), std::string(config));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::KeypointsModel::KeypointsModel(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1574
	// ("cv::dnn::KeypointsModel::KeypointsModel", vec![(pred!(mut, ["model"], ["const cv::String*"]), _)]),
	void cv_dnn_KeypointsModel_KeypointsModel_const_StringR(const char* model, Result<cv::dnn::KeypointsModel*>* ocvrs_return) {
		try {
			cv::dnn::KeypointsModel* ret = new cv::dnn::KeypointsModel(std::string(model));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// KeypointsModel(const Net &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1580
	// ("cv::dnn::KeypointsModel::KeypointsModel", vec![(pred!(mut, ["network"], ["const cv::dnn::Net*"]), _)]),
	void cv_dnn_KeypointsModel_KeypointsModel_const_NetR(const cv::dnn::Net* network, Result<cv::dnn::KeypointsModel*>* ocvrs_return) {
		try {
			cv::dnn::KeypointsModel* ret = new cv::dnn::KeypointsModel(*network);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimate(InputArray, float)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1588
	// ("cv::dnn::KeypointsModel::estimate", vec![(pred!(mut, ["frame", "thresh"], ["const cv::_InputArray*", "float"]), _)]),
	void cv_dnn_KeypointsModel_estimate_const__InputArrayR_float(cv::dnn::KeypointsModel* instance, const cv::_InputArray* frame, float thresh, Result<std::vector<cv::Point2f>*>* ocvrs_return) {
		try {
			std::vector<cv::Point2f> ret = instance->estimate(*frame, thresh);
			Ok(new std::vector<cv::Point2f>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::KeypointsModel::estimate(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1588
	// ("cv::dnn::KeypointsModel::estimate", vec![(pred!(mut, ["frame"], ["const cv::_InputArray*"]), _)]),
	void cv_dnn_KeypointsModel_estimate_const__InputArrayR(cv::dnn::KeypointsModel* instance, const cv::_InputArray* frame, Result<std::vector<cv::Point2f>*>* ocvrs_return) {
		try {
			std::vector<cv::Point2f> ret = instance->estimate(*frame);
			Ok(new std::vector<cv::Point2f>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::KeypointsModel::implicitClone() generated
	// ("cv::dnn::KeypointsModel::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::dnn::KeypointsModel* cv_dnn_KeypointsModel_implicitClone_const(const cv::dnn::KeypointsModel* instance) {
			return new cv::dnn::KeypointsModel(*instance);
	}

	// cv::dnn::KeypointsModel::to_Model() generated
	// ("cv::dnn::KeypointsModel::to_Model", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Model* cv_dnn_KeypointsModel_to_Model(cv::dnn::KeypointsModel* instance) {
			return dynamic_cast<cv::dnn::Model*>(instance);
	}

	// cv::dnn::KeypointsModel::delete() generated
	// ("cv::dnn::KeypointsModel::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_KeypointsModel_delete(cv::dnn::KeypointsModel* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:325
	// ("cv::dnn::LRNLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_LRNLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::LRNLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::LRNLayer> ret = cv::dnn::LRNLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::LRNLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::LRNLayer::defaultNew() generated
	// ("cv::dnn::LRNLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::LRNLayer* cv_dnn_LRNLayer_defaultNew_const() {
			cv::dnn::LRNLayer* ret = new cv::dnn::LRNLayer();
			return ret;
	}

	// cv::dnn::LRNLayer::type() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:319
	// ("cv::dnn::LRNLayer::type", vec![(pred!(const, [], []), _)]),
	int cv_dnn_LRNLayer_propType_const(const cv::dnn::LRNLayer* instance) {
			int ret = instance->type;
			return ret;
	}

	// cv::dnn::LRNLayer::setType(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:319
	// ("cv::dnn::LRNLayer::setType", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_LRNLayer_propType_const_int(cv::dnn::LRNLayer* instance, const int val) {
			instance->type = val;
	}

	// cv::dnn::LRNLayer::size() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:321
	// ("cv::dnn::LRNLayer::size", vec![(pred!(const, [], []), _)]),
	int cv_dnn_LRNLayer_propSize_const(const cv::dnn::LRNLayer* instance) {
			int ret = instance->size;
			return ret;
	}

	// cv::dnn::LRNLayer::setSize(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:321
	// ("cv::dnn::LRNLayer::setSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_LRNLayer_propSize_const_int(cv::dnn::LRNLayer* instance, const int val) {
			instance->size = val;
	}

	// cv::dnn::LRNLayer::alpha() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:322
	// ("cv::dnn::LRNLayer::alpha", vec![(pred!(const, [], []), _)]),
	float cv_dnn_LRNLayer_propAlpha_const(const cv::dnn::LRNLayer* instance) {
			float ret = instance->alpha;
			return ret;
	}

	// cv::dnn::LRNLayer::setAlpha(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:322
	// ("cv::dnn::LRNLayer::setAlpha", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_LRNLayer_propAlpha_const_float(cv::dnn::LRNLayer* instance, const float val) {
			instance->alpha = val;
	}

	// cv::dnn::LRNLayer::beta() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:322
	// ("cv::dnn::LRNLayer::beta", vec![(pred!(const, [], []), _)]),
	float cv_dnn_LRNLayer_propBeta_const(const cv::dnn::LRNLayer* instance) {
			float ret = instance->beta;
			return ret;
	}

	// cv::dnn::LRNLayer::setBeta(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:322
	// ("cv::dnn::LRNLayer::setBeta", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_LRNLayer_propBeta_const_float(cv::dnn::LRNLayer* instance, const float val) {
			instance->beta = val;
	}

	// cv::dnn::LRNLayer::bias() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:322
	// ("cv::dnn::LRNLayer::bias", vec![(pred!(const, [], []), _)]),
	float cv_dnn_LRNLayer_propBias_const(const cv::dnn::LRNLayer* instance) {
			float ret = instance->bias;
			return ret;
	}

	// cv::dnn::LRNLayer::setBias(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:322
	// ("cv::dnn::LRNLayer::setBias", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_LRNLayer_propBias_const_float(cv::dnn::LRNLayer* instance, const float val) {
			instance->bias = val;
	}

	// cv::dnn::LRNLayer::normBySize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:323
	// ("cv::dnn::LRNLayer::normBySize", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_LRNLayer_propNormBySize_const(const cv::dnn::LRNLayer* instance) {
			bool ret = instance->normBySize;
			return ret;
	}

	// cv::dnn::LRNLayer::setNormBySize(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:323
	// ("cv::dnn::LRNLayer::setNormBySize", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_dnn_LRNLayer_propNormBySize_const_bool(cv::dnn::LRNLayer* instance, const bool val) {
			instance->normBySize = val;
	}

	// cv::dnn::LRNLayer::to_Algorithm() generated
	// ("cv::dnn::LRNLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_LRNLayer_to_Algorithm(cv::dnn::LRNLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::LRNLayer::to_Layer() generated
	// ("cv::dnn::LRNLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_LRNLayer_to_Layer(cv::dnn::LRNLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::LRNLayer::delete() generated
	// ("cv::dnn::LRNLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_LRNLayer_delete(cv::dnn::LRNLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:94
	// ("cv::dnn::LSTMLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_LSTMLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::LSTMLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::LSTMLayer> ret = cv::dnn::LSTMLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::LSTMLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWeights(const Mat &, const Mat &, const Mat &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:128
	// ("cv::dnn::LSTMLayer::setWeights", vec![(pred!(mut, ["Wh", "Wx", "b"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_dnn_LSTMLayer_setWeights_const_MatR_const_MatR_const_MatR(cv::dnn::LSTMLayer* instance, const cv::Mat* Wh, const cv::Mat* Wx, const cv::Mat* b, ResultVoid* ocvrs_return) {
		try {
			instance->setWeights(*Wh, *Wx, *b);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setOutShape(const MatShape &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:134
	// ("cv::dnn::LSTMLayer::setOutShape", vec![(pred!(mut, ["outTailShape"], ["const cv::dnn::MatShape*"]), _)]),
	void cv_dnn_LSTMLayer_setOutShape_const_MatShapeR(cv::dnn::LSTMLayer* instance, const cv::dnn::MatShape* outTailShape, ResultVoid* ocvrs_return) {
		try {
			instance->setOutShape(*outTailShape);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::LSTMLayer::setOutShape() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:134
	// ("cv::dnn::LSTMLayer::setOutShape", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_LSTMLayer_setOutShape(cv::dnn::LSTMLayer* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setOutShape();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseTimstampsDim(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:145
	// ("cv::dnn::LSTMLayer::setUseTimstampsDim", vec![(pred!(mut, ["use"], ["bool"]), _)]),
	void cv_dnn_LSTMLayer_setUseTimstampsDim_bool(cv::dnn::LSTMLayer* instance, bool use, ResultVoid* ocvrs_return) {
		try {
			instance->setUseTimstampsDim(use);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::LSTMLayer::setUseTimstampsDim() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:145
	// ("cv::dnn::LSTMLayer::setUseTimstampsDim", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_LSTMLayer_setUseTimstampsDim(cv::dnn::LSTMLayer* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setUseTimstampsDim();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setProduceCellOutput(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:151
	// ("cv::dnn::LSTMLayer::setProduceCellOutput", vec![(pred!(mut, ["produce"], ["bool"]), _)]),
	void cv_dnn_LSTMLayer_setProduceCellOutput_bool(cv::dnn::LSTMLayer* instance, bool produce, ResultVoid* ocvrs_return) {
		try {
			instance->setProduceCellOutput(produce);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::LSTMLayer::setProduceCellOutput() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:151
	// ("cv::dnn::LSTMLayer::setProduceCellOutput", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_LSTMLayer_setProduceCellOutput(cv::dnn::LSTMLayer* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setProduceCellOutput();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// inputNameToIndex(String)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:164
	// ("cv::dnn::LSTMLayer::inputNameToIndex", vec![(pred!(mut, ["inputName"], ["cv::String"]), _)]),
	void cv_dnn_LSTMLayer_inputNameToIndex_String(cv::dnn::LSTMLayer* instance, const char* inputName, Result<int>* ocvrs_return) {
		try {
			int ret = instance->inputNameToIndex(std::string(inputName));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// outputNameToIndex(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:165
	// ("cv::dnn::LSTMLayer::outputNameToIndex", vec![(pred!(mut, ["outputName"], ["const cv::String*"]), _)]),
	void cv_dnn_LSTMLayer_outputNameToIndex_const_StringR(cv::dnn::LSTMLayer* instance, const char* outputName, Result<int>* ocvrs_return) {
		try {
			int ret = instance->outputNameToIndex(std::string(outputName));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::LSTMLayer::to_Algorithm() generated
	// ("cv::dnn::LSTMLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_LSTMLayer_to_Algorithm(cv::dnn::LSTMLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::LSTMLayer::to_Layer() generated
	// ("cv::dnn::LSTMLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_LSTMLayer_to_Layer(cv::dnn::LSTMLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::LSTMLayer::delete() generated
	// ("cv::dnn::LSTMLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_LSTMLayer_delete(cv::dnn::LSTMLayer* instance) {
			delete instance;
	}

	// finalize(InputArrayOfArrays, OutputArrayOfArrays)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:245
	// ("cv::dnn::Layer::finalize", vec![(pred!(mut, ["inputs", "outputs"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_dnn_Layer_finalize_const__InputArrayR_const__OutputArrayR(cv::dnn::Layer* instance, const cv::_InputArray* inputs, const cv::_OutputArray* outputs, ResultVoid* ocvrs_return) {
		try {
			instance->finalize(*inputs, *outputs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// forward(std::vector<Mat *> &, std::vector<Mat> &, std::vector<Mat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:254
	// ("cv::dnn::Layer::forward", vec![(pred!(mut, ["input", "output", "internals"], ["std::vector<cv::Mat*>*", "std::vector<cv::Mat>*", "std::vector<cv::Mat>*"]), _)]),
	void cv_dnn_Layer_forward_vectorLMatXGR_vectorLMatGR_vectorLMatGR(cv::dnn::Layer* instance, std::vector<cv::Mat*>* input, std::vector<cv::Mat>* output, std::vector<cv::Mat>* internals, ResultVoid* ocvrs_return) {
		try {
			instance->forward(*input, *output, *internals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// forward(InputArrayOfArrays, OutputArrayOfArrays, OutputArrayOfArrays)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:261
	// ("cv::dnn::Layer::forward", vec![(pred!(mut, ["inputs", "outputs", "internals"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_dnn_Layer_forward_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::dnn::Layer* instance, const cv::_InputArray* inputs, const cv::_OutputArray* outputs, const cv::_OutputArray* internals, ResultVoid* ocvrs_return) {
		try {
			instance->forward(*inputs, *outputs, *internals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// tryQuantize(const std::vector<std::vector<float>> &, const std::vector<std::vector<int>> &, LayerParams &)(CppPassByVoidPtr, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:269
	// ("cv::dnn::Layer::tryQuantize", vec![(pred!(mut, ["scales", "zeropoints", "params"], ["const std::vector<std::vector<float>>*", "const std::vector<std::vector<int>>*", "cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_Layer_tryQuantize_const_vectorLvectorLfloatGGR_const_vectorLvectorLintGGR_LayerParamsR(cv::dnn::Layer* instance, const std::vector<std::vector<float>>* scales, const std::vector<std::vector<int>>* zeropoints, cv::dnn::LayerParams* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->tryQuantize(*scales, *zeropoints, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// forward_fallback(InputArrayOfArrays, OutputArrayOfArrays, OutputArrayOfArrays)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:277
	// ("cv::dnn::Layer::forward_fallback", vec![(pred!(mut, ["inputs", "outputs", "internals"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_dnn_Layer_forward_fallback_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::dnn::Layer* instance, const cv::_InputArray* inputs, const cv::_OutputArray* outputs, const cv::_OutputArray* internals, ResultVoid* ocvrs_return) {
		try {
			instance->forward_fallback(*inputs, *outputs, *internals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// finalize(const std::vector<Mat> &, std::vector<Mat> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:284
	// ("cv::dnn::Layer::finalize", vec![(pred!(mut, ["inputs", "outputs"], ["const std::vector<cv::Mat>*", "std::vector<cv::Mat>*"]), _)]),
	void cv_dnn_Layer_finalize_const_vectorLMatGR_vectorLMatGR(cv::dnn::Layer* instance, const std::vector<cv::Mat>* inputs, std::vector<cv::Mat>* outputs, ResultVoid* ocvrs_return) {
		try {
			instance->finalize(*inputs, *outputs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// finalize(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:290
	// ("cv::dnn::Layer::finalize", vec![(pred!(mut, ["inputs"], ["const std::vector<cv::Mat>*"]), _)]),
	void cv_dnn_Layer_finalize_const_vectorLMatGR(cv::dnn::Layer* instance, const std::vector<cv::Mat>* inputs, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			std::vector<cv::Mat> ret = instance->finalize(*inputs);
			Ok(new std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// run(const std::vector<Mat> &, std::vector<Mat> &, std::vector<Mat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:295
	// ("cv::dnn::Layer::run", vec![(pred!(mut, ["inputs", "outputs", "internals"], ["const std::vector<cv::Mat>*", "std::vector<cv::Mat>*", "std::vector<cv::Mat>*"]), _)]),
	void cv_dnn_Layer_run_const_vectorLMatGR_vectorLMatGR_vectorLMatGR(cv::dnn::Layer* instance, const std::vector<cv::Mat>* inputs, std::vector<cv::Mat>* outputs, std::vector<cv::Mat>* internals, ResultVoid* ocvrs_return) {
		try {
			instance->run(*inputs, *outputs, *internals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// inputNameToIndex(String)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:304
	// ("cv::dnn::Layer::inputNameToIndex", vec![(pred!(mut, ["inputName"], ["cv::String"]), _)]),
	void cv_dnn_Layer_inputNameToIndex_String(cv::dnn::Layer* instance, const char* inputName, Result<int>* ocvrs_return) {
		try {
			int ret = instance->inputNameToIndex(std::string(inputName));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// outputNameToIndex(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:308
	// ("cv::dnn::Layer::outputNameToIndex", vec![(pred!(mut, ["outputName"], ["const cv::String*"]), _)]),
	void cv_dnn_Layer_outputNameToIndex_const_StringR(cv::dnn::Layer* instance, const char* outputName, Result<int>* ocvrs_return) {
		try {
			int ret = instance->outputNameToIndex(std::string(outputName));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// supportBackend(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:315
	// ("cv::dnn::Layer::supportBackend", vec![(pred!(mut, ["backendId"], ["int"]), _)]),
	void cv_dnn_Layer_supportBackend_int(cv::dnn::Layer* instance, int backendId, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->supportBackend(backendId);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initHalide(const std::vector<Ptr<BackendWrapper>> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:327
	// ("cv::dnn::Layer::initHalide", vec![(pred!(mut, ["inputs"], ["const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>*"]), _)]),
	void cv_dnn_Layer_initHalide_const_vectorLPtrLBackendWrapperGGR(cv::dnn::Layer* instance, const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* inputs, Result<cv::Ptr<cv::dnn::BackendNode>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::BackendNode> ret = instance->initHalide(*inputs);
			Ok(new cv::Ptr<cv::dnn::BackendNode>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initNgraph(const std::vector<Ptr<BackendWrapper>> &, const std::vector<Ptr<BackendNode>> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:329
	// ("cv::dnn::Layer::initNgraph", vec![(pred!(mut, ["inputs", "nodes"], ["const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>*", "const std::vector<cv::Ptr<cv::dnn::BackendNode>>*"]), _)]),
	void cv_dnn_Layer_initNgraph_const_vectorLPtrLBackendWrapperGGR_const_vectorLPtrLBackendNodeGGR(cv::dnn::Layer* instance, const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* inputs, const std::vector<cv::Ptr<cv::dnn::BackendNode>>* nodes, Result<cv::Ptr<cv::dnn::BackendNode>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::BackendNode> ret = instance->initNgraph(*inputs, *nodes);
			Ok(new cv::Ptr<cv::dnn::BackendNode>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initVkCom(const std::vector<Ptr<BackendWrapper>> &, std::vector<Ptr<BackendWrapper>> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:331
	// ("cv::dnn::Layer::initVkCom", vec![(pred!(mut, ["inputs", "outputs"], ["const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>*", "std::vector<cv::Ptr<cv::dnn::BackendWrapper>>*"]), _)]),
	void cv_dnn_Layer_initVkCom_const_vectorLPtrLBackendWrapperGGR_vectorLPtrLBackendWrapperGGR(cv::dnn::Layer* instance, const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* inputs, std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* outputs, Result<cv::Ptr<cv::dnn::BackendNode>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::BackendNode> ret = instance->initVkCom(*inputs, *outputs);
			Ok(new cv::Ptr<cv::dnn::BackendNode>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initWebnn(const std::vector<Ptr<BackendWrapper>> &, const std::vector<Ptr<BackendNode>> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:333
	// ("cv::dnn::Layer::initWebnn", vec![(pred!(mut, ["inputs", "nodes"], ["const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>*", "const std::vector<cv::Ptr<cv::dnn::BackendNode>>*"]), _)]),
	void cv_dnn_Layer_initWebnn_const_vectorLPtrLBackendWrapperGGR_const_vectorLPtrLBackendNodeGGR(cv::dnn::Layer* instance, const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* inputs, const std::vector<cv::Ptr<cv::dnn::BackendNode>>* nodes, Result<cv::Ptr<cv::dnn::BackendNode>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::BackendNode> ret = instance->initWebnn(*inputs, *nodes);
			Ok(new cv::Ptr<cv::dnn::BackendNode>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initCUDA(void *, const std::vector<Ptr<BackendWrapper>> &, const std::vector<Ptr<BackendWrapper>> &)(Indirect, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:342
	// ("cv::dnn::Layer::initCUDA", vec![(pred!(mut, ["context", "inputs", "outputs"], ["void*", "const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>*", "const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>*"]), _)]),
	void cv_dnn_Layer_initCUDA_voidX_const_vectorLPtrLBackendWrapperGGR_const_vectorLPtrLBackendWrapperGGR(cv::dnn::Layer* instance, void* context, const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* inputs, const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* outputs, Result<cv::Ptr<cv::dnn::BackendNode>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::BackendNode> ret = instance->initCUDA(context, *inputs, *outputs);
			Ok(new cv::Ptr<cv::dnn::BackendNode>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initTimVX(void *, const std::vector<Ptr<BackendWrapper>> &, const std::vector<Ptr<BackendWrapper>> &, bool)(Indirect, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:356
	// ("cv::dnn::Layer::initTimVX", vec![(pred!(mut, ["timVxInfo", "inputsWrapper", "outputsWrapper", "isLast"], ["void*", "const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>*", "const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>*", "bool"]), _)]),
	void cv_dnn_Layer_initTimVX_voidX_const_vectorLPtrLBackendWrapperGGR_const_vectorLPtrLBackendWrapperGGR_bool(cv::dnn::Layer* instance, void* timVxInfo, const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* inputsWrapper, const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* outputsWrapper, bool isLast, Result<cv::Ptr<cv::dnn::BackendNode>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::BackendNode> ret = instance->initTimVX(timVxInfo, *inputsWrapper, *outputsWrapper, isLast);
			Ok(new cv::Ptr<cv::dnn::BackendNode>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initCann(const std::vector<Ptr<BackendWrapper>> &, const std::vector<Ptr<BackendWrapper>> &, const std::vector<Ptr<BackendNode>> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:368
	// ("cv::dnn::Layer::initCann", vec![(pred!(mut, ["inputs", "outputs", "nodes"], ["const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>*", "const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>*", "const std::vector<cv::Ptr<cv::dnn::BackendNode>>*"]), _)]),
	void cv_dnn_Layer_initCann_const_vectorLPtrLBackendWrapperGGR_const_vectorLPtrLBackendWrapperGGR_const_vectorLPtrLBackendNodeGGR(cv::dnn::Layer* instance, const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* inputs, const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* outputs, const std::vector<cv::Ptr<cv::dnn::BackendNode>>* nodes, Result<cv::Ptr<cv::dnn::BackendNode>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::BackendNode> ret = instance->initCann(*inputs, *outputs, *nodes);
			Ok(new cv::Ptr<cv::dnn::BackendNode>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// applyHalideScheduler(Ptr<BackendNode> &, const std::vector<Mat *> &, const std::vector<Mat> &, int)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:383
	// ("cv::dnn::Layer::applyHalideScheduler", vec![(pred!(const, ["node", "inputs", "outputs", "targetId"], ["cv::Ptr<cv::dnn::BackendNode>*", "const std::vector<cv::Mat*>*", "const std::vector<cv::Mat>*", "int"]), _)]),
	void cv_dnn_Layer_applyHalideScheduler_const_PtrLBackendNodeGR_const_vectorLMatXGR_const_vectorLMatGR_int(const cv::dnn::Layer* instance, cv::Ptr<cv::dnn::BackendNode>* node, const std::vector<cv::Mat*>* inputs, const std::vector<cv::Mat>* outputs, int targetId, ResultVoid* ocvrs_return) {
		try {
			instance->applyHalideScheduler(*node, *inputs, *outputs, targetId);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// tryAttach(const Ptr<BackendNode> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:397
	// ("cv::dnn::Layer::tryAttach", vec![(pred!(mut, ["node"], ["const cv::Ptr<cv::dnn::BackendNode>*"]), _)]),
	void cv_dnn_Layer_tryAttach_const_PtrLBackendNodeGR(cv::dnn::Layer* instance, const cv::Ptr<cv::dnn::BackendNode>* node, Result<cv::Ptr<cv::dnn::BackendNode>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::BackendNode> ret = instance->tryAttach(*node);
			Ok(new cv::Ptr<cv::dnn::BackendNode>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setActivation(const Ptr<ActivationLayer> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:405
	// ("cv::dnn::Layer::setActivation", vec![(pred!(mut, ["layer"], ["const cv::Ptr<cv::dnn::ActivationLayer>*"]), _)]),
	void cv_dnn_Layer_setActivation_const_PtrLActivationLayerGR(cv::dnn::Layer* instance, const cv::Ptr<cv::dnn::ActivationLayer>* layer, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->setActivation(*layer);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// tryFuse(Ptr<Layer> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:412
	// ("cv::dnn::Layer::tryFuse", vec![(pred!(mut, ["top"], ["cv::Ptr<cv::dnn::Layer>*"]), _)]),
	void cv_dnn_Layer_tryFuse_PtrLLayerGR(cv::dnn::Layer* instance, cv::Ptr<cv::dnn::Layer>* top, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->tryFuse(*top);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScaleShift(Mat &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:427
	// ("cv::dnn::Layer::getScaleShift", vec![(pred!(const, ["scale", "shift"], ["cv::Mat*", "cv::Mat*"]), _)]),
	void cv_dnn_Layer_getScaleShift_const_MatR_MatR(const cv::dnn::Layer* instance, cv::Mat* scale, cv::Mat* shift, ResultVoid* ocvrs_return) {
		try {
			instance->getScaleShift(*scale, *shift);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScaleZeropoint(float &, int &)(Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:436
	// ("cv::dnn::Layer::getScaleZeropoint", vec![(pred!(const, ["scale", "zeropoint"], ["float*", "int*"]), _)]),
	void cv_dnn_Layer_getScaleZeropoint_const_floatR_intR(const cv::dnn::Layer* instance, float* scale, int* zeropoint, ResultVoid* ocvrs_return) {
		try {
			instance->getScaleZeropoint(*scale, *zeropoint);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// unsetAttached()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:442
	// ("cv::dnn::Layer::unsetAttached", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_Layer_unsetAttached(cv::dnn::Layer* instance, ResultVoid* ocvrs_return) {
		try {
			instance->unsetAttached();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMemoryShapes(const std::vector<MatShape> &, const int, std::vector<MatShape> &, std::vector<MatShape> &)(CppPassByVoidPtr, Primitive, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:444
	// ("cv::dnn::Layer::getMemoryShapes", vec![(pred!(const, ["inputs", "requiredOutputs", "outputs", "internals"], ["const std::vector<cv::dnn::MatShape>*", "const int", "std::vector<cv::dnn::MatShape>*", "std::vector<cv::dnn::MatShape>*"]), _)]),
	void cv_dnn_Layer_getMemoryShapes_const_const_vectorLMatShapeGR_const_int_vectorLMatShapeGR_vectorLMatShapeGR(const cv::dnn::Layer* instance, const std::vector<cv::dnn::MatShape>* inputs, const int requiredOutputs, std::vector<cv::dnn::MatShape>* outputs, std::vector<cv::dnn::MatShape>* internals, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getMemoryShapes(*inputs, requiredOutputs, *outputs, *internals);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFLOPS(const std::vector<MatShape> &, const std::vector<MatShape> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:449
	// ("cv::dnn::Layer::getFLOPS", vec![(pred!(const, ["inputs", "outputs"], ["const std::vector<cv::dnn::MatShape>*", "const std::vector<cv::dnn::MatShape>*"]), _)]),
	void cv_dnn_Layer_getFLOPS_const_const_vectorLMatShapeGR_const_vectorLMatShapeGR(const cv::dnn::Layer* instance, const std::vector<cv::dnn::MatShape>* inputs, const std::vector<cv::dnn::MatShape>* outputs, Result<int64_t>* ocvrs_return) {
		try {
			int64_t ret = instance->getFLOPS(*inputs, *outputs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// updateMemoryShapes(const std::vector<MatShape> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:452
	// ("cv::dnn::Layer::updateMemoryShapes", vec![(pred!(mut, ["inputs"], ["const std::vector<cv::dnn::MatShape>*"]), _)]),
	void cv_dnn_Layer_updateMemoryShapes_const_vectorLMatShapeGR(cv::dnn::Layer* instance, const std::vector<cv::dnn::MatShape>* inputs, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->updateMemoryShapes(*inputs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Layer()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:458
	// ("cv::dnn::Layer::Layer", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_Layer_Layer(Result<cv::dnn::Layer*>* ocvrs_return) {
		try {
			cv::dnn::Layer* ret = new cv::dnn::Layer();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Layer(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:459
	// ("cv::dnn::Layer::Layer", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_Layer_Layer_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::dnn::Layer*>* ocvrs_return) {
		try {
			cv::dnn::Layer* ret = new cv::dnn::Layer(*params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setParamsFrom(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:460
	// ("cv::dnn::Layer::setParamsFrom", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_Layer_setParamsFrom_const_LayerParamsR(cv::dnn::Layer* instance, const cv::dnn::LayerParams* params, ResultVoid* ocvrs_return) {
		try {
			instance->setParamsFrom(*params);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::Layer::blobs() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:225
	// ("cv::dnn::Layer::blobs", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Mat>* cv_dnn_Layer_propBlobs_const(const cv::dnn::Layer* instance) {
			std::vector<cv::Mat> ret = instance->blobs;
			return new std::vector<cv::Mat>(ret);
	}

	// cv::dnn::Layer::setBlobs(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:225
	// ("cv::dnn::Layer::setBlobs", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	void cv_dnn_Layer_propBlobs_const_vectorLMatG(cv::dnn::Layer* instance, const std::vector<cv::Mat>* val) {
			instance->blobs = *val;
	}

	// cv::dnn::Layer::name() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:454
	// ("cv::dnn::Layer::name", vec![(pred!(const, [], []), _)]),
	void* cv_dnn_Layer_propName_const(const cv::dnn::Layer* instance) {
			cv::String ret = instance->name;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::dnn::Layer::setName(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:454
	// ("cv::dnn::Layer::setName", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	void cv_dnn_Layer_propName_const_String(cv::dnn::Layer* instance, const char* val) {
			instance->name = std::string(val);
	}

	// cv::dnn::Layer::type() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:455
	// ("cv::dnn::Layer::type", vec![(pred!(const, [], []), _)]),
	void* cv_dnn_Layer_propType_const(const cv::dnn::Layer* instance) {
			cv::String ret = instance->type;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::dnn::Layer::setType(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:455
	// ("cv::dnn::Layer::setType", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	void cv_dnn_Layer_propType_const_String(cv::dnn::Layer* instance, const char* val) {
			instance->type = std::string(val);
	}

	// cv::dnn::Layer::preferableTarget() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:456
	// ("cv::dnn::Layer::preferableTarget", vec![(pred!(const, [], []), _)]),
	int cv_dnn_Layer_propPreferableTarget_const(const cv::dnn::Layer* instance) {
			int ret = instance->preferableTarget;
			return ret;
	}

	// cv::dnn::Layer::setPreferableTarget(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:456
	// ("cv::dnn::Layer::setPreferableTarget", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_Layer_propPreferableTarget_const_int(cv::dnn::Layer* instance, const int val) {
			instance->preferableTarget = val;
	}

	// cv::dnn::Layer::to_AbsLayer() generated
	// ("cv::dnn::Layer::to_AbsLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::AbsLayer* cv_dnn_Layer_to_AbsLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::AbsLayer*>(instance);
	}

	// cv::dnn::Layer::to_AccumLayer() generated
	// ("cv::dnn::Layer::to_AccumLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::AccumLayer* cv_dnn_Layer_to_AccumLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::AccumLayer*>(instance);
	}

	// cv::dnn::Layer::to_AcosLayer() generated
	// ("cv::dnn::Layer::to_AcosLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::AcosLayer* cv_dnn_Layer_to_AcosLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::AcosLayer*>(instance);
	}

	// cv::dnn::Layer::to_AcoshLayer() generated
	// ("cv::dnn::Layer::to_AcoshLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::AcoshLayer* cv_dnn_Layer_to_AcoshLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::AcoshLayer*>(instance);
	}

	// cv::dnn::Layer::to_ActivationLayer() generated
	// ("cv::dnn::Layer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_Layer_to_ActivationLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::Layer::to_ActivationLayerInt8() generated
	// ("cv::dnn::Layer::to_ActivationLayerInt8", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayerInt8* cv_dnn_Layer_to_ActivationLayerInt8(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayerInt8*>(instance);
	}

	// cv::dnn::Layer::to_ArgLayer() generated
	// ("cv::dnn::Layer::to_ArgLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ArgLayer* cv_dnn_Layer_to_ArgLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ArgLayer*>(instance);
	}

	// cv::dnn::Layer::to_AsinLayer() generated
	// ("cv::dnn::Layer::to_AsinLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::AsinLayer* cv_dnn_Layer_to_AsinLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::AsinLayer*>(instance);
	}

	// cv::dnn::Layer::to_AsinhLayer() generated
	// ("cv::dnn::Layer::to_AsinhLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::AsinhLayer* cv_dnn_Layer_to_AsinhLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::AsinhLayer*>(instance);
	}

	// cv::dnn::Layer::to_AtanLayer() generated
	// ("cv::dnn::Layer::to_AtanLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::AtanLayer* cv_dnn_Layer_to_AtanLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::AtanLayer*>(instance);
	}

	// cv::dnn::Layer::to_AtanhLayer() generated
	// ("cv::dnn::Layer::to_AtanhLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::AtanhLayer* cv_dnn_Layer_to_AtanhLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::AtanhLayer*>(instance);
	}

	// cv::dnn::Layer::to_AttentionLayer() generated
	// ("cv::dnn::Layer::to_AttentionLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::AttentionLayer* cv_dnn_Layer_to_AttentionLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::AttentionLayer*>(instance);
	}

	// cv::dnn::Layer::to_BNLLLayer() generated
	// ("cv::dnn::Layer::to_BNLLLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::BNLLLayer* cv_dnn_Layer_to_BNLLLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::BNLLLayer*>(instance);
	}

	// cv::dnn::Layer::to_BaseConvolutionLayer() generated
	// ("cv::dnn::Layer::to_BaseConvolutionLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::BaseConvolutionLayer* cv_dnn_Layer_to_BaseConvolutionLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::BaseConvolutionLayer*>(instance);
	}

	// cv::dnn::Layer::to_BatchNormLayer() generated
	// ("cv::dnn::Layer::to_BatchNormLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::BatchNormLayer* cv_dnn_Layer_to_BatchNormLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::BatchNormLayer*>(instance);
	}

	// cv::dnn::Layer::to_BatchNormLayerInt8() generated
	// ("cv::dnn::Layer::to_BatchNormLayerInt8", vec![(pred!(mut, [], []), _)]),
	cv::dnn::BatchNormLayerInt8* cv_dnn_Layer_to_BatchNormLayerInt8(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::BatchNormLayerInt8*>(instance);
	}

	// cv::dnn::Layer::to_BlankLayer() generated
	// ("cv::dnn::Layer::to_BlankLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::BlankLayer* cv_dnn_Layer_to_BlankLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::BlankLayer*>(instance);
	}

	// cv::dnn::Layer::to_CeilLayer() generated
	// ("cv::dnn::Layer::to_CeilLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::CeilLayer* cv_dnn_Layer_to_CeilLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::CeilLayer*>(instance);
	}

	// cv::dnn::Layer::to_CeluLayer() generated
	// ("cv::dnn::Layer::to_CeluLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::CeluLayer* cv_dnn_Layer_to_CeluLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::CeluLayer*>(instance);
	}

	// cv::dnn::Layer::to_ChannelsPReLULayer() generated
	// ("cv::dnn::Layer::to_ChannelsPReLULayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ChannelsPReLULayer* cv_dnn_Layer_to_ChannelsPReLULayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ChannelsPReLULayer*>(instance);
	}

	// cv::dnn::Layer::to_CompareLayer() generated
	// ("cv::dnn::Layer::to_CompareLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::CompareLayer* cv_dnn_Layer_to_CompareLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::CompareLayer*>(instance);
	}

	// cv::dnn::Layer::to_ConcatLayer() generated
	// ("cv::dnn::Layer::to_ConcatLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ConcatLayer* cv_dnn_Layer_to_ConcatLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ConcatLayer*>(instance);
	}

	// cv::dnn::Layer::to_ConstLayer() generated
	// ("cv::dnn::Layer::to_ConstLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ConstLayer* cv_dnn_Layer_to_ConstLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ConstLayer*>(instance);
	}

	// cv::dnn::Layer::to_ConvolutionLayer() generated
	// ("cv::dnn::Layer::to_ConvolutionLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ConvolutionLayer* cv_dnn_Layer_to_ConvolutionLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ConvolutionLayer*>(instance);
	}

	// cv::dnn::Layer::to_ConvolutionLayerInt8() generated
	// ("cv::dnn::Layer::to_ConvolutionLayerInt8", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ConvolutionLayerInt8* cv_dnn_Layer_to_ConvolutionLayerInt8(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ConvolutionLayerInt8*>(instance);
	}

	// cv::dnn::Layer::to_CorrelationLayer() generated
	// ("cv::dnn::Layer::to_CorrelationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::CorrelationLayer* cv_dnn_Layer_to_CorrelationLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::CorrelationLayer*>(instance);
	}

	// cv::dnn::Layer::to_CosLayer() generated
	// ("cv::dnn::Layer::to_CosLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::CosLayer* cv_dnn_Layer_to_CosLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::CosLayer*>(instance);
	}

	// cv::dnn::Layer::to_CoshLayer() generated
	// ("cv::dnn::Layer::to_CoshLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::CoshLayer* cv_dnn_Layer_to_CoshLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::CoshLayer*>(instance);
	}

	// cv::dnn::Layer::to_CropAndResizeLayer() generated
	// ("cv::dnn::Layer::to_CropAndResizeLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::CropAndResizeLayer* cv_dnn_Layer_to_CropAndResizeLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::CropAndResizeLayer*>(instance);
	}

	// cv::dnn::Layer::to_CropLayer() generated
	// ("cv::dnn::Layer::to_CropLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::CropLayer* cv_dnn_Layer_to_CropLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::CropLayer*>(instance);
	}

	// cv::dnn::Layer::to_CumSumLayer() generated
	// ("cv::dnn::Layer::to_CumSumLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::CumSumLayer* cv_dnn_Layer_to_CumSumLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::CumSumLayer*>(instance);
	}

	// cv::dnn::Layer::to_DataAugmentationLayer() generated
	// ("cv::dnn::Layer::to_DataAugmentationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::DataAugmentationLayer* cv_dnn_Layer_to_DataAugmentationLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::DataAugmentationLayer*>(instance);
	}

	// cv::dnn::Layer::to_DeconvolutionLayer() generated
	// ("cv::dnn::Layer::to_DeconvolutionLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::DeconvolutionLayer* cv_dnn_Layer_to_DeconvolutionLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::DeconvolutionLayer*>(instance);
	}

	// cv::dnn::Layer::to_DepthToSpaceLayer() generated
	// ("cv::dnn::Layer::to_DepthToSpaceLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::DepthToSpaceLayer* cv_dnn_Layer_to_DepthToSpaceLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::DepthToSpaceLayer*>(instance);
	}

	// cv::dnn::Layer::to_DequantizeLayer() generated
	// ("cv::dnn::Layer::to_DequantizeLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::DequantizeLayer* cv_dnn_Layer_to_DequantizeLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::DequantizeLayer*>(instance);
	}

	// cv::dnn::Layer::to_DetectionOutputLayer() generated
	// ("cv::dnn::Layer::to_DetectionOutputLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::DetectionOutputLayer* cv_dnn_Layer_to_DetectionOutputLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::DetectionOutputLayer*>(instance);
	}

	// cv::dnn::Layer::to_ELULayer() generated
	// ("cv::dnn::Layer::to_ELULayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ELULayer* cv_dnn_Layer_to_ELULayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ELULayer*>(instance);
	}

	// cv::dnn::Layer::to_EinsumLayer() generated
	// ("cv::dnn::Layer::to_EinsumLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::EinsumLayer* cv_dnn_Layer_to_EinsumLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::EinsumLayer*>(instance);
	}

	// cv::dnn::Layer::to_EltwiseLayer() generated
	// ("cv::dnn::Layer::to_EltwiseLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::EltwiseLayer* cv_dnn_Layer_to_EltwiseLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::EltwiseLayer*>(instance);
	}

	// cv::dnn::Layer::to_EltwiseLayerInt8() generated
	// ("cv::dnn::Layer::to_EltwiseLayerInt8", vec![(pred!(mut, [], []), _)]),
	cv::dnn::EltwiseLayerInt8* cv_dnn_Layer_to_EltwiseLayerInt8(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::EltwiseLayerInt8*>(instance);
	}

	// cv::dnn::Layer::to_ErfLayer() generated
	// ("cv::dnn::Layer::to_ErfLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ErfLayer* cv_dnn_Layer_to_ErfLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ErfLayer*>(instance);
	}

	// cv::dnn::Layer::to_ExpLayer() generated
	// ("cv::dnn::Layer::to_ExpLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ExpLayer* cv_dnn_Layer_to_ExpLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ExpLayer*>(instance);
	}

	// cv::dnn::Layer::to_ExpandLayer() generated
	// ("cv::dnn::Layer::to_ExpandLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ExpandLayer* cv_dnn_Layer_to_ExpandLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ExpandLayer*>(instance);
	}

	// cv::dnn::Layer::to_FlattenLayer() generated
	// ("cv::dnn::Layer::to_FlattenLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::FlattenLayer* cv_dnn_Layer_to_FlattenLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::FlattenLayer*>(instance);
	}

	// cv::dnn::Layer::to_FloorLayer() generated
	// ("cv::dnn::Layer::to_FloorLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::FloorLayer* cv_dnn_Layer_to_FloorLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::FloorLayer*>(instance);
	}

	// cv::dnn::Layer::to_FlowWarpLayer() generated
	// ("cv::dnn::Layer::to_FlowWarpLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::FlowWarpLayer* cv_dnn_Layer_to_FlowWarpLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::FlowWarpLayer*>(instance);
	}

	// cv::dnn::Layer::to_GRULayer() generated
	// ("cv::dnn::Layer::to_GRULayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::GRULayer* cv_dnn_Layer_to_GRULayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::GRULayer*>(instance);
	}

	// cv::dnn::Layer::to_GatherElementsLayer() generated
	// ("cv::dnn::Layer::to_GatherElementsLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::GatherElementsLayer* cv_dnn_Layer_to_GatherElementsLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::GatherElementsLayer*>(instance);
	}

	// cv::dnn::Layer::to_GatherLayer() generated
	// ("cv::dnn::Layer::to_GatherLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::GatherLayer* cv_dnn_Layer_to_GatherLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::GatherLayer*>(instance);
	}

	// cv::dnn::Layer::to_GeluApproximationLayer() generated
	// ("cv::dnn::Layer::to_GeluApproximationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::GeluApproximationLayer* cv_dnn_Layer_to_GeluApproximationLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::GeluApproximationLayer*>(instance);
	}

	// cv::dnn::Layer::to_GeluLayer() generated
	// ("cv::dnn::Layer::to_GeluLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::GeluLayer* cv_dnn_Layer_to_GeluLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::GeluLayer*>(instance);
	}

	// cv::dnn::Layer::to_GemmLayer() generated
	// ("cv::dnn::Layer::to_GemmLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::GemmLayer* cv_dnn_Layer_to_GemmLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::GemmLayer*>(instance);
	}

	// cv::dnn::Layer::to_GroupNormLayer() generated
	// ("cv::dnn::Layer::to_GroupNormLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::GroupNormLayer* cv_dnn_Layer_to_GroupNormLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::GroupNormLayer*>(instance);
	}

	// cv::dnn::Layer::to_HardSigmoidLayer() generated
	// ("cv::dnn::Layer::to_HardSigmoidLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::HardSigmoidLayer* cv_dnn_Layer_to_HardSigmoidLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::HardSigmoidLayer*>(instance);
	}

	// cv::dnn::Layer::to_HardSwishLayer() generated
	// ("cv::dnn::Layer::to_HardSwishLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::HardSwishLayer* cv_dnn_Layer_to_HardSwishLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::HardSwishLayer*>(instance);
	}

	// cv::dnn::Layer::to_InnerProductLayer() generated
	// ("cv::dnn::Layer::to_InnerProductLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::InnerProductLayer* cv_dnn_Layer_to_InnerProductLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::InnerProductLayer*>(instance);
	}

	// cv::dnn::Layer::to_InnerProductLayerInt8() generated
	// ("cv::dnn::Layer::to_InnerProductLayerInt8", vec![(pred!(mut, [], []), _)]),
	cv::dnn::InnerProductLayerInt8* cv_dnn_Layer_to_InnerProductLayerInt8(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::InnerProductLayerInt8*>(instance);
	}

	// cv::dnn::Layer::to_InstanceNormLayer() generated
	// ("cv::dnn::Layer::to_InstanceNormLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::InstanceNormLayer* cv_dnn_Layer_to_InstanceNormLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::InstanceNormLayer*>(instance);
	}

	// cv::dnn::Layer::to_InterpLayer() generated
	// ("cv::dnn::Layer::to_InterpLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::InterpLayer* cv_dnn_Layer_to_InterpLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::InterpLayer*>(instance);
	}

	// cv::dnn::Layer::to_LRNLayer() generated
	// ("cv::dnn::Layer::to_LRNLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::LRNLayer* cv_dnn_Layer_to_LRNLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::LRNLayer*>(instance);
	}

	// cv::dnn::Layer::to_LSTMLayer() generated
	// ("cv::dnn::Layer::to_LSTMLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::LSTMLayer* cv_dnn_Layer_to_LSTMLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::LSTMLayer*>(instance);
	}

	// cv::dnn::Layer::to_LayerNormLayer() generated
	// ("cv::dnn::Layer::to_LayerNormLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::LayerNormLayer* cv_dnn_Layer_to_LayerNormLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::LayerNormLayer*>(instance);
	}

	// cv::dnn::Layer::to_LogLayer() generated
	// ("cv::dnn::Layer::to_LogLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::LogLayer* cv_dnn_Layer_to_LogLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::LogLayer*>(instance);
	}

	// cv::dnn::Layer::to_MVNLayer() generated
	// ("cv::dnn::Layer::to_MVNLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::MVNLayer* cv_dnn_Layer_to_MVNLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::MVNLayer*>(instance);
	}

	// cv::dnn::Layer::to_MatMulLayer() generated
	// ("cv::dnn::Layer::to_MatMulLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::MatMulLayer* cv_dnn_Layer_to_MatMulLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::MatMulLayer*>(instance);
	}

	// cv::dnn::Layer::to_MaxUnpoolLayer() generated
	// ("cv::dnn::Layer::to_MaxUnpoolLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::MaxUnpoolLayer* cv_dnn_Layer_to_MaxUnpoolLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::MaxUnpoolLayer*>(instance);
	}

	// cv::dnn::Layer::to_MishLayer() generated
	// ("cv::dnn::Layer::to_MishLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::MishLayer* cv_dnn_Layer_to_MishLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::MishLayer*>(instance);
	}

	// cv::dnn::Layer::to_NaryEltwiseLayer() generated
	// ("cv::dnn::Layer::to_NaryEltwiseLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::NaryEltwiseLayer* cv_dnn_Layer_to_NaryEltwiseLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::NaryEltwiseLayer*>(instance);
	}

	// cv::dnn::Layer::to_NormalizeBBoxLayer() generated
	// ("cv::dnn::Layer::to_NormalizeBBoxLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::NormalizeBBoxLayer* cv_dnn_Layer_to_NormalizeBBoxLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::NormalizeBBoxLayer*>(instance);
	}

	// cv::dnn::Layer::to_NotLayer() generated
	// ("cv::dnn::Layer::to_NotLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::NotLayer* cv_dnn_Layer_to_NotLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::NotLayer*>(instance);
	}

	// cv::dnn::Layer::to_PaddingLayer() generated
	// ("cv::dnn::Layer::to_PaddingLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::PaddingLayer* cv_dnn_Layer_to_PaddingLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::PaddingLayer*>(instance);
	}

	// cv::dnn::Layer::to_PermuteLayer() generated
	// ("cv::dnn::Layer::to_PermuteLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::PermuteLayer* cv_dnn_Layer_to_PermuteLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::PermuteLayer*>(instance);
	}

	// cv::dnn::Layer::to_PoolingLayer() generated
	// ("cv::dnn::Layer::to_PoolingLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::PoolingLayer* cv_dnn_Layer_to_PoolingLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::PoolingLayer*>(instance);
	}

	// cv::dnn::Layer::to_PoolingLayerInt8() generated
	// ("cv::dnn::Layer::to_PoolingLayerInt8", vec![(pred!(mut, [], []), _)]),
	cv::dnn::PoolingLayerInt8* cv_dnn_Layer_to_PoolingLayerInt8(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::PoolingLayerInt8*>(instance);
	}

	// cv::dnn::Layer::to_PowerLayer() generated
	// ("cv::dnn::Layer::to_PowerLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::PowerLayer* cv_dnn_Layer_to_PowerLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::PowerLayer*>(instance);
	}

	// cv::dnn::Layer::to_PriorBoxLayer() generated
	// ("cv::dnn::Layer::to_PriorBoxLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::PriorBoxLayer* cv_dnn_Layer_to_PriorBoxLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::PriorBoxLayer*>(instance);
	}

	// cv::dnn::Layer::to_ProposalLayer() generated
	// ("cv::dnn::Layer::to_ProposalLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ProposalLayer* cv_dnn_Layer_to_ProposalLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ProposalLayer*>(instance);
	}

	// cv::dnn::Layer::to_QuantizeLayer() generated
	// ("cv::dnn::Layer::to_QuantizeLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::QuantizeLayer* cv_dnn_Layer_to_QuantizeLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::QuantizeLayer*>(instance);
	}

	// cv::dnn::Layer::to_RNNLayer() generated
	// ("cv::dnn::Layer::to_RNNLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::RNNLayer* cv_dnn_Layer_to_RNNLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::RNNLayer*>(instance);
	}

	// cv::dnn::Layer::to_ReLU6Layer() generated
	// ("cv::dnn::Layer::to_ReLU6Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ReLU6Layer* cv_dnn_Layer_to_ReLU6Layer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ReLU6Layer*>(instance);
	}

	// cv::dnn::Layer::to_ReLULayer() generated
	// ("cv::dnn::Layer::to_ReLULayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ReLULayer* cv_dnn_Layer_to_ReLULayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ReLULayer*>(instance);
	}

	// cv::dnn::Layer::to_ReciprocalLayer() generated
	// ("cv::dnn::Layer::to_ReciprocalLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ReciprocalLayer* cv_dnn_Layer_to_ReciprocalLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ReciprocalLayer*>(instance);
	}

	// cv::dnn::Layer::to_ReduceLayer() generated
	// ("cv::dnn::Layer::to_ReduceLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ReduceLayer* cv_dnn_Layer_to_ReduceLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ReduceLayer*>(instance);
	}

	// cv::dnn::Layer::to_RegionLayer() generated
	// ("cv::dnn::Layer::to_RegionLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::RegionLayer* cv_dnn_Layer_to_RegionLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::RegionLayer*>(instance);
	}

	// cv::dnn::Layer::to_ReorgLayer() generated
	// ("cv::dnn::Layer::to_ReorgLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ReorgLayer* cv_dnn_Layer_to_ReorgLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ReorgLayer*>(instance);
	}

	// cv::dnn::Layer::to_RequantizeLayer() generated
	// ("cv::dnn::Layer::to_RequantizeLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::RequantizeLayer* cv_dnn_Layer_to_RequantizeLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::RequantizeLayer*>(instance);
	}

	// cv::dnn::Layer::to_ReshapeLayer() generated
	// ("cv::dnn::Layer::to_ReshapeLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ReshapeLayer* cv_dnn_Layer_to_ReshapeLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ReshapeLayer*>(instance);
	}

	// cv::dnn::Layer::to_ResizeLayer() generated
	// ("cv::dnn::Layer::to_ResizeLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ResizeLayer* cv_dnn_Layer_to_ResizeLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ResizeLayer*>(instance);
	}

	// cv::dnn::Layer::to_RoundLayer() generated
	// ("cv::dnn::Layer::to_RoundLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::RoundLayer* cv_dnn_Layer_to_RoundLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::RoundLayer*>(instance);
	}

	// cv::dnn::Layer::to_ScaleLayer() generated
	// ("cv::dnn::Layer::to_ScaleLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ScaleLayer* cv_dnn_Layer_to_ScaleLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ScaleLayer*>(instance);
	}

	// cv::dnn::Layer::to_ScaleLayerInt8() generated
	// ("cv::dnn::Layer::to_ScaleLayerInt8", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ScaleLayerInt8* cv_dnn_Layer_to_ScaleLayerInt8(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ScaleLayerInt8*>(instance);
	}

	// cv::dnn::Layer::to_ScatterLayer() generated
	// ("cv::dnn::Layer::to_ScatterLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ScatterLayer* cv_dnn_Layer_to_ScatterLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ScatterLayer*>(instance);
	}

	// cv::dnn::Layer::to_ScatterNDLayer() generated
	// ("cv::dnn::Layer::to_ScatterNDLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ScatterNDLayer* cv_dnn_Layer_to_ScatterNDLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ScatterNDLayer*>(instance);
	}

	// cv::dnn::Layer::to_SeluLayer() generated
	// ("cv::dnn::Layer::to_SeluLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SeluLayer* cv_dnn_Layer_to_SeluLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::SeluLayer*>(instance);
	}

	// cv::dnn::Layer::to_ShiftLayer() generated
	// ("cv::dnn::Layer::to_ShiftLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ShiftLayer* cv_dnn_Layer_to_ShiftLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ShiftLayer*>(instance);
	}

	// cv::dnn::Layer::to_ShiftLayerInt8() generated
	// ("cv::dnn::Layer::to_ShiftLayerInt8", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ShiftLayerInt8* cv_dnn_Layer_to_ShiftLayerInt8(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ShiftLayerInt8*>(instance);
	}

	// cv::dnn::Layer::to_ShrinkLayer() generated
	// ("cv::dnn::Layer::to_ShrinkLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ShrinkLayer* cv_dnn_Layer_to_ShrinkLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ShrinkLayer*>(instance);
	}

	// cv::dnn::Layer::to_ShuffleChannelLayer() generated
	// ("cv::dnn::Layer::to_ShuffleChannelLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ShuffleChannelLayer* cv_dnn_Layer_to_ShuffleChannelLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ShuffleChannelLayer*>(instance);
	}

	// cv::dnn::Layer::to_SigmoidLayer() generated
	// ("cv::dnn::Layer::to_SigmoidLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SigmoidLayer* cv_dnn_Layer_to_SigmoidLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::SigmoidLayer*>(instance);
	}

	// cv::dnn::Layer::to_SignLayer() generated
	// ("cv::dnn::Layer::to_SignLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SignLayer* cv_dnn_Layer_to_SignLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::SignLayer*>(instance);
	}

	// cv::dnn::Layer::to_SinLayer() generated
	// ("cv::dnn::Layer::to_SinLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SinLayer* cv_dnn_Layer_to_SinLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::SinLayer*>(instance);
	}

	// cv::dnn::Layer::to_SinhLayer() generated
	// ("cv::dnn::Layer::to_SinhLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SinhLayer* cv_dnn_Layer_to_SinhLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::SinhLayer*>(instance);
	}

	// cv::dnn::Layer::to_SliceLayer() generated
	// ("cv::dnn::Layer::to_SliceLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SliceLayer* cv_dnn_Layer_to_SliceLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::SliceLayer*>(instance);
	}

	// cv::dnn::Layer::to_SoftmaxLayer() generated
	// ("cv::dnn::Layer::to_SoftmaxLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SoftmaxLayer* cv_dnn_Layer_to_SoftmaxLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::SoftmaxLayer*>(instance);
	}

	// cv::dnn::Layer::to_SoftmaxLayerInt8() generated
	// ("cv::dnn::Layer::to_SoftmaxLayerInt8", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SoftmaxLayerInt8* cv_dnn_Layer_to_SoftmaxLayerInt8(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::SoftmaxLayerInt8*>(instance);
	}

	// cv::dnn::Layer::to_SoftplusLayer() generated
	// ("cv::dnn::Layer::to_SoftplusLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SoftplusLayer* cv_dnn_Layer_to_SoftplusLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::SoftplusLayer*>(instance);
	}

	// cv::dnn::Layer::to_SoftsignLayer() generated
	// ("cv::dnn::Layer::to_SoftsignLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SoftsignLayer* cv_dnn_Layer_to_SoftsignLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::SoftsignLayer*>(instance);
	}

	// cv::dnn::Layer::to_SpaceToDepthLayer() generated
	// ("cv::dnn::Layer::to_SpaceToDepthLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SpaceToDepthLayer* cv_dnn_Layer_to_SpaceToDepthLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::SpaceToDepthLayer*>(instance);
	}

	// cv::dnn::Layer::to_SplitLayer() generated
	// ("cv::dnn::Layer::to_SplitLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SplitLayer* cv_dnn_Layer_to_SplitLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::SplitLayer*>(instance);
	}

	// cv::dnn::Layer::to_SqrtLayer() generated
	// ("cv::dnn::Layer::to_SqrtLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SqrtLayer* cv_dnn_Layer_to_SqrtLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::SqrtLayer*>(instance);
	}

	// cv::dnn::Layer::to_SwishLayer() generated
	// ("cv::dnn::Layer::to_SwishLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SwishLayer* cv_dnn_Layer_to_SwishLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::SwishLayer*>(instance);
	}

	// cv::dnn::Layer::to_TanHLayer() generated
	// ("cv::dnn::Layer::to_TanHLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::TanHLayer* cv_dnn_Layer_to_TanHLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::TanHLayer*>(instance);
	}

	// cv::dnn::Layer::to_TanLayer() generated
	// ("cv::dnn::Layer::to_TanLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::TanLayer* cv_dnn_Layer_to_TanLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::TanLayer*>(instance);
	}

	// cv::dnn::Layer::to_ThresholdedReluLayer() generated
	// ("cv::dnn::Layer::to_ThresholdedReluLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ThresholdedReluLayer* cv_dnn_Layer_to_ThresholdedReluLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ThresholdedReluLayer*>(instance);
	}

	// cv::dnn::Layer::to_TileLayer() generated
	// ("cv::dnn::Layer::to_TileLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::TileLayer* cv_dnn_Layer_to_TileLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::TileLayer*>(instance);
	}

	// cv::dnn::Layer::to_TopKLayer() generated
	// ("cv::dnn::Layer::to_TopKLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::TopKLayer* cv_dnn_Layer_to_TopKLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::TopKLayer*>(instance);
	}

	// cv::dnn::Layer::to_Algorithm() generated
	// ("cv::dnn::Layer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_Layer_to_Algorithm(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::Layer::delete() generated
	// ("cv::dnn::Layer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_Layer_delete(cv::dnn::Layer* instance) {
			delete instance;
	}

	// registerLayer(const String &, Constructor)(InString, Function) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/layer.hpp:64
	// ("cv::dnn::LayerFactory::registerLayer", vec![(pred!(mut, ["type", "constructor"], ["const cv::String*", "cv::dnn::LayerFactory::Constructor"]), _)]),
	void cv_dnn_LayerFactory_registerLayer_const_StringR_Constructor(const char* type, cv::dnn::LayerFactory::Constructor constructor, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::LayerFactory::registerLayer(std::string(type), constructor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// unregisterLayer(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/layer.hpp:67
	// ("cv::dnn::LayerFactory::unregisterLayer", vec![(pred!(mut, ["type"], ["const cv::String*"]), _)]),
	void cv_dnn_LayerFactory_unregisterLayer_const_StringR(const char* type, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::LayerFactory::unregisterLayer(std::string(type));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isLayerRegistered(const std::string &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/layer.hpp:70
	// ("cv::dnn::LayerFactory::isLayerRegistered", vec![(pred!(mut, ["type"], ["const std::string*"]), _)]),
	void cv_dnn_LayerFactory_isLayerRegistered_const_stringR(const char* type, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::dnn::LayerFactory::isLayerRegistered(std::string(type));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createLayerInstance(const String &, LayerParams &)(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/layer.hpp:77
	// ("cv::dnn::LayerFactory::createLayerInstance", vec![(pred!(mut, ["type", "params"], ["const cv::String*", "cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_LayerFactory_createLayerInstance_const_StringR_LayerParamsR(const char* type, cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::Layer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::LayerFactory::createLayerInstance(std::string(type), *params);
			Ok(new cv::Ptr<cv::dnn::Layer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::LayerFactory::delete() generated
	// ("cv::dnn::LayerFactory::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_LayerFactory_delete(cv::dnn::LayerFactory* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1150
	// ("cv::dnn::LayerNormLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_LayerNormLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::LayerNormLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::LayerNormLayer> ret = cv::dnn::LayerNormLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::LayerNormLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::LayerNormLayer::defaultNew() generated
	// ("cv::dnn::LayerNormLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::LayerNormLayer* cv_dnn_LayerNormLayer_defaultNew_const() {
			cv::dnn::LayerNormLayer* ret = new cv::dnn::LayerNormLayer();
			return ret;
	}

	// cv::dnn::LayerNormLayer::hasBias() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1146
	// ("cv::dnn::LayerNormLayer::hasBias", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_LayerNormLayer_propHasBias_const(const cv::dnn::LayerNormLayer* instance) {
			bool ret = instance->hasBias;
			return ret;
	}

	// cv::dnn::LayerNormLayer::setHasBias(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1146
	// ("cv::dnn::LayerNormLayer::setHasBias", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_dnn_LayerNormLayer_propHasBias_const_bool(cv::dnn::LayerNormLayer* instance, const bool val) {
			instance->hasBias = val;
	}

	// cv::dnn::LayerNormLayer::axis() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1147
	// ("cv::dnn::LayerNormLayer::axis", vec![(pred!(const, [], []), _)]),
	int cv_dnn_LayerNormLayer_propAxis_const(const cv::dnn::LayerNormLayer* instance) {
			int ret = instance->axis;
			return ret;
	}

	// cv::dnn::LayerNormLayer::setAxis(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1147
	// ("cv::dnn::LayerNormLayer::setAxis", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_LayerNormLayer_propAxis_const_int(cv::dnn::LayerNormLayer* instance, const int val) {
			instance->axis = val;
	}

	// cv::dnn::LayerNormLayer::epsilon() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1148
	// ("cv::dnn::LayerNormLayer::epsilon", vec![(pred!(const, [], []), _)]),
	float cv_dnn_LayerNormLayer_propEpsilon_const(const cv::dnn::LayerNormLayer* instance) {
			float ret = instance->epsilon;
			return ret;
	}

	// cv::dnn::LayerNormLayer::setEpsilon(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1148
	// ("cv::dnn::LayerNormLayer::setEpsilon", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_LayerNormLayer_propEpsilon_const_float(cv::dnn::LayerNormLayer* instance, const float val) {
			instance->epsilon = val;
	}

	// cv::dnn::LayerNormLayer::to_Algorithm() generated
	// ("cv::dnn::LayerNormLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_LayerNormLayer_to_Algorithm(cv::dnn::LayerNormLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::LayerNormLayer::to_Layer() generated
	// ("cv::dnn::LayerNormLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_LayerNormLayer_to_Layer(cv::dnn::LayerNormLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::LayerNormLayer::delete() generated
	// ("cv::dnn::LayerNormLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_LayerNormLayer_delete(cv::dnn::LayerNormLayer* instance) {
			delete instance;
	}

	// cv::dnn::LayerParams::defaultNew() generated
	// ("cv::dnn::LayerParams::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::LayerParams* cv_dnn_LayerParams_defaultNew_const() {
			cv::dnn::LayerParams* ret = new cv::dnn::LayerParams();
			return ret;
	}

	// cv::dnn::LayerParams::blobs() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:149
	// ("cv::dnn::LayerParams::blobs", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Mat>* cv_dnn_LayerParams_propBlobs_const(const cv::dnn::LayerParams* instance) {
			std::vector<cv::Mat> ret = instance->blobs;
			return new std::vector<cv::Mat>(ret);
	}

	// cv::dnn::LayerParams::setBlobs(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:149
	// ("cv::dnn::LayerParams::setBlobs", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	void cv_dnn_LayerParams_propBlobs_const_vectorLMatG(cv::dnn::LayerParams* instance, const std::vector<cv::Mat>* val) {
			instance->blobs = *val;
	}

	// cv::dnn::LayerParams::name() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:151
	// ("cv::dnn::LayerParams::name", vec![(pred!(const, [], []), _)]),
	void* cv_dnn_LayerParams_propName_const(const cv::dnn::LayerParams* instance) {
			cv::String ret = instance->name;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::dnn::LayerParams::setName(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:151
	// ("cv::dnn::LayerParams::setName", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	void cv_dnn_LayerParams_propName_const_String(cv::dnn::LayerParams* instance, const char* val) {
			instance->name = std::string(val);
	}

	// cv::dnn::LayerParams::type() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:152
	// ("cv::dnn::LayerParams::type", vec![(pred!(const, [], []), _)]),
	void* cv_dnn_LayerParams_propType_const(const cv::dnn::LayerParams* instance) {
			cv::String ret = instance->type;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::dnn::LayerParams::setType(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:152
	// ("cv::dnn::LayerParams::setType", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	void cv_dnn_LayerParams_propType_const_String(cv::dnn::LayerParams* instance, const char* val) {
			instance->type = std::string(val);
	}

	// cv::dnn::LayerParams::to_Dict() generated
	// ("cv::dnn::LayerParams::to_Dict", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Dict* cv_dnn_LayerParams_to_Dict(cv::dnn::LayerParams* instance) {
			return dynamic_cast<cv::dnn::Dict*>(instance);
	}

	// cv::dnn::LayerParams::delete() generated
	// ("cv::dnn::LayerParams::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_LayerParams_delete(cv::dnn::LayerParams* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:711
	// ("cv::dnn::LogLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_LogLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::LogLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::LogLayer> ret = cv::dnn::LogLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::LogLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::LogLayer::defaultNew() generated
	// ("cv::dnn::LogLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::LogLayer* cv_dnn_LogLayer_defaultNew_const() {
			cv::dnn::LogLayer* ret = new cv::dnn::LogLayer();
			return ret;
	}

	// cv::dnn::LogLayer::to_ActivationLayer() generated
	// ("cv::dnn::LogLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_LogLayer_to_ActivationLayer(cv::dnn::LogLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::LogLayer::to_Algorithm() generated
	// ("cv::dnn::LogLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_LogLayer_to_Algorithm(cv::dnn::LogLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::LogLayer::to_Layer() generated
	// ("cv::dnn::LogLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_LogLayer_to_Layer(cv::dnn::LogLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::LogLayer::delete() generated
	// ("cv::dnn::LogLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_LogLayer_delete(cv::dnn::LogLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:445
	// ("cv::dnn::MVNLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_MVNLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::MVNLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::MVNLayer> ret = cv::dnn::MVNLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::MVNLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::MVNLayer::defaultNew() generated
	// ("cv::dnn::MVNLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::MVNLayer* cv_dnn_MVNLayer_defaultNew_const() {
			cv::dnn::MVNLayer* ret = new cv::dnn::MVNLayer();
			return ret;
	}

	// cv::dnn::MVNLayer::eps() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:442
	// ("cv::dnn::MVNLayer::eps", vec![(pred!(const, [], []), _)]),
	float cv_dnn_MVNLayer_propEps_const(const cv::dnn::MVNLayer* instance) {
			float ret = instance->eps;
			return ret;
	}

	// cv::dnn::MVNLayer::setEps(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:442
	// ("cv::dnn::MVNLayer::setEps", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_MVNLayer_propEps_const_float(cv::dnn::MVNLayer* instance, const float val) {
			instance->eps = val;
	}

	// cv::dnn::MVNLayer::normVariance() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:443
	// ("cv::dnn::MVNLayer::normVariance", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_MVNLayer_propNormVariance_const(const cv::dnn::MVNLayer* instance) {
			bool ret = instance->normVariance;
			return ret;
	}

	// cv::dnn::MVNLayer::setNormVariance(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:443
	// ("cv::dnn::MVNLayer::setNormVariance", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_dnn_MVNLayer_propNormVariance_const_bool(cv::dnn::MVNLayer* instance, const bool val) {
			instance->normVariance = val;
	}

	// cv::dnn::MVNLayer::acrossChannels() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:443
	// ("cv::dnn::MVNLayer::acrossChannels", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_MVNLayer_propAcrossChannels_const(const cv::dnn::MVNLayer* instance) {
			bool ret = instance->acrossChannels;
			return ret;
	}

	// cv::dnn::MVNLayer::setAcrossChannels(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:443
	// ("cv::dnn::MVNLayer::setAcrossChannels", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_dnn_MVNLayer_propAcrossChannels_const_bool(cv::dnn::MVNLayer* instance, const bool val) {
			instance->acrossChannels = val;
	}

	// cv::dnn::MVNLayer::to_Algorithm() generated
	// ("cv::dnn::MVNLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_MVNLayer_to_Algorithm(cv::dnn::MVNLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::MVNLayer::to_Layer() generated
	// ("cv::dnn::MVNLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_MVNLayer_to_Layer(cv::dnn::MVNLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::MVNLayer::delete() generated
	// ("cv::dnn::MVNLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_MVNLayer_delete(cv::dnn::MVNLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1165
	// ("cv::dnn::MatMulLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_MatMulLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::MatMulLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::MatMulLayer> ret = cv::dnn::MatMulLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::MatMulLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::MatMulLayer::defaultNew() generated
	// ("cv::dnn::MatMulLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::MatMulLayer* cv_dnn_MatMulLayer_defaultNew_const() {
			cv::dnn::MatMulLayer* ret = new cv::dnn::MatMulLayer();
			return ret;
	}

	// cv::dnn::MatMulLayer::to_Algorithm() generated
	// ("cv::dnn::MatMulLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_MatMulLayer_to_Algorithm(cv::dnn::MatMulLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::MatMulLayer::to_Layer() generated
	// ("cv::dnn::MatMulLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_MatMulLayer_to_Layer(cv::dnn::MatMulLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::MatMulLayer::delete() generated
	// ("cv::dnn::MatMulLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_MatMulLayer_delete(cv::dnn::MatMulLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:951
	// ("cv::dnn::MaxUnpoolLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_MaxUnpoolLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::MaxUnpoolLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::MaxUnpoolLayer> ret = cv::dnn::MaxUnpoolLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::MaxUnpoolLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::MaxUnpoolLayer::defaultNew() generated
	// ("cv::dnn::MaxUnpoolLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::MaxUnpoolLayer* cv_dnn_MaxUnpoolLayer_defaultNew_const() {
			cv::dnn::MaxUnpoolLayer* ret = new cv::dnn::MaxUnpoolLayer();
			return ret;
	}

	// cv::dnn::MaxUnpoolLayer::poolKernel() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:947
	// ("cv::dnn::MaxUnpoolLayer::poolKernel", vec![(pred!(const, [], []), _)]),
	void cv_dnn_MaxUnpoolLayer_propPoolKernel_const(const cv::dnn::MaxUnpoolLayer* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->poolKernel;
			*ocvrs_return = ret;
	}

	// cv::dnn::MaxUnpoolLayer::setPoolKernel(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:947
	// ("cv::dnn::MaxUnpoolLayer::setPoolKernel", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_dnn_MaxUnpoolLayer_propPoolKernel_const_Size(cv::dnn::MaxUnpoolLayer* instance, const cv::Size* val) {
			instance->poolKernel = *val;
	}

	// cv::dnn::MaxUnpoolLayer::poolPad() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:948
	// ("cv::dnn::MaxUnpoolLayer::poolPad", vec![(pred!(const, [], []), _)]),
	void cv_dnn_MaxUnpoolLayer_propPoolPad_const(const cv::dnn::MaxUnpoolLayer* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->poolPad;
			*ocvrs_return = ret;
	}

	// cv::dnn::MaxUnpoolLayer::setPoolPad(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:948
	// ("cv::dnn::MaxUnpoolLayer::setPoolPad", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_dnn_MaxUnpoolLayer_propPoolPad_const_Size(cv::dnn::MaxUnpoolLayer* instance, const cv::Size* val) {
			instance->poolPad = *val;
	}

	// cv::dnn::MaxUnpoolLayer::poolStride() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:949
	// ("cv::dnn::MaxUnpoolLayer::poolStride", vec![(pred!(const, [], []), _)]),
	void cv_dnn_MaxUnpoolLayer_propPoolStride_const(const cv::dnn::MaxUnpoolLayer* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->poolStride;
			*ocvrs_return = ret;
	}

	// cv::dnn::MaxUnpoolLayer::setPoolStride(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:949
	// ("cv::dnn::MaxUnpoolLayer::setPoolStride", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_dnn_MaxUnpoolLayer_propPoolStride_const_Size(cv::dnn::MaxUnpoolLayer* instance, const cv::Size* val) {
			instance->poolStride = *val;
	}

	// cv::dnn::MaxUnpoolLayer::to_Algorithm() generated
	// ("cv::dnn::MaxUnpoolLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_MaxUnpoolLayer_to_Algorithm(cv::dnn::MaxUnpoolLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::MaxUnpoolLayer::to_Layer() generated
	// ("cv::dnn::MaxUnpoolLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_MaxUnpoolLayer_to_Layer(cv::dnn::MaxUnpoolLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::MaxUnpoolLayer::delete() generated
	// ("cv::dnn::MaxUnpoolLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_MaxUnpoolLayer_delete(cv::dnn::MaxUnpoolLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:659
	// ("cv::dnn::MishLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_MishLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::MishLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::MishLayer> ret = cv::dnn::MishLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::MishLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::MishLayer::defaultNew() generated
	// ("cv::dnn::MishLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::MishLayer* cv_dnn_MishLayer_defaultNew_const() {
			cv::dnn::MishLayer* ret = new cv::dnn::MishLayer();
			return ret;
	}

	// cv::dnn::MishLayer::to_ActivationLayer() generated
	// ("cv::dnn::MishLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_MishLayer_to_ActivationLayer(cv::dnn::MishLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::MishLayer::to_Algorithm() generated
	// ("cv::dnn::MishLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_MishLayer_to_Algorithm(cv::dnn::MishLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::MishLayer::to_Layer() generated
	// ("cv::dnn::MishLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_MishLayer_to_Layer(cv::dnn::MishLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::MishLayer::delete() generated
	// ("cv::dnn::MishLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_MishLayer_delete(cv::dnn::MishLayer* instance) {
			delete instance;
	}

	// Model()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1401
	// ("cv::dnn::Model::Model", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_Model_Model(Result<cv::dnn::Model*>* ocvrs_return) {
		try {
			cv::dnn::Model* ret = new cv::dnn::Model();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Model(const Model &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1403
	// ("cv::dnn::Model::Model", vec![(pred!(mut, ["unnamed"], ["const cv::dnn::Model*"]), _)]),
	cv::dnn::Model* cv_dnn_Model_Model_const_ModelR(const cv::dnn::Model* unnamed) {
			cv::dnn::Model* ret = new cv::dnn::Model(*unnamed);
			return ret;
	}

	// Model(Model &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1404
	// ("cv::dnn::Model::Model", vec![(pred!(mut, ["unnamed"], ["cv::dnn::Model*"]), _)]),
	cv::dnn::Model* cv_dnn_Model_Model_ModelRR(cv::dnn::Model* unnamed) {
			cv::dnn::Model* ret = new cv::dnn::Model(std::move(*unnamed));
			return ret;
	}

	// operator=(const Model &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1405
	// ("cv::dnn::Model::operator=", vec![(pred!(mut, ["unnamed"], ["const cv::dnn::Model*"]), _)]),
	void cv_dnn_Model_operatorST_const_ModelR(cv::dnn::Model* instance, const cv::dnn::Model* unnamed) {
			instance->operator=(*unnamed);
	}

	// operator=(Model &&)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1406
	// ("cv::dnn::Model::operator=", vec![(pred!(mut, ["unnamed"], ["cv::dnn::Model*"]), _)]),
	void cv_dnn_Model_operatorST_ModelRR(cv::dnn::Model* instance, cv::dnn::Model* unnamed) {
			instance->operator=(std::move(*unnamed));
	}

	// Model(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1414
	// ("cv::dnn::Model::Model", vec![(pred!(mut, ["model", "config"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_dnn_Model_Model_const_StringR_const_StringR(const char* model, const char* config, Result<cv::dnn::Model*>* ocvrs_return) {
		try {
			cv::dnn::Model* ret = new cv::dnn::Model(std::string(model), std::string(config));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::Model::Model(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1414
	// ("cv::dnn::Model::Model", vec![(pred!(mut, ["model"], ["const cv::String*"]), _)]),
	void cv_dnn_Model_Model_const_StringR(const char* model, Result<cv::dnn::Model*>* ocvrs_return) {
		try {
			cv::dnn::Model* ret = new cv::dnn::Model(std::string(model));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Model(const Net &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1420
	// ("cv::dnn::Model::Model", vec![(pred!(mut, ["network"], ["const cv::dnn::Net*"]), _)]),
	void cv_dnn_Model_Model_const_NetR(const cv::dnn::Net* network, Result<cv::dnn::Model*>* ocvrs_return) {
		try {
			cv::dnn::Model* ret = new cv::dnn::Model(*network);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInputSize(const Size &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1426
	// ("cv::dnn::Model::setInputSize", vec![(pred!(mut, ["size"], ["const cv::Size*"]), _)]),
	void cv_dnn_Model_setInputSize_const_SizeR(cv::dnn::Model* instance, const cv::Size* size, Result<cv::dnn::Model*>* ocvrs_return) {
		try {
			cv::dnn::Model ret = instance->setInputSize(*size);
			Ok(new cv::dnn::Model(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInputSize(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1433
	// ("cv::dnn::Model::setInputSize", vec![(pred!(mut, ["width", "height"], ["int", "int"]), _)]),
	void cv_dnn_Model_setInputSize_int_int(cv::dnn::Model* instance, int width, int height, Result<cv::dnn::Model*>* ocvrs_return) {
		try {
			cv::dnn::Model ret = instance->setInputSize(width, height);
			Ok(new cv::dnn::Model(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInputMean(const Scalar &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1438
	// ("cv::dnn::Model::setInputMean", vec![(pred!(mut, ["mean"], ["const cv::Scalar*"]), _)]),
	void cv_dnn_Model_setInputMean_const_ScalarR(cv::dnn::Model* instance, const cv::Scalar* mean, Result<cv::dnn::Model*>* ocvrs_return) {
		try {
			cv::dnn::Model ret = instance->setInputMean(*mean);
			Ok(new cv::dnn::Model(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInputScale(const Scalar &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1443
	// ("cv::dnn::Model::setInputScale", vec![(pred!(mut, ["scale"], ["const cv::Scalar*"]), _)]),
	void cv_dnn_Model_setInputScale_const_ScalarR(cv::dnn::Model* instance, const cv::Scalar* scale, Result<cv::dnn::Model*>* ocvrs_return) {
		try {
			cv::dnn::Model ret = instance->setInputScale(*scale);
			Ok(new cv::dnn::Model(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInputCrop(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1448
	// ("cv::dnn::Model::setInputCrop", vec![(pred!(mut, ["crop"], ["bool"]), _)]),
	void cv_dnn_Model_setInputCrop_bool(cv::dnn::Model* instance, bool crop, Result<cv::dnn::Model*>* ocvrs_return) {
		try {
			cv::dnn::Model ret = instance->setInputCrop(crop);
			Ok(new cv::dnn::Model(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInputSwapRB(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1453
	// ("cv::dnn::Model::setInputSwapRB", vec![(pred!(mut, ["swapRB"], ["bool"]), _)]),
	void cv_dnn_Model_setInputSwapRB_bool(cv::dnn::Model* instance, bool swapRB, Result<cv::dnn::Model*>* ocvrs_return) {
		try {
			cv::dnn::Model ret = instance->setInputSwapRB(swapRB);
			Ok(new cv::dnn::Model(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setOutputNames(const std::vector<String> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1458
	// ("cv::dnn::Model::setOutputNames", vec![(pred!(mut, ["outNames"], ["const std::vector<cv::String>*"]), _)]),
	void cv_dnn_Model_setOutputNames_const_vectorLStringGR(cv::dnn::Model* instance, const std::vector<cv::String>* outNames, Result<cv::dnn::Model*>* ocvrs_return) {
		try {
			cv::dnn::Model ret = instance->setOutputNames(*outNames);
			Ok(new cv::dnn::Model(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInputParams(double, const Size &, const Scalar &, bool, bool)(Primitive, SimpleClass, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1468
	// ("cv::dnn::Model::setInputParams", vec![(pred!(mut, ["scale", "size", "mean", "swapRB", "crop"], ["double", "const cv::Size*", "const cv::Scalar*", "bool", "bool"]), _)]),
	void cv_dnn_Model_setInputParams_double_const_SizeR_const_ScalarR_bool_bool(cv::dnn::Model* instance, double scale, const cv::Size* size, const cv::Scalar* mean, bool swapRB, bool crop, ResultVoid* ocvrs_return) {
		try {
			instance->setInputParams(scale, *size, *mean, swapRB, crop);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::Model::setInputParams() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1468
	// ("cv::dnn::Model::setInputParams", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_Model_setInputParams(cv::dnn::Model* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setInputParams();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// predict(InputArray, OutputArrayOfArrays)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1475
	// ("cv::dnn::Model::predict", vec![(pred!(const, ["frame", "outs"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_dnn_Model_predict_const_const__InputArrayR_const__OutputArrayR(const cv::dnn::Model* instance, const cv::_InputArray* frame, const cv::_OutputArray* outs, ResultVoid* ocvrs_return) {
		try {
			instance->predict(*frame, *outs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPreferableBackend(dnn::Backend)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1485
	// ("cv::dnn::Model::setPreferableBackend", vec![(pred!(mut, ["backendId"], ["cv::dnn::Backend"]), _)]),
	void cv_dnn_Model_setPreferableBackend_Backend(cv::dnn::Model* instance, cv::dnn::Backend backendId, Result<cv::dnn::Model*>* ocvrs_return) {
		try {
			cv::dnn::Model ret = instance->setPreferableBackend(backendId);
			Ok(new cv::dnn::Model(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPreferableTarget(dnn::Target)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1487
	// ("cv::dnn::Model::setPreferableTarget", vec![(pred!(mut, ["targetId"], ["cv::dnn::Target"]), _)]),
	void cv_dnn_Model_setPreferableTarget_Target(cv::dnn::Model* instance, cv::dnn::Target targetId, Result<cv::dnn::Model*>* ocvrs_return) {
		try {
			cv::dnn::Model ret = instance->setPreferableTarget(targetId);
			Ok(new cv::dnn::Model(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// enableWinograd(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1490
	// ("cv::dnn::Model::enableWinograd", vec![(pred!(mut, ["useWinograd"], ["bool"]), _)]),
	void cv_dnn_Model_enableWinograd_bool(cv::dnn::Model* instance, bool useWinograd, Result<cv::dnn::Model*>* ocvrs_return) {
		try {
			cv::dnn::Model ret = instance->enableWinograd(useWinograd);
			Ok(new cv::dnn::Model(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNetwork_()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1496
	// ("cv::dnn::Model::getNetwork_", vec![(pred!(const, [], []), _)]),
	void cv_dnn_Model_getNetwork__const(const cv::dnn::Model* instance, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = instance->getNetwork_();
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNetwork_()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1497
	// ("cv::dnn::Model::getNetwork_", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_Model_getNetwork_(cv::dnn::Model* instance, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = instance->getNetwork_();
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::Model::implicitClone() generated
	// ("cv::dnn::Model::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::dnn::Model* cv_dnn_Model_implicitClone_const(const cv::dnn::Model* instance) {
			return new cv::dnn::Model(*instance);
	}

	// cv::dnn::Model::delete() generated
	// ("cv::dnn::Model::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_Model_delete(cv::dnn::Model* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:924
	// ("cv::dnn::NaryEltwiseLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_NaryEltwiseLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::NaryEltwiseLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::NaryEltwiseLayer> ret = cv::dnn::NaryEltwiseLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::NaryEltwiseLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::NaryEltwiseLayer::defaultNew() generated
	// ("cv::dnn::NaryEltwiseLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::NaryEltwiseLayer* cv_dnn_NaryEltwiseLayer_defaultNew_const() {
			cv::dnn::NaryEltwiseLayer* ret = new cv::dnn::NaryEltwiseLayer();
			return ret;
	}

	// cv::dnn::NaryEltwiseLayer::to_Algorithm() generated
	// ("cv::dnn::NaryEltwiseLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_NaryEltwiseLayer_to_Algorithm(cv::dnn::NaryEltwiseLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::NaryEltwiseLayer::to_Layer() generated
	// ("cv::dnn::NaryEltwiseLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_NaryEltwiseLayer_to_Layer(cv::dnn::NaryEltwiseLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::NaryEltwiseLayer::delete() generated
	// ("cv::dnn::NaryEltwiseLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_NaryEltwiseLayer_delete(cv::dnn::NaryEltwiseLayer* instance) {
			delete instance;
	}

	// Net()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:478
	// ("cv::dnn::Net::Net", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_Net_Net(Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net* ret = new cv::dnn::Net();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readFromModelOptimizer(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:487
	// ("cv::dnn::Net::readFromModelOptimizer", vec![(pred!(mut, ["xml", "bin"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_dnn_Net_readFromModelOptimizer_const_StringR_const_StringR(const char* xml, const char* bin, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::Net::readFromModelOptimizer(std::string(xml), std::string(bin));
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readFromModelOptimizer(const std::vector<uchar> &, const std::vector<uchar> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:495
	// ("cv::dnn::Net::readFromModelOptimizer", vec![(pred!(mut, ["bufferModelConfig", "bufferWeights"], ["const std::vector<unsigned char>*", "const std::vector<unsigned char>*"]), _)]),
	void cv_dnn_Net_readFromModelOptimizer_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR(const std::vector<unsigned char>* bufferModelConfig, const std::vector<unsigned char>* bufferWeights, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::Net::readFromModelOptimizer(*bufferModelConfig, *bufferWeights);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readFromModelOptimizer(const uchar *, size_t, const uchar *, size_t)(VariableArray, Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:505
	// ("cv::dnn::Net::readFromModelOptimizer", vec![(pred!(mut, ["bufferModelConfigPtr", "bufferModelConfigSize", "bufferWeightsPtr", "bufferWeightsSize"], ["const unsigned char*", "size_t", "const unsigned char*", "size_t"]), _)]),
	void cv_dnn_Net_readFromModelOptimizer_const_unsigned_charX_size_t_const_unsigned_charX_size_t(const unsigned char* bufferModelConfigPtr, size_t bufferModelConfigSize, const unsigned char* bufferWeightsPtr, size_t bufferWeightsSize, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::Net::readFromModelOptimizer(bufferModelConfigPtr, bufferModelConfigSize, bufferWeightsPtr, bufferWeightsSize);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:509
	// ("cv::dnn::Net::empty", vec![(pred!(const, [], []), _)]),
	void cv_dnn_Net_empty_const(const cv::dnn::Net* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dump()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:515
	// ("cv::dnn::Net::dump", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_Net_dump(cv::dnn::Net* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->dump();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dumpToFile(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:520
	// ("cv::dnn::Net::dumpToFile", vec![(pred!(mut, ["path"], ["const cv::String*"]), _)]),
	void cv_dnn_Net_dumpToFile_const_StringR(cv::dnn::Net* instance, const char* path, ResultVoid* ocvrs_return) {
		try {
			instance->dumpToFile(std::string(path));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dumpToPbtxt(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:527
	// ("cv::dnn::Net::dumpToPbtxt", vec![(pred!(mut, ["path"], ["const cv::String*"]), _)]),
	void cv_dnn_Net_dumpToPbtxt_const_StringR(cv::dnn::Net* instance, const char* path, ResultVoid* ocvrs_return) {
		try {
			instance->dumpToPbtxt(std::string(path));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addLayer(const String &, const String &, const int &, LayerParams &)(InString, InString, Indirect, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:536
	// ("cv::dnn::Net::addLayer", vec![(pred!(mut, ["name", "type", "dtype", "params"], ["const cv::String*", "const cv::String*", "const int*", "cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_Net_addLayer_const_StringR_const_StringR_const_intR_LayerParamsR(cv::dnn::Net* instance, const char* name, const char* type, const int* dtype, cv::dnn::LayerParams* params, Result<int>* ocvrs_return) {
		try {
			int ret = instance->addLayer(std::string(name), std::string(type), *dtype, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addLayer(const String &, const String &, LayerParams &)(InString, InString, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:539
	// ("cv::dnn::Net::addLayer", vec![(pred!(mut, ["name", "type", "params"], ["const cv::String*", "const cv::String*", "cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_Net_addLayer_const_StringR_const_StringR_LayerParamsR(cv::dnn::Net* instance, const char* name, const char* type, cv::dnn::LayerParams* params, Result<int>* ocvrs_return) {
		try {
			int ret = instance->addLayer(std::string(name), std::string(type), *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addLayerToPrev(const String &, const String &, const int &, LayerParams &)(InString, InString, Indirect, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:544
	// ("cv::dnn::Net::addLayerToPrev", vec![(pred!(mut, ["name", "type", "dtype", "params"], ["const cv::String*", "const cv::String*", "const int*", "cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_Net_addLayerToPrev_const_StringR_const_StringR_const_intR_LayerParamsR(cv::dnn::Net* instance, const char* name, const char* type, const int* dtype, cv::dnn::LayerParams* params, Result<int>* ocvrs_return) {
		try {
			int ret = instance->addLayerToPrev(std::string(name), std::string(type), *dtype, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addLayerToPrev(const String &, const String &, LayerParams &)(InString, InString, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:547
	// ("cv::dnn::Net::addLayerToPrev", vec![(pred!(mut, ["name", "type", "params"], ["const cv::String*", "const cv::String*", "cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_Net_addLayerToPrev_const_StringR_const_StringR_LayerParamsR(cv::dnn::Net* instance, const char* name, const char* type, cv::dnn::LayerParams* params, Result<int>* ocvrs_return) {
		try {
			int ret = instance->addLayerToPrev(std::string(name), std::string(type), *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLayerId(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:552
	// ("cv::dnn::Net::getLayerId", vec![(pred!(const, ["layer"], ["const cv::String*"]), _)]),
	void cv_dnn_Net_getLayerId_const_const_StringR(const cv::dnn::Net* instance, const char* layer, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getLayerId(std::string(layer));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLayerNames()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:554
	// ("cv::dnn::Net::getLayerNames", vec![(pred!(const, [], []), _)]),
	void cv_dnn_Net_getLayerNames_const(const cv::dnn::Net* instance, Result<std::vector<cv::String>*>* ocvrs_return) {
		try {
			std::vector<cv::String> ret = instance->getLayerNames();
			Ok(new std::vector<cv::String>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLayer(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:563
	// ("cv::dnn::Net::getLayer", vec![(pred!(const, ["layerId"], ["int"]), _)]),
	void cv_dnn_Net_getLayer_const_int(const cv::dnn::Net* instance, int layerId, Result<cv::Ptr<cv::dnn::Layer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = instance->getLayer(layerId);
			Ok(new cv::Ptr<cv::dnn::Layer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLayer(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:567
	// ("cv::dnn::Net::getLayer", vec![(pred!(const, ["layerName"], ["const cv::String*"]), _)]),
	void cv_dnn_Net_getLayer_const_const_StringR(const cv::dnn::Net* instance, const char* layerName, Result<cv::Ptr<cv::dnn::Layer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = instance->getLayer(std::string(layerName));
			Ok(new cv::Ptr<cv::dnn::Layer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLayer(const LayerId &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:571
	// ("cv::dnn::Net::getLayer", vec![(pred!(const, ["layerId"], ["const cv::dnn::Net::LayerId*"]), _)]),
	void cv_dnn_Net_getLayer_const_const_LayerIdR(const cv::dnn::Net* instance, const cv::dnn::Net::LayerId* layerId, Result<cv::Ptr<cv::dnn::Layer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = instance->getLayer(*layerId);
			Ok(new cv::Ptr<cv::dnn::Layer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLayerInputs(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:574
	// ("cv::dnn::Net::getLayerInputs", vec![(pred!(const, ["layerId"], ["int"]), _)]),
	void cv_dnn_Net_getLayerInputs_const_int(const cv::dnn::Net* instance, int layerId, Result<std::vector<cv::Ptr<cv::dnn::Layer>>*>* ocvrs_return) {
		try {
			std::vector<cv::Ptr<cv::dnn::Layer>> ret = instance->getLayerInputs(layerId);
			Ok(new std::vector<cv::Ptr<cv::dnn::Layer>>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// connect(String, String)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:589
	// ("cv::dnn::Net::connect", vec![(pred!(mut, ["outPin", "inpPin"], ["cv::String", "cv::String"]), _)]),
	void cv_dnn_Net_connect_String_String(cv::dnn::Net* instance, const char* outPin, const char* inpPin, ResultVoid* ocvrs_return) {
		try {
			instance->connect(std::string(outPin), std::string(inpPin));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// connect(int, int, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:597
	// ("cv::dnn::Net::connect", vec![(pred!(mut, ["outLayerId", "outNum", "inpLayerId", "inpNum"], ["int", "int", "int", "int"]), _)]),
	void cv_dnn_Net_connect_int_int_int_int(cv::dnn::Net* instance, int outLayerId, int outNum, int inpLayerId, int inpNum, ResultVoid* ocvrs_return) {
		try {
			instance->connect(outLayerId, outNum, inpLayerId, inpNum);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// registerOutput(const std::string &, int, int)(InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:609
	// ("cv::dnn::Net::registerOutput", vec![(pred!(mut, ["outputName", "layerId", "outputPort"], ["const std::string*", "int", "int"]), _)]),
	void cv_dnn_Net_registerOutput_const_stringR_int_int(cv::dnn::Net* instance, const char* outputName, int layerId, int outputPort, Result<int>* ocvrs_return) {
		try {
			int ret = instance->registerOutput(std::string(outputName), layerId, outputPort);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInputsNames(const std::vector<String> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:618
	// ("cv::dnn::Net::setInputsNames", vec![(pred!(mut, ["inputBlobNames"], ["const std::vector<cv::String>*"]), _)]),
	void cv_dnn_Net_setInputsNames_const_vectorLStringGR(cv::dnn::Net* instance, const std::vector<cv::String>* inputBlobNames, ResultVoid* ocvrs_return) {
		try {
			instance->setInputsNames(*inputBlobNames);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInputShape(const String &, const MatShape &)(InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:622
	// ("cv::dnn::Net::setInputShape", vec![(pred!(mut, ["inputName", "shape"], ["const cv::String*", "const cv::dnn::MatShape*"]), _)]),
	void cv_dnn_Net_setInputShape_const_StringR_const_MatShapeR(cv::dnn::Net* instance, const char* inputName, const cv::dnn::MatShape* shape, ResultVoid* ocvrs_return) {
		try {
			instance->setInputShape(std::string(inputName), *shape);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// forward(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:629
	// ("cv::dnn::Net::forward", vec![(pred!(mut, ["outputName"], ["const cv::String*"]), _)]),
	void cv_dnn_Net_forward_const_StringR(cv::dnn::Net* instance, const char* outputName, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->forward(std::string(outputName));
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::Net::forward() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:629
	// ("cv::dnn::Net::forward", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_Net_forward(cv::dnn::Net* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->forward();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// forwardAsync(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:638
	// ("cv::dnn::Net::forwardAsync", vec![(pred!(mut, ["outputName"], ["const cv::String*"]), _)]),
	void cv_dnn_Net_forwardAsync_const_StringR(cv::dnn::Net* instance, const char* outputName, Result<cv::AsyncArray*>* ocvrs_return) {
		try {
			cv::AsyncArray ret = instance->forwardAsync(std::string(outputName));
			Ok(new cv::AsyncArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::Net::forwardAsync() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:638
	// ("cv::dnn::Net::forwardAsync", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_Net_forwardAsync(cv::dnn::Net* instance, Result<cv::AsyncArray*>* ocvrs_return) {
		try {
			cv::AsyncArray ret = instance->forwardAsync();
			Ok(new cv::AsyncArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// forward(OutputArrayOfArrays, const String &)(OutputArray, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:645
	// ("cv::dnn::Net::forward", vec![(pred!(mut, ["outputBlobs", "outputName"], ["const cv::_OutputArray*", "const cv::String*"]), _)]),
	void cv_dnn_Net_forward_const__OutputArrayR_const_StringR(cv::dnn::Net* instance, const cv::_OutputArray* outputBlobs, const char* outputName, ResultVoid* ocvrs_return) {
		try {
			instance->forward(*outputBlobs, std::string(outputName));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::Net::forward(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:645
	// ("cv::dnn::Net::forward", vec![(pred!(mut, ["outputBlobs"], ["const cv::_OutputArray*"]), _)]),
	void cv_dnn_Net_forward_const__OutputArrayR(cv::dnn::Net* instance, const cv::_OutputArray* outputBlobs, ResultVoid* ocvrs_return) {
		try {
			instance->forward(*outputBlobs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// forward(OutputArrayOfArrays, const std::vector<String> &)(OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:651
	// ("cv::dnn::Net::forward", vec![(pred!(mut, ["outputBlobs", "outBlobNames"], ["const cv::_OutputArray*", "const std::vector<cv::String>*"]), _)]),
	void cv_dnn_Net_forward_const__OutputArrayR_const_vectorLStringGR(cv::dnn::Net* instance, const cv::_OutputArray* outputBlobs, const std::vector<cv::String>* outBlobNames, ResultVoid* ocvrs_return) {
		try {
			instance->forward(*outputBlobs, *outBlobNames);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// forward(std::vector<std::vector<Mat>> &, const std::vector<String> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:658
	// ("cv::dnn::Net::forward", vec![(pred!(mut, ["outputBlobs", "outBlobNames"], ["std::vector<std::vector<cv::Mat>>*", "const std::vector<cv::String>*"]), _)]),
	void cv_dnn_Net_forward_vectorLvectorLMatGGR_const_vectorLStringGR(cv::dnn::Net* instance, std::vector<std::vector<cv::Mat>>* outputBlobs, const std::vector<cv::String>* outBlobNames, ResultVoid* ocvrs_return) {
		try {
			instance->forward(*outputBlobs, *outBlobNames);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// quantize(InputArrayOfArrays, int, int, bool)(InputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:668
	// ("cv::dnn::Net::quantize", vec![(pred!(mut, ["calibData", "inputsDtype", "outputsDtype", "perChannel"], ["const cv::_InputArray*", "int", "int", "bool"]), _)]),
	void cv_dnn_Net_quantize_const__InputArrayR_int_int_bool(cv::dnn::Net* instance, const cv::_InputArray* calibData, int inputsDtype, int outputsDtype, bool perChannel, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = instance->quantize(*calibData, inputsDtype, outputsDtype, perChannel);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::Net::quantize(InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:668
	// ("cv::dnn::Net::quantize", vec![(pred!(mut, ["calibData", "inputsDtype", "outputsDtype"], ["const cv::_InputArray*", "int", "int"]), _)]),
	void cv_dnn_Net_quantize_const__InputArrayR_int_int(cv::dnn::Net* instance, const cv::_InputArray* calibData, int inputsDtype, int outputsDtype, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = instance->quantize(*calibData, inputsDtype, outputsDtype);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getInputDetails(std::vector<float> &, std::vector<int> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:674
	// ("cv::dnn::Net::getInputDetails", vec![(pred!(const, ["scales", "zeropoints"], ["std::vector<float>*", "std::vector<int>*"]), _)]),
	void cv_dnn_Net_getInputDetails_const_vectorLfloatGR_vectorLintGR(const cv::dnn::Net* instance, std::vector<float>* scales, std::vector<int>* zeropoints, ResultVoid* ocvrs_return) {
		try {
			instance->getInputDetails(*scales, *zeropoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOutputDetails(std::vector<float> &, std::vector<int> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:680
	// ("cv::dnn::Net::getOutputDetails", vec![(pred!(const, ["scales", "zeropoints"], ["std::vector<float>*", "std::vector<int>*"]), _)]),
	void cv_dnn_Net_getOutputDetails_const_vectorLfloatGR_vectorLintGR(const cv::dnn::Net* instance, std::vector<float>* scales, std::vector<int>* zeropoints, ResultVoid* ocvrs_return) {
		try {
			instance->getOutputDetails(*scales, *zeropoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setHalideScheduler(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:691
	// ("cv::dnn::Net::setHalideScheduler", vec![(pred!(mut, ["scheduler"], ["const cv::String*"]), _)]),
	void cv_dnn_Net_setHalideScheduler_const_StringR(cv::dnn::Net* instance, const char* scheduler, ResultVoid* ocvrs_return) {
		try {
			instance->setHalideScheduler(std::string(scheduler));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPreferableBackend(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:698
	// ("cv::dnn::Net::setPreferableBackend", vec![(pred!(mut, ["backendId"], ["int"]), _)]),
	void cv_dnn_Net_setPreferableBackend_int(cv::dnn::Net* instance, int backendId, ResultVoid* ocvrs_return) {
		try {
			instance->setPreferableBackend(backendId);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPreferableTarget(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:717
	// ("cv::dnn::Net::setPreferableTarget", vec![(pred!(mut, ["targetId"], ["int"]), _)]),
	void cv_dnn_Net_setPreferableTarget_int(cv::dnn::Net* instance, int targetId, ResultVoid* ocvrs_return) {
		try {
			instance->setPreferableTarget(targetId);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInput(InputArray, const String &, double, const Scalar &)(InputArray, InString, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:730
	// ("cv::dnn::Net::setInput", vec![(pred!(mut, ["blob", "name", "scalefactor", "mean"], ["const cv::_InputArray*", "const cv::String*", "double", "const cv::Scalar*"]), _)]),
	void cv_dnn_Net_setInput_const__InputArrayR_const_StringR_double_const_ScalarR(cv::dnn::Net* instance, const cv::_InputArray* blob, const char* name, double scalefactor, const cv::Scalar* mean, ResultVoid* ocvrs_return) {
		try {
			instance->setInput(*blob, std::string(name), scalefactor, *mean);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::Net::setInput(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:730
	// ("cv::dnn::Net::setInput", vec![(pred!(mut, ["blob"], ["const cv::_InputArray*"]), _)]),
	void cv_dnn_Net_setInput_const__InputArrayR(cv::dnn::Net* instance, const cv::_InputArray* blob, ResultVoid* ocvrs_return) {
		try {
			instance->setInput(*blob);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setParam(int, int, const Mat &)(Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:741
	// ("cv::dnn::Net::setParam", vec![(pred!(mut, ["layer", "numParam", "blob"], ["int", "int", "const cv::Mat*"]), _)]),
	void cv_dnn_Net_setParam_int_int_const_MatR(cv::dnn::Net* instance, int layer, int numParam, const cv::Mat* blob, ResultVoid* ocvrs_return) {
		try {
			instance->setParam(layer, numParam, *blob);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setParam(const String &, int, const Mat &)(InString, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:742
	// ("cv::dnn::Net::setParam", vec![(pred!(mut, ["layerName", "numParam", "blob"], ["const cv::String*", "int", "const cv::Mat*"]), _)]),
	void cv_dnn_Net_setParam_const_StringR_int_const_MatR(cv::dnn::Net* instance, const char* layerName, int numParam, const cv::Mat* blob, ResultVoid* ocvrs_return) {
		try {
			instance->setParam(std::string(layerName), numParam, *blob);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getParam(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:749
	// ("cv::dnn::Net::getParam", vec![(pred!(const, ["layer", "numParam"], ["int", "int"]), _)]),
	void cv_dnn_Net_getParam_const_int_int(const cv::dnn::Net* instance, int layer, int numParam, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getParam(layer, numParam);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::Net::getParam(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:749
	// ("cv::dnn::Net::getParam", vec![(pred!(const, ["layer"], ["int"]), _)]),
	void cv_dnn_Net_getParam_const_int(const cv::dnn::Net* instance, int layer, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getParam(layer);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getParam(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:750
	// ("cv::dnn::Net::getParam", vec![(pred!(const, ["layerName", "numParam"], ["const cv::String*", "int"]), _)]),
	void cv_dnn_Net_getParam_const_const_StringR_int(const cv::dnn::Net* instance, const char* layerName, int numParam, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getParam(std::string(layerName), numParam);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::Net::getParam(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:750
	// ("cv::dnn::Net::getParam", vec![(pred!(const, ["layerName"], ["const cv::String*"]), _)]),
	void cv_dnn_Net_getParam_const_const_StringR(const cv::dnn::Net* instance, const char* layerName, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getParam(std::string(layerName));
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUnconnectedOutLayers()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:756
	// ("cv::dnn::Net::getUnconnectedOutLayers", vec![(pred!(const, [], []), _)]),
	void cv_dnn_Net_getUnconnectedOutLayers_const(const cv::dnn::Net* instance, Result<std::vector<int>*>* ocvrs_return) {
		try {
			std::vector<int> ret = instance->getUnconnectedOutLayers();
			Ok(new std::vector<int>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUnconnectedOutLayersNames()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:762
	// ("cv::dnn::Net::getUnconnectedOutLayersNames", vec![(pred!(const, [], []), _)]),
	void cv_dnn_Net_getUnconnectedOutLayersNames_const(const cv::dnn::Net* instance, Result<std::vector<cv::String>*>* ocvrs_return) {
		try {
			std::vector<cv::String> ret = instance->getUnconnectedOutLayersNames();
			Ok(new std::vector<cv::String>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLayersShapes(const std::vector<MatShape> &, std::vector<int> &, std::vector<std::vector<MatShape>> &, std::vector<std::vector<MatShape>> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:773
	// ("cv::dnn::Net::getLayersShapes", vec![(pred!(const, ["netInputShapes", "layersIds", "inLayersShapes", "outLayersShapes"], ["const std::vector<cv::dnn::MatShape>*", "std::vector<int>*", "std::vector<std::vector<cv::dnn::MatShape>>*", "std::vector<std::vector<cv::dnn::MatShape>>*"]), _)]),
	void cv_dnn_Net_getLayersShapes_const_const_vectorLMatShapeGR_vectorLintGR_vectorLvectorLMatShapeGGR_vectorLvectorLMatShapeGGR(const cv::dnn::Net* instance, const std::vector<cv::dnn::MatShape>* netInputShapes, std::vector<int>* layersIds, std::vector<std::vector<cv::dnn::MatShape>>* inLayersShapes, std::vector<std::vector<cv::dnn::MatShape>>* outLayersShapes, ResultVoid* ocvrs_return) {
		try {
			instance->getLayersShapes(*netInputShapes, *layersIds, *inLayersShapes, *outLayersShapes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLayersShapes(const MatShape &, std::vector<int> &, std::vector<std::vector<MatShape>> &, std::vector<std::vector<MatShape>> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:779
	// ("cv::dnn::Net::getLayersShapes", vec![(pred!(const, ["netInputShape", "layersIds", "inLayersShapes", "outLayersShapes"], ["const cv::dnn::MatShape*", "std::vector<int>*", "std::vector<std::vector<cv::dnn::MatShape>>*", "std::vector<std::vector<cv::dnn::MatShape>>*"]), _)]),
	void cv_dnn_Net_getLayersShapes_const_const_MatShapeR_vectorLintGR_vectorLvectorLMatShapeGGR_vectorLvectorLMatShapeGGR(const cv::dnn::Net* instance, const cv::dnn::MatShape* netInputShape, std::vector<int>* layersIds, std::vector<std::vector<cv::dnn::MatShape>>* inLayersShapes, std::vector<std::vector<cv::dnn::MatShape>>* outLayersShapes, ResultVoid* ocvrs_return) {
		try {
			instance->getLayersShapes(*netInputShape, *layersIds, *inLayersShapes, *outLayersShapes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLayerShapes(const MatShape &, const int, std::vector<MatShape> &, std::vector<MatShape> &)(CppPassByVoidPtr, Primitive, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:793
	// ("cv::dnn::Net::getLayerShapes", vec![(pred!(const, ["netInputShape", "layerId", "inLayerShapes", "outLayerShapes"], ["const cv::dnn::MatShape*", "const int", "std::vector<cv::dnn::MatShape>*", "std::vector<cv::dnn::MatShape>*"]), _)]),
	void cv_dnn_Net_getLayerShapes_const_const_MatShapeR_const_int_vectorLMatShapeGR_vectorLMatShapeGR(const cv::dnn::Net* instance, const cv::dnn::MatShape* netInputShape, const int layerId, std::vector<cv::dnn::MatShape>* inLayerShapes, std::vector<cv::dnn::MatShape>* outLayerShapes, ResultVoid* ocvrs_return) {
		try {
			instance->getLayerShapes(*netInputShape, layerId, *inLayerShapes, *outLayerShapes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLayerShapes(const std::vector<MatShape> &, const int, std::vector<MatShape> &, std::vector<MatShape> &)(CppPassByVoidPtr, Primitive, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:799
	// ("cv::dnn::Net::getLayerShapes", vec![(pred!(const, ["netInputShapes", "layerId", "inLayerShapes", "outLayerShapes"], ["const std::vector<cv::dnn::MatShape>*", "const int", "std::vector<cv::dnn::MatShape>*", "std::vector<cv::dnn::MatShape>*"]), _)]),
	void cv_dnn_Net_getLayerShapes_const_const_vectorLMatShapeGR_const_int_vectorLMatShapeGR_vectorLMatShapeGR(const cv::dnn::Net* instance, const std::vector<cv::dnn::MatShape>* netInputShapes, const int layerId, std::vector<cv::dnn::MatShape>* inLayerShapes, std::vector<cv::dnn::MatShape>* outLayerShapes, ResultVoid* ocvrs_return) {
		try {
			instance->getLayerShapes(*netInputShapes, layerId, *inLayerShapes, *outLayerShapes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFLOPS(const std::vector<MatShape> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:808
	// ("cv::dnn::Net::getFLOPS", vec![(pred!(const, ["netInputShapes"], ["const std::vector<cv::dnn::MatShape>*"]), _)]),
	void cv_dnn_Net_getFLOPS_const_const_vectorLMatShapeGR(const cv::dnn::Net* instance, const std::vector<cv::dnn::MatShape>* netInputShapes, Result<int64_t>* ocvrs_return) {
		try {
			int64_t ret = instance->getFLOPS(*netInputShapes);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFLOPS(const MatShape &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:810
	// ("cv::dnn::Net::getFLOPS", vec![(pred!(const, ["netInputShape"], ["const cv::dnn::MatShape*"]), _)]),
	void cv_dnn_Net_getFLOPS_const_const_MatShapeR(const cv::dnn::Net* instance, const cv::dnn::MatShape* netInputShape, Result<int64_t>* ocvrs_return) {
		try {
			int64_t ret = instance->getFLOPS(*netInputShape);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFLOPS(const int, const std::vector<MatShape> &)(Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:812
	// ("cv::dnn::Net::getFLOPS", vec![(pred!(const, ["layerId", "netInputShapes"], ["const int", "const std::vector<cv::dnn::MatShape>*"]), _)]),
	void cv_dnn_Net_getFLOPS_const_const_int_const_vectorLMatShapeGR(const cv::dnn::Net* instance, const int layerId, const std::vector<cv::dnn::MatShape>* netInputShapes, Result<int64_t>* ocvrs_return) {
		try {
			int64_t ret = instance->getFLOPS(layerId, *netInputShapes);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFLOPS(const int, const MatShape &)(Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:815
	// ("cv::dnn::Net::getFLOPS", vec![(pred!(const, ["layerId", "netInputShape"], ["const int", "const cv::dnn::MatShape*"]), _)]),
	void cv_dnn_Net_getFLOPS_const_const_int_const_MatShapeR(const cv::dnn::Net* instance, const int layerId, const cv::dnn::MatShape* netInputShape, Result<int64_t>* ocvrs_return) {
		try {
			int64_t ret = instance->getFLOPS(layerId, *netInputShape);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLayerTypes(std::vector<String> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:821
	// ("cv::dnn::Net::getLayerTypes", vec![(pred!(const, ["layersTypes"], ["std::vector<cv::String>*"]), _)]),
	void cv_dnn_Net_getLayerTypes_const_vectorLStringGR(const cv::dnn::Net* instance, std::vector<cv::String>* layersTypes, ResultVoid* ocvrs_return) {
		try {
			instance->getLayerTypes(*layersTypes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLayersCount(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:827
	// ("cv::dnn::Net::getLayersCount", vec![(pred!(const, ["layerType"], ["const cv::String*"]), _)]),
	void cv_dnn_Net_getLayersCount_const_const_StringR(const cv::dnn::Net* instance, const char* layerType, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getLayersCount(std::string(layerType));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMemoryConsumption(const std::vector<MatShape> &, size_t &, size_t &)(CppPassByVoidPtr, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:835
	// ("cv::dnn::Net::getMemoryConsumption", vec![(pred!(const, ["netInputShapes", "weights", "blobs"], ["const std::vector<cv::dnn::MatShape>*", "size_t*", "size_t*"]), _)]),
	void cv_dnn_Net_getMemoryConsumption_const_const_vectorLMatShapeGR_size_tR_size_tR(const cv::dnn::Net* instance, const std::vector<cv::dnn::MatShape>* netInputShapes, size_t* weights, size_t* blobs, ResultVoid* ocvrs_return) {
		try {
			instance->getMemoryConsumption(*netInputShapes, *weights, *blobs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMemoryConsumption(const MatShape &, size_t &, size_t &)(CppPassByVoidPtr, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:838
	// ("cv::dnn::Net::getMemoryConsumption", vec![(pred!(const, ["netInputShape", "weights", "blobs"], ["const cv::dnn::MatShape*", "size_t*", "size_t*"]), _)]),
	void cv_dnn_Net_getMemoryConsumption_const_const_MatShapeR_size_tR_size_tR(const cv::dnn::Net* instance, const cv::dnn::MatShape* netInputShape, size_t* weights, size_t* blobs, ResultVoid* ocvrs_return) {
		try {
			instance->getMemoryConsumption(*netInputShape, *weights, *blobs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMemoryConsumption(const int, const std::vector<MatShape> &, size_t &, size_t &)(Primitive, CppPassByVoidPtr, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:841
	// ("cv::dnn::Net::getMemoryConsumption", vec![(pred!(const, ["layerId", "netInputShapes", "weights", "blobs"], ["const int", "const std::vector<cv::dnn::MatShape>*", "size_t*", "size_t*"]), _)]),
	void cv_dnn_Net_getMemoryConsumption_const_const_int_const_vectorLMatShapeGR_size_tR_size_tR(const cv::dnn::Net* instance, const int layerId, const std::vector<cv::dnn::MatShape>* netInputShapes, size_t* weights, size_t* blobs, ResultVoid* ocvrs_return) {
		try {
			instance->getMemoryConsumption(layerId, *netInputShapes, *weights, *blobs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMemoryConsumption(const int, const MatShape &, size_t &, size_t &)(Primitive, CppPassByVoidPtr, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:845
	// ("cv::dnn::Net::getMemoryConsumption", vec![(pred!(const, ["layerId", "netInputShape", "weights", "blobs"], ["const int", "const cv::dnn::MatShape*", "size_t*", "size_t*"]), _)]),
	void cv_dnn_Net_getMemoryConsumption_const_const_int_const_MatShapeR_size_tR_size_tR(const cv::dnn::Net* instance, const int layerId, const cv::dnn::MatShape* netInputShape, size_t* weights, size_t* blobs, ResultVoid* ocvrs_return) {
		try {
			instance->getMemoryConsumption(layerId, *netInputShape, *weights, *blobs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMemoryConsumption(const std::vector<MatShape> &, std::vector<int> &, std::vector<size_t> &, std::vector<size_t> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:856
	// ("cv::dnn::Net::getMemoryConsumption", vec![(pred!(const, ["netInputShapes", "layerIds", "weights", "blobs"], ["const std::vector<cv::dnn::MatShape>*", "std::vector<int>*", "std::vector<size_t>*", "std::vector<size_t>*"]), _)]),
	void cv_dnn_Net_getMemoryConsumption_const_const_vectorLMatShapeGR_vectorLintGR_vectorLsize_tGR_vectorLsize_tGR(const cv::dnn::Net* instance, const std::vector<cv::dnn::MatShape>* netInputShapes, std::vector<int>* layerIds, std::vector<size_t>* weights, std::vector<size_t>* blobs, ResultVoid* ocvrs_return) {
		try {
			instance->getMemoryConsumption(*netInputShapes, *layerIds, *weights, *blobs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMemoryConsumption(const MatShape &, std::vector<int> &, std::vector<size_t> &, std::vector<size_t> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:861
	// ("cv::dnn::Net::getMemoryConsumption", vec![(pred!(const, ["netInputShape", "layerIds", "weights", "blobs"], ["const cv::dnn::MatShape*", "std::vector<int>*", "std::vector<size_t>*", "std::vector<size_t>*"]), _)]),
	void cv_dnn_Net_getMemoryConsumption_const_const_MatShapeR_vectorLintGR_vectorLsize_tGR_vectorLsize_tGR(const cv::dnn::Net* instance, const cv::dnn::MatShape* netInputShape, std::vector<int>* layerIds, std::vector<size_t>* weights, std::vector<size_t>* blobs, ResultVoid* ocvrs_return) {
		try {
			instance->getMemoryConsumption(*netInputShape, *layerIds, *weights, *blobs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// enableFusion(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:869
	// ("cv::dnn::Net::enableFusion", vec![(pred!(mut, ["fusion"], ["bool"]), _)]),
	void cv_dnn_Net_enableFusion_bool(cv::dnn::Net* instance, bool fusion, ResultVoid* ocvrs_return) {
		try {
			instance->enableFusion(fusion);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// enableWinograd(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:875
	// ("cv::dnn::Net::enableWinograd", vec![(pred!(mut, ["useWinograd"], ["bool"]), _)]),
	void cv_dnn_Net_enableWinograd_bool(cv::dnn::Net* instance, bool useWinograd, ResultVoid* ocvrs_return) {
		try {
			instance->enableWinograd(useWinograd);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPerfProfile(std::vector<double> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:885
	// ("cv::dnn::Net::getPerfProfile", vec![(pred!(mut, ["timings"], ["std::vector<double>*"]), _)]),
	void cv_dnn_Net_getPerfProfile_vectorLdoubleGR(cv::dnn::Net* instance, std::vector<double>* timings, Result<int64_t>* ocvrs_return) {
		try {
			int64_t ret = instance->getPerfProfile(*timings);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::Net::implicitClone() generated
	// ("cv::dnn::Net::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::dnn::Net* cv_dnn_Net_implicitClone_const(const cv::dnn::Net* instance) {
			return new cv::dnn::Net(*instance);
	}

	// cv::dnn::Net::delete() generated
	// ("cv::dnn::Net::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_Net_delete(cv::dnn::Net* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1079
	// ("cv::dnn::NormalizeBBoxLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_NormalizeBBoxLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::NormalizeBBoxLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::NormalizeBBoxLayer> ret = cv::dnn::NormalizeBBoxLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::NormalizeBBoxLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::NormalizeBBoxLayer::defaultNew() generated
	// ("cv::dnn::NormalizeBBoxLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::NormalizeBBoxLayer* cv_dnn_NormalizeBBoxLayer_defaultNew_const() {
			cv::dnn::NormalizeBBoxLayer* ret = new cv::dnn::NormalizeBBoxLayer();
			return ret;
	}

	// cv::dnn::NormalizeBBoxLayer::pnorm() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1076
	// ("cv::dnn::NormalizeBBoxLayer::pnorm", vec![(pred!(const, [], []), _)]),
	float cv_dnn_NormalizeBBoxLayer_propPnorm_const(const cv::dnn::NormalizeBBoxLayer* instance) {
			float ret = instance->pnorm;
			return ret;
	}

	// cv::dnn::NormalizeBBoxLayer::setPnorm(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1076
	// ("cv::dnn::NormalizeBBoxLayer::setPnorm", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_NormalizeBBoxLayer_propPnorm_const_float(cv::dnn::NormalizeBBoxLayer* instance, const float val) {
			instance->pnorm = val;
	}

	// cv::dnn::NormalizeBBoxLayer::epsilon() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1076
	// ("cv::dnn::NormalizeBBoxLayer::epsilon", vec![(pred!(const, [], []), _)]),
	float cv_dnn_NormalizeBBoxLayer_propEpsilon_const(const cv::dnn::NormalizeBBoxLayer* instance) {
			float ret = instance->epsilon;
			return ret;
	}

	// cv::dnn::NormalizeBBoxLayer::setEpsilon(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1076
	// ("cv::dnn::NormalizeBBoxLayer::setEpsilon", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_NormalizeBBoxLayer_propEpsilon_const_float(cv::dnn::NormalizeBBoxLayer* instance, const float val) {
			instance->epsilon = val;
	}

	// cv::dnn::NormalizeBBoxLayer::acrossSpatial() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1077
	// ("cv::dnn::NormalizeBBoxLayer::acrossSpatial", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_NormalizeBBoxLayer_propAcrossSpatial_const(const cv::dnn::NormalizeBBoxLayer* instance) {
			bool ret = instance->acrossSpatial;
			return ret;
	}

	// cv::dnn::NormalizeBBoxLayer::setAcrossSpatial(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1077
	// ("cv::dnn::NormalizeBBoxLayer::setAcrossSpatial", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_dnn_NormalizeBBoxLayer_propAcrossSpatial_const_bool(cv::dnn::NormalizeBBoxLayer* instance, const bool val) {
			instance->acrossSpatial = val;
	}

	// cv::dnn::NormalizeBBoxLayer::to_Algorithm() generated
	// ("cv::dnn::NormalizeBBoxLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_NormalizeBBoxLayer_to_Algorithm(cv::dnn::NormalizeBBoxLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::NormalizeBBoxLayer::to_Layer() generated
	// ("cv::dnn::NormalizeBBoxLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_NormalizeBBoxLayer_to_Layer(cv::dnn::NormalizeBBoxLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::NormalizeBBoxLayer::delete() generated
	// ("cv::dnn::NormalizeBBoxLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_NormalizeBBoxLayer_delete(cv::dnn::NormalizeBBoxLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:729
	// ("cv::dnn::NotLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_NotLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::NotLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::NotLayer> ret = cv::dnn::NotLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::NotLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::NotLayer::defaultNew() generated
	// ("cv::dnn::NotLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::NotLayer* cv_dnn_NotLayer_defaultNew_const() {
			cv::dnn::NotLayer* ret = new cv::dnn::NotLayer();
			return ret;
	}

	// cv::dnn::NotLayer::to_ActivationLayer() generated
	// ("cv::dnn::NotLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_NotLayer_to_ActivationLayer(cv::dnn::NotLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::NotLayer::to_Algorithm() generated
	// ("cv::dnn::NotLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_NotLayer_to_Algorithm(cv::dnn::NotLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::NotLayer::to_Layer() generated
	// ("cv::dnn::NotLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_NotLayer_to_Layer(cv::dnn::NotLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::NotLayer::delete() generated
	// ("cv::dnn::NotLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_NotLayer_delete(cv::dnn::NotLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:599
	// ("cv::dnn::PaddingLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_PaddingLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::PaddingLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::PaddingLayer> ret = cv::dnn::PaddingLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::PaddingLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::PaddingLayer::defaultNew() generated
	// ("cv::dnn::PaddingLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::PaddingLayer* cv_dnn_PaddingLayer_defaultNew_const() {
			cv::dnn::PaddingLayer* ret = new cv::dnn::PaddingLayer();
			return ret;
	}

	// cv::dnn::PaddingLayer::to_Algorithm() generated
	// ("cv::dnn::PaddingLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_PaddingLayer_to_Algorithm(cv::dnn::PaddingLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::PaddingLayer::to_Layer() generated
	// ("cv::dnn::PaddingLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_PaddingLayer_to_Layer(cv::dnn::PaddingLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::PaddingLayer::delete() generated
	// ("cv::dnn::PaddingLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_PaddingLayer_delete(cv::dnn::PaddingLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:556
	// ("cv::dnn::PermuteLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_PermuteLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::PermuteLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::PermuteLayer> ret = cv::dnn::PermuteLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::PermuteLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::PermuteLayer::defaultNew() generated
	// ("cv::dnn::PermuteLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::PermuteLayer* cv_dnn_PermuteLayer_defaultNew_const() {
			cv::dnn::PermuteLayer* ret = new cv::dnn::PermuteLayer();
			return ret;
	}

	// cv::dnn::PermuteLayer::to_Algorithm() generated
	// ("cv::dnn::PermuteLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_PermuteLayer_to_Algorithm(cv::dnn::PermuteLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::PermuteLayer::to_Layer() generated
	// ("cv::dnn::PermuteLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_PermuteLayer_to_Layer(cv::dnn::PermuteLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::PermuteLayer::delete() generated
	// ("cv::dnn::PermuteLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_PermuteLayer_delete(cv::dnn::PermuteLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:383
	// ("cv::dnn::PoolingLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_PoolingLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::PoolingLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::PoolingLayer> ret = cv::dnn::PoolingLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::PoolingLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::PoolingLayer::defaultNew() generated
	// ("cv::dnn::PoolingLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::PoolingLayer* cv_dnn_PoolingLayer_defaultNew_const() {
			cv::dnn::PoolingLayer* ret = new cv::dnn::PoolingLayer();
			return ret;
	}

	// cv::dnn::PoolingLayer::type() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:365
	// ("cv::dnn::PoolingLayer::type", vec![(pred!(const, [], []), _)]),
	int cv_dnn_PoolingLayer_propType_const(const cv::dnn::PoolingLayer* instance) {
			int ret = instance->type;
			return ret;
	}

	// cv::dnn::PoolingLayer::setType(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:365
	// ("cv::dnn::PoolingLayer::setType", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_PoolingLayer_propType_const_int(cv::dnn::PoolingLayer* instance, const int val) {
			instance->type = val;
	}

	// cv::dnn::PoolingLayer::kernel_size() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:366
	// ("cv::dnn::PoolingLayer::kernel_size", vec![(pred!(const, [], []), _)]),
	std::vector<size_t>* cv_dnn_PoolingLayer_propKernel_size_const(const cv::dnn::PoolingLayer* instance) {
			std::vector<size_t> ret = instance->kernel_size;
			return new std::vector<size_t>(ret);
	}

	// cv::dnn::PoolingLayer::setKernel_size(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:366
	// ("cv::dnn::PoolingLayer::setKernel_size", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
	void cv_dnn_PoolingLayer_propKernel_size_const_vectorLsize_tG(cv::dnn::PoolingLayer* instance, const std::vector<size_t>* val) {
			instance->kernel_size = *val;
	}

	// cv::dnn::PoolingLayer::strides() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:366
	// ("cv::dnn::PoolingLayer::strides", vec![(pred!(const, [], []), _)]),
	std::vector<size_t>* cv_dnn_PoolingLayer_propStrides_const(const cv::dnn::PoolingLayer* instance) {
			std::vector<size_t> ret = instance->strides;
			return new std::vector<size_t>(ret);
	}

	// cv::dnn::PoolingLayer::setStrides(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:366
	// ("cv::dnn::PoolingLayer::setStrides", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
	void cv_dnn_PoolingLayer_propStrides_const_vectorLsize_tG(cv::dnn::PoolingLayer* instance, const std::vector<size_t>* val) {
			instance->strides = *val;
	}

	// cv::dnn::PoolingLayer::pads_begin() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:367
	// ("cv::dnn::PoolingLayer::pads_begin", vec![(pred!(const, [], []), _)]),
	std::vector<size_t>* cv_dnn_PoolingLayer_propPads_begin_const(const cv::dnn::PoolingLayer* instance) {
			std::vector<size_t> ret = instance->pads_begin;
			return new std::vector<size_t>(ret);
	}

	// cv::dnn::PoolingLayer::setPads_begin(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:367
	// ("cv::dnn::PoolingLayer::setPads_begin", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
	void cv_dnn_PoolingLayer_propPads_begin_const_vectorLsize_tG(cv::dnn::PoolingLayer* instance, const std::vector<size_t>* val) {
			instance->pads_begin = *val;
	}

	// cv::dnn::PoolingLayer::pads_end() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:367
	// ("cv::dnn::PoolingLayer::pads_end", vec![(pred!(const, [], []), _)]),
	std::vector<size_t>* cv_dnn_PoolingLayer_propPads_end_const(const cv::dnn::PoolingLayer* instance) {
			std::vector<size_t> ret = instance->pads_end;
			return new std::vector<size_t>(ret);
	}

	// cv::dnn::PoolingLayer::setPads_end(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:367
	// ("cv::dnn::PoolingLayer::setPads_end", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
	void cv_dnn_PoolingLayer_propPads_end_const_vectorLsize_tG(cv::dnn::PoolingLayer* instance, const std::vector<size_t>* val) {
			instance->pads_end = *val;
	}

	// cv::dnn::PoolingLayer::globalPooling() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:368
	// ("cv::dnn::PoolingLayer::globalPooling", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_PoolingLayer_propGlobalPooling_const(const cv::dnn::PoolingLayer* instance) {
			bool ret = instance->globalPooling;
			return ret;
	}

	// cv::dnn::PoolingLayer::setGlobalPooling(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:368
	// ("cv::dnn::PoolingLayer::setGlobalPooling", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_dnn_PoolingLayer_propGlobalPooling_const_bool(cv::dnn::PoolingLayer* instance, const bool val) {
			instance->globalPooling = val;
	}

	// cv::dnn::PoolingLayer::isGlobalPooling() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:369
	// ("cv::dnn::PoolingLayer::isGlobalPooling", vec![(pred!(const, [], []), _)]),
	std::vector<bool>* cv_dnn_PoolingLayer_propIsGlobalPooling_const(const cv::dnn::PoolingLayer* instance) {
			std::vector<bool> ret = instance->isGlobalPooling;
			return new std::vector<bool>(ret);
	}

	// cv::dnn::PoolingLayer::setIsGlobalPooling(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:369
	// ("cv::dnn::PoolingLayer::setIsGlobalPooling", vec![(pred!(mut, ["val"], ["const std::vector<bool>"]), _)]),
	void cv_dnn_PoolingLayer_propIsGlobalPooling_const_vectorLboolG(cv::dnn::PoolingLayer* instance, const std::vector<bool>* val) {
			instance->isGlobalPooling = *val;
	}

	// cv::dnn::PoolingLayer::computeMaxIdx() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:370
	// ("cv::dnn::PoolingLayer::computeMaxIdx", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_PoolingLayer_propComputeMaxIdx_const(const cv::dnn::PoolingLayer* instance) {
			bool ret = instance->computeMaxIdx;
			return ret;
	}

	// cv::dnn::PoolingLayer::setComputeMaxIdx(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:370
	// ("cv::dnn::PoolingLayer::setComputeMaxIdx", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_dnn_PoolingLayer_propComputeMaxIdx_const_bool(cv::dnn::PoolingLayer* instance, const bool val) {
			instance->computeMaxIdx = val;
	}

	// cv::dnn::PoolingLayer::padMode() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:371
	// ("cv::dnn::PoolingLayer::padMode", vec![(pred!(const, [], []), _)]),
	void* cv_dnn_PoolingLayer_propPadMode_const(const cv::dnn::PoolingLayer* instance) {
			cv::String ret = instance->padMode;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::dnn::PoolingLayer::setPadMode(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:371
	// ("cv::dnn::PoolingLayer::setPadMode", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	void cv_dnn_PoolingLayer_propPadMode_const_String(cv::dnn::PoolingLayer* instance, const char* val) {
			instance->padMode = std::string(val);
	}

	// cv::dnn::PoolingLayer::ceilMode() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:372
	// ("cv::dnn::PoolingLayer::ceilMode", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_PoolingLayer_propCeilMode_const(const cv::dnn::PoolingLayer* instance) {
			bool ret = instance->ceilMode;
			return ret;
	}

	// cv::dnn::PoolingLayer::setCeilMode(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:372
	// ("cv::dnn::PoolingLayer::setCeilMode", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_dnn_PoolingLayer_propCeilMode_const_bool(cv::dnn::PoolingLayer* instance, const bool val) {
			instance->ceilMode = val;
	}

	// cv::dnn::PoolingLayer::avePoolPaddedArea() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:376
	// ("cv::dnn::PoolingLayer::avePoolPaddedArea", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_PoolingLayer_propAvePoolPaddedArea_const(const cv::dnn::PoolingLayer* instance) {
			bool ret = instance->avePoolPaddedArea;
			return ret;
	}

	// cv::dnn::PoolingLayer::setAvePoolPaddedArea(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:376
	// ("cv::dnn::PoolingLayer::setAvePoolPaddedArea", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_dnn_PoolingLayer_propAvePoolPaddedArea_const_bool(cv::dnn::PoolingLayer* instance, const bool val) {
			instance->avePoolPaddedArea = val;
	}

	// cv::dnn::PoolingLayer::pooledSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:378
	// ("cv::dnn::PoolingLayer::pooledSize", vec![(pred!(const, [], []), _)]),
	void cv_dnn_PoolingLayer_propPooledSize_const(const cv::dnn::PoolingLayer* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->pooledSize;
			*ocvrs_return = ret;
	}

	// cv::dnn::PoolingLayer::setPooledSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:378
	// ("cv::dnn::PoolingLayer::setPooledSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_dnn_PoolingLayer_propPooledSize_const_Size(cv::dnn::PoolingLayer* instance, const cv::Size* val) {
			instance->pooledSize = *val;
	}

	// cv::dnn::PoolingLayer::spatialScale() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:379
	// ("cv::dnn::PoolingLayer::spatialScale", vec![(pred!(const, [], []), _)]),
	float cv_dnn_PoolingLayer_propSpatialScale_const(const cv::dnn::PoolingLayer* instance) {
			float ret = instance->spatialScale;
			return ret;
	}

	// cv::dnn::PoolingLayer::setSpatialScale(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:379
	// ("cv::dnn::PoolingLayer::setSpatialScale", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_PoolingLayer_propSpatialScale_const_float(cv::dnn::PoolingLayer* instance, const float val) {
			instance->spatialScale = val;
	}

	// cv::dnn::PoolingLayer::psRoiOutChannels() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:381
	// ("cv::dnn::PoolingLayer::psRoiOutChannels", vec![(pred!(const, [], []), _)]),
	int cv_dnn_PoolingLayer_propPsRoiOutChannels_const(const cv::dnn::PoolingLayer* instance) {
			int ret = instance->psRoiOutChannels;
			return ret;
	}

	// cv::dnn::PoolingLayer::setPsRoiOutChannels(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:381
	// ("cv::dnn::PoolingLayer::setPsRoiOutChannels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_PoolingLayer_propPsRoiOutChannels_const_int(cv::dnn::PoolingLayer* instance, const int val) {
			instance->psRoiOutChannels = val;
	}

	// cv::dnn::PoolingLayer::to_Algorithm() generated
	// ("cv::dnn::PoolingLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_PoolingLayer_to_Algorithm(cv::dnn::PoolingLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::PoolingLayer::to_Layer() generated
	// ("cv::dnn::PoolingLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_PoolingLayer_to_Layer(cv::dnn::PoolingLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::PoolingLayer::delete() generated
	// ("cv::dnn::PoolingLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_PoolingLayer_delete(cv::dnn::PoolingLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:391
	// ("cv::dnn::PoolingLayerInt8::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_PoolingLayerInt8_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::PoolingLayerInt8>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::PoolingLayerInt8> ret = cv::dnn::PoolingLayerInt8::create(*params);
			Ok(new cv::Ptr<cv::dnn::PoolingLayerInt8>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::PoolingLayerInt8::defaultNew() generated
	// ("cv::dnn::PoolingLayerInt8::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::PoolingLayerInt8* cv_dnn_PoolingLayerInt8_defaultNew_const() {
			cv::dnn::PoolingLayerInt8* ret = new cv::dnn::PoolingLayerInt8();
			return ret;
	}

	// cv::dnn::PoolingLayerInt8::input_zp() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:389
	// ("cv::dnn::PoolingLayerInt8::input_zp", vec![(pred!(const, [], []), _)]),
	int cv_dnn_PoolingLayerInt8_propInput_zp_const(const cv::dnn::PoolingLayerInt8* instance) {
			int ret = instance->input_zp;
			return ret;
	}

	// cv::dnn::PoolingLayerInt8::setInput_zp(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:389
	// ("cv::dnn::PoolingLayerInt8::setInput_zp", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_PoolingLayerInt8_propInput_zp_const_int(cv::dnn::PoolingLayerInt8* instance, const int val) {
			instance->input_zp = val;
	}

	// cv::dnn::PoolingLayerInt8::output_zp() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:389
	// ("cv::dnn::PoolingLayerInt8::output_zp", vec![(pred!(const, [], []), _)]),
	int cv_dnn_PoolingLayerInt8_propOutput_zp_const(const cv::dnn::PoolingLayerInt8* instance) {
			int ret = instance->output_zp;
			return ret;
	}

	// cv::dnn::PoolingLayerInt8::setOutput_zp(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:389
	// ("cv::dnn::PoolingLayerInt8::setOutput_zp", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_PoolingLayerInt8_propOutput_zp_const_int(cv::dnn::PoolingLayerInt8* instance, const int val) {
			instance->output_zp = val;
	}

	// cv::dnn::PoolingLayerInt8::input_sc() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:390
	// ("cv::dnn::PoolingLayerInt8::input_sc", vec![(pred!(const, [], []), _)]),
	float cv_dnn_PoolingLayerInt8_propInput_sc_const(const cv::dnn::PoolingLayerInt8* instance) {
			float ret = instance->input_sc;
			return ret;
	}

	// cv::dnn::PoolingLayerInt8::setInput_sc(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:390
	// ("cv::dnn::PoolingLayerInt8::setInput_sc", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_PoolingLayerInt8_propInput_sc_const_float(cv::dnn::PoolingLayerInt8* instance, const float val) {
			instance->input_sc = val;
	}

	// cv::dnn::PoolingLayerInt8::output_sc() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:390
	// ("cv::dnn::PoolingLayerInt8::output_sc", vec![(pred!(const, [], []), _)]),
	float cv_dnn_PoolingLayerInt8_propOutput_sc_const(const cv::dnn::PoolingLayerInt8* instance) {
			float ret = instance->output_sc;
			return ret;
	}

	// cv::dnn::PoolingLayerInt8::setOutput_sc(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:390
	// ("cv::dnn::PoolingLayerInt8::setOutput_sc", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_PoolingLayerInt8_propOutput_sc_const_float(cv::dnn::PoolingLayerInt8* instance, const float val) {
			instance->output_sc = val;
	}

	// cv::dnn::PoolingLayerInt8::to_Algorithm() generated
	// ("cv::dnn::PoolingLayerInt8::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_PoolingLayerInt8_to_Algorithm(cv::dnn::PoolingLayerInt8* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::PoolingLayerInt8::to_Layer() generated
	// ("cv::dnn::PoolingLayerInt8::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_PoolingLayerInt8_to_Layer(cv::dnn::PoolingLayerInt8* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::PoolingLayerInt8::to_PoolingLayer() generated
	// ("cv::dnn::PoolingLayerInt8::to_PoolingLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::PoolingLayer* cv_dnn_PoolingLayerInt8_to_PoolingLayer(cv::dnn::PoolingLayerInt8* instance) {
			return dynamic_cast<cv::dnn::PoolingLayer*>(instance);
	}

	// cv::dnn::PoolingLayerInt8::delete() generated
	// ("cv::dnn::PoolingLayerInt8::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_PoolingLayerInt8_delete(cv::dnn::PoolingLayerInt8* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:685
	// ("cv::dnn::PowerLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_PowerLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::PowerLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::PowerLayer> ret = cv::dnn::PowerLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::PowerLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::PowerLayer::defaultNew() generated
	// ("cv::dnn::PowerLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::PowerLayer* cv_dnn_PowerLayer_defaultNew_const() {
			cv::dnn::PowerLayer* ret = new cv::dnn::PowerLayer();
			return ret;
	}

	// cv::dnn::PowerLayer::power() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:683
	// ("cv::dnn::PowerLayer::power", vec![(pred!(const, [], []), _)]),
	float cv_dnn_PowerLayer_propPower_const(const cv::dnn::PowerLayer* instance) {
			float ret = instance->power;
			return ret;
	}

	// cv::dnn::PowerLayer::setPower(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:683
	// ("cv::dnn::PowerLayer::setPower", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_PowerLayer_propPower_const_float(cv::dnn::PowerLayer* instance, const float val) {
			instance->power = val;
	}

	// cv::dnn::PowerLayer::scale() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:683
	// ("cv::dnn::PowerLayer::scale", vec![(pred!(const, [], []), _)]),
	float cv_dnn_PowerLayer_propScale_const(const cv::dnn::PowerLayer* instance) {
			float ret = instance->scale;
			return ret;
	}

	// cv::dnn::PowerLayer::setScale(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:683
	// ("cv::dnn::PowerLayer::setScale", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_PowerLayer_propScale_const_float(cv::dnn::PowerLayer* instance, const float val) {
			instance->scale = val;
	}

	// cv::dnn::PowerLayer::shift() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:683
	// ("cv::dnn::PowerLayer::shift", vec![(pred!(const, [], []), _)]),
	float cv_dnn_PowerLayer_propShift_const(const cv::dnn::PowerLayer* instance) {
			float ret = instance->shift;
			return ret;
	}

	// cv::dnn::PowerLayer::setShift(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:683
	// ("cv::dnn::PowerLayer::setShift", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_PowerLayer_propShift_const_float(cv::dnn::PowerLayer* instance, const float val) {
			instance->shift = val;
	}

	// cv::dnn::PowerLayer::to_ActivationLayer() generated
	// ("cv::dnn::PowerLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_PowerLayer_to_ActivationLayer(cv::dnn::PowerLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::PowerLayer::to_Algorithm() generated
	// ("cv::dnn::PowerLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_PowerLayer_to_Algorithm(cv::dnn::PowerLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::PowerLayer::to_Layer() generated
	// ("cv::dnn::PowerLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_PowerLayer_to_Layer(cv::dnn::PowerLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::PowerLayer::delete() generated
	// ("cv::dnn::PowerLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_PowerLayer_delete(cv::dnn::PowerLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1017
	// ("cv::dnn::PriorBoxLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_PriorBoxLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::PriorBoxLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::PriorBoxLayer> ret = cv::dnn::PriorBoxLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::PriorBoxLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::PriorBoxLayer::defaultNew() generated
	// ("cv::dnn::PriorBoxLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::PriorBoxLayer* cv_dnn_PriorBoxLayer_defaultNew_const() {
			cv::dnn::PriorBoxLayer* ret = new cv::dnn::PriorBoxLayer();
			return ret;
	}

	// cv::dnn::PriorBoxLayer::to_Algorithm() generated
	// ("cv::dnn::PriorBoxLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_PriorBoxLayer_to_Algorithm(cv::dnn::PriorBoxLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::PriorBoxLayer::to_Layer() generated
	// ("cv::dnn::PriorBoxLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_PriorBoxLayer_to_Layer(cv::dnn::PriorBoxLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::PriorBoxLayer::delete() generated
	// ("cv::dnn::PriorBoxLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_PriorBoxLayer_delete(cv::dnn::PriorBoxLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1107
	// ("cv::dnn::ProposalLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ProposalLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::ProposalLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::ProposalLayer> ret = cv::dnn::ProposalLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::ProposalLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ProposalLayer::defaultNew() generated
	// ("cv::dnn::ProposalLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::ProposalLayer* cv_dnn_ProposalLayer_defaultNew_const() {
			cv::dnn::ProposalLayer* ret = new cv::dnn::ProposalLayer();
			return ret;
	}

	// cv::dnn::ProposalLayer::to_Algorithm() generated
	// ("cv::dnn::ProposalLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_ProposalLayer_to_Algorithm(cv::dnn::ProposalLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::ProposalLayer::to_Layer() generated
	// ("cv::dnn::ProposalLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_ProposalLayer_to_Layer(cv::dnn::ProposalLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::ProposalLayer::delete() generated
	// ("cv::dnn::ProposalLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_ProposalLayer_delete(cv::dnn::ProposalLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:470
	// ("cv::dnn::QuantizeLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_QuantizeLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::QuantizeLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::QuantizeLayer> ret = cv::dnn::QuantizeLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::QuantizeLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::QuantizeLayer::defaultNew() generated
	// ("cv::dnn::QuantizeLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::QuantizeLayer* cv_dnn_QuantizeLayer_defaultNew_const() {
			cv::dnn::QuantizeLayer* ret = new cv::dnn::QuantizeLayer();
			return ret;
	}

	// cv::dnn::QuantizeLayer::scales() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:468
	// ("cv::dnn::QuantizeLayer::scales", vec![(pred!(const, [], []), _)]),
	std::vector<float>* cv_dnn_QuantizeLayer_propScales_const(const cv::dnn::QuantizeLayer* instance) {
			std::vector<float> ret = instance->scales;
			return new std::vector<float>(ret);
	}

	// cv::dnn::QuantizeLayer::setScales(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:468
	// ("cv::dnn::QuantizeLayer::setScales", vec![(pred!(mut, ["val"], ["const std::vector<float>"]), _)]),
	void cv_dnn_QuantizeLayer_propScales_const_vectorLfloatG(cv::dnn::QuantizeLayer* instance, const std::vector<float>* val) {
			instance->scales = *val;
	}

	// cv::dnn::QuantizeLayer::zeropoints() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:469
	// ("cv::dnn::QuantizeLayer::zeropoints", vec![(pred!(const, [], []), _)]),
	std::vector<int>* cv_dnn_QuantizeLayer_propZeropoints_const(const cv::dnn::QuantizeLayer* instance) {
			std::vector<int> ret = instance->zeropoints;
			return new std::vector<int>(ret);
	}

	// cv::dnn::QuantizeLayer::setZeropoints(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:469
	// ("cv::dnn::QuantizeLayer::setZeropoints", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
	void cv_dnn_QuantizeLayer_propZeropoints_const_vectorLintG(cv::dnn::QuantizeLayer* instance, const std::vector<int>* val) {
			instance->zeropoints = *val;
	}

	// cv::dnn::QuantizeLayer::to_Algorithm() generated
	// ("cv::dnn::QuantizeLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_QuantizeLayer_to_Algorithm(cv::dnn::QuantizeLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::QuantizeLayer::to_Layer() generated
	// ("cv::dnn::QuantizeLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_QuantizeLayer_to_Layer(cv::dnn::QuantizeLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::QuantizeLayer::delete() generated
	// ("cv::dnn::QuantizeLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_QuantizeLayer_delete(cv::dnn::QuantizeLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:219
	// ("cv::dnn::RNNLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_RNNLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::RNNLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::RNNLayer> ret = cv::dnn::RNNLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::RNNLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWeights(const Mat &, const Mat &, const Mat &, const Mat &, const Mat &)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:235
	// ("cv::dnn::RNNLayer::setWeights", vec![(pred!(mut, ["Wxh", "bh", "Whh", "Who", "bo"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_dnn_RNNLayer_setWeights_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR(cv::dnn::RNNLayer* instance, const cv::Mat* Wxh, const cv::Mat* bh, const cv::Mat* Whh, const cv::Mat* Who, const cv::Mat* bo, ResultVoid* ocvrs_return) {
		try {
			instance->setWeights(*Wxh, *bh, *Whh, *Who, *bo);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setProduceHiddenOutput(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:240
	// ("cv::dnn::RNNLayer::setProduceHiddenOutput", vec![(pred!(mut, ["produce"], ["bool"]), _)]),
	void cv_dnn_RNNLayer_setProduceHiddenOutput_bool(cv::dnn::RNNLayer* instance, bool produce, ResultVoid* ocvrs_return) {
		try {
			instance->setProduceHiddenOutput(produce);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::RNNLayer::setProduceHiddenOutput() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:240
	// ("cv::dnn::RNNLayer::setProduceHiddenOutput", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_RNNLayer_setProduceHiddenOutput(cv::dnn::RNNLayer* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setProduceHiddenOutput();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::RNNLayer::to_Algorithm() generated
	// ("cv::dnn::RNNLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_RNNLayer_to_Algorithm(cv::dnn::RNNLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::RNNLayer::to_Layer() generated
	// ("cv::dnn::RNNLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_RNNLayer_to_Layer(cv::dnn::RNNLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::RNNLayer::delete() generated
	// ("cv::dnn::RNNLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_RNNLayer_delete(cv::dnn::RNNLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:627
	// ("cv::dnn::ReLU6Layer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ReLU6Layer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::ReLU6Layer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::ReLU6Layer> ret = cv::dnn::ReLU6Layer::create(*params);
			Ok(new cv::Ptr<cv::dnn::ReLU6Layer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ReLU6Layer::defaultNew() generated
	// ("cv::dnn::ReLU6Layer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::ReLU6Layer* cv_dnn_ReLU6Layer_defaultNew_const() {
			cv::dnn::ReLU6Layer* ret = new cv::dnn::ReLU6Layer();
			return ret;
	}

	// cv::dnn::ReLU6Layer::minValue() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:625
	// ("cv::dnn::ReLU6Layer::minValue", vec![(pred!(const, [], []), _)]),
	float cv_dnn_ReLU6Layer_propMinValue_const(const cv::dnn::ReLU6Layer* instance) {
			float ret = instance->minValue;
			return ret;
	}

	// cv::dnn::ReLU6Layer::setMinValue(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:625
	// ("cv::dnn::ReLU6Layer::setMinValue", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_ReLU6Layer_propMinValue_const_float(cv::dnn::ReLU6Layer* instance, const float val) {
			instance->minValue = val;
	}

	// cv::dnn::ReLU6Layer::maxValue() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:625
	// ("cv::dnn::ReLU6Layer::maxValue", vec![(pred!(const, [], []), _)]),
	float cv_dnn_ReLU6Layer_propMaxValue_const(const cv::dnn::ReLU6Layer* instance) {
			float ret = instance->maxValue;
			return ret;
	}

	// cv::dnn::ReLU6Layer::setMaxValue(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:625
	// ("cv::dnn::ReLU6Layer::setMaxValue", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_ReLU6Layer_propMaxValue_const_float(cv::dnn::ReLU6Layer* instance, const float val) {
			instance->maxValue = val;
	}

	// cv::dnn::ReLU6Layer::to_ActivationLayer() generated
	// ("cv::dnn::ReLU6Layer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_ReLU6Layer_to_ActivationLayer(cv::dnn::ReLU6Layer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::ReLU6Layer::to_Algorithm() generated
	// ("cv::dnn::ReLU6Layer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_ReLU6Layer_to_Algorithm(cv::dnn::ReLU6Layer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::ReLU6Layer::to_Layer() generated
	// ("cv::dnn::ReLU6Layer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_ReLU6Layer_to_Layer(cv::dnn::ReLU6Layer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::ReLU6Layer::delete() generated
	// ("cv::dnn::ReLU6Layer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_ReLU6Layer_delete(cv::dnn::ReLU6Layer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:619
	// ("cv::dnn::ReLULayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ReLULayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::ReLULayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::ReLULayer> ret = cv::dnn::ReLULayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::ReLULayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ReLULayer::defaultNew() generated
	// ("cv::dnn::ReLULayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::ReLULayer* cv_dnn_ReLULayer_defaultNew_const() {
			cv::dnn::ReLULayer* ret = new cv::dnn::ReLULayer();
			return ret;
	}

	// cv::dnn::ReLULayer::negativeSlope() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:617
	// ("cv::dnn::ReLULayer::negativeSlope", vec![(pred!(const, [], []), _)]),
	float cv_dnn_ReLULayer_propNegativeSlope_const(const cv::dnn::ReLULayer* instance) {
			float ret = instance->negativeSlope;
			return ret;
	}

	// cv::dnn::ReLULayer::setNegativeSlope(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:617
	// ("cv::dnn::ReLULayer::setNegativeSlope", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_ReLULayer_propNegativeSlope_const_float(cv::dnn::ReLULayer* instance, const float val) {
			instance->negativeSlope = val;
	}

	// cv::dnn::ReLULayer::to_ActivationLayer() generated
	// ("cv::dnn::ReLULayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_ReLULayer_to_ActivationLayer(cv::dnn::ReLULayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::ReLULayer::to_Algorithm() generated
	// ("cv::dnn::ReLULayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_ReLULayer_to_Algorithm(cv::dnn::ReLULayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::ReLULayer::to_Layer() generated
	// ("cv::dnn::ReLULayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_ReLULayer_to_Layer(cv::dnn::ReLULayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::ReLULayer::delete() generated
	// ("cv::dnn::ReLULayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_ReLULayer_delete(cv::dnn::ReLULayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:891
	// ("cv::dnn::ReciprocalLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ReciprocalLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::ReciprocalLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::ReciprocalLayer> ret = cv::dnn::ReciprocalLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::ReciprocalLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ReciprocalLayer::defaultNew() generated
	// ("cv::dnn::ReciprocalLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::ReciprocalLayer* cv_dnn_ReciprocalLayer_defaultNew_const() {
			cv::dnn::ReciprocalLayer* ret = new cv::dnn::ReciprocalLayer();
			return ret;
	}

	// cv::dnn::ReciprocalLayer::to_ActivationLayer() generated
	// ("cv::dnn::ReciprocalLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_ReciprocalLayer_to_ActivationLayer(cv::dnn::ReciprocalLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::ReciprocalLayer::to_Algorithm() generated
	// ("cv::dnn::ReciprocalLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_ReciprocalLayer_to_Algorithm(cv::dnn::ReciprocalLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::ReciprocalLayer::to_Layer() generated
	// ("cv::dnn::ReciprocalLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_ReciprocalLayer_to_Layer(cv::dnn::ReciprocalLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::ReciprocalLayer::delete() generated
	// ("cv::dnn::ReciprocalLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_ReciprocalLayer_delete(cv::dnn::ReciprocalLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:397
	// ("cv::dnn::ReduceLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ReduceLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::ReduceLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::ReduceLayer> ret = cv::dnn::ReduceLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::ReduceLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ReduceLayer::defaultNew() generated
	// ("cv::dnn::ReduceLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::ReduceLayer* cv_dnn_ReduceLayer_defaultNew_const() {
			cv::dnn::ReduceLayer* ret = new cv::dnn::ReduceLayer();
			return ret;
	}

	// cv::dnn::ReduceLayer::to_Algorithm() generated
	// ("cv::dnn::ReduceLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_ReduceLayer_to_Algorithm(cv::dnn::ReduceLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::ReduceLayer::to_Layer() generated
	// ("cv::dnn::ReduceLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_ReduceLayer_to_Layer(cv::dnn::ReduceLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::ReduceLayer::delete() generated
	// ("cv::dnn::ReduceLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_ReduceLayer_delete(cv::dnn::ReduceLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1031
	// ("cv::dnn::RegionLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_RegionLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::RegionLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::RegionLayer> ret = cv::dnn::RegionLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::RegionLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::RegionLayer::defaultNew() generated
	// ("cv::dnn::RegionLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::RegionLayer* cv_dnn_RegionLayer_defaultNew_const() {
			cv::dnn::RegionLayer* ret = new cv::dnn::RegionLayer();
			return ret;
	}

	// cv::dnn::RegionLayer::nmsThreshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1029
	// ("cv::dnn::RegionLayer::nmsThreshold", vec![(pred!(const, [], []), _)]),
	float cv_dnn_RegionLayer_propNmsThreshold_const(const cv::dnn::RegionLayer* instance) {
			float ret = instance->nmsThreshold;
			return ret;
	}

	// cv::dnn::RegionLayer::setNmsThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1029
	// ("cv::dnn::RegionLayer::setNmsThreshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_RegionLayer_propNmsThreshold_const_float(cv::dnn::RegionLayer* instance, const float val) {
			instance->nmsThreshold = val;
	}

	// cv::dnn::RegionLayer::to_Algorithm() generated
	// ("cv::dnn::RegionLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_RegionLayer_to_Algorithm(cv::dnn::RegionLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::RegionLayer::to_Layer() generated
	// ("cv::dnn::RegionLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_RegionLayer_to_Layer(cv::dnn::RegionLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::RegionLayer::delete() generated
	// ("cv::dnn::RegionLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_RegionLayer_delete(cv::dnn::RegionLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1023
	// ("cv::dnn::ReorgLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ReorgLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::ReorgLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::ReorgLayer> ret = cv::dnn::ReorgLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::ReorgLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ReorgLayer::defaultNew() generated
	// ("cv::dnn::ReorgLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::ReorgLayer* cv_dnn_ReorgLayer_defaultNew_const() {
			cv::dnn::ReorgLayer* ret = new cv::dnn::ReorgLayer();
			return ret;
	}

	// cv::dnn::ReorgLayer::to_Algorithm() generated
	// ("cv::dnn::ReorgLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_ReorgLayer_to_Algorithm(cv::dnn::ReorgLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::ReorgLayer::to_Layer() generated
	// ("cv::dnn::ReorgLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_ReorgLayer_to_Layer(cv::dnn::ReorgLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::ReorgLayer::delete() generated
	// ("cv::dnn::ReorgLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_ReorgLayer_delete(cv::dnn::ReorgLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:485
	// ("cv::dnn::RequantizeLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_RequantizeLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::RequantizeLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::RequantizeLayer> ret = cv::dnn::RequantizeLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::RequantizeLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::RequantizeLayer::defaultNew() generated
	// ("cv::dnn::RequantizeLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::RequantizeLayer* cv_dnn_RequantizeLayer_defaultNew_const() {
			cv::dnn::RequantizeLayer* ret = new cv::dnn::RequantizeLayer();
			return ret;
	}

	// cv::dnn::RequantizeLayer::scale() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:484
	// ("cv::dnn::RequantizeLayer::scale", vec![(pred!(const, [], []), _)]),
	float cv_dnn_RequantizeLayer_propScale_const(const cv::dnn::RequantizeLayer* instance) {
			float ret = instance->scale;
			return ret;
	}

	// cv::dnn::RequantizeLayer::setScale(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:484
	// ("cv::dnn::RequantizeLayer::setScale", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_RequantizeLayer_propScale_const_float(cv::dnn::RequantizeLayer* instance, const float val) {
			instance->scale = val;
	}

	// cv::dnn::RequantizeLayer::shift() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:484
	// ("cv::dnn::RequantizeLayer::shift", vec![(pred!(const, [], []), _)]),
	float cv_dnn_RequantizeLayer_propShift_const(const cv::dnn::RequantizeLayer* instance) {
			float ret = instance->shift;
			return ret;
	}

	// cv::dnn::RequantizeLayer::setShift(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:484
	// ("cv::dnn::RequantizeLayer::setShift", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_RequantizeLayer_propShift_const_float(cv::dnn::RequantizeLayer* instance, const float val) {
			instance->shift = val;
	}

	// cv::dnn::RequantizeLayer::to_Algorithm() generated
	// ("cv::dnn::RequantizeLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_RequantizeLayer_to_Algorithm(cv::dnn::RequantizeLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::RequantizeLayer::to_Layer() generated
	// ("cv::dnn::RequantizeLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_RequantizeLayer_to_Layer(cv::dnn::RequantizeLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::RequantizeLayer::delete() generated
	// ("cv::dnn::RequantizeLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_RequantizeLayer_delete(cv::dnn::RequantizeLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:456
	// ("cv::dnn::ReshapeLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ReshapeLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::ReshapeLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::ReshapeLayer> ret = cv::dnn::ReshapeLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::ReshapeLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ReshapeLayer::defaultNew() generated
	// ("cv::dnn::ReshapeLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::ReshapeLayer* cv_dnn_ReshapeLayer_defaultNew_const() {
			cv::dnn::ReshapeLayer* ret = new cv::dnn::ReshapeLayer();
			return ret;
	}

	// cv::dnn::ReshapeLayer::newShapeDesc() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:453
	// ("cv::dnn::ReshapeLayer::newShapeDesc", vec![(pred!(const, [], []), _)]),
	cv::dnn::MatShape* cv_dnn_ReshapeLayer_propNewShapeDesc_const(const cv::dnn::ReshapeLayer* instance) {
			cv::dnn::MatShape ret = instance->newShapeDesc;
			return new cv::dnn::MatShape(ret);
	}

	// cv::dnn::ReshapeLayer::setNewShapeDesc(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:453
	// ("cv::dnn::ReshapeLayer::setNewShapeDesc", vec![(pred!(mut, ["val"], ["const cv::dnn::MatShape"]), _)]),
	void cv_dnn_ReshapeLayer_propNewShapeDesc_const_MatShape(cv::dnn::ReshapeLayer* instance, const cv::dnn::MatShape* val) {
			instance->newShapeDesc = *val;
	}

	// cv::dnn::ReshapeLayer::newShapeRange() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:454
	// ("cv::dnn::ReshapeLayer::newShapeRange", vec![(pred!(const, [], []), _)]),
	cv::Range* cv_dnn_ReshapeLayer_propNewShapeRange_const(const cv::dnn::ReshapeLayer* instance) {
			cv::Range ret = instance->newShapeRange;
			return new cv::Range(ret);
	}

	// cv::dnn::ReshapeLayer::setNewShapeRange(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:454
	// ("cv::dnn::ReshapeLayer::setNewShapeRange", vec![(pred!(mut, ["val"], ["const cv::Range"]), _)]),
	void cv_dnn_ReshapeLayer_propNewShapeRange_const_Range(cv::dnn::ReshapeLayer* instance, const cv::Range* val) {
			instance->newShapeRange = *val;
	}

	// cv::dnn::ReshapeLayer::to_Algorithm() generated
	// ("cv::dnn::ReshapeLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_ReshapeLayer_to_Algorithm(cv::dnn::ReshapeLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::ReshapeLayer::to_Layer() generated
	// ("cv::dnn::ReshapeLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_ReshapeLayer_to_Layer(cv::dnn::ReshapeLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::ReshapeLayer::delete() generated
	// ("cv::dnn::ReshapeLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_ReshapeLayer_delete(cv::dnn::ReshapeLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1090
	// ("cv::dnn::ResizeLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ResizeLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::ResizeLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::ResizeLayer> ret = cv::dnn::ResizeLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::ResizeLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ResizeLayer::defaultNew() generated
	// ("cv::dnn::ResizeLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::ResizeLayer* cv_dnn_ResizeLayer_defaultNew_const() {
			cv::dnn::ResizeLayer* ret = new cv::dnn::ResizeLayer();
			return ret;
	}

	// cv::dnn::ResizeLayer::to_Algorithm() generated
	// ("cv::dnn::ResizeLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_ResizeLayer_to_Algorithm(cv::dnn::ResizeLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::ResizeLayer::to_Layer() generated
	// ("cv::dnn::ResizeLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_ResizeLayer_to_Layer(cv::dnn::ResizeLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::ResizeLayer::delete() generated
	// ("cv::dnn::ResizeLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_ResizeLayer_delete(cv::dnn::ResizeLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:717
	// ("cv::dnn::RoundLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_RoundLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::RoundLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::RoundLayer> ret = cv::dnn::RoundLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::RoundLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::RoundLayer::defaultNew() generated
	// ("cv::dnn::RoundLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::RoundLayer* cv_dnn_RoundLayer_defaultNew_const() {
			cv::dnn::RoundLayer* ret = new cv::dnn::RoundLayer();
			return ret;
	}

	// cv::dnn::RoundLayer::to_ActivationLayer() generated
	// ("cv::dnn::RoundLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_RoundLayer_to_ActivationLayer(cv::dnn::RoundLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::RoundLayer::to_Algorithm() generated
	// ("cv::dnn::RoundLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_RoundLayer_to_Algorithm(cv::dnn::RoundLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::RoundLayer::to_Layer() generated
	// ("cv::dnn::RoundLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_RoundLayer_to_Layer(cv::dnn::RoundLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::RoundLayer::delete() generated
	// ("cv::dnn::RoundLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_RoundLayer_delete(cv::dnn::RoundLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:961
	// ("cv::dnn::ScaleLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ScaleLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::ScaleLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::ScaleLayer> ret = cv::dnn::ScaleLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::ScaleLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ScaleLayer::defaultNew() generated
	// ("cv::dnn::ScaleLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::ScaleLayer* cv_dnn_ScaleLayer_defaultNew_const() {
			cv::dnn::ScaleLayer* ret = new cv::dnn::ScaleLayer();
			return ret;
	}

	// cv::dnn::ScaleLayer::hasBias() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:957
	// ("cv::dnn::ScaleLayer::hasBias", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_ScaleLayer_propHasBias_const(const cv::dnn::ScaleLayer* instance) {
			bool ret = instance->hasBias;
			return ret;
	}

	// cv::dnn::ScaleLayer::setHasBias(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:957
	// ("cv::dnn::ScaleLayer::setHasBias", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_dnn_ScaleLayer_propHasBias_const_bool(cv::dnn::ScaleLayer* instance, const bool val) {
			instance->hasBias = val;
	}

	// cv::dnn::ScaleLayer::axis() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:958
	// ("cv::dnn::ScaleLayer::axis", vec![(pred!(const, [], []), _)]),
	int cv_dnn_ScaleLayer_propAxis_const(const cv::dnn::ScaleLayer* instance) {
			int ret = instance->axis;
			return ret;
	}

	// cv::dnn::ScaleLayer::setAxis(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:958
	// ("cv::dnn::ScaleLayer::setAxis", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_ScaleLayer_propAxis_const_int(cv::dnn::ScaleLayer* instance, const int val) {
			instance->axis = val;
	}

	// cv::dnn::ScaleLayer::mode() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:959
	// ("cv::dnn::ScaleLayer::mode", vec![(pred!(const, [], []), _)]),
	void* cv_dnn_ScaleLayer_propMode_const(const cv::dnn::ScaleLayer* instance) {
			cv::String ret = instance->mode;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::dnn::ScaleLayer::setMode(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:959
	// ("cv::dnn::ScaleLayer::setMode", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	void cv_dnn_ScaleLayer_propMode_const_String(cv::dnn::ScaleLayer* instance, const char* val) {
			instance->mode = std::string(val);
	}

	// cv::dnn::ScaleLayer::to_Algorithm() generated
	// ("cv::dnn::ScaleLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_ScaleLayer_to_Algorithm(cv::dnn::ScaleLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::ScaleLayer::to_Layer() generated
	// ("cv::dnn::ScaleLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_ScaleLayer_to_Layer(cv::dnn::ScaleLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::ScaleLayer::delete() generated
	// ("cv::dnn::ScaleLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_ScaleLayer_delete(cv::dnn::ScaleLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:969
	// ("cv::dnn::ScaleLayerInt8::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ScaleLayerInt8_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::ScaleLayerInt8>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::ScaleLayerInt8> ret = cv::dnn::ScaleLayerInt8::create(*params);
			Ok(new cv::Ptr<cv::dnn::ScaleLayerInt8>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ScaleLayerInt8::defaultNew() generated
	// ("cv::dnn::ScaleLayerInt8::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::ScaleLayerInt8* cv_dnn_ScaleLayerInt8_defaultNew_const() {
			cv::dnn::ScaleLayerInt8* ret = new cv::dnn::ScaleLayerInt8();
			return ret;
	}

	// cv::dnn::ScaleLayerInt8::output_sc() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:967
	// ("cv::dnn::ScaleLayerInt8::output_sc", vec![(pred!(const, [], []), _)]),
	float cv_dnn_ScaleLayerInt8_propOutput_sc_const(const cv::dnn::ScaleLayerInt8* instance) {
			float ret = instance->output_sc;
			return ret;
	}

	// cv::dnn::ScaleLayerInt8::setOutput_sc(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:967
	// ("cv::dnn::ScaleLayerInt8::setOutput_sc", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_ScaleLayerInt8_propOutput_sc_const_float(cv::dnn::ScaleLayerInt8* instance, const float val) {
			instance->output_sc = val;
	}

	// cv::dnn::ScaleLayerInt8::output_zp() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:968
	// ("cv::dnn::ScaleLayerInt8::output_zp", vec![(pred!(const, [], []), _)]),
	int cv_dnn_ScaleLayerInt8_propOutput_zp_const(const cv::dnn::ScaleLayerInt8* instance) {
			int ret = instance->output_zp;
			return ret;
	}

	// cv::dnn::ScaleLayerInt8::setOutput_zp(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:968
	// ("cv::dnn::ScaleLayerInt8::setOutput_zp", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_ScaleLayerInt8_propOutput_zp_const_int(cv::dnn::ScaleLayerInt8* instance, const int val) {
			instance->output_zp = val;
	}

	// cv::dnn::ScaleLayerInt8::to_Algorithm() generated
	// ("cv::dnn::ScaleLayerInt8::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_ScaleLayerInt8_to_Algorithm(cv::dnn::ScaleLayerInt8* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::ScaleLayerInt8::to_Layer() generated
	// ("cv::dnn::ScaleLayerInt8::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_ScaleLayerInt8_to_Layer(cv::dnn::ScaleLayerInt8* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::ScaleLayerInt8::to_ScaleLayer() generated
	// ("cv::dnn::ScaleLayerInt8::to_ScaleLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ScaleLayer* cv_dnn_ScaleLayerInt8_to_ScaleLayer(cv::dnn::ScaleLayerInt8* instance) {
			return dynamic_cast<cv::dnn::ScaleLayer*>(instance);
	}

	// cv::dnn::ScaleLayerInt8::delete() generated
	// ("cv::dnn::ScaleLayerInt8::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_ScaleLayerInt8_delete(cv::dnn::ScaleLayerInt8* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1128
	// ("cv::dnn::ScatterLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ScatterLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::ScatterLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::ScatterLayer> ret = cv::dnn::ScatterLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::ScatterLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ScatterLayer::defaultNew() generated
	// ("cv::dnn::ScatterLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::ScatterLayer* cv_dnn_ScatterLayer_defaultNew_const() {
			cv::dnn::ScatterLayer* ret = new cv::dnn::ScatterLayer();
			return ret;
	}

	// cv::dnn::ScatterLayer::to_Algorithm() generated
	// ("cv::dnn::ScatterLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_ScatterLayer_to_Algorithm(cv::dnn::ScatterLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::ScatterLayer::to_Layer() generated
	// ("cv::dnn::ScatterLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_ScatterLayer_to_Layer(cv::dnn::ScatterLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::ScatterLayer::delete() generated
	// ("cv::dnn::ScatterLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_ScatterLayer_delete(cv::dnn::ScatterLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1134
	// ("cv::dnn::ScatterNDLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ScatterNDLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::ScatterNDLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::ScatterNDLayer> ret = cv::dnn::ScatterNDLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::ScatterNDLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ScatterNDLayer::defaultNew() generated
	// ("cv::dnn::ScatterNDLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::ScatterNDLayer* cv_dnn_ScatterNDLayer_defaultNew_const() {
			cv::dnn::ScatterNDLayer* ret = new cv::dnn::ScatterNDLayer();
			return ret;
	}

	// cv::dnn::ScatterNDLayer::to_Algorithm() generated
	// ("cv::dnn::ScatterNDLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_ScatterNDLayer_to_Algorithm(cv::dnn::ScatterNDLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::ScatterNDLayer::to_Layer() generated
	// ("cv::dnn::ScatterNDLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_ScatterNDLayer_to_Layer(cv::dnn::ScatterNDLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::ScatterNDLayer::delete() generated
	// ("cv::dnn::ScatterNDLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_ScatterNDLayer_delete(cv::dnn::ScatterNDLayer* instance) {
			delete instance;
	}

	// SegmentationModel(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1606
	// ("cv::dnn::SegmentationModel::SegmentationModel", vec![(pred!(mut, ["model", "config"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_dnn_SegmentationModel_SegmentationModel_const_StringR_const_StringR(const char* model, const char* config, Result<cv::dnn::SegmentationModel*>* ocvrs_return) {
		try {
			cv::dnn::SegmentationModel* ret = new cv::dnn::SegmentationModel(std::string(model), std::string(config));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::SegmentationModel::SegmentationModel(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1606
	// ("cv::dnn::SegmentationModel::SegmentationModel", vec![(pred!(mut, ["model"], ["const cv::String*"]), _)]),
	void cv_dnn_SegmentationModel_SegmentationModel_const_StringR(const char* model, Result<cv::dnn::SegmentationModel*>* ocvrs_return) {
		try {
			cv::dnn::SegmentationModel* ret = new cv::dnn::SegmentationModel(std::string(model));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// SegmentationModel(const Net &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1612
	// ("cv::dnn::SegmentationModel::SegmentationModel", vec![(pred!(mut, ["network"], ["const cv::dnn::Net*"]), _)]),
	void cv_dnn_SegmentationModel_SegmentationModel_const_NetR(const cv::dnn::Net* network, Result<cv::dnn::SegmentationModel*>* ocvrs_return) {
		try {
			cv::dnn::SegmentationModel* ret = new cv::dnn::SegmentationModel(*network);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// segment(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1618
	// ("cv::dnn::SegmentationModel::segment", vec![(pred!(mut, ["frame", "mask"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_dnn_SegmentationModel_segment_const__InputArrayR_const__OutputArrayR(cv::dnn::SegmentationModel* instance, const cv::_InputArray* frame, const cv::_OutputArray* mask, ResultVoid* ocvrs_return) {
		try {
			instance->segment(*frame, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::SegmentationModel::implicitClone() generated
	// ("cv::dnn::SegmentationModel::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::dnn::SegmentationModel* cv_dnn_SegmentationModel_implicitClone_const(const cv::dnn::SegmentationModel* instance) {
			return new cv::dnn::SegmentationModel(*instance);
	}

	// cv::dnn::SegmentationModel::to_Model() generated
	// ("cv::dnn::SegmentationModel::to_Model", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Model* cv_dnn_SegmentationModel_to_Model(cv::dnn::SegmentationModel* instance) {
			return dynamic_cast<cv::dnn::Model*>(instance);
	}

	// cv::dnn::SegmentationModel::delete() generated
	// ("cv::dnn::SegmentationModel::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_SegmentationModel_delete(cv::dnn::SegmentationModel* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:845
	// ("cv::dnn::SeluLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_SeluLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::SeluLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::SeluLayer> ret = cv::dnn::SeluLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::SeluLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::SeluLayer::defaultNew() generated
	// ("cv::dnn::SeluLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::SeluLayer* cv_dnn_SeluLayer_defaultNew_const() {
			cv::dnn::SeluLayer* ret = new cv::dnn::SeluLayer();
			return ret;
	}

	// cv::dnn::SeluLayer::alpha() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:842
	// ("cv::dnn::SeluLayer::alpha", vec![(pred!(const, [], []), _)]),
	float cv_dnn_SeluLayer_propAlpha_const(const cv::dnn::SeluLayer* instance) {
			float ret = instance->alpha;
			return ret;
	}

	// cv::dnn::SeluLayer::setAlpha(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:842
	// ("cv::dnn::SeluLayer::setAlpha", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_SeluLayer_propAlpha_const_float(cv::dnn::SeluLayer* instance, const float val) {
			instance->alpha = val;
	}

	// cv::dnn::SeluLayer::gamma() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:843
	// ("cv::dnn::SeluLayer::gamma", vec![(pred!(const, [], []), _)]),
	float cv_dnn_SeluLayer_propGamma_const(const cv::dnn::SeluLayer* instance) {
			float ret = instance->gamma;
			return ret;
	}

	// cv::dnn::SeluLayer::setGamma(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:843
	// ("cv::dnn::SeluLayer::setGamma", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_SeluLayer_propGamma_const_float(cv::dnn::SeluLayer* instance, const float val) {
			instance->gamma = val;
	}

	// cv::dnn::SeluLayer::to_ActivationLayer() generated
	// ("cv::dnn::SeluLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_SeluLayer_to_ActivationLayer(cv::dnn::SeluLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::SeluLayer::to_Algorithm() generated
	// ("cv::dnn::SeluLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_SeluLayer_to_Algorithm(cv::dnn::SeluLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::SeluLayer::to_Layer() generated
	// ("cv::dnn::SeluLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_SeluLayer_to_Layer(cv::dnn::SeluLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::SeluLayer::delete() generated
	// ("cv::dnn::SeluLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_SeluLayer_delete(cv::dnn::SeluLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:975
	// ("cv::dnn::ShiftLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ShiftLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::Layer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::ShiftLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::Layer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ShiftLayer::defaultNew() generated
	// ("cv::dnn::ShiftLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::ShiftLayer* cv_dnn_ShiftLayer_defaultNew_const() {
			cv::dnn::ShiftLayer* ret = new cv::dnn::ShiftLayer();
			return ret;
	}

	// cv::dnn::ShiftLayer::to_Algorithm() generated
	// ("cv::dnn::ShiftLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_ShiftLayer_to_Algorithm(cv::dnn::ShiftLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::ShiftLayer::to_Layer() generated
	// ("cv::dnn::ShiftLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_ShiftLayer_to_Layer(cv::dnn::ShiftLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::ShiftLayer::delete() generated
	// ("cv::dnn::ShiftLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_ShiftLayer_delete(cv::dnn::ShiftLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:981
	// ("cv::dnn::ShiftLayerInt8::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ShiftLayerInt8_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::Layer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::ShiftLayerInt8::create(*params);
			Ok(new cv::Ptr<cv::dnn::Layer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ShiftLayerInt8::defaultNew() generated
	// ("cv::dnn::ShiftLayerInt8::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::ShiftLayerInt8* cv_dnn_ShiftLayerInt8_defaultNew_const() {
			cv::dnn::ShiftLayerInt8* ret = new cv::dnn::ShiftLayerInt8();
			return ret;
	}

	// cv::dnn::ShiftLayerInt8::to_Algorithm() generated
	// ("cv::dnn::ShiftLayerInt8::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_ShiftLayerInt8_to_Algorithm(cv::dnn::ShiftLayerInt8* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::ShiftLayerInt8::to_Layer() generated
	// ("cv::dnn::ShiftLayerInt8::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_ShiftLayerInt8_to_Layer(cv::dnn::ShiftLayerInt8* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::ShiftLayerInt8::delete() generated
	// ("cv::dnn::ShiftLayerInt8::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_ShiftLayerInt8_delete(cv::dnn::ShiftLayerInt8* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:885
	// ("cv::dnn::ShrinkLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ShrinkLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::ShrinkLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::ShrinkLayer> ret = cv::dnn::ShrinkLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::ShrinkLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ShrinkLayer::defaultNew() generated
	// ("cv::dnn::ShrinkLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::ShrinkLayer* cv_dnn_ShrinkLayer_defaultNew_const() {
			cv::dnn::ShrinkLayer* ret = new cv::dnn::ShrinkLayer();
			return ret;
	}

	// cv::dnn::ShrinkLayer::bias() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:883
	// ("cv::dnn::ShrinkLayer::bias", vec![(pred!(const, [], []), _)]),
	float cv_dnn_ShrinkLayer_propBias_const(const cv::dnn::ShrinkLayer* instance) {
			float ret = instance->bias;
			return ret;
	}

	// cv::dnn::ShrinkLayer::setBias(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:883
	// ("cv::dnn::ShrinkLayer::setBias", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_ShrinkLayer_propBias_const_float(cv::dnn::ShrinkLayer* instance, const float val) {
			instance->bias = val;
	}

	// cv::dnn::ShrinkLayer::lambd() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:884
	// ("cv::dnn::ShrinkLayer::lambd", vec![(pred!(const, [], []), _)]),
	float cv_dnn_ShrinkLayer_propLambd_const(const cv::dnn::ShrinkLayer* instance) {
			float ret = instance->lambd;
			return ret;
	}

	// cv::dnn::ShrinkLayer::setLambd(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:884
	// ("cv::dnn::ShrinkLayer::setLambd", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_ShrinkLayer_propLambd_const_float(cv::dnn::ShrinkLayer* instance, const float val) {
			instance->lambd = val;
	}

	// cv::dnn::ShrinkLayer::to_ActivationLayer() generated
	// ("cv::dnn::ShrinkLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_ShrinkLayer_to_ActivationLayer(cv::dnn::ShrinkLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::ShrinkLayer::to_Algorithm() generated
	// ("cv::dnn::ShrinkLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_ShrinkLayer_to_Algorithm(cv::dnn::ShrinkLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::ShrinkLayer::to_Layer() generated
	// ("cv::dnn::ShrinkLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_ShrinkLayer_to_Layer(cv::dnn::ShrinkLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::ShrinkLayer::delete() generated
	// ("cv::dnn::ShrinkLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_ShrinkLayer_delete(cv::dnn::ShrinkLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:571
	// ("cv::dnn::ShuffleChannelLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ShuffleChannelLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::Layer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::ShuffleChannelLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::Layer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ShuffleChannelLayer::defaultNew() generated
	// ("cv::dnn::ShuffleChannelLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::ShuffleChannelLayer* cv_dnn_ShuffleChannelLayer_defaultNew_const() {
			cv::dnn::ShuffleChannelLayer* ret = new cv::dnn::ShuffleChannelLayer();
			return ret;
	}

	// cv::dnn::ShuffleChannelLayer::group() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:573
	// ("cv::dnn::ShuffleChannelLayer::group", vec![(pred!(const, [], []), _)]),
	int cv_dnn_ShuffleChannelLayer_propGroup_const(const cv::dnn::ShuffleChannelLayer* instance) {
			int ret = instance->group;
			return ret;
	}

	// cv::dnn::ShuffleChannelLayer::setGroup(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:573
	// ("cv::dnn::ShuffleChannelLayer::setGroup", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_ShuffleChannelLayer_propGroup_const_int(cv::dnn::ShuffleChannelLayer* instance, const int val) {
			instance->group = val;
	}

	// cv::dnn::ShuffleChannelLayer::to_Algorithm() generated
	// ("cv::dnn::ShuffleChannelLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_ShuffleChannelLayer_to_Algorithm(cv::dnn::ShuffleChannelLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::ShuffleChannelLayer::to_Layer() generated
	// ("cv::dnn::ShuffleChannelLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_ShuffleChannelLayer_to_Layer(cv::dnn::ShuffleChannelLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::ShuffleChannelLayer::delete() generated
	// ("cv::dnn::ShuffleChannelLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_ShuffleChannelLayer_delete(cv::dnn::ShuffleChannelLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:665
	// ("cv::dnn::SigmoidLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_SigmoidLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::SigmoidLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::SigmoidLayer> ret = cv::dnn::SigmoidLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::SigmoidLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::SigmoidLayer::defaultNew() generated
	// ("cv::dnn::SigmoidLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::SigmoidLayer* cv_dnn_SigmoidLayer_defaultNew_const() {
			cv::dnn::SigmoidLayer* ret = new cv::dnn::SigmoidLayer();
			return ret;
	}

	// cv::dnn::SigmoidLayer::to_ActivationLayer() generated
	// ("cv::dnn::SigmoidLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_SigmoidLayer_to_ActivationLayer(cv::dnn::SigmoidLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::SigmoidLayer::to_Algorithm() generated
	// ("cv::dnn::SigmoidLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_SigmoidLayer_to_Algorithm(cv::dnn::SigmoidLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::SigmoidLayer::to_Layer() generated
	// ("cv::dnn::SigmoidLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_SigmoidLayer_to_Layer(cv::dnn::SigmoidLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::SigmoidLayer::delete() generated
	// ("cv::dnn::SigmoidLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_SigmoidLayer_delete(cv::dnn::SigmoidLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:877
	// ("cv::dnn::SignLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_SignLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::SignLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::SignLayer> ret = cv::dnn::SignLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::SignLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::SignLayer::defaultNew() generated
	// ("cv::dnn::SignLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::SignLayer* cv_dnn_SignLayer_defaultNew_const() {
			cv::dnn::SignLayer* ret = new cv::dnn::SignLayer();
			return ret;
	}

	// cv::dnn::SignLayer::to_ActivationLayer() generated
	// ("cv::dnn::SignLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_SignLayer_to_ActivationLayer(cv::dnn::SignLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::SignLayer::to_Algorithm() generated
	// ("cv::dnn::SignLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_SignLayer_to_Algorithm(cv::dnn::SignLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::SignLayer::to_Layer() generated
	// ("cv::dnn::SignLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_SignLayer_to_Layer(cv::dnn::SignLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::SignLayer::delete() generated
	// ("cv::dnn::SignLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_SignLayer_delete(cv::dnn::SignLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:795
	// ("cv::dnn::SinLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_SinLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::SinLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::SinLayer> ret = cv::dnn::SinLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::SinLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::SinLayer::defaultNew() generated
	// ("cv::dnn::SinLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::SinLayer* cv_dnn_SinLayer_defaultNew_const() {
			cv::dnn::SinLayer* ret = new cv::dnn::SinLayer();
			return ret;
	}

	// cv::dnn::SinLayer::to_ActivationLayer() generated
	// ("cv::dnn::SinLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_SinLayer_to_ActivationLayer(cv::dnn::SinLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::SinLayer::to_Algorithm() generated
	// ("cv::dnn::SinLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_SinLayer_to_Algorithm(cv::dnn::SinLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::SinLayer::to_Layer() generated
	// ("cv::dnn::SinLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_SinLayer_to_Layer(cv::dnn::SinLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::SinLayer::delete() generated
	// ("cv::dnn::SinLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_SinLayer_delete(cv::dnn::SinLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:801
	// ("cv::dnn::SinhLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_SinhLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::SinhLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::SinhLayer> ret = cv::dnn::SinhLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::SinhLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::SinhLayer::defaultNew() generated
	// ("cv::dnn::SinhLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::SinhLayer* cv_dnn_SinhLayer_defaultNew_const() {
			cv::dnn::SinhLayer* ret = new cv::dnn::SinhLayer();
			return ret;
	}

	// cv::dnn::SinhLayer::to_ActivationLayer() generated
	// ("cv::dnn::SinhLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_SinhLayer_to_ActivationLayer(cv::dnn::SinhLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::SinhLayer::to_Algorithm() generated
	// ("cv::dnn::SinhLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_SinhLayer_to_Algorithm(cv::dnn::SinhLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::SinhLayer::to_Layer() generated
	// ("cv::dnn::SinhLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_SinhLayer_to_Layer(cv::dnn::SinhLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::SinhLayer::delete() generated
	// ("cv::dnn::SinhLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_SinhLayer_delete(cv::dnn::SinhLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:550
	// ("cv::dnn::SliceLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_SliceLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::SliceLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::SliceLayer> ret = cv::dnn::SliceLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::SliceLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::SliceLayer::defaultNew() generated
	// ("cv::dnn::SliceLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::SliceLayer* cv_dnn_SliceLayer_defaultNew_const() {
			cv::dnn::SliceLayer* ret = new cv::dnn::SliceLayer();
			return ret;
	}

	// cv::dnn::SliceLayer::sliceRanges() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:545
	// ("cv::dnn::SliceLayer::sliceRanges", vec![(pred!(const, [], []), _)]),
	std::vector<std::vector<cv::Range>>* cv_dnn_SliceLayer_propSliceRanges_const(const cv::dnn::SliceLayer* instance) {
			std::vector<std::vector<cv::Range>> ret = instance->sliceRanges;
			return new std::vector<std::vector<cv::Range>>(ret);
	}

	// cv::dnn::SliceLayer::setSliceRanges(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:545
	// ("cv::dnn::SliceLayer::setSliceRanges", vec![(pred!(mut, ["val"], ["const std::vector<std::vector<cv::Range>>"]), _)]),
	void cv_dnn_SliceLayer_propSliceRanges_const_vectorLvectorLRangeGG(cv::dnn::SliceLayer* instance, const std::vector<std::vector<cv::Range>>* val) {
			instance->sliceRanges = *val;
	}

	// cv::dnn::SliceLayer::sliceSteps() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:546
	// ("cv::dnn::SliceLayer::sliceSteps", vec![(pred!(const, [], []), _)]),
	std::vector<std::vector<int>>* cv_dnn_SliceLayer_propSliceSteps_const(const cv::dnn::SliceLayer* instance) {
			std::vector<std::vector<int>> ret = instance->sliceSteps;
			return new std::vector<std::vector<int>>(ret);
	}

	// cv::dnn::SliceLayer::setSliceSteps(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:546
	// ("cv::dnn::SliceLayer::setSliceSteps", vec![(pred!(mut, ["val"], ["const std::vector<std::vector<int>>"]), _)]),
	void cv_dnn_SliceLayer_propSliceSteps_const_vectorLvectorLintGG(cv::dnn::SliceLayer* instance, const std::vector<std::vector<int>>* val) {
			instance->sliceSteps = *val;
	}

	// cv::dnn::SliceLayer::axis() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:547
	// ("cv::dnn::SliceLayer::axis", vec![(pred!(const, [], []), _)]),
	int cv_dnn_SliceLayer_propAxis_const(const cv::dnn::SliceLayer* instance) {
			int ret = instance->axis;
			return ret;
	}

	// cv::dnn::SliceLayer::setAxis(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:547
	// ("cv::dnn::SliceLayer::setAxis", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_SliceLayer_propAxis_const_int(cv::dnn::SliceLayer* instance, const int val) {
			instance->axis = val;
	}

	// cv::dnn::SliceLayer::num_split() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:548
	// ("cv::dnn::SliceLayer::num_split", vec![(pred!(const, [], []), _)]),
	int cv_dnn_SliceLayer_propNum_split_const(const cv::dnn::SliceLayer* instance) {
			int ret = instance->num_split;
			return ret;
	}

	// cv::dnn::SliceLayer::setNum_split(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:548
	// ("cv::dnn::SliceLayer::setNum_split", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_SliceLayer_propNum_split_const_int(cv::dnn::SliceLayer* instance, const int val) {
			instance->num_split = val;
	}

	// cv::dnn::SliceLayer::to_Algorithm() generated
	// ("cv::dnn::SliceLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_SliceLayer_to_Algorithm(cv::dnn::SliceLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::SliceLayer::to_Layer() generated
	// ("cv::dnn::SliceLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_SliceLayer_to_Layer(cv::dnn::SliceLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::SliceLayer::delete() generated
	// ("cv::dnn::SliceLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_SliceLayer_delete(cv::dnn::SliceLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:405
	// ("cv::dnn::SoftmaxLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_SoftmaxLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::SoftmaxLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::SoftmaxLayer> ret = cv::dnn::SoftmaxLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::SoftmaxLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::SoftmaxLayer::defaultNew() generated
	// ("cv::dnn::SoftmaxLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::SoftmaxLayer* cv_dnn_SoftmaxLayer_defaultNew_const() {
			cv::dnn::SoftmaxLayer* ret = new cv::dnn::SoftmaxLayer();
			return ret;
	}

	// cv::dnn::SoftmaxLayer::logSoftMax() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:403
	// ("cv::dnn::SoftmaxLayer::logSoftMax", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_SoftmaxLayer_propLogSoftMax_const(const cv::dnn::SoftmaxLayer* instance) {
			bool ret = instance->logSoftMax;
			return ret;
	}

	// cv::dnn::SoftmaxLayer::setLogSoftMax(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:403
	// ("cv::dnn::SoftmaxLayer::setLogSoftMax", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_dnn_SoftmaxLayer_propLogSoftMax_const_bool(cv::dnn::SoftmaxLayer* instance, const bool val) {
			instance->logSoftMax = val;
	}

	// cv::dnn::SoftmaxLayer::to_Algorithm() generated
	// ("cv::dnn::SoftmaxLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_SoftmaxLayer_to_Algorithm(cv::dnn::SoftmaxLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::SoftmaxLayer::to_Layer() generated
	// ("cv::dnn::SoftmaxLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_SoftmaxLayer_to_Layer(cv::dnn::SoftmaxLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::SoftmaxLayer::delete() generated
	// ("cv::dnn::SoftmaxLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_SoftmaxLayer_delete(cv::dnn::SoftmaxLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:413
	// ("cv::dnn::SoftmaxLayerInt8::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_SoftmaxLayerInt8_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::SoftmaxLayerInt8>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::SoftmaxLayerInt8> ret = cv::dnn::SoftmaxLayerInt8::create(*params);
			Ok(new cv::Ptr<cv::dnn::SoftmaxLayerInt8>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::SoftmaxLayerInt8::defaultNew() generated
	// ("cv::dnn::SoftmaxLayerInt8::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::SoftmaxLayerInt8* cv_dnn_SoftmaxLayerInt8_defaultNew_const() {
			cv::dnn::SoftmaxLayerInt8* ret = new cv::dnn::SoftmaxLayerInt8();
			return ret;
	}

	// cv::dnn::SoftmaxLayerInt8::output_sc() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:411
	// ("cv::dnn::SoftmaxLayerInt8::output_sc", vec![(pred!(const, [], []), _)]),
	float cv_dnn_SoftmaxLayerInt8_propOutput_sc_const(const cv::dnn::SoftmaxLayerInt8* instance) {
			float ret = instance->output_sc;
			return ret;
	}

	// cv::dnn::SoftmaxLayerInt8::setOutput_sc(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:411
	// ("cv::dnn::SoftmaxLayerInt8::setOutput_sc", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_SoftmaxLayerInt8_propOutput_sc_const_float(cv::dnn::SoftmaxLayerInt8* instance, const float val) {
			instance->output_sc = val;
	}

	// cv::dnn::SoftmaxLayerInt8::output_zp() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:412
	// ("cv::dnn::SoftmaxLayerInt8::output_zp", vec![(pred!(const, [], []), _)]),
	int cv_dnn_SoftmaxLayerInt8_propOutput_zp_const(const cv::dnn::SoftmaxLayerInt8* instance) {
			int ret = instance->output_zp;
			return ret;
	}

	// cv::dnn::SoftmaxLayerInt8::setOutput_zp(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:412
	// ("cv::dnn::SoftmaxLayerInt8::setOutput_zp", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_SoftmaxLayerInt8_propOutput_zp_const_int(cv::dnn::SoftmaxLayerInt8* instance, const int val) {
			instance->output_zp = val;
	}

	// cv::dnn::SoftmaxLayerInt8::to_Algorithm() generated
	// ("cv::dnn::SoftmaxLayerInt8::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_SoftmaxLayerInt8_to_Algorithm(cv::dnn::SoftmaxLayerInt8* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::SoftmaxLayerInt8::to_Layer() generated
	// ("cv::dnn::SoftmaxLayerInt8::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_SoftmaxLayerInt8_to_Layer(cv::dnn::SoftmaxLayerInt8* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::SoftmaxLayerInt8::to_SoftmaxLayer() generated
	// ("cv::dnn::SoftmaxLayerInt8::to_SoftmaxLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SoftmaxLayer* cv_dnn_SoftmaxLayerInt8_to_SoftmaxLayer(cv::dnn::SoftmaxLayerInt8* instance) {
			return dynamic_cast<cv::dnn::SoftmaxLayer*>(instance);
	}

	// cv::dnn::SoftmaxLayerInt8::delete() generated
	// ("cv::dnn::SoftmaxLayerInt8::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_SoftmaxLayerInt8_delete(cv::dnn::SoftmaxLayerInt8* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:807
	// ("cv::dnn::SoftplusLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_SoftplusLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::SoftplusLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::SoftplusLayer> ret = cv::dnn::SoftplusLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::SoftplusLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::SoftplusLayer::defaultNew() generated
	// ("cv::dnn::SoftplusLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::SoftplusLayer* cv_dnn_SoftplusLayer_defaultNew_const() {
			cv::dnn::SoftplusLayer* ret = new cv::dnn::SoftplusLayer();
			return ret;
	}

	// cv::dnn::SoftplusLayer::to_ActivationLayer() generated
	// ("cv::dnn::SoftplusLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_SoftplusLayer_to_ActivationLayer(cv::dnn::SoftplusLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::SoftplusLayer::to_Algorithm() generated
	// ("cv::dnn::SoftplusLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_SoftplusLayer_to_Algorithm(cv::dnn::SoftplusLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::SoftplusLayer::to_Layer() generated
	// ("cv::dnn::SoftplusLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_SoftplusLayer_to_Layer(cv::dnn::SoftplusLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::SoftplusLayer::delete() generated
	// ("cv::dnn::SoftplusLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_SoftplusLayer_delete(cv::dnn::SoftplusLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:813
	// ("cv::dnn::SoftsignLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_SoftsignLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::SoftsignLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::SoftsignLayer> ret = cv::dnn::SoftsignLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::SoftsignLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::SoftsignLayer::defaultNew() generated
	// ("cv::dnn::SoftsignLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::SoftsignLayer* cv_dnn_SoftsignLayer_defaultNew_const() {
			cv::dnn::SoftsignLayer* ret = new cv::dnn::SoftsignLayer();
			return ret;
	}

	// cv::dnn::SoftsignLayer::to_ActivationLayer() generated
	// ("cv::dnn::SoftsignLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_SoftsignLayer_to_ActivationLayer(cv::dnn::SoftsignLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::SoftsignLayer::to_Algorithm() generated
	// ("cv::dnn::SoftsignLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_SoftsignLayer_to_Algorithm(cv::dnn::SoftsignLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::SoftsignLayer::to_Layer() generated
	// ("cv::dnn::SoftsignLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_SoftsignLayer_to_Layer(cv::dnn::SoftsignLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::SoftsignLayer::delete() generated
	// ("cv::dnn::SoftsignLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_SoftsignLayer_delete(cv::dnn::SoftsignLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1198
	// ("cv::dnn::SpaceToDepthLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_SpaceToDepthLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::SpaceToDepthLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::SpaceToDepthLayer> ret = cv::dnn::SpaceToDepthLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::SpaceToDepthLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::SpaceToDepthLayer::defaultNew() generated
	// ("cv::dnn::SpaceToDepthLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::SpaceToDepthLayer* cv_dnn_SpaceToDepthLayer_defaultNew_const() {
			cv::dnn::SpaceToDepthLayer* ret = new cv::dnn::SpaceToDepthLayer();
			return ret;
	}

	// cv::dnn::SpaceToDepthLayer::to_Algorithm() generated
	// ("cv::dnn::SpaceToDepthLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_SpaceToDepthLayer_to_Algorithm(cv::dnn::SpaceToDepthLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::SpaceToDepthLayer::to_Layer() generated
	// ("cv::dnn::SpaceToDepthLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_SpaceToDepthLayer_to_Layer(cv::dnn::SpaceToDepthLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::SpaceToDepthLayer::delete() generated
	// ("cv::dnn::SpaceToDepthLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_SpaceToDepthLayer_delete(cv::dnn::SpaceToDepthLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:509
	// ("cv::dnn::SplitLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_SplitLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::SplitLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::SplitLayer> ret = cv::dnn::SplitLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::SplitLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::SplitLayer::defaultNew() generated
	// ("cv::dnn::SplitLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::SplitLayer* cv_dnn_SplitLayer_defaultNew_const() {
			cv::dnn::SplitLayer* ret = new cv::dnn::SplitLayer();
			return ret;
	}

	// cv::dnn::SplitLayer::outputsCount() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:507
	// ("cv::dnn::SplitLayer::outputsCount", vec![(pred!(const, [], []), _)]),
	int cv_dnn_SplitLayer_propOutputsCount_const(const cv::dnn::SplitLayer* instance) {
			int ret = instance->outputsCount;
			return ret;
	}

	// cv::dnn::SplitLayer::setOutputsCount(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:507
	// ("cv::dnn::SplitLayer::setOutputsCount", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_SplitLayer_propOutputsCount_const_int(cv::dnn::SplitLayer* instance, const int val) {
			instance->outputsCount = val;
	}

	// cv::dnn::SplitLayer::to_Algorithm() generated
	// ("cv::dnn::SplitLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_SplitLayer_to_Algorithm(cv::dnn::SplitLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::SplitLayer::to_Layer() generated
	// ("cv::dnn::SplitLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_SplitLayer_to_Layer(cv::dnn::SplitLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::SplitLayer::delete() generated
	// ("cv::dnn::SplitLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_SplitLayer_delete(cv::dnn::SplitLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:723
	// ("cv::dnn::SqrtLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_SqrtLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::SqrtLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::SqrtLayer> ret = cv::dnn::SqrtLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::SqrtLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::SqrtLayer::defaultNew() generated
	// ("cv::dnn::SqrtLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::SqrtLayer* cv_dnn_SqrtLayer_defaultNew_const() {
			cv::dnn::SqrtLayer* ret = new cv::dnn::SqrtLayer();
			return ret;
	}

	// cv::dnn::SqrtLayer::to_ActivationLayer() generated
	// ("cv::dnn::SqrtLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_SqrtLayer_to_ActivationLayer(cv::dnn::SqrtLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::SqrtLayer::to_Algorithm() generated
	// ("cv::dnn::SqrtLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_SqrtLayer_to_Algorithm(cv::dnn::SqrtLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::SqrtLayer::to_Layer() generated
	// ("cv::dnn::SqrtLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_SqrtLayer_to_Layer(cv::dnn::SqrtLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::SqrtLayer::delete() generated
	// ("cv::dnn::SqrtLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_SqrtLayer_delete(cv::dnn::SqrtLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:653
	// ("cv::dnn::SwishLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_SwishLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::SwishLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::SwishLayer> ret = cv::dnn::SwishLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::SwishLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::SwishLayer::defaultNew() generated
	// ("cv::dnn::SwishLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::SwishLayer* cv_dnn_SwishLayer_defaultNew_const() {
			cv::dnn::SwishLayer* ret = new cv::dnn::SwishLayer();
			return ret;
	}

	// cv::dnn::SwishLayer::to_ActivationLayer() generated
	// ("cv::dnn::SwishLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_SwishLayer_to_ActivationLayer(cv::dnn::SwishLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::SwishLayer::to_Algorithm() generated
	// ("cv::dnn::SwishLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_SwishLayer_to_Algorithm(cv::dnn::SwishLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::SwishLayer::to_Layer() generated
	// ("cv::dnn::SwishLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_SwishLayer_to_Layer(cv::dnn::SwishLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::SwishLayer::delete() generated
	// ("cv::dnn::SwishLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_SwishLayer_delete(cv::dnn::SwishLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:647
	// ("cv::dnn::TanHLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_TanHLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::TanHLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::TanHLayer> ret = cv::dnn::TanHLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::TanHLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::TanHLayer::defaultNew() generated
	// ("cv::dnn::TanHLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::TanHLayer* cv_dnn_TanHLayer_defaultNew_const() {
			cv::dnn::TanHLayer* ret = new cv::dnn::TanHLayer();
			return ret;
	}

	// cv::dnn::TanHLayer::to_ActivationLayer() generated
	// ("cv::dnn::TanHLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_TanHLayer_to_ActivationLayer(cv::dnn::TanHLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::TanHLayer::to_Algorithm() generated
	// ("cv::dnn::TanHLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_TanHLayer_to_Algorithm(cv::dnn::TanHLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::TanHLayer::to_Layer() generated
	// ("cv::dnn::TanHLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_TanHLayer_to_Layer(cv::dnn::TanHLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::TanHLayer::delete() generated
	// ("cv::dnn::TanHLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_TanHLayer_delete(cv::dnn::TanHLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:819
	// ("cv::dnn::TanLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_TanLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::TanLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::TanLayer> ret = cv::dnn::TanLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::TanLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::TanLayer::defaultNew() generated
	// ("cv::dnn::TanLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::TanLayer* cv_dnn_TanLayer_defaultNew_const() {
			cv::dnn::TanLayer* ret = new cv::dnn::TanLayer();
			return ret;
	}

	// cv::dnn::TanLayer::to_ActivationLayer() generated
	// ("cv::dnn::TanLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_TanLayer_to_ActivationLayer(cv::dnn::TanLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::TanLayer::to_Algorithm() generated
	// ("cv::dnn::TanLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_TanLayer_to_Algorithm(cv::dnn::TanLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::TanLayer::to_Layer() generated
	// ("cv::dnn::TanLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_TanLayer_to_Layer(cv::dnn::TanLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::TanLayer::delete() generated
	// ("cv::dnn::TanLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_TanLayer_delete(cv::dnn::TanLayer* instance) {
			delete instance;
	}

	// detect(InputArray, std::vector<std::vector<Point>> &, std::vector<float> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1793
	// ("cv::dnn::TextDetectionModel::detect", vec![(pred!(const, ["frame", "detections", "confidences"], ["const cv::_InputArray*", "std::vector<std::vector<cv::Point>>*", "std::vector<float>*"]), _)]),
	void cv_dnn_TextDetectionModel_detect_const_const__InputArrayR_vectorLvectorLPointGGR_vectorLfloatGR(const cv::dnn::TextDetectionModel* instance, const cv::_InputArray* frame, std::vector<std::vector<cv::Point>>* detections, std::vector<float>* confidences, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*frame, *detections, *confidences);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detect(InputArray, std::vector<std::vector<Point>> &)(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1801
	// ("cv::dnn::TextDetectionModel::detect", vec![(pred!(const, ["frame", "detections"], ["const cv::_InputArray*", "std::vector<std::vector<cv::Point>>*"]), _)]),
	void cv_dnn_TextDetectionModel_detect_const_const__InputArrayR_vectorLvectorLPointGGR(const cv::dnn::TextDetectionModel* instance, const cv::_InputArray* frame, std::vector<std::vector<cv::Point>>* detections, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*frame, *detections);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectTextRectangles(InputArray, std::vector<cv::RotatedRect> &, std::vector<float> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1819
	// ("cv::dnn::TextDetectionModel::detectTextRectangles", vec![(pred!(const, ["frame", "detections", "confidences"], ["const cv::_InputArray*", "std::vector<cv::RotatedRect>*", "std::vector<float>*"]), _)]),
	void cv_dnn_TextDetectionModel_detectTextRectangles_const_const__InputArrayR_vectorLRotatedRectGR_vectorLfloatGR(const cv::dnn::TextDetectionModel* instance, const cv::_InputArray* frame, std::vector<cv::RotatedRect>* detections, std::vector<float>* confidences, ResultVoid* ocvrs_return) {
		try {
			instance->detectTextRectangles(*frame, *detections, *confidences);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectTextRectangles(InputArray, std::vector<cv::RotatedRect> &)(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1827
	// ("cv::dnn::TextDetectionModel::detectTextRectangles", vec![(pred!(const, ["frame", "detections"], ["const cv::_InputArray*", "std::vector<cv::RotatedRect>*"]), _)]),
	void cv_dnn_TextDetectionModel_detectTextRectangles_const_const__InputArrayR_vectorLRotatedRectGR(const cv::dnn::TextDetectionModel* instance, const cv::_InputArray* frame, std::vector<cv::RotatedRect>* detections, ResultVoid* ocvrs_return) {
		try {
			instance->detectTextRectangles(*frame, *detections);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::TextDetectionModel::to_Model() generated
	// ("cv::dnn::TextDetectionModel::to_Model", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Model* cv_dnn_TextDetectionModel_to_Model(cv::dnn::TextDetectionModel* instance) {
			return dynamic_cast<cv::dnn::Model*>(instance);
	}

	// cv::dnn::TextDetectionModel::delete() generated
	// ("cv::dnn::TextDetectionModel::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_TextDetectionModel_delete(cv::dnn::TextDetectionModel* instance) {
			delete instance;
	}

	// TextDetectionModel_DB()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1904
	// ("cv::dnn::TextDetectionModel_DB::TextDetectionModel_DB", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_TextDetectionModel_DB_TextDetectionModel_DB(Result<cv::dnn::TextDetectionModel_DB*>* ocvrs_return) {
		try {
			cv::dnn::TextDetectionModel_DB* ret = new cv::dnn::TextDetectionModel_DB();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// TextDetectionModel_DB(const Net &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1910
	// ("cv::dnn::TextDetectionModel_DB::TextDetectionModel_DB", vec![(pred!(mut, ["network"], ["const cv::dnn::Net*"]), _)]),
	void cv_dnn_TextDetectionModel_DB_TextDetectionModel_DB_const_NetR(const cv::dnn::Net* network, Result<cv::dnn::TextDetectionModel_DB*>* ocvrs_return) {
		try {
			cv::dnn::TextDetectionModel_DB* ret = new cv::dnn::TextDetectionModel_DB(*network);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// TextDetectionModel_DB(const std::string &, const std::string &)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1919
	// ("cv::dnn::TextDetectionModel_DB::TextDetectionModel_DB", vec![(pred!(mut, ["model", "config"], ["const std::string*", "const std::string*"]), _)]),
	void cv_dnn_TextDetectionModel_DB_TextDetectionModel_DB_const_stringR_const_stringR(const char* model, const char* config, Result<cv::dnn::TextDetectionModel_DB*>* ocvrs_return) {
		try {
			cv::dnn::TextDetectionModel_DB* ret = new cv::dnn::TextDetectionModel_DB(std::string(model), std::string(config));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::TextDetectionModel_DB::TextDetectionModel_DB(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1919
	// ("cv::dnn::TextDetectionModel_DB::TextDetectionModel_DB", vec![(pred!(mut, ["model"], ["const std::string*"]), _)]),
	void cv_dnn_TextDetectionModel_DB_TextDetectionModel_DB_const_stringR(const char* model, Result<cv::dnn::TextDetectionModel_DB*>* ocvrs_return) {
		try {
			cv::dnn::TextDetectionModel_DB* ret = new cv::dnn::TextDetectionModel_DB(std::string(model));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBinaryThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1922
	// ("cv::dnn::TextDetectionModel_DB::setBinaryThreshold", vec![(pred!(mut, ["binaryThreshold"], ["float"]), _)]),
	void cv_dnn_TextDetectionModel_DB_setBinaryThreshold_float(cv::dnn::TextDetectionModel_DB* instance, float binaryThreshold, Result<cv::dnn::TextDetectionModel_DB*>* ocvrs_return) {
		try {
			cv::dnn::TextDetectionModel_DB ret = instance->setBinaryThreshold(binaryThreshold);
			Ok(new cv::dnn::TextDetectionModel_DB(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBinaryThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1923
	// ("cv::dnn::TextDetectionModel_DB::getBinaryThreshold", vec![(pred!(const, [], []), _)]),
	void cv_dnn_TextDetectionModel_DB_getBinaryThreshold_const(const cv::dnn::TextDetectionModel_DB* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getBinaryThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPolygonThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1925
	// ("cv::dnn::TextDetectionModel_DB::setPolygonThreshold", vec![(pred!(mut, ["polygonThreshold"], ["float"]), _)]),
	void cv_dnn_TextDetectionModel_DB_setPolygonThreshold_float(cv::dnn::TextDetectionModel_DB* instance, float polygonThreshold, Result<cv::dnn::TextDetectionModel_DB*>* ocvrs_return) {
		try {
			cv::dnn::TextDetectionModel_DB ret = instance->setPolygonThreshold(polygonThreshold);
			Ok(new cv::dnn::TextDetectionModel_DB(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPolygonThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1926
	// ("cv::dnn::TextDetectionModel_DB::getPolygonThreshold", vec![(pred!(const, [], []), _)]),
	void cv_dnn_TextDetectionModel_DB_getPolygonThreshold_const(const cv::dnn::TextDetectionModel_DB* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getPolygonThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUnclipRatio(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1928
	// ("cv::dnn::TextDetectionModel_DB::setUnclipRatio", vec![(pred!(mut, ["unclipRatio"], ["double"]), _)]),
	void cv_dnn_TextDetectionModel_DB_setUnclipRatio_double(cv::dnn::TextDetectionModel_DB* instance, double unclipRatio, Result<cv::dnn::TextDetectionModel_DB*>* ocvrs_return) {
		try {
			cv::dnn::TextDetectionModel_DB ret = instance->setUnclipRatio(unclipRatio);
			Ok(new cv::dnn::TextDetectionModel_DB(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUnclipRatio()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1929
	// ("cv::dnn::TextDetectionModel_DB::getUnclipRatio", vec![(pred!(const, [], []), _)]),
	void cv_dnn_TextDetectionModel_DB_getUnclipRatio_const(const cv::dnn::TextDetectionModel_DB* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getUnclipRatio();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxCandidates(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1931
	// ("cv::dnn::TextDetectionModel_DB::setMaxCandidates", vec![(pred!(mut, ["maxCandidates"], ["int"]), _)]),
	void cv_dnn_TextDetectionModel_DB_setMaxCandidates_int(cv::dnn::TextDetectionModel_DB* instance, int maxCandidates, Result<cv::dnn::TextDetectionModel_DB*>* ocvrs_return) {
		try {
			cv::dnn::TextDetectionModel_DB ret = instance->setMaxCandidates(maxCandidates);
			Ok(new cv::dnn::TextDetectionModel_DB(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxCandidates()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1932
	// ("cv::dnn::TextDetectionModel_DB::getMaxCandidates", vec![(pred!(const, [], []), _)]),
	void cv_dnn_TextDetectionModel_DB_getMaxCandidates_const(const cv::dnn::TextDetectionModel_DB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxCandidates();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::TextDetectionModel_DB::implicitClone() generated
	// ("cv::dnn::TextDetectionModel_DB::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::dnn::TextDetectionModel_DB* cv_dnn_TextDetectionModel_DB_implicitClone_const(const cv::dnn::TextDetectionModel_DB* instance) {
			return new cv::dnn::TextDetectionModel_DB(*instance);
	}

	// cv::dnn::TextDetectionModel_DB::to_Model() generated
	// ("cv::dnn::TextDetectionModel_DB::to_Model", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Model* cv_dnn_TextDetectionModel_DB_to_Model(cv::dnn::TextDetectionModel_DB* instance) {
			return dynamic_cast<cv::dnn::Model*>(instance);
	}

	// cv::dnn::TextDetectionModel_DB::to_TextDetectionModel() generated
	// ("cv::dnn::TextDetectionModel_DB::to_TextDetectionModel", vec![(pred!(mut, [], []), _)]),
	cv::dnn::TextDetectionModel* cv_dnn_TextDetectionModel_DB_to_TextDetectionModel(cv::dnn::TextDetectionModel_DB* instance) {
			return dynamic_cast<cv::dnn::TextDetectionModel*>(instance);
	}

	// cv::dnn::TextDetectionModel_DB::delete() generated
	// ("cv::dnn::TextDetectionModel_DB::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_TextDetectionModel_DB_delete(cv::dnn::TextDetectionModel_DB* instance) {
			delete instance;
	}

	// TextDetectionModel_EAST()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1843
	// ("cv::dnn::TextDetectionModel_EAST::TextDetectionModel_EAST", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_TextDetectionModel_EAST_TextDetectionModel_EAST(Result<cv::dnn::TextDetectionModel_EAST*>* ocvrs_return) {
		try {
			cv::dnn::TextDetectionModel_EAST* ret = new cv::dnn::TextDetectionModel_EAST();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// TextDetectionModel_EAST(const Net &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1849
	// ("cv::dnn::TextDetectionModel_EAST::TextDetectionModel_EAST", vec![(pred!(mut, ["network"], ["const cv::dnn::Net*"]), _)]),
	void cv_dnn_TextDetectionModel_EAST_TextDetectionModel_EAST_const_NetR(const cv::dnn::Net* network, Result<cv::dnn::TextDetectionModel_EAST*>* ocvrs_return) {
		try {
			cv::dnn::TextDetectionModel_EAST* ret = new cv::dnn::TextDetectionModel_EAST(*network);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// TextDetectionModel_EAST(const std::string &, const std::string &)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1858
	// ("cv::dnn::TextDetectionModel_EAST::TextDetectionModel_EAST", vec![(pred!(mut, ["model", "config"], ["const std::string*", "const std::string*"]), _)]),
	void cv_dnn_TextDetectionModel_EAST_TextDetectionModel_EAST_const_stringR_const_stringR(const char* model, const char* config, Result<cv::dnn::TextDetectionModel_EAST*>* ocvrs_return) {
		try {
			cv::dnn::TextDetectionModel_EAST* ret = new cv::dnn::TextDetectionModel_EAST(std::string(model), std::string(config));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::TextDetectionModel_EAST::TextDetectionModel_EAST(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1858
	// ("cv::dnn::TextDetectionModel_EAST::TextDetectionModel_EAST", vec![(pred!(mut, ["model"], ["const std::string*"]), _)]),
	void cv_dnn_TextDetectionModel_EAST_TextDetectionModel_EAST_const_stringR(const char* model, Result<cv::dnn::TextDetectionModel_EAST*>* ocvrs_return) {
		try {
			cv::dnn::TextDetectionModel_EAST* ret = new cv::dnn::TextDetectionModel_EAST(std::string(model));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setConfidenceThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1866
	// ("cv::dnn::TextDetectionModel_EAST::setConfidenceThreshold", vec![(pred!(mut, ["confThreshold"], ["float"]), _)]),
	void cv_dnn_TextDetectionModel_EAST_setConfidenceThreshold_float(cv::dnn::TextDetectionModel_EAST* instance, float confThreshold, Result<cv::dnn::TextDetectionModel_EAST*>* ocvrs_return) {
		try {
			cv::dnn::TextDetectionModel_EAST ret = instance->setConfidenceThreshold(confThreshold);
			Ok(new cv::dnn::TextDetectionModel_EAST(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getConfidenceThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1872
	// ("cv::dnn::TextDetectionModel_EAST::getConfidenceThreshold", vec![(pred!(const, [], []), _)]),
	void cv_dnn_TextDetectionModel_EAST_getConfidenceThreshold_const(const cv::dnn::TextDetectionModel_EAST* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getConfidenceThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNMSThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1879
	// ("cv::dnn::TextDetectionModel_EAST::setNMSThreshold", vec![(pred!(mut, ["nmsThreshold"], ["float"]), _)]),
	void cv_dnn_TextDetectionModel_EAST_setNMSThreshold_float(cv::dnn::TextDetectionModel_EAST* instance, float nmsThreshold, Result<cv::dnn::TextDetectionModel_EAST*>* ocvrs_return) {
		try {
			cv::dnn::TextDetectionModel_EAST ret = instance->setNMSThreshold(nmsThreshold);
			Ok(new cv::dnn::TextDetectionModel_EAST(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNMSThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1885
	// ("cv::dnn::TextDetectionModel_EAST::getNMSThreshold", vec![(pred!(const, [], []), _)]),
	void cv_dnn_TextDetectionModel_EAST_getNMSThreshold_const(const cv::dnn::TextDetectionModel_EAST* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getNMSThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::TextDetectionModel_EAST::implicitClone() generated
	// ("cv::dnn::TextDetectionModel_EAST::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::dnn::TextDetectionModel_EAST* cv_dnn_TextDetectionModel_EAST_implicitClone_const(const cv::dnn::TextDetectionModel_EAST* instance) {
			return new cv::dnn::TextDetectionModel_EAST(*instance);
	}

	// cv::dnn::TextDetectionModel_EAST::to_Model() generated
	// ("cv::dnn::TextDetectionModel_EAST::to_Model", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Model* cv_dnn_TextDetectionModel_EAST_to_Model(cv::dnn::TextDetectionModel_EAST* instance) {
			return dynamic_cast<cv::dnn::Model*>(instance);
	}

	// cv::dnn::TextDetectionModel_EAST::to_TextDetectionModel() generated
	// ("cv::dnn::TextDetectionModel_EAST::to_TextDetectionModel", vec![(pred!(mut, [], []), _)]),
	cv::dnn::TextDetectionModel* cv_dnn_TextDetectionModel_EAST_to_TextDetectionModel(cv::dnn::TextDetectionModel_EAST* instance) {
			return dynamic_cast<cv::dnn::TextDetectionModel*>(instance);
	}

	// cv::dnn::TextDetectionModel_EAST::delete() generated
	// ("cv::dnn::TextDetectionModel_EAST::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_TextDetectionModel_EAST_delete(cv::dnn::TextDetectionModel_EAST* instance) {
			delete instance;
	}

	// TextRecognitionModel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1687
	// ("cv::dnn::TextRecognitionModel::TextRecognitionModel", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_TextRecognitionModel_TextRecognitionModel(Result<cv::dnn::TextRecognitionModel*>* ocvrs_return) {
		try {
			cv::dnn::TextRecognitionModel* ret = new cv::dnn::TextRecognitionModel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// TextRecognitionModel(const Net &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1694
	// ("cv::dnn::TextRecognitionModel::TextRecognitionModel", vec![(pred!(mut, ["network"], ["const cv::dnn::Net*"]), _)]),
	void cv_dnn_TextRecognitionModel_TextRecognitionModel_const_NetR(const cv::dnn::Net* network, Result<cv::dnn::TextRecognitionModel*>* ocvrs_return) {
		try {
			cv::dnn::TextRecognitionModel* ret = new cv::dnn::TextRecognitionModel(*network);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// TextRecognitionModel(const std::string &, const std::string &)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1703
	// ("cv::dnn::TextRecognitionModel::TextRecognitionModel", vec![(pred!(mut, ["model", "config"], ["const std::string*", "const std::string*"]), _)]),
	void cv_dnn_TextRecognitionModel_TextRecognitionModel_const_stringR_const_stringR(const char* model, const char* config, Result<cv::dnn::TextRecognitionModel*>* ocvrs_return) {
		try {
			cv::dnn::TextRecognitionModel* ret = new cv::dnn::TextRecognitionModel(std::string(model), std::string(config));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::TextRecognitionModel::TextRecognitionModel(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1703
	// ("cv::dnn::TextRecognitionModel::TextRecognitionModel", vec![(pred!(mut, ["model"], ["const std::string*"]), _)]),
	void cv_dnn_TextRecognitionModel_TextRecognitionModel_const_stringR(const char* model, Result<cv::dnn::TextRecognitionModel*>* ocvrs_return) {
		try {
			cv::dnn::TextRecognitionModel* ret = new cv::dnn::TextRecognitionModel(std::string(model));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDecodeType(const std::string &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1713
	// ("cv::dnn::TextRecognitionModel::setDecodeType", vec![(pred!(mut, ["decodeType"], ["const std::string*"]), _)]),
	void cv_dnn_TextRecognitionModel_setDecodeType_const_stringR(cv::dnn::TextRecognitionModel* instance, const char* decodeType, Result<cv::dnn::TextRecognitionModel*>* ocvrs_return) {
		try {
			cv::dnn::TextRecognitionModel ret = instance->setDecodeType(std::string(decodeType));
			Ok(new cv::dnn::TextRecognitionModel(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDecodeType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1720
	// ("cv::dnn::TextRecognitionModel::getDecodeType", vec![(pred!(const, [], []), _)]),
	void cv_dnn_TextRecognitionModel_getDecodeType_const(const cv::dnn::TextRecognitionModel* instance, Result<void*>* ocvrs_return) {
		try {
			const std::string ret = instance->getDecodeType();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDecodeOptsCTCPrefixBeamSearch(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1729
	// ("cv::dnn::TextRecognitionModel::setDecodeOptsCTCPrefixBeamSearch", vec![(pred!(mut, ["beamSize", "vocPruneSize"], ["int", "int"]), _)]),
	void cv_dnn_TextRecognitionModel_setDecodeOptsCTCPrefixBeamSearch_int_int(cv::dnn::TextRecognitionModel* instance, int beamSize, int vocPruneSize, Result<cv::dnn::TextRecognitionModel*>* ocvrs_return) {
		try {
			cv::dnn::TextRecognitionModel ret = instance->setDecodeOptsCTCPrefixBeamSearch(beamSize, vocPruneSize);
			Ok(new cv::dnn::TextRecognitionModel(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::TextRecognitionModel::setDecodeOptsCTCPrefixBeamSearch(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1729
	// ("cv::dnn::TextRecognitionModel::setDecodeOptsCTCPrefixBeamSearch", vec![(pred!(mut, ["beamSize"], ["int"]), _)]),
	void cv_dnn_TextRecognitionModel_setDecodeOptsCTCPrefixBeamSearch_int(cv::dnn::TextRecognitionModel* instance, int beamSize, Result<cv::dnn::TextRecognitionModel*>* ocvrs_return) {
		try {
			cv::dnn::TextRecognitionModel ret = instance->setDecodeOptsCTCPrefixBeamSearch(beamSize);
			Ok(new cv::dnn::TextRecognitionModel(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setVocabulary(const std::vector<std::string> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1736
	// ("cv::dnn::TextRecognitionModel::setVocabulary", vec![(pred!(mut, ["vocabulary"], ["const std::vector<std::string>*"]), _)]),
	void cv_dnn_TextRecognitionModel_setVocabulary_const_vectorLstringGR(cv::dnn::TextRecognitionModel* instance, const std::vector<std::string>* vocabulary, Result<cv::dnn::TextRecognitionModel*>* ocvrs_return) {
		try {
			cv::dnn::TextRecognitionModel ret = instance->setVocabulary(*vocabulary);
			Ok(new cv::dnn::TextRecognitionModel(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVocabulary()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1743
	// ("cv::dnn::TextRecognitionModel::getVocabulary", vec![(pred!(const, [], []), _)]),
	void cv_dnn_TextRecognitionModel_getVocabulary_const(const cv::dnn::TextRecognitionModel* instance, Result<std::vector<std::string>*>* ocvrs_return) {
		try {
			const std::vector<std::string> ret = instance->getVocabulary();
			Ok(new const std::vector<std::string>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// recognize(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1751
	// ("cv::dnn::TextRecognitionModel::recognize", vec![(pred!(const, ["frame"], ["const cv::_InputArray*"]), _)]),
	void cv_dnn_TextRecognitionModel_recognize_const_const__InputArrayR(const cv::dnn::TextRecognitionModel* instance, const cv::_InputArray* frame, Result<void*>* ocvrs_return) {
		try {
			std::string ret = instance->recognize(*frame);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// recognize(InputArray, InputArrayOfArrays, std::vector<std::string> &)(InputArray, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/dnn.hpp:1760
	// ("cv::dnn::TextRecognitionModel::recognize", vec![(pred!(const, ["frame", "roiRects", "results"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::string>*"]), _)]),
	void cv_dnn_TextRecognitionModel_recognize_const_const__InputArrayR_const__InputArrayR_vectorLstringGR(const cv::dnn::TextRecognitionModel* instance, const cv::_InputArray* frame, const cv::_InputArray* roiRects, std::vector<std::string>* results, ResultVoid* ocvrs_return) {
		try {
			instance->recognize(*frame, *roiRects, *results);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::TextRecognitionModel::implicitClone() generated
	// ("cv::dnn::TextRecognitionModel::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::dnn::TextRecognitionModel* cv_dnn_TextRecognitionModel_implicitClone_const(const cv::dnn::TextRecognitionModel* instance) {
			return new cv::dnn::TextRecognitionModel(*instance);
	}

	// cv::dnn::TextRecognitionModel::to_Model() generated
	// ("cv::dnn::TextRecognitionModel::to_Model", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Model* cv_dnn_TextRecognitionModel_to_Model(cv::dnn::TextRecognitionModel* instance) {
			return dynamic_cast<cv::dnn::Model*>(instance);
	}

	// cv::dnn::TextRecognitionModel::delete() generated
	// ("cv::dnn::TextRecognitionModel::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_TextRecognitionModel_delete(cv::dnn::TextRecognitionModel* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:865
	// ("cv::dnn::ThresholdedReluLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ThresholdedReluLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::ThresholdedReluLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::ThresholdedReluLayer> ret = cv::dnn::ThresholdedReluLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::ThresholdedReluLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ThresholdedReluLayer::defaultNew() generated
	// ("cv::dnn::ThresholdedReluLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::ThresholdedReluLayer* cv_dnn_ThresholdedReluLayer_defaultNew_const() {
			cv::dnn::ThresholdedReluLayer* ret = new cv::dnn::ThresholdedReluLayer();
			return ret;
	}

	// cv::dnn::ThresholdedReluLayer::alpha() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:863
	// ("cv::dnn::ThresholdedReluLayer::alpha", vec![(pred!(const, [], []), _)]),
	float cv_dnn_ThresholdedReluLayer_propAlpha_const(const cv::dnn::ThresholdedReluLayer* instance) {
			float ret = instance->alpha;
			return ret;
	}

	// cv::dnn::ThresholdedReluLayer::setAlpha(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:863
	// ("cv::dnn::ThresholdedReluLayer::setAlpha", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_ThresholdedReluLayer_propAlpha_const_float(cv::dnn::ThresholdedReluLayer* instance, const float val) {
			instance->alpha = val;
	}

	// cv::dnn::ThresholdedReluLayer::to_ActivationLayer() generated
	// ("cv::dnn::ThresholdedReluLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_ThresholdedReluLayer_to_ActivationLayer(cv::dnn::ThresholdedReluLayer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
	}

	// cv::dnn::ThresholdedReluLayer::to_Algorithm() generated
	// ("cv::dnn::ThresholdedReluLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_ThresholdedReluLayer_to_Algorithm(cv::dnn::ThresholdedReluLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::ThresholdedReluLayer::to_Layer() generated
	// ("cv::dnn::ThresholdedReluLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_ThresholdedReluLayer_to_Layer(cv::dnn::ThresholdedReluLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::ThresholdedReluLayer::delete() generated
	// ("cv::dnn::ThresholdedReluLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_ThresholdedReluLayer_delete(cv::dnn::ThresholdedReluLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1140
	// ("cv::dnn::TileLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_TileLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::TileLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::TileLayer> ret = cv::dnn::TileLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::TileLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::TileLayer::defaultNew() generated
	// ("cv::dnn::TileLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::TileLayer* cv_dnn_TileLayer_defaultNew_const() {
			cv::dnn::TileLayer* ret = new cv::dnn::TileLayer();
			return ret;
	}

	// cv::dnn::TileLayer::to_Algorithm() generated
	// ("cv::dnn::TileLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_TileLayer_to_Algorithm(cv::dnn::TileLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::TileLayer::to_Layer() generated
	// ("cv::dnn::TileLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_TileLayer_to_Layer(cv::dnn::TileLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::TileLayer::delete() generated
	// ("cv::dnn::TileLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_TileLayer_delete(cv::dnn::TileLayer* instance) {
			delete instance;
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/all_layers.hpp:1204
	// ("cv::dnn::TopKLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_TopKLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::TopKLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::TopKLayer> ret = cv::dnn::TopKLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::TopKLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::TopKLayer::defaultNew() generated
	// ("cv::dnn::TopKLayer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::TopKLayer* cv_dnn_TopKLayer_defaultNew_const() {
			cv::dnn::TopKLayer* ret = new cv::dnn::TopKLayer();
			return ret;
	}

	// cv::dnn::TopKLayer::to_Algorithm() generated
	// ("cv::dnn::TopKLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_dnn_TopKLayer_to_Algorithm(cv::dnn::TopKLayer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::dnn::TopKLayer::to_Layer() generated
	// ("cv::dnn::TopKLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Layer* cv_dnn_TopKLayer_to_Layer(cv::dnn::TopKLayer* instance) {
			return dynamic_cast<cv::dnn::Layer*>(instance);
	}

	// cv::dnn::TopKLayer::delete() generated
	// ("cv::dnn::TopKLayer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_TopKLayer_delete(cv::dnn::TopKLayer* instance) {
			delete instance;
	}

	// _Range(const Range &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/shape_utils.hpp:59
	// ("cv::dnn::_Range::_Range", vec![(pred!(mut, ["r"], ["const cv::Range*"]), _)]),
	void cv_dnn__Range__Range_const_RangeR(const cv::Range* r, Result<cv::dnn::_Range*>* ocvrs_return) {
		try {
			cv::dnn::_Range* ret = new cv::dnn::_Range(*r);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _Range(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/shape_utils.hpp:60
	// ("cv::dnn::_Range::_Range", vec![(pred!(mut, ["start_", "size_"], ["int", "int"]), _)]),
	void cv_dnn__Range__Range_int_int(int start_, int size_, Result<cv::dnn::_Range*>* ocvrs_return) {
		try {
			cv::dnn::_Range* ret = new cv::dnn::_Range(start_, size_);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::_Range::_Range(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn/shape_utils.hpp:60
	// ("cv::dnn::_Range::_Range", vec![(pred!(mut, ["start_"], ["int"]), _)]),
	void cv_dnn__Range__Range_int(int start_, Result<cv::dnn::_Range*>* ocvrs_return) {
		try {
			cv::dnn::_Range* ret = new cv::dnn::_Range(start_);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::_Range::to_Range() generated
	// ("cv::dnn::_Range::to_Range", vec![(pred!(mut, [], []), _)]),
	cv::Range* cv_dnn__Range_to_Range(cv::dnn::_Range* instance) {
			return dynamic_cast<cv::Range*>(instance);
	}

	// cv::dnn::_Range::delete() generated
	// ("cv::dnn::_Range::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn__Range_delete(cv::dnn::_Range* instance) {
			delete instance;
	}

}
