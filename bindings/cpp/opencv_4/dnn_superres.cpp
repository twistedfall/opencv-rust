#include "common.hpp"
#include <opencv2/dnn_superres.hpp>
#include "dnn_superres_types.hpp"

extern "C" {
	void cv_DnnSuperResImpl_delete(cv::dnn_superres::DnnSuperResImpl* instance) {
		delete instance;
	}
	Result<void*> cv_dnn_superres_DnnSuperResImpl_DnnSuperResImpl() {
		try {
			cv::dnn_superres::DnnSuperResImpl* ret = new cv::dnn_superres::DnnSuperResImpl();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_dnn_superres_DnnSuperResImpl_DnnSuperResImpl_const_stringX_int(const char* algo, int scale) {
		try {
			cv::dnn_superres::DnnSuperResImpl* ret = new cv::dnn_superres::DnnSuperResImpl(std::string(algo), scale);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_dnn_superres_DnnSuperResImpl_readModel_const_stringX(void* instance, const char* path) {
		try {
			reinterpret_cast<cv::dnn_superres::DnnSuperResImpl*>(instance)->readModel(std::string(path));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_superres_DnnSuperResImpl_readModel_const_stringX_const_stringX(void* instance, const char* weights, const char* definition) {
		try {
			reinterpret_cast<cv::dnn_superres::DnnSuperResImpl*>(instance)->readModel(std::string(weights), std::string(definition));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_superres_DnnSuperResImpl_setModel_const_stringX_int(void* instance, const char* algo, int scale) {
		try {
			reinterpret_cast<cv::dnn_superres::DnnSuperResImpl*>(instance)->setModel(std::string(algo), scale);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_superres_DnnSuperResImpl_upsample_const__InputArrayX_const__OutputArrayX(void* instance, void* img, void* result) {
		try {
			reinterpret_cast<cv::dnn_superres::DnnSuperResImpl*>(instance)->upsample(*reinterpret_cast<const cv::_InputArray*>(img), *reinterpret_cast<const cv::_OutputArray*>(result));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dnn_superres_DnnSuperResImpl_upsampleMultioutput_const__InputArrayX_vector_Mat_X_const_vector_int_X_const_vector_String_X(void* instance, void* img, void* imgs_new, void* scale_factors, void* node_names) {
		try {
			reinterpret_cast<cv::dnn_superres::DnnSuperResImpl*>(instance)->upsampleMultioutput(*reinterpret_cast<const cv::_InputArray*>(img), *reinterpret_cast<std::vector<cv::Mat>*>(imgs_new), *reinterpret_cast<const std::vector<int>*>(scale_factors), *reinterpret_cast<const std::vector<cv::String>*>(node_names));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_dnn_superres_DnnSuperResImpl_getScale(void* instance) {
		try {
			int ret = reinterpret_cast<cv::dnn_superres::DnnSuperResImpl*>(instance)->getScale();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv_dnn_superres_DnnSuperResImpl_getAlgorithm(void* instance) {
		try {
			std::string ret = reinterpret_cast<cv::dnn_superres::DnnSuperResImpl*>(instance)->getAlgorithm();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
}
