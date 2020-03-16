#include "dnn.hpp"
#include "dnn_types.hpp"

extern "C" {
	Result_void cv_dnn_NMSBoxes_const_vector_Rect2d_X_const_vector_float_X_float_float_vector_int_X_float_int(const std::vector<cv::Rect2d>* bboxes, const std::vector<float>* scores, float score_threshold, float nms_threshold, std::vector<int>* indices, float eta, int top_k) {
		try {
			cv::dnn::NMSBoxes(*bboxes, *scores, score_threshold, nms_threshold, *indices, eta, top_k);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_NMSBoxes_const_vector_Rect_X_const_vector_float_X_float_float_vector_int_X_float_int(const std::vector<cv::Rect>* bboxes, const std::vector<float>* scores, float score_threshold, float nms_threshold, std::vector<int>* indices, float eta, int top_k) {
		try {
			cv::dnn::NMSBoxes(*bboxes, *scores, score_threshold, nms_threshold, *indices, eta, top_k);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_NMSBoxes_const_vector_RotatedRect_X_const_vector_float_X_float_float_vector_int_X_float_int(const std::vector<cv::RotatedRect>* bboxes, const std::vector<float>* scores, float score_threshold, float nms_threshold, std::vector<int>* indices, float eta, int top_k) {
		try {
			cv::dnn::NMSBoxes(*bboxes, *scores, score_threshold, nms_threshold, *indices, eta, top_k);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_blobFromImage_const__InputArrayX_const__OutputArrayX_double_const_SizeX_const_ScalarX_bool_bool_int(const cv::_InputArray* image, const cv::_OutputArray* blob, double scalefactor, const cv::Size* size, const cv::Scalar* mean, bool swapRB, bool crop, int ddepth) {
		try {
			cv::dnn::blobFromImage(*image, *blob, scalefactor, *size, *mean, swapRB, crop, ddepth);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_dnn_blobFromImage_const__InputArrayX_double_const_SizeX_const_ScalarX_bool_bool_int(const cv::_InputArray* image, double scalefactor, const cv::Size* size, const cv::Scalar* mean, bool swapRB, bool crop, int ddepth) {
		try {
			cv::Mat ret = cv::dnn::blobFromImage(*image, scalefactor, *size, *mean, swapRB, crop, ddepth);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result_void cv_dnn_blobFromImages_const__InputArrayX_const__OutputArrayX_double_Size_const_ScalarX_bool_bool_int(const cv::_InputArray* images, const cv::_OutputArray* blob, double scalefactor, const cv::Size* size, const cv::Scalar* mean, bool swapRB, bool crop, int ddepth) {
		try {
			cv::dnn::blobFromImages(*images, *blob, scalefactor, *size, *mean, swapRB, crop, ddepth);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_dnn_blobFromImages_const__InputArrayX_double_Size_const_ScalarX_bool_bool_int(const cv::_InputArray* images, double scalefactor, const cv::Size* size, const cv::Scalar* mean, bool swapRB, bool crop, int ddepth) {
		try {
			cv::Mat ret = cv::dnn::blobFromImages(*images, scalefactor, *size, *mean, swapRB, crop, ddepth);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Range*> cv_dnn_clamp_const_RangeX_int(const cv::Range* r, int axisSize) {
		try {
			cv::Range ret = cv::dnn::clamp(*r, axisSize);
			return Ok(new cv::Range(ret));
		} OCVRS_CATCH(Result<cv::Range*>)
	}
	
	Result<int> cv_dnn_clamp_int_const_MatShapeX(int ax, const cv::dnn::MatShape* shape) {
		try {
			int ret = cv::dnn::clamp(ax, *shape);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_dnn_clamp_int_int(int ax, int dims) {
		try {
			int ret = cv::dnn::clamp(ax, dims);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<cv::dnn::MatShape*> cv_dnn_concat_const_MatShapeX_const_MatShapeX(const cv::dnn::MatShape* a, const cv::dnn::MatShape* b) {
		try {
			cv::dnn::MatShape ret = cv::dnn::concat(*a, *b);
			return Ok(new cv::dnn::MatShape(ret));
		} OCVRS_CATCH(Result<cv::dnn::MatShape*>)
	}
	
	Result<std::vector<cv::dnn::Target>*> cv_dnn_getAvailableTargets_Backend(cv::dnn::Backend be) {
		try {
			std::vector<cv::dnn::Target> ret = cv::dnn::getAvailableTargets(be);
			return Ok(new std::vector<cv::dnn::Target>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::dnn::Target>*>)
	}
	
	Result<void*> cv_dnn_getInferenceEngineBackendType() {
		try {
			cv::String ret = cv::dnn::getInferenceEngineBackendType();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_getInferenceEngineVPUType() {
		try {
			cv::String ret = cv::dnn::getInferenceEngineVPUType();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::Mat*> cv_dnn_getPlane_const_MatX_int_int(const cv::Mat* m, int n, int cn) {
		try {
			cv::Mat ret = cv::dnn::getPlane(*m, n, cn);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result_void cv_dnn_imagesFromBlob_const_MatX_const__OutputArrayX(const cv::Mat* blob_, const cv::_OutputArray* images_) {
		try {
			cv::dnn::imagesFromBlob(*blob_, *images_);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_print_const_MatShapeX_const_StringX(const cv::dnn::MatShape* shape, const char* name) {
		try {
			cv::dnn::print(*shape, std::string(name));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromCaffe_const_StringX_const_StringX(const char* prototxt, const char* caffeModel) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromCaffe(std::string(prototxt), std::string(caffeModel));
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<cv::dnn::Net*>)
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromCaffe_const_charX_size_t_const_charX_size_t(const char* bufferProto, size_t lenProto, const char* bufferModel, size_t lenModel) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromCaffe(bufferProto, lenProto, bufferModel, lenModel);
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<cv::dnn::Net*>)
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromCaffe_const_vector_unsigned_char_X_const_vector_unsigned_char_X(const std::vector<unsigned char>* bufferProto, const std::vector<unsigned char>* bufferModel) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromCaffe(*bufferProto, *bufferModel);
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<cv::dnn::Net*>)
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromDarknet_const_StringX_const_StringX(const char* cfgFile, const char* darknetModel) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromDarknet(std::string(cfgFile), std::string(darknetModel));
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<cv::dnn::Net*>)
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromDarknet_const_charX_size_t_const_charX_size_t(const char* bufferCfg, size_t lenCfg, const char* bufferModel, size_t lenModel) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromDarknet(bufferCfg, lenCfg, bufferModel, lenModel);
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<cv::dnn::Net*>)
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromDarknet_const_vector_unsigned_char_X_const_vector_unsigned_char_X(const std::vector<unsigned char>* bufferCfg, const std::vector<unsigned char>* bufferModel) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromDarknet(*bufferCfg, *bufferModel);
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<cv::dnn::Net*>)
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromModelOptimizer_const_StringX_const_StringX(const char* xml, const char* bin) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromModelOptimizer(std::string(xml), std::string(bin));
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<cv::dnn::Net*>)
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromModelOptimizer_const_unsigned_charX_size_t_const_unsigned_charX_size_t(const unsigned char* bufferModelConfigPtr, size_t bufferModelConfigSize, const unsigned char* bufferWeightsPtr, size_t bufferWeightsSize) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromModelOptimizer(bufferModelConfigPtr, bufferModelConfigSize, bufferWeightsPtr, bufferWeightsSize);
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<cv::dnn::Net*>)
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromModelOptimizer_const_vector_unsigned_char_X_const_vector_unsigned_char_X(const std::vector<unsigned char>* bufferModelConfig, const std::vector<unsigned char>* bufferWeights) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromModelOptimizer(*bufferModelConfig, *bufferWeights);
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<cv::dnn::Net*>)
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromONNX_const_StringX(const char* onnxFile) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromONNX(std::string(onnxFile));
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<cv::dnn::Net*>)
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromONNX_const_charX_size_t(const char* buffer, size_t sizeBuffer) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromONNX(buffer, sizeBuffer);
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<cv::dnn::Net*>)
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromONNX_const_vector_unsigned_char_X(const std::vector<unsigned char>* buffer) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromONNX(*buffer);
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<cv::dnn::Net*>)
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromTensorflow_const_StringX_const_StringX(const char* model, const char* config) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromTensorflow(std::string(model), std::string(config));
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<cv::dnn::Net*>)
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromTensorflow_const_charX_size_t_const_charX_size_t(const char* bufferModel, size_t lenModel, const char* bufferConfig, size_t lenConfig) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromTensorflow(bufferModel, lenModel, bufferConfig, lenConfig);
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<cv::dnn::Net*>)
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromTensorflow_const_vector_unsigned_char_X_const_vector_unsigned_char_X(const std::vector<unsigned char>* bufferModel, const std::vector<unsigned char>* bufferConfig) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromTensorflow(*bufferModel, *bufferConfig);
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<cv::dnn::Net*>)
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNetFromTorch_const_StringX_bool_bool(const char* model, bool isBinary, bool evaluate) {
		try {
			cv::dnn::Net ret = cv::dnn::readNetFromTorch(std::string(model), isBinary, evaluate);
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<cv::dnn::Net*>)
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNet_const_StringX_const_StringX_const_StringX(const char* model, const char* config, const char* framework) {
		try {
			cv::dnn::Net ret = cv::dnn::readNet(std::string(model), std::string(config), std::string(framework));
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<cv::dnn::Net*>)
	}
	
	Result<cv::dnn::Net*> cv_dnn_readNet_const_StringX_const_vector_unsigned_char_X_const_vector_unsigned_char_X(const char* framework, const std::vector<unsigned char>* bufferModel, const std::vector<unsigned char>* bufferConfig) {
		try {
			cv::dnn::Net ret = cv::dnn::readNet(std::string(framework), *bufferModel, *bufferConfig);
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<cv::dnn::Net*>)
	}
	
	Result<cv::Mat*> cv_dnn_readTensorFromONNX_const_StringX(const char* path) {
		try {
			cv::Mat ret = cv::dnn::readTensorFromONNX(std::string(path));
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_dnn_readTorchBlob_const_StringX_bool(const char* filename, bool isBinary) {
		try {
			cv::Mat ret = cv::dnn::readTorchBlob(std::string(filename), isBinary);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
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
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::dnn::MatShape*> cv_dnn_shape_const_MatSizeX(const cv::MatSize* sz) {
		try {
			cv::dnn::MatShape ret = cv::dnn::shape(*sz);
			return Ok(new cv::dnn::MatShape(ret));
		} OCVRS_CATCH(Result<cv::dnn::MatShape*>)
	}
	
	Result<cv::dnn::MatShape*> cv_dnn_shape_const_MatX(const cv::Mat* mat) {
		try {
			cv::dnn::MatShape ret = cv::dnn::shape(*mat);
			return Ok(new cv::dnn::MatShape(ret));
		} OCVRS_CATCH(Result<cv::dnn::MatShape*>)
	}
	
	Result<cv::dnn::MatShape*> cv_dnn_shape_const_UMatX(const cv::UMat* mat) {
		try {
			cv::dnn::MatShape ret = cv::dnn::shape(*mat);
			return Ok(new cv::dnn::MatShape(ret));
		} OCVRS_CATCH(Result<cv::dnn::MatShape*>)
	}
	
	Result<cv::dnn::MatShape*> cv_dnn_shape_const_intX_int(const int* dims, int n) {
		try {
			cv::dnn::MatShape ret = cv::dnn::shape(dims, n);
			return Ok(new cv::dnn::MatShape(ret));
		} OCVRS_CATCH(Result<cv::dnn::MatShape*>)
	}
	
	Result<cv::dnn::MatShape*> cv_dnn_shape_int_int_int_int(int a0, int a1, int a2, int a3) {
		try {
			cv::dnn::MatShape ret = cv::dnn::shape(a0, a1, a2, a3);
			return Ok(new cv::dnn::MatShape(ret));
		} OCVRS_CATCH(Result<cv::dnn::MatShape*>)
	}
	
	Result_void cv_dnn_shrinkCaffeModel_const_StringX_const_StringX_const_vector_String_X(const char* src, const char* dst, const std::vector<cv::String>* layersTypes) {
		try {
			cv::dnn::shrinkCaffeModel(std::string(src), std::string(dst), *layersTypes);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_dnn_slice_const_MatX_const__RangeX(const cv::Mat* m, const cv::dnn::_Range* r0) {
		try {
			cv::Mat ret = cv::dnn::slice(*m, *r0);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_dnn_slice_const_MatX_const__RangeX_const__RangeX(const cv::Mat* m, const cv::dnn::_Range* r0, const cv::dnn::_Range* r1) {
		try {
			cv::Mat ret = cv::dnn::slice(*m, *r0, *r1);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_dnn_slice_const_MatX_const__RangeX_const__RangeX_const__RangeX(const cv::Mat* m, const cv::dnn::_Range* r0, const cv::dnn::_Range* r1, const cv::dnn::_Range* r2) {
		try {
			cv::Mat ret = cv::dnn::slice(*m, *r0, *r1, *r2);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_dnn_slice_const_MatX_const__RangeX_const__RangeX_const__RangeX_const__RangeX(const cv::Mat* m, const cv::dnn::_Range* r0, const cv::dnn::_Range* r1, const cv::dnn::_Range* r2, const cv::dnn::_Range* r3) {
		try {
			cv::Mat ret = cv::dnn::slice(*m, *r0, *r1, *r2, *r3);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<void*> cv_dnn_toString_const_MatShapeX_const_StringX(const cv::dnn::MatShape* shape, const char* name) {
		try {
			std::string ret = cv::dnn::toString(*shape, std::string(name));
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_dnn_total_const_MatShapeX_int_int(const cv::dnn::MatShape* shape, int start, int end) {
		try {
			int ret = cv::dnn::total(*shape, start, end);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_writeTextGraph_const_StringX_const_StringX(const char* model, const char* output) {
		try {
			cv::dnn::writeTextGraph(std::string(model), std::string(output));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::dnn::AbsLayer>*> cv_dnn_AbsLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::AbsLayer> ret = cv::dnn::AbsLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::AbsLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::AbsLayer>*>)
	}
	
	Result_void cv_dnn_ActivationLayer_forwardSlice_const_const_floatX_floatX_int_size_t_int_int(const cv::dnn::ActivationLayer* instance, const float* src, float* dst, int len, size_t outPlaneSize, int cn0, int cn1) {
		try {
			instance->forwardSlice(src, dst, len, outPlaneSize, cn0, cn1);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::dnn::BNLLLayer>*> cv_dnn_BNLLLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::BNLLLayer> ret = cv::dnn::BNLLLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::BNLLLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::BNLLLayer>*>)
	}
	
	Result<int> cv_dnn_BackendNode_backendId_const(const cv::dnn::BackendNode* instance) {
		try {
			int ret = instance->backendId;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_BackendNode_setBackendId_int(cv::dnn::BackendNode* instance, int val) {
		try {
			instance->backendId = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_BackendNode_delete(cv::dnn::BackendNode* instance) {
		delete instance;
	}
	Result<cv::dnn::BackendNode*> cv_dnn_BackendNode_BackendNode_int(int backendId) {
		try {
			cv::dnn::BackendNode* ret = new cv::dnn::BackendNode(backendId);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::dnn::BackendNode*>)
	}
	
	Result<int> cv_dnn_BackendWrapper_backendId_const(const cv::dnn::BackendWrapper* instance) {
		try {
			int ret = instance->backendId;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_BackendWrapper_setBackendId_int(cv::dnn::BackendWrapper* instance, int val) {
		try {
			instance->backendId = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_BackendWrapper_targetId_const(const cv::dnn::BackendWrapper* instance) {
		try {
			int ret = instance->targetId;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_BackendWrapper_setTargetId_int(cv::dnn::BackendWrapper* instance, int val) {
		try {
			instance->targetId = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_BackendWrapper_copyToHost(cv::dnn::BackendWrapper* instance) {
		try {
			instance->copyToHost();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_BackendWrapper_setHostDirty(cv::dnn::BackendWrapper* instance) {
		try {
			instance->setHostDirty();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_dnn_BaseConvolutionLayer_kernel_const(const cv::dnn::BaseConvolutionLayer* instance) {
		try {
			cv::Size ret = instance->kernel;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setKernel_Size(cv::dnn::BaseConvolutionLayer* instance, const cv::Size* val) {
		try {
			instance->kernel = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_dnn_BaseConvolutionLayer_stride_const(const cv::dnn::BaseConvolutionLayer* instance) {
		try {
			cv::Size ret = instance->stride;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setStride_Size(cv::dnn::BaseConvolutionLayer* instance, const cv::Size* val) {
		try {
			instance->stride = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_dnn_BaseConvolutionLayer_pad_const(const cv::dnn::BaseConvolutionLayer* instance) {
		try {
			cv::Size ret = instance->pad;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setPad_Size(cv::dnn::BaseConvolutionLayer* instance, const cv::Size* val) {
		try {
			instance->pad = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_dnn_BaseConvolutionLayer_dilation_const(const cv::dnn::BaseConvolutionLayer* instance) {
		try {
			cv::Size ret = instance->dilation;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setDilation_Size(cv::dnn::BaseConvolutionLayer* instance, const cv::Size* val) {
		try {
			instance->dilation = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_dnn_BaseConvolutionLayer_adjustPad_const(const cv::dnn::BaseConvolutionLayer* instance) {
		try {
			cv::Size ret = instance->adjustPad;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setAdjustPad_Size(cv::dnn::BaseConvolutionLayer* instance, const cv::Size* val) {
		try {
			instance->adjustPad = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<size_t>*> cv_dnn_BaseConvolutionLayer_adjust_pads(cv::dnn::BaseConvolutionLayer* instance) {
		try {
			std::vector<size_t> ret = instance->adjust_pads;
			return Ok(new std::vector<size_t>(ret));
		} OCVRS_CATCH(Result<std::vector<size_t>*>)
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setAdjust_pads_vector_size_t_(cv::dnn::BaseConvolutionLayer* instance, std::vector<size_t>* val) {
		try {
			instance->adjust_pads = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<size_t>*> cv_dnn_BaseConvolutionLayer_kernel_size(cv::dnn::BaseConvolutionLayer* instance) {
		try {
			std::vector<size_t> ret = instance->kernel_size;
			return Ok(new std::vector<size_t>(ret));
		} OCVRS_CATCH(Result<std::vector<size_t>*>)
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setKernel_size_vector_size_t_(cv::dnn::BaseConvolutionLayer* instance, std::vector<size_t>* val) {
		try {
			instance->kernel_size = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<size_t>*> cv_dnn_BaseConvolutionLayer_strides(cv::dnn::BaseConvolutionLayer* instance) {
		try {
			std::vector<size_t> ret = instance->strides;
			return Ok(new std::vector<size_t>(ret));
		} OCVRS_CATCH(Result<std::vector<size_t>*>)
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setStrides_vector_size_t_(cv::dnn::BaseConvolutionLayer* instance, std::vector<size_t>* val) {
		try {
			instance->strides = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<size_t>*> cv_dnn_BaseConvolutionLayer_dilations(cv::dnn::BaseConvolutionLayer* instance) {
		try {
			std::vector<size_t> ret = instance->dilations;
			return Ok(new std::vector<size_t>(ret));
		} OCVRS_CATCH(Result<std::vector<size_t>*>)
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setDilations_vector_size_t_(cv::dnn::BaseConvolutionLayer* instance, std::vector<size_t>* val) {
		try {
			instance->dilations = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<size_t>*> cv_dnn_BaseConvolutionLayer_pads_begin(cv::dnn::BaseConvolutionLayer* instance) {
		try {
			std::vector<size_t> ret = instance->pads_begin;
			return Ok(new std::vector<size_t>(ret));
		} OCVRS_CATCH(Result<std::vector<size_t>*>)
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setPads_begin_vector_size_t_(cv::dnn::BaseConvolutionLayer* instance, std::vector<size_t>* val) {
		try {
			instance->pads_begin = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<size_t>*> cv_dnn_BaseConvolutionLayer_pads_end(cv::dnn::BaseConvolutionLayer* instance) {
		try {
			std::vector<size_t> ret = instance->pads_end;
			return Ok(new std::vector<size_t>(ret));
		} OCVRS_CATCH(Result<std::vector<size_t>*>)
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setPads_end_vector_size_t_(cv::dnn::BaseConvolutionLayer* instance, std::vector<size_t>* val) {
		try {
			instance->pads_end = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_BaseConvolutionLayer_padMode_const(const cv::dnn::BaseConvolutionLayer* instance) {
		try {
			cv::String ret = instance->padMode;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setPadMode_String(cv::dnn::BaseConvolutionLayer* instance, char* val) {
		try {
			instance->padMode = std::string(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_BaseConvolutionLayer_numOutput_const(const cv::dnn::BaseConvolutionLayer* instance) {
		try {
			int ret = instance->numOutput;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_BaseConvolutionLayer_setNumOutput_int(cv::dnn::BaseConvolutionLayer* instance, int val) {
		try {
			instance->numOutput = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_BaseConvolutionLayer_delete(cv::dnn::BaseConvolutionLayer* instance) {
		delete instance;
	}
	Result<bool> cv_dnn_BatchNormLayer_hasWeights_const(const cv::dnn::BatchNormLayer* instance) {
		try {
			bool ret = instance->hasWeights;
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_BatchNormLayer_setHasWeights_bool(cv::dnn::BatchNormLayer* instance, bool val) {
		try {
			instance->hasWeights = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_dnn_BatchNormLayer_hasBias_const(const cv::dnn::BatchNormLayer* instance) {
		try {
			bool ret = instance->hasBias;
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_BatchNormLayer_setHasBias_bool(cv::dnn::BatchNormLayer* instance, bool val) {
		try {
			instance->hasBias = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_dnn_BatchNormLayer_epsilon_const(const cv::dnn::BatchNormLayer* instance) {
		try {
			float ret = instance->epsilon;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_dnn_BatchNormLayer_setEpsilon_float(cv::dnn::BatchNormLayer* instance, float val) {
		try {
			instance->epsilon = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::dnn::BatchNormLayer>*> cv_dnn_BatchNormLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::BatchNormLayer> ret = cv::dnn::BatchNormLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::BatchNormLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::BatchNormLayer>*>)
	}
	
	void cv_BlankLayer_delete(cv::dnn::BlankLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::Layer>*> cv_dnn_BlankLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::BlankLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::Layer>*>)
	}
	
	Result<cv::Ptr<cv::dnn::Layer>*> cv_dnn_ChannelsPReLULayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::ChannelsPReLULayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::Layer>*>)
	}
	
	void cv_ClassificationModel_delete(cv::dnn::ClassificationModel* instance) {
		delete instance;
	}
	Result<cv::dnn::ClassificationModel*> cv_dnn_ClassificationModel_ClassificationModel_const_StringX_const_StringX(const char* model, const char* config) {
		try {
			cv::dnn::ClassificationModel* ret = new cv::dnn::ClassificationModel(std::string(model), std::string(config));
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::dnn::ClassificationModel*>)
	}
	
	Result<cv::dnn::ClassificationModel*> cv_dnn_ClassificationModel_ClassificationModel_const_NetX(const cv::dnn::Net* network) {
		try {
			cv::dnn::ClassificationModel* ret = new cv::dnn::ClassificationModel(*network);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::dnn::ClassificationModel*>)
	}
	
	Result_void cv_dnn_ClassificationModel_classify_const__InputArrayX_intX_floatX(cv::dnn::ClassificationModel* instance, const cv::_InputArray* frame, int* classId, float* conf) {
		try {
			instance->classify(*frame, *classId, *conf);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_ConcatLayer_axis_const(const cv::dnn::ConcatLayer* instance) {
		try {
			int ret = instance->axis;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_ConcatLayer_setAxis_int(cv::dnn::ConcatLayer* instance, int val) {
		try {
			instance->axis = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_dnn_ConcatLayer_padding_const(const cv::dnn::ConcatLayer* instance) {
		try {
			bool ret = instance->padding;
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_ConcatLayer_setPadding_bool(cv::dnn::ConcatLayer* instance, bool val) {
		try {
			instance->padding = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_ConcatLayer_delete(cv::dnn::ConcatLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::ConcatLayer>*> cv_dnn_ConcatLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::ConcatLayer> ret = cv::dnn::ConcatLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::ConcatLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::ConcatLayer>*>)
	}
	
	void cv_ConstLayer_delete(cv::dnn::ConstLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::Layer>*> cv_dnn_ConstLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::ConstLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::Layer>*>)
	}
	
	void cv_ConvolutionLayer_delete(cv::dnn::ConvolutionLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::BaseConvolutionLayer>*> cv_dnn_ConvolutionLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::BaseConvolutionLayer> ret = cv::dnn::ConvolutionLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::BaseConvolutionLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::BaseConvolutionLayer>*>)
	}
	
	void cv_CropAndResizeLayer_delete(cv::dnn::CropAndResizeLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::Layer>*> cv_dnn_CropAndResizeLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::CropAndResizeLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::Layer>*>)
	}
	
	void cv_CropLayer_delete(cv::dnn::CropLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::Layer>*> cv_dnn_CropLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::CropLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::Layer>*>)
	}
	
	void cv_DeconvolutionLayer_delete(cv::dnn::DeconvolutionLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::BaseConvolutionLayer>*> cv_dnn_DeconvolutionLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::BaseConvolutionLayer> ret = cv::dnn::DeconvolutionLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::BaseConvolutionLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::BaseConvolutionLayer>*>)
	}
	
	void cv_DetectionModel_delete(cv::dnn::DetectionModel* instance) {
		delete instance;
	}
	Result<cv::dnn::DetectionModel*> cv_dnn_DetectionModel_DetectionModel_const_StringX_const_StringX(const char* model, const char* config) {
		try {
			cv::dnn::DetectionModel* ret = new cv::dnn::DetectionModel(std::string(model), std::string(config));
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::dnn::DetectionModel*>)
	}
	
	Result<cv::dnn::DetectionModel*> cv_dnn_DetectionModel_DetectionModel_const_NetX(const cv::dnn::Net* network) {
		try {
			cv::dnn::DetectionModel* ret = new cv::dnn::DetectionModel(*network);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::dnn::DetectionModel*>)
	}
	
	Result_void cv_dnn_DetectionModel_detect_const__InputArrayX_vector_int_X_vector_float_X_vector_Rect_X_float_float(cv::dnn::DetectionModel* instance, const cv::_InputArray* frame, std::vector<int>* classIds, std::vector<float>* confidences, std::vector<cv::Rect>* boxes, float confThreshold, float nmsThreshold) {
		try {
			instance->detect(*frame, *classIds, *confidences, *boxes, confThreshold, nmsThreshold);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_DetectionOutputLayer_delete(cv::dnn::DetectionOutputLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::DetectionOutputLayer>*> cv_dnn_DetectionOutputLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::DetectionOutputLayer> ret = cv::dnn::DetectionOutputLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::DetectionOutputLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::DetectionOutputLayer>*>)
	}
	
	void cv_Dict_delete(cv::dnn::Dict* instance) {
		delete instance;
	}
	Result<bool> cv_dnn_Dict_has_const_const_StringX(const cv::dnn::Dict* instance, const char* key) {
		try {
			bool ret = instance->has(std::string(key));
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<cv::dnn::DictValue**> cv_dnn_Dict_ptr_const_StringX(cv::dnn::Dict* instance, const char* key) {
		try {
			cv::dnn::DictValue* ret = instance->ptr(std::string(key));
			return Ok(new cv::dnn::DictValue*(ret));
		} OCVRS_CATCH(Result<cv::dnn::DictValue**>)
	}
	
	Result<const cv::dnn::DictValue**> cv_dnn_Dict_ptr_const_const_StringX(const cv::dnn::Dict* instance, const char* key) {
		try {
			const cv::dnn::DictValue* ret = instance->ptr(std::string(key));
			return Ok(new const cv::dnn::DictValue*(ret));
		} OCVRS_CATCH(Result<const cv::dnn::DictValue**>)
	}
	
	Result<cv::dnn::DictValue*> cv_dnn_Dict_get_const_const_StringX(const cv::dnn::Dict* instance, const char* key) {
		try {
			cv::dnn::DictValue ret = instance->get(std::string(key));
			return Ok(new cv::dnn::DictValue(ret));
		} OCVRS_CATCH(Result<cv::dnn::DictValue*>)
	}
	
	Result<void*> cv_dnn_Dict_set_cv_String_const_StringX_const_StringX(cv::dnn::Dict* instance, const char* key, const char* value) {
		try {
			cv::String ret = instance->set<cv::String>(std::string(key), std::string(value));
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::dnn::DictValue*> cv_dnn_Dict_set_cv_dnn_DictValue_const_StringX_const_DictValueX(cv::dnn::Dict* instance, const char* key, const cv::dnn::DictValue* value) {
		try {
			cv::dnn::DictValue ret = instance->set<cv::dnn::DictValue>(std::string(key), *value);
			return Ok(new cv::dnn::DictValue(ret));
		} OCVRS_CATCH(Result<cv::dnn::DictValue*>)
	}
	
	Result<double> cv_dnn_Dict_set_double_const_StringX_const_doubleX(cv::dnn::Dict* instance, const char* key, const double* value) {
		try {
			double ret = instance->set<double>(std::string(key), *value);
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<int64_t> cv_dnn_Dict_set_int64_t_const_StringX_const_int64_tX(cv::dnn::Dict* instance, const char* key, const int64_t* value) {
		try {
			int64_t ret = instance->set<int64_t>(std::string(key), *value);
			return Ok(ret);
		} OCVRS_CATCH(Result<int64_t>)
	}
	
	Result_void cv_dnn_Dict_erase_const_StringX(cv::dnn::Dict* instance, const char* key) {
		try {
			instance->erase(std::string(key));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_DictValue_delete(cv::dnn::DictValue* instance) {
		delete instance;
	}
	Result<cv::dnn::DictValue*> cv_dnn_DictValue_DictValue_const_DictValueX(const cv::dnn::DictValue* r) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(*r);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::dnn::DictValue*>)
	}
	
	Result<cv::dnn::DictValue*> cv_dnn_DictValue_DictValue_bool(bool i) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(i);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::dnn::DictValue*>)
	}
	
	Result<cv::dnn::DictValue*> cv_dnn_DictValue_DictValue_int64_t(int64_t i) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(i);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::dnn::DictValue*>)
	}
	
	Result<cv::dnn::DictValue*> cv_dnn_DictValue_DictValue_int(int i) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(i);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::dnn::DictValue*>)
	}
	
	Result<cv::dnn::DictValue*> cv_dnn_DictValue_DictValue_unsigned_int(unsigned int p) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(p);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::dnn::DictValue*>)
	}
	
	Result<cv::dnn::DictValue*> cv_dnn_DictValue_DictValue_double(double p) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(p);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::dnn::DictValue*>)
	}
	
	Result<cv::dnn::DictValue*> cv_dnn_DictValue_DictValue_const_charX(const char* s) {
		try {
			cv::dnn::DictValue* ret = new cv::dnn::DictValue(s);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::dnn::DictValue*>)
	}
	
	Result<void*> cv_dnn_DictValue_get_cv_String_const_int(const cv::dnn::DictValue* instance, int idx) {
		try {
			cv::String ret = instance->get<cv::String>(idx);
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<double> cv_dnn_DictValue_get_double_const_int(const cv::dnn::DictValue* instance, int idx) {
		try {
			double ret = instance->get<double>(idx);
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<int> cv_dnn_DictValue_get_int_const_int(const cv::dnn::DictValue* instance, int idx) {
		try {
			int ret = instance->get<int>(idx);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int64_t> cv_dnn_DictValue_get_int64_t_const_int(const cv::dnn::DictValue* instance, int idx) {
		try {
			int64_t ret = instance->get<int64_t>(idx);
			return Ok(ret);
		} OCVRS_CATCH(Result<int64_t>)
	}
	
	Result<int> cv_dnn_DictValue_size_const(const cv::dnn::DictValue* instance) {
		try {
			int ret = instance->size();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<bool> cv_dnn_DictValue_isInt_const(const cv::dnn::DictValue* instance) {
		try {
			bool ret = instance->isInt();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_dnn_DictValue_isString_const(const cv::dnn::DictValue* instance) {
		try {
			bool ret = instance->isString();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_dnn_DictValue_isReal_const(const cv::dnn::DictValue* instance) {
		try {
			bool ret = instance->isReal();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<int> cv_dnn_DictValue_getIntValue_const_int(const cv::dnn::DictValue* instance, int idx) {
		try {
			int ret = instance->getIntValue(idx);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<double> cv_dnn_DictValue_getRealValue_const_int(const cv::dnn::DictValue* instance, int idx) {
		try {
			double ret = instance->getRealValue(idx);
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<void*> cv_dnn_DictValue_getStringValue_const_int(const cv::dnn::DictValue* instance, int idx) {
		try {
			cv::String ret = instance->getStringValue(idx);
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::Ptr<cv::dnn::ELULayer>*> cv_dnn_ELULayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::ELULayer> ret = cv::dnn::ELULayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::ELULayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::ELULayer>*>)
	}
	
	void cv_EltwiseLayer_delete(cv::dnn::EltwiseLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::EltwiseLayer>*> cv_dnn_EltwiseLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::EltwiseLayer> ret = cv::dnn::EltwiseLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::EltwiseLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::EltwiseLayer>*>)
	}
	
	void cv_FlattenLayer_delete(cv::dnn::FlattenLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::FlattenLayer>*> cv_dnn_FlattenLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::FlattenLayer> ret = cv::dnn::FlattenLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::FlattenLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::FlattenLayer>*>)
	}
	
	Result<int> cv_dnn_InnerProductLayer_axis_const(const cv::dnn::InnerProductLayer* instance) {
		try {
			int ret = instance->axis;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_InnerProductLayer_setAxis_int(cv::dnn::InnerProductLayer* instance, int val) {
		try {
			instance->axis = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_InnerProductLayer_delete(cv::dnn::InnerProductLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::InnerProductLayer>*> cv_dnn_InnerProductLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::InnerProductLayer> ret = cv::dnn::InnerProductLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::InnerProductLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::InnerProductLayer>*>)
	}
	
	void cv_InterpLayer_delete(cv::dnn::InterpLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::Layer>*> cv_dnn_InterpLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::InterpLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::Layer>*>)
	}
	
	void cv_KeypointsModel_delete(cv::dnn::KeypointsModel* instance) {
		delete instance;
	}
	Result<cv::dnn::KeypointsModel*> cv_dnn_KeypointsModel_KeypointsModel_const_StringX_const_StringX(const char* model, const char* config) {
		try {
			cv::dnn::KeypointsModel* ret = new cv::dnn::KeypointsModel(std::string(model), std::string(config));
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::dnn::KeypointsModel*>)
	}
	
	Result<cv::dnn::KeypointsModel*> cv_dnn_KeypointsModel_KeypointsModel_const_NetX(const cv::dnn::Net* network) {
		try {
			cv::dnn::KeypointsModel* ret = new cv::dnn::KeypointsModel(*network);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::dnn::KeypointsModel*>)
	}
	
	Result<std::vector<cv::Point2f>*> cv_dnn_KeypointsModel_estimate_const__InputArrayX_float(cv::dnn::KeypointsModel* instance, const cv::_InputArray* frame, float thresh) {
		try {
			std::vector<cv::Point2f> ret = instance->estimate(*frame, thresh);
			return Ok(new std::vector<cv::Point2f>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::Point2f>*>)
	}
	
	Result<int> cv_dnn_LRNLayer_type_const(const cv::dnn::LRNLayer* instance) {
		try {
			int ret = instance->type;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_LRNLayer_setType_int(cv::dnn::LRNLayer* instance, int val) {
		try {
			instance->type = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_LRNLayer_size_const(const cv::dnn::LRNLayer* instance) {
		try {
			int ret = instance->size;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_LRNLayer_setSize_int(cv::dnn::LRNLayer* instance, int val) {
		try {
			instance->size = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_dnn_LRNLayer_alpha_const(const cv::dnn::LRNLayer* instance) {
		try {
			float ret = instance->alpha;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_dnn_LRNLayer_setAlpha_float(cv::dnn::LRNLayer* instance, float val) {
		try {
			instance->alpha = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_dnn_LRNLayer_beta_const(const cv::dnn::LRNLayer* instance) {
		try {
			float ret = instance->beta;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_dnn_LRNLayer_setBeta_float(cv::dnn::LRNLayer* instance, float val) {
		try {
			instance->beta = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_dnn_LRNLayer_bias_const(const cv::dnn::LRNLayer* instance) {
		try {
			float ret = instance->bias;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_dnn_LRNLayer_setBias_float(cv::dnn::LRNLayer* instance, float val) {
		try {
			instance->bias = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_dnn_LRNLayer_normBySize_const(const cv::dnn::LRNLayer* instance) {
		try {
			bool ret = instance->normBySize;
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_LRNLayer_setNormBySize_bool(cv::dnn::LRNLayer* instance, bool val) {
		try {
			instance->normBySize = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_LRNLayer_delete(cv::dnn::LRNLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::LRNLayer>*> cv_dnn_LRNLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::LRNLayer> ret = cv::dnn::LRNLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::LRNLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::LRNLayer>*>)
	}
	
	Result<cv::Ptr<cv::dnn::LSTMLayer>*> cv_dnn_LSTMLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::LSTMLayer> ret = cv::dnn::LSTMLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::LSTMLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::LSTMLayer>*>)
	}
	
	Result_void cv_dnn_LSTMLayer_setWeights_const_MatX_const_MatX_const_MatX(cv::dnn::LSTMLayer* instance, const cv::Mat* Wh, const cv::Mat* Wx, const cv::Mat* b) {
		try {
			instance->setWeights(*Wh, *Wx, *b);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_LSTMLayer_setOutShape_const_MatShapeX(cv::dnn::LSTMLayer* instance, const cv::dnn::MatShape* outTailShape) {
		try {
			instance->setOutShape(*outTailShape);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_LSTMLayer_setUseTimstampsDim_bool(cv::dnn::LSTMLayer* instance, bool use) {
		try {
			instance->setUseTimstampsDim(use);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_LSTMLayer_setProduceCellOutput_bool(cv::dnn::LSTMLayer* instance, bool produce) {
		try {
			instance->setProduceCellOutput(produce);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_LSTMLayer_inputNameToIndex_String(cv::dnn::LSTMLayer* instance, char* inputName) {
		try {
			int ret = instance->inputNameToIndex(std::string(inputName));
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_dnn_LSTMLayer_outputNameToIndex_const_StringX(cv::dnn::LSTMLayer* instance, const char* outputName) {
		try {
			int ret = instance->outputNameToIndex(std::string(outputName));
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<std::vector<cv::Mat>*> cv_dnn_Layer_blobs(cv::dnn::Layer* instance) {
		try {
			std::vector<cv::Mat> ret = instance->blobs;
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::Mat>*>)
	}
	
	Result_void cv_dnn_Layer_setBlobs_vector_Mat_(cv::dnn::Layer* instance, std::vector<cv::Mat>* val) {
		try {
			instance->blobs = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_Layer_name_const(const cv::dnn::Layer* instance) {
		try {
			cv::String ret = instance->name;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_Layer_setName_String(cv::dnn::Layer* instance, char* val) {
		try {
			instance->name = std::string(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_Layer_type_const(const cv::dnn::Layer* instance) {
		try {
			cv::String ret = instance->type;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_Layer_setType_String(cv::dnn::Layer* instance, char* val) {
		try {
			instance->type = std::string(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_Layer_preferableTarget_const(const cv::dnn::Layer* instance) {
		try {
			int ret = instance->preferableTarget;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_Layer_setPreferableTarget_int(cv::dnn::Layer* instance, int val) {
		try {
			instance->preferableTarget = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Layer_delete(cv::dnn::Layer* instance) {
		delete instance;
	}
	Result_void cv_dnn_Layer_finalize_const__InputArrayX_const__OutputArrayX(cv::dnn::Layer* instance, const cv::_InputArray* inputs, const cv::_OutputArray* outputs) {
		try {
			instance->finalize(*inputs, *outputs);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Layer_forward_vector_MatX_X_vector_Mat_X_vector_Mat_X(cv::dnn::Layer* instance, std::vector<cv::Mat*>* input, std::vector<cv::Mat>* output, std::vector<cv::Mat>* internals) {
		try {
			instance->forward(*input, *output, *internals);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Layer_forward_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(cv::dnn::Layer* instance, const cv::_InputArray* inputs, const cv::_OutputArray* outputs, const cv::_OutputArray* internals) {
		try {
			instance->forward(*inputs, *outputs, *internals);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Layer_forward_fallback_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(cv::dnn::Layer* instance, const cv::_InputArray* inputs, const cv::_OutputArray* outputs, const cv::_OutputArray* internals) {
		try {
			instance->forward_fallback(*inputs, *outputs, *internals);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Layer_finalize_const_vector_Mat_X_vector_Mat_X(cv::dnn::Layer* instance, const std::vector<cv::Mat>* inputs, std::vector<cv::Mat>* outputs) {
		try {
			instance->finalize(*inputs, *outputs);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<cv::Mat>*> cv_dnn_Layer_finalize_const_vector_Mat_X(cv::dnn::Layer* instance, const std::vector<cv::Mat>* inputs) {
		try {
			std::vector<cv::Mat> ret = instance->finalize(*inputs);
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::Mat>*>)
	}
	
	Result_void cv_dnn_Layer_run_const_vector_Mat_X_vector_Mat_X_vector_Mat_X(cv::dnn::Layer* instance, const std::vector<cv::Mat>* inputs, std::vector<cv::Mat>* outputs, std::vector<cv::Mat>* internals) {
		try {
			instance->run(*inputs, *outputs, *internals);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_Layer_inputNameToIndex_String(cv::dnn::Layer* instance, char* inputName) {
		try {
			int ret = instance->inputNameToIndex(std::string(inputName));
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_dnn_Layer_outputNameToIndex_const_StringX(cv::dnn::Layer* instance, const char* outputName) {
		try {
			int ret = instance->outputNameToIndex(std::string(outputName));
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<bool> cv_dnn_Layer_supportBackend_int(cv::dnn::Layer* instance, int backendId) {
		try {
			bool ret = instance->supportBackend(backendId);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<cv::Ptr<cv::dnn::BackendNode>*> cv_dnn_Layer_initHalide_const_vector_Ptr_BackendWrapper__X(cv::dnn::Layer* instance, const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* inputs) {
		try {
			cv::Ptr<cv::dnn::BackendNode> ret = instance->initHalide(*inputs);
			return Ok(new cv::Ptr<cv::dnn::BackendNode>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::BackendNode>*>)
	}
	
	Result<cv::Ptr<cv::dnn::BackendNode>*> cv_dnn_Layer_initInfEngine_const_vector_Ptr_BackendWrapper__X(cv::dnn::Layer* instance, const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* inputs) {
		try {
			cv::Ptr<cv::dnn::BackendNode> ret = instance->initInfEngine(*inputs);
			return Ok(new cv::Ptr<cv::dnn::BackendNode>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::BackendNode>*>)
	}
	
	Result<cv::Ptr<cv::dnn::BackendNode>*> cv_dnn_Layer_initNgraph_const_vector_Ptr_BackendWrapper__X_const_vector_Ptr_BackendNode__X(cv::dnn::Layer* instance, const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* inputs, const std::vector<cv::Ptr<cv::dnn::BackendNode>>* nodes) {
		try {
			cv::Ptr<cv::dnn::BackendNode> ret = instance->initNgraph(*inputs, *nodes);
			return Ok(new cv::Ptr<cv::dnn::BackendNode>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::BackendNode>*>)
	}
	
	Result<cv::Ptr<cv::dnn::BackendNode>*> cv_dnn_Layer_initVkCom_const_vector_Ptr_BackendWrapper__X(cv::dnn::Layer* instance, const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* inputs) {
		try {
			cv::Ptr<cv::dnn::BackendNode> ret = instance->initVkCom(*inputs);
			return Ok(new cv::Ptr<cv::dnn::BackendNode>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::BackendNode>*>)
	}
	
	Result<cv::Ptr<cv::dnn::BackendNode>*> cv_dnn_Layer_initCUDA_voidX_const_vector_Ptr_BackendWrapper__X_const_vector_Ptr_BackendWrapper__X(cv::dnn::Layer* instance, void* context, const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* inputs, const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* outputs) {
		try {
			cv::Ptr<cv::dnn::BackendNode> ret = instance->initCUDA(context, *inputs, *outputs);
			return Ok(new cv::Ptr<cv::dnn::BackendNode>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::BackendNode>*>)
	}
	
	Result_void cv_dnn_Layer_applyHalideScheduler_const_Ptr_BackendNode_X_const_vector_MatX_X_const_vector_Mat_X_int(const cv::dnn::Layer* instance, cv::Ptr<cv::dnn::BackendNode>* node, const std::vector<cv::Mat*>* inputs, const std::vector<cv::Mat>* outputs, int targetId) {
		try {
			instance->applyHalideScheduler(*node, *inputs, *outputs, targetId);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::dnn::BackendNode>*> cv_dnn_Layer_tryAttach_const_Ptr_BackendNode_X(cv::dnn::Layer* instance, const cv::Ptr<cv::dnn::BackendNode>* node) {
		try {
			cv::Ptr<cv::dnn::BackendNode> ret = instance->tryAttach(*node);
			return Ok(new cv::Ptr<cv::dnn::BackendNode>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::BackendNode>*>)
	}
	
	Result<bool> cv_dnn_Layer_setActivation_const_Ptr_ActivationLayer_X(cv::dnn::Layer* instance, const cv::Ptr<cv::dnn::ActivationLayer>* layer) {
		try {
			bool ret = instance->setActivation(*layer);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_dnn_Layer_tryFuse_Ptr_Layer_X(cv::dnn::Layer* instance, cv::Ptr<cv::dnn::Layer>* top) {
		try {
			bool ret = instance->tryFuse(*top);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_Layer_getScaleShift_const_MatX_MatX(const cv::dnn::Layer* instance, cv::Mat* scale, cv::Mat* shift) {
		try {
			instance->getScaleShift(*scale, *shift);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Layer_unsetAttached(cv::dnn::Layer* instance) {
		try {
			instance->unsetAttached();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_dnn_Layer_getMemoryShapes_const_const_vector_MatShape_X_int_vector_MatShape_X_vector_MatShape_X(const cv::dnn::Layer* instance, const std::vector<cv::dnn::MatShape>* inputs, int requiredOutputs, std::vector<cv::dnn::MatShape>* outputs, std::vector<cv::dnn::MatShape>* internals) {
		try {
			bool ret = instance->getMemoryShapes(*inputs, requiredOutputs, *outputs, *internals);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<int64_t> cv_dnn_Layer_getFLOPS_const_const_vector_MatShape_X_const_vector_MatShape_X(const cv::dnn::Layer* instance, const std::vector<cv::dnn::MatShape>* inputs, const std::vector<cv::dnn::MatShape>* outputs) {
		try {
			int64_t ret = instance->getFLOPS(*inputs, *outputs);
			return Ok(ret);
		} OCVRS_CATCH(Result<int64_t>)
	}
	
	Result<cv::dnn::Layer*> cv_dnn_Layer_Layer() {
		try {
			cv::dnn::Layer* ret = new cv::dnn::Layer();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::dnn::Layer*>)
	}
	
	Result<cv::dnn::Layer*> cv_dnn_Layer_Layer_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::dnn::Layer* ret = new cv::dnn::Layer(*params);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::dnn::Layer*>)
	}
	
	Result_void cv_dnn_Layer_setParamsFrom_const_LayerParamsX(cv::dnn::Layer* instance, const cv::dnn::LayerParams* params) {
		try {
			instance->setParamsFrom(*params);
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
	
	Result<cv::Ptr<cv::dnn::Layer>*> cv_dnn_LayerFactory_createLayerInstance_const_StringX_LayerParamsX(const char* type, cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::LayerFactory::createLayerInstance(std::string(type), *params);
			return Ok(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::Layer>*>)
	}
	
	Result<std::vector<cv::Mat>*> cv_dnn_LayerParams_blobs(cv::dnn::LayerParams* instance) {
		try {
			std::vector<cv::Mat> ret = instance->blobs;
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::Mat>*>)
	}
	
	Result_void cv_dnn_LayerParams_setBlobs_vector_Mat_(cv::dnn::LayerParams* instance, std::vector<cv::Mat>* val) {
		try {
			instance->blobs = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_LayerParams_name_const(const cv::dnn::LayerParams* instance) {
		try {
			cv::String ret = instance->name;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_LayerParams_setName_String(cv::dnn::LayerParams* instance, char* val) {
		try {
			instance->name = std::string(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_LayerParams_type_const(const cv::dnn::LayerParams* instance) {
		try {
			cv::String ret = instance->type;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_LayerParams_setType_String(cv::dnn::LayerParams* instance, char* val) {
		try {
			instance->type = std::string(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_LayerParams_delete(cv::dnn::LayerParams* instance) {
		delete instance;
	}
	Result<float> cv_dnn_MVNLayer_eps_const(const cv::dnn::MVNLayer* instance) {
		try {
			float ret = instance->eps;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_dnn_MVNLayer_setEps_float(cv::dnn::MVNLayer* instance, float val) {
		try {
			instance->eps = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_dnn_MVNLayer_normVariance_const(const cv::dnn::MVNLayer* instance) {
		try {
			bool ret = instance->normVariance;
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_MVNLayer_setNormVariance_bool(cv::dnn::MVNLayer* instance, bool val) {
		try {
			instance->normVariance = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_dnn_MVNLayer_acrossChannels_const(const cv::dnn::MVNLayer* instance) {
		try {
			bool ret = instance->acrossChannels;
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_MVNLayer_setAcrossChannels_bool(cv::dnn::MVNLayer* instance, bool val) {
		try {
			instance->acrossChannels = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_MVNLayer_delete(cv::dnn::MVNLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::MVNLayer>*> cv_dnn_MVNLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::MVNLayer> ret = cv::dnn::MVNLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::MVNLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::MVNLayer>*>)
	}
	
	Result<cv::Size> cv_dnn_MaxUnpoolLayer_poolKernel_const(const cv::dnn::MaxUnpoolLayer* instance) {
		try {
			cv::Size ret = instance->poolKernel;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_dnn_MaxUnpoolLayer_setPoolKernel_Size(cv::dnn::MaxUnpoolLayer* instance, const cv::Size* val) {
		try {
			instance->poolKernel = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_dnn_MaxUnpoolLayer_poolPad_const(const cv::dnn::MaxUnpoolLayer* instance) {
		try {
			cv::Size ret = instance->poolPad;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_dnn_MaxUnpoolLayer_setPoolPad_Size(cv::dnn::MaxUnpoolLayer* instance, const cv::Size* val) {
		try {
			instance->poolPad = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_dnn_MaxUnpoolLayer_poolStride_const(const cv::dnn::MaxUnpoolLayer* instance) {
		try {
			cv::Size ret = instance->poolStride;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_dnn_MaxUnpoolLayer_setPoolStride_Size(cv::dnn::MaxUnpoolLayer* instance, const cv::Size* val) {
		try {
			instance->poolStride = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_MaxUnpoolLayer_delete(cv::dnn::MaxUnpoolLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::MaxUnpoolLayer>*> cv_dnn_MaxUnpoolLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::MaxUnpoolLayer> ret = cv::dnn::MaxUnpoolLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::MaxUnpoolLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::MaxUnpoolLayer>*>)
	}
	
	Result<cv::Ptr<cv::dnn::MishLayer>*> cv_dnn_MishLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::MishLayer> ret = cv::dnn::MishLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::MishLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::MishLayer>*>)
	}
	
	void cv_Model_delete(cv::dnn::Model* instance) {
		delete instance;
	}
	Result<cv::dnn::Model*> cv_dnn_Model_Model() {
		try {
			cv::dnn::Model* ret = new cv::dnn::Model();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::dnn::Model*>)
	}
	
	Result<cv::dnn::Model*> cv_dnn_Model_Model_const_StringX_const_StringX(const char* model, const char* config) {
		try {
			cv::dnn::Model* ret = new cv::dnn::Model(std::string(model), std::string(config));
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::dnn::Model*>)
	}
	
	Result<cv::dnn::Model*> cv_dnn_Model_Model_const_NetX(const cv::dnn::Net* network) {
		try {
			cv::dnn::Model* ret = new cv::dnn::Model(*network);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::dnn::Model*>)
	}
	
	Result<cv::dnn::Model*> cv_dnn_Model_setInputSize_const_SizeX(cv::dnn::Model* instance, const cv::Size* size) {
		try {
			cv::dnn::Model ret = instance->setInputSize(*size);
			return Ok(new cv::dnn::Model(ret));
		} OCVRS_CATCH(Result<cv::dnn::Model*>)
	}
	
	Result<cv::dnn::Model*> cv_dnn_Model_setInputSize_int_int(cv::dnn::Model* instance, int width, int height) {
		try {
			cv::dnn::Model ret = instance->setInputSize(width, height);
			return Ok(new cv::dnn::Model(ret));
		} OCVRS_CATCH(Result<cv::dnn::Model*>)
	}
	
	Result<cv::dnn::Model*> cv_dnn_Model_setInputMean_const_ScalarX(cv::dnn::Model* instance, const cv::Scalar* mean) {
		try {
			cv::dnn::Model ret = instance->setInputMean(*mean);
			return Ok(new cv::dnn::Model(ret));
		} OCVRS_CATCH(Result<cv::dnn::Model*>)
	}
	
	Result<cv::dnn::Model*> cv_dnn_Model_setInputScale_double(cv::dnn::Model* instance, double scale) {
		try {
			cv::dnn::Model ret = instance->setInputScale(scale);
			return Ok(new cv::dnn::Model(ret));
		} OCVRS_CATCH(Result<cv::dnn::Model*>)
	}
	
	Result<cv::dnn::Model*> cv_dnn_Model_setInputCrop_bool(cv::dnn::Model* instance, bool crop) {
		try {
			cv::dnn::Model ret = instance->setInputCrop(crop);
			return Ok(new cv::dnn::Model(ret));
		} OCVRS_CATCH(Result<cv::dnn::Model*>)
	}
	
	Result<cv::dnn::Model*> cv_dnn_Model_setInputSwapRB_bool(cv::dnn::Model* instance, bool swapRB) {
		try {
			cv::dnn::Model ret = instance->setInputSwapRB(swapRB);
			return Ok(new cv::dnn::Model(ret));
		} OCVRS_CATCH(Result<cv::dnn::Model*>)
	}
	
	Result_void cv_dnn_Model_setInputParams_double_const_SizeX_const_ScalarX_bool_bool(cv::dnn::Model* instance, double scale, const cv::Size* size, const cv::Scalar* mean, bool swapRB, bool crop) {
		try {
			instance->setInputParams(scale, *size, *mean, swapRB, crop);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Model_predict_const__InputArrayX_const__OutputArrayX(cv::dnn::Model* instance, const cv::_InputArray* frame, const cv::_OutputArray* outs) {
		try {
			instance->predict(*frame, *outs);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Net_delete(cv::dnn::Net* instance) {
		delete instance;
	}
	Result<cv::dnn::Net*> cv_dnn_Net_Net() {
		try {
			cv::dnn::Net* ret = new cv::dnn::Net();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::dnn::Net*>)
	}
	
	Result<cv::dnn::Net*> cv_dnn_Net_readFromModelOptimizer_const_StringX_const_StringX(const char* xml, const char* bin) {
		try {
			cv::dnn::Net ret = cv::dnn::Net::readFromModelOptimizer(std::string(xml), std::string(bin));
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<cv::dnn::Net*>)
	}
	
	Result<cv::dnn::Net*> cv_dnn_Net_readFromModelOptimizer_const_vector_unsigned_char_X_const_vector_unsigned_char_X(const std::vector<unsigned char>* bufferModelConfig, const std::vector<unsigned char>* bufferWeights) {
		try {
			cv::dnn::Net ret = cv::dnn::Net::readFromModelOptimizer(*bufferModelConfig, *bufferWeights);
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<cv::dnn::Net*>)
	}
	
	Result<cv::dnn::Net*> cv_dnn_Net_readFromModelOptimizer_const_unsigned_charX_size_t_const_unsigned_charX_size_t(const unsigned char* bufferModelConfigPtr, size_t bufferModelConfigSize, const unsigned char* bufferWeightsPtr, size_t bufferWeightsSize) {
		try {
			cv::dnn::Net ret = cv::dnn::Net::readFromModelOptimizer(bufferModelConfigPtr, bufferModelConfigSize, bufferWeightsPtr, bufferWeightsSize);
			return Ok(new cv::dnn::Net(ret));
		} OCVRS_CATCH(Result<cv::dnn::Net*>)
	}
	
	Result<bool> cv_dnn_Net_empty_const(const cv::dnn::Net* instance) {
		try {
			bool ret = instance->empty();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_dnn_Net_dump(cv::dnn::Net* instance) {
		try {
			cv::String ret = instance->dump();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_Net_dumpToFile_const_StringX(cv::dnn::Net* instance, const char* path) {
		try {
			instance->dumpToFile(std::string(path));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_Net_addLayer_const_StringX_const_StringX_LayerParamsX(cv::dnn::Net* instance, const char* name, const char* type, cv::dnn::LayerParams* params) {
		try {
			int ret = instance->addLayer(std::string(name), std::string(type), *params);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_dnn_Net_addLayerToPrev_const_StringX_const_StringX_LayerParamsX(cv::dnn::Net* instance, const char* name, const char* type, cv::dnn::LayerParams* params) {
		try {
			int ret = instance->addLayerToPrev(std::string(name), std::string(type), *params);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_dnn_Net_getLayerId_const_StringX(cv::dnn::Net* instance, const char* layer) {
		try {
			int ret = instance->getLayerId(std::string(layer));
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<std::vector<cv::String>*> cv_dnn_Net_getLayerNames_const(const cv::dnn::Net* instance) {
		try {
			std::vector<cv::String> ret = instance->getLayerNames();
			return Ok(new std::vector<cv::String>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::String>*>)
	}
	
	Result<cv::Ptr<cv::dnn::Layer>*> cv_dnn_Net_getLayer_LayerId(cv::dnn::Net* instance, cv::dnn::Net::LayerId* layerId) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = instance->getLayer(*layerId);
			return Ok(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::Layer>*>)
	}
	
	Result<std::vector<cv::Ptr<cv::dnn::Layer>>*> cv_dnn_Net_getLayerInputs_LayerId(cv::dnn::Net* instance, cv::dnn::Net::LayerId* layerId) {
		try {
			std::vector<cv::Ptr<cv::dnn::Layer>> ret = instance->getLayerInputs(*layerId);
			return Ok(new std::vector<cv::Ptr<cv::dnn::Layer>>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::Ptr<cv::dnn::Layer>>*>)
	}
	
	Result_void cv_dnn_Net_connect_String_String(cv::dnn::Net* instance, char* outPin, char* inpPin) {
		try {
			instance->connect(std::string(outPin), std::string(inpPin));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_connect_int_int_int_int(cv::dnn::Net* instance, int outLayerId, int outNum, int inpLayerId, int inpNum) {
		try {
			instance->connect(outLayerId, outNum, inpLayerId, inpNum);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_setInputsNames_const_vector_String_X(cv::dnn::Net* instance, const std::vector<cv::String>* inputBlobNames) {
		try {
			instance->setInputsNames(*inputBlobNames);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_dnn_Net_forward_const_StringX(cv::dnn::Net* instance, const char* outputName) {
		try {
			cv::Mat ret = instance->forward(std::string(outputName));
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::AsyncArray*> cv_dnn_Net_forwardAsync_const_StringX(cv::dnn::Net* instance, const char* outputName) {
		try {
			cv::AsyncArray ret = instance->forwardAsync(std::string(outputName));
			return Ok(new cv::AsyncArray(ret));
		} OCVRS_CATCH(Result<cv::AsyncArray*>)
	}
	
	Result_void cv_dnn_Net_forward_const__OutputArrayX_const_StringX(cv::dnn::Net* instance, const cv::_OutputArray* outputBlobs, const char* outputName) {
		try {
			instance->forward(*outputBlobs, std::string(outputName));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_forward_const__OutputArrayX_const_vector_String_X(cv::dnn::Net* instance, const cv::_OutputArray* outputBlobs, const std::vector<cv::String>* outBlobNames) {
		try {
			instance->forward(*outputBlobs, *outBlobNames);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_forward_vector_vector_Mat__X_const_vector_String_X(cv::dnn::Net* instance, std::vector<std::vector<cv::Mat>>* outputBlobs, const std::vector<cv::String>* outBlobNames) {
		try {
			instance->forward(*outputBlobs, *outBlobNames);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_setHalideScheduler_const_StringX(cv::dnn::Net* instance, const char* scheduler) {
		try {
			instance->setHalideScheduler(std::string(scheduler));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_setPreferableBackend_int(cv::dnn::Net* instance, int backendId) {
		try {
			instance->setPreferableBackend(backendId);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_setPreferableTarget_int(cv::dnn::Net* instance, int targetId) {
		try {
			instance->setPreferableTarget(targetId);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_setInput_const__InputArrayX_const_StringX_double_const_ScalarX(cv::dnn::Net* instance, const cv::_InputArray* blob, const char* name, double scalefactor, const cv::Scalar* mean) {
		try {
			instance->setInput(*blob, std::string(name), scalefactor, *mean);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_setParam_LayerId_int_const_MatX(cv::dnn::Net* instance, cv::dnn::Net::LayerId* layer, int numParam, const cv::Mat* blob) {
		try {
			instance->setParam(*layer, numParam, *blob);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_dnn_Net_getParam_LayerId_int(cv::dnn::Net* instance, cv::dnn::Net::LayerId* layer, int numParam) {
		try {
			cv::Mat ret = instance->getParam(*layer, numParam);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<std::vector<int>*> cv_dnn_Net_getUnconnectedOutLayers_const(const cv::dnn::Net* instance) {
		try {
			std::vector<int> ret = instance->getUnconnectedOutLayers();
			return Ok(new std::vector<int>(ret));
		} OCVRS_CATCH(Result<std::vector<int>*>)
	}
	
	Result<std::vector<cv::String>*> cv_dnn_Net_getUnconnectedOutLayersNames_const(const cv::dnn::Net* instance) {
		try {
			std::vector<cv::String> ret = instance->getUnconnectedOutLayersNames();
			return Ok(new std::vector<cv::String>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::String>*>)
	}
	
	Result_void cv_dnn_Net_getLayersShapes_const_const_vector_MatShape_X_vector_int_X_vector_vector_MatShape__X_vector_vector_MatShape__X(const cv::dnn::Net* instance, const std::vector<cv::dnn::MatShape>* netInputShapes, std::vector<int>* layersIds, std::vector<std::vector<cv::dnn::MatShape>>* inLayersShapes, std::vector<std::vector<cv::dnn::MatShape>>* outLayersShapes) {
		try {
			instance->getLayersShapes(*netInputShapes, *layersIds, *inLayersShapes, *outLayersShapes);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_getLayersShapes_const_const_MatShapeX_vector_int_X_vector_vector_MatShape__X_vector_vector_MatShape__X(const cv::dnn::Net* instance, const cv::dnn::MatShape* netInputShape, std::vector<int>* layersIds, std::vector<std::vector<cv::dnn::MatShape>>* inLayersShapes, std::vector<std::vector<cv::dnn::MatShape>>* outLayersShapes) {
		try {
			instance->getLayersShapes(*netInputShape, *layersIds, *inLayersShapes, *outLayersShapes);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_getLayerShapes_const_const_MatShapeX_int_vector_MatShape_X_vector_MatShape_X(const cv::dnn::Net* instance, const cv::dnn::MatShape* netInputShape, int layerId, std::vector<cv::dnn::MatShape>* inLayerShapes, std::vector<cv::dnn::MatShape>* outLayerShapes) {
		try {
			instance->getLayerShapes(*netInputShape, layerId, *inLayerShapes, *outLayerShapes);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_getLayerShapes_const_const_vector_MatShape_X_int_vector_MatShape_X_vector_MatShape_X(const cv::dnn::Net* instance, const std::vector<cv::dnn::MatShape>* netInputShapes, int layerId, std::vector<cv::dnn::MatShape>* inLayerShapes, std::vector<cv::dnn::MatShape>* outLayerShapes) {
		try {
			instance->getLayerShapes(*netInputShapes, layerId, *inLayerShapes, *outLayerShapes);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int64_t> cv_dnn_Net_getFLOPS_const_const_vector_MatShape_X(const cv::dnn::Net* instance, const std::vector<cv::dnn::MatShape>* netInputShapes) {
		try {
			int64_t ret = instance->getFLOPS(*netInputShapes);
			return Ok(ret);
		} OCVRS_CATCH(Result<int64_t>)
	}
	
	Result<int64_t> cv_dnn_Net_getFLOPS_const_const_MatShapeX(const cv::dnn::Net* instance, const cv::dnn::MatShape* netInputShape) {
		try {
			int64_t ret = instance->getFLOPS(*netInputShape);
			return Ok(ret);
		} OCVRS_CATCH(Result<int64_t>)
	}
	
	Result<int64_t> cv_dnn_Net_getFLOPS_const_int_const_vector_MatShape_X(const cv::dnn::Net* instance, int layerId, const std::vector<cv::dnn::MatShape>* netInputShapes) {
		try {
			int64_t ret = instance->getFLOPS(layerId, *netInputShapes);
			return Ok(ret);
		} OCVRS_CATCH(Result<int64_t>)
	}
	
	Result<int64_t> cv_dnn_Net_getFLOPS_const_int_const_MatShapeX(const cv::dnn::Net* instance, int layerId, const cv::dnn::MatShape* netInputShape) {
		try {
			int64_t ret = instance->getFLOPS(layerId, *netInputShape);
			return Ok(ret);
		} OCVRS_CATCH(Result<int64_t>)
	}
	
	Result_void cv_dnn_Net_getLayerTypes_const_vector_String_X(const cv::dnn::Net* instance, std::vector<cv::String>* layersTypes) {
		try {
			instance->getLayerTypes(*layersTypes);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_Net_getLayersCount_const_const_StringX(const cv::dnn::Net* instance, const char* layerType) {
		try {
			int ret = instance->getLayersCount(std::string(layerType));
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_Net_getMemoryConsumption_const_const_vector_MatShape_X_size_tX_size_tX(const cv::dnn::Net* instance, const std::vector<cv::dnn::MatShape>* netInputShapes, size_t* weights, size_t* blobs) {
		try {
			instance->getMemoryConsumption(*netInputShapes, *weights, *blobs);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_getMemoryConsumption_const_const_MatShapeX_size_tX_size_tX(const cv::dnn::Net* instance, const cv::dnn::MatShape* netInputShape, size_t* weights, size_t* blobs) {
		try {
			instance->getMemoryConsumption(*netInputShape, *weights, *blobs);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_getMemoryConsumption_const_int_const_vector_MatShape_X_size_tX_size_tX(const cv::dnn::Net* instance, int layerId, const std::vector<cv::dnn::MatShape>* netInputShapes, size_t* weights, size_t* blobs) {
		try {
			instance->getMemoryConsumption(layerId, *netInputShapes, *weights, *blobs);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_getMemoryConsumption_const_int_const_MatShapeX_size_tX_size_tX(const cv::dnn::Net* instance, int layerId, const cv::dnn::MatShape* netInputShape, size_t* weights, size_t* blobs) {
		try {
			instance->getMemoryConsumption(layerId, *netInputShape, *weights, *blobs);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_getMemoryConsumption_const_const_vector_MatShape_X_vector_int_X_vector_size_t_X_vector_size_t_X(const cv::dnn::Net* instance, const std::vector<cv::dnn::MatShape>* netInputShapes, std::vector<int>* layerIds, std::vector<size_t>* weights, std::vector<size_t>* blobs) {
		try {
			instance->getMemoryConsumption(*netInputShapes, *layerIds, *weights, *blobs);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_getMemoryConsumption_const_const_MatShapeX_vector_int_X_vector_size_t_X_vector_size_t_X(const cv::dnn::Net* instance, const cv::dnn::MatShape* netInputShape, std::vector<int>* layerIds, std::vector<size_t>* weights, std::vector<size_t>* blobs) {
		try {
			instance->getMemoryConsumption(*netInputShape, *layerIds, *weights, *blobs);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_Net_enableFusion_bool(cv::dnn::Net* instance, bool fusion) {
		try {
			instance->enableFusion(fusion);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int64_t> cv_dnn_Net_getPerfProfile_vector_double_X(cv::dnn::Net* instance, std::vector<double>* timings) {
		try {
			int64_t ret = instance->getPerfProfile(*timings);
			return Ok(ret);
		} OCVRS_CATCH(Result<int64_t>)
	}
	
	Result<float> cv_dnn_NormalizeBBoxLayer_pnorm_const(const cv::dnn::NormalizeBBoxLayer* instance) {
		try {
			float ret = instance->pnorm;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_dnn_NormalizeBBoxLayer_setPnorm_float(cv::dnn::NormalizeBBoxLayer* instance, float val) {
		try {
			instance->pnorm = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_dnn_NormalizeBBoxLayer_epsilon_const(const cv::dnn::NormalizeBBoxLayer* instance) {
		try {
			float ret = instance->epsilon;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_dnn_NormalizeBBoxLayer_setEpsilon_float(cv::dnn::NormalizeBBoxLayer* instance, float val) {
		try {
			instance->epsilon = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_dnn_NormalizeBBoxLayer_acrossSpatial_const(const cv::dnn::NormalizeBBoxLayer* instance) {
		try {
			bool ret = instance->acrossSpatial;
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_NormalizeBBoxLayer_setAcrossSpatial_bool(cv::dnn::NormalizeBBoxLayer* instance, bool val) {
		try {
			instance->acrossSpatial = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_NormalizeBBoxLayer_delete(cv::dnn::NormalizeBBoxLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::NormalizeBBoxLayer>*> cv_dnn_NormalizeBBoxLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::NormalizeBBoxLayer> ret = cv::dnn::NormalizeBBoxLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::NormalizeBBoxLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::NormalizeBBoxLayer>*>)
	}
	
	void cv_PaddingLayer_delete(cv::dnn::PaddingLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::PaddingLayer>*> cv_dnn_PaddingLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::PaddingLayer> ret = cv::dnn::PaddingLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::PaddingLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::PaddingLayer>*>)
	}
	
	void cv_PermuteLayer_delete(cv::dnn::PermuteLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::PermuteLayer>*> cv_dnn_PermuteLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::PermuteLayer> ret = cv::dnn::PermuteLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::PermuteLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::PermuteLayer>*>)
	}
	
	Result<int> cv_dnn_PoolingLayer_type_const(const cv::dnn::PoolingLayer* instance) {
		try {
			int ret = instance->type;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_PoolingLayer_setType_int(cv::dnn::PoolingLayer* instance, int val) {
		try {
			instance->type = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<size_t>*> cv_dnn_PoolingLayer_kernel_size(cv::dnn::PoolingLayer* instance) {
		try {
			std::vector<size_t> ret = instance->kernel_size;
			return Ok(new std::vector<size_t>(ret));
		} OCVRS_CATCH(Result<std::vector<size_t>*>)
	}
	
	Result_void cv_dnn_PoolingLayer_setKernel_size_vector_size_t_(cv::dnn::PoolingLayer* instance, std::vector<size_t>* val) {
		try {
			instance->kernel_size = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<size_t>*> cv_dnn_PoolingLayer_strides(cv::dnn::PoolingLayer* instance) {
		try {
			std::vector<size_t> ret = instance->strides;
			return Ok(new std::vector<size_t>(ret));
		} OCVRS_CATCH(Result<std::vector<size_t>*>)
	}
	
	Result_void cv_dnn_PoolingLayer_setStrides_vector_size_t_(cv::dnn::PoolingLayer* instance, std::vector<size_t>* val) {
		try {
			instance->strides = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<size_t>*> cv_dnn_PoolingLayer_pads_begin(cv::dnn::PoolingLayer* instance) {
		try {
			std::vector<size_t> ret = instance->pads_begin;
			return Ok(new std::vector<size_t>(ret));
		} OCVRS_CATCH(Result<std::vector<size_t>*>)
	}
	
	Result_void cv_dnn_PoolingLayer_setPads_begin_vector_size_t_(cv::dnn::PoolingLayer* instance, std::vector<size_t>* val) {
		try {
			instance->pads_begin = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<size_t>*> cv_dnn_PoolingLayer_pads_end(cv::dnn::PoolingLayer* instance) {
		try {
			std::vector<size_t> ret = instance->pads_end;
			return Ok(new std::vector<size_t>(ret));
		} OCVRS_CATCH(Result<std::vector<size_t>*>)
	}
	
	Result_void cv_dnn_PoolingLayer_setPads_end_vector_size_t_(cv::dnn::PoolingLayer* instance, std::vector<size_t>* val) {
		try {
			instance->pads_end = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_dnn_PoolingLayer_kernel_const(const cv::dnn::PoolingLayer* instance) {
		try {
			cv::Size ret = instance->kernel;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_dnn_PoolingLayer_setKernel_Size(cv::dnn::PoolingLayer* instance, const cv::Size* val) {
		try {
			instance->kernel = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_dnn_PoolingLayer_stride_const(const cv::dnn::PoolingLayer* instance) {
		try {
			cv::Size ret = instance->stride;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_dnn_PoolingLayer_setStride_Size(cv::dnn::PoolingLayer* instance, const cv::Size* val) {
		try {
			instance->stride = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_dnn_PoolingLayer_pad_const(const cv::dnn::PoolingLayer* instance) {
		try {
			cv::Size ret = instance->pad;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_dnn_PoolingLayer_setPad_Size(cv::dnn::PoolingLayer* instance, const cv::Size* val) {
		try {
			instance->pad = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_PoolingLayer_pad_l_const(const cv::dnn::PoolingLayer* instance) {
		try {
			int ret = instance->pad_l;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_PoolingLayer_setPad_l_int(cv::dnn::PoolingLayer* instance, int val) {
		try {
			instance->pad_l = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_PoolingLayer_pad_t_const(const cv::dnn::PoolingLayer* instance) {
		try {
			int ret = instance->pad_t;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_PoolingLayer_setPad_t_int(cv::dnn::PoolingLayer* instance, int val) {
		try {
			instance->pad_t = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_PoolingLayer_pad_r_const(const cv::dnn::PoolingLayer* instance) {
		try {
			int ret = instance->pad_r;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_PoolingLayer_setPad_r_int(cv::dnn::PoolingLayer* instance, int val) {
		try {
			instance->pad_r = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_PoolingLayer_pad_b_const(const cv::dnn::PoolingLayer* instance) {
		try {
			int ret = instance->pad_b;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_PoolingLayer_setPad_b_int(cv::dnn::PoolingLayer* instance, int val) {
		try {
			instance->pad_b = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_dnn_PoolingLayer_globalPooling_const(const cv::dnn::PoolingLayer* instance) {
		try {
			bool ret = instance->globalPooling;
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_PoolingLayer_setGlobalPooling_bool(cv::dnn::PoolingLayer* instance, bool val) {
		try {
			instance->globalPooling = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_dnn_PoolingLayer_computeMaxIdx_const(const cv::dnn::PoolingLayer* instance) {
		try {
			bool ret = instance->computeMaxIdx;
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_PoolingLayer_setComputeMaxIdx_bool(cv::dnn::PoolingLayer* instance, bool val) {
		try {
			instance->computeMaxIdx = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_dnn_PoolingLayer_padMode_const(const cv::dnn::PoolingLayer* instance) {
		try {
			cv::String ret = instance->padMode;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_PoolingLayer_setPadMode_String(cv::dnn::PoolingLayer* instance, char* val) {
		try {
			instance->padMode = std::string(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_dnn_PoolingLayer_ceilMode_const(const cv::dnn::PoolingLayer* instance) {
		try {
			bool ret = instance->ceilMode;
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_PoolingLayer_setCeilMode_bool(cv::dnn::PoolingLayer* instance, bool val) {
		try {
			instance->ceilMode = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_dnn_PoolingLayer_avePoolPaddedArea_const(const cv::dnn::PoolingLayer* instance) {
		try {
			bool ret = instance->avePoolPaddedArea;
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_PoolingLayer_setAvePoolPaddedArea_bool(cv::dnn::PoolingLayer* instance, bool val) {
		try {
			instance->avePoolPaddedArea = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_dnn_PoolingLayer_pooledSize_const(const cv::dnn::PoolingLayer* instance) {
		try {
			cv::Size ret = instance->pooledSize;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_dnn_PoolingLayer_setPooledSize_Size(cv::dnn::PoolingLayer* instance, const cv::Size* val) {
		try {
			instance->pooledSize = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_dnn_PoolingLayer_spatialScale_const(const cv::dnn::PoolingLayer* instance) {
		try {
			float ret = instance->spatialScale;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_dnn_PoolingLayer_setSpatialScale_float(cv::dnn::PoolingLayer* instance, float val) {
		try {
			instance->spatialScale = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_PoolingLayer_psRoiOutChannels_const(const cv::dnn::PoolingLayer* instance) {
		try {
			int ret = instance->psRoiOutChannels;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_PoolingLayer_setPsRoiOutChannels_int(cv::dnn::PoolingLayer* instance, int val) {
		try {
			instance->psRoiOutChannels = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_PoolingLayer_delete(cv::dnn::PoolingLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::PoolingLayer>*> cv_dnn_PoolingLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::PoolingLayer> ret = cv::dnn::PoolingLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::PoolingLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::PoolingLayer>*>)
	}
	
	Result<float> cv_dnn_PowerLayer_power_const(const cv::dnn::PowerLayer* instance) {
		try {
			float ret = instance->power;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_dnn_PowerLayer_setPower_float(cv::dnn::PowerLayer* instance, float val) {
		try {
			instance->power = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_dnn_PowerLayer_scale_const(const cv::dnn::PowerLayer* instance) {
		try {
			float ret = instance->scale;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_dnn_PowerLayer_setScale_float(cv::dnn::PowerLayer* instance, float val) {
		try {
			instance->scale = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_dnn_PowerLayer_shift_const(const cv::dnn::PowerLayer* instance) {
		try {
			float ret = instance->shift;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_dnn_PowerLayer_setShift_float(cv::dnn::PowerLayer* instance, float val) {
		try {
			instance->shift = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::dnn::PowerLayer>*> cv_dnn_PowerLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::PowerLayer> ret = cv::dnn::PowerLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::PowerLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::PowerLayer>*>)
	}
	
	void cv_PriorBoxLayer_delete(cv::dnn::PriorBoxLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::PriorBoxLayer>*> cv_dnn_PriorBoxLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::PriorBoxLayer> ret = cv::dnn::PriorBoxLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::PriorBoxLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::PriorBoxLayer>*>)
	}
	
	void cv_ProposalLayer_delete(cv::dnn::ProposalLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::ProposalLayer>*> cv_dnn_ProposalLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::ProposalLayer> ret = cv::dnn::ProposalLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::ProposalLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::ProposalLayer>*>)
	}
	
	Result<cv::Ptr<cv::dnn::RNNLayer>*> cv_dnn_RNNLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::RNNLayer> ret = cv::dnn::RNNLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::RNNLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::RNNLayer>*>)
	}
	
	Result_void cv_dnn_RNNLayer_setWeights_const_MatX_const_MatX_const_MatX_const_MatX_const_MatX(cv::dnn::RNNLayer* instance, const cv::Mat* Wxh, const cv::Mat* bh, const cv::Mat* Whh, const cv::Mat* Who, const cv::Mat* bo) {
		try {
			instance->setWeights(*Wxh, *bh, *Whh, *Who, *bo);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_RNNLayer_setProduceHiddenOutput_bool(cv::dnn::RNNLayer* instance, bool produce) {
		try {
			instance->setProduceHiddenOutput(produce);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_dnn_ReLU6Layer_minValue_const(const cv::dnn::ReLU6Layer* instance) {
		try {
			float ret = instance->minValue;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_dnn_ReLU6Layer_setMinValue_float(cv::dnn::ReLU6Layer* instance, float val) {
		try {
			instance->minValue = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_dnn_ReLU6Layer_maxValue_const(const cv::dnn::ReLU6Layer* instance) {
		try {
			float ret = instance->maxValue;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_dnn_ReLU6Layer_setMaxValue_float(cv::dnn::ReLU6Layer* instance, float val) {
		try {
			instance->maxValue = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::dnn::ReLU6Layer>*> cv_dnn_ReLU6Layer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::ReLU6Layer> ret = cv::dnn::ReLU6Layer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::ReLU6Layer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::ReLU6Layer>*>)
	}
	
	Result<float> cv_dnn_ReLULayer_negativeSlope_const(const cv::dnn::ReLULayer* instance) {
		try {
			float ret = instance->negativeSlope;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_dnn_ReLULayer_setNegativeSlope_float(cv::dnn::ReLULayer* instance, float val) {
		try {
			instance->negativeSlope = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::dnn::ReLULayer>*> cv_dnn_ReLULayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::ReLULayer> ret = cv::dnn::ReLULayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::ReLULayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::ReLULayer>*>)
	}
	
	void cv_RegionLayer_delete(cv::dnn::RegionLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::RegionLayer>*> cv_dnn_RegionLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::RegionLayer> ret = cv::dnn::RegionLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::RegionLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::RegionLayer>*>)
	}
	
	void cv_ReorgLayer_delete(cv::dnn::ReorgLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::ReorgLayer>*> cv_dnn_ReorgLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::ReorgLayer> ret = cv::dnn::ReorgLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::ReorgLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::ReorgLayer>*>)
	}
	
	Result<cv::dnn::MatShape*> cv_dnn_ReshapeLayer_newShapeDesc(cv::dnn::ReshapeLayer* instance) {
		try {
			cv::dnn::MatShape ret = instance->newShapeDesc;
			return Ok(new cv::dnn::MatShape(ret));
		} OCVRS_CATCH(Result<cv::dnn::MatShape*>)
	}
	
	Result_void cv_dnn_ReshapeLayer_setNewShapeDesc_MatShape(cv::dnn::ReshapeLayer* instance, cv::dnn::MatShape* val) {
		try {
			instance->newShapeDesc = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Range*> cv_dnn_ReshapeLayer_newShapeRange(cv::dnn::ReshapeLayer* instance) {
		try {
			cv::Range ret = instance->newShapeRange;
			return Ok(new cv::Range(ret));
		} OCVRS_CATCH(Result<cv::Range*>)
	}
	
	Result_void cv_dnn_ReshapeLayer_setNewShapeRange_Range(cv::dnn::ReshapeLayer* instance, cv::Range* val) {
		try {
			instance->newShapeRange = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_ReshapeLayer_delete(cv::dnn::ReshapeLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::ReshapeLayer>*> cv_dnn_ReshapeLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::ReshapeLayer> ret = cv::dnn::ReshapeLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::ReshapeLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::ReshapeLayer>*>)
	}
	
	void cv_ResizeLayer_delete(cv::dnn::ResizeLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::ResizeLayer>*> cv_dnn_ResizeLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::ResizeLayer> ret = cv::dnn::ResizeLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::ResizeLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::ResizeLayer>*>)
	}
	
	Result<bool> cv_dnn_ScaleLayer_hasBias_const(const cv::dnn::ScaleLayer* instance) {
		try {
			bool ret = instance->hasBias;
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_ScaleLayer_setHasBias_bool(cv::dnn::ScaleLayer* instance, bool val) {
		try {
			instance->hasBias = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_ScaleLayer_axis_const(const cv::dnn::ScaleLayer* instance) {
		try {
			int ret = instance->axis;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_ScaleLayer_setAxis_int(cv::dnn::ScaleLayer* instance, int val) {
		try {
			instance->axis = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_ScaleLayer_delete(cv::dnn::ScaleLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::ScaleLayer>*> cv_dnn_ScaleLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::ScaleLayer> ret = cv::dnn::ScaleLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::ScaleLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::ScaleLayer>*>)
	}
	
	void cv_SegmentationModel_delete(cv::dnn::SegmentationModel* instance) {
		delete instance;
	}
	Result<cv::dnn::SegmentationModel*> cv_dnn_SegmentationModel_SegmentationModel_const_StringX_const_StringX(const char* model, const char* config) {
		try {
			cv::dnn::SegmentationModel* ret = new cv::dnn::SegmentationModel(std::string(model), std::string(config));
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::dnn::SegmentationModel*>)
	}
	
	Result<cv::dnn::SegmentationModel*> cv_dnn_SegmentationModel_SegmentationModel_const_NetX(const cv::dnn::Net* network) {
		try {
			cv::dnn::SegmentationModel* ret = new cv::dnn::SegmentationModel(*network);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::dnn::SegmentationModel*>)
	}
	
	Result_void cv_dnn_SegmentationModel_segment_const__InputArrayX_const__OutputArrayX(cv::dnn::SegmentationModel* instance, const cv::_InputArray* frame, const cv::_OutputArray* mask) {
		try {
			instance->segment(*frame, *mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_ShiftLayer_delete(cv::dnn::ShiftLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::Layer>*> cv_dnn_ShiftLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::ShiftLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::Layer>*>)
	}
	
	Result<int> cv_dnn_ShuffleChannelLayer_group_const(const cv::dnn::ShuffleChannelLayer* instance) {
		try {
			int ret = instance->group;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_ShuffleChannelLayer_setGroup_int(cv::dnn::ShuffleChannelLayer* instance, int val) {
		try {
			instance->group = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_ShuffleChannelLayer_delete(cv::dnn::ShuffleChannelLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::Layer>*> cv_dnn_ShuffleChannelLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::Layer> ret = cv::dnn::ShuffleChannelLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::Layer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::Layer>*>)
	}
	
	Result<cv::Ptr<cv::dnn::SigmoidLayer>*> cv_dnn_SigmoidLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::SigmoidLayer> ret = cv::dnn::SigmoidLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::SigmoidLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::SigmoidLayer>*>)
	}
	
	Result<std::vector<std::vector<cv::Range>>*> cv_dnn_SliceLayer_sliceRanges(cv::dnn::SliceLayer* instance) {
		try {
			std::vector<std::vector<cv::Range>> ret = instance->sliceRanges;
			return Ok(new std::vector<std::vector<cv::Range>>(ret));
		} OCVRS_CATCH(Result<std::vector<std::vector<cv::Range>>*>)
	}
	
	Result_void cv_dnn_SliceLayer_setSliceRanges_vector_vector_Range__(cv::dnn::SliceLayer* instance, std::vector<std::vector<cv::Range>>* val) {
		try {
			instance->sliceRanges = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_SliceLayer_axis_const(const cv::dnn::SliceLayer* instance) {
		try {
			int ret = instance->axis;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_SliceLayer_setAxis_int(cv::dnn::SliceLayer* instance, int val) {
		try {
			instance->axis = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_SliceLayer_num_split_const(const cv::dnn::SliceLayer* instance) {
		try {
			int ret = instance->num_split;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_SliceLayer_setNum_split_int(cv::dnn::SliceLayer* instance, int val) {
		try {
			instance->num_split = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_SliceLayer_delete(cv::dnn::SliceLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::SliceLayer>*> cv_dnn_SliceLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::SliceLayer> ret = cv::dnn::SliceLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::SliceLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::SliceLayer>*>)
	}
	
	Result<bool> cv_dnn_SoftmaxLayer_logSoftMax_const(const cv::dnn::SoftmaxLayer* instance) {
		try {
			bool ret = instance->logSoftMax;
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_dnn_SoftmaxLayer_setLogSoftMax_bool(cv::dnn::SoftmaxLayer* instance, bool val) {
		try {
			instance->logSoftMax = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_SoftmaxLayer_delete(cv::dnn::SoftmaxLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::SoftmaxLayer>*> cv_dnn_SoftmaxLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::SoftmaxLayer> ret = cv::dnn::SoftmaxLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::SoftmaxLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::SoftmaxLayer>*>)
	}
	
	Result<int> cv_dnn_SplitLayer_outputsCount_const(const cv::dnn::SplitLayer* instance) {
		try {
			int ret = instance->outputsCount;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_dnn_SplitLayer_setOutputsCount_int(cv::dnn::SplitLayer* instance, int val) {
		try {
			instance->outputsCount = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_SplitLayer_delete(cv::dnn::SplitLayer* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::dnn::SplitLayer>*> cv_dnn_SplitLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::SplitLayer> ret = cv::dnn::SplitLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::SplitLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::SplitLayer>*>)
	}
	
	Result<cv::Ptr<cv::dnn::SwishLayer>*> cv_dnn_SwishLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::SwishLayer> ret = cv::dnn::SwishLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::SwishLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::SwishLayer>*>)
	}
	
	Result<cv::Ptr<cv::dnn::TanHLayer>*> cv_dnn_TanHLayer_create_const_LayerParamsX(const cv::dnn::LayerParams* params) {
		try {
			cv::Ptr<cv::dnn::TanHLayer> ret = cv::dnn::TanHLayer::create(*params);
			return Ok(new cv::Ptr<cv::dnn::TanHLayer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::dnn::TanHLayer>*>)
	}
	
	void cv__Range_delete(cv::dnn::_Range* instance) {
		delete instance;
	}
	Result<cv::dnn::_Range*> cv_dnn__Range__Range_const_RangeX(const cv::Range* r) {
		try {
			cv::dnn::_Range* ret = new cv::dnn::_Range(*r);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::dnn::_Range*>)
	}
	
	Result<cv::dnn::_Range*> cv_dnn__Range__Range_int_int(int start_, int size_) {
		try {
			cv::dnn::_Range* ret = new cv::dnn::_Range(start_, size_);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::dnn::_Range*>)
	}
	
}
