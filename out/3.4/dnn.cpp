#include "dnn.hpp"
#include "dnn_types.hpp"

extern "C" {
	// cv::dnn::NMSBoxes(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1078
	// ("cv::dnn::NMSBoxes", vec![(pred!(mut, ["bboxes", "scores", "score_threshold", "nms_threshold", "indices"], ["const std::vector<cv::Rect2d>*", "const std::vector<float>*", "const float", "const float", "std::vector<int>*"]), _)]),
	void cv_dnn_NMSBoxes_const_vectorLRect2dGR_const_vectorLfloatGR_const_float_const_float_vectorLintGR(const std::vector<cv::Rect2d>* bboxes, const std::vector<float>* scores, const float score_threshold, const float nms_threshold, std::vector<int>* indices, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::NMSBoxes(*bboxes, *scores, score_threshold, nms_threshold, *indices);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// NMSBoxes(const std::vector<Rect2d> &, const std::vector<float> &, const float, const float, std::vector<int> &, const float, const int)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1078
	// ("cv::dnn::NMSBoxes", vec![(pred!(mut, ["bboxes", "scores", "score_threshold", "nms_threshold", "indices", "eta", "top_k"], ["const std::vector<cv::Rect2d>*", "const std::vector<float>*", "const float", "const float", "std::vector<int>*", "const float", "const int"]), _)]),
	void cv_dnn_NMSBoxes_const_vectorLRect2dGR_const_vectorLfloatGR_const_float_const_float_vectorLintGR_const_float_const_int(const std::vector<cv::Rect2d>* bboxes, const std::vector<float>* scores, const float score_threshold, const float nms_threshold, std::vector<int>* indices, const float eta, const int top_k, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::NMSBoxes(*bboxes, *scores, score_threshold, nms_threshold, *indices, eta, top_k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::NMSBoxes(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1073
	// ("cv::dnn::NMSBoxes", vec![(pred!(mut, ["bboxes", "scores", "score_threshold", "nms_threshold", "indices"], ["const std::vector<cv::Rect>*", "const std::vector<float>*", "const float", "const float", "std::vector<int>*"]), _)]),
	void cv_dnn_NMSBoxes_const_vectorLRectGR_const_vectorLfloatGR_const_float_const_float_vectorLintGR(const std::vector<cv::Rect>* bboxes, const std::vector<float>* scores, const float score_threshold, const float nms_threshold, std::vector<int>* indices, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::NMSBoxes(*bboxes, *scores, score_threshold, nms_threshold, *indices);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// NMSBoxes(const std::vector<Rect> &, const std::vector<float> &, const float, const float, std::vector<int> &, const float, const int)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1073
	// ("cv::dnn::NMSBoxes", vec![(pred!(mut, ["bboxes", "scores", "score_threshold", "nms_threshold", "indices", "eta", "top_k"], ["const std::vector<cv::Rect>*", "const std::vector<float>*", "const float", "const float", "std::vector<int>*", "const float", "const int"]), _)]),
	void cv_dnn_NMSBoxes_const_vectorLRectGR_const_vectorLfloatGR_const_float_const_float_vectorLintGR_const_float_const_int(const std::vector<cv::Rect>* bboxes, const std::vector<float>* scores, const float score_threshold, const float nms_threshold, std::vector<int>* indices, const float eta, const int top_k, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::NMSBoxes(*bboxes, *scores, score_threshold, nms_threshold, *indices, eta, top_k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::NMSBoxes(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1083
	// ("cv::dnn::NMSBoxes", vec![(pred!(mut, ["bboxes", "scores", "score_threshold", "nms_threshold", "indices"], ["const std::vector<cv::RotatedRect>*", "const std::vector<float>*", "const float", "const float", "std::vector<int>*"]), _)]),
	void cv_dnn_NMSBoxes_const_vectorLRotatedRectGR_const_vectorLfloatGR_const_float_const_float_vectorLintGR(const std::vector<cv::RotatedRect>* bboxes, const std::vector<float>* scores, const float score_threshold, const float nms_threshold, std::vector<int>* indices, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::NMSBoxes(*bboxes, *scores, score_threshold, nms_threshold, *indices);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// NMSBoxes(const std::vector<RotatedRect> &, const std::vector<float> &, const float, const float, std::vector<int> &, const float, const int)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1083
	// ("cv::dnn::NMSBoxes", vec![(pred!(mut, ["bboxes", "scores", "score_threshold", "nms_threshold", "indices", "eta", "top_k"], ["const std::vector<cv::RotatedRect>*", "const std::vector<float>*", "const float", "const float", "std::vector<int>*", "const float", "const int"]), _)]),
	void cv_dnn_NMSBoxes_const_vectorLRotatedRectGR_const_vectorLfloatGR_const_float_const_float_vectorLintGR_const_float_const_int(const std::vector<cv::RotatedRect>* bboxes, const std::vector<float>* scores, const float score_threshold, const float nms_threshold, std::vector<int>* indices, const float eta, const int top_k, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::NMSBoxes(*bboxes, *scores, score_threshold, nms_threshold, *indices, eta, top_k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::blobFromImage(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:986
	// ("cv::dnn::blobFromImage", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
	void cv_dnn_blobFromImage_const__InputArrayR(const cv::_InputArray* image, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::dnn::blobFromImage(*image);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::blobFromImage(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:994
	// ("cv::dnn::blobFromImage", vec![(pred!(mut, ["image", "blob"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_dnn_blobFromImage_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* image, const cv::_OutputArray* blob, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::blobFromImage(*image, *blob);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// blobFromImage(InputArray, OutputArray, double, const Size &, const Scalar &, bool, bool, int)(InputArray, OutputArray, Primitive, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:994
	// ("cv::dnn::blobFromImage", vec![(pred!(mut, ["image", "blob", "scalefactor", "size", "mean", "swapRB", "crop", "ddepth"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "const cv::Size*", "const cv::Scalar*", "bool", "bool", "int"]), _)]),
	void cv_dnn_blobFromImage_const__InputArrayR_const__OutputArrayR_double_const_SizeR_const_ScalarR_bool_bool_int(const cv::_InputArray* image, const cv::_OutputArray* blob, double scalefactor, const cv::Size* size, const cv::Scalar* mean, bool swapRB, bool crop, int ddepth, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::blobFromImage(*image, *blob, scalefactor, *size, *mean, swapRB, crop, ddepth);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// blobFromImage(InputArray, double, const Size &, const Scalar &, bool, bool, int)(InputArray, Primitive, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:986
	// ("cv::dnn::blobFromImage", vec![(pred!(mut, ["image", "scalefactor", "size", "mean", "swapRB", "crop", "ddepth"], ["const cv::_InputArray*", "double", "const cv::Size*", "const cv::Scalar*", "bool", "bool", "int"]), _)]),
	void cv_dnn_blobFromImage_const__InputArrayR_double_const_SizeR_const_ScalarR_bool_bool_int(const cv::_InputArray* image, double scalefactor, const cv::Size* size, const cv::Scalar* mean, bool swapRB, bool crop, int ddepth, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::dnn::blobFromImage(*image, scalefactor, *size, *mean, swapRB, crop, ddepth);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::blobFromImages(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1016
	// ("cv::dnn::blobFromImages", vec![(pred!(mut, ["images"], ["const cv::_InputArray*"]), _)]),
	void cv_dnn_blobFromImages_const__InputArrayR(const cv::_InputArray* images, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::dnn::blobFromImages(*images);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::blobFromImages(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1024
	// ("cv::dnn::blobFromImages", vec![(pred!(mut, ["images", "blob"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_dnn_blobFromImages_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* images, const cv::_OutputArray* blob, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::blobFromImages(*images, *blob);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// blobFromImages(InputArrayOfArrays, OutputArray, double, Size, const Scalar &, bool, bool, int)(InputArray, OutputArray, Primitive, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1024
	// ("cv::dnn::blobFromImages", vec![(pred!(mut, ["images", "blob", "scalefactor", "size", "mean", "swapRB", "crop", "ddepth"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "cv::Size", "const cv::Scalar*", "bool", "bool", "int"]), _)]),
	void cv_dnn_blobFromImages_const__InputArrayR_const__OutputArrayR_double_Size_const_ScalarR_bool_bool_int(const cv::_InputArray* images, const cv::_OutputArray* blob, double scalefactor, cv::Size* size, const cv::Scalar* mean, bool swapRB, bool crop, int ddepth, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::blobFromImages(*images, *blob, scalefactor, *size, *mean, swapRB, crop, ddepth);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// blobFromImages(InputArrayOfArrays, double, Size, const Scalar &, bool, bool, int)(InputArray, Primitive, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1016
	// ("cv::dnn::blobFromImages", vec![(pred!(mut, ["images", "scalefactor", "size", "mean", "swapRB", "crop", "ddepth"], ["const cv::_InputArray*", "double", "cv::Size", "const cv::Scalar*", "bool", "bool", "int"]), _)]),
	void cv_dnn_blobFromImages_const__InputArrayR_double_Size_const_ScalarR_bool_bool_int(const cv::_InputArray* images, double scalefactor, cv::Size* size, const cv::Scalar* mean, bool swapRB, bool crop, int ddepth, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::dnn::blobFromImages(*images, scalefactor, *size, *mean, swapRB, crop, ddepth);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// concat(const MatShape &, const MatShape &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:206
	// ("cv::dnn::concat", vec![(pred!(mut, ["a", "b"], ["const cv::dnn::MatShape*", "const cv::dnn::MatShape*"]), _)]),
	void cv_dnn_concat_const_MatShapeR_const_MatShapeR(const cv::dnn::MatShape* a, const cv::dnn::MatShape* b, Result<cv::dnn::MatShape*>* ocvrs_return) {
		try {
			cv::dnn::MatShape ret = cv::dnn::concat(*a, *b);
			Ok(new cv::dnn::MatShape(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAvailableBackends()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:104
	// ("cv::dnn::getAvailableBackends", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_getAvailableBackends(Result<std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>*>* ocvrs_return) {
		try {
			std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>> ret = cv::dnn::getAvailableBackends();
			Ok(new std::vector<std::pair<cv::dnn::Backend, cv::dnn::Target>>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAvailableTargets(dnn::Backend)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:105
	// ("cv::dnn::getAvailableTargets", vec![(pred!(mut, ["be"], ["cv::dnn::Backend"]), _)]),
	void cv_dnn_getAvailableTargets_Backend(cv::dnn::Backend be, Result<std::vector<cv::dnn::Target>*>* ocvrs_return) {
		try {
			std::vector<cv::dnn::Target> ret = cv::dnn::getAvailableTargets(be);
			Ok(new std::vector<cv::dnn::Target>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getInferenceEngineBackendType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/utils/inference_engine.hpp:31
	// ("cv::dnn::getInferenceEngineBackendType", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_getInferenceEngineBackendType(Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::dnn::getInferenceEngineBackendType();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getInferenceEngineCPUType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/utils/inference_engine.hpp:72
	// ("cv::dnn::getInferenceEngineCPUType", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_getInferenceEngineCPUType(Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::dnn::getInferenceEngineCPUType();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getInferenceEngineVPUType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/utils/inference_engine.hpp:66
	// ("cv::dnn::getInferenceEngineVPUType", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_getInferenceEngineVPUType(Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::dnn::getInferenceEngineVPUType();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPlane(const Mat &, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:108
	// ("cv::dnn::getPlane", vec![(pred!(mut, ["m", "n", "cn"], ["const cv::Mat*", "int", "int"]), _)]),
	void cv_dnn_getPlane_const_MatR_int_int(const cv::Mat* m, int n, int cn, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::dnn::getPlane(*m, n, cn);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imagesFromBlob(const cv::Mat &, OutputArrayOfArrays)(TraitClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1037
	// ("cv::dnn::imagesFromBlob", vec![(pred!(mut, ["blob_", "images_"], ["const cv::Mat*", "const cv::_OutputArray*"]), _)]),
	void cv_dnn_imagesFromBlob_const_MatR_const__OutputArrayR(const cv::Mat* blob_, const cv::_OutputArray* images_, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::imagesFromBlob(*blob_, *images_);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::readNetFromCaffe(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:787
	// ("cv::dnn::readNetFromCaffe", vec![(pred!(mut, ["prototxt"], ["const cv::String*"]), _)]),
	void cv_dnn_readNetFromCaffe_const_StringR(const char* prototxt, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromCaffe(cv::String(prototxt));
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromCaffe(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:787
	// ("cv::dnn::readNetFromCaffe", vec![(pred!(mut, ["prototxt", "caffeModel"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_dnn_readNetFromCaffe_const_StringR_const_StringR(const char* prototxt, const char* caffeModel, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromCaffe(cv::String(prototxt), cv::String(caffeModel));
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::readNetFromCaffe(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:806
	// ("cv::dnn::readNetFromCaffe", vec![(pred!(mut, ["bufferProto", "lenProto"], ["const char*", "size_t"]), _)]),
	void cv_dnn_readNetFromCaffe_const_charX_size_t(const char* bufferProto, size_t lenProto, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromCaffe(bufferProto, lenProto);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromCaffe(const char *, size_t, const char *, size_t)(InString, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:806
	// ("cv::dnn::readNetFromCaffe", vec![(pred!(mut, ["bufferProto", "lenProto", "bufferModel", "lenModel"], ["const char*", "size_t", "const char*", "size_t"]), _)]),
	void cv_dnn_readNetFromCaffe_const_charX_size_t_const_charX_size_t(const char* bufferProto, size_t lenProto, const char* bufferModel, size_t lenModel, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromCaffe(bufferProto, lenProto, bufferModel, lenModel);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::readNetFromCaffe(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:794
	// ("cv::dnn::readNetFromCaffe", vec![(pred!(mut, ["bufferProto"], ["const std::vector<unsigned char>*"]), _)]),
	void cv_dnn_readNetFromCaffe_const_vectorLunsigned_charGR(const std::vector<unsigned char>* bufferProto, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromCaffe(*bufferProto);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromCaffe(const std::vector<uchar> &, const std::vector<uchar> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:794
	// ("cv::dnn::readNetFromCaffe", vec![(pred!(mut, ["bufferProto", "bufferModel"], ["const std::vector<unsigned char>*", "const std::vector<unsigned char>*"]), _)]),
	void cv_dnn_readNetFromCaffe_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR(const std::vector<unsigned char>* bufferProto, const std::vector<unsigned char>* bufferModel, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromCaffe(*bufferProto, *bufferModel);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::readNetFromDarknet(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:762
	// ("cv::dnn::readNetFromDarknet", vec![(pred!(mut, ["cfgFile"], ["const cv::String*"]), _)]),
	void cv_dnn_readNetFromDarknet_const_StringR(const char* cfgFile, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromDarknet(cv::String(cfgFile));
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromDarknet(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:762
	// ("cv::dnn::readNetFromDarknet", vec![(pred!(mut, ["cfgFile", "darknetModel"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_dnn_readNetFromDarknet_const_StringR_const_StringR(const char* cfgFile, const char* darknetModel, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromDarknet(cv::String(cfgFile), cv::String(darknetModel));
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::readNetFromDarknet(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:779
	// ("cv::dnn::readNetFromDarknet", vec![(pred!(mut, ["bufferCfg", "lenCfg"], ["const char*", "size_t"]), _)]),
	void cv_dnn_readNetFromDarknet_const_charX_size_t(const char* bufferCfg, size_t lenCfg, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromDarknet(bufferCfg, lenCfg);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromDarknet(const char *, size_t, const char *, size_t)(InString, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:779
	// ("cv::dnn::readNetFromDarknet", vec![(pred!(mut, ["bufferCfg", "lenCfg", "bufferModel", "lenModel"], ["const char*", "size_t", "const char*", "size_t"]), _)]),
	void cv_dnn_readNetFromDarknet_const_charX_size_t_const_charX_size_t(const char* bufferCfg, size_t lenCfg, const char* bufferModel, size_t lenModel, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromDarknet(bufferCfg, lenCfg, bufferModel, lenModel);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::readNetFromDarknet(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:769
	// ("cv::dnn::readNetFromDarknet", vec![(pred!(mut, ["bufferCfg"], ["const std::vector<unsigned char>*"]), _)]),
	void cv_dnn_readNetFromDarknet_const_vectorLunsigned_charGR(const std::vector<unsigned char>* bufferCfg, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromDarknet(*bufferCfg);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromDarknet(const std::vector<uchar> &, const std::vector<uchar> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:769
	// ("cv::dnn::readNetFromDarknet", vec![(pred!(mut, ["bufferCfg", "bufferModel"], ["const std::vector<unsigned char>*", "const std::vector<unsigned char>*"]), _)]),
	void cv_dnn_readNetFromDarknet_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR(const std::vector<unsigned char>* bufferCfg, const std::vector<unsigned char>* bufferModel, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromDarknet(*bufferCfg, *bufferModel);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromModelOptimizer(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:916
	// ("cv::dnn::readNetFromModelOptimizer", vec![(pred!(mut, ["xml", "bin"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_dnn_readNetFromModelOptimizer_const_StringR_const_StringR(const char* xml, const char* bin, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromModelOptimizer(cv::String(xml), cv::String(bin));
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromModelOptimizer(const uchar *, size_t, const uchar *, size_t)(VariableArray, Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:938
	// ("cv::dnn::readNetFromModelOptimizer", vec![(pred!(mut, ["bufferModelConfigPtr", "bufferModelConfigSize", "bufferWeightsPtr", "bufferWeightsSize"], ["const unsigned char*", "size_t", "const unsigned char*", "size_t"]), _)]),
	void cv_dnn_readNetFromModelOptimizer_const_unsigned_charX_size_t_const_unsigned_charX_size_t(const unsigned char* bufferModelConfigPtr, size_t bufferModelConfigSize, const unsigned char* bufferWeightsPtr, size_t bufferWeightsSize, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromModelOptimizer(bufferModelConfigPtr, bufferModelConfigSize, bufferWeightsPtr, bufferWeightsSize);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromModelOptimizer(const std::vector<uchar> &, const std::vector<uchar> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:926
	// ("cv::dnn::readNetFromModelOptimizer", vec![(pred!(mut, ["bufferModelConfig", "bufferWeights"], ["const std::vector<unsigned char>*", "const std::vector<unsigned char>*"]), _)]),
	void cv_dnn_readNetFromModelOptimizer_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR(const std::vector<unsigned char>* bufferModelConfig, const std::vector<unsigned char>* bufferWeights, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromModelOptimizer(*bufferModelConfig, *bufferWeights);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromONNX(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:945
	// ("cv::dnn::readNetFromONNX", vec![(pred!(mut, ["onnxFile"], ["const cv::String*"]), _)]),
	void cv_dnn_readNetFromONNX_const_StringR(const char* onnxFile, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromONNX(cv::String(onnxFile));
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromONNX(const char *, size_t)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:954
	// ("cv::dnn::readNetFromONNX", vec![(pred!(mut, ["buffer", "sizeBuffer"], ["const char*", "size_t"]), _)]),
	void cv_dnn_readNetFromONNX_const_charX_size_t(const char* buffer, size_t sizeBuffer, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromONNX(buffer, sizeBuffer);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromONNX(const std::vector<uchar> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:962
	// ("cv::dnn::readNetFromONNX", vec![(pred!(mut, ["buffer"], ["const std::vector<unsigned char>*"]), _)]),
	void cv_dnn_readNetFromONNX_const_vectorLunsigned_charGR(const std::vector<unsigned char>* buffer, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromONNX(*buffer);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::readNetFromTensorflow(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:816
	// ("cv::dnn::readNetFromTensorflow", vec![(pred!(mut, ["model"], ["const cv::String*"]), _)]),
	void cv_dnn_readNetFromTensorflow_const_StringR(const char* model, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromTensorflow(cv::String(model));
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromTensorflow(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:816
	// ("cv::dnn::readNetFromTensorflow", vec![(pred!(mut, ["model", "config"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_dnn_readNetFromTensorflow_const_StringR_const_StringR(const char* model, const char* config, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromTensorflow(cv::String(model), cv::String(config));
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::readNetFromTensorflow(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:834
	// ("cv::dnn::readNetFromTensorflow", vec![(pred!(mut, ["bufferModel", "lenModel"], ["const char*", "size_t"]), _)]),
	void cv_dnn_readNetFromTensorflow_const_charX_size_t(const char* bufferModel, size_t lenModel, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromTensorflow(bufferModel, lenModel);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromTensorflow(const char *, size_t, const char *, size_t)(InString, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:834
	// ("cv::dnn::readNetFromTensorflow", vec![(pred!(mut, ["bufferModel", "lenModel", "bufferConfig", "lenConfig"], ["const char*", "size_t", "const char*", "size_t"]), _)]),
	void cv_dnn_readNetFromTensorflow_const_charX_size_t_const_charX_size_t(const char* bufferModel, size_t lenModel, const char* bufferConfig, size_t lenConfig, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromTensorflow(bufferModel, lenModel, bufferConfig, lenConfig);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::readNetFromTensorflow(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:823
	// ("cv::dnn::readNetFromTensorflow", vec![(pred!(mut, ["bufferModel"], ["const std::vector<unsigned char>*"]), _)]),
	void cv_dnn_readNetFromTensorflow_const_vectorLunsigned_charGR(const std::vector<unsigned char>* bufferModel, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromTensorflow(*bufferModel);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromTensorflow(const std::vector<uchar> &, const std::vector<uchar> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:823
	// ("cv::dnn::readNetFromTensorflow", vec![(pred!(mut, ["bufferModel", "bufferConfig"], ["const std::vector<unsigned char>*", "const std::vector<unsigned char>*"]), _)]),
	void cv_dnn_readNetFromTensorflow_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR(const std::vector<unsigned char>* bufferModel, const std::vector<unsigned char>* bufferConfig, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromTensorflow(*bufferModel, *bufferConfig);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::readNetFromTorch(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:863
	// ("cv::dnn::readNetFromTorch", vec![(pred!(mut, ["model"], ["const cv::String*"]), _)]),
	void cv_dnn_readNetFromTorch_const_StringR(const char* model, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromTorch(cv::String(model));
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNetFromTorch(const String &, bool, bool)(InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:863
	// ("cv::dnn::readNetFromTorch", vec![(pred!(mut, ["model", "isBinary", "evaluate"], ["const cv::String*", "bool", "bool"]), _)]),
	void cv_dnn_readNetFromTorch_const_StringR_bool_bool(const char* model, bool isBinary, bool evaluate, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromTorch(cv::String(model), isBinary, evaluate);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::readNet(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:889
	// ("cv::dnn::readNet", vec![(pred!(mut, ["model"], ["const cv::String*"]), _)]),
	void cv_dnn_readNet_const_StringR(const char* model, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNet(cv::String(model));
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNet(const String &, const String &, const String &)(InString, InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:889
	// ("cv::dnn::readNet", vec![(pred!(mut, ["model", "config", "framework"], ["const cv::String*", "const cv::String*", "const cv::String*"]), _)]),
	void cv_dnn_readNet_const_StringR_const_StringR_const_StringR(const char* model, const char* config, const char* framework, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNet(cv::String(model), cv::String(config), cv::String(framework));
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::readNet(InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:900
	// ("cv::dnn::readNet", vec![(pred!(mut, ["framework", "bufferModel"], ["const cv::String*", "const std::vector<unsigned char>*"]), _)]),
	void cv_dnn_readNet_const_StringR_const_vectorLunsigned_charGR(const char* framework, const std::vector<unsigned char>* bufferModel, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNet(cv::String(framework), *bufferModel);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readNet(const String &, const std::vector<uchar> &, const std::vector<uchar> &)(InString, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:900
	// ("cv::dnn::readNet", vec![(pred!(mut, ["framework", "bufferModel", "bufferConfig"], ["const cv::String*", "const std::vector<unsigned char>*", "const std::vector<unsigned char>*"]), _)]),
	void cv_dnn_readNet_const_StringR_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR(const char* framework, const std::vector<unsigned char>* bufferModel, const std::vector<unsigned char>* bufferConfig, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::readNet(cv::String(framework), *bufferModel, *bufferConfig);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readTensorFromONNX(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:968
	// ("cv::dnn::readTensorFromONNX", vec![(pred!(mut, ["path"], ["const cv::String*"]), _)]),
	void cv_dnn_readTensorFromONNX_const_StringR(const char* path, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::dnn::readTensorFromONNX(cv::String(path));
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::readTorchBlob(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:906
	// ("cv::dnn::readTorchBlob", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_dnn_readTorchBlob_const_StringR(const char* filename, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::dnn::readTorchBlob(cv::String(filename));
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readTorchBlob(const String &, bool)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:906
	// ("cv::dnn::readTorchBlob", vec![(pred!(mut, ["filename", "isBinary"], ["const cv::String*", "bool"]), _)]),
	void cv_dnn_readTorchBlob_const_StringR_bool(const char* filename, bool isBinary, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::dnn::readTorchBlob(cv::String(filename), isBinary);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// resetMyriadDevice()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/utils/inference_engine.hpp:49
	// ("cv::dnn::resetMyriadDevice", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_resetMyriadDevice(ResultVoid* ocvrs_return) {
		try {
			cv::dnn::resetMyriadDevice();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInferenceEngineBackendType(const cv::String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/utils/inference_engine.hpp:41
	// ("cv::dnn::setInferenceEngineBackendType", vec![(pred!(mut, ["newBackendType"], ["const cv::String*"]), _)]),
	void cv_dnn_setInferenceEngineBackendType_const_StringR(const char* newBackendType, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::dnn::setInferenceEngineBackendType(cv::String(newBackendType));
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// shape(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:126
	// ("cv::dnn::shape", vec![(pred!(mut, ["mat"], ["const cv::Mat*"]), _)]),
	void cv_dnn_shape_const_MatR(const cv::Mat* mat, Result<cv::dnn::MatShape*>* ocvrs_return) {
		try {
			cv::dnn::MatShape ret = cv::dnn::shape(*mat);
			Ok(new cv::dnn::MatShape(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// shape(const MatSize &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:131
	// ("cv::dnn::shape", vec![(pred!(mut, ["sz"], ["const cv::MatSize*"]), _)]),
	void cv_dnn_shape_const_MatSizeR(const cv::MatSize* sz, Result<cv::dnn::MatShape*>* ocvrs_return) {
		try {
			cv::dnn::MatShape ret = cv::dnn::shape(*sz);
			Ok(new cv::dnn::MatShape(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// shape(const UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:136
	// ("cv::dnn::shape", vec![(pred!(mut, ["mat"], ["const cv::UMat*"]), _)]),
	void cv_dnn_shape_const_UMatR(const cv::UMat* mat, Result<cv::dnn::MatShape*>* ocvrs_return) {
		try {
			cv::dnn::MatShape ret = cv::dnn::shape(*mat);
			Ok(new cv::dnn::MatShape(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// shape(const int *, const int)(Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:119
	// ("cv::dnn::shape", vec![(pred!(mut, ["dims", "n"], ["const int*", "const int"]), _)]),
	void cv_dnn_shape_const_intX_const_int(const int* dims, const int n, Result<cv::dnn::MatShape*>* ocvrs_return) {
		try {
			cv::dnn::MatShape ret = cv::dnn::shape(dims, n);
			Ok(new cv::dnn::MatShape(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::shape(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:153
	// ("cv::dnn::shape", vec![(pred!(mut, ["a0"], ["int"]), _)]),
	void cv_dnn_shape_int(int a0, Result<cv::dnn::MatShape*>* ocvrs_return) {
		try {
			cv::dnn::MatShape ret = cv::dnn::shape(a0);
			Ok(new cv::dnn::MatShape(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// shape(int, int, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:153
	// ("cv::dnn::shape", vec![(pred!(mut, ["a0", "a1", "a2", "a3"], ["int", "int", "int", "int"]), _)]),
	void cv_dnn_shape_int_int_int_int(int a0, int a1, int a2, int a3, Result<cv::dnn::MatShape*>* ocvrs_return) {
		try {
			cv::dnn::MatShape ret = cv::dnn::shape(a0, a1, a2, a3);
			Ok(new cv::dnn::MatShape(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::shrinkCaffeModel(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1052
	// ("cv::dnn::shrinkCaffeModel", vec![(pred!(mut, ["src", "dst"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_dnn_shrinkCaffeModel_const_StringR_const_StringR(const char* src, const char* dst, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::shrinkCaffeModel(cv::String(src), cv::String(dst));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// shrinkCaffeModel(const String &, const String &, const std::vector<String> &)(InString, InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1052
	// ("cv::dnn::shrinkCaffeModel", vec![(pred!(mut, ["src", "dst", "layersTypes"], ["const cv::String*", "const cv::String*", "const std::vector<cv::String>*"]), _)]),
	void cv_dnn_shrinkCaffeModel_const_StringR_const_StringR_const_vectorLStringGR(const char* src, const char* dst, const std::vector<cv::String>* layersTypes, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::shrinkCaffeModel(cv::String(src), cv::String(dst), *layersTypes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// slice(const Mat &, const _Range &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:63
	// ("cv::dnn::slice", vec![(pred!(mut, ["m", "r0"], ["const cv::Mat*", "const cv::dnn::_Range*"]), _)]),
	void cv_dnn_slice_const_MatR_const__RangeR(const cv::Mat* m, const cv::dnn::_Range* r0, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::dnn::slice(*m, *r0);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// slice(const Mat &, const _Range &, const _Range &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:72
	// ("cv::dnn::slice", vec![(pred!(mut, ["m", "r0", "r1"], ["const cv::Mat*", "const cv::dnn::_Range*", "const cv::dnn::_Range*"]), _)]),
	void cv_dnn_slice_const_MatR_const__RangeR_const__RangeR(const cv::Mat* m, const cv::dnn::_Range* r0, const cv::dnn::_Range* r1, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::dnn::slice(*m, *r0, *r1);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// slice(const Mat &, const _Range &, const _Range &, const _Range &)(TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:83
	// ("cv::dnn::slice", vec![(pred!(mut, ["m", "r0", "r1", "r2"], ["const cv::Mat*", "const cv::dnn::_Range*", "const cv::dnn::_Range*", "const cv::dnn::_Range*"]), _)]),
	void cv_dnn_slice_const_MatR_const__RangeR_const__RangeR_const__RangeR(const cv::Mat* m, const cv::dnn::_Range* r0, const cv::dnn::_Range* r1, const cv::dnn::_Range* r2, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::dnn::slice(*m, *r0, *r1, *r2);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// slice(const Mat &, const _Range &, const _Range &, const _Range &, const _Range &)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:95
	// ("cv::dnn::slice", vec![(pred!(mut, ["m", "r0", "r1", "r2", "r3"], ["const cv::Mat*", "const cv::dnn::_Range*", "const cv::dnn::_Range*", "const cv::dnn::_Range*", "const cv::dnn::_Range*"]), _)]),
	void cv_dnn_slice_const_MatR_const__RangeR_const__RangeR_const__RangeR_const__RangeR(const cv::Mat* m, const cv::dnn::_Range* r0, const cv::dnn::_Range* r1, const cv::dnn::_Range* r2, const cv::dnn::_Range* r3, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::dnn::slice(*m, *r0, *r1, *r2, *r3);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::total(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:184
	// ("cv::dnn::total", vec![(pred!(mut, ["mat"], ["const cv::Mat*"]), _)]),
	void cv_dnn_total_const_MatR(const cv::Mat* mat, Result<int>* ocvrs_return) {
		try {
			int ret = cv::dnn::total(*mat);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// total(const Mat &, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:184
	// ("cv::dnn::total", vec![(pred!(mut, ["mat", "start", "end"], ["const cv::Mat*", "int", "int"]), _)]),
	void cv_dnn_total_const_MatR_int_int(const cv::Mat* mat, int start, int end, Result<int>* ocvrs_return) {
		try {
			int ret = cv::dnn::total(*mat, start, end);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::total(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:161
	// ("cv::dnn::total", vec![(pred!(mut, ["shape"], ["const cv::dnn::MatShape*"]), _)]),
	void cv_dnn_total_const_MatShapeR(const cv::dnn::MatShape* shape, Result<int>* ocvrs_return) {
		try {
			int ret = cv::dnn::total(*shape);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// total(const MatShape &, int, int)(CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:161
	// ("cv::dnn::total", vec![(pred!(mut, ["shape", "start", "end"], ["const cv::dnn::MatShape*", "int", "int"]), _)]),
	void cv_dnn_total_const_MatShapeR_int_int(const cv::dnn::MatShape* shape, int start, int end, Result<int>* ocvrs_return) {
		try {
			int ret = cv::dnn::total(*shape, start, end);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writeTextGraph(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1061
	// ("cv::dnn::writeTextGraph", vec![(pred!(mut, ["model", "output"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_dnn_writeTextGraph_const_StringR_const_StringR(const char* model, const char* output, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::writeTextGraph(cv::String(model), cv::String(output));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:494
	// ("cv::dnn::AbsLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_AbsLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::AbsLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::AbsLayer> ret = cv::dnn::AbsLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::AbsLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:583
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

	// forwardSlice(const float *, float *, int, size_t, int, int)(VariableArray, VariableArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:427
	// ("cv::dnn::ActivationLayer::forwardSlice", vec![(pred!(const, ["src", "dst", "len", "outPlaneSize", "cn0", "cn1"], ["const float*", "float*", "int", "size_t", "int", "int"]), _)]),
	void cv_dnn_ActivationLayer_forwardSlice_const_const_floatX_floatX_int_size_t_int_int(const cv::dnn::ActivationLayer* instance, const float* src, float* dst, int len, size_t outPlaneSize, int cn0, int cn1, ResultVoid* ocvrs_return) {
		try {
			instance->forwardSlice(src, dst, len, outPlaneSize, cn0, cn1);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ActivationLayer::to_AbsLayer() generated
	// ("cv::dnn::ActivationLayer::to_AbsLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::AbsLayer* cv_dnn_ActivationLayer_to_AbsLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::AbsLayer*>(instance);
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

	// cv::dnn::ActivationLayer::to_ChannelsPReLULayer() generated
	// ("cv::dnn::ActivationLayer::to_ChannelsPReLULayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ChannelsPReLULayer* cv_dnn_ActivationLayer_to_ChannelsPReLULayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::ChannelsPReLULayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_ELULayer() generated
	// ("cv::dnn::ActivationLayer::to_ELULayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ELULayer* cv_dnn_ActivationLayer_to_ELULayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::ELULayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_ExpLayer() generated
	// ("cv::dnn::ActivationLayer::to_ExpLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ExpLayer* cv_dnn_ActivationLayer_to_ExpLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::ExpLayer*>(instance);
	}

	// cv::dnn::ActivationLayer::to_MishLayer() generated
	// ("cv::dnn::ActivationLayer::to_MishLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::MishLayer* cv_dnn_ActivationLayer_to_MishLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::MishLayer*>(instance);
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

	// cv::dnn::ActivationLayer::to_SigmoidLayer() generated
	// ("cv::dnn::ActivationLayer::to_SigmoidLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SigmoidLayer* cv_dnn_ActivationLayer_to_SigmoidLayer(cv::dnn::ActivationLayer* instance) {
			return dynamic_cast<cv::dnn::SigmoidLayer*>(instance);
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:488
	// ("cv::dnn::BNLLLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_BNLLLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::BNLLLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::BNLLLayer> ret = cv::dnn::BNLLLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::BNLLLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
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

	// cv::dnn::BackendNode::backendId() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:132
	// ("cv::dnn::BackendNode::backendId", vec![(pred!(const, [], []), _)]),
	int cv_dnn_BackendNode_propBackendId_const(const cv::dnn::BackendNode* instance) {
			int ret = instance->backendId;
			return ret;
	}

	// cv::dnn::BackendNode::setBackendId(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:132
	// ("cv::dnn::BackendNode::setBackendId", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_BackendNode_propBackendId_const_int(cv::dnn::BackendNode* instance, const int val) {
			instance->backendId = val;
	}

	// cv::dnn::BackendNode::delete() generated
	// ("cv::dnn::BackendNode::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_BackendNode_delete(cv::dnn::BackendNode* instance) {
			delete instance;
	}

	// copyToHost()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:169
	// ("cv::dnn::BackendWrapper::copyToHost", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_BackendWrapper_copyToHost(cv::dnn::BackendWrapper* instance, ResultVoid* ocvrs_return) {
		try {
			instance->copyToHost();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setHostDirty()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:174
	// ("cv::dnn::BackendWrapper::setHostDirty", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_BackendWrapper_setHostDirty(cv::dnn::BackendWrapper* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setHostDirty();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::BackendWrapper::backendId() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:176
	// ("cv::dnn::BackendWrapper::backendId", vec![(pred!(const, [], []), _)]),
	int cv_dnn_BackendWrapper_propBackendId_const(const cv::dnn::BackendWrapper* instance) {
			int ret = instance->backendId;
			return ret;
	}

	// cv::dnn::BackendWrapper::setBackendId(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:176
	// ("cv::dnn::BackendWrapper::setBackendId", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_BackendWrapper_propBackendId_const_int(cv::dnn::BackendWrapper* instance, const int val) {
			instance->backendId = val;
	}

	// cv::dnn::BackendWrapper::targetId() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:177
	// ("cv::dnn::BackendWrapper::targetId", vec![(pred!(const, [], []), _)]),
	int cv_dnn_BackendWrapper_propTargetId_const(const cv::dnn::BackendWrapper* instance) {
			int ret = instance->targetId;
			return ret;
	}

	// cv::dnn::BackendWrapper::setTargetId(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:177
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

	// cv::dnn::BaseConvolutionLayer::kernel() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:213
	// ("cv::dnn::BaseConvolutionLayer::kernel", vec![(pred!(const, [], []), _)]),
	void cv_dnn_BaseConvolutionLayer_propKernel_const(const cv::dnn::BaseConvolutionLayer* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->kernel;
			*ocvrs_return = ret;
	}

	// cv::dnn::BaseConvolutionLayer::setKernel(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:213
	// ("cv::dnn::BaseConvolutionLayer::setKernel", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_dnn_BaseConvolutionLayer_propKernel_const_Size(cv::dnn::BaseConvolutionLayer* instance, const cv::Size* val) {
			instance->kernel = *val;
	}

	// cv::dnn::BaseConvolutionLayer::stride() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:213
	// ("cv::dnn::BaseConvolutionLayer::stride", vec![(pred!(const, [], []), _)]),
	void cv_dnn_BaseConvolutionLayer_propStride_const(const cv::dnn::BaseConvolutionLayer* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->stride;
			*ocvrs_return = ret;
	}

	// cv::dnn::BaseConvolutionLayer::setStride(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:213
	// ("cv::dnn::BaseConvolutionLayer::setStride", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_dnn_BaseConvolutionLayer_propStride_const_Size(cv::dnn::BaseConvolutionLayer* instance, const cv::Size* val) {
			instance->stride = *val;
	}

	// cv::dnn::BaseConvolutionLayer::pad() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:213
	// ("cv::dnn::BaseConvolutionLayer::pad", vec![(pred!(const, [], []), _)]),
	void cv_dnn_BaseConvolutionLayer_propPad_const(const cv::dnn::BaseConvolutionLayer* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->pad;
			*ocvrs_return = ret;
	}

	// cv::dnn::BaseConvolutionLayer::setPad(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:213
	// ("cv::dnn::BaseConvolutionLayer::setPad", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_dnn_BaseConvolutionLayer_propPad_const_Size(cv::dnn::BaseConvolutionLayer* instance, const cv::Size* val) {
			instance->pad = *val;
	}

	// cv::dnn::BaseConvolutionLayer::dilation() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:213
	// ("cv::dnn::BaseConvolutionLayer::dilation", vec![(pred!(const, [], []), _)]),
	void cv_dnn_BaseConvolutionLayer_propDilation_const(const cv::dnn::BaseConvolutionLayer* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->dilation;
			*ocvrs_return = ret;
	}

	// cv::dnn::BaseConvolutionLayer::setDilation(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:213
	// ("cv::dnn::BaseConvolutionLayer::setDilation", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_dnn_BaseConvolutionLayer_propDilation_const_Size(cv::dnn::BaseConvolutionLayer* instance, const cv::Size* val) {
			instance->dilation = *val;
	}

	// cv::dnn::BaseConvolutionLayer::adjustPad() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:213
	// ("cv::dnn::BaseConvolutionLayer::adjustPad", vec![(pred!(const, [], []), _)]),
	void cv_dnn_BaseConvolutionLayer_propAdjustPad_const(const cv::dnn::BaseConvolutionLayer* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->adjustPad;
			*ocvrs_return = ret;
	}

	// cv::dnn::BaseConvolutionLayer::setAdjustPad(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:213
	// ("cv::dnn::BaseConvolutionLayer::setAdjustPad", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_dnn_BaseConvolutionLayer_propAdjustPad_const_Size(cv::dnn::BaseConvolutionLayer* instance, const cv::Size* val) {
			instance->adjustPad = *val;
	}

	// cv::dnn::BaseConvolutionLayer::adjust_pads() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:214
	// ("cv::dnn::BaseConvolutionLayer::adjust_pads", vec![(pred!(const, [], []), _)]),
	std::vector<size_t>* cv_dnn_BaseConvolutionLayer_propAdjust_pads_const(const cv::dnn::BaseConvolutionLayer* instance) {
			std::vector<size_t> ret = instance->adjust_pads;
			return new std::vector<size_t>(ret);
	}

	// cv::dnn::BaseConvolutionLayer::setAdjust_pads(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:214
	// ("cv::dnn::BaseConvolutionLayer::setAdjust_pads", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
	void cv_dnn_BaseConvolutionLayer_propAdjust_pads_const_vectorLsize_tG(cv::dnn::BaseConvolutionLayer* instance, const std::vector<size_t>* val) {
			instance->adjust_pads = *val;
	}

	// cv::dnn::BaseConvolutionLayer::kernel_size() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:215
	// ("cv::dnn::BaseConvolutionLayer::kernel_size", vec![(pred!(const, [], []), _)]),
	std::vector<size_t>* cv_dnn_BaseConvolutionLayer_propKernel_size_const(const cv::dnn::BaseConvolutionLayer* instance) {
			std::vector<size_t> ret = instance->kernel_size;
			return new std::vector<size_t>(ret);
	}

	// cv::dnn::BaseConvolutionLayer::setKernel_size(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:215
	// ("cv::dnn::BaseConvolutionLayer::setKernel_size", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
	void cv_dnn_BaseConvolutionLayer_propKernel_size_const_vectorLsize_tG(cv::dnn::BaseConvolutionLayer* instance, const std::vector<size_t>* val) {
			instance->kernel_size = *val;
	}

	// cv::dnn::BaseConvolutionLayer::strides() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:215
	// ("cv::dnn::BaseConvolutionLayer::strides", vec![(pred!(const, [], []), _)]),
	std::vector<size_t>* cv_dnn_BaseConvolutionLayer_propStrides_const(const cv::dnn::BaseConvolutionLayer* instance) {
			std::vector<size_t> ret = instance->strides;
			return new std::vector<size_t>(ret);
	}

	// cv::dnn::BaseConvolutionLayer::setStrides(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:215
	// ("cv::dnn::BaseConvolutionLayer::setStrides", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
	void cv_dnn_BaseConvolutionLayer_propStrides_const_vectorLsize_tG(cv::dnn::BaseConvolutionLayer* instance, const std::vector<size_t>* val) {
			instance->strides = *val;
	}

	// cv::dnn::BaseConvolutionLayer::dilations() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:215
	// ("cv::dnn::BaseConvolutionLayer::dilations", vec![(pred!(const, [], []), _)]),
	std::vector<size_t>* cv_dnn_BaseConvolutionLayer_propDilations_const(const cv::dnn::BaseConvolutionLayer* instance) {
			std::vector<size_t> ret = instance->dilations;
			return new std::vector<size_t>(ret);
	}

	// cv::dnn::BaseConvolutionLayer::setDilations(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:215
	// ("cv::dnn::BaseConvolutionLayer::setDilations", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
	void cv_dnn_BaseConvolutionLayer_propDilations_const_vectorLsize_tG(cv::dnn::BaseConvolutionLayer* instance, const std::vector<size_t>* val) {
			instance->dilations = *val;
	}

	// cv::dnn::BaseConvolutionLayer::pads_begin() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:216
	// ("cv::dnn::BaseConvolutionLayer::pads_begin", vec![(pred!(const, [], []), _)]),
	std::vector<size_t>* cv_dnn_BaseConvolutionLayer_propPads_begin_const(const cv::dnn::BaseConvolutionLayer* instance) {
			std::vector<size_t> ret = instance->pads_begin;
			return new std::vector<size_t>(ret);
	}

	// cv::dnn::BaseConvolutionLayer::setPads_begin(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:216
	// ("cv::dnn::BaseConvolutionLayer::setPads_begin", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
	void cv_dnn_BaseConvolutionLayer_propPads_begin_const_vectorLsize_tG(cv::dnn::BaseConvolutionLayer* instance, const std::vector<size_t>* val) {
			instance->pads_begin = *val;
	}

	// cv::dnn::BaseConvolutionLayer::pads_end() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:216
	// ("cv::dnn::BaseConvolutionLayer::pads_end", vec![(pred!(const, [], []), _)]),
	std::vector<size_t>* cv_dnn_BaseConvolutionLayer_propPads_end_const(const cv::dnn::BaseConvolutionLayer* instance) {
			std::vector<size_t> ret = instance->pads_end;
			return new std::vector<size_t>(ret);
	}

	// cv::dnn::BaseConvolutionLayer::setPads_end(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:216
	// ("cv::dnn::BaseConvolutionLayer::setPads_end", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
	void cv_dnn_BaseConvolutionLayer_propPads_end_const_vectorLsize_tG(cv::dnn::BaseConvolutionLayer* instance, const std::vector<size_t>* val) {
			instance->pads_end = *val;
	}

	// cv::dnn::BaseConvolutionLayer::padMode() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:217
	// ("cv::dnn::BaseConvolutionLayer::padMode", vec![(pred!(const, [], []), _)]),
	void* cv_dnn_BaseConvolutionLayer_propPadMode_const(const cv::dnn::BaseConvolutionLayer* instance) {
			cv::String ret = instance->padMode;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::dnn::BaseConvolutionLayer::setPadMode(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:217
	// ("cv::dnn::BaseConvolutionLayer::setPadMode", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	void cv_dnn_BaseConvolutionLayer_propPadMode_const_String(cv::dnn::BaseConvolutionLayer* instance, const char* val) {
			instance->padMode = cv::String(val);
	}

	// cv::dnn::BaseConvolutionLayer::numOutput() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:218
	// ("cv::dnn::BaseConvolutionLayer::numOutput", vec![(pred!(const, [], []), _)]),
	int cv_dnn_BaseConvolutionLayer_propNumOutput_const(const cv::dnn::BaseConvolutionLayer* instance) {
			int ret = instance->numOutput;
			return ret;
	}

	// cv::dnn::BaseConvolutionLayer::setNumOutput(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:218
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:540
	// ("cv::dnn::BatchNormLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_BatchNormLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::BatchNormLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::BatchNormLayer> ret = cv::dnn::BatchNormLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::BatchNormLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::BatchNormLayer::hasWeights() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:537
	// ("cv::dnn::BatchNormLayer::hasWeights", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_BatchNormLayer_propHasWeights_const(const cv::dnn::BatchNormLayer* instance) {
			bool ret = instance->hasWeights;
			return ret;
	}

	// cv::dnn::BatchNormLayer::setHasWeights(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:537
	// ("cv::dnn::BatchNormLayer::setHasWeights", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_dnn_BatchNormLayer_propHasWeights_const_bool(cv::dnn::BatchNormLayer* instance, const bool val) {
			instance->hasWeights = val;
	}

	// cv::dnn::BatchNormLayer::hasBias() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:537
	// ("cv::dnn::BatchNormLayer::hasBias", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_BatchNormLayer_propHasBias_const(const cv::dnn::BatchNormLayer* instance) {
			bool ret = instance->hasBias;
			return ret;
	}

	// cv::dnn::BatchNormLayer::setHasBias(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:537
	// ("cv::dnn::BatchNormLayer::setHasBias", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_dnn_BatchNormLayer_propHasBias_const_bool(cv::dnn::BatchNormLayer* instance, const bool val) {
			instance->hasBias = val;
	}

	// cv::dnn::BatchNormLayer::epsilon() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:538
	// ("cv::dnn::BatchNormLayer::epsilon", vec![(pred!(const, [], []), _)]),
	float cv_dnn_BatchNormLayer_propEpsilon_const(const cv::dnn::BatchNormLayer* instance) {
			float ret = instance->epsilon;
			return ret;
	}

	// cv::dnn::BatchNormLayer::setEpsilon(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:538
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:77
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:450
	// ("cv::dnn::ChannelsPReLULayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ChannelsPReLULayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::Layer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::ChannelsPReLULayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::Layer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:322
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

	// cv::dnn::ConcatLayer::axis() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:313
	// ("cv::dnn::ConcatLayer::axis", vec![(pred!(const, [], []), _)]),
	int cv_dnn_ConcatLayer_propAxis_const(const cv::dnn::ConcatLayer* instance) {
			int ret = instance->axis;
			return ret;
	}

	// cv::dnn::ConcatLayer::setAxis(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:313
	// ("cv::dnn::ConcatLayer::setAxis", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_ConcatLayer_propAxis_const_int(cv::dnn::ConcatLayer* instance, const int val) {
			instance->axis = val;
	}

	// cv::dnn::ConcatLayer::padding() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:320
	// ("cv::dnn::ConcatLayer::padding", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_ConcatLayer_propPadding_const(const cv::dnn::ConcatLayer* instance) {
			bool ret = instance->padding;
			return ret;
	}

	// cv::dnn::ConcatLayer::setPadding(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:320
	// ("cv::dnn::ConcatLayer::setPadding", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_dnn_ConcatLayer_propPadding_const_bool(cv::dnn::ConcatLayer* instance, const bool val) {
			instance->padding = val;
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:86
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:224
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:577
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:689
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:518
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:571
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:230
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:621
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

	// has(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:122
	// ("cv::dnn::Dict::has", vec![(pred!(const, ["key"], ["const cv::String*"]), _)]),
	void cv_dnn_Dict_has_const_const_StringR(const cv::dnn::Dict* instance, const char* key, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->has(cv::String(key));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ptr(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:125
	// ("cv::dnn::Dict::ptr", vec![(pred!(mut, ["key"], ["const cv::String*"]), _)]),
	void cv_dnn_Dict_ptr_const_StringR(cv::dnn::Dict* instance, const char* key, Result<cv::dnn::DictValue*>* ocvrs_return) {
		try {
			cv::dnn::DictValue* ret = instance->ptr(cv::String(key));
			Ok(new cv::dnn::DictValue(*ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ptr(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:128
	// ("cv::dnn::Dict::ptr", vec![(pred!(const, ["key"], ["const cv::String*"]), _)]),
	void cv_dnn_Dict_ptr_const_const_StringR(const cv::dnn::Dict* instance, const char* key, Result<const cv::dnn::DictValue*>* ocvrs_return) {
		try {
			const cv::dnn::DictValue* ret = instance->ptr(cv::String(key));
			Ok(new const cv::dnn::DictValue(*ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// get(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:131
	// ("cv::dnn::Dict::get", vec![(pred!(const, ["key"], ["const cv::String*"]), _)]),
	void cv_dnn_Dict_get_const_const_StringR(const cv::dnn::Dict* instance, const char* key, Result<cv::dnn::DictValue*>* ocvrs_return) {
		try {
			const cv::dnn::DictValue ret = instance->get(cv::String(key));
			Ok(new const cv::dnn::DictValue(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::Dict::set(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:143
	// ("cv::dnn::Dict::set", vec![(pred!(mut, ["key", "value"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_dnn_Dict_set_const_cv_String_const_StringR_const_StringR(cv::dnn::Dict* instance, const char* key, const char* value, Result<void*>* ocvrs_return) {
		try {
			const cv::String ret = instance->set<cv::String>(cv::String(key), cv::String(value));
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::Dict::set(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:143
	// ("cv::dnn::Dict::set", vec![(pred!(mut, ["key", "value"], ["const cv::String*", "const cv::dnn::DictValue*"]), _)]),
	void cv_dnn_Dict_set_const_cv_dnn_DictValue_const_StringR_const_DictValueR(cv::dnn::Dict* instance, const char* key, const cv::dnn::DictValue* value, Result<cv::dnn::DictValue*>* ocvrs_return) {
		try {
			const cv::dnn::DictValue ret = instance->set<cv::dnn::DictValue>(cv::String(key), *value);
			Ok(new const cv::dnn::DictValue(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::Dict::set(InString, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:143
	// ("cv::dnn::Dict::set", vec![(pred!(mut, ["key", "value"], ["const cv::String*", "const double*"]), _)]),
	void cv_dnn_Dict_set_const_double_const_StringR_const_doubleR(cv::dnn::Dict* instance, const char* key, const double* value, Result<double>* ocvrs_return) {
		try {
			const double ret = instance->set<double>(cv::String(key), *value);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::Dict::set(InString, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:143
	// ("cv::dnn::Dict::set", vec![(pred!(mut, ["key", "value"], ["const cv::String*", "const int64_t*"]), _)]),
	void cv_dnn_Dict_set_const_int64_t_const_StringR_const_int64_tR(cv::dnn::Dict* instance, const char* key, const int64_t* value, Result<int64_t>* ocvrs_return) {
		try {
			const int64_t ret = instance->set<int64_t>(cv::String(key), *value);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// erase(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:146
	// ("cv::dnn::Dict::erase", vec![(pred!(mut, ["key"], ["const cv::String*"]), _)]),
	void cv_dnn_Dict_erase_const_StringR(cv::dnn::Dict* instance, const char* key, ResultVoid* ocvrs_return) {
		try {
			instance->erase(cv::String(key));
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

	// DictValue(const DictValue &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:62
	// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, ["r"], ["const cv::dnn::DictValue*"]), _)]),
	void cv_dnn_DictValue_DictValue_const_DictValueR(const cv::dnn::DictValue* r, Result<cv::dnn::DictValue*>* ocvrs_return) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(*r);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DictValue(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:63
	// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, ["i"], ["bool"]), _)]),
	void cv_dnn_DictValue_DictValue_bool(bool i, Result<cv::dnn::DictValue*>* ocvrs_return) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DictValue(int64)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:64
	// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, ["i"], ["int64_t"]), _)]),
	void cv_dnn_DictValue_DictValue_int64_t(int64_t i, Result<cv::dnn::DictValue*>* ocvrs_return) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DictValue::DictValue() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:64
	// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_DictValue_DictValue(Result<cv::dnn::DictValue*>* ocvrs_return) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DictValue(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:65
	// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, ["i"], ["int"]), _)]),
	void cv_dnn_DictValue_DictValue_int(int i, Result<cv::dnn::DictValue*>* ocvrs_return) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DictValue(unsigned int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:66
	// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, ["p"], ["unsigned int"]), _)]),
	void cv_dnn_DictValue_DictValue_unsigned_int(unsigned int p, Result<cv::dnn::DictValue*>* ocvrs_return) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(p);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DictValue(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:67
	// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, ["p"], ["double"]), _)]),
	void cv_dnn_DictValue_DictValue_double(double p, Result<cv::dnn::DictValue*>* ocvrs_return) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(p);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DictValue(const char *)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:69
	// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, ["s"], ["const char*"]), _)]),
	void cv_dnn_DictValue_DictValue_const_charX(const char* s, Result<cv::dnn::DictValue*>* ocvrs_return) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(s);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DictValue::get(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:79
	// ("cv::dnn::DictValue::get", vec![(pred!(const, ["idx"], ["int"]), _)]),
	void cv_dnn_DictValue_get_cv_String_const_int(const cv::dnn::DictValue* instance, int idx, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->get<cv::String>(idx);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DictValue::get() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:79
	// ("cv::dnn::DictValue::get", vec![(pred!(const, [], []), _)]),
	void cv_dnn_DictValue_get_cv_String_const(const cv::dnn::DictValue* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->get<cv::String>();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DictValue::get(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:79
	// ("cv::dnn::DictValue::get", vec![(pred!(const, ["idx"], ["int"]), _)]),
	void cv_dnn_DictValue_get_double_const_int(const cv::dnn::DictValue* instance, int idx, Result<double>* ocvrs_return) {
		try {
			double ret = instance->get<double>(idx);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DictValue::get() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:79
	// ("cv::dnn::DictValue::get", vec![(pred!(const, [], []), _)]),
	void cv_dnn_DictValue_get_double_const(const cv::dnn::DictValue* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->get<double>();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DictValue::get(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:79
	// ("cv::dnn::DictValue::get", vec![(pred!(const, ["idx"], ["int"]), _)]),
	void cv_dnn_DictValue_get_int_const_int(const cv::dnn::DictValue* instance, int idx, Result<int>* ocvrs_return) {
		try {
			int ret = instance->get<int>(idx);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DictValue::get() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:79
	// ("cv::dnn::DictValue::get", vec![(pred!(const, [], []), _)]),
	void cv_dnn_DictValue_get_int_const(const cv::dnn::DictValue* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->get<int>();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DictValue::get(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:79
	// ("cv::dnn::DictValue::get", vec![(pred!(const, ["idx"], ["int"]), _)]),
	void cv_dnn_DictValue_get_int64_t_const_int(const cv::dnn::DictValue* instance, int idx, Result<int64_t>* ocvrs_return) {
		try {
			int64_t ret = instance->get<int64_t>(idx);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DictValue::get() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:79
	// ("cv::dnn::DictValue::get", vec![(pred!(const, [], []), _)]),
	void cv_dnn_DictValue_get_int64_t_const(const cv::dnn::DictValue* instance, Result<int64_t>* ocvrs_return) {
		try {
			int64_t ret = instance->get<int64_t>();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// size()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:81
	// ("cv::dnn::DictValue::size", vec![(pred!(const, [], []), _)]),
	void cv_dnn_DictValue_size_const(const cv::dnn::DictValue* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->size();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isInt()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:83
	// ("cv::dnn::DictValue::isInt", vec![(pred!(const, [], []), _)]),
	void cv_dnn_DictValue_isInt_const(const cv::dnn::DictValue* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isInt();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isString()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:84
	// ("cv::dnn::DictValue::isString", vec![(pred!(const, [], []), _)]),
	void cv_dnn_DictValue_isString_const(const cv::dnn::DictValue* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isString();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isReal()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:85
	// ("cv::dnn::DictValue::isReal", vec![(pred!(const, [], []), _)]),
	void cv_dnn_DictValue_isReal_const(const cv::dnn::DictValue* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isReal();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getIntValue(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:87
	// ("cv::dnn::DictValue::getIntValue", vec![(pred!(const, ["idx"], ["int"]), _)]),
	void cv_dnn_DictValue_getIntValue_const_int(const cv::dnn::DictValue* instance, int idx, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getIntValue(idx);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DictValue::getIntValue() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:87
	// ("cv::dnn::DictValue::getIntValue", vec![(pred!(const, [], []), _)]),
	void cv_dnn_DictValue_getIntValue_const(const cv::dnn::DictValue* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getIntValue();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRealValue(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:88
	// ("cv::dnn::DictValue::getRealValue", vec![(pred!(const, ["idx"], ["int"]), _)]),
	void cv_dnn_DictValue_getRealValue_const_int(const cv::dnn::DictValue* instance, int idx, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getRealValue(idx);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DictValue::getRealValue() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:88
	// ("cv::dnn::DictValue::getRealValue", vec![(pred!(const, [], []), _)]),
	void cv_dnn_DictValue_getRealValue_const(const cv::dnn::DictValue* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getRealValue();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getStringValue(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:89
	// ("cv::dnn::DictValue::getStringValue", vec![(pred!(const, ["idx"], ["int"]), _)]),
	void cv_dnn_DictValue_getStringValue_const_int(const cv::dnn::DictValue* instance, int idx, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getStringValue(idx);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::DictValue::getStringValue() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:89
	// ("cv::dnn::DictValue::getStringValue", vec![(pred!(const, [], []), _)]),
	void cv_dnn_DictValue_getStringValue_const(const cv::dnn::DictValue* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getStringValue();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator=(const DictValue &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:91
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:458
	// ("cv::dnn::ELULayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ELULayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::ELULayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::ELULayer> ret = cv::dnn::ELULayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::ELULayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ELULayer::alpha() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:456
	// ("cv::dnn::ELULayer::alpha", vec![(pred!(const, [], []), _)]),
	float cv_dnn_ELULayer_propAlpha_const(const cv::dnn::ELULayer* instance) {
			float ret = instance->alpha;
			return ret;
	}

	// cv::dnn::ELULayer::setAlpha(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:456
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:531
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:510
	// ("cv::dnn::ExpLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ExpLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::ExpLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::ExpLayer> ret = cv::dnn::ExpLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::ExpLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ExpLayer::base() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:508
	// ("cv::dnn::ExpLayer::base", vec![(pred!(const, [], []), _)]),
	float cv_dnn_ExpLayer_propBase_const(const cv::dnn::ExpLayer* instance) {
			float ret = instance->base;
			return ret;
	}

	// cv::dnn::ExpLayer::setBase(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:508
	// ("cv::dnn::ExpLayer::setBase", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_ExpLayer_propBase_const_float(cv::dnn::ExpLayer* instance, const float val) {
			instance->base = val;
	}

	// cv::dnn::ExpLayer::scale() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:508
	// ("cv::dnn::ExpLayer::scale", vec![(pred!(const, [], []), _)]),
	float cv_dnn_ExpLayer_propScale_const(const cv::dnn::ExpLayer* instance) {
			float ret = instance->scale;
			return ret;
	}

	// cv::dnn::ExpLayer::setScale(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:508
	// ("cv::dnn::ExpLayer::setScale", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_ExpLayer_propScale_const_float(cv::dnn::ExpLayer* instance, const float val) {
			instance->scale = val;
	}

	// cv::dnn::ExpLayer::shift() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:508
	// ("cv::dnn::ExpLayer::shift", vec![(pred!(const, [], []), _)]),
	float cv_dnn_ExpLayer_propShift_const(const cv::dnn::ExpLayer* instance) {
			float ret = instance->shift;
			return ret;
	}

	// cv::dnn::ExpLayer::setShift(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:508
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:307
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:589
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:281
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

	// cv::dnn::InnerProductLayer::axis() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:280
	// ("cv::dnn::InnerProductLayer::axis", vec![(pred!(const, [], []), _)]),
	int cv_dnn_InnerProductLayer_propAxis_const(const cv::dnn::InnerProductLayer* instance) {
			int ret = instance->axis;
			return ret;
	}

	// cv::dnn::InnerProductLayer::setAxis(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:280
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:677
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:242
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

	// cv::dnn::LRNLayer::type() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:236
	// ("cv::dnn::LRNLayer::type", vec![(pred!(const, [], []), _)]),
	int cv_dnn_LRNLayer_propType_const(const cv::dnn::LRNLayer* instance) {
			int ret = instance->type;
			return ret;
	}

	// cv::dnn::LRNLayer::setType(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:236
	// ("cv::dnn::LRNLayer::setType", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_LRNLayer_propType_const_int(cv::dnn::LRNLayer* instance, const int val) {
			instance->type = val;
	}

	// cv::dnn::LRNLayer::size() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:238
	// ("cv::dnn::LRNLayer::size", vec![(pred!(const, [], []), _)]),
	int cv_dnn_LRNLayer_propSize_const(const cv::dnn::LRNLayer* instance) {
			int ret = instance->size;
			return ret;
	}

	// cv::dnn::LRNLayer::setSize(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:238
	// ("cv::dnn::LRNLayer::setSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_LRNLayer_propSize_const_int(cv::dnn::LRNLayer* instance, const int val) {
			instance->size = val;
	}

	// cv::dnn::LRNLayer::alpha() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:239
	// ("cv::dnn::LRNLayer::alpha", vec![(pred!(const, [], []), _)]),
	float cv_dnn_LRNLayer_propAlpha_const(const cv::dnn::LRNLayer* instance) {
			float ret = instance->alpha;
			return ret;
	}

	// cv::dnn::LRNLayer::setAlpha(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:239
	// ("cv::dnn::LRNLayer::setAlpha", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_LRNLayer_propAlpha_const_float(cv::dnn::LRNLayer* instance, const float val) {
			instance->alpha = val;
	}

	// cv::dnn::LRNLayer::beta() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:239
	// ("cv::dnn::LRNLayer::beta", vec![(pred!(const, [], []), _)]),
	float cv_dnn_LRNLayer_propBeta_const(const cv::dnn::LRNLayer* instance) {
			float ret = instance->beta;
			return ret;
	}

	// cv::dnn::LRNLayer::setBeta(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:239
	// ("cv::dnn::LRNLayer::setBeta", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_LRNLayer_propBeta_const_float(cv::dnn::LRNLayer* instance, const float val) {
			instance->beta = val;
	}

	// cv::dnn::LRNLayer::bias() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:239
	// ("cv::dnn::LRNLayer::bias", vec![(pred!(const, [], []), _)]),
	float cv_dnn_LRNLayer_propBias_const(const cv::dnn::LRNLayer* instance) {
			float ret = instance->bias;
			return ret;
	}

	// cv::dnn::LRNLayer::setBias(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:239
	// ("cv::dnn::LRNLayer::setBias", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_LRNLayer_propBias_const_float(cv::dnn::LRNLayer* instance, const float val) {
			instance->bias = val;
	}

	// cv::dnn::LRNLayer::normBySize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:240
	// ("cv::dnn::LRNLayer::normBySize", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_LRNLayer_propNormBySize_const(const cv::dnn::LRNLayer* instance) {
			bool ret = instance->normBySize;
			return ret;
	}

	// cv::dnn::LRNLayer::setNormBySize(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:240
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:94
	// ("cv::dnn::LSTMLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_LSTMLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::LSTMLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::LSTMLayer> ret = cv::dnn::LSTMLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::LSTMLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWeights(const Mat &, const Mat &, const Mat &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:128
	// ("cv::dnn::LSTMLayer::setWeights", vec![(pred!(mut, ["Wh", "Wx", "b"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_dnn_LSTMLayer_setWeights_const_MatR_const_MatR_const_MatR(cv::dnn::LSTMLayer* instance, const cv::Mat* Wh, const cv::Mat* Wx, const cv::Mat* b, ResultVoid* ocvrs_return) {
		try {
			instance->setWeights(*Wh, *Wx, *b);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setOutShape(const MatShape &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:134
	// ("cv::dnn::LSTMLayer::setOutShape", vec![(pred!(mut, ["outTailShape"], ["const cv::dnn::MatShape*"]), _)]),
	void cv_dnn_LSTMLayer_setOutShape_const_MatShapeR(cv::dnn::LSTMLayer* instance, const cv::dnn::MatShape* outTailShape, ResultVoid* ocvrs_return) {
		try {
			instance->setOutShape(*outTailShape);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::LSTMLayer::setOutShape() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:134
	// ("cv::dnn::LSTMLayer::setOutShape", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_LSTMLayer_setOutShape(cv::dnn::LSTMLayer* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setOutShape();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseTimstampsDim(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:145
	// ("cv::dnn::LSTMLayer::setUseTimstampsDim", vec![(pred!(mut, ["use"], ["bool"]), _)]),
	void cv_dnn_LSTMLayer_setUseTimstampsDim_bool(cv::dnn::LSTMLayer* instance, bool use, ResultVoid* ocvrs_return) {
		try {
			instance->setUseTimstampsDim(use);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::LSTMLayer::setUseTimstampsDim() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:145
	// ("cv::dnn::LSTMLayer::setUseTimstampsDim", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_LSTMLayer_setUseTimstampsDim(cv::dnn::LSTMLayer* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setUseTimstampsDim();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setProduceCellOutput(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:151
	// ("cv::dnn::LSTMLayer::setProduceCellOutput", vec![(pred!(mut, ["produce"], ["bool"]), _)]),
	void cv_dnn_LSTMLayer_setProduceCellOutput_bool(cv::dnn::LSTMLayer* instance, bool produce, ResultVoid* ocvrs_return) {
		try {
			instance->setProduceCellOutput(produce);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::LSTMLayer::setProduceCellOutput() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:151
	// ("cv::dnn::LSTMLayer::setProduceCellOutput", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_LSTMLayer_setProduceCellOutput(cv::dnn::LSTMLayer* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setProduceCellOutput();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// inputNameToIndex(String)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:164
	// ("cv::dnn::LSTMLayer::inputNameToIndex", vec![(pred!(mut, ["inputName"], ["cv::String"]), _)]),
	void cv_dnn_LSTMLayer_inputNameToIndex_String(cv::dnn::LSTMLayer* instance, const char* inputName, Result<int>* ocvrs_return) {
		try {
			int ret = instance->inputNameToIndex(cv::String(inputName));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// outputNameToIndex(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:165
	// ("cv::dnn::LSTMLayer::outputNameToIndex", vec![(pred!(mut, ["outputName"], ["const cv::String*"]), _)]),
	void cv_dnn_LSTMLayer_outputNameToIndex_const_StringR(cv::dnn::LSTMLayer* instance, const char* outputName, Result<int>* ocvrs_return) {
		try {
			int ret = instance->outputNameToIndex(cv::String(outputName));
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

	// finalize(InputArrayOfArrays, OutputArrayOfArrays)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:212
	// ("cv::dnn::Layer::finalize", vec![(pred!(mut, ["inputs", "outputs"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_dnn_Layer_finalize_const__InputArrayR_const__OutputArrayR(cv::dnn::Layer* instance, const cv::_InputArray* inputs, const cv::_OutputArray* outputs, ResultVoid* ocvrs_return) {
		try {
			instance->finalize(*inputs, *outputs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// forward(std::vector<Mat *> &, std::vector<Mat> &, std::vector<Mat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:221
	// ("cv::dnn::Layer::forward", vec![(pred!(mut, ["input", "output", "internals"], ["std::vector<cv::Mat*>*", "std::vector<cv::Mat>*", "std::vector<cv::Mat>*"]), _)]),
	void cv_dnn_Layer_forward_vectorLMatXGR_vectorLMatGR_vectorLMatGR(cv::dnn::Layer* instance, std::vector<cv::Mat*>* input, std::vector<cv::Mat>* output, std::vector<cv::Mat>* internals, ResultVoid* ocvrs_return) {
		try {
			instance->forward(*input, *output, *internals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// forward(InputArrayOfArrays, OutputArrayOfArrays, OutputArrayOfArrays)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:228
	// ("cv::dnn::Layer::forward", vec![(pred!(mut, ["inputs", "outputs", "internals"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_dnn_Layer_forward_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::dnn::Layer* instance, const cv::_InputArray* inputs, const cv::_OutputArray* outputs, const cv::_OutputArray* internals, ResultVoid* ocvrs_return) {
		try {
			instance->forward(*inputs, *outputs, *internals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// forward_fallback(InputArrayOfArrays, OutputArrayOfArrays, OutputArrayOfArrays)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:235
	// ("cv::dnn::Layer::forward_fallback", vec![(pred!(mut, ["inputs", "outputs", "internals"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_dnn_Layer_forward_fallback_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::dnn::Layer* instance, const cv::_InputArray* inputs, const cv::_OutputArray* outputs, const cv::_OutputArray* internals, ResultVoid* ocvrs_return) {
		try {
			instance->forward_fallback(*inputs, *outputs, *internals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// finalize(const std::vector<Mat> &, std::vector<Mat> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:242
	// ("cv::dnn::Layer::finalize", vec![(pred!(mut, ["inputs", "outputs"], ["const std::vector<cv::Mat>*", "std::vector<cv::Mat>*"]), _)]),
	void cv_dnn_Layer_finalize_const_vectorLMatGR_vectorLMatGR(cv::dnn::Layer* instance, const std::vector<cv::Mat>* inputs, std::vector<cv::Mat>* outputs, ResultVoid* ocvrs_return) {
		try {
			instance->finalize(*inputs, *outputs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// finalize(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:248
	// ("cv::dnn::Layer::finalize", vec![(pred!(mut, ["inputs"], ["const std::vector<cv::Mat>*"]), _)]),
	void cv_dnn_Layer_finalize_const_vectorLMatGR(cv::dnn::Layer* instance, const std::vector<cv::Mat>* inputs, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			std::vector<cv::Mat> ret = instance->finalize(*inputs);
			Ok(new std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// run(const std::vector<Mat> &, std::vector<Mat> &, std::vector<Mat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:253
	// ("cv::dnn::Layer::run", vec![(pred!(mut, ["inputs", "outputs", "internals"], ["const std::vector<cv::Mat>*", "std::vector<cv::Mat>*", "std::vector<cv::Mat>*"]), _)]),
	void cv_dnn_Layer_run_const_vectorLMatGR_vectorLMatGR_vectorLMatGR(cv::dnn::Layer* instance, const std::vector<cv::Mat>* inputs, std::vector<cv::Mat>* outputs, std::vector<cv::Mat>* internals, ResultVoid* ocvrs_return) {
		try {
			instance->run(*inputs, *outputs, *internals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// inputNameToIndex(String)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:262
	// ("cv::dnn::Layer::inputNameToIndex", vec![(pred!(mut, ["inputName"], ["cv::String"]), _)]),
	void cv_dnn_Layer_inputNameToIndex_String(cv::dnn::Layer* instance, const char* inputName, Result<int>* ocvrs_return) {
		try {
			int ret = instance->inputNameToIndex(cv::String(inputName));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// outputNameToIndex(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:266
	// ("cv::dnn::Layer::outputNameToIndex", vec![(pred!(mut, ["outputName"], ["const cv::String*"]), _)]),
	void cv_dnn_Layer_outputNameToIndex_const_StringR(cv::dnn::Layer* instance, const char* outputName, Result<int>* ocvrs_return) {
		try {
			int ret = instance->outputNameToIndex(cv::String(outputName));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// supportBackend(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:273
	// ("cv::dnn::Layer::supportBackend", vec![(pred!(mut, ["backendId"], ["int"]), _)]),
	void cv_dnn_Layer_supportBackend_int(cv::dnn::Layer* instance, int backendId, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->supportBackend(backendId);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initHalide(const std::vector<Ptr<BackendWrapper>> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:285
	// ("cv::dnn::Layer::initHalide", vec![(pred!(mut, ["inputs"], ["const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>*"]), _)]),
	void cv_dnn_Layer_initHalide_const_vectorLPtrLBackendWrapperGGR(cv::dnn::Layer* instance, const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* inputs, Result<cv::Ptr<cv::dnn::BackendNode>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::BackendNode> ret = instance->initHalide(*inputs);
			Ok(new cv::Ptr<cv::dnn::BackendNode>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initNgraph(const std::vector<Ptr<BackendWrapper>> &, const std::vector<Ptr<BackendNode>> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:287
	// ("cv::dnn::Layer::initNgraph", vec![(pred!(mut, ["inputs", "nodes"], ["const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>*", "const std::vector<cv::Ptr<cv::dnn::BackendNode>>*"]), _)]),
	void cv_dnn_Layer_initNgraph_const_vectorLPtrLBackendWrapperGGR_const_vectorLPtrLBackendNodeGGR(cv::dnn::Layer* instance, const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* inputs, const std::vector<cv::Ptr<cv::dnn::BackendNode>>* nodes, Result<cv::Ptr<cv::dnn::BackendNode>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::BackendNode> ret = instance->initNgraph(*inputs, *nodes);
			Ok(new cv::Ptr<cv::dnn::BackendNode>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// applyHalideScheduler(Ptr<BackendNode> &, const std::vector<Mat *> &, const std::vector<Mat> &, int)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:300
	// ("cv::dnn::Layer::applyHalideScheduler", vec![(pred!(const, ["node", "inputs", "outputs", "targetId"], ["cv::Ptr<cv::dnn::BackendNode>*", "const std::vector<cv::Mat*>*", "const std::vector<cv::Mat>*", "int"]), _)]),
	void cv_dnn_Layer_applyHalideScheduler_const_PtrLBackendNodeGR_const_vectorLMatXGR_const_vectorLMatGR_int(const cv::dnn::Layer* instance, cv::Ptr<cv::dnn::BackendNode>* node, const std::vector<cv::Mat*>* inputs, const std::vector<cv::Mat>* outputs, int targetId, ResultVoid* ocvrs_return) {
		try {
			instance->applyHalideScheduler(*node, *inputs, *outputs, targetId);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// tryAttach(const Ptr<BackendNode> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:314
	// ("cv::dnn::Layer::tryAttach", vec![(pred!(mut, ["node"], ["const cv::Ptr<cv::dnn::BackendNode>*"]), _)]),
	void cv_dnn_Layer_tryAttach_const_PtrLBackendNodeGR(cv::dnn::Layer* instance, const cv::Ptr<cv::dnn::BackendNode>* node, Result<cv::Ptr<cv::dnn::BackendNode>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::BackendNode> ret = instance->tryAttach(*node);
			Ok(new cv::Ptr<cv::dnn::BackendNode>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setActivation(const Ptr<ActivationLayer> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:322
	// ("cv::dnn::Layer::setActivation", vec![(pred!(mut, ["layer"], ["const cv::Ptr<cv::dnn::ActivationLayer>*"]), _)]),
	void cv_dnn_Layer_setActivation_const_PtrLActivationLayerGR(cv::dnn::Layer* instance, const cv::Ptr<cv::dnn::ActivationLayer>* layer, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->setActivation(*layer);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// tryFuse(Ptr<Layer> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:329
	// ("cv::dnn::Layer::tryFuse", vec![(pred!(mut, ["top"], ["cv::Ptr<cv::dnn::Layer>*"]), _)]),
	void cv_dnn_Layer_tryFuse_PtrLLayerGR(cv::dnn::Layer* instance, cv::Ptr<cv::dnn::Layer>* top, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->tryFuse(*top);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScaleShift(Mat &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:344
	// ("cv::dnn::Layer::getScaleShift", vec![(pred!(const, ["scale", "shift"], ["cv::Mat*", "cv::Mat*"]), _)]),
	void cv_dnn_Layer_getScaleShift_const_MatR_MatR(const cv::dnn::Layer* instance, cv::Mat* scale, cv::Mat* shift, ResultVoid* ocvrs_return) {
		try {
			instance->getScaleShift(*scale, *shift);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// unsetAttached()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:349
	// ("cv::dnn::Layer::unsetAttached", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_Layer_unsetAttached(cv::dnn::Layer* instance, ResultVoid* ocvrs_return) {
		try {
			instance->unsetAttached();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMemoryShapes(const std::vector<MatShape> &, const int, std::vector<MatShape> &, std::vector<MatShape> &)(CppPassByVoidPtr, Primitive, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:351
	// ("cv::dnn::Layer::getMemoryShapes", vec![(pred!(const, ["inputs", "requiredOutputs", "outputs", "internals"], ["const std::vector<cv::dnn::MatShape>*", "const int", "std::vector<cv::dnn::MatShape>*", "std::vector<cv::dnn::MatShape>*"]), _)]),
	void cv_dnn_Layer_getMemoryShapes_const_const_vectorLMatShapeGR_const_int_vectorLMatShapeGR_vectorLMatShapeGR(const cv::dnn::Layer* instance, const std::vector<cv::dnn::MatShape>* inputs, const int requiredOutputs, std::vector<cv::dnn::MatShape>* outputs, std::vector<cv::dnn::MatShape>* internals, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getMemoryShapes(*inputs, requiredOutputs, *outputs, *internals);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFLOPS(const std::vector<MatShape> &, const std::vector<MatShape> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:356
	// ("cv::dnn::Layer::getFLOPS", vec![(pred!(const, ["inputs", "outputs"], ["const std::vector<cv::dnn::MatShape>*", "const std::vector<cv::dnn::MatShape>*"]), _)]),
	void cv_dnn_Layer_getFLOPS_const_const_vectorLMatShapeGR_const_vectorLMatShapeGR(const cv::dnn::Layer* instance, const std::vector<cv::dnn::MatShape>* inputs, const std::vector<cv::dnn::MatShape>* outputs, Result<int64_t>* ocvrs_return) {
		try {
			int64_t ret = instance->getFLOPS(*inputs, *outputs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// updateMemoryShapes(const std::vector<MatShape> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:359
	// ("cv::dnn::Layer::updateMemoryShapes", vec![(pred!(mut, ["inputs"], ["const std::vector<cv::dnn::MatShape>*"]), _)]),
	void cv_dnn_Layer_updateMemoryShapes_const_vectorLMatShapeGR(cv::dnn::Layer* instance, const std::vector<cv::dnn::MatShape>* inputs, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->updateMemoryShapes(*inputs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Layer()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:365
	// ("cv::dnn::Layer::Layer", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_Layer_Layer(Result<cv::dnn::Layer*>* ocvrs_return) {
		try {
			cv::dnn::Layer* ret = new cv::dnn::Layer();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Layer(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:366
	// ("cv::dnn::Layer::Layer", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_Layer_Layer_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::dnn::Layer*>* ocvrs_return) {
		try {
			cv::dnn::Layer* ret = new cv::dnn::Layer(*params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setParamsFrom(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:367
	// ("cv::dnn::Layer::setParamsFrom", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_Layer_setParamsFrom_const_LayerParamsR(cv::dnn::Layer* instance, const cv::dnn::LayerParams* params, ResultVoid* ocvrs_return) {
		try {
			instance->setParamsFrom(*params);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::Layer::blobs() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:192
	// ("cv::dnn::Layer::blobs", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Mat>* cv_dnn_Layer_propBlobs_const(const cv::dnn::Layer* instance) {
			std::vector<cv::Mat> ret = instance->blobs;
			return new std::vector<cv::Mat>(ret);
	}

	// cv::dnn::Layer::setBlobs(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:192
	// ("cv::dnn::Layer::setBlobs", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	void cv_dnn_Layer_propBlobs_const_vectorLMatG(cv::dnn::Layer* instance, const std::vector<cv::Mat>* val) {
			instance->blobs = *val;
	}

	// cv::dnn::Layer::name() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:361
	// ("cv::dnn::Layer::name", vec![(pred!(const, [], []), _)]),
	void* cv_dnn_Layer_propName_const(const cv::dnn::Layer* instance) {
			cv::String ret = instance->name;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::dnn::Layer::setName(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:361
	// ("cv::dnn::Layer::setName", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	void cv_dnn_Layer_propName_const_String(cv::dnn::Layer* instance, const char* val) {
			instance->name = cv::String(val);
	}

	// cv::dnn::Layer::type() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:362
	// ("cv::dnn::Layer::type", vec![(pred!(const, [], []), _)]),
	void* cv_dnn_Layer_propType_const(const cv::dnn::Layer* instance) {
			cv::String ret = instance->type;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::dnn::Layer::setType(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:362
	// ("cv::dnn::Layer::setType", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	void cv_dnn_Layer_propType_const_String(cv::dnn::Layer* instance, const char* val) {
			instance->type = cv::String(val);
	}

	// cv::dnn::Layer::preferableTarget() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:363
	// ("cv::dnn::Layer::preferableTarget", vec![(pred!(const, [], []), _)]),
	int cv_dnn_Layer_propPreferableTarget_const(const cv::dnn::Layer* instance) {
			int ret = instance->preferableTarget;
			return ret;
	}

	// cv::dnn::Layer::setPreferableTarget(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:363
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

	// cv::dnn::Layer::to_ActivationLayer() generated
	// ("cv::dnn::Layer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_dnn_Layer_to_ActivationLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ActivationLayer*>(instance);
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

	// cv::dnn::Layer::to_BlankLayer() generated
	// ("cv::dnn::Layer::to_BlankLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::BlankLayer* cv_dnn_Layer_to_BlankLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::BlankLayer*>(instance);
	}

	// cv::dnn::Layer::to_ChannelsPReLULayer() generated
	// ("cv::dnn::Layer::to_ChannelsPReLULayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ChannelsPReLULayer* cv_dnn_Layer_to_ChannelsPReLULayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ChannelsPReLULayer*>(instance);
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

	// cv::dnn::Layer::to_CorrelationLayer() generated
	// ("cv::dnn::Layer::to_CorrelationLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::CorrelationLayer* cv_dnn_Layer_to_CorrelationLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::CorrelationLayer*>(instance);
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

	// cv::dnn::Layer::to_EltwiseLayer() generated
	// ("cv::dnn::Layer::to_EltwiseLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::EltwiseLayer* cv_dnn_Layer_to_EltwiseLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::EltwiseLayer*>(instance);
	}

	// cv::dnn::Layer::to_ExpLayer() generated
	// ("cv::dnn::Layer::to_ExpLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ExpLayer* cv_dnn_Layer_to_ExpLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ExpLayer*>(instance);
	}

	// cv::dnn::Layer::to_FlattenLayer() generated
	// ("cv::dnn::Layer::to_FlattenLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::FlattenLayer* cv_dnn_Layer_to_FlattenLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::FlattenLayer*>(instance);
	}

	// cv::dnn::Layer::to_FlowWarpLayer() generated
	// ("cv::dnn::Layer::to_FlowWarpLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::FlowWarpLayer* cv_dnn_Layer_to_FlowWarpLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::FlowWarpLayer*>(instance);
	}

	// cv::dnn::Layer::to_InnerProductLayer() generated
	// ("cv::dnn::Layer::to_InnerProductLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::InnerProductLayer* cv_dnn_Layer_to_InnerProductLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::InnerProductLayer*>(instance);
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

	// cv::dnn::Layer::to_MVNLayer() generated
	// ("cv::dnn::Layer::to_MVNLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::MVNLayer* cv_dnn_Layer_to_MVNLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::MVNLayer*>(instance);
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

	// cv::dnn::Layer::to_NormalizeBBoxLayer() generated
	// ("cv::dnn::Layer::to_NormalizeBBoxLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::NormalizeBBoxLayer* cv_dnn_Layer_to_NormalizeBBoxLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::NormalizeBBoxLayer*>(instance);
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

	// cv::dnn::Layer::to_ScaleLayer() generated
	// ("cv::dnn::Layer::to_ScaleLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ScaleLayer* cv_dnn_Layer_to_ScaleLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ScaleLayer*>(instance);
	}

	// cv::dnn::Layer::to_ShiftLayer() generated
	// ("cv::dnn::Layer::to_ShiftLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ShiftLayer* cv_dnn_Layer_to_ShiftLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::ShiftLayer*>(instance);
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

	// cv::dnn::Layer::to_SplitLayer() generated
	// ("cv::dnn::Layer::to_SplitLayer", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SplitLayer* cv_dnn_Layer_to_SplitLayer(cv::dnn::Layer* instance) {
			return dynamic_cast<cv::dnn::SplitLayer*>(instance);
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

	// registerLayer(const String &, Constructor)(InString, Function) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/layer.hpp:64
	// ("cv::dnn::LayerFactory::registerLayer", vec![(pred!(mut, ["type", "constructor"], ["const cv::String*", "cv::dnn::LayerFactory::Constructor"]), _)]),
	void cv_dnn_LayerFactory_registerLayer_const_StringR_Constructor(const char* type, cv::dnn::LayerFactory::Constructor constructor, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::LayerFactory::registerLayer(cv::String(type), constructor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// unregisterLayer(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/layer.hpp:67
	// ("cv::dnn::LayerFactory::unregisterLayer", vec![(pred!(mut, ["type"], ["const cv::String*"]), _)]),
	void cv_dnn_LayerFactory_unregisterLayer_const_StringR(const char* type, ResultVoid* ocvrs_return) {
		try {
			cv::dnn::LayerFactory::unregisterLayer(cv::String(type));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createLayerInstance(const String &, LayerParams &)(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/layer.hpp:74
	// ("cv::dnn::LayerFactory::createLayerInstance", vec![(pred!(mut, ["type", "params"], ["const cv::String*", "cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_LayerFactory_createLayerInstance_const_StringR_LayerParamsR(const char* type, cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::Layer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::LayerFactory::createLayerInstance(cv::String(type), *params);
			Ok(new cv::Ptr<cv::dnn::Layer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::LayerFactory::delete() generated
	// ("cv::dnn::LayerFactory::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_LayerFactory_delete(cv::dnn::LayerFactory* instance) {
			delete instance;
	}

	// cv::dnn::LayerParams::defaultNew() generated
	// ("cv::dnn::LayerParams::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::dnn::LayerParams* cv_dnn_LayerParams_defaultNew_const() {
			cv::dnn::LayerParams* ret = new cv::dnn::LayerParams();
			return ret;
	}

	// cv::dnn::LayerParams::blobs() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:116
	// ("cv::dnn::LayerParams::blobs", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Mat>* cv_dnn_LayerParams_propBlobs_const(const cv::dnn::LayerParams* instance) {
			std::vector<cv::Mat> ret = instance->blobs;
			return new std::vector<cv::Mat>(ret);
	}

	// cv::dnn::LayerParams::setBlobs(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:116
	// ("cv::dnn::LayerParams::setBlobs", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	void cv_dnn_LayerParams_propBlobs_const_vectorLMatG(cv::dnn::LayerParams* instance, const std::vector<cv::Mat>* val) {
			instance->blobs = *val;
	}

	// cv::dnn::LayerParams::name() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:118
	// ("cv::dnn::LayerParams::name", vec![(pred!(const, [], []), _)]),
	void* cv_dnn_LayerParams_propName_const(const cv::dnn::LayerParams* instance) {
			cv::String ret = instance->name;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::dnn::LayerParams::setName(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:118
	// ("cv::dnn::LayerParams::setName", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	void cv_dnn_LayerParams_propName_const_String(cv::dnn::LayerParams* instance, const char* val) {
			instance->name = cv::String(val);
	}

	// cv::dnn::LayerParams::type() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:119
	// ("cv::dnn::LayerParams::type", vec![(pred!(const, [], []), _)]),
	void* cv_dnn_LayerParams_propType_const(const cv::dnn::LayerParams* instance) {
			cv::String ret = instance->type;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::dnn::LayerParams::setType(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:119
	// ("cv::dnn::LayerParams::setType", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	void cv_dnn_LayerParams_propType_const_String(cv::dnn::LayerParams* instance, const char* val) {
			instance->type = cv::String(val);
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:290
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

	// cv::dnn::MVNLayer::eps() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:287
	// ("cv::dnn::MVNLayer::eps", vec![(pred!(const, [], []), _)]),
	float cv_dnn_MVNLayer_propEps_const(const cv::dnn::MVNLayer* instance) {
			float ret = instance->eps;
			return ret;
	}

	// cv::dnn::MVNLayer::setEps(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:287
	// ("cv::dnn::MVNLayer::setEps", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_MVNLayer_propEps_const_float(cv::dnn::MVNLayer* instance, const float val) {
			instance->eps = val;
	}

	// cv::dnn::MVNLayer::normVariance() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:288
	// ("cv::dnn::MVNLayer::normVariance", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_MVNLayer_propNormVariance_const(const cv::dnn::MVNLayer* instance) {
			bool ret = instance->normVariance;
			return ret;
	}

	// cv::dnn::MVNLayer::setNormVariance(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:288
	// ("cv::dnn::MVNLayer::setNormVariance", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_dnn_MVNLayer_propNormVariance_const_bool(cv::dnn::MVNLayer* instance, const bool val) {
			instance->normVariance = val;
	}

	// cv::dnn::MVNLayer::acrossChannels() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:288
	// ("cv::dnn::MVNLayer::acrossChannels", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_MVNLayer_propAcrossChannels_const(const cv::dnn::MVNLayer* instance) {
			bool ret = instance->acrossChannels;
			return ret;
	}

	// cv::dnn::MVNLayer::setAcrossChannels(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:288
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:550
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

	// cv::dnn::MaxUnpoolLayer::poolKernel() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:546
	// ("cv::dnn::MaxUnpoolLayer::poolKernel", vec![(pred!(const, [], []), _)]),
	void cv_dnn_MaxUnpoolLayer_propPoolKernel_const(const cv::dnn::MaxUnpoolLayer* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->poolKernel;
			*ocvrs_return = ret;
	}

	// cv::dnn::MaxUnpoolLayer::setPoolKernel(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:546
	// ("cv::dnn::MaxUnpoolLayer::setPoolKernel", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_dnn_MaxUnpoolLayer_propPoolKernel_const_Size(cv::dnn::MaxUnpoolLayer* instance, const cv::Size* val) {
			instance->poolKernel = *val;
	}

	// cv::dnn::MaxUnpoolLayer::poolPad() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:547
	// ("cv::dnn::MaxUnpoolLayer::poolPad", vec![(pred!(const, [], []), _)]),
	void cv_dnn_MaxUnpoolLayer_propPoolPad_const(const cv::dnn::MaxUnpoolLayer* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->poolPad;
			*ocvrs_return = ret;
	}

	// cv::dnn::MaxUnpoolLayer::setPoolPad(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:547
	// ("cv::dnn::MaxUnpoolLayer::setPoolPad", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_dnn_MaxUnpoolLayer_propPoolPad_const_Size(cv::dnn::MaxUnpoolLayer* instance, const cv::Size* val) {
			instance->poolPad = *val;
	}

	// cv::dnn::MaxUnpoolLayer::poolStride() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:548
	// ("cv::dnn::MaxUnpoolLayer::poolStride", vec![(pred!(const, [], []), _)]),
	void cv_dnn_MaxUnpoolLayer_propPoolStride_const(const cv::dnn::MaxUnpoolLayer* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->poolStride;
			*ocvrs_return = ret;
	}

	// cv::dnn::MaxUnpoolLayer::setPoolStride(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:548
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:476
	// ("cv::dnn::MishLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_MishLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::MishLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::MishLayer> ret = cv::dnn::MishLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::MishLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
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

	// Net()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:385
	// ("cv::dnn::Net::Net", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_Net_Net(Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net* ret = new cv::dnn::Net();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readFromModelOptimizer(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:394
	// ("cv::dnn::Net::readFromModelOptimizer", vec![(pred!(mut, ["xml", "bin"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_dnn_Net_readFromModelOptimizer_const_StringR_const_StringR(const char* xml, const char* bin, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::Net::readFromModelOptimizer(cv::String(xml), cv::String(bin));
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readFromModelOptimizer(const std::vector<uchar> &, const std::vector<uchar> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:402
	// ("cv::dnn::Net::readFromModelOptimizer", vec![(pred!(mut, ["bufferModelConfig", "bufferWeights"], ["const std::vector<unsigned char>*", "const std::vector<unsigned char>*"]), _)]),
	void cv_dnn_Net_readFromModelOptimizer_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR(const std::vector<unsigned char>* bufferModelConfig, const std::vector<unsigned char>* bufferWeights, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::Net::readFromModelOptimizer(*bufferModelConfig, *bufferWeights);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readFromModelOptimizer(const uchar *, size_t, const uchar *, size_t)(VariableArray, Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:412
	// ("cv::dnn::Net::readFromModelOptimizer", vec![(pred!(mut, ["bufferModelConfigPtr", "bufferModelConfigSize", "bufferWeightsPtr", "bufferWeightsSize"], ["const unsigned char*", "size_t", "const unsigned char*", "size_t"]), _)]),
	void cv_dnn_Net_readFromModelOptimizer_const_unsigned_charX_size_t_const_unsigned_charX_size_t(const unsigned char* bufferModelConfigPtr, size_t bufferModelConfigSize, const unsigned char* bufferWeightsPtr, size_t bufferWeightsSize, Result<cv::dnn::Net*>* ocvrs_return) {
		try {
			cv::dnn::Net ret = cv::dnn::Net::readFromModelOptimizer(bufferModelConfigPtr, bufferModelConfigSize, bufferWeightsPtr, bufferWeightsSize);
			Ok(new cv::dnn::Net(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:416
	// ("cv::dnn::Net::empty", vec![(pred!(const, [], []), _)]),
	void cv_dnn_Net_empty_const(const cv::dnn::Net* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dump()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:422
	// ("cv::dnn::Net::dump", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_Net_dump(cv::dnn::Net* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->dump();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dumpToFile(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:427
	// ("cv::dnn::Net::dumpToFile", vec![(pred!(mut, ["path"], ["const cv::String*"]), _)]),
	void cv_dnn_Net_dumpToFile_const_StringR(cv::dnn::Net* instance, const char* path, ResultVoid* ocvrs_return) {
		try {
			instance->dumpToFile(cv::String(path));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addLayer(const String &, const String &, LayerParams &)(InString, InString, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:434
	// ("cv::dnn::Net::addLayer", vec![(pred!(mut, ["name", "type", "params"], ["const cv::String*", "const cv::String*", "cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_Net_addLayer_const_StringR_const_StringR_LayerParamsR(cv::dnn::Net* instance, const char* name, const char* type, cv::dnn::LayerParams* params, Result<int>* ocvrs_return) {
		try {
			int ret = instance->addLayer(cv::String(name), cv::String(type), *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addLayerToPrev(const String &, const String &, LayerParams &)(InString, InString, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:438
	// ("cv::dnn::Net::addLayerToPrev", vec![(pred!(mut, ["name", "type", "params"], ["const cv::String*", "const cv::String*", "cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_Net_addLayerToPrev_const_StringR_const_StringR_LayerParamsR(cv::dnn::Net* instance, const char* name, const char* type, cv::dnn::LayerParams* params, Result<int>* ocvrs_return) {
		try {
			int ret = instance->addLayerToPrev(cv::String(name), cv::String(type), *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLayerId(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:443
	// ("cv::dnn::Net::getLayerId", vec![(pred!(const, ["layer"], ["const cv::String*"]), _)]),
	void cv_dnn_Net_getLayerId_const_const_StringR(const cv::dnn::Net* instance, const char* layer, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getLayerId(cv::String(layer));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLayerNames()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:445
	// ("cv::dnn::Net::getLayerNames", vec![(pred!(const, [], []), _)]),
	void cv_dnn_Net_getLayerNames_const(const cv::dnn::Net* instance, Result<std::vector<cv::String>*>* ocvrs_return) {
		try {
			std::vector<cv::String> ret = instance->getLayerNames();
			Ok(new std::vector<cv::String>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLayer(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:454
	// ("cv::dnn::Net::getLayer", vec![(pred!(const, ["layerId"], ["int"]), _)]),
	void cv_dnn_Net_getLayer_const_int(const cv::dnn::Net* instance, int layerId, Result<cv::Ptr<cv::dnn::Layer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = instance->getLayer(layerId);
			Ok(new cv::Ptr<cv::dnn::Layer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLayer(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:458
	// ("cv::dnn::Net::getLayer", vec![(pred!(const, ["layerName"], ["const cv::String*"]), _)]),
	void cv_dnn_Net_getLayer_const_const_StringR(const cv::dnn::Net* instance, const char* layerName, Result<cv::Ptr<cv::dnn::Layer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = instance->getLayer(cv::String(layerName));
			Ok(new cv::Ptr<cv::dnn::Layer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLayer(const LayerId &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:462
	// ("cv::dnn::Net::getLayer", vec![(pred!(const, ["layerId"], ["const cv::dnn::Net::LayerId*"]), _)]),
	void cv_dnn_Net_getLayer_const_const_LayerIdR(const cv::dnn::Net* instance, const cv::dnn::Net::LayerId* layerId, Result<cv::Ptr<cv::dnn::Layer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = instance->getLayer(*layerId);
			Ok(new cv::Ptr<cv::dnn::Layer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLayerInputs(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:465
	// ("cv::dnn::Net::getLayerInputs", vec![(pred!(const, ["layerId"], ["int"]), _)]),
	void cv_dnn_Net_getLayerInputs_const_int(const cv::dnn::Net* instance, int layerId, Result<std::vector<cv::Ptr<cv::dnn::Layer>>*>* ocvrs_return) {
		try {
			std::vector<cv::Ptr<cv::dnn::Layer>> ret = instance->getLayerInputs(layerId);
			Ok(new std::vector<cv::Ptr<cv::dnn::Layer>>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// connect(String, String)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:480
	// ("cv::dnn::Net::connect", vec![(pred!(mut, ["outPin", "inpPin"], ["cv::String", "cv::String"]), _)]),
	void cv_dnn_Net_connect_String_String(cv::dnn::Net* instance, const char* outPin, const char* inpPin, ResultVoid* ocvrs_return) {
		try {
			instance->connect(cv::String(outPin), cv::String(inpPin));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// connect(int, int, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:488
	// ("cv::dnn::Net::connect", vec![(pred!(mut, ["outLayerId", "outNum", "inpLayerId", "inpNum"], ["int", "int", "int", "int"]), _)]),
	void cv_dnn_Net_connect_int_int_int_int(cv::dnn::Net* instance, int outLayerId, int outNum, int inpLayerId, int inpNum, ResultVoid* ocvrs_return) {
		try {
			instance->connect(outLayerId, outNum, inpLayerId, inpNum);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// registerOutput(const std::string &, int, int)(InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:500
	// ("cv::dnn::Net::registerOutput", vec![(pred!(mut, ["outputName", "layerId", "outputPort"], ["const std::string*", "int", "int"]), _)]),
	void cv_dnn_Net_registerOutput_const_stringR_int_int(cv::dnn::Net* instance, const char* outputName, int layerId, int outputPort, Result<int>* ocvrs_return) {
		try {
			int ret = instance->registerOutput(std::string(outputName), layerId, outputPort);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInputsNames(const std::vector<String> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:509
	// ("cv::dnn::Net::setInputsNames", vec![(pred!(mut, ["inputBlobNames"], ["const std::vector<cv::String>*"]), _)]),
	void cv_dnn_Net_setInputsNames_const_vectorLStringGR(cv::dnn::Net* instance, const std::vector<cv::String>* inputBlobNames, ResultVoid* ocvrs_return) {
		try {
			instance->setInputsNames(*inputBlobNames);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInputShape(const String &, const MatShape &)(InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:513
	// ("cv::dnn::Net::setInputShape", vec![(pred!(mut, ["inputName", "shape"], ["const cv::String*", "const cv::dnn::MatShape*"]), _)]),
	void cv_dnn_Net_setInputShape_const_StringR_const_MatShapeR(cv::dnn::Net* instance, const char* inputName, const cv::dnn::MatShape* shape, ResultVoid* ocvrs_return) {
		try {
			instance->setInputShape(cv::String(inputName), *shape);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// forward(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:520
	// ("cv::dnn::Net::forward", vec![(pred!(mut, ["outputName"], ["const cv::String*"]), _)]),
	void cv_dnn_Net_forward_const_StringR(cv::dnn::Net* instance, const char* outputName, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->forward(cv::String(outputName));
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::Net::forward() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:520
	// ("cv::dnn::Net::forward", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_Net_forward(cv::dnn::Net* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->forward();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// forwardAsync(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:529
	// ("cv::dnn::Net::forwardAsync", vec![(pred!(mut, ["outputName"], ["const cv::String*"]), _)]),
	void cv_dnn_Net_forwardAsync_const_StringR(cv::dnn::Net* instance, const char* outputName, Result<cv::AsyncArray*>* ocvrs_return) {
		try {
			cv::AsyncArray ret = instance->forwardAsync(cv::String(outputName));
			Ok(new cv::AsyncArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::Net::forwardAsync() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:529
	// ("cv::dnn::Net::forwardAsync", vec![(pred!(mut, [], []), _)]),
	void cv_dnn_Net_forwardAsync(cv::dnn::Net* instance, Result<cv::AsyncArray*>* ocvrs_return) {
		try {
			cv::AsyncArray ret = instance->forwardAsync();
			Ok(new cv::AsyncArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// forward(OutputArrayOfArrays, const String &)(OutputArray, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:536
	// ("cv::dnn::Net::forward", vec![(pred!(mut, ["outputBlobs", "outputName"], ["const cv::_OutputArray*", "const cv::String*"]), _)]),
	void cv_dnn_Net_forward_const__OutputArrayR_const_StringR(cv::dnn::Net* instance, const cv::_OutputArray* outputBlobs, const char* outputName, ResultVoid* ocvrs_return) {
		try {
			instance->forward(*outputBlobs, cv::String(outputName));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::Net::forward(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:536
	// ("cv::dnn::Net::forward", vec![(pred!(mut, ["outputBlobs"], ["const cv::_OutputArray*"]), _)]),
	void cv_dnn_Net_forward_const__OutputArrayR(cv::dnn::Net* instance, const cv::_OutputArray* outputBlobs, ResultVoid* ocvrs_return) {
		try {
			instance->forward(*outputBlobs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// forward(OutputArrayOfArrays, const std::vector<String> &)(OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:542
	// ("cv::dnn::Net::forward", vec![(pred!(mut, ["outputBlobs", "outBlobNames"], ["const cv::_OutputArray*", "const std::vector<cv::String>*"]), _)]),
	void cv_dnn_Net_forward_const__OutputArrayR_const_vectorLStringGR(cv::dnn::Net* instance, const cv::_OutputArray* outputBlobs, const std::vector<cv::String>* outBlobNames, ResultVoid* ocvrs_return) {
		try {
			instance->forward(*outputBlobs, *outBlobNames);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// forward(std::vector<std::vector<Mat>> &, const std::vector<String> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:549
	// ("cv::dnn::Net::forward", vec![(pred!(mut, ["outputBlobs", "outBlobNames"], ["std::vector<std::vector<cv::Mat>>*", "const std::vector<cv::String>*"]), _)]),
	void cv_dnn_Net_forward_vectorLvectorLMatGGR_const_vectorLStringGR(cv::dnn::Net* instance, std::vector<std::vector<cv::Mat>>* outputBlobs, const std::vector<cv::String>* outBlobNames, ResultVoid* ocvrs_return) {
		try {
			instance->forward(*outputBlobs, *outBlobNames);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setHalideScheduler(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:561
	// ("cv::dnn::Net::setHalideScheduler", vec![(pred!(mut, ["scheduler"], ["const cv::String*"]), _)]),
	void cv_dnn_Net_setHalideScheduler_const_StringR(cv::dnn::Net* instance, const char* scheduler, ResultVoid* ocvrs_return) {
		try {
			instance->setHalideScheduler(cv::String(scheduler));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPreferableBackend(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:571
	// ("cv::dnn::Net::setPreferableBackend", vec![(pred!(mut, ["backendId"], ["int"]), _)]),
	void cv_dnn_Net_setPreferableBackend_int(cv::dnn::Net* instance, int backendId, ResultVoid* ocvrs_return) {
		try {
			instance->setPreferableBackend(backendId);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPreferableTarget(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:587
	// ("cv::dnn::Net::setPreferableTarget", vec![(pred!(mut, ["targetId"], ["int"]), _)]),
	void cv_dnn_Net_setPreferableTarget_int(cv::dnn::Net* instance, int targetId, ResultVoid* ocvrs_return) {
		try {
			instance->setPreferableTarget(targetId);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInput(InputArray, const String &, double, const Scalar &)(InputArray, InString, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:600
	// ("cv::dnn::Net::setInput", vec![(pred!(mut, ["blob", "name", "scalefactor", "mean"], ["const cv::_InputArray*", "const cv::String*", "double", "const cv::Scalar*"]), _)]),
	void cv_dnn_Net_setInput_const__InputArrayR_const_StringR_double_const_ScalarR(cv::dnn::Net* instance, const cv::_InputArray* blob, const char* name, double scalefactor, const cv::Scalar* mean, ResultVoid* ocvrs_return) {
		try {
			instance->setInput(*blob, cv::String(name), scalefactor, *mean);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::Net::setInput(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:600
	// ("cv::dnn::Net::setInput", vec![(pred!(mut, ["blob"], ["const cv::_InputArray*"]), _)]),
	void cv_dnn_Net_setInput_const__InputArrayR(cv::dnn::Net* instance, const cv::_InputArray* blob, ResultVoid* ocvrs_return) {
		try {
			instance->setInput(*blob);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setParam(int, int, const Mat &)(Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:611
	// ("cv::dnn::Net::setParam", vec![(pred!(mut, ["layer", "numParam", "blob"], ["int", "int", "const cv::Mat*"]), _)]),
	void cv_dnn_Net_setParam_int_int_const_MatR(cv::dnn::Net* instance, int layer, int numParam, const cv::Mat* blob, ResultVoid* ocvrs_return) {
		try {
			instance->setParam(layer, numParam, *blob);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setParam(const String &, int, const Mat &)(InString, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:612
	// ("cv::dnn::Net::setParam", vec![(pred!(mut, ["layerName", "numParam", "blob"], ["const cv::String*", "int", "const cv::Mat*"]), _)]),
	void cv_dnn_Net_setParam_const_StringR_int_const_MatR(cv::dnn::Net* instance, const char* layerName, int numParam, const cv::Mat* blob, ResultVoid* ocvrs_return) {
		try {
			instance->setParam(cv::String(layerName), numParam, *blob);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getParam(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:619
	// ("cv::dnn::Net::getParam", vec![(pred!(const, ["layer", "numParam"], ["int", "int"]), _)]),
	void cv_dnn_Net_getParam_const_int_int(const cv::dnn::Net* instance, int layer, int numParam, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getParam(layer, numParam);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::Net::getParam(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:619
	// ("cv::dnn::Net::getParam", vec![(pred!(const, ["layer"], ["int"]), _)]),
	void cv_dnn_Net_getParam_const_int(const cv::dnn::Net* instance, int layer, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getParam(layer);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getParam(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:620
	// ("cv::dnn::Net::getParam", vec![(pred!(const, ["layerName", "numParam"], ["const cv::String*", "int"]), _)]),
	void cv_dnn_Net_getParam_const_const_StringR_int(const cv::dnn::Net* instance, const char* layerName, int numParam, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getParam(cv::String(layerName), numParam);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::Net::getParam(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:620
	// ("cv::dnn::Net::getParam", vec![(pred!(const, ["layerName"], ["const cv::String*"]), _)]),
	void cv_dnn_Net_getParam_const_const_StringR(const cv::dnn::Net* instance, const char* layerName, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getParam(cv::String(layerName));
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUnconnectedOutLayers()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:626
	// ("cv::dnn::Net::getUnconnectedOutLayers", vec![(pred!(const, [], []), _)]),
	void cv_dnn_Net_getUnconnectedOutLayers_const(const cv::dnn::Net* instance, Result<std::vector<int>*>* ocvrs_return) {
		try {
			std::vector<int> ret = instance->getUnconnectedOutLayers();
			Ok(new std::vector<int>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUnconnectedOutLayersNames()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:632
	// ("cv::dnn::Net::getUnconnectedOutLayersNames", vec![(pred!(const, [], []), _)]),
	void cv_dnn_Net_getUnconnectedOutLayersNames_const(const cv::dnn::Net* instance, Result<std::vector<cv::String>*>* ocvrs_return) {
		try {
			std::vector<cv::String> ret = instance->getUnconnectedOutLayersNames();
			Ok(new std::vector<cv::String>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLayersShapes(const std::vector<MatShape> &, std::vector<int> &, std::vector<std::vector<MatShape>> &, std::vector<std::vector<MatShape>> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:643
	// ("cv::dnn::Net::getLayersShapes", vec![(pred!(const, ["netInputShapes", "layersIds", "inLayersShapes", "outLayersShapes"], ["const std::vector<cv::dnn::MatShape>*", "std::vector<int>*", "std::vector<std::vector<cv::dnn::MatShape>>*", "std::vector<std::vector<cv::dnn::MatShape>>*"]), _)]),
	void cv_dnn_Net_getLayersShapes_const_const_vectorLMatShapeGR_vectorLintGR_vectorLvectorLMatShapeGGR_vectorLvectorLMatShapeGGR(const cv::dnn::Net* instance, const std::vector<cv::dnn::MatShape>* netInputShapes, std::vector<int>* layersIds, std::vector<std::vector<cv::dnn::MatShape>>* inLayersShapes, std::vector<std::vector<cv::dnn::MatShape>>* outLayersShapes, ResultVoid* ocvrs_return) {
		try {
			instance->getLayersShapes(*netInputShapes, *layersIds, *inLayersShapes, *outLayersShapes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLayersShapes(const MatShape &, std::vector<int> &, std::vector<std::vector<MatShape>> &, std::vector<std::vector<MatShape>> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:649
	// ("cv::dnn::Net::getLayersShapes", vec![(pred!(const, ["netInputShape", "layersIds", "inLayersShapes", "outLayersShapes"], ["const cv::dnn::MatShape*", "std::vector<int>*", "std::vector<std::vector<cv::dnn::MatShape>>*", "std::vector<std::vector<cv::dnn::MatShape>>*"]), _)]),
	void cv_dnn_Net_getLayersShapes_const_const_MatShapeR_vectorLintGR_vectorLvectorLMatShapeGGR_vectorLvectorLMatShapeGGR(const cv::dnn::Net* instance, const cv::dnn::MatShape* netInputShape, std::vector<int>* layersIds, std::vector<std::vector<cv::dnn::MatShape>>* inLayersShapes, std::vector<std::vector<cv::dnn::MatShape>>* outLayersShapes, ResultVoid* ocvrs_return) {
		try {
			instance->getLayersShapes(*netInputShape, *layersIds, *inLayersShapes, *outLayersShapes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLayerShapes(const MatShape &, const int, std::vector<MatShape> &, std::vector<MatShape> &)(CppPassByVoidPtr, Primitive, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:663
	// ("cv::dnn::Net::getLayerShapes", vec![(pred!(const, ["netInputShape", "layerId", "inLayerShapes", "outLayerShapes"], ["const cv::dnn::MatShape*", "const int", "std::vector<cv::dnn::MatShape>*", "std::vector<cv::dnn::MatShape>*"]), _)]),
	void cv_dnn_Net_getLayerShapes_const_const_MatShapeR_const_int_vectorLMatShapeGR_vectorLMatShapeGR(const cv::dnn::Net* instance, const cv::dnn::MatShape* netInputShape, const int layerId, std::vector<cv::dnn::MatShape>* inLayerShapes, std::vector<cv::dnn::MatShape>* outLayerShapes, ResultVoid* ocvrs_return) {
		try {
			instance->getLayerShapes(*netInputShape, layerId, *inLayerShapes, *outLayerShapes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLayerShapes(const std::vector<MatShape> &, const int, std::vector<MatShape> &, std::vector<MatShape> &)(CppPassByVoidPtr, Primitive, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:669
	// ("cv::dnn::Net::getLayerShapes", vec![(pred!(const, ["netInputShapes", "layerId", "inLayerShapes", "outLayerShapes"], ["const std::vector<cv::dnn::MatShape>*", "const int", "std::vector<cv::dnn::MatShape>*", "std::vector<cv::dnn::MatShape>*"]), _)]),
	void cv_dnn_Net_getLayerShapes_const_const_vectorLMatShapeGR_const_int_vectorLMatShapeGR_vectorLMatShapeGR(const cv::dnn::Net* instance, const std::vector<cv::dnn::MatShape>* netInputShapes, const int layerId, std::vector<cv::dnn::MatShape>* inLayerShapes, std::vector<cv::dnn::MatShape>* outLayerShapes, ResultVoid* ocvrs_return) {
		try {
			instance->getLayerShapes(*netInputShapes, layerId, *inLayerShapes, *outLayerShapes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFLOPS(const std::vector<MatShape> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:678
	// ("cv::dnn::Net::getFLOPS", vec![(pred!(const, ["netInputShapes"], ["const std::vector<cv::dnn::MatShape>*"]), _)]),
	void cv_dnn_Net_getFLOPS_const_const_vectorLMatShapeGR(const cv::dnn::Net* instance, const std::vector<cv::dnn::MatShape>* netInputShapes, Result<int64_t>* ocvrs_return) {
		try {
			int64_t ret = instance->getFLOPS(*netInputShapes);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFLOPS(const MatShape &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:680
	// ("cv::dnn::Net::getFLOPS", vec![(pred!(const, ["netInputShape"], ["const cv::dnn::MatShape*"]), _)]),
	void cv_dnn_Net_getFLOPS_const_const_MatShapeR(const cv::dnn::Net* instance, const cv::dnn::MatShape* netInputShape, Result<int64_t>* ocvrs_return) {
		try {
			int64_t ret = instance->getFLOPS(*netInputShape);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFLOPS(const int, const std::vector<MatShape> &)(Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:682
	// ("cv::dnn::Net::getFLOPS", vec![(pred!(const, ["layerId", "netInputShapes"], ["const int", "const std::vector<cv::dnn::MatShape>*"]), _)]),
	void cv_dnn_Net_getFLOPS_const_const_int_const_vectorLMatShapeGR(const cv::dnn::Net* instance, const int layerId, const std::vector<cv::dnn::MatShape>* netInputShapes, Result<int64_t>* ocvrs_return) {
		try {
			int64_t ret = instance->getFLOPS(layerId, *netInputShapes);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFLOPS(const int, const MatShape &)(Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:685
	// ("cv::dnn::Net::getFLOPS", vec![(pred!(const, ["layerId", "netInputShape"], ["const int", "const cv::dnn::MatShape*"]), _)]),
	void cv_dnn_Net_getFLOPS_const_const_int_const_MatShapeR(const cv::dnn::Net* instance, const int layerId, const cv::dnn::MatShape* netInputShape, Result<int64_t>* ocvrs_return) {
		try {
			int64_t ret = instance->getFLOPS(layerId, *netInputShape);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLayerTypes(std::vector<String> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:691
	// ("cv::dnn::Net::getLayerTypes", vec![(pred!(const, ["layersTypes"], ["std::vector<cv::String>*"]), _)]),
	void cv_dnn_Net_getLayerTypes_const_vectorLStringGR(const cv::dnn::Net* instance, std::vector<cv::String>* layersTypes, ResultVoid* ocvrs_return) {
		try {
			instance->getLayerTypes(*layersTypes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLayersCount(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:697
	// ("cv::dnn::Net::getLayersCount", vec![(pred!(const, ["layerType"], ["const cv::String*"]), _)]),
	void cv_dnn_Net_getLayersCount_const_const_StringR(const cv::dnn::Net* instance, const char* layerType, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getLayersCount(cv::String(layerType));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMemoryConsumption(const std::vector<MatShape> &, size_t &, size_t &)(CppPassByVoidPtr, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:705
	// ("cv::dnn::Net::getMemoryConsumption", vec![(pred!(const, ["netInputShapes", "weights", "blobs"], ["const std::vector<cv::dnn::MatShape>*", "size_t*", "size_t*"]), _)]),
	void cv_dnn_Net_getMemoryConsumption_const_const_vectorLMatShapeGR_size_tR_size_tR(const cv::dnn::Net* instance, const std::vector<cv::dnn::MatShape>* netInputShapes, size_t* weights, size_t* blobs, ResultVoid* ocvrs_return) {
		try {
			instance->getMemoryConsumption(*netInputShapes, *weights, *blobs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMemoryConsumption(const MatShape &, size_t &, size_t &)(CppPassByVoidPtr, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:708
	// ("cv::dnn::Net::getMemoryConsumption", vec![(pred!(const, ["netInputShape", "weights", "blobs"], ["const cv::dnn::MatShape*", "size_t*", "size_t*"]), _)]),
	void cv_dnn_Net_getMemoryConsumption_const_const_MatShapeR_size_tR_size_tR(const cv::dnn::Net* instance, const cv::dnn::MatShape* netInputShape, size_t* weights, size_t* blobs, ResultVoid* ocvrs_return) {
		try {
			instance->getMemoryConsumption(*netInputShape, *weights, *blobs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMemoryConsumption(const int, const std::vector<MatShape> &, size_t &, size_t &)(Primitive, CppPassByVoidPtr, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:711
	// ("cv::dnn::Net::getMemoryConsumption", vec![(pred!(const, ["layerId", "netInputShapes", "weights", "blobs"], ["const int", "const std::vector<cv::dnn::MatShape>*", "size_t*", "size_t*"]), _)]),
	void cv_dnn_Net_getMemoryConsumption_const_const_int_const_vectorLMatShapeGR_size_tR_size_tR(const cv::dnn::Net* instance, const int layerId, const std::vector<cv::dnn::MatShape>* netInputShapes, size_t* weights, size_t* blobs, ResultVoid* ocvrs_return) {
		try {
			instance->getMemoryConsumption(layerId, *netInputShapes, *weights, *blobs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMemoryConsumption(const int, const MatShape &, size_t &, size_t &)(Primitive, CppPassByVoidPtr, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:715
	// ("cv::dnn::Net::getMemoryConsumption", vec![(pred!(const, ["layerId", "netInputShape", "weights", "blobs"], ["const int", "const cv::dnn::MatShape*", "size_t*", "size_t*"]), _)]),
	void cv_dnn_Net_getMemoryConsumption_const_const_int_const_MatShapeR_size_tR_size_tR(const cv::dnn::Net* instance, const int layerId, const cv::dnn::MatShape* netInputShape, size_t* weights, size_t* blobs, ResultVoid* ocvrs_return) {
		try {
			instance->getMemoryConsumption(layerId, *netInputShape, *weights, *blobs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMemoryConsumption(const std::vector<MatShape> &, std::vector<int> &, std::vector<size_t> &, std::vector<size_t> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:726
	// ("cv::dnn::Net::getMemoryConsumption", vec![(pred!(const, ["netInputShapes", "layerIds", "weights", "blobs"], ["const std::vector<cv::dnn::MatShape>*", "std::vector<int>*", "std::vector<size_t>*", "std::vector<size_t>*"]), _)]),
	void cv_dnn_Net_getMemoryConsumption_const_const_vectorLMatShapeGR_vectorLintGR_vectorLsize_tGR_vectorLsize_tGR(const cv::dnn::Net* instance, const std::vector<cv::dnn::MatShape>* netInputShapes, std::vector<int>* layerIds, std::vector<size_t>* weights, std::vector<size_t>* blobs, ResultVoid* ocvrs_return) {
		try {
			instance->getMemoryConsumption(*netInputShapes, *layerIds, *weights, *blobs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMemoryConsumption(const MatShape &, std::vector<int> &, std::vector<size_t> &, std::vector<size_t> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:731
	// ("cv::dnn::Net::getMemoryConsumption", vec![(pred!(const, ["netInputShape", "layerIds", "weights", "blobs"], ["const cv::dnn::MatShape*", "std::vector<int>*", "std::vector<size_t>*", "std::vector<size_t>*"]), _)]),
	void cv_dnn_Net_getMemoryConsumption_const_const_MatShapeR_vectorLintGR_vectorLsize_tGR_vectorLsize_tGR(const cv::dnn::Net* instance, const cv::dnn::MatShape* netInputShape, std::vector<int>* layerIds, std::vector<size_t>* weights, std::vector<size_t>* blobs, ResultVoid* ocvrs_return) {
		try {
			instance->getMemoryConsumption(*netInputShape, *layerIds, *weights, *blobs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// enableFusion(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:739
	// ("cv::dnn::Net::enableFusion", vec![(pred!(mut, ["fusion"], ["bool"]), _)]),
	void cv_dnn_Net_enableFusion_bool(cv::dnn::Net* instance, bool fusion, ResultVoid* ocvrs_return) {
		try {
			instance->enableFusion(fusion);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPerfProfile(std::vector<double> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:749
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:655
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

	// cv::dnn::NormalizeBBoxLayer::pnorm() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:652
	// ("cv::dnn::NormalizeBBoxLayer::pnorm", vec![(pred!(const, [], []), _)]),
	float cv_dnn_NormalizeBBoxLayer_propPnorm_const(const cv::dnn::NormalizeBBoxLayer* instance) {
			float ret = instance->pnorm;
			return ret;
	}

	// cv::dnn::NormalizeBBoxLayer::setPnorm(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:652
	// ("cv::dnn::NormalizeBBoxLayer::setPnorm", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_NormalizeBBoxLayer_propPnorm_const_float(cv::dnn::NormalizeBBoxLayer* instance, const float val) {
			instance->pnorm = val;
	}

	// cv::dnn::NormalizeBBoxLayer::epsilon() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:652
	// ("cv::dnn::NormalizeBBoxLayer::epsilon", vec![(pred!(const, [], []), _)]),
	float cv_dnn_NormalizeBBoxLayer_propEpsilon_const(const cv::dnn::NormalizeBBoxLayer* instance) {
			float ret = instance->epsilon;
			return ret;
	}

	// cv::dnn::NormalizeBBoxLayer::setEpsilon(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:652
	// ("cv::dnn::NormalizeBBoxLayer::setEpsilon", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_NormalizeBBoxLayer_propEpsilon_const_float(cv::dnn::NormalizeBBoxLayer* instance, const float val) {
			instance->epsilon = val;
	}

	// cv::dnn::NormalizeBBoxLayer::acrossSpatial() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:653
	// ("cv::dnn::NormalizeBBoxLayer::acrossSpatial", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_NormalizeBBoxLayer_propAcrossSpatial_const(const cv::dnn::NormalizeBBoxLayer* instance) {
			bool ret = instance->acrossSpatial;
			return ret;
	}

	// cv::dnn::NormalizeBBoxLayer::setAcrossSpatial(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:653
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:420
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:377
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:266
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

	// cv::dnn::PoolingLayer::type() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:248
	// ("cv::dnn::PoolingLayer::type", vec![(pred!(const, [], []), _)]),
	int cv_dnn_PoolingLayer_propType_const(const cv::dnn::PoolingLayer* instance) {
			int ret = instance->type;
			return ret;
	}

	// cv::dnn::PoolingLayer::setType(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:248
	// ("cv::dnn::PoolingLayer::setType", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_PoolingLayer_propType_const_int(cv::dnn::PoolingLayer* instance, const int val) {
			instance->type = val;
	}

	// cv::dnn::PoolingLayer::kernel_size() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:249
	// ("cv::dnn::PoolingLayer::kernel_size", vec![(pred!(const, [], []), _)]),
	std::vector<size_t>* cv_dnn_PoolingLayer_propKernel_size_const(const cv::dnn::PoolingLayer* instance) {
			std::vector<size_t> ret = instance->kernel_size;
			return new std::vector<size_t>(ret);
	}

	// cv::dnn::PoolingLayer::setKernel_size(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:249
	// ("cv::dnn::PoolingLayer::setKernel_size", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
	void cv_dnn_PoolingLayer_propKernel_size_const_vectorLsize_tG(cv::dnn::PoolingLayer* instance, const std::vector<size_t>* val) {
			instance->kernel_size = *val;
	}

	// cv::dnn::PoolingLayer::strides() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:249
	// ("cv::dnn::PoolingLayer::strides", vec![(pred!(const, [], []), _)]),
	std::vector<size_t>* cv_dnn_PoolingLayer_propStrides_const(const cv::dnn::PoolingLayer* instance) {
			std::vector<size_t> ret = instance->strides;
			return new std::vector<size_t>(ret);
	}

	// cv::dnn::PoolingLayer::setStrides(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:249
	// ("cv::dnn::PoolingLayer::setStrides", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
	void cv_dnn_PoolingLayer_propStrides_const_vectorLsize_tG(cv::dnn::PoolingLayer* instance, const std::vector<size_t>* val) {
			instance->strides = *val;
	}

	// cv::dnn::PoolingLayer::pads_begin() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:250
	// ("cv::dnn::PoolingLayer::pads_begin", vec![(pred!(const, [], []), _)]),
	std::vector<size_t>* cv_dnn_PoolingLayer_propPads_begin_const(const cv::dnn::PoolingLayer* instance) {
			std::vector<size_t> ret = instance->pads_begin;
			return new std::vector<size_t>(ret);
	}

	// cv::dnn::PoolingLayer::setPads_begin(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:250
	// ("cv::dnn::PoolingLayer::setPads_begin", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
	void cv_dnn_PoolingLayer_propPads_begin_const_vectorLsize_tG(cv::dnn::PoolingLayer* instance, const std::vector<size_t>* val) {
			instance->pads_begin = *val;
	}

	// cv::dnn::PoolingLayer::pads_end() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:250
	// ("cv::dnn::PoolingLayer::pads_end", vec![(pred!(const, [], []), _)]),
	std::vector<size_t>* cv_dnn_PoolingLayer_propPads_end_const(const cv::dnn::PoolingLayer* instance) {
			std::vector<size_t> ret = instance->pads_end;
			return new std::vector<size_t>(ret);
	}

	// cv::dnn::PoolingLayer::setPads_end(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:250
	// ("cv::dnn::PoolingLayer::setPads_end", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
	void cv_dnn_PoolingLayer_propPads_end_const_vectorLsize_tG(cv::dnn::PoolingLayer* instance, const std::vector<size_t>* val) {
			instance->pads_end = *val;
	}

	// cv::dnn::PoolingLayer::globalPooling() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:251
	// ("cv::dnn::PoolingLayer::globalPooling", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_PoolingLayer_propGlobalPooling_const(const cv::dnn::PoolingLayer* instance) {
			bool ret = instance->globalPooling;
			return ret;
	}

	// cv::dnn::PoolingLayer::setGlobalPooling(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:251
	// ("cv::dnn::PoolingLayer::setGlobalPooling", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_dnn_PoolingLayer_propGlobalPooling_const_bool(cv::dnn::PoolingLayer* instance, const bool val) {
			instance->globalPooling = val;
	}

	// cv::dnn::PoolingLayer::isGlobalPooling() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:252
	// ("cv::dnn::PoolingLayer::isGlobalPooling", vec![(pred!(const, [], []), _)]),
	std::vector<bool>* cv_dnn_PoolingLayer_propIsGlobalPooling_const(const cv::dnn::PoolingLayer* instance) {
			std::vector<bool> ret = instance->isGlobalPooling;
			return new std::vector<bool>(ret);
	}

	// cv::dnn::PoolingLayer::setIsGlobalPooling(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:252
	// ("cv::dnn::PoolingLayer::setIsGlobalPooling", vec![(pred!(mut, ["val"], ["const std::vector<bool>"]), _)]),
	void cv_dnn_PoolingLayer_propIsGlobalPooling_const_vectorLboolG(cv::dnn::PoolingLayer* instance, const std::vector<bool>* val) {
			instance->isGlobalPooling = *val;
	}

	// cv::dnn::PoolingLayer::computeMaxIdx() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:253
	// ("cv::dnn::PoolingLayer::computeMaxIdx", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_PoolingLayer_propComputeMaxIdx_const(const cv::dnn::PoolingLayer* instance) {
			bool ret = instance->computeMaxIdx;
			return ret;
	}

	// cv::dnn::PoolingLayer::setComputeMaxIdx(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:253
	// ("cv::dnn::PoolingLayer::setComputeMaxIdx", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_dnn_PoolingLayer_propComputeMaxIdx_const_bool(cv::dnn::PoolingLayer* instance, const bool val) {
			instance->computeMaxIdx = val;
	}

	// cv::dnn::PoolingLayer::padMode() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:254
	// ("cv::dnn::PoolingLayer::padMode", vec![(pred!(const, [], []), _)]),
	void* cv_dnn_PoolingLayer_propPadMode_const(const cv::dnn::PoolingLayer* instance) {
			cv::String ret = instance->padMode;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::dnn::PoolingLayer::setPadMode(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:254
	// ("cv::dnn::PoolingLayer::setPadMode", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	void cv_dnn_PoolingLayer_propPadMode_const_String(cv::dnn::PoolingLayer* instance, const char* val) {
			instance->padMode = cv::String(val);
	}

	// cv::dnn::PoolingLayer::ceilMode() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:255
	// ("cv::dnn::PoolingLayer::ceilMode", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_PoolingLayer_propCeilMode_const(const cv::dnn::PoolingLayer* instance) {
			bool ret = instance->ceilMode;
			return ret;
	}

	// cv::dnn::PoolingLayer::setCeilMode(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:255
	// ("cv::dnn::PoolingLayer::setCeilMode", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_dnn_PoolingLayer_propCeilMode_const_bool(cv::dnn::PoolingLayer* instance, const bool val) {
			instance->ceilMode = val;
	}

	// cv::dnn::PoolingLayer::avePoolPaddedArea() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:259
	// ("cv::dnn::PoolingLayer::avePoolPaddedArea", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_PoolingLayer_propAvePoolPaddedArea_const(const cv::dnn::PoolingLayer* instance) {
			bool ret = instance->avePoolPaddedArea;
			return ret;
	}

	// cv::dnn::PoolingLayer::setAvePoolPaddedArea(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:259
	// ("cv::dnn::PoolingLayer::setAvePoolPaddedArea", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_dnn_PoolingLayer_propAvePoolPaddedArea_const_bool(cv::dnn::PoolingLayer* instance, const bool val) {
			instance->avePoolPaddedArea = val;
	}

	// cv::dnn::PoolingLayer::pooledSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:261
	// ("cv::dnn::PoolingLayer::pooledSize", vec![(pred!(const, [], []), _)]),
	void cv_dnn_PoolingLayer_propPooledSize_const(const cv::dnn::PoolingLayer* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->pooledSize;
			*ocvrs_return = ret;
	}

	// cv::dnn::PoolingLayer::setPooledSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:261
	// ("cv::dnn::PoolingLayer::setPooledSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_dnn_PoolingLayer_propPooledSize_const_Size(cv::dnn::PoolingLayer* instance, const cv::Size* val) {
			instance->pooledSize = *val;
	}

	// cv::dnn::PoolingLayer::spatialScale() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:262
	// ("cv::dnn::PoolingLayer::spatialScale", vec![(pred!(const, [], []), _)]),
	float cv_dnn_PoolingLayer_propSpatialScale_const(const cv::dnn::PoolingLayer* instance) {
			float ret = instance->spatialScale;
			return ret;
	}

	// cv::dnn::PoolingLayer::setSpatialScale(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:262
	// ("cv::dnn::PoolingLayer::setSpatialScale", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_PoolingLayer_propSpatialScale_const_float(cv::dnn::PoolingLayer* instance, const float val) {
			instance->spatialScale = val;
	}

	// cv::dnn::PoolingLayer::psRoiOutChannels() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:264
	// ("cv::dnn::PoolingLayer::psRoiOutChannels", vec![(pred!(const, [], []), _)]),
	int cv_dnn_PoolingLayer_propPsRoiOutChannels_const(const cv::dnn::PoolingLayer* instance) {
			int ret = instance->psRoiOutChannels;
			return ret;
	}

	// cv::dnn::PoolingLayer::setPsRoiOutChannels(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:264
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:502
	// ("cv::dnn::PowerLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_PowerLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::PowerLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::PowerLayer> ret = cv::dnn::PowerLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::PowerLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::PowerLayer::power() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:500
	// ("cv::dnn::PowerLayer::power", vec![(pred!(const, [], []), _)]),
	float cv_dnn_PowerLayer_propPower_const(const cv::dnn::PowerLayer* instance) {
			float ret = instance->power;
			return ret;
	}

	// cv::dnn::PowerLayer::setPower(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:500
	// ("cv::dnn::PowerLayer::setPower", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_PowerLayer_propPower_const_float(cv::dnn::PowerLayer* instance, const float val) {
			instance->power = val;
	}

	// cv::dnn::PowerLayer::scale() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:500
	// ("cv::dnn::PowerLayer::scale", vec![(pred!(const, [], []), _)]),
	float cv_dnn_PowerLayer_propScale_const(const cv::dnn::PowerLayer* instance) {
			float ret = instance->scale;
			return ret;
	}

	// cv::dnn::PowerLayer::setScale(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:500
	// ("cv::dnn::PowerLayer::setScale", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_PowerLayer_propScale_const_float(cv::dnn::PowerLayer* instance, const float val) {
			instance->scale = val;
	}

	// cv::dnn::PowerLayer::shift() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:500
	// ("cv::dnn::PowerLayer::shift", vec![(pred!(const, [], []), _)]),
	float cv_dnn_PowerLayer_propShift_const(const cv::dnn::PowerLayer* instance) {
			float ret = instance->shift;
			return ret;
	}

	// cv::dnn::PowerLayer::setShift(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:500
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:595
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:683
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:185
	// ("cv::dnn::RNNLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_RNNLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::RNNLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::RNNLayer> ret = cv::dnn::RNNLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::RNNLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWeights(const Mat &, const Mat &, const Mat &, const Mat &, const Mat &)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:201
	// ("cv::dnn::RNNLayer::setWeights", vec![(pred!(mut, ["Wxh", "bh", "Whh", "Who", "bo"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_dnn_RNNLayer_setWeights_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR(cv::dnn::RNNLayer* instance, const cv::Mat* Wxh, const cv::Mat* bh, const cv::Mat* Whh, const cv::Mat* Who, const cv::Mat* bo, ResultVoid* ocvrs_return) {
		try {
			instance->setWeights(*Wxh, *bh, *Whh, *Who, *bo);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setProduceHiddenOutput(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:206
	// ("cv::dnn::RNNLayer::setProduceHiddenOutput", vec![(pred!(mut, ["produce"], ["bool"]), _)]),
	void cv_dnn_RNNLayer_setProduceHiddenOutput_bool(cv::dnn::RNNLayer* instance, bool produce, ResultVoid* ocvrs_return) {
		try {
			instance->setProduceHiddenOutput(produce);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::RNNLayer::setProduceHiddenOutput() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:206
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:444
	// ("cv::dnn::ReLU6Layer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ReLU6Layer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::ReLU6Layer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::ReLU6Layer> ret = cv::dnn::ReLU6Layer::create(*params);
			Ok(new cv::Ptr<cv::dnn::ReLU6Layer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ReLU6Layer::minValue() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:442
	// ("cv::dnn::ReLU6Layer::minValue", vec![(pred!(const, [], []), _)]),
	float cv_dnn_ReLU6Layer_propMinValue_const(const cv::dnn::ReLU6Layer* instance) {
			float ret = instance->minValue;
			return ret;
	}

	// cv::dnn::ReLU6Layer::setMinValue(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:442
	// ("cv::dnn::ReLU6Layer::setMinValue", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dnn_ReLU6Layer_propMinValue_const_float(cv::dnn::ReLU6Layer* instance, const float val) {
			instance->minValue = val;
	}

	// cv::dnn::ReLU6Layer::maxValue() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:442
	// ("cv::dnn::ReLU6Layer::maxValue", vec![(pred!(const, [], []), _)]),
	float cv_dnn_ReLU6Layer_propMaxValue_const(const cv::dnn::ReLU6Layer* instance) {
			float ret = instance->maxValue;
			return ret;
	}

	// cv::dnn::ReLU6Layer::setMaxValue(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:442
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:436
	// ("cv::dnn::ReLULayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_ReLULayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::ReLULayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::ReLULayer> ret = cv::dnn::ReLULayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::ReLULayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::ReLULayer::negativeSlope() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:434
	// ("cv::dnn::ReLULayer::negativeSlope", vec![(pred!(const, [], []), _)]),
	float cv_dnn_ReLULayer_propNegativeSlope_const(const cv::dnn::ReLULayer* instance) {
			float ret = instance->negativeSlope;
			return ret;
	}

	// cv::dnn::ReLULayer::setNegativeSlope(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:434
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:607
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:601
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:301
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

	// cv::dnn::ReshapeLayer::newShapeDesc() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:298
	// ("cv::dnn::ReshapeLayer::newShapeDesc", vec![(pred!(const, [], []), _)]),
	cv::dnn::MatShape* cv_dnn_ReshapeLayer_propNewShapeDesc_const(const cv::dnn::ReshapeLayer* instance) {
			cv::dnn::MatShape ret = instance->newShapeDesc;
			return new cv::dnn::MatShape(ret);
	}

	// cv::dnn::ReshapeLayer::setNewShapeDesc(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:298
	// ("cv::dnn::ReshapeLayer::setNewShapeDesc", vec![(pred!(mut, ["val"], ["const cv::dnn::MatShape"]), _)]),
	void cv_dnn_ReshapeLayer_propNewShapeDesc_const_MatShape(cv::dnn::ReshapeLayer* instance, const cv::dnn::MatShape* val) {
			instance->newShapeDesc = *val;
	}

	// cv::dnn::ReshapeLayer::newShapeRange() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:299
	// ("cv::dnn::ReshapeLayer::newShapeRange", vec![(pred!(const, [], []), _)]),
	cv::Range* cv_dnn_ReshapeLayer_propNewShapeRange_const(const cv::dnn::ReshapeLayer* instance) {
			cv::Range ret = instance->newShapeRange;
			return new cv::Range(ret);
	}

	// cv::dnn::ReshapeLayer::setNewShapeRange(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:299
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:666
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:559
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

	// cv::dnn::ScaleLayer::hasBias() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:556
	// ("cv::dnn::ScaleLayer::hasBias", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_ScaleLayer_propHasBias_const(const cv::dnn::ScaleLayer* instance) {
			bool ret = instance->hasBias;
			return ret;
	}

	// cv::dnn::ScaleLayer::setHasBias(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:556
	// ("cv::dnn::ScaleLayer::setHasBias", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_dnn_ScaleLayer_propHasBias_const_bool(cv::dnn::ScaleLayer* instance, const bool val) {
			instance->hasBias = val;
	}

	// cv::dnn::ScaleLayer::axis() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:557
	// ("cv::dnn::ScaleLayer::axis", vec![(pred!(const, [], []), _)]),
	int cv_dnn_ScaleLayer_propAxis_const(const cv::dnn::ScaleLayer* instance) {
			int ret = instance->axis;
			return ret;
	}

	// cv::dnn::ScaleLayer::setAxis(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:557
	// ("cv::dnn::ScaleLayer::setAxis", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_ScaleLayer_propAxis_const_int(cv::dnn::ScaleLayer* instance, const int val) {
			instance->axis = val;
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:565
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:392
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

	// cv::dnn::ShuffleChannelLayer::group() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:394
	// ("cv::dnn::ShuffleChannelLayer::group", vec![(pred!(const, [], []), _)]),
	int cv_dnn_ShuffleChannelLayer_propGroup_const(const cv::dnn::ShuffleChannelLayer* instance) {
			int ret = instance->group;
			return ret;
	}

	// cv::dnn::ShuffleChannelLayer::setGroup(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:394
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:482
	// ("cv::dnn::SigmoidLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_SigmoidLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::SigmoidLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::SigmoidLayer> ret = cv::dnn::SigmoidLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::SigmoidLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:371
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

	// cv::dnn::SliceLayer::sliceRanges() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:366
	// ("cv::dnn::SliceLayer::sliceRanges", vec![(pred!(const, [], []), _)]),
	std::vector<std::vector<cv::Range>>* cv_dnn_SliceLayer_propSliceRanges_const(const cv::dnn::SliceLayer* instance) {
			std::vector<std::vector<cv::Range>> ret = instance->sliceRanges;
			return new std::vector<std::vector<cv::Range>>(ret);
	}

	// cv::dnn::SliceLayer::setSliceRanges(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:366
	// ("cv::dnn::SliceLayer::setSliceRanges", vec![(pred!(mut, ["val"], ["const std::vector<std::vector<cv::Range>>"]), _)]),
	void cv_dnn_SliceLayer_propSliceRanges_const_vectorLvectorLRangeGG(cv::dnn::SliceLayer* instance, const std::vector<std::vector<cv::Range>>* val) {
			instance->sliceRanges = *val;
	}

	// cv::dnn::SliceLayer::sliceSteps() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:367
	// ("cv::dnn::SliceLayer::sliceSteps", vec![(pred!(const, [], []), _)]),
	std::vector<std::vector<int>>* cv_dnn_SliceLayer_propSliceSteps_const(const cv::dnn::SliceLayer* instance) {
			std::vector<std::vector<int>> ret = instance->sliceSteps;
			return new std::vector<std::vector<int>>(ret);
	}

	// cv::dnn::SliceLayer::setSliceSteps(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:367
	// ("cv::dnn::SliceLayer::setSliceSteps", vec![(pred!(mut, ["val"], ["const std::vector<std::vector<int>>"]), _)]),
	void cv_dnn_SliceLayer_propSliceSteps_const_vectorLvectorLintGG(cv::dnn::SliceLayer* instance, const std::vector<std::vector<int>>* val) {
			instance->sliceSteps = *val;
	}

	// cv::dnn::SliceLayer::axis() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:368
	// ("cv::dnn::SliceLayer::axis", vec![(pred!(const, [], []), _)]),
	int cv_dnn_SliceLayer_propAxis_const(const cv::dnn::SliceLayer* instance) {
			int ret = instance->axis;
			return ret;
	}

	// cv::dnn::SliceLayer::setAxis(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:368
	// ("cv::dnn::SliceLayer::setAxis", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dnn_SliceLayer_propAxis_const_int(cv::dnn::SliceLayer* instance, const int val) {
			instance->axis = val;
	}

	// cv::dnn::SliceLayer::num_split() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:369
	// ("cv::dnn::SliceLayer::num_split", vec![(pred!(const, [], []), _)]),
	int cv_dnn_SliceLayer_propNum_split_const(const cv::dnn::SliceLayer* instance) {
			int ret = instance->num_split;
			return ret;
	}

	// cv::dnn::SliceLayer::setNum_split(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:369
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:274
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

	// cv::dnn::SoftmaxLayer::logSoftMax() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:272
	// ("cv::dnn::SoftmaxLayer::logSoftMax", vec![(pred!(const, [], []), _)]),
	bool cv_dnn_SoftmaxLayer_propLogSoftMax_const(const cv::dnn::SoftmaxLayer* instance) {
			bool ret = instance->logSoftMax;
			return ret;
	}

	// cv::dnn::SoftmaxLayer::setLogSoftMax(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:272
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:330
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

	// cv::dnn::SplitLayer::outputsCount() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:328
	// ("cv::dnn::SplitLayer::outputsCount", vec![(pred!(const, [], []), _)]),
	int cv_dnn_SplitLayer_propOutputsCount_const(const cv::dnn::SplitLayer* instance) {
			int ret = instance->outputsCount;
			return ret;
	}

	// cv::dnn::SplitLayer::setOutputsCount(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:328
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:470
	// ("cv::dnn::SwishLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_SwishLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::SwishLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::SwishLayer> ret = cv::dnn::SwishLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::SwishLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
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

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:464
	// ("cv::dnn::TanHLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	void cv_dnn_TanHLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params, Result<cv::Ptr<cv::dnn::TanHLayer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dnn::TanHLayer> ret = cv::dnn::TanHLayer::create(*params);
			Ok(new cv::Ptr<cv::dnn::TanHLayer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
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

	// _Range(const Range &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:59
	// ("cv::dnn::_Range::_Range", vec![(pred!(mut, ["r"], ["const cv::Range*"]), _)]),
	void cv_dnn__Range__Range_const_RangeR(const cv::Range* r, Result<cv::dnn::_Range*>* ocvrs_return) {
		try {
			cv::dnn::_Range* ret = new cv::dnn::_Range(*r);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// _Range(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:60
	// ("cv::dnn::_Range::_Range", vec![(pred!(mut, ["start_", "size_"], ["int", "int"]), _)]),
	void cv_dnn__Range__Range_int_int(int start_, int size_, Result<cv::dnn::_Range*>* ocvrs_return) {
		try {
			cv::dnn::_Range* ret = new cv::dnn::_Range(start_, size_);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dnn::_Range::_Range(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:60
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
