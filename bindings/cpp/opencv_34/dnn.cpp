#include "dnn.hpp"
#include "dnn_types.hpp"

extern "C" {
	Result_void cv_dnn_NMSBoxes_const_vector_Rect2d_R_const_vector_float_R_const_float_const_float_vector_int_R_const_float_const_int(const std::vector<cv::Rect2d>* bboxes, const std::vector<float>* scores, const float score_threshold, const float nms_threshold, std::vector<int>* indices, const float eta, const int top_k) {
		try {
			cv::dnn::NMSBoxes(*bboxes, *scores, score_threshold, nms_threshold, *indices, eta, top_k);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_NMSBoxes_const_vector_Rect_R_const_vector_float_R_const_float_const_float_vector_int_R_const_float_const_int(const std::vector<cv::Rect>* bboxes, const std::vector<float>* scores, const float score_threshold, const float nms_threshold, std::vector<int>* indices, const float eta, const int top_k) {
		try {
			cv::dnn::NMSBoxes(*bboxes, *scores, score_threshold, nms_threshold, *indices, eta, top_k);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_NMSBoxes_const_vector_RotatedRect_R_const_vector_float_R_const_float_const_float_vector_int_R_const_float_const_int(const std::vector<cv::RotatedRect>* bboxes, const std::vector<float>* scores, const float score_threshold, const float nms_threshold, std::vector<int>* indices, const float eta, const int top_k) {
		try {
			cv::dnn::NMSBoxes(*bboxes, *scores, score_threshold, nms_threshold, *indices, eta, top_k);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_blobFromImage_const__InputArrayR_const__OutputArrayR_double_const_SizeR_const_ScalarR_bool_bool_int(const cv::_InputArray* image, const cv::_OutputArray* blob, double scalefactor, const cv::Size* size, const cv::Scalar* mean, bool swapRB, bool crop, int ddepth) {
		try {
			cv::dnn::blobFromImage(*image, *blob, scalefactor, *size, *mean, swapRB, crop, ddepth);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_dnn_blobFromImage_const__InputArrayR_double_const_SizeR_const_ScalarR_bool_bool_int(const cv::_InputArray* image, double scalefactor, const cv::Size* size, const cv::Scalar* mean, bool swapRB, bool crop, int ddepth) {
		try {
			cv::Mat ret = cv::dnn::blobFromImage(*image, scalefactor, *size, *mean, swapRB, crop, ddepth);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_dnn_blobFromImages_const__InputArrayR_const__OutputArrayR_double_Size_const_ScalarR_bool_bool_int(const cv::_InputArray* images, const cv::_OutputArray* blob, double scalefactor, cv::Size* size, const cv::Scalar* mean, bool swapRB, bool crop, int ddepth) {
		try {
			cv::dnn::blobFromImages(*images, *blob, scalefactor, *size, *mean, swapRB, crop, ddepth);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_dnn_blobFromImages_const__InputArrayR_double_Size_const_ScalarR_bool_bool_int(const cv::_InputArray* images, double scalefactor, cv::Size* size, const cv::Scalar* mean, bool swapRB, bool crop, int ddepth) {
		try {
			cv::Mat ret = cv::dnn::blobFromImages(*images, scalefactor, *size, *mean, swapRB, crop, ddepth);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Range*> cv_dnn_clamp_const_RangeR_int(const cv::Range* r, int axisSize) {
		try {
			cv::Range ret = cv::dnn::clamp(*r, axisSize);
			return Ok(new cv::Range(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Range*>))
	}
	
	Result<int> cv_dnn_clamp_int_const_MatShapeR(int ax, const cv::dnn::MatShape* shape) {
		try {
			int ret = cv::dnn::clamp(ax, *shape);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_dnn_clamp_int_int(int ax, int dims) {
		try {
			int ret = cv::dnn::clamp(ax, dims);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<cv::dnn::MatShape*> cv_dnn_concat_const_MatShapeR_const_MatShapeR(const cv::dnn::MatShape* a, const cv::dnn::MatShape* b) {
		try {
			cv::dnn::MatShape ret = cv::dnn::concat(*a, *b);
			return Ok(new cv::dnn::MatShape(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::MatShape*>))
	}
	
	Result<std::vector<cv::dnn::Target>*> cv_dnn_getAvailableTargets_Backend(cv::dnn::Backend be) {
		try {
			std::vector<cv::dnn::Target> ret = cv::dnn::getAvailableTargets(be);
			return Ok(new std::vector<cv::dnn::Target>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::dnn::Target>*>))
	}
	
	Result<void*> cv_dnn_getInferenceEngineBackendType() {
		try {
			cv::String ret = cv::dnn::getInferenceEngineBackendType();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<void*> cv_dnn_getInferenceEngineVPUType() {
		try {
			cv::String ret = cv::dnn::getInferenceEngineVPUType();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<cv::Mat*> cv_dnn_getPlane_const_MatR_int_int(const cv::Mat* m, int n, int cn) {
		try {
			cv::Mat ret = cv::dnn::getPlane(*m, n, cn);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_dnn_imagesFromBlob_const_MatR_const__OutputArrayR(const cv::Mat* blob_, const cv::_OutputArray* images_) {
		try {
			cv::dnn::imagesFromBlob(*blob_, *images_);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_print_const_MatShapeR_const_StringR(const cv::dnn::MatShape* shape, const char* name) {
		try {
			cv::dnn::print(*shape, cv::String(name));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromCaffe_const_StringR_const_StringR(const char* prototxt, const char* caffeModel) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromCaffe(cv::String(prototxt), cv::String(caffeModel));
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Net*>))
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromCaffe_const_charX_size_t_const_charX_size_t(const char* bufferProto, size_t lenProto, const char* bufferModel, size_t lenModel) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromCaffe(bufferProto, lenProto, bufferModel, lenModel);
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Net*>))
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromCaffe_const_vector_unsigned_char_R_const_vector_unsigned_char_R(const std::vector<unsigned char>* bufferProto, const std::vector<unsigned char>* bufferModel) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromCaffe(*bufferProto, *bufferModel);
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Net*>))
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromDarknet_const_StringR_const_StringR(const char* cfgFile, const char* darknetModel) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromDarknet(cv::String(cfgFile), cv::String(darknetModel));
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Net*>))
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromDarknet_const_charX_size_t_const_charX_size_t(const char* bufferCfg, size_t lenCfg, const char* bufferModel, size_t lenModel) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromDarknet(bufferCfg, lenCfg, bufferModel, lenModel);
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Net*>))
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromDarknet_const_vector_unsigned_char_R_const_vector_unsigned_char_R(const std::vector<unsigned char>* bufferCfg, const std::vector<unsigned char>* bufferModel) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromDarknet(*bufferCfg, *bufferModel);
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Net*>))
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromModelOptimizer_const_StringR_const_StringR(const char* xml, const char* bin) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromModelOptimizer(cv::String(xml), cv::String(bin));
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Net*>))
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromModelOptimizer_const_unsigned_charX_size_t_const_unsigned_charX_size_t(const unsigned char* bufferModelConfigPtr, size_t bufferModelConfigSize, const unsigned char* bufferWeightsPtr, size_t bufferWeightsSize) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromModelOptimizer(bufferModelConfigPtr, bufferModelConfigSize, bufferWeightsPtr, bufferWeightsSize);
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Net*>))
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromModelOptimizer_const_vector_unsigned_char_R_const_vector_unsigned_char_R(const std::vector<unsigned char>* bufferModelConfig, const std::vector<unsigned char>* bufferWeights) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromModelOptimizer(*bufferModelConfig, *bufferWeights);
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Net*>))
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromONNX_const_StringR(const char* onnxFile) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromONNX(cv::String(onnxFile));
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Net*>))
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromONNX_const_charX_size_t(const char* buffer, size_t sizeBuffer) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromONNX(buffer, sizeBuffer);
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Net*>))
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromONNX_const_vector_unsigned_char_R(const std::vector<unsigned char>* buffer) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromONNX(*buffer);
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Net*>))
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromTensorflow_const_StringR_const_StringR(const char* model, const char* config) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromTensorflow(cv::String(model), cv::String(config));
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Net*>))
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromTensorflow_const_charX_size_t_const_charX_size_t(const char* bufferModel, size_t lenModel, const char* bufferConfig, size_t lenConfig) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromTensorflow(bufferModel, lenModel, bufferConfig, lenConfig);
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Net*>))
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromTensorflow_const_vector_unsigned_char_R_const_vector_unsigned_char_R(const std::vector<unsigned char>* bufferModel, const std::vector<unsigned char>* bufferConfig) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromTensorflow(*bufferModel, *bufferConfig);
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Net*>))
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromTorch_const_StringR_bool_bool(const char* model, bool isBinary, bool evaluate) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromTorch(cv::String(model), isBinary, evaluate);
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Net*>))
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNet_const_StringR_const_StringR_const_StringR(const char* model, const char* config, const char* framework) {
		try {
			cv::dnn::Net ret = cv::dnn::readNet(cv::String(model), cv::String(config), cv::String(framework));
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Net*>))
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNet_const_StringR_const_vector_unsigned_char_R_const_vector_unsigned_char_R(const char* framework, const std::vector<unsigned char>* bufferModel, const std::vector<unsigned char>* bufferConfig) {
		try {
			cv::dnn::Net ret = cv::dnn::readNet(cv::String(framework), *bufferModel, *bufferConfig);
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Net*>))
	}
	
	Result<cv::Mat*> cv_dnn_readTensorFromONNX_const_StringR(const char* path) {
		try {
			cv::Mat ret = cv::dnn::readTensorFromONNX(cv::String(path));
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_dnn_readTorchBlob_const_StringR_bool(const char* filename, bool isBinary) {
		try {
			cv::Mat ret = cv::dnn::readTorchBlob(cv::String(filename), isBinary);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_dnn_resetMyriadDevice() {
		try {
			cv::dnn::resetMyriadDevice();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_dnn_setInferenceEngineBackendType_const_StringR(const char* newBackendType) {
		try {
			cv::String ret = cv::dnn::setInferenceEngineBackendType(cv::String(newBackendType));
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<cv::dnn::MatShape*> cv_dnn_shape_const_MatR(const cv::Mat* mat) {
		try {
			cv::dnn::MatShape ret = cv::dnn::shape(*mat);
			return Ok(new cv::dnn::MatShape(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::MatShape*>))
	}
	
	Result<cv::dnn::MatShape*> cv_dnn_shape_const_MatSizeR(const cv::MatSize* sz) {
		try {
			cv::dnn::MatShape ret = cv::dnn::shape(*sz);
			return Ok(new cv::dnn::MatShape(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::MatShape*>))
	}
	
	Result<cv::dnn::MatShape*> cv_dnn_shape_const_UMatR(const cv::UMat* mat) {
		try {
			cv::dnn::MatShape ret = cv::dnn::shape(*mat);
			return Ok(new cv::dnn::MatShape(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::MatShape*>))
	}
	
	Result<cv::dnn::MatShape*> cv_dnn_shape_const_intX_const_int(const int* dims, const int n) {
		try {
			cv::dnn::MatShape ret = cv::dnn::shape(dims, n);
			return Ok(new cv::dnn::MatShape(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::MatShape*>))
	}
	
	Result<cv::dnn::MatShape*> cv_dnn_shape_int_int_int_int(int a0, int a1, int a2, int a3) {
		try {
			cv::dnn::MatShape ret = cv::dnn::shape(a0, a1, a2, a3);
			return Ok(new cv::dnn::MatShape(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::MatShape*>))
	}
	
	Result_void cv_dnn_shrinkCaffeModel_const_StringR_const_StringR_const_vector_String_R(const char* src, const char* dst, const std::vector<cv::String>* layersTypes) {
		try {
			cv::dnn::shrinkCaffeModel(cv::String(src), cv::String(dst), *layersTypes);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_dnn_slice_const_MatR_const__RangeR(const cv::Mat* m, const cv::dnn::_Range* r0) {
		try {
			cv::Mat ret = cv::dnn::slice(*m, *r0);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_dnn_slice_const_MatR_const__RangeR_const__RangeR(const cv::Mat* m, const cv::dnn::_Range* r0, const cv::dnn::_Range* r1) {
		try {
			cv::Mat ret = cv::dnn::slice(*m, *r0, *r1);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_dnn_slice_const_MatR_const__RangeR_const__RangeR_const__RangeR(const cv::Mat* m, const cv::dnn::_Range* r0, const cv::dnn::_Range* r1, const cv::dnn::_Range* r2) {
		try {
			cv::Mat ret = cv::dnn::slice(*m, *r0, *r1, *r2);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_dnn_slice_const_MatR_const__RangeR_const__RangeR_const__RangeR_const__RangeR(const cv::Mat* m, const cv::dnn::_Range* r0, const cv::dnn::_Range* r1, const cv::dnn::_Range* r2, const cv::dnn::_Range* r3) {
		try {
			cv::Mat ret = cv::dnn::slice(*m, *r0, *r1, *r2, *r3);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<void*> cv_dnn_toString_const_MatShapeR_const_StringR(const cv::dnn::MatShape* shape, const char* name) {
		try {
			std::string ret = cv::dnn::toString(*shape, cv::String(name));
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<int> cv_dnn_total_const_MatShapeR_int_int(const cv::dnn::MatShape* shape, int start, int end) {
		try {
			int ret = cv::dnn::total(*shape, start, end);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_dnn_writeTextGraph_const_StringR_const_StringR(const char* model, const char* output) {
		try {
			cv::dnn::writeTextGraph(cv::String(model), cv::String(output));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::dnn::AbsLayer>*> cv_dnn_AbsLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::AbsLayer> ret = cv::dnn::AbsLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::AbsLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::AbsLayer>*>))
	}
	
	void cv_AccumLayer_delete(cv::dnn::AccumLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::AccumLayer>*> cv_dnn_AccumLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::AccumLayer> ret = cv::dnn::AccumLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::AccumLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::AccumLayer>*>))
	}
	
	Result_void cv_dnn_ActivationLayer_forwardSlice_const_const_floatX_floatX_int_size_t_int_int(const cv::dnn::ActivationLayer* instance, const float* src, float* dst, int len, size_t outPlaneSize, int cn0, int cn1) {
		try {
			instance->forwardSlice(src, dst, len, outPlaneSize, cn0, cn1);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::dnn::BNLLLayer>*> cv_dnn_BNLLLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::BNLLLayer> ret = cv::dnn::BNLLLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::BNLLLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::BNLLLayer>*>))
	}
	
	Result<int> cv_dnn_BackendNode_getPropBackendId_const(const cv::dnn::BackendNode* instance) {
		try {
			int ret = instance->backendId;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_dnn_BackendNode_setPropBackendId_int(cv::dnn::BackendNode* instance, int val) {
		try {
			instance->backendId = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_BackendNode_delete(cv::dnn::BackendNode* instance) {
		delete instance;
	}
	Result<int> cv_dnn_BackendWrapper_getPropBackendId_const(const cv::dnn::BackendWrapper* instance) {
		try {
			int ret = instance->backendId;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_dnn_BackendWrapper_setPropBackendId_int(cv::dnn::BackendWrapper* instance, int val) {
		try {
			instance->backendId = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_dnn_BackendWrapper_getPropTargetId_const(const cv::dnn::BackendWrapper* instance) {
		try {
			int ret = instance->targetId;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_dnn_BackendWrapper_setPropTargetId_int(cv::dnn::BackendWrapper* instance, int val) {
		try {
			instance->targetId = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_BackendWrapper_copyToHost(cv::dnn::BackendWrapper* instance) {
		try {
			instance->copyToHost();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_BackendWrapper_setHostDirty(cv::dnn::BackendWrapper* instance) {
		try {
			instance->setHostDirty();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_dnn_BaseConvolutionLayer_getPropKernel_const(const cv::dnn::BaseConvolutionLayer* instance) {
		try {
			cv::Size ret = instance->kernel;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setPropKernel_Size(cv::dnn::BaseConvolutionLayer* instance, cv::Size* val) {
		try {
			instance->kernel = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_dnn_BaseConvolutionLayer_getPropStride_const(const cv::dnn::BaseConvolutionLayer* instance) {
		try {
			cv::Size ret = instance->stride;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setPropStride_Size(cv::dnn::BaseConvolutionLayer* instance, cv::Size* val) {
		try {
			instance->stride = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_dnn_BaseConvolutionLayer_getPropPad_const(const cv::dnn::BaseConvolutionLayer* instance) {
		try {
			cv::Size ret = instance->pad;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setPropPad_Size(cv::dnn::BaseConvolutionLayer* instance, cv::Size* val) {
		try {
			instance->pad = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_dnn_BaseConvolutionLayer_getPropDilation_const(const cv::dnn::BaseConvolutionLayer* instance) {
		try {
			cv::Size ret = instance->dilation;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setPropDilation_Size(cv::dnn::BaseConvolutionLayer* instance, cv::Size* val) {
		try {
			instance->dilation = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_dnn_BaseConvolutionLayer_getPropAdjustPad_const(const cv::dnn::BaseConvolutionLayer* instance) {
		try {
			cv::Size ret = instance->adjustPad;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setPropAdjustPad_Size(cv::dnn::BaseConvolutionLayer* instance, cv::Size* val) {
		try {
			instance->adjustPad = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<size_t>*> cv_dnn_BaseConvolutionLayer_getPropAdjust_pads(cv::dnn::BaseConvolutionLayer* instance) {
		try {
			std::vector<size_t> ret = instance->adjust_pads;
			return Ok(new std::vector<size_t>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<size_t>*>))
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setPropAdjust_pads_vector_size_t_(cv::dnn::BaseConvolutionLayer* instance, std::vector<size_t>* val) {
		try {
			instance->adjust_pads = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<size_t>*> cv_dnn_BaseConvolutionLayer_getPropKernel_size(cv::dnn::BaseConvolutionLayer* instance) {
		try {
			std::vector<size_t> ret = instance->kernel_size;
			return Ok(new std::vector<size_t>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<size_t>*>))
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setPropKernel_size_vector_size_t_(cv::dnn::BaseConvolutionLayer* instance, std::vector<size_t>* val) {
		try {
			instance->kernel_size = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<size_t>*> cv_dnn_BaseConvolutionLayer_getPropStrides(cv::dnn::BaseConvolutionLayer* instance) {
		try {
			std::vector<size_t> ret = instance->strides;
			return Ok(new std::vector<size_t>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<size_t>*>))
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setPropStrides_vector_size_t_(cv::dnn::BaseConvolutionLayer* instance, std::vector<size_t>* val) {
		try {
			instance->strides = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<size_t>*> cv_dnn_BaseConvolutionLayer_getPropDilations(cv::dnn::BaseConvolutionLayer* instance) {
		try {
			std::vector<size_t> ret = instance->dilations;
			return Ok(new std::vector<size_t>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<size_t>*>))
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setPropDilations_vector_size_t_(cv::dnn::BaseConvolutionLayer* instance, std::vector<size_t>* val) {
		try {
			instance->dilations = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<size_t>*> cv_dnn_BaseConvolutionLayer_getPropPads_begin(cv::dnn::BaseConvolutionLayer* instance) {
		try {
			std::vector<size_t> ret = instance->pads_begin;
			return Ok(new std::vector<size_t>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<size_t>*>))
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setPropPads_begin_vector_size_t_(cv::dnn::BaseConvolutionLayer* instance, std::vector<size_t>* val) {
		try {
			instance->pads_begin = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<size_t>*> cv_dnn_BaseConvolutionLayer_getPropPads_end(cv::dnn::BaseConvolutionLayer* instance) {
		try {
			std::vector<size_t> ret = instance->pads_end;
			return Ok(new std::vector<size_t>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<size_t>*>))
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setPropPads_end_vector_size_t_(cv::dnn::BaseConvolutionLayer* instance, std::vector<size_t>* val) {
		try {
			instance->pads_end = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_dnn_BaseConvolutionLayer_getPropPadMode_const(const cv::dnn::BaseConvolutionLayer* instance) {
		try {
			cv::String ret = instance->padMode;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setPropPadMode_String(cv::dnn::BaseConvolutionLayer* instance, char* val) {
		try {
			instance->padMode = cv::String(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_dnn_BaseConvolutionLayer_getPropNumOutput_const(const cv::dnn::BaseConvolutionLayer* instance) {
		try {
			int ret = instance->numOutput;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setPropNumOutput_int(cv::dnn::BaseConvolutionLayer* instance, int val) {
		try {
			instance->numOutput = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_BaseConvolutionLayer_delete(cv::dnn::BaseConvolutionLayer* instance) {
		delete instance;
	}
	Result<bool> cv_dnn_BatchNormLayer_getPropHasWeights_const(const cv::dnn::BatchNormLayer* instance) {
		try {
			bool ret = instance->hasWeights;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_dnn_BatchNormLayer_setPropHasWeights_bool(cv::dnn::BatchNormLayer* instance, bool val) {
		try {
			instance->hasWeights = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_dnn_BatchNormLayer_getPropHasBias_const(const cv::dnn::BatchNormLayer* instance) {
		try {
			bool ret = instance->hasBias;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_dnn_BatchNormLayer_setPropHasBias_bool(cv::dnn::BatchNormLayer* instance, bool val) {
		try {
			instance->hasBias = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_dnn_BatchNormLayer_getPropEpsilon_const(const cv::dnn::BatchNormLayer* instance) {
		try {
			float ret = instance->epsilon;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_dnn_BatchNormLayer_setPropEpsilon_float(cv::dnn::BatchNormLayer* instance, float val) {
		try {
			instance->epsilon = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::dnn::BatchNormLayer>*> cv_dnn_BatchNormLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::BatchNormLayer> ret = cv::dnn::BatchNormLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::BatchNormLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::BatchNormLayer>*>))
	}
	
	void cv_BlankLayer_delete(cv::dnn::BlankLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::Layer>*> cv_dnn_BlankLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::BlankLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::Layer>*>))
	}
	
	Result<cv::Ptr<cv::dnn::Layer>*> cv_dnn_ChannelsPReLULayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::ChannelsPReLULayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::Layer>*>))
	}
	
	Result<int> cv_dnn_ConcatLayer_getPropAxis_const(const cv::dnn::ConcatLayer* instance) {
		try {
			int ret = instance->axis;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_dnn_ConcatLayer_setPropAxis_int(cv::dnn::ConcatLayer* instance, int val) {
		try {
			instance->axis = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_dnn_ConcatLayer_getPropPadding_const(const cv::dnn::ConcatLayer* instance) {
		try {
			bool ret = instance->padding;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_dnn_ConcatLayer_setPropPadding_bool(cv::dnn::ConcatLayer* instance, bool val) {
		try {
			instance->padding = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_ConcatLayer_delete(cv::dnn::ConcatLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::ConcatLayer>*> cv_dnn_ConcatLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::ConcatLayer> ret = cv::dnn::ConcatLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::ConcatLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::ConcatLayer>*>))
	}
	
	void cv_ConstLayer_delete(cv::dnn::ConstLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::Layer>*> cv_dnn_ConstLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::ConstLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::Layer>*>))
	}
	
	void cv_ConvolutionLayer_delete(cv::dnn::ConvolutionLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::BaseConvolutionLayer>*> cv_dnn_ConvolutionLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::BaseConvolutionLayer> ret = cv::dnn::ConvolutionLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::BaseConvolutionLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::BaseConvolutionLayer>*>))
	}
	
	void cv_CorrelationLayer_delete(cv::dnn::CorrelationLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::CorrelationLayer>*> cv_dnn_CorrelationLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::CorrelationLayer> ret = cv::dnn::CorrelationLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::CorrelationLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::CorrelationLayer>*>))
	}
	
	void cv_CropAndResizeLayer_delete(cv::dnn::CropAndResizeLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::Layer>*> cv_dnn_CropAndResizeLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::CropAndResizeLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::Layer>*>))
	}
	
	void cv_CropLayer_delete(cv::dnn::CropLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::Layer>*> cv_dnn_CropLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::CropLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::Layer>*>))
	}
	
	void cv_DataAugmentationLayer_delete(cv::dnn::DataAugmentationLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::DataAugmentationLayer>*> cv_dnn_DataAugmentationLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::DataAugmentationLayer> ret = cv::dnn::DataAugmentationLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::DataAugmentationLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::DataAugmentationLayer>*>))
	}
	
	void cv_DeconvolutionLayer_delete(cv::dnn::DeconvolutionLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::BaseConvolutionLayer>*> cv_dnn_DeconvolutionLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::BaseConvolutionLayer> ret = cv::dnn::DeconvolutionLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::BaseConvolutionLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::BaseConvolutionLayer>*>))
	}
	
	void cv_DetectionOutputLayer_delete(cv::dnn::DetectionOutputLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::DetectionOutputLayer>*> cv_dnn_DetectionOutputLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::DetectionOutputLayer> ret = cv::dnn::DetectionOutputLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::DetectionOutputLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::DetectionOutputLayer>*>))
	}
	
	void cv_Dict_delete(cv::dnn::Dict* instance) {
		delete instance;
	}
	Result<bool> cv_dnn_Dict_has_const_const_StringR(const cv::dnn::Dict* instance, const char* key) {
		try {
			bool ret = instance->has(cv::String(key));
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<cv::dnn::DictValue**> cv_dnn_Dict_ptr_const_StringR(cv::dnn::Dict* instance, const char* key) {
		try {
			cv::dnn::DictValue* ret = instance->ptr(cv::String(key));
			return Ok(new cv::dnn::DictValue*(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::DictValue**>))
	}
	
	Result<const cv::dnn::DictValue**> cv_dnn_Dict_ptr_const_const_StringR(const cv::dnn::Dict* instance, const char* key) {
		try {
			const cv::dnn::DictValue* ret = instance->ptr(cv::String(key));
			return Ok(new const cv::dnn::DictValue*(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<const cv::dnn::DictValue**>))
	}
	
	Result<const cv::dnn::DictValue*> cv_dnn_Dict_get_const_const_StringR(const cv::dnn::Dict* instance, const char* key) {
		try {
			const cv::dnn::DictValue ret = instance->get(cv::String(key));
			return Ok(new const cv::dnn::DictValue(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<const cv::dnn::DictValue*>))
	}
	
	Result<void*> cv_dnn_Dict_set_cv_String_const_StringR_const_StringR(cv::dnn::Dict* instance, const char* key, const char* value) {
		try {
			const cv::String ret = instance->set<cv::String>(cv::String(key), cv::String(value));
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<const cv::dnn::DictValue*> cv_dnn_Dict_set_cv_dnn_DictValue_const_StringR_const_DictValueR(cv::dnn::Dict* instance, const char* key, const cv::dnn::DictValue* value) {
		try {
			const cv::dnn::DictValue ret = instance->set<cv::dnn::DictValue>(cv::String(key), *value);
			return Ok(new const cv::dnn::DictValue(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<const cv::dnn::DictValue*>))
	}
	
	Result<const double> cv_dnn_Dict_set_double_const_StringR_const_doubleR(cv::dnn::Dict* instance, const char* key, const double* value) {
		try {
			const double ret = instance->set<double>(cv::String(key), *value);
			return Ok<const double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const double>))
	}
	
	Result<const int64_t> cv_dnn_Dict_set_int64_t_const_StringR_const_int64_tR(cv::dnn::Dict* instance, const char* key, const int64_t* value) {
		try {
			const int64_t ret = instance->set<int64_t>(cv::String(key), *value);
			return Ok<const int64_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const int64_t>))
	}
	
	Result_void cv_dnn_Dict_erase_const_StringR(cv::dnn::Dict* instance, const char* key) {
		try {
			instance->erase(cv::String(key));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_DictValue_delete(cv::dnn::DictValue* instance) {
		delete instance;
	}
	Result<cv::dnn::DictValue*> cv_dnn_DictValue_DictValue_const_DictValueR(const cv::dnn::DictValue* r) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(*r);
			return Ok<cv::dnn::DictValue*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::DictValue*>))
	}
	
	Result<cv::dnn::DictValue*> cv_dnn_DictValue_DictValue_bool(bool i) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(i);
			return Ok<cv::dnn::DictValue*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::DictValue*>))
	}
	
	Result<cv::dnn::DictValue*> cv_dnn_DictValue_DictValue_int64_t(int64_t i) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(i);
			return Ok<cv::dnn::DictValue*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::DictValue*>))
	}
	
	Result<cv::dnn::DictValue*> cv_dnn_DictValue_DictValue_int(int i) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(i);
			return Ok<cv::dnn::DictValue*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::DictValue*>))
	}
	
	Result<cv::dnn::DictValue*> cv_dnn_DictValue_DictValue_unsigned_int(unsigned int p) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(p);
			return Ok<cv::dnn::DictValue*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::DictValue*>))
	}
	
	Result<cv::dnn::DictValue*> cv_dnn_DictValue_DictValue_double(double p) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(p);
			return Ok<cv::dnn::DictValue*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::DictValue*>))
	}
	
	Result<cv::dnn::DictValue*> cv_dnn_DictValue_DictValue_const_charX(const char* s) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(s);
			return Ok<cv::dnn::DictValue*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::DictValue*>))
	}
	
	Result<void*> cv_dnn_DictValue_get_cv_String_const_int(const cv::dnn::DictValue* instance, int idx) {
		try {
			cv::String ret = instance->get<cv::String>(idx);
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<double> cv_dnn_DictValue_get_double_const_int(const cv::dnn::DictValue* instance, int idx) {
		try {
			double ret = instance->get<double>(idx);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<int> cv_dnn_DictValue_get_int_const_int(const cv::dnn::DictValue* instance, int idx) {
		try {
			int ret = instance->get<int>(idx);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int64_t> cv_dnn_DictValue_get_int64_t_const_int(const cv::dnn::DictValue* instance, int idx) {
		try {
			int64_t ret = instance->get<int64_t>(idx);
			return Ok<int64_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int64_t>))
	}
	
	Result<int> cv_dnn_DictValue_size_const(const cv::dnn::DictValue* instance) {
		try {
			int ret = instance->size();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<bool> cv_dnn_DictValue_isInt_const(const cv::dnn::DictValue* instance) {
		try {
			bool ret = instance->isInt();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_dnn_DictValue_isString_const(const cv::dnn::DictValue* instance) {
		try {
			bool ret = instance->isString();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_dnn_DictValue_isReal_const(const cv::dnn::DictValue* instance) {
		try {
			bool ret = instance->isReal();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<int> cv_dnn_DictValue_getIntValue_const_int(const cv::dnn::DictValue* instance, int idx) {
		try {
			int ret = instance->getIntValue(idx);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<double> cv_dnn_DictValue_getRealValue_const_int(const cv::dnn::DictValue* instance, int idx) {
		try {
			double ret = instance->getRealValue(idx);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<void*> cv_dnn_DictValue_getStringValue_const_int(const cv::dnn::DictValue* instance, int idx) {
		try {
			cv::String ret = instance->getStringValue(idx);
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<cv::Ptr<cv::dnn::ELULayer>*> cv_dnn_ELULayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::ELULayer> ret = cv::dnn::ELULayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::ELULayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::ELULayer>*>))
	}
	
	void cv_EltwiseLayer_delete(cv::dnn::EltwiseLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::EltwiseLayer>*> cv_dnn_EltwiseLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::EltwiseLayer> ret = cv::dnn::EltwiseLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::EltwiseLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::EltwiseLayer>*>))
	}
	
	void cv_FlattenLayer_delete(cv::dnn::FlattenLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::FlattenLayer>*> cv_dnn_FlattenLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::FlattenLayer> ret = cv::dnn::FlattenLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::FlattenLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::FlattenLayer>*>))
	}
	
	void cv_FlowWarpLayer_delete(cv::dnn::FlowWarpLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::FlowWarpLayer>*> cv_dnn_FlowWarpLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::FlowWarpLayer> ret = cv::dnn::FlowWarpLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::FlowWarpLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::FlowWarpLayer>*>))
	}
	
	Result<int> cv_dnn_InnerProductLayer_getPropAxis_const(const cv::dnn::InnerProductLayer* instance) {
		try {
			int ret = instance->axis;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_dnn_InnerProductLayer_setPropAxis_int(cv::dnn::InnerProductLayer* instance, int val) {
		try {
			instance->axis = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_InnerProductLayer_delete(cv::dnn::InnerProductLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::InnerProductLayer>*> cv_dnn_InnerProductLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::InnerProductLayer> ret = cv::dnn::InnerProductLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::InnerProductLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::InnerProductLayer>*>))
	}
	
	void cv_InterpLayer_delete(cv::dnn::InterpLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::Layer>*> cv_dnn_InterpLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::InterpLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::Layer>*>))
	}
	
	Result<int> cv_dnn_LRNLayer_getPropType_const(const cv::dnn::LRNLayer* instance) {
		try {
			int ret = instance->type;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_dnn_LRNLayer_setPropType_int(cv::dnn::LRNLayer* instance, int val) {
		try {
			instance->type = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_dnn_LRNLayer_getPropSize_const(const cv::dnn::LRNLayer* instance) {
		try {
			int ret = instance->size;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_dnn_LRNLayer_setPropSize_int(cv::dnn::LRNLayer* instance, int val) {
		try {
			instance->size = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_dnn_LRNLayer_getPropAlpha_const(const cv::dnn::LRNLayer* instance) {
		try {
			float ret = instance->alpha;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_dnn_LRNLayer_setPropAlpha_float(cv::dnn::LRNLayer* instance, float val) {
		try {
			instance->alpha = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_dnn_LRNLayer_getPropBeta_const(const cv::dnn::LRNLayer* instance) {
		try {
			float ret = instance->beta;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_dnn_LRNLayer_setPropBeta_float(cv::dnn::LRNLayer* instance, float val) {
		try {
			instance->beta = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_dnn_LRNLayer_getPropBias_const(const cv::dnn::LRNLayer* instance) {
		try {
			float ret = instance->bias;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_dnn_LRNLayer_setPropBias_float(cv::dnn::LRNLayer* instance, float val) {
		try {
			instance->bias = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_dnn_LRNLayer_getPropNormBySize_const(const cv::dnn::LRNLayer* instance) {
		try {
			bool ret = instance->normBySize;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_dnn_LRNLayer_setPropNormBySize_bool(cv::dnn::LRNLayer* instance, bool val) {
		try {
			instance->normBySize = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_LRNLayer_delete(cv::dnn::LRNLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::LRNLayer>*> cv_dnn_LRNLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::LRNLayer> ret = cv::dnn::LRNLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::LRNLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::LRNLayer>*>))
	}
	
	Result<cv::Ptr<cv::dnn::LSTMLayer>*> cv_dnn_LSTMLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::LSTMLayer> ret = cv::dnn::LSTMLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::LSTMLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::LSTMLayer>*>))
	}
	
	Result_void cv_dnn_LSTMLayer_setWeights_const_MatR_const_MatR_const_MatR(cv::dnn::LSTMLayer* instance, const cv::Mat* Wh, const cv::Mat* Wx, const cv::Mat* b) {
		try {
			instance->setWeights(*Wh, *Wx, *b);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_LSTMLayer_setOutShape_const_MatShapeR(cv::dnn::LSTMLayer* instance, const cv::dnn::MatShape* outTailShape) {
		try {
			instance->setOutShape(*outTailShape);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_LSTMLayer_setUseTimstampsDim_bool(cv::dnn::LSTMLayer* instance, bool use) {
		try {
			instance->setUseTimstampsDim(use);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_LSTMLayer_setProduceCellOutput_bool(cv::dnn::LSTMLayer* instance, bool produce) {
		try {
			instance->setProduceCellOutput(produce);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_dnn_LSTMLayer_inputNameToIndex_String(cv::dnn::LSTMLayer* instance, char* inputName) {
		try {
			int ret = instance->inputNameToIndex(cv::String(inputName));
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_dnn_LSTMLayer_outputNameToIndex_const_StringR(cv::dnn::LSTMLayer* instance, const char* outputName) {
		try {
			int ret = instance->outputNameToIndex(cv::String(outputName));
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<std::vector<cv::Mat>*> cv_dnn_Layer_getPropBlobs(cv::dnn::Layer* instance) {
		try {
			std::vector<cv::Mat> ret = instance->blobs;
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Mat>*>))
	}
	
	Result_void cv_dnn_Layer_setPropBlobs_vector_Mat_(cv::dnn::Layer* instance, std::vector<cv::Mat>* val) {
		try {
			instance->blobs = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_dnn_Layer_getPropName_const(const cv::dnn::Layer* instance) {
		try {
			cv::String ret = instance->name;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_dnn_Layer_setPropName_String(cv::dnn::Layer* instance, char* val) {
		try {
			instance->name = cv::String(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_dnn_Layer_getPropType_const(const cv::dnn::Layer* instance) {
		try {
			cv::String ret = instance->type;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_dnn_Layer_setPropType_String(cv::dnn::Layer* instance, char* val) {
		try {
			instance->type = cv::String(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_dnn_Layer_getPropPreferableTarget_const(const cv::dnn::Layer* instance) {
		try {
			int ret = instance->preferableTarget;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_dnn_Layer_setPropPreferableTarget_int(cv::dnn::Layer* instance, int val) {
		try {
			instance->preferableTarget = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Layer_delete(cv::dnn::Layer* instance) {
		delete instance;
	}
	Result_void cv_dnn_Layer_finalize_const__InputArrayR_const__OutputArrayR(cv::dnn::Layer* instance, const cv::_InputArray* inputs, const cv::_OutputArray* outputs) {
		try {
			instance->finalize(*inputs, *outputs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Layer_forward_vector_MatX_R_vector_Mat_R_vector_Mat_R(cv::dnn::Layer* instance, std::vector<cv::Mat*>* input, std::vector<cv::Mat>* output, std::vector<cv::Mat>* internals) {
		try {
			instance->forward(*input, *output, *internals);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Layer_forward_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::dnn::Layer* instance, const cv::_InputArray* inputs, const cv::_OutputArray* outputs, const cv::_OutputArray* internals) {
		try {
			instance->forward(*inputs, *outputs, *internals);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Layer_forward_fallback_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::dnn::Layer* instance, const cv::_InputArray* inputs, const cv::_OutputArray* outputs, const cv::_OutputArray* internals) {
		try {
			instance->forward_fallback(*inputs, *outputs, *internals);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Layer_finalize_const_vector_Mat_R_vector_Mat_R(cv::dnn::Layer* instance, const std::vector<cv::Mat>* inputs, std::vector<cv::Mat>* outputs) {
		try {
			instance->finalize(*inputs, *outputs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<cv::Mat>*> cv_dnn_Layer_finalize_const_vector_Mat_R(cv::dnn::Layer* instance, const std::vector<cv::Mat>* inputs) {
		try {
			std::vector<cv::Mat> ret = instance->finalize(*inputs);
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Mat>*>))
	}
	
	Result_void cv_dnn_Layer_run_const_vector_Mat_R_vector_Mat_R_vector_Mat_R(cv::dnn::Layer* instance, const std::vector<cv::Mat>* inputs, std::vector<cv::Mat>* outputs, std::vector<cv::Mat>* internals) {
		try {
			instance->run(*inputs, *outputs, *internals);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_dnn_Layer_inputNameToIndex_String(cv::dnn::Layer* instance, char* inputName) {
		try {
			int ret = instance->inputNameToIndex(cv::String(inputName));
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_dnn_Layer_outputNameToIndex_const_StringR(cv::dnn::Layer* instance, const char* outputName) {
		try {
			int ret = instance->outputNameToIndex(cv::String(outputName));
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<bool> cv_dnn_Layer_supportBackend_int(cv::dnn::Layer* instance, int backendId) {
		try {
			bool ret = instance->supportBackend(backendId);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<cv::Ptr<cv::dnn::BackendNode>*> cv_dnn_Layer_initHalide_const_vector_Ptr_BackendWrapper__R(cv::dnn::Layer* instance, const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* inputs) {
		try {
			cv::Ptr<cv::dnn::BackendNode> ret = instance->initHalide(*inputs);
			return Ok(new cv::Ptr<cv::dnn::BackendNode>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::BackendNode>*>))
	}
	
	Result<cv::Ptr<cv::dnn::BackendNode>*> cv_dnn_Layer_initInfEngine_const_vector_Ptr_BackendWrapper__R(cv::dnn::Layer* instance, const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* inputs) {
		try {
			cv::Ptr<cv::dnn::BackendNode> ret = instance->initInfEngine(*inputs);
			return Ok(new cv::Ptr<cv::dnn::BackendNode>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::BackendNode>*>))
	}
	
	Result<cv::Ptr<cv::dnn::BackendNode>*> cv_dnn_Layer_initNgraph_const_vector_Ptr_BackendWrapper__R_const_vector_Ptr_BackendNode__R(cv::dnn::Layer* instance, const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* inputs, const std::vector<cv::Ptr<cv::dnn::BackendNode>>* nodes) {
		try {
			cv::Ptr<cv::dnn::BackendNode> ret = instance->initNgraph(*inputs, *nodes);
			return Ok(new cv::Ptr<cv::dnn::BackendNode>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::BackendNode>*>))
	}
	
	Result_void cv_dnn_Layer_applyHalideScheduler_const_Ptr_BackendNode_R_const_vector_MatX_R_const_vector_Mat_R_int(const cv::dnn::Layer* instance, cv::Ptr<cv::dnn::BackendNode>* node, const std::vector<cv::Mat*>* inputs, const std::vector<cv::Mat>* outputs, int targetId) {
		try {
			instance->applyHalideScheduler(*node, *inputs, *outputs, targetId);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::dnn::BackendNode>*> cv_dnn_Layer_tryAttach_const_Ptr_BackendNode_R(cv::dnn::Layer* instance, const cv::Ptr<cv::dnn::BackendNode>* node) {
		try {
			cv::Ptr<cv::dnn::BackendNode> ret = instance->tryAttach(*node);
			return Ok(new cv::Ptr<cv::dnn::BackendNode>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::BackendNode>*>))
	}
	
	Result<bool> cv_dnn_Layer_setActivation_const_Ptr_ActivationLayer_R(cv::dnn::Layer* instance, const cv::Ptr<cv::dnn::ActivationLayer>* layer) {
		try {
			bool ret = instance->setActivation(*layer);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_dnn_Layer_tryFuse_Ptr_Layer_R(cv::dnn::Layer* instance, cv::Ptr<cv::dnn::Layer>* top) {
		try {
			bool ret = instance->tryFuse(*top);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_dnn_Layer_getScaleShift_const_MatR_MatR(const cv::dnn::Layer* instance, cv::Mat* scale, cv::Mat* shift) {
		try {
			instance->getScaleShift(*scale, *shift);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Layer_unsetAttached(cv::dnn::Layer* instance) {
		try {
			instance->unsetAttached();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_dnn_Layer_getMemoryShapes_const_const_vector_MatShape_R_const_int_vector_MatShape_R_vector_MatShape_R(const cv::dnn::Layer* instance, const std::vector<cv::dnn::MatShape>* inputs, const int requiredOutputs, std::vector<cv::dnn::MatShape>* outputs, std::vector<cv::dnn::MatShape>* internals) {
		try {
			bool ret = instance->getMemoryShapes(*inputs, requiredOutputs, *outputs, *internals);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<int64_t> cv_dnn_Layer_getFLOPS_const_const_vector_MatShape_R_const_vector_MatShape_R(const cv::dnn::Layer* instance, const std::vector<cv::dnn::MatShape>* inputs, const std::vector<cv::dnn::MatShape>* outputs) {
		try {
			int64_t ret = instance->getFLOPS(*inputs, *outputs);
			return Ok<int64_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int64_t>))
	}
	
	Result<bool> cv_dnn_Layer_updateMemoryShapes_const_vector_MatShape_R(cv::dnn::Layer* instance, const std::vector<cv::dnn::MatShape>* inputs) {
		try {
			bool ret = instance->updateMemoryShapes(*inputs);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<cv::dnn::Layer*> cv_dnn_Layer_Layer() {
		try {
			cv::dnn::Layer* ret = new cv::dnn::Layer();
			return Ok<cv::dnn::Layer*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Layer*>))
	}
	
	Result<cv::dnn::Layer*> cv_dnn_Layer_Layer_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::dnn::Layer* ret = new cv::dnn::Layer(*params);
			return Ok<cv::dnn::Layer*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Layer*>))
	}
	
	Result_void cv_dnn_Layer_setParamsFrom_const_LayerParamsR(cv::dnn::Layer* instance, const cv::dnn::LayerParams* params) {
		try {
			instance->setParamsFrom(*params);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_LayerFactory_delete(cv::dnn::LayerFactory* instance) {
		delete instance;
	}
	Result_void cv_dnn_LayerFactory_registerLayer_const_StringR_Constructor(const char* type, cv::dnn::LayerFactory::Constructor constructor) {
		try {
			cv::dnn::LayerFactory::registerLayer(cv::String(type), constructor);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_LayerFactory_unregisterLayer_const_StringR(const char* type) {
		try {
			cv::dnn::LayerFactory::unregisterLayer(cv::String(type));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::dnn::Layer>*> cv_dnn_LayerFactory_createLayerInstance_const_StringR_LayerParamsR(const char* type, cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::LayerFactory::createLayerInstance(cv::String(type), *params);
			return Ok(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::Layer>*>))
	}
	
	Result<std::vector<cv::Mat>*> cv_dnn_LayerParams_getPropBlobs(cv::dnn::LayerParams* instance) {
		try {
			std::vector<cv::Mat> ret = instance->blobs;
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Mat>*>))
	}
	
	Result_void cv_dnn_LayerParams_setPropBlobs_vector_Mat_(cv::dnn::LayerParams* instance, std::vector<cv::Mat>* val) {
		try {
			instance->blobs = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_dnn_LayerParams_getPropName_const(const cv::dnn::LayerParams* instance) {
		try {
			cv::String ret = instance->name;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_dnn_LayerParams_setPropName_String(cv::dnn::LayerParams* instance, char* val) {
		try {
			instance->name = cv::String(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_dnn_LayerParams_getPropType_const(const cv::dnn::LayerParams* instance) {
		try {
			cv::String ret = instance->type;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_dnn_LayerParams_setPropType_String(cv::dnn::LayerParams* instance, char* val) {
		try {
			instance->type = cv::String(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_LayerParams_delete(cv::dnn::LayerParams* instance) {
		delete instance;
	}
	Result<float> cv_dnn_MVNLayer_getPropEps_const(const cv::dnn::MVNLayer* instance) {
		try {
			float ret = instance->eps;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_dnn_MVNLayer_setPropEps_float(cv::dnn::MVNLayer* instance, float val) {
		try {
			instance->eps = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_dnn_MVNLayer_getPropNormVariance_const(const cv::dnn::MVNLayer* instance) {
		try {
			bool ret = instance->normVariance;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_dnn_MVNLayer_setPropNormVariance_bool(cv::dnn::MVNLayer* instance, bool val) {
		try {
			instance->normVariance = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_dnn_MVNLayer_getPropAcrossChannels_const(const cv::dnn::MVNLayer* instance) {
		try {
			bool ret = instance->acrossChannels;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_dnn_MVNLayer_setPropAcrossChannels_bool(cv::dnn::MVNLayer* instance, bool val) {
		try {
			instance->acrossChannels = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_MVNLayer_delete(cv::dnn::MVNLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::MVNLayer>*> cv_dnn_MVNLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::MVNLayer> ret = cv::dnn::MVNLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::MVNLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::MVNLayer>*>))
	}
	
	Result<cv::Size> cv_dnn_MaxUnpoolLayer_getPropPoolKernel_const(const cv::dnn::MaxUnpoolLayer* instance) {
		try {
			cv::Size ret = instance->poolKernel;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_dnn_MaxUnpoolLayer_setPropPoolKernel_Size(cv::dnn::MaxUnpoolLayer* instance, cv::Size* val) {
		try {
			instance->poolKernel = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_dnn_MaxUnpoolLayer_getPropPoolPad_const(const cv::dnn::MaxUnpoolLayer* instance) {
		try {
			cv::Size ret = instance->poolPad;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_dnn_MaxUnpoolLayer_setPropPoolPad_Size(cv::dnn::MaxUnpoolLayer* instance, cv::Size* val) {
		try {
			instance->poolPad = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_dnn_MaxUnpoolLayer_getPropPoolStride_const(const cv::dnn::MaxUnpoolLayer* instance) {
		try {
			cv::Size ret = instance->poolStride;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_dnn_MaxUnpoolLayer_setPropPoolStride_Size(cv::dnn::MaxUnpoolLayer* instance, cv::Size* val) {
		try {
			instance->poolStride = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_MaxUnpoolLayer_delete(cv::dnn::MaxUnpoolLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::MaxUnpoolLayer>*> cv_dnn_MaxUnpoolLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::MaxUnpoolLayer> ret = cv::dnn::MaxUnpoolLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::MaxUnpoolLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::MaxUnpoolLayer>*>))
	}
	
	Result<cv::Ptr<cv::dnn::MishLayer>*> cv_dnn_MishLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::MishLayer> ret = cv::dnn::MishLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::MishLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::MishLayer>*>))
	}
	
	void cv_Net_delete(cv::dnn::Net* instance) {
		delete instance;
	}
	Result<cv::dnn::Net*> cv_dnn_Net_Net() {
		try {
			cv::dnn::Net* ret = new cv::dnn::Net();
			return Ok<cv::dnn::Net*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Net*>))
	}
	
	Result<cv::dnn::Net*> cv_dnn_Net_readFromModelOptimizer_const_StringR_const_StringR(const char* xml, const char* bin) {
		try {
			cv::dnn::Net ret = cv::dnn::Net::readFromModelOptimizer(cv::String(xml), cv::String(bin));
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Net*>))
	}
	
	Result<cv::dnn::Net*> cv_dnn_Net_readFromModelOptimizer_const_vector_unsigned_char_R_const_vector_unsigned_char_R(const std::vector<unsigned char>* bufferModelConfig, const std::vector<unsigned char>* bufferWeights) {
		try {
			cv::dnn::Net ret = cv::dnn::Net::readFromModelOptimizer(*bufferModelConfig, *bufferWeights);
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Net*>))
	}
	
	Result<cv::dnn::Net*> cv_dnn_Net_readFromModelOptimizer_const_unsigned_charX_size_t_const_unsigned_charX_size_t(const unsigned char* bufferModelConfigPtr, size_t bufferModelConfigSize, const unsigned char* bufferWeightsPtr, size_t bufferWeightsSize) {
		try {
			cv::dnn::Net ret = cv::dnn::Net::readFromModelOptimizer(bufferModelConfigPtr, bufferModelConfigSize, bufferWeightsPtr, bufferWeightsSize);
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::Net*>))
	}
	
	Result<bool> cv_dnn_Net_empty_const(const cv::dnn::Net* instance) {
		try {
			bool ret = instance->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<void*> cv_dnn_Net_dump(cv::dnn::Net* instance) {
		try {
			cv::String ret = instance->dump();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_dnn_Net_dumpToFile_const_StringR(cv::dnn::Net* instance, const char* path) {
		try {
			instance->dumpToFile(cv::String(path));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_dnn_Net_addLayer_const_StringR_const_StringR_LayerParamsR(cv::dnn::Net* instance, const char* name, const char* type, cv::dnn::LayerParams* params) {
		try {
			int ret = instance->addLayer(cv::String(name), cv::String(type), *params);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_dnn_Net_addLayerToPrev_const_StringR_const_StringR_LayerParamsR(cv::dnn::Net* instance, const char* name, const char* type, cv::dnn::LayerParams* params) {
		try {
			int ret = instance->addLayerToPrev(cv::String(name), cv::String(type), *params);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_dnn_Net_getLayerId_const_StringR(cv::dnn::Net* instance, const char* layer) {
		try {
			int ret = instance->getLayerId(cv::String(layer));
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<std::vector<cv::String>*> cv_dnn_Net_getLayerNames_const(const cv::dnn::Net* instance) {
		try {
			std::vector<cv::String> ret = instance->getLayerNames();
			return Ok(new std::vector<cv::String>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::String>*>))
	}
	
	Result<cv::Ptr<cv::dnn::Layer>*> cv_dnn_Net_getLayer_LayerId(cv::dnn::Net* instance, cv::dnn::Net::LayerId* layerId) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = instance->getLayer(*layerId);
			return Ok(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::Layer>*>))
	}
	
	Result<std::vector<cv::Ptr<cv::dnn::Layer>>*> cv_dnn_Net_getLayerInputs_LayerId(cv::dnn::Net* instance, cv::dnn::Net::LayerId* layerId) {
		try {
			std::vector<cv::Ptr<cv::dnn::Layer>> ret = instance->getLayerInputs(*layerId);
			return Ok(new std::vector<cv::Ptr<cv::dnn::Layer>>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Ptr<cv::dnn::Layer>>*>))
	}
	
	Result_void cv_dnn_Net_connect_String_String(cv::dnn::Net* instance, char* outPin, char* inpPin) {
		try {
			instance->connect(cv::String(outPin), cv::String(inpPin));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Net_connect_int_int_int_int(cv::dnn::Net* instance, int outLayerId, int outNum, int inpLayerId, int inpNum) {
		try {
			instance->connect(outLayerId, outNum, inpLayerId, inpNum);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Net_setInputsNames_const_vector_String_R(cv::dnn::Net* instance, const std::vector<cv::String>* inputBlobNames) {
		try {
			instance->setInputsNames(*inputBlobNames);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Net_setInputShape_const_StringR_const_MatShapeR(cv::dnn::Net* instance, const char* inputName, const cv::dnn::MatShape* shape) {
		try {
			instance->setInputShape(cv::String(inputName), *shape);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_dnn_Net_forward_const_StringR(cv::dnn::Net* instance, const char* outputName) {
		try {
			cv::Mat ret = instance->forward(cv::String(outputName));
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::AsyncArray*> cv_dnn_Net_forwardAsync_const_StringR(cv::dnn::Net* instance, const char* outputName) {
		try {
			cv::AsyncArray ret = instance->forwardAsync(cv::String(outputName));
			return Ok(new cv::AsyncArray(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::AsyncArray*>))
	}
	
	Result_void cv_dnn_Net_forward_const__OutputArrayR_const_StringR(cv::dnn::Net* instance, const cv::_OutputArray* outputBlobs, const char* outputName) {
		try {
			instance->forward(*outputBlobs, cv::String(outputName));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Net_forward_const__OutputArrayR_const_vector_String_R(cv::dnn::Net* instance, const cv::_OutputArray* outputBlobs, const std::vector<cv::String>* outBlobNames) {
		try {
			instance->forward(*outputBlobs, *outBlobNames);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Net_forward_vector_vector_Mat__R_const_vector_String_R(cv::dnn::Net* instance, std::vector<std::vector<cv::Mat>>* outputBlobs, const std::vector<cv::String>* outBlobNames) {
		try {
			instance->forward(*outputBlobs, *outBlobNames);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Net_setHalideScheduler_const_StringR(cv::dnn::Net* instance, const char* scheduler) {
		try {
			instance->setHalideScheduler(cv::String(scheduler));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Net_setPreferableBackend_int(cv::dnn::Net* instance, int backendId) {
		try {
			instance->setPreferableBackend(backendId);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Net_setPreferableTarget_int(cv::dnn::Net* instance, int targetId) {
		try {
			instance->setPreferableTarget(targetId);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Net_setInput_const__InputArrayR_const_StringR_double_const_ScalarR(cv::dnn::Net* instance, const cv::_InputArray* blob, const char* name, double scalefactor, const cv::Scalar* mean) {
		try {
			instance->setInput(*blob, cv::String(name), scalefactor, *mean);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Net_setParam_LayerId_int_const_MatR(cv::dnn::Net* instance, cv::dnn::Net::LayerId* layer, int numParam, const cv::Mat* blob) {
		try {
			instance->setParam(*layer, numParam, *blob);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_dnn_Net_getParam_LayerId_int(cv::dnn::Net* instance, cv::dnn::Net::LayerId* layer, int numParam) {
		try {
			cv::Mat ret = instance->getParam(*layer, numParam);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<std::vector<int>*> cv_dnn_Net_getUnconnectedOutLayers_const(const cv::dnn::Net* instance) {
		try {
			std::vector<int> ret = instance->getUnconnectedOutLayers();
			return Ok(new std::vector<int>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<int>*>))
	}
	
	Result<std::vector<cv::String>*> cv_dnn_Net_getUnconnectedOutLayersNames_const(const cv::dnn::Net* instance) {
		try {
			std::vector<cv::String> ret = instance->getUnconnectedOutLayersNames();
			return Ok(new std::vector<cv::String>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::String>*>))
	}
	
	Result_void cv_dnn_Net_getLayersShapes_const_const_vector_MatShape_R_vector_int_R_vector_vector_MatShape__R_vector_vector_MatShape__R(const cv::dnn::Net* instance, const std::vector<cv::dnn::MatShape>* netInputShapes, std::vector<int>* layersIds, std::vector<std::vector<cv::dnn::MatShape>>* inLayersShapes, std::vector<std::vector<cv::dnn::MatShape>>* outLayersShapes) {
		try {
			instance->getLayersShapes(*netInputShapes, *layersIds, *inLayersShapes, *outLayersShapes);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Net_getLayersShapes_const_const_MatShapeR_vector_int_R_vector_vector_MatShape__R_vector_vector_MatShape__R(const cv::dnn::Net* instance, const cv::dnn::MatShape* netInputShape, std::vector<int>* layersIds, std::vector<std::vector<cv::dnn::MatShape>>* inLayersShapes, std::vector<std::vector<cv::dnn::MatShape>>* outLayersShapes) {
		try {
			instance->getLayersShapes(*netInputShape, *layersIds, *inLayersShapes, *outLayersShapes);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Net_getLayerShapes_const_const_MatShapeR_const_int_vector_MatShape_R_vector_MatShape_R(const cv::dnn::Net* instance, const cv::dnn::MatShape* netInputShape, const int layerId, std::vector<cv::dnn::MatShape>* inLayerShapes, std::vector<cv::dnn::MatShape>* outLayerShapes) {
		try {
			instance->getLayerShapes(*netInputShape, layerId, *inLayerShapes, *outLayerShapes);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Net_getLayerShapes_const_const_vector_MatShape_R_const_int_vector_MatShape_R_vector_MatShape_R(const cv::dnn::Net* instance, const std::vector<cv::dnn::MatShape>* netInputShapes, const int layerId, std::vector<cv::dnn::MatShape>* inLayerShapes, std::vector<cv::dnn::MatShape>* outLayerShapes) {
		try {
			instance->getLayerShapes(*netInputShapes, layerId, *inLayerShapes, *outLayerShapes);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int64_t> cv_dnn_Net_getFLOPS_const_const_vector_MatShape_R(const cv::dnn::Net* instance, const std::vector<cv::dnn::MatShape>* netInputShapes) {
		try {
			int64_t ret = instance->getFLOPS(*netInputShapes);
			return Ok<int64_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int64_t>))
	}
	
	Result<int64_t> cv_dnn_Net_getFLOPS_const_const_MatShapeR(const cv::dnn::Net* instance, const cv::dnn::MatShape* netInputShape) {
		try {
			int64_t ret = instance->getFLOPS(*netInputShape);
			return Ok<int64_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int64_t>))
	}
	
	Result<int64_t> cv_dnn_Net_getFLOPS_const_const_int_const_vector_MatShape_R(const cv::dnn::Net* instance, const int layerId, const std::vector<cv::dnn::MatShape>* netInputShapes) {
		try {
			int64_t ret = instance->getFLOPS(layerId, *netInputShapes);
			return Ok<int64_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int64_t>))
	}
	
	Result<int64_t> cv_dnn_Net_getFLOPS_const_const_int_const_MatShapeR(const cv::dnn::Net* instance, const int layerId, const cv::dnn::MatShape* netInputShape) {
		try {
			int64_t ret = instance->getFLOPS(layerId, *netInputShape);
			return Ok<int64_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int64_t>))
	}
	
	Result_void cv_dnn_Net_getLayerTypes_const_vector_String_R(const cv::dnn::Net* instance, std::vector<cv::String>* layersTypes) {
		try {
			instance->getLayerTypes(*layersTypes);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_dnn_Net_getLayersCount_const_const_StringR(const cv::dnn::Net* instance, const char* layerType) {
		try {
			int ret = instance->getLayersCount(cv::String(layerType));
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_dnn_Net_getMemoryConsumption_const_const_vector_MatShape_R_size_tR_size_tR(const cv::dnn::Net* instance, const std::vector<cv::dnn::MatShape>* netInputShapes, size_t* weights, size_t* blobs) {
		try {
			instance->getMemoryConsumption(*netInputShapes, *weights, *blobs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Net_getMemoryConsumption_const_const_MatShapeR_size_tR_size_tR(const cv::dnn::Net* instance, const cv::dnn::MatShape* netInputShape, size_t* weights, size_t* blobs) {
		try {
			instance->getMemoryConsumption(*netInputShape, *weights, *blobs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Net_getMemoryConsumption_const_const_int_const_vector_MatShape_R_size_tR_size_tR(const cv::dnn::Net* instance, const int layerId, const std::vector<cv::dnn::MatShape>* netInputShapes, size_t* weights, size_t* blobs) {
		try {
			instance->getMemoryConsumption(layerId, *netInputShapes, *weights, *blobs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Net_getMemoryConsumption_const_const_int_const_MatShapeR_size_tR_size_tR(const cv::dnn::Net* instance, const int layerId, const cv::dnn::MatShape* netInputShape, size_t* weights, size_t* blobs) {
		try {
			instance->getMemoryConsumption(layerId, *netInputShape, *weights, *blobs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Net_getMemoryConsumption_const_const_vector_MatShape_R_vector_int_R_vector_size_t_R_vector_size_t_R(const cv::dnn::Net* instance, const std::vector<cv::dnn::MatShape>* netInputShapes, std::vector<int>* layerIds, std::vector<size_t>* weights, std::vector<size_t>* blobs) {
		try {
			instance->getMemoryConsumption(*netInputShapes, *layerIds, *weights, *blobs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Net_getMemoryConsumption_const_const_MatShapeR_vector_int_R_vector_size_t_R_vector_size_t_R(const cv::dnn::Net* instance, const cv::dnn::MatShape* netInputShape, std::vector<int>* layerIds, std::vector<size_t>* weights, std::vector<size_t>* blobs) {
		try {
			instance->getMemoryConsumption(*netInputShape, *layerIds, *weights, *blobs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_Net_enableFusion_bool(cv::dnn::Net* instance, bool fusion) {
		try {
			instance->enableFusion(fusion);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int64_t> cv_dnn_Net_getPerfProfile_vector_double_R(cv::dnn::Net* instance, std::vector<double>* timings) {
		try {
			int64_t ret = instance->getPerfProfile(*timings);
			return Ok<int64_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int64_t>))
	}
	
	Result<float> cv_dnn_NormalizeBBoxLayer_getPropPnorm_const(const cv::dnn::NormalizeBBoxLayer* instance) {
		try {
			float ret = instance->pnorm;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_dnn_NormalizeBBoxLayer_setPropPnorm_float(cv::dnn::NormalizeBBoxLayer* instance, float val) {
		try {
			instance->pnorm = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_dnn_NormalizeBBoxLayer_getPropEpsilon_const(const cv::dnn::NormalizeBBoxLayer* instance) {
		try {
			float ret = instance->epsilon;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_dnn_NormalizeBBoxLayer_setPropEpsilon_float(cv::dnn::NormalizeBBoxLayer* instance, float val) {
		try {
			instance->epsilon = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_dnn_NormalizeBBoxLayer_getPropAcrossSpatial_const(const cv::dnn::NormalizeBBoxLayer* instance) {
		try {
			bool ret = instance->acrossSpatial;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_dnn_NormalizeBBoxLayer_setPropAcrossSpatial_bool(cv::dnn::NormalizeBBoxLayer* instance, bool val) {
		try {
			instance->acrossSpatial = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_NormalizeBBoxLayer_delete(cv::dnn::NormalizeBBoxLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::NormalizeBBoxLayer>*> cv_dnn_NormalizeBBoxLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::NormalizeBBoxLayer> ret = cv::dnn::NormalizeBBoxLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::NormalizeBBoxLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::NormalizeBBoxLayer>*>))
	}
	
	void cv_PaddingLayer_delete(cv::dnn::PaddingLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::PaddingLayer>*> cv_dnn_PaddingLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::PaddingLayer> ret = cv::dnn::PaddingLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::PaddingLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::PaddingLayer>*>))
	}
	
	void cv_PermuteLayer_delete(cv::dnn::PermuteLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::PermuteLayer>*> cv_dnn_PermuteLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::PermuteLayer> ret = cv::dnn::PermuteLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::PermuteLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::PermuteLayer>*>))
	}
	
	Result<int> cv_dnn_PoolingLayer_getPropType_const(const cv::dnn::PoolingLayer* instance) {
		try {
			int ret = instance->type;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_dnn_PoolingLayer_setPropType_int(cv::dnn::PoolingLayer* instance, int val) {
		try {
			instance->type = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<size_t>*> cv_dnn_PoolingLayer_getPropKernel_size(cv::dnn::PoolingLayer* instance) {
		try {
			std::vector<size_t> ret = instance->kernel_size;
			return Ok(new std::vector<size_t>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<size_t>*>))
	}
	
	Result_void cv_dnn_PoolingLayer_setPropKernel_size_vector_size_t_(cv::dnn::PoolingLayer* instance, std::vector<size_t>* val) {
		try {
			instance->kernel_size = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<size_t>*> cv_dnn_PoolingLayer_getPropStrides(cv::dnn::PoolingLayer* instance) {
		try {
			std::vector<size_t> ret = instance->strides;
			return Ok(new std::vector<size_t>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<size_t>*>))
	}
	
	Result_void cv_dnn_PoolingLayer_setPropStrides_vector_size_t_(cv::dnn::PoolingLayer* instance, std::vector<size_t>* val) {
		try {
			instance->strides = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<size_t>*> cv_dnn_PoolingLayer_getPropPads_begin(cv::dnn::PoolingLayer* instance) {
		try {
			std::vector<size_t> ret = instance->pads_begin;
			return Ok(new std::vector<size_t>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<size_t>*>))
	}
	
	Result_void cv_dnn_PoolingLayer_setPropPads_begin_vector_size_t_(cv::dnn::PoolingLayer* instance, std::vector<size_t>* val) {
		try {
			instance->pads_begin = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<size_t>*> cv_dnn_PoolingLayer_getPropPads_end(cv::dnn::PoolingLayer* instance) {
		try {
			std::vector<size_t> ret = instance->pads_end;
			return Ok(new std::vector<size_t>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<size_t>*>))
	}
	
	Result_void cv_dnn_PoolingLayer_setPropPads_end_vector_size_t_(cv::dnn::PoolingLayer* instance, std::vector<size_t>* val) {
		try {
			instance->pads_end = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_dnn_PoolingLayer_getPropGlobalPooling_const(const cv::dnn::PoolingLayer* instance) {
		try {
			bool ret = instance->globalPooling;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_dnn_PoolingLayer_setPropGlobalPooling_bool(cv::dnn::PoolingLayer* instance, bool val) {
		try {
			instance->globalPooling = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<bool>*> cv_dnn_PoolingLayer_getPropIsGlobalPooling(cv::dnn::PoolingLayer* instance) {
		try {
			std::vector<bool> ret = instance->isGlobalPooling;
			return Ok(new std::vector<bool>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<bool>*>))
	}
	
	Result_void cv_dnn_PoolingLayer_setPropIsGlobalPooling_vector_bool_(cv::dnn::PoolingLayer* instance, std::vector<bool>* val) {
		try {
			instance->isGlobalPooling = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_dnn_PoolingLayer_getPropComputeMaxIdx_const(const cv::dnn::PoolingLayer* instance) {
		try {
			bool ret = instance->computeMaxIdx;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_dnn_PoolingLayer_setPropComputeMaxIdx_bool(cv::dnn::PoolingLayer* instance, bool val) {
		try {
			instance->computeMaxIdx = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_dnn_PoolingLayer_getPropPadMode_const(const cv::dnn::PoolingLayer* instance) {
		try {
			cv::String ret = instance->padMode;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_dnn_PoolingLayer_setPropPadMode_String(cv::dnn::PoolingLayer* instance, char* val) {
		try {
			instance->padMode = cv::String(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_dnn_PoolingLayer_getPropCeilMode_const(const cv::dnn::PoolingLayer* instance) {
		try {
			bool ret = instance->ceilMode;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_dnn_PoolingLayer_setPropCeilMode_bool(cv::dnn::PoolingLayer* instance, bool val) {
		try {
			instance->ceilMode = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_dnn_PoolingLayer_getPropAvePoolPaddedArea_const(const cv::dnn::PoolingLayer* instance) {
		try {
			bool ret = instance->avePoolPaddedArea;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_dnn_PoolingLayer_setPropAvePoolPaddedArea_bool(cv::dnn::PoolingLayer* instance, bool val) {
		try {
			instance->avePoolPaddedArea = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_dnn_PoolingLayer_getPropPooledSize_const(const cv::dnn::PoolingLayer* instance) {
		try {
			cv::Size ret = instance->pooledSize;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_dnn_PoolingLayer_setPropPooledSize_Size(cv::dnn::PoolingLayer* instance, cv::Size* val) {
		try {
			instance->pooledSize = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_dnn_PoolingLayer_getPropSpatialScale_const(const cv::dnn::PoolingLayer* instance) {
		try {
			float ret = instance->spatialScale;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_dnn_PoolingLayer_setPropSpatialScale_float(cv::dnn::PoolingLayer* instance, float val) {
		try {
			instance->spatialScale = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_dnn_PoolingLayer_getPropPsRoiOutChannels_const(const cv::dnn::PoolingLayer* instance) {
		try {
			int ret = instance->psRoiOutChannels;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_dnn_PoolingLayer_setPropPsRoiOutChannels_int(cv::dnn::PoolingLayer* instance, int val) {
		try {
			instance->psRoiOutChannels = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_PoolingLayer_delete(cv::dnn::PoolingLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::PoolingLayer>*> cv_dnn_PoolingLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::PoolingLayer> ret = cv::dnn::PoolingLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::PoolingLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::PoolingLayer>*>))
	}
	
	Result<float> cv_dnn_PowerLayer_getPropPower_const(const cv::dnn::PowerLayer* instance) {
		try {
			float ret = instance->power;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_dnn_PowerLayer_setPropPower_float(cv::dnn::PowerLayer* instance, float val) {
		try {
			instance->power = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_dnn_PowerLayer_getPropScale_const(const cv::dnn::PowerLayer* instance) {
		try {
			float ret = instance->scale;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_dnn_PowerLayer_setPropScale_float(cv::dnn::PowerLayer* instance, float val) {
		try {
			instance->scale = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_dnn_PowerLayer_getPropShift_const(const cv::dnn::PowerLayer* instance) {
		try {
			float ret = instance->shift;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_dnn_PowerLayer_setPropShift_float(cv::dnn::PowerLayer* instance, float val) {
		try {
			instance->shift = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::dnn::PowerLayer>*> cv_dnn_PowerLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::PowerLayer> ret = cv::dnn::PowerLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::PowerLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::PowerLayer>*>))
	}
	
	void cv_PriorBoxLayer_delete(cv::dnn::PriorBoxLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::PriorBoxLayer>*> cv_dnn_PriorBoxLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::PriorBoxLayer> ret = cv::dnn::PriorBoxLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::PriorBoxLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::PriorBoxLayer>*>))
	}
	
	void cv_ProposalLayer_delete(cv::dnn::ProposalLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::ProposalLayer>*> cv_dnn_ProposalLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::ProposalLayer> ret = cv::dnn::ProposalLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::ProposalLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::ProposalLayer>*>))
	}
	
	Result<cv::Ptr<cv::dnn::RNNLayer>*> cv_dnn_RNNLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::RNNLayer> ret = cv::dnn::RNNLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::RNNLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::RNNLayer>*>))
	}
	
	Result_void cv_dnn_RNNLayer_setWeights_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR(cv::dnn::RNNLayer* instance, const cv::Mat* Wxh, const cv::Mat* bh, const cv::Mat* Whh, const cv::Mat* Who, const cv::Mat* bo) {
		try {
			instance->setWeights(*Wxh, *bh, *Whh, *Who, *bo);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dnn_RNNLayer_setProduceHiddenOutput_bool(cv::dnn::RNNLayer* instance, bool produce) {
		try {
			instance->setProduceHiddenOutput(produce);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_dnn_ReLU6Layer_getPropMinValue_const(const cv::dnn::ReLU6Layer* instance) {
		try {
			float ret = instance->minValue;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_dnn_ReLU6Layer_setPropMinValue_float(cv::dnn::ReLU6Layer* instance, float val) {
		try {
			instance->minValue = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_dnn_ReLU6Layer_getPropMaxValue_const(const cv::dnn::ReLU6Layer* instance) {
		try {
			float ret = instance->maxValue;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_dnn_ReLU6Layer_setPropMaxValue_float(cv::dnn::ReLU6Layer* instance, float val) {
		try {
			instance->maxValue = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::dnn::ReLU6Layer>*> cv_dnn_ReLU6Layer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::ReLU6Layer> ret = cv::dnn::ReLU6Layer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::ReLU6Layer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::ReLU6Layer>*>))
	}
	
	Result<float> cv_dnn_ReLULayer_getPropNegativeSlope_const(const cv::dnn::ReLULayer* instance) {
		try {
			float ret = instance->negativeSlope;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_dnn_ReLULayer_setPropNegativeSlope_float(cv::dnn::ReLULayer* instance, float val) {
		try {
			instance->negativeSlope = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::dnn::ReLULayer>*> cv_dnn_ReLULayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::ReLULayer> ret = cv::dnn::ReLULayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::ReLULayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::ReLULayer>*>))
	}
	
	void cv_RegionLayer_delete(cv::dnn::RegionLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::RegionLayer>*> cv_dnn_RegionLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::RegionLayer> ret = cv::dnn::RegionLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::RegionLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::RegionLayer>*>))
	}
	
	void cv_ReorgLayer_delete(cv::dnn::ReorgLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::ReorgLayer>*> cv_dnn_ReorgLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::ReorgLayer> ret = cv::dnn::ReorgLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::ReorgLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::ReorgLayer>*>))
	}
	
	Result<cv::dnn::MatShape*> cv_dnn_ReshapeLayer_getPropNewShapeDesc(cv::dnn::ReshapeLayer* instance) {
		try {
			cv::dnn::MatShape ret = instance->newShapeDesc;
			return Ok(new cv::dnn::MatShape(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::MatShape*>))
	}
	
	Result_void cv_dnn_ReshapeLayer_setPropNewShapeDesc_MatShape(cv::dnn::ReshapeLayer* instance, cv::dnn::MatShape* val) {
		try {
			instance->newShapeDesc = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Range*> cv_dnn_ReshapeLayer_getPropNewShapeRange(cv::dnn::ReshapeLayer* instance) {
		try {
			cv::Range ret = instance->newShapeRange;
			return Ok(new cv::Range(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Range*>))
	}
	
	Result_void cv_dnn_ReshapeLayer_setPropNewShapeRange_Range(cv::dnn::ReshapeLayer* instance, cv::Range* val) {
		try {
			instance->newShapeRange = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_ReshapeLayer_delete(cv::dnn::ReshapeLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::ReshapeLayer>*> cv_dnn_ReshapeLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::ReshapeLayer> ret = cv::dnn::ReshapeLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::ReshapeLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::ReshapeLayer>*>))
	}
	
	void cv_ResizeLayer_delete(cv::dnn::ResizeLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::ResizeLayer>*> cv_dnn_ResizeLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::ResizeLayer> ret = cv::dnn::ResizeLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::ResizeLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::ResizeLayer>*>))
	}
	
	Result<bool> cv_dnn_ScaleLayer_getPropHasBias_const(const cv::dnn::ScaleLayer* instance) {
		try {
			bool ret = instance->hasBias;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_dnn_ScaleLayer_setPropHasBias_bool(cv::dnn::ScaleLayer* instance, bool val) {
		try {
			instance->hasBias = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_dnn_ScaleLayer_getPropAxis_const(const cv::dnn::ScaleLayer* instance) {
		try {
			int ret = instance->axis;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_dnn_ScaleLayer_setPropAxis_int(cv::dnn::ScaleLayer* instance, int val) {
		try {
			instance->axis = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_ScaleLayer_delete(cv::dnn::ScaleLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::ScaleLayer>*> cv_dnn_ScaleLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::ScaleLayer> ret = cv::dnn::ScaleLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::ScaleLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::ScaleLayer>*>))
	}
	
	void cv_ShiftLayer_delete(cv::dnn::ShiftLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::Layer>*> cv_dnn_ShiftLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::ShiftLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::Layer>*>))
	}
	
	Result<int> cv_dnn_ShuffleChannelLayer_getPropGroup_const(const cv::dnn::ShuffleChannelLayer* instance) {
		try {
			int ret = instance->group;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_dnn_ShuffleChannelLayer_setPropGroup_int(cv::dnn::ShuffleChannelLayer* instance, int val) {
		try {
			instance->group = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_ShuffleChannelLayer_delete(cv::dnn::ShuffleChannelLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::Layer>*> cv_dnn_ShuffleChannelLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::ShuffleChannelLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::Layer>*>))
	}
	
	Result<cv::Ptr<cv::dnn::SigmoidLayer>*> cv_dnn_SigmoidLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::SigmoidLayer> ret = cv::dnn::SigmoidLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::SigmoidLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::SigmoidLayer>*>))
	}
	
	Result<std::vector<std::vector<cv::Range>>*> cv_dnn_SliceLayer_getPropSliceRanges(cv::dnn::SliceLayer* instance) {
		try {
			std::vector<std::vector<cv::Range>> ret = instance->sliceRanges;
			return Ok(new std::vector<std::vector<cv::Range>>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<std::vector<cv::Range>>*>))
	}
	
	Result_void cv_dnn_SliceLayer_setPropSliceRanges_vector_vector_Range__(cv::dnn::SliceLayer* instance, std::vector<std::vector<cv::Range>>* val) {
		try {
			instance->sliceRanges = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_dnn_SliceLayer_getPropAxis_const(const cv::dnn::SliceLayer* instance) {
		try {
			int ret = instance->axis;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_dnn_SliceLayer_setPropAxis_int(cv::dnn::SliceLayer* instance, int val) {
		try {
			instance->axis = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_dnn_SliceLayer_getPropNum_split_const(const cv::dnn::SliceLayer* instance) {
		try {
			int ret = instance->num_split;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_dnn_SliceLayer_setPropNum_split_int(cv::dnn::SliceLayer* instance, int val) {
		try {
			instance->num_split = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_SliceLayer_delete(cv::dnn::SliceLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::SliceLayer>*> cv_dnn_SliceLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::SliceLayer> ret = cv::dnn::SliceLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::SliceLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::SliceLayer>*>))
	}
	
	Result<bool> cv_dnn_SoftmaxLayer_getPropLogSoftMax_const(const cv::dnn::SoftmaxLayer* instance) {
		try {
			bool ret = instance->logSoftMax;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_dnn_SoftmaxLayer_setPropLogSoftMax_bool(cv::dnn::SoftmaxLayer* instance, bool val) {
		try {
			instance->logSoftMax = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_SoftmaxLayer_delete(cv::dnn::SoftmaxLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::SoftmaxLayer>*> cv_dnn_SoftmaxLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::SoftmaxLayer> ret = cv::dnn::SoftmaxLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::SoftmaxLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::SoftmaxLayer>*>))
	}
	
	Result<int> cv_dnn_SplitLayer_getPropOutputsCount_const(const cv::dnn::SplitLayer* instance) {
		try {
			int ret = instance->outputsCount;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_dnn_SplitLayer_setPropOutputsCount_int(cv::dnn::SplitLayer* instance, int val) {
		try {
			instance->outputsCount = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_SplitLayer_delete(cv::dnn::SplitLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::SplitLayer>*> cv_dnn_SplitLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::SplitLayer> ret = cv::dnn::SplitLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::SplitLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::SplitLayer>*>))
	}
	
	Result<cv::Ptr<cv::dnn::SwishLayer>*> cv_dnn_SwishLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::SwishLayer> ret = cv::dnn::SwishLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::SwishLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::SwishLayer>*>))
	}
	
	Result<cv::Ptr<cv::dnn::TanHLayer>*> cv_dnn_TanHLayer_create_const_LayerParamsR(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::TanHLayer> ret = cv::dnn::TanHLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::TanHLayer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dnn::TanHLayer>*>))
	}
	
	void cv__Range_delete(cv::dnn::_Range* instance) {
		delete instance;
	}
	Result<cv::dnn::_Range*> cv_dnn__Range__Range_const_RangeR(const cv::Range* r) {
		try {
			cv::dnn::_Range* ret = new cv::dnn::_Range(*r);
			return Ok<cv::dnn::_Range*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::_Range*>))
	}
	
	Result<cv::dnn::_Range*> cv_dnn__Range__Range_int_int(int start_, int size_) {
		try {
			cv::dnn::_Range* ret = new cv::dnn::_Range(start_, size_);
			return Ok<cv::dnn::_Range*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dnn::_Range*>))
	}
	
}
