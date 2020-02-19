#include "dnn.hpp"
#include "dnn_types.hpp"

extern "C" {
	Result_void cv_dnn_NMSBoxes_const_vector_Rect2d_X_const_vector_float_X_float_float_vector_int_X_float_int(void* bboxes, void* scores, float score_threshold, float nms_threshold, void* indices, float eta, int top_k) {
		try {
			cv::dnn::NMSBoxes(*reinterpret_cast<const std::vector<cv::Rect2d>*>(bboxes), *reinterpret_cast<const std::vector<float>*>(scores), score_threshold, nms_threshold, *reinterpret_cast<std::vector<int>*>(indices), eta, top_k);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_NMSBoxes_const_vector_Rect_X_const_vector_float_X_float_float_vector_int_X_float_int(void* bboxes, void* scores, float score_threshold, float nms_threshold, void* indices, float eta, int top_k) {
		try {
			cv::dnn::NMSBoxes(*reinterpret_cast<const std::vector<cv::Rect>*>(bboxes), *reinterpret_cast<const std::vector<float>*>(scores), score_threshold, nms_threshold, *reinterpret_cast<std::vector<int>*>(indices), eta, top_k);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_NMSBoxes_const_vector_RotatedRect_X_const_vector_float_X_float_float_vector_int_X_float_int(void* bboxes, void* scores, float score_threshold, float nms_threshold, void* indices, float eta, int top_k) {
		try {
			cv::dnn::NMSBoxes(*reinterpret_cast<const std::vector<cv::RotatedRect>*>(bboxes), *reinterpret_cast<const std::vector<float>*>(scores), score_threshold, nms_threshold, *reinterpret_cast<std::vector<int>*>(indices), eta, top_k);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_blobFromImage_const__InputArrayX_const__OutputArrayX_double_const_SizeX_const_ScalarX_bool_bool_int(void* image, void* blob, double scalefactor, const cv::Size* size, const cv::Scalar* mean, bool swapRB, bool crop, int ddepth) {
		try {
			cv::dnn::blobFromImage(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_OutputArray*>(blob), scalefactor, *size, *mean, swapRB, crop, ddepth);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_blobFromImage_const__InputArrayX_double_const_SizeX_const_ScalarX_bool_bool_int(void* image, double scalefactor, const cv::Size* size, const cv::Scalar* mean, bool swapRB, bool crop, int ddepth) {
		try {
			cv::Mat ret = cv::dnn::blobFromImage(*reinterpret_cast<const cv::_InputArray*>(image), scalefactor, *size, *mean, swapRB, crop, ddepth);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_blobFromImages_const__InputArrayX_const__OutputArrayX_double_Size_const_ScalarX_bool_bool_int(void* images, void* blob, double scalefactor, cv::Size size, const cv::Scalar* mean, bool swapRB, bool crop, int ddepth) {
		try {
			cv::dnn::blobFromImages(*reinterpret_cast<const cv::_InputArray*>(images), *reinterpret_cast<const cv::_OutputArray*>(blob), scalefactor, size, *mean, swapRB, crop, ddepth);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_blobFromImages_const__InputArrayX_double_Size_const_ScalarX_bool_bool_int(void* images, double scalefactor, cv::Size size, const cv::Scalar* mean, bool swapRB, bool crop, int ddepth) {
		try {
			cv::Mat ret = cv::dnn::blobFromImages(*reinterpret_cast<const cv::_InputArray*>(images), scalefactor, size, *mean, swapRB, crop, ddepth);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_clamp_const_RangeX_int(void* r, int axisSize) {
		try {
			cv::Range ret = cv::dnn::clamp(*reinterpret_cast<const cv::Range*>(r), axisSize);
			return Ok<void*>(new cv::Range(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_dnn_clamp_int_const_MatShapeX(int ax, void* shape) {
		try {
			int ret = cv::dnn::clamp(ax, *reinterpret_cast<const cv::dnn::MatShape*>(shape));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_dnn_clamp_int_int(int ax, int dims) {
		try {
			int ret = cv::dnn::clamp(ax, dims);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv_dnn_concat_const_MatShapeX_const_MatShapeX(void* a, void* b) {
		try {
			cv::dnn::MatShape ret = cv::dnn::concat(*reinterpret_cast<const cv::dnn::MatShape*>(a), *reinterpret_cast<const cv::dnn::MatShape*>(b));
			return Ok<void*>(new cv::dnn::MatShape(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_getAvailableTargets_Backend(cv::dnn::Backend be) {
		try {
			std::vector<cv::dnn::Target> ret = cv::dnn::getAvailableTargets(be);
			return Ok<void*>(new std::vector<cv::dnn::Target>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_getInferenceEngineBackendType() {
		try {
			cv::String ret = cv::dnn::getInferenceEngineBackendType();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_getInferenceEngineVPUType() {
		try {
			cv::String ret = cv::dnn::getInferenceEngineVPUType();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_getPlane_const_MatX_int_int(void* m, int n, int cn) {
		try {
			cv::Mat ret = cv::dnn::getPlane(*reinterpret_cast<const cv::Mat*>(m), n, cn);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_imagesFromBlob_const_MatX_const__OutputArrayX(void* blob_, void* images_) {
		try {
			cv::dnn::imagesFromBlob(*reinterpret_cast<const cv::Mat*>(blob_), *reinterpret_cast<const cv::_OutputArray*>(images_));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_print_const_MatShapeX_const_StringX(void* shape, const char* name) {
		try {
			cv::dnn::print(*reinterpret_cast<const cv::dnn::MatShape*>(shape), std::string(name));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_readNetFromCaffe_const_StringX_const_StringX(const char* prototxt, const char* caffeModel) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromCaffe(std::string(prototxt), std::string(caffeModel));
			return Ok<void*>(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_readNetFromCaffe_const_charX_size_t_const_charX_size_t(const char* bufferProto, size_t lenProto, const char* bufferModel, size_t lenModel) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromCaffe(bufferProto, lenProto, bufferModel, lenModel);
			return Ok<void*>(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_readNetFromCaffe_const_vector_unsigned_char_X_const_vector_unsigned_char_X(void* bufferProto, void* bufferModel) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromCaffe(*reinterpret_cast<const std::vector<unsigned char>*>(bufferProto), *reinterpret_cast<const std::vector<unsigned char>*>(bufferModel));
			return Ok<void*>(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_readNetFromDarknet_const_StringX_const_StringX(const char* cfgFile, const char* darknetModel) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromDarknet(std::string(cfgFile), std::string(darknetModel));
			return Ok<void*>(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_readNetFromDarknet_const_charX_size_t_const_charX_size_t(const char* bufferCfg, size_t lenCfg, const char* bufferModel, size_t lenModel) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromDarknet(bufferCfg, lenCfg, bufferModel, lenModel);
			return Ok<void*>(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_readNetFromDarknet_const_vector_unsigned_char_X_const_vector_unsigned_char_X(void* bufferCfg, void* bufferModel) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromDarknet(*reinterpret_cast<const std::vector<unsigned char>*>(bufferCfg), *reinterpret_cast<const std::vector<unsigned char>*>(bufferModel));
			return Ok<void*>(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_readNetFromModelOptimizer_const_StringX_const_StringX(const char* xml, const char* bin) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromModelOptimizer(std::string(xml), std::string(bin));
			return Ok<void*>(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_readNetFromModelOptimizer_const_unsigned_charX_size_t_const_unsigned_charX_size_t(const unsigned char* bufferModelConfigPtr, size_t bufferModelConfigSize, const unsigned char* bufferWeightsPtr, size_t bufferWeightsSize) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromModelOptimizer(bufferModelConfigPtr, bufferModelConfigSize, bufferWeightsPtr, bufferWeightsSize);
			return Ok<void*>(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_readNetFromModelOptimizer_const_vector_unsigned_char_X_const_vector_unsigned_char_X(void* bufferModelConfig, void* bufferWeights) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromModelOptimizer(*reinterpret_cast<const std::vector<unsigned char>*>(bufferModelConfig), *reinterpret_cast<const std::vector<unsigned char>*>(bufferWeights));
			return Ok<void*>(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_readNetFromONNX_const_StringX(const char* onnxFile) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromONNX(std::string(onnxFile));
			return Ok<void*>(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_readNetFromONNX_const_charX_size_t(const char* buffer, size_t sizeBuffer) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromONNX(buffer, sizeBuffer);
			return Ok<void*>(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_readNetFromONNX_const_vector_unsigned_char_X(void* buffer) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromONNX(*reinterpret_cast<const std::vector<unsigned char>*>(buffer));
			return Ok<void*>(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_readNetFromTensorflow_const_StringX_const_StringX(const char* model, const char* config) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromTensorflow(std::string(model), std::string(config));
			return Ok<void*>(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_readNetFromTensorflow_const_charX_size_t_const_charX_size_t(const char* bufferModel, size_t lenModel, const char* bufferConfig, size_t lenConfig) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromTensorflow(bufferModel, lenModel, bufferConfig, lenConfig);
			return Ok<void*>(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_readNetFromTensorflow_const_vector_unsigned_char_X_const_vector_unsigned_char_X(void* bufferModel, void* bufferConfig) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromTensorflow(*reinterpret_cast<const std::vector<unsigned char>*>(bufferModel), *reinterpret_cast<const std::vector<unsigned char>*>(bufferConfig));
			return Ok<void*>(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_readNetFromTorch_const_StringX_bool_bool(const char* model, bool isBinary, bool evaluate) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromTorch(std::string(model), isBinary, evaluate);
			return Ok<void*>(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_readNet_const_StringX_const_StringX_const_StringX(const char* model, const char* config, const char* framework) {
		try {
			cv::dnn::Net ret = cv::dnn::readNet(std::string(model), std::string(config), std::string(framework));
			return Ok<void*>(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_readNet_const_StringX_const_vector_unsigned_char_X_const_vector_unsigned_char_X(const char* framework, void* bufferModel, void* bufferConfig) {
		try {
			cv::dnn::Net ret = cv::dnn::readNet(std::string(framework), *reinterpret_cast<const std::vector<unsigned char>*>(bufferModel), *reinterpret_cast<const std::vector<unsigned char>*>(bufferConfig));
			return Ok<void*>(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_readTensorFromONNX_const_StringX(const char* path) {
		try {
			cv::Mat ret = cv::dnn::readTensorFromONNX(std::string(path));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_readTorchBlob_const_StringX_bool(const char* filename, bool isBinary) {
		try {
			cv::Mat ret = cv::dnn::readTorchBlob(std::string(filename), isBinary);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_resetMyriadDevice() {
		try {
			cv::dnn::resetMyriadDevice();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_setInferenceEngineBackendType_const_StringX(const char* newBackendType) {
		try {
			cv::String ret = cv::dnn::setInferenceEngineBackendType(std::string(newBackendType));
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_shape_const_MatSizeX(void* sz) {
		try {
			cv::dnn::MatShape ret = cv::dnn::shape(*reinterpret_cast<const cv::MatSize*>(sz));
			return Ok<void*>(new cv::dnn::MatShape(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_shape_const_MatX(void* mat) {
		try {
			cv::dnn::MatShape ret = cv::dnn::shape(*reinterpret_cast<const cv::Mat*>(mat));
			return Ok<void*>(new cv::dnn::MatShape(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_shape_const_UMatX(void* mat) {
		try {
			cv::dnn::MatShape ret = cv::dnn::shape(*reinterpret_cast<const cv::UMat*>(mat));
			return Ok<void*>(new cv::dnn::MatShape(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_shape_const_intX_int(const int* dims, int n) {
		try {
			cv::dnn::MatShape ret = cv::dnn::shape(dims, n);
			return Ok<void*>(new cv::dnn::MatShape(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_shape_int_int_int_int(int a0, int a1, int a2, int a3) {
		try {
			cv::dnn::MatShape ret = cv::dnn::shape(a0, a1, a2, a3);
			return Ok<void*>(new cv::dnn::MatShape(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_shrinkCaffeModel_const_StringX_const_StringX_const_vector_String_X(const char* src, const char* dst, void* layersTypes) {
		try {
			cv::dnn::shrinkCaffeModel(std::string(src), std::string(dst), *reinterpret_cast<const std::vector<cv::String>*>(layersTypes));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_slice_const_MatX_const__RangeX(void* m, void* r0) {
		try {
			cv::Mat ret = cv::dnn::slice(*reinterpret_cast<const cv::Mat*>(m), *reinterpret_cast<const cv::dnn::_Range*>(r0));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_slice_const_MatX_const__RangeX_const__RangeX(void* m, void* r0, void* r1) {
		try {
			cv::Mat ret = cv::dnn::slice(*reinterpret_cast<const cv::Mat*>(m), *reinterpret_cast<const cv::dnn::_Range*>(r0), *reinterpret_cast<const cv::dnn::_Range*>(r1));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_slice_const_MatX_const__RangeX_const__RangeX_const__RangeX(void* m, void* r0, void* r1, void* r2) {
		try {
			cv::Mat ret = cv::dnn::slice(*reinterpret_cast<const cv::Mat*>(m), *reinterpret_cast<const cv::dnn::_Range*>(r0), *reinterpret_cast<const cv::dnn::_Range*>(r1), *reinterpret_cast<const cv::dnn::_Range*>(r2));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_slice_const_MatX_const__RangeX_const__RangeX_const__RangeX_const__RangeX(void* m, void* r0, void* r1, void* r2, void* r3) {
		try {
			cv::Mat ret = cv::dnn::slice(*reinterpret_cast<const cv::Mat*>(m), *reinterpret_cast<const cv::dnn::_Range*>(r0), *reinterpret_cast<const cv::dnn::_Range*>(r1), *reinterpret_cast<const cv::dnn::_Range*>(r2), *reinterpret_cast<const cv::dnn::_Range*>(r3));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_toString_const_MatShapeX_const_StringX(void* shape, const char* name) {
		try {
			std::string ret = cv::dnn::toString(*reinterpret_cast<const cv::dnn::MatShape*>(shape), std::string(name));
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_dnn_total_const_MatShapeX_int_int(void* shape, int start, int end) {
		try {
			int ret = cv::dnn::total(*reinterpret_cast<const cv::dnn::MatShape*>(shape), start, end);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_writeTextGraph_const_StringX_const_StringX(const char* model, const char* output) {
		try {
			cv::dnn::writeTextGraph(std::string(model), std::string(output));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_AbsLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::AbsLayer> ret = cv::dnn::AbsLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::AbsLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_ActivationLayer_forwardSlice_const_const_floatX_floatX_int_size_t_int_int(void* instance, const float* src, float* dst, int len, size_t outPlaneSize, int cn0, int cn1) {
		try {
			reinterpret_cast<cv::dnn::ActivationLayer*>(instance)->forwardSlice(src, dst, len, outPlaneSize, cn0, cn1);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_BNLLLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::BNLLLayer> ret = cv::dnn::BNLLLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::BNLLLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_dnn_BackendNode_backendId_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::BackendNode*>(instance)->backendId;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_BackendNode_setBackendId_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::dnn::BackendNode*>(instance)->backendId = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_BackendNode_delete(cv::dnn::BackendNode* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_BackendNode_BackendNode_int(int backendId) {
		try {
			cv::dnn::BackendNode* ret = new cv::dnn::BackendNode(backendId);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_dnn_BackendWrapper_backendId_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::BackendWrapper*>(instance)->backendId;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_BackendWrapper_setBackendId_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::dnn::BackendWrapper*>(instance)->backendId = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_BackendWrapper_targetId_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::BackendWrapper*>(instance)->targetId;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_BackendWrapper_setTargetId_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::dnn::BackendWrapper*>(instance)->targetId = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_BackendWrapper_copyToHost(void* instance) {
		try {
			reinterpret_cast<cv::dnn::BackendWrapper*>(instance)->copyToHost();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_BackendWrapper_setHostDirty(void* instance) {
		try {
			reinterpret_cast<cv::dnn::BackendWrapper*>(instance)->setHostDirty();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_dnn_BaseConvolutionLayer_kernel_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->kernel;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setKernel_Size(void* instance, cv::Size val) {
		try {
			reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->kernel = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_dnn_BaseConvolutionLayer_stride_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->stride;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setStride_Size(void* instance, cv::Size val) {
		try {
			reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->stride = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_dnn_BaseConvolutionLayer_pad_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->pad;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setPad_Size(void* instance, cv::Size val) {
		try {
			reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->pad = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_dnn_BaseConvolutionLayer_dilation_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->dilation;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setDilation_Size(void* instance, cv::Size val) {
		try {
			reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->dilation = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_dnn_BaseConvolutionLayer_adjustPad_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->adjustPad;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setAdjustPad_Size(void* instance, cv::Size val) {
		try {
			reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->adjustPad = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_BaseConvolutionLayer_adjust_pads(void* instance) {
		try {
			std::vector<size_t> ret = reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->adjust_pads;
			return Ok<void*>(new std::vector<size_t>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setAdjust_pads_vector_size_t_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->adjust_pads = *reinterpret_cast<std::vector<size_t>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_BaseConvolutionLayer_kernel_size(void* instance) {
		try {
			std::vector<size_t> ret = reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->kernel_size;
			return Ok<void*>(new std::vector<size_t>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setKernel_size_vector_size_t_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->kernel_size = *reinterpret_cast<std::vector<size_t>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_BaseConvolutionLayer_strides(void* instance) {
		try {
			std::vector<size_t> ret = reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->strides;
			return Ok<void*>(new std::vector<size_t>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setStrides_vector_size_t_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->strides = *reinterpret_cast<std::vector<size_t>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_BaseConvolutionLayer_dilations(void* instance) {
		try {
			std::vector<size_t> ret = reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->dilations;
			return Ok<void*>(new std::vector<size_t>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setDilations_vector_size_t_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->dilations = *reinterpret_cast<std::vector<size_t>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_BaseConvolutionLayer_pads_begin(void* instance) {
		try {
			std::vector<size_t> ret = reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->pads_begin;
			return Ok<void*>(new std::vector<size_t>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setPads_begin_vector_size_t_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->pads_begin = *reinterpret_cast<std::vector<size_t>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_BaseConvolutionLayer_pads_end(void* instance) {
		try {
			std::vector<size_t> ret = reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->pads_end;
			return Ok<void*>(new std::vector<size_t>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setPads_end_vector_size_t_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->pads_end = *reinterpret_cast<std::vector<size_t>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_BaseConvolutionLayer_padMode_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->padMode;
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setPadMode_String(void* instance, char* val) {
		try {
			reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->padMode = std::string(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_BaseConvolutionLayer_numOutput_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->numOutput;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setNumOutput_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::dnn::BaseConvolutionLayer*>(instance)->numOutput = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_BaseConvolutionLayer_delete(cv::dnn::BaseConvolutionLayer* instance) {
		delete instance;
	}
	Result<bool> cv_dnn_BatchNormLayer_hasWeights_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::dnn::BatchNormLayer*>(instance)->hasWeights;
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_BatchNormLayer_setHasWeights_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::dnn::BatchNormLayer*>(instance)->hasWeights = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_dnn_BatchNormLayer_hasBias_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::dnn::BatchNormLayer*>(instance)->hasBias;
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_BatchNormLayer_setHasBias_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::dnn::BatchNormLayer*>(instance)->hasBias = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_dnn_BatchNormLayer_epsilon_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::dnn::BatchNormLayer*>(instance)->epsilon;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_dnn_BatchNormLayer_setEpsilon_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::dnn::BatchNormLayer*>(instance)->epsilon = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_BatchNormLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::BatchNormLayer> ret = cv::dnn::BatchNormLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::BatchNormLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_BlankLayer_delete(cv::dnn::BlankLayer* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_BlankLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::BlankLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_ChannelsPReLULayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::ChannelsPReLULayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_ClassificationModel_delete(cv::dnn::ClassificationModel* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_ClassificationModel_ClassificationModel_const_StringX_const_StringX(const char* model, const char* config) {
		try {
			cv::dnn::ClassificationModel* ret = new cv::dnn::ClassificationModel(std::string(model), std::string(config));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_ClassificationModel_ClassificationModel_const_NetX(void* network) {
		try {
			cv::dnn::ClassificationModel* ret = new cv::dnn::ClassificationModel(*reinterpret_cast<const cv::dnn::Net*>(network));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_ClassificationModel_classify_const__InputArrayX_intX_floatX(void* instance, void* frame, int* classId, float* conf) {
		try {
			reinterpret_cast<cv::dnn::ClassificationModel*>(instance)->classify(*reinterpret_cast<const cv::_InputArray*>(frame), *classId, *conf);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_ConcatLayer_axis_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::ConcatLayer*>(instance)->axis;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_ConcatLayer_setAxis_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::dnn::ConcatLayer*>(instance)->axis = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_dnn_ConcatLayer_padding_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::dnn::ConcatLayer*>(instance)->padding;
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_ConcatLayer_setPadding_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::dnn::ConcatLayer*>(instance)->padding = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_ConcatLayer_delete(cv::dnn::ConcatLayer* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_ConcatLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::ConcatLayer> ret = cv::dnn::ConcatLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::ConcatLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_ConstLayer_delete(cv::dnn::ConstLayer* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_ConstLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::ConstLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_ConvolutionLayer_delete(cv::dnn::ConvolutionLayer* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_ConvolutionLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::BaseConvolutionLayer> ret = cv::dnn::ConvolutionLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::BaseConvolutionLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_CropAndResizeLayer_delete(cv::dnn::CropAndResizeLayer* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_CropAndResizeLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::CropAndResizeLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_CropLayer_delete(cv::dnn::CropLayer* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_CropLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::CropLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_DeconvolutionLayer_delete(cv::dnn::DeconvolutionLayer* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_DeconvolutionLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::BaseConvolutionLayer> ret = cv::dnn::DeconvolutionLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::BaseConvolutionLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_DetectionModel_delete(cv::dnn::DetectionModel* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_DetectionModel_DetectionModel_const_StringX_const_StringX(const char* model, const char* config) {
		try {
			cv::dnn::DetectionModel* ret = new cv::dnn::DetectionModel(std::string(model), std::string(config));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_DetectionModel_DetectionModel_const_NetX(void* network) {
		try {
			cv::dnn::DetectionModel* ret = new cv::dnn::DetectionModel(*reinterpret_cast<const cv::dnn::Net*>(network));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_DetectionModel_detect_const__InputArrayX_vector_int_X_vector_float_X_vector_Rect_X_float_float(void* instance, void* frame, void* classIds, void* confidences, void* boxes, float confThreshold, float nmsThreshold) {
		try {
			reinterpret_cast<cv::dnn::DetectionModel*>(instance)->detect(*reinterpret_cast<const cv::_InputArray*>(frame), *reinterpret_cast<std::vector<int>*>(classIds), *reinterpret_cast<std::vector<float>*>(confidences), *reinterpret_cast<std::vector<cv::Rect>*>(boxes), confThreshold, nmsThreshold);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_DetectionOutputLayer_delete(cv::dnn::DetectionOutputLayer* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_DetectionOutputLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::DetectionOutputLayer> ret = cv::dnn::DetectionOutputLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::DetectionOutputLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Dict_delete(cv::dnn::Dict* instance) {
		delete instance;
	}
	Result<bool> cv_dnn_Dict_has_const_const_StringX(void* instance, const char* key) {
		try {
			bool ret = reinterpret_cast<cv::dnn::Dict*>(instance)->has(std::string(key));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_dnn_Dict_ptr_const_StringX(void* instance, const char* key) {
		try {
			cv::dnn::DictValue* ret = reinterpret_cast<cv::dnn::Dict*>(instance)->ptr(std::string(key));
			return Ok<void*>(new cv::dnn::DictValue*(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Dict_ptr_const_const_StringX(void* instance, const char* key) {
		try {
			const cv::dnn::DictValue* ret = reinterpret_cast<cv::dnn::Dict*>(instance)->ptr(std::string(key));
			return Ok<void*>(new const cv::dnn::DictValue*(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Dict_get_const_const_StringX(void* instance, const char* key) {
		try {
			cv::dnn::DictValue ret = reinterpret_cast<cv::dnn::Dict*>(instance)->get(std::string(key));
			return Ok<void*>(new cv::dnn::DictValue(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Dict_set_cv_String_const_StringX_const_StringX(void* instance, const char* key, const char* value) {
		try {
			cv::String ret = reinterpret_cast<cv::dnn::Dict*>(instance)->set<cv::String>(std::string(key), std::string(value));
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Dict_set_cv_dnn_DictValue_const_StringX_const_DictValueX(void* instance, const char* key, void* value) {
		try {
			cv::dnn::DictValue ret = reinterpret_cast<cv::dnn::Dict*>(instance)->set<cv::dnn::DictValue>(std::string(key), *reinterpret_cast<const cv::dnn::DictValue*>(value));
			return Ok<void*>(new cv::dnn::DictValue(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<double> cv_dnn_Dict_set_double_const_StringX_const_doubleX(void* instance, const char* key, const double* value) {
		try {
			double ret = reinterpret_cast<cv::dnn::Dict*>(instance)->set<double>(std::string(key), *value);
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<int64_t> cv_dnn_Dict_set_int64_t_const_StringX_const_int64_tX(void* instance, const char* key, const int64_t* value) {
		try {
			int64_t ret = reinterpret_cast<cv::dnn::Dict*>(instance)->set<int64_t>(std::string(key), *value);
			return Ok<int64_t>(ret);
		} OCVRS_CATCH(Result<int64_t>)
	}
	
	Result_void cv_dnn_Dict_erase_const_StringX(void* instance, const char* key) {
		try {
			reinterpret_cast<cv::dnn::Dict*>(instance)->erase(std::string(key));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_DictValue_delete(cv::dnn::DictValue* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_DictValue_DictValue_const_DictValueX(void* r) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(*reinterpret_cast<const cv::dnn::DictValue*>(r));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_DictValue_DictValue_bool(bool i) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(i);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_DictValue_DictValue_int64_t(int64_t i) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(i);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_DictValue_DictValue_int(int i) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(i);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_DictValue_DictValue_unsigned_int(unsigned int p) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(p);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_DictValue_DictValue_double(double p) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(p);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_DictValue_DictValue_const_charX(const char* s) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(s);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_DictValue_get_cv_String_const_int(void* instance, int idx) {
		try {
			cv::String ret = reinterpret_cast<cv::dnn::DictValue*>(instance)->get<cv::String>(idx);
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<double> cv_dnn_DictValue_get_double_const_int(void* instance, int idx) {
		try {
			double ret = reinterpret_cast<cv::dnn::DictValue*>(instance)->get<double>(idx);
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<int> cv_dnn_DictValue_get_int_const_int(void* instance, int idx) {
		try {
			int ret = reinterpret_cast<cv::dnn::DictValue*>(instance)->get<int>(idx);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int64_t> cv_dnn_DictValue_get_int64_t_const_int(void* instance, int idx) {
		try {
			int64_t ret = reinterpret_cast<cv::dnn::DictValue*>(instance)->get<int64_t>(idx);
			return Ok<int64_t>(ret);
		} OCVRS_CATCH(Result<int64_t>)
	}
	
	Result<int> cv_dnn_DictValue_size_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::DictValue*>(instance)->size();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<bool> cv_dnn_DictValue_isInt_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::dnn::DictValue*>(instance)->isInt();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_dnn_DictValue_isString_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::dnn::DictValue*>(instance)->isString();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_dnn_DictValue_isReal_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::dnn::DictValue*>(instance)->isReal();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<int> cv_dnn_DictValue_getIntValue_const_int(void* instance, int idx) {
		try {
			int ret = reinterpret_cast<cv::dnn::DictValue*>(instance)->getIntValue(idx);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<double> cv_dnn_DictValue_getRealValue_const_int(void* instance, int idx) {
		try {
			double ret = reinterpret_cast<cv::dnn::DictValue*>(instance)->getRealValue(idx);
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<void*> cv_dnn_DictValue_getStringValue_const_int(void* instance, int idx) {
		try {
			cv::String ret = reinterpret_cast<cv::dnn::DictValue*>(instance)->getStringValue(idx);
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_ELULayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::ELULayer> ret = cv::dnn::ELULayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::ELULayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_EltwiseLayer_delete(cv::dnn::EltwiseLayer* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_EltwiseLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::EltwiseLayer> ret = cv::dnn::EltwiseLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::EltwiseLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_FlattenLayer_delete(cv::dnn::FlattenLayer* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_FlattenLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::FlattenLayer> ret = cv::dnn::FlattenLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::FlattenLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_dnn_InnerProductLayer_axis_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::InnerProductLayer*>(instance)->axis;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_InnerProductLayer_setAxis_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::dnn::InnerProductLayer*>(instance)->axis = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_InnerProductLayer_delete(cv::dnn::InnerProductLayer* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_InnerProductLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::InnerProductLayer> ret = cv::dnn::InnerProductLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::InnerProductLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_InterpLayer_delete(cv::dnn::InterpLayer* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_InterpLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::InterpLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_KeypointsModel_delete(cv::dnn::KeypointsModel* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_KeypointsModel_KeypointsModel_const_StringX_const_StringX(const char* model, const char* config) {
		try {
			cv::dnn::KeypointsModel* ret = new cv::dnn::KeypointsModel(std::string(model), std::string(config));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_KeypointsModel_KeypointsModel_const_NetX(void* network) {
		try {
			cv::dnn::KeypointsModel* ret = new cv::dnn::KeypointsModel(*reinterpret_cast<const cv::dnn::Net*>(network));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_KeypointsModel_estimate_const__InputArrayX_float(void* instance, void* frame, float thresh) {
		try {
			std::vector<cv::Point2f> ret = reinterpret_cast<cv::dnn::KeypointsModel*>(instance)->estimate(*reinterpret_cast<const cv::_InputArray*>(frame), thresh);
			return Ok<void*>(new std::vector<cv::Point2f>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_dnn_LRNLayer_type_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::LRNLayer*>(instance)->type;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_LRNLayer_setType_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::dnn::LRNLayer*>(instance)->type = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_LRNLayer_size_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::LRNLayer*>(instance)->size;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_LRNLayer_setSize_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::dnn::LRNLayer*>(instance)->size = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_dnn_LRNLayer_alpha_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::dnn::LRNLayer*>(instance)->alpha;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_dnn_LRNLayer_setAlpha_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::dnn::LRNLayer*>(instance)->alpha = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_dnn_LRNLayer_beta_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::dnn::LRNLayer*>(instance)->beta;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_dnn_LRNLayer_setBeta_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::dnn::LRNLayer*>(instance)->beta = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_dnn_LRNLayer_bias_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::dnn::LRNLayer*>(instance)->bias;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_dnn_LRNLayer_setBias_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::dnn::LRNLayer*>(instance)->bias = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_dnn_LRNLayer_normBySize_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::dnn::LRNLayer*>(instance)->normBySize;
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_LRNLayer_setNormBySize_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::dnn::LRNLayer*>(instance)->normBySize = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_LRNLayer_delete(cv::dnn::LRNLayer* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_LRNLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::LRNLayer> ret = cv::dnn::LRNLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::LRNLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_LSTMLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::LSTMLayer> ret = cv::dnn::LSTMLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::LSTMLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_LSTMLayer_setWeights_const_MatX_const_MatX_const_MatX(void* instance, void* Wh, void* Wx, void* b) {
		try {
			reinterpret_cast<cv::dnn::LSTMLayer*>(instance)->setWeights(*reinterpret_cast<const cv::Mat*>(Wh), *reinterpret_cast<const cv::Mat*>(Wx), *reinterpret_cast<const cv::Mat*>(b));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_LSTMLayer_setOutShape_const_MatShapeX(void* instance, void* outTailShape) {
		try {
			reinterpret_cast<cv::dnn::LSTMLayer*>(instance)->setOutShape(*reinterpret_cast<const cv::dnn::MatShape*>(outTailShape));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_LSTMLayer_setUseTimstampsDim_bool(void* instance, bool use) {
		try {
			reinterpret_cast<cv::dnn::LSTMLayer*>(instance)->setUseTimstampsDim(use);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_LSTMLayer_setProduceCellOutput_bool(void* instance, bool produce) {
		try {
			reinterpret_cast<cv::dnn::LSTMLayer*>(instance)->setProduceCellOutput(produce);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_LSTMLayer_inputNameToIndex_String(void* instance, char* inputName) {
		try {
			int ret = reinterpret_cast<cv::dnn::LSTMLayer*>(instance)->inputNameToIndex(std::string(inputName));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_dnn_LSTMLayer_outputNameToIndex_const_StringX(void* instance, const char* outputName) {
		try {
			int ret = reinterpret_cast<cv::dnn::LSTMLayer*>(instance)->outputNameToIndex(std::string(outputName));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv_dnn_Layer_blobs(void* instance) {
		try {
			std::vector<cv::Mat> ret = reinterpret_cast<cv::dnn::Layer*>(instance)->blobs;
			return Ok<void*>(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_Layer_setBlobs_vector_Mat_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::dnn::Layer*>(instance)->blobs = *reinterpret_cast<std::vector<cv::Mat>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_Layer_name_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::dnn::Layer*>(instance)->name;
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_Layer_setName_String(void* instance, char* val) {
		try {
			reinterpret_cast<cv::dnn::Layer*>(instance)->name = std::string(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_Layer_type_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::dnn::Layer*>(instance)->type;
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_Layer_setType_String(void* instance, char* val) {
		try {
			reinterpret_cast<cv::dnn::Layer*>(instance)->type = std::string(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_Layer_preferableTarget_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::Layer*>(instance)->preferableTarget;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_Layer_setPreferableTarget_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::dnn::Layer*>(instance)->preferableTarget = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Layer_delete(cv::dnn::Layer* instance) {
		delete instance;
	}
	Result_void cv_dnn_Layer_finalize_const__InputArrayX_const__OutputArrayX(void* instance, void* inputs, void* outputs) {
		try {
			reinterpret_cast<cv::dnn::Layer*>(instance)->finalize(*reinterpret_cast<const cv::_InputArray*>(inputs), *reinterpret_cast<const cv::_OutputArray*>(outputs));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Layer_forward_vector_MatX_X_vector_Mat_X_vector_Mat_X(void* instance, void* input, void* output, void* internals) {
		try {
			reinterpret_cast<cv::dnn::Layer*>(instance)->forward(*reinterpret_cast<std::vector<cv::Mat*>*>(input), *reinterpret_cast<std::vector<cv::Mat>*>(output), *reinterpret_cast<std::vector<cv::Mat>*>(internals));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Layer_forward_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(void* instance, void* inputs, void* outputs, void* internals) {
		try {
			reinterpret_cast<cv::dnn::Layer*>(instance)->forward(*reinterpret_cast<const cv::_InputArray*>(inputs), *reinterpret_cast<const cv::_OutputArray*>(outputs), *reinterpret_cast<const cv::_OutputArray*>(internals));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Layer_forward_fallback_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(void* instance, void* inputs, void* outputs, void* internals) {
		try {
			reinterpret_cast<cv::dnn::Layer*>(instance)->forward_fallback(*reinterpret_cast<const cv::_InputArray*>(inputs), *reinterpret_cast<const cv::_OutputArray*>(outputs), *reinterpret_cast<const cv::_OutputArray*>(internals));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Layer_finalize_const_vector_Mat_X_vector_Mat_X(void* instance, void* inputs, void* outputs) {
		try {
			reinterpret_cast<cv::dnn::Layer*>(instance)->finalize(*reinterpret_cast<const std::vector<cv::Mat>*>(inputs), *reinterpret_cast<std::vector<cv::Mat>*>(outputs));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_Layer_finalize_const_vector_Mat_X(void* instance, void* inputs) {
		try {
			std::vector<cv::Mat> ret = reinterpret_cast<cv::dnn::Layer*>(instance)->finalize(*reinterpret_cast<const std::vector<cv::Mat>*>(inputs));
			return Ok<void*>(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_Layer_run_const_vector_Mat_X_vector_Mat_X_vector_Mat_X(void* instance, void* inputs, void* outputs, void* internals) {
		try {
			reinterpret_cast<cv::dnn::Layer*>(instance)->run(*reinterpret_cast<const std::vector<cv::Mat>*>(inputs), *reinterpret_cast<std::vector<cv::Mat>*>(outputs), *reinterpret_cast<std::vector<cv::Mat>*>(internals));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_Layer_inputNameToIndex_String(void* instance, char* inputName) {
		try {
			int ret = reinterpret_cast<cv::dnn::Layer*>(instance)->inputNameToIndex(std::string(inputName));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_dnn_Layer_outputNameToIndex_const_StringX(void* instance, const char* outputName) {
		try {
			int ret = reinterpret_cast<cv::dnn::Layer*>(instance)->outputNameToIndex(std::string(outputName));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<bool> cv_dnn_Layer_supportBackend_int(void* instance, int backendId) {
		try {
			bool ret = reinterpret_cast<cv::dnn::Layer*>(instance)->supportBackend(backendId);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_dnn_Layer_initHalide_const_vector_Ptr_BackendWrapper__X(void* instance, void* inputs) {
		try {
			cv::Ptr<cv::dnn::BackendNode> ret = reinterpret_cast<cv::dnn::Layer*>(instance)->initHalide(*reinterpret_cast<const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>*>(inputs));
			return Ok<void*>(new cv::Ptr<cv::dnn::BackendNode>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Layer_initInfEngine_const_vector_Ptr_BackendWrapper__X(void* instance, void* inputs) {
		try {
			cv::Ptr<cv::dnn::BackendNode> ret = reinterpret_cast<cv::dnn::Layer*>(instance)->initInfEngine(*reinterpret_cast<const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>*>(inputs));
			return Ok<void*>(new cv::Ptr<cv::dnn::BackendNode>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Layer_initNgraph_const_vector_Ptr_BackendWrapper__X_const_vector_Ptr_BackendNode__X(void* instance, void* inputs, void* nodes) {
		try {
			cv::Ptr<cv::dnn::BackendNode> ret = reinterpret_cast<cv::dnn::Layer*>(instance)->initNgraph(*reinterpret_cast<const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>*>(inputs), *reinterpret_cast<const std::vector<cv::Ptr<cv::dnn::BackendNode>>*>(nodes));
			return Ok<void*>(new cv::Ptr<cv::dnn::BackendNode>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Layer_initVkCom_const_vector_Ptr_BackendWrapper__X(void* instance, void* inputs) {
		try {
			cv::Ptr<cv::dnn::BackendNode> ret = reinterpret_cast<cv::dnn::Layer*>(instance)->initVkCom(*reinterpret_cast<const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>*>(inputs));
			return Ok<void*>(new cv::Ptr<cv::dnn::BackendNode>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Layer_initCUDA_voidX_const_vector_Ptr_BackendWrapper__X_const_vector_Ptr_BackendWrapper__X(void* instance, void* context, void* inputs, void* outputs) {
		try {
			cv::Ptr<cv::dnn::BackendNode> ret = reinterpret_cast<cv::dnn::Layer*>(instance)->initCUDA(context, *reinterpret_cast<const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>*>(inputs), *reinterpret_cast<const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>*>(outputs));
			return Ok<void*>(new cv::Ptr<cv::dnn::BackendNode>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_Layer_applyHalideScheduler_const_Ptr_BackendNode_X_const_vector_MatX_X_const_vector_Mat_X_int(void* instance, void* node, void* inputs, void* outputs, int targetId) {
		try {
			reinterpret_cast<cv::dnn::Layer*>(instance)->applyHalideScheduler(*reinterpret_cast<cv::Ptr<cv::dnn::BackendNode>*>(node), *reinterpret_cast<const std::vector<cv::Mat*>*>(inputs), *reinterpret_cast<const std::vector<cv::Mat>*>(outputs), targetId);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_Layer_tryAttach_const_Ptr_BackendNode_X(void* instance, void* node) {
		try {
			cv::Ptr<cv::dnn::BackendNode> ret = reinterpret_cast<cv::dnn::Layer*>(instance)->tryAttach(*reinterpret_cast<const cv::Ptr<cv::dnn::BackendNode>*>(node));
			return Ok<void*>(new cv::Ptr<cv::dnn::BackendNode>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_dnn_Layer_setActivation_const_Ptr_ActivationLayer_X(void* instance, void* layer) {
		try {
			bool ret = reinterpret_cast<cv::dnn::Layer*>(instance)->setActivation(*reinterpret_cast<const cv::Ptr<cv::dnn::ActivationLayer>*>(layer));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_dnn_Layer_tryFuse_Ptr_Layer_X(void* instance, void* top) {
		try {
			bool ret = reinterpret_cast<cv::dnn::Layer*>(instance)->tryFuse(*reinterpret_cast<cv::Ptr<cv::dnn::Layer>*>(top));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_Layer_getScaleShift_const_MatX_MatX(void* instance, void* scale, void* shift) {
		try {
			reinterpret_cast<cv::dnn::Layer*>(instance)->getScaleShift(*reinterpret_cast<cv::Mat*>(scale), *reinterpret_cast<cv::Mat*>(shift));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Layer_unsetAttached(void* instance) {
		try {
			reinterpret_cast<cv::dnn::Layer*>(instance)->unsetAttached();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_dnn_Layer_getMemoryShapes_const_const_vector_MatShape_X_int_vector_MatShape_X_vector_MatShape_X(void* instance, void* inputs, int requiredOutputs, void* outputs, void* internals) {
		try {
			bool ret = reinterpret_cast<cv::dnn::Layer*>(instance)->getMemoryShapes(*reinterpret_cast<const std::vector<cv::dnn::MatShape>*>(inputs), requiredOutputs, *reinterpret_cast<std::vector<cv::dnn::MatShape>*>(outputs), *reinterpret_cast<std::vector<cv::dnn::MatShape>*>(internals));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<int64_t> cv_dnn_Layer_getFLOPS_const_const_vector_MatShape_X_const_vector_MatShape_X(void* instance, void* inputs, void* outputs) {
		try {
			int64_t ret = reinterpret_cast<cv::dnn::Layer*>(instance)->getFLOPS(*reinterpret_cast<const std::vector<cv::dnn::MatShape>*>(inputs), *reinterpret_cast<const std::vector<cv::dnn::MatShape>*>(outputs));
			return Ok<int64_t>(ret);
		} OCVRS_CATCH(Result<int64_t>)
	}
	
	Result<void*> cv_dnn_Layer_Layer() {
		try {
			cv::dnn::Layer* ret = new cv::dnn::Layer();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Layer_Layer_const_LayerParamsX(void* params) {
		try {
			cv::dnn::Layer* ret = new cv::dnn::Layer(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_Layer_setParamsFrom_const_LayerParamsX(void* instance, void* params) {
		try {
			reinterpret_cast<cv::dnn::Layer*>(instance)->setParamsFrom(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_LayerFactory_delete(cv::dnn::LayerFactory* instance) {
		delete instance;
	}
	Result_void cv_dnn_LayerFactory_registerLayer_const_StringX_Constructor(const char* type, cv::dnn::LayerFactory::Constructor constructor) {
		try {
			cv::dnn::LayerFactory::registerLayer(std::string(type), constructor);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_LayerFactory_unregisterLayer_const_StringX(const char* type) {
		try {
			cv::dnn::LayerFactory::unregisterLayer(std::string(type));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_LayerFactory_createLayerInstance_const_StringX_LayerParamsX(const char* type, void* params) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::LayerFactory::createLayerInstance(std::string(type), *reinterpret_cast<cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_LayerParams_blobs(void* instance) {
		try {
			std::vector<cv::Mat> ret = reinterpret_cast<cv::dnn::LayerParams*>(instance)->blobs;
			return Ok<void*>(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_LayerParams_setBlobs_vector_Mat_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::dnn::LayerParams*>(instance)->blobs = *reinterpret_cast<std::vector<cv::Mat>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_LayerParams_name_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::dnn::LayerParams*>(instance)->name;
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_LayerParams_setName_String(void* instance, char* val) {
		try {
			reinterpret_cast<cv::dnn::LayerParams*>(instance)->name = std::string(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_LayerParams_type_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::dnn::LayerParams*>(instance)->type;
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_LayerParams_setType_String(void* instance, char* val) {
		try {
			reinterpret_cast<cv::dnn::LayerParams*>(instance)->type = std::string(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_LayerParams_delete(cv::dnn::LayerParams* instance) {
		delete instance;
	}
	Result<float> cv_dnn_MVNLayer_eps_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::dnn::MVNLayer*>(instance)->eps;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_dnn_MVNLayer_setEps_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::dnn::MVNLayer*>(instance)->eps = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_dnn_MVNLayer_normVariance_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::dnn::MVNLayer*>(instance)->normVariance;
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_MVNLayer_setNormVariance_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::dnn::MVNLayer*>(instance)->normVariance = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_dnn_MVNLayer_acrossChannels_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::dnn::MVNLayer*>(instance)->acrossChannels;
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_MVNLayer_setAcrossChannels_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::dnn::MVNLayer*>(instance)->acrossChannels = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_MVNLayer_delete(cv::dnn::MVNLayer* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_MVNLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::MVNLayer> ret = cv::dnn::MVNLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::MVNLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::Size> cv_dnn_MaxUnpoolLayer_poolKernel_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::dnn::MaxUnpoolLayer*>(instance)->poolKernel;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_dnn_MaxUnpoolLayer_setPoolKernel_Size(void* instance, cv::Size val) {
		try {
			reinterpret_cast<cv::dnn::MaxUnpoolLayer*>(instance)->poolKernel = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_dnn_MaxUnpoolLayer_poolPad_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::dnn::MaxUnpoolLayer*>(instance)->poolPad;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_dnn_MaxUnpoolLayer_setPoolPad_Size(void* instance, cv::Size val) {
		try {
			reinterpret_cast<cv::dnn::MaxUnpoolLayer*>(instance)->poolPad = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_dnn_MaxUnpoolLayer_poolStride_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::dnn::MaxUnpoolLayer*>(instance)->poolStride;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_dnn_MaxUnpoolLayer_setPoolStride_Size(void* instance, cv::Size val) {
		try {
			reinterpret_cast<cv::dnn::MaxUnpoolLayer*>(instance)->poolStride = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_MaxUnpoolLayer_delete(cv::dnn::MaxUnpoolLayer* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_MaxUnpoolLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::MaxUnpoolLayer> ret = cv::dnn::MaxUnpoolLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::MaxUnpoolLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_MishLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::MishLayer> ret = cv::dnn::MishLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::MishLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Model_delete(cv::dnn::Model* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_Model_Model() {
		try {
			cv::dnn::Model* ret = new cv::dnn::Model();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Model_Model_const_StringX_const_StringX(const char* model, const char* config) {
		try {
			cv::dnn::Model* ret = new cv::dnn::Model(std::string(model), std::string(config));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Model_Model_const_NetX(void* network) {
		try {
			cv::dnn::Model* ret = new cv::dnn::Model(*reinterpret_cast<const cv::dnn::Net*>(network));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Model_setInputSize_const_SizeX(void* instance, const cv::Size* size) {
		try {
			cv::dnn::Model ret = reinterpret_cast<cv::dnn::Model*>(instance)->setInputSize(*size);
			return Ok<void*>(new cv::dnn::Model(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Model_setInputSize_int_int(void* instance, int width, int height) {
		try {
			cv::dnn::Model ret = reinterpret_cast<cv::dnn::Model*>(instance)->setInputSize(width, height);
			return Ok<void*>(new cv::dnn::Model(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Model_setInputMean_const_ScalarX(void* instance, const cv::Scalar* mean) {
		try {
			cv::dnn::Model ret = reinterpret_cast<cv::dnn::Model*>(instance)->setInputMean(*mean);
			return Ok<void*>(new cv::dnn::Model(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Model_setInputScale_double(void* instance, double scale) {
		try {
			cv::dnn::Model ret = reinterpret_cast<cv::dnn::Model*>(instance)->setInputScale(scale);
			return Ok<void*>(new cv::dnn::Model(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Model_setInputCrop_bool(void* instance, bool crop) {
		try {
			cv::dnn::Model ret = reinterpret_cast<cv::dnn::Model*>(instance)->setInputCrop(crop);
			return Ok<void*>(new cv::dnn::Model(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Model_setInputSwapRB_bool(void* instance, bool swapRB) {
		try {
			cv::dnn::Model ret = reinterpret_cast<cv::dnn::Model*>(instance)->setInputSwapRB(swapRB);
			return Ok<void*>(new cv::dnn::Model(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_Model_setInputParams_double_const_SizeX_const_ScalarX_bool_bool(void* instance, double scale, const cv::Size* size, const cv::Scalar* mean, bool swapRB, bool crop) {
		try {
			reinterpret_cast<cv::dnn::Model*>(instance)->setInputParams(scale, *size, *mean, swapRB, crop);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Model_predict_const__InputArrayX_const__OutputArrayX(void* instance, void* frame, void* outs) {
		try {
			reinterpret_cast<cv::dnn::Model*>(instance)->predict(*reinterpret_cast<const cv::_InputArray*>(frame), *reinterpret_cast<const cv::_OutputArray*>(outs));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Net_delete(cv::dnn::Net* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_Net_Net() {
		try {
			cv::dnn::Net* ret = new cv::dnn::Net();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Net_readFromModelOptimizer_const_StringX_const_StringX(const char* xml, const char* bin) {
		try {
			cv::dnn::Net ret = cv::dnn::Net::readFromModelOptimizer(std::string(xml), std::string(bin));
			return Ok<void*>(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Net_readFromModelOptimizer_const_vector_unsigned_char_X_const_vector_unsigned_char_X(void* bufferModelConfig, void* bufferWeights) {
		try {
			cv::dnn::Net ret = cv::dnn::Net::readFromModelOptimizer(*reinterpret_cast<const std::vector<unsigned char>*>(bufferModelConfig), *reinterpret_cast<const std::vector<unsigned char>*>(bufferWeights));
			return Ok<void*>(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Net_readFromModelOptimizer_const_unsigned_charX_size_t_const_unsigned_charX_size_t(const unsigned char* bufferModelConfigPtr, size_t bufferModelConfigSize, const unsigned char* bufferWeightsPtr, size_t bufferWeightsSize) {
		try {
			cv::dnn::Net ret = cv::dnn::Net::readFromModelOptimizer(bufferModelConfigPtr, bufferModelConfigSize, bufferWeightsPtr, bufferWeightsSize);
			return Ok<void*>(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_dnn_Net_empty_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::dnn::Net*>(instance)->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_dnn_Net_dump(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::dnn::Net*>(instance)->dump();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_Net_dumpToFile_const_StringX(void* instance, const char* path) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->dumpToFile(std::string(path));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_Net_addLayer_const_StringX_const_StringX_LayerParamsX(void* instance, const char* name, const char* type, void* params) {
		try {
			int ret = reinterpret_cast<cv::dnn::Net*>(instance)->addLayer(std::string(name), std::string(type), *reinterpret_cast<cv::dnn::LayerParams*>(params));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_dnn_Net_addLayerToPrev_const_StringX_const_StringX_LayerParamsX(void* instance, const char* name, const char* type, void* params) {
		try {
			int ret = reinterpret_cast<cv::dnn::Net*>(instance)->addLayerToPrev(std::string(name), std::string(type), *reinterpret_cast<cv::dnn::LayerParams*>(params));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_dnn_Net_getLayerId_const_StringX(void* instance, const char* layer) {
		try {
			int ret = reinterpret_cast<cv::dnn::Net*>(instance)->getLayerId(std::string(layer));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv_dnn_Net_getLayerNames_const(void* instance) {
		try {
			std::vector<cv::String> ret = reinterpret_cast<cv::dnn::Net*>(instance)->getLayerNames();
			return Ok<void*>(new std::vector<cv::String>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Net_getLayer_LayerId(void* instance, void* layerId) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = reinterpret_cast<cv::dnn::Net*>(instance)->getLayer(*reinterpret_cast<cv::dnn::Net::LayerId*>(layerId));
			return Ok<void*>(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Net_getLayerInputs_LayerId(void* instance, void* layerId) {
		try {
			std::vector<cv::Ptr<cv::dnn::Layer>> ret = reinterpret_cast<cv::dnn::Net*>(instance)->getLayerInputs(*reinterpret_cast<cv::dnn::Net::LayerId*>(layerId));
			return Ok<void*>(new std::vector<cv::Ptr<cv::dnn::Layer>>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_Net_connect_String_String(void* instance, char* outPin, char* inpPin) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->connect(std::string(outPin), std::string(inpPin));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_connect_int_int_int_int(void* instance, int outLayerId, int outNum, int inpLayerId, int inpNum) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->connect(outLayerId, outNum, inpLayerId, inpNum);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_setInputsNames_const_vector_String_X(void* instance, void* inputBlobNames) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->setInputsNames(*reinterpret_cast<const std::vector<cv::String>*>(inputBlobNames));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_Net_forward_const_StringX(void* instance, const char* outputName) {
		try {
			cv::Mat ret = reinterpret_cast<cv::dnn::Net*>(instance)->forward(std::string(outputName));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Net_forwardAsync_const_StringX(void* instance, const char* outputName) {
		try {
			cv::AsyncArray ret = reinterpret_cast<cv::dnn::Net*>(instance)->forwardAsync(std::string(outputName));
			return Ok<void*>(new cv::AsyncArray(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_Net_forward_const__OutputArrayX_const_StringX(void* instance, void* outputBlobs, const char* outputName) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->forward(*reinterpret_cast<const cv::_OutputArray*>(outputBlobs), std::string(outputName));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_forward_const__OutputArrayX_const_vector_String_X(void* instance, void* outputBlobs, void* outBlobNames) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->forward(*reinterpret_cast<const cv::_OutputArray*>(outputBlobs), *reinterpret_cast<const std::vector<cv::String>*>(outBlobNames));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_forward_vector_vector_Mat__X_const_vector_String_X(void* instance, void* outputBlobs, void* outBlobNames) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->forward(*reinterpret_cast<std::vector<std::vector<cv::Mat>>*>(outputBlobs), *reinterpret_cast<const std::vector<cv::String>*>(outBlobNames));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_setHalideScheduler_const_StringX(void* instance, const char* scheduler) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->setHalideScheduler(std::string(scheduler));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_setPreferableBackend_int(void* instance, int backendId) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->setPreferableBackend(backendId);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_setPreferableTarget_int(void* instance, int targetId) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->setPreferableTarget(targetId);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_setInput_const__InputArrayX_const_StringX_double_const_ScalarX(void* instance, void* blob, const char* name, double scalefactor, const cv::Scalar* mean) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->setInput(*reinterpret_cast<const cv::_InputArray*>(blob), std::string(name), scalefactor, *mean);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_setParam_LayerId_int_const_MatX(void* instance, void* layer, int numParam, void* blob) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->setParam(*reinterpret_cast<cv::dnn::Net::LayerId*>(layer), numParam, *reinterpret_cast<const cv::Mat*>(blob));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_Net_getParam_LayerId_int(void* instance, void* layer, int numParam) {
		try {
			cv::Mat ret = reinterpret_cast<cv::dnn::Net*>(instance)->getParam(*reinterpret_cast<cv::dnn::Net::LayerId*>(layer), numParam);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Net_getUnconnectedOutLayers_const(void* instance) {
		try {
			std::vector<int> ret = reinterpret_cast<cv::dnn::Net*>(instance)->getUnconnectedOutLayers();
			return Ok<void*>(new std::vector<int>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_Net_getUnconnectedOutLayersNames_const(void* instance) {
		try {
			std::vector<cv::String> ret = reinterpret_cast<cv::dnn::Net*>(instance)->getUnconnectedOutLayersNames();
			return Ok<void*>(new std::vector<cv::String>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_Net_getLayersShapes_const_const_vector_MatShape_X_vector_int_X_vector_vector_MatShape__X_vector_vector_MatShape__X(void* instance, void* netInputShapes, void* layersIds, void* inLayersShapes, void* outLayersShapes) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->getLayersShapes(*reinterpret_cast<const std::vector<cv::dnn::MatShape>*>(netInputShapes), *reinterpret_cast<std::vector<int>*>(layersIds), *reinterpret_cast<std::vector<std::vector<cv::dnn::MatShape>>*>(inLayersShapes), *reinterpret_cast<std::vector<std::vector<cv::dnn::MatShape>>*>(outLayersShapes));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_getLayersShapes_const_const_MatShapeX_vector_int_X_vector_vector_MatShape__X_vector_vector_MatShape__X(void* instance, void* netInputShape, void* layersIds, void* inLayersShapes, void* outLayersShapes) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->getLayersShapes(*reinterpret_cast<const cv::dnn::MatShape*>(netInputShape), *reinterpret_cast<std::vector<int>*>(layersIds), *reinterpret_cast<std::vector<std::vector<cv::dnn::MatShape>>*>(inLayersShapes), *reinterpret_cast<std::vector<std::vector<cv::dnn::MatShape>>*>(outLayersShapes));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_getLayerShapes_const_const_MatShapeX_int_vector_MatShape_X_vector_MatShape_X(void* instance, void* netInputShape, int layerId, void* inLayerShapes, void* outLayerShapes) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->getLayerShapes(*reinterpret_cast<const cv::dnn::MatShape*>(netInputShape), layerId, *reinterpret_cast<std::vector<cv::dnn::MatShape>*>(inLayerShapes), *reinterpret_cast<std::vector<cv::dnn::MatShape>*>(outLayerShapes));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_getLayerShapes_const_const_vector_MatShape_X_int_vector_MatShape_X_vector_MatShape_X(void* instance, void* netInputShapes, int layerId, void* inLayerShapes, void* outLayerShapes) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->getLayerShapes(*reinterpret_cast<const std::vector<cv::dnn::MatShape>*>(netInputShapes), layerId, *reinterpret_cast<std::vector<cv::dnn::MatShape>*>(inLayerShapes), *reinterpret_cast<std::vector<cv::dnn::MatShape>*>(outLayerShapes));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int64_t> cv_dnn_Net_getFLOPS_const_const_vector_MatShape_X(void* instance, void* netInputShapes) {
		try {
			int64_t ret = reinterpret_cast<cv::dnn::Net*>(instance)->getFLOPS(*reinterpret_cast<const std::vector<cv::dnn::MatShape>*>(netInputShapes));
			return Ok<int64_t>(ret);
		} OCVRS_CATCH(Result<int64_t>)
	}
	
	Result<int64_t> cv_dnn_Net_getFLOPS_const_const_MatShapeX(void* instance, void* netInputShape) {
		try {
			int64_t ret = reinterpret_cast<cv::dnn::Net*>(instance)->getFLOPS(*reinterpret_cast<const cv::dnn::MatShape*>(netInputShape));
			return Ok<int64_t>(ret);
		} OCVRS_CATCH(Result<int64_t>)
	}
	
	Result<int64_t> cv_dnn_Net_getFLOPS_const_int_const_vector_MatShape_X(void* instance, int layerId, void* netInputShapes) {
		try {
			int64_t ret = reinterpret_cast<cv::dnn::Net*>(instance)->getFLOPS(layerId, *reinterpret_cast<const std::vector<cv::dnn::MatShape>*>(netInputShapes));
			return Ok<int64_t>(ret);
		} OCVRS_CATCH(Result<int64_t>)
	}
	
	Result<int64_t> cv_dnn_Net_getFLOPS_const_int_const_MatShapeX(void* instance, int layerId, void* netInputShape) {
		try {
			int64_t ret = reinterpret_cast<cv::dnn::Net*>(instance)->getFLOPS(layerId, *reinterpret_cast<const cv::dnn::MatShape*>(netInputShape));
			return Ok<int64_t>(ret);
		} OCVRS_CATCH(Result<int64_t>)
	}
	
	Result_void cv_dnn_Net_getLayerTypes_const_vector_String_X(void* instance, void* layersTypes) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->getLayerTypes(*reinterpret_cast<std::vector<cv::String>*>(layersTypes));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_Net_getLayersCount_const_const_StringX(void* instance, const char* layerType) {
		try {
			int ret = reinterpret_cast<cv::dnn::Net*>(instance)->getLayersCount(std::string(layerType));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_Net_getMemoryConsumption_const_const_vector_MatShape_X_size_tX_size_tX(void* instance, void* netInputShapes, size_t* weights, size_t* blobs) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->getMemoryConsumption(*reinterpret_cast<const std::vector<cv::dnn::MatShape>*>(netInputShapes), *weights, *blobs);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_getMemoryConsumption_const_const_MatShapeX_size_tX_size_tX(void* instance, void* netInputShape, size_t* weights, size_t* blobs) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->getMemoryConsumption(*reinterpret_cast<const cv::dnn::MatShape*>(netInputShape), *weights, *blobs);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_getMemoryConsumption_const_int_const_vector_MatShape_X_size_tX_size_tX(void* instance, int layerId, void* netInputShapes, size_t* weights, size_t* blobs) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->getMemoryConsumption(layerId, *reinterpret_cast<const std::vector<cv::dnn::MatShape>*>(netInputShapes), *weights, *blobs);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_getMemoryConsumption_const_int_const_MatShapeX_size_tX_size_tX(void* instance, int layerId, void* netInputShape, size_t* weights, size_t* blobs) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->getMemoryConsumption(layerId, *reinterpret_cast<const cv::dnn::MatShape*>(netInputShape), *weights, *blobs);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_getMemoryConsumption_const_const_vector_MatShape_X_vector_int_X_vector_size_t_X_vector_size_t_X(void* instance, void* netInputShapes, void* layerIds, void* weights, void* blobs) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->getMemoryConsumption(*reinterpret_cast<const std::vector<cv::dnn::MatShape>*>(netInputShapes), *reinterpret_cast<std::vector<int>*>(layerIds), *reinterpret_cast<std::vector<size_t>*>(weights), *reinterpret_cast<std::vector<size_t>*>(blobs));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_getMemoryConsumption_const_const_MatShapeX_vector_int_X_vector_size_t_X_vector_size_t_X(void* instance, void* netInputShape, void* layerIds, void* weights, void* blobs) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->getMemoryConsumption(*reinterpret_cast<const cv::dnn::MatShape*>(netInputShape), *reinterpret_cast<std::vector<int>*>(layerIds), *reinterpret_cast<std::vector<size_t>*>(weights), *reinterpret_cast<std::vector<size_t>*>(blobs));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_enableFusion_bool(void* instance, bool fusion) {
		try {
			reinterpret_cast<cv::dnn::Net*>(instance)->enableFusion(fusion);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int64_t> cv_dnn_Net_getPerfProfile_vector_double_X(void* instance, void* timings) {
		try {
			int64_t ret = reinterpret_cast<cv::dnn::Net*>(instance)->getPerfProfile(*reinterpret_cast<std::vector<double>*>(timings));
			return Ok<int64_t>(ret);
		} OCVRS_CATCH(Result<int64_t>)
	}
	
	Result<float> cv_dnn_NormalizeBBoxLayer_pnorm_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::dnn::NormalizeBBoxLayer*>(instance)->pnorm;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_dnn_NormalizeBBoxLayer_setPnorm_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::dnn::NormalizeBBoxLayer*>(instance)->pnorm = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_dnn_NormalizeBBoxLayer_epsilon_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::dnn::NormalizeBBoxLayer*>(instance)->epsilon;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_dnn_NormalizeBBoxLayer_setEpsilon_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::dnn::NormalizeBBoxLayer*>(instance)->epsilon = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_dnn_NormalizeBBoxLayer_acrossSpatial_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::dnn::NormalizeBBoxLayer*>(instance)->acrossSpatial;
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_NormalizeBBoxLayer_setAcrossSpatial_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::dnn::NormalizeBBoxLayer*>(instance)->acrossSpatial = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_NormalizeBBoxLayer_delete(cv::dnn::NormalizeBBoxLayer* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_NormalizeBBoxLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::NormalizeBBoxLayer> ret = cv::dnn::NormalizeBBoxLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::NormalizeBBoxLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_PaddingLayer_delete(cv::dnn::PaddingLayer* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_PaddingLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::PaddingLayer> ret = cv::dnn::PaddingLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::PaddingLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_PermuteLayer_delete(cv::dnn::PermuteLayer* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_PermuteLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::PermuteLayer> ret = cv::dnn::PermuteLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::PermuteLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_dnn_PoolingLayer_type_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->type;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_PoolingLayer_setType_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->type = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_PoolingLayer_kernel_size(void* instance) {
		try {
			std::vector<size_t> ret = reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->kernel_size;
			return Ok<void*>(new std::vector<size_t>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_PoolingLayer_setKernel_size_vector_size_t_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->kernel_size = *reinterpret_cast<std::vector<size_t>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_PoolingLayer_strides(void* instance) {
		try {
			std::vector<size_t> ret = reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->strides;
			return Ok<void*>(new std::vector<size_t>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_PoolingLayer_setStrides_vector_size_t_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->strides = *reinterpret_cast<std::vector<size_t>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_PoolingLayer_pads_begin(void* instance) {
		try {
			std::vector<size_t> ret = reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->pads_begin;
			return Ok<void*>(new std::vector<size_t>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_PoolingLayer_setPads_begin_vector_size_t_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->pads_begin = *reinterpret_cast<std::vector<size_t>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_PoolingLayer_pads_end(void* instance) {
		try {
			std::vector<size_t> ret = reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->pads_end;
			return Ok<void*>(new std::vector<size_t>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_PoolingLayer_setPads_end_vector_size_t_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->pads_end = *reinterpret_cast<std::vector<size_t>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_dnn_PoolingLayer_kernel_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->kernel;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_dnn_PoolingLayer_setKernel_Size(void* instance, cv::Size val) {
		try {
			reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->kernel = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_dnn_PoolingLayer_stride_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->stride;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_dnn_PoolingLayer_setStride_Size(void* instance, cv::Size val) {
		try {
			reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->stride = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_dnn_PoolingLayer_pad_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->pad;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_dnn_PoolingLayer_setPad_Size(void* instance, cv::Size val) {
		try {
			reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->pad = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_PoolingLayer_pad_l_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->pad_l;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_PoolingLayer_setPad_l_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->pad_l = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_PoolingLayer_pad_t_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->pad_t;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_PoolingLayer_setPad_t_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->pad_t = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_PoolingLayer_pad_r_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->pad_r;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_PoolingLayer_setPad_r_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->pad_r = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_PoolingLayer_pad_b_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->pad_b;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_PoolingLayer_setPad_b_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->pad_b = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_dnn_PoolingLayer_globalPooling_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->globalPooling;
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_PoolingLayer_setGlobalPooling_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->globalPooling = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_dnn_PoolingLayer_computeMaxIdx_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->computeMaxIdx;
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_PoolingLayer_setComputeMaxIdx_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->computeMaxIdx = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_PoolingLayer_padMode_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->padMode;
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_PoolingLayer_setPadMode_String(void* instance, char* val) {
		try {
			reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->padMode = std::string(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_dnn_PoolingLayer_ceilMode_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->ceilMode;
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_PoolingLayer_setCeilMode_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->ceilMode = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_dnn_PoolingLayer_avePoolPaddedArea_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->avePoolPaddedArea;
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_PoolingLayer_setAvePoolPaddedArea_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->avePoolPaddedArea = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_dnn_PoolingLayer_pooledSize_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->pooledSize;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_dnn_PoolingLayer_setPooledSize_Size(void* instance, cv::Size val) {
		try {
			reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->pooledSize = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_dnn_PoolingLayer_spatialScale_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->spatialScale;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_dnn_PoolingLayer_setSpatialScale_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->spatialScale = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_PoolingLayer_psRoiOutChannels_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->psRoiOutChannels;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_PoolingLayer_setPsRoiOutChannels_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::dnn::PoolingLayer*>(instance)->psRoiOutChannels = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_PoolingLayer_delete(cv::dnn::PoolingLayer* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_PoolingLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::PoolingLayer> ret = cv::dnn::PoolingLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::PoolingLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<float> cv_dnn_PowerLayer_power_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::dnn::PowerLayer*>(instance)->power;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_dnn_PowerLayer_setPower_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::dnn::PowerLayer*>(instance)->power = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_dnn_PowerLayer_scale_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::dnn::PowerLayer*>(instance)->scale;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_dnn_PowerLayer_setScale_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::dnn::PowerLayer*>(instance)->scale = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_dnn_PowerLayer_shift_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::dnn::PowerLayer*>(instance)->shift;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_dnn_PowerLayer_setShift_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::dnn::PowerLayer*>(instance)->shift = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_PowerLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::PowerLayer> ret = cv::dnn::PowerLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::PowerLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_PriorBoxLayer_delete(cv::dnn::PriorBoxLayer* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_PriorBoxLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::PriorBoxLayer> ret = cv::dnn::PriorBoxLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::PriorBoxLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_ProposalLayer_delete(cv::dnn::ProposalLayer* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_ProposalLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::ProposalLayer> ret = cv::dnn::ProposalLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::ProposalLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_RNNLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::RNNLayer> ret = cv::dnn::RNNLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::RNNLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_RNNLayer_setWeights_const_MatX_const_MatX_const_MatX_const_MatX_const_MatX(void* instance, void* Wxh, void* bh, void* Whh, void* Who, void* bo) {
		try {
			reinterpret_cast<cv::dnn::RNNLayer*>(instance)->setWeights(*reinterpret_cast<const cv::Mat*>(Wxh), *reinterpret_cast<const cv::Mat*>(bh), *reinterpret_cast<const cv::Mat*>(Whh), *reinterpret_cast<const cv::Mat*>(Who), *reinterpret_cast<const cv::Mat*>(bo));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_RNNLayer_setProduceHiddenOutput_bool(void* instance, bool produce) {
		try {
			reinterpret_cast<cv::dnn::RNNLayer*>(instance)->setProduceHiddenOutput(produce);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_dnn_ReLU6Layer_minValue_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::dnn::ReLU6Layer*>(instance)->minValue;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_dnn_ReLU6Layer_setMinValue_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::dnn::ReLU6Layer*>(instance)->minValue = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_dnn_ReLU6Layer_maxValue_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::dnn::ReLU6Layer*>(instance)->maxValue;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_dnn_ReLU6Layer_setMaxValue_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::dnn::ReLU6Layer*>(instance)->maxValue = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_ReLU6Layer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::ReLU6Layer> ret = cv::dnn::ReLU6Layer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::ReLU6Layer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<float> cv_dnn_ReLULayer_negativeSlope_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::dnn::ReLULayer*>(instance)->negativeSlope;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_dnn_ReLULayer_setNegativeSlope_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::dnn::ReLULayer*>(instance)->negativeSlope = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_ReLULayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::ReLULayer> ret = cv::dnn::ReLULayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::ReLULayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_RegionLayer_delete(cv::dnn::RegionLayer* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_RegionLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::RegionLayer> ret = cv::dnn::RegionLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::RegionLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_ReorgLayer_delete(cv::dnn::ReorgLayer* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_ReorgLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::ReorgLayer> ret = cv::dnn::ReorgLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::ReorgLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_ReshapeLayer_newShapeDesc(void* instance) {
		try {
			cv::dnn::MatShape ret = reinterpret_cast<cv::dnn::ReshapeLayer*>(instance)->newShapeDesc;
			return Ok<void*>(new cv::dnn::MatShape(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_ReshapeLayer_setNewShapeDesc_MatShape(void* instance, void* val) {
		try {
			reinterpret_cast<cv::dnn::ReshapeLayer*>(instance)->newShapeDesc = *reinterpret_cast<cv::dnn::MatShape*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_ReshapeLayer_newShapeRange(void* instance) {
		try {
			cv::Range ret = reinterpret_cast<cv::dnn::ReshapeLayer*>(instance)->newShapeRange;
			return Ok<void*>(new cv::Range(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_ReshapeLayer_setNewShapeRange_Range(void* instance, void* val) {
		try {
			reinterpret_cast<cv::dnn::ReshapeLayer*>(instance)->newShapeRange = *reinterpret_cast<cv::Range*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_ReshapeLayer_delete(cv::dnn::ReshapeLayer* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_ReshapeLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::ReshapeLayer> ret = cv::dnn::ReshapeLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::ReshapeLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_ResizeLayer_delete(cv::dnn::ResizeLayer* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_ResizeLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::ResizeLayer> ret = cv::dnn::ResizeLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::ResizeLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_dnn_ScaleLayer_hasBias_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::dnn::ScaleLayer*>(instance)->hasBias;
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_ScaleLayer_setHasBias_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::dnn::ScaleLayer*>(instance)->hasBias = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_ScaleLayer_axis_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::ScaleLayer*>(instance)->axis;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_ScaleLayer_setAxis_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::dnn::ScaleLayer*>(instance)->axis = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_ScaleLayer_delete(cv::dnn::ScaleLayer* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_ScaleLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::ScaleLayer> ret = cv::dnn::ScaleLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::ScaleLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_SegmentationModel_delete(cv::dnn::SegmentationModel* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_SegmentationModel_SegmentationModel_const_StringX_const_StringX(const char* model, const char* config) {
		try {
			cv::dnn::SegmentationModel* ret = new cv::dnn::SegmentationModel(std::string(model), std::string(config));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_SegmentationModel_SegmentationModel_const_NetX(void* network) {
		try {
			cv::dnn::SegmentationModel* ret = new cv::dnn::SegmentationModel(*reinterpret_cast<const cv::dnn::Net*>(network));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_SegmentationModel_segment_const__InputArrayX_const__OutputArrayX(void* instance, void* frame, void* mask) {
		try {
			reinterpret_cast<cv::dnn::SegmentationModel*>(instance)->segment(*reinterpret_cast<const cv::_InputArray*>(frame), *reinterpret_cast<const cv::_OutputArray*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_ShiftLayer_delete(cv::dnn::ShiftLayer* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_ShiftLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::ShiftLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_dnn_ShuffleChannelLayer_group_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::ShuffleChannelLayer*>(instance)->group;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_ShuffleChannelLayer_setGroup_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::dnn::ShuffleChannelLayer*>(instance)->group = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_ShuffleChannelLayer_delete(cv::dnn::ShuffleChannelLayer* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_ShuffleChannelLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::ShuffleChannelLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_SigmoidLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::SigmoidLayer> ret = cv::dnn::SigmoidLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::SigmoidLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_SliceLayer_sliceRanges(void* instance) {
		try {
			std::vector<std::vector<cv::Range>> ret = reinterpret_cast<cv::dnn::SliceLayer*>(instance)->sliceRanges;
			return Ok<void*>(new std::vector<std::vector<cv::Range>>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_SliceLayer_setSliceRanges_vector_vector_Range__(void* instance, void* val) {
		try {
			reinterpret_cast<cv::dnn::SliceLayer*>(instance)->sliceRanges = *reinterpret_cast<std::vector<std::vector<cv::Range>>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_SliceLayer_axis_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::SliceLayer*>(instance)->axis;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_SliceLayer_setAxis_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::dnn::SliceLayer*>(instance)->axis = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_SliceLayer_num_split_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::SliceLayer*>(instance)->num_split;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_SliceLayer_setNum_split_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::dnn::SliceLayer*>(instance)->num_split = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_SliceLayer_delete(cv::dnn::SliceLayer* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_SliceLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::SliceLayer> ret = cv::dnn::SliceLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::SliceLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_dnn_SoftmaxLayer_logSoftMax_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::dnn::SoftmaxLayer*>(instance)->logSoftMax;
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_SoftmaxLayer_setLogSoftMax_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::dnn::SoftmaxLayer*>(instance)->logSoftMax = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_SoftmaxLayer_delete(cv::dnn::SoftmaxLayer* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_SoftmaxLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::SoftmaxLayer> ret = cv::dnn::SoftmaxLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::SoftmaxLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_dnn_SplitLayer_outputsCount_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn::SplitLayer*>(instance)->outputsCount;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_SplitLayer_setOutputsCount_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::dnn::SplitLayer*>(instance)->outputsCount = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_SplitLayer_delete(cv::dnn::SplitLayer* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_SplitLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::SplitLayer> ret = cv::dnn::SplitLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::SplitLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_SwishLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::SwishLayer> ret = cv::dnn::SwishLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::SwishLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_TanHLayer_create_const_LayerParamsX(void* params) {
		try {
			cv::Ptr<cv::dnn::TanHLayer> ret = cv::dnn::TanHLayer::create(*reinterpret_cast<const cv::dnn::LayerParams*>(params));
			return Ok<void*>(new cv::Ptr<cv::dnn::TanHLayer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv__Range_delete(cv::dnn::_Range* instance) {
		delete instance;
	}
	Result<void*> cv_dnn__Range__Range_const_RangeX(void* r) {
		try {
			cv::dnn::_Range* ret = new cv::dnn::_Range(*reinterpret_cast<const cv::Range*>(r));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn__Range__Range_int_int(int start_, int size_) {
		try {
			cv::dnn::_Range* ret = new cv::dnn::_Range(start_, size_);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
}
