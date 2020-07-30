#include "flann.hpp"
#include "flann_types.hpp"

extern "C" {
	Result<cvflann::flann_distance_t> cvflann_flann_distance_type() {
		try {
			cvflann::flann_distance_t ret = cvflann::flann_distance_type();
			return Ok<cvflann::flann_distance_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cvflann::flann_distance_t>))
	}
	
	Result_void cvflann_set_distance_type_flann_distance_t_int(cvflann::flann_distance_t distance_type, int order) {
		try {
			cvflann::set_distance_type(distance_type, order);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_AutotunedIndexParams_delete(cv::flann::AutotunedIndexParams* instance) {
		delete instance;
	}
	Result<cv::flann::AutotunedIndexParams*> cv_flann_AutotunedIndexParams_AutotunedIndexParams_float_float_float_float(float target_precision, float build_weight, float memory_weight, float sample_fraction) {
		try {
			cv::flann::AutotunedIndexParams* ret = new cv::flann::AutotunedIndexParams(target_precision, build_weight, memory_weight, sample_fraction);
			return Ok<cv::flann::AutotunedIndexParams*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::flann::AutotunedIndexParams*>))
	}
	
	void cv_CompositeIndexParams_delete(cv::flann::CompositeIndexParams* instance) {
		delete instance;
	}
	Result<cv::flann::CompositeIndexParams*> cv_flann_CompositeIndexParams_CompositeIndexParams_int_int_int_flann_centers_init_t_float(int trees, int branching, int iterations, cvflann::flann_centers_init_t centers_init, float cb_index) {
		try {
			cv::flann::CompositeIndexParams* ret = new cv::flann::CompositeIndexParams(trees, branching, iterations, centers_init, cb_index);
			return Ok<cv::flann::CompositeIndexParams*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::flann::CompositeIndexParams*>))
	}
	
	void cv_HierarchicalClusteringIndexParams_delete(cv::flann::HierarchicalClusteringIndexParams* instance) {
		delete instance;
	}
	Result<cv::flann::HierarchicalClusteringIndexParams*> cv_flann_HierarchicalClusteringIndexParams_HierarchicalClusteringIndexParams_int_flann_centers_init_t_int_int(int branching, cvflann::flann_centers_init_t centers_init, int trees, int leaf_size) {
		try {
			cv::flann::HierarchicalClusteringIndexParams* ret = new cv::flann::HierarchicalClusteringIndexParams(branching, centers_init, trees, leaf_size);
			return Ok<cv::flann::HierarchicalClusteringIndexParams*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::flann::HierarchicalClusteringIndexParams*>))
	}
	
	void cv_Index_delete(cv::flann::Index* instance) {
		delete instance;
	}
	Result<cv::flann::Index*> cv_flann_Index_Index() {
		try {
			cv::flann::Index* ret = new cv::flann::Index();
			return Ok<cv::flann::Index*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::flann::Index*>))
	}
	
	Result<cv::flann::Index*> cv_flann_Index_Index_const__InputArrayR_const_IndexParamsR_flann_distance_t(const cv::_InputArray* features, const cv::flann::IndexParams* params, cvflann::flann_distance_t distType) {
		try {
			cv::flann::Index* ret = new cv::flann::Index(*features, *params, distType);
			return Ok<cv::flann::Index*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::flann::Index*>))
	}
	
	Result_void cv_flann_Index_build_const__InputArrayR_const_IndexParamsR_flann_distance_t(cv::flann::Index* instance, const cv::_InputArray* features, const cv::flann::IndexParams* params, cvflann::flann_distance_t distType) {
		try {
			instance->build(*features, *params, distType);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_flann_Index_knnSearch_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_const_SearchParamsR(cv::flann::Index* instance, const cv::_InputArray* query, const cv::_OutputArray* indices, const cv::_OutputArray* dists, int knn, const cv::flann::SearchParams* params) {
		try {
			instance->knnSearch(*query, *indices, *dists, knn, *params);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_flann_Index_radiusSearch_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_int_const_SearchParamsR(cv::flann::Index* instance, const cv::_InputArray* query, const cv::_OutputArray* indices, const cv::_OutputArray* dists, double radius, int maxResults, const cv::flann::SearchParams* params) {
		try {
			int ret = instance->radiusSearch(*query, *indices, *dists, radius, maxResults, *params);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_flann_Index_save_const_const_StringR(const cv::flann::Index* instance, const char* filename) {
		try {
			instance->save(std::string(filename));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_flann_Index_load_const__InputArrayR_const_StringR(cv::flann::Index* instance, const cv::_InputArray* features, const char* filename) {
		try {
			bool ret = instance->load(*features, std::string(filename));
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_flann_Index_release(cv::flann::Index* instance) {
		try {
			instance->release();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cvflann::flann_distance_t> cv_flann_Index_getDistance_const(const cv::flann::Index* instance) {
		try {
			cvflann::flann_distance_t ret = instance->getDistance();
			return Ok<cvflann::flann_distance_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cvflann::flann_distance_t>))
	}
	
	Result<cvflann::flann_algorithm_t> cv_flann_Index_getAlgorithm_const(const cv::flann::Index* instance) {
		try {
			cvflann::flann_algorithm_t ret = instance->getAlgorithm();
			return Ok<cvflann::flann_algorithm_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cvflann::flann_algorithm_t>))
	}
	
	Result<void*> cv_flann_IndexParams_getPropParams(cv::flann::IndexParams* instance) {
		try {
			void* ret = instance->params;
			return Ok<void*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_flann_IndexParams_setPropParams_voidX(cv::flann::IndexParams* instance, void* val) {
		try {
			instance->params = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_IndexParams_delete(cv::flann::IndexParams* instance) {
		delete instance;
	}
	Result<cv::flann::IndexParams*> cv_flann_IndexParams_IndexParams() {
		try {
			cv::flann::IndexParams* ret = new cv::flann::IndexParams();
			return Ok<cv::flann::IndexParams*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::flann::IndexParams*>))
	}
	
	Result<void*> cv_flann_IndexParams_getString_const_const_StringR_const_StringR(const cv::flann::IndexParams* instance, const char* key, const char* defaultVal) {
		try {
			cv::String ret = instance->getString(std::string(key), std::string(defaultVal));
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<int> cv_flann_IndexParams_getInt_const_const_StringR_int(const cv::flann::IndexParams* instance, const char* key, int defaultVal) {
		try {
			int ret = instance->getInt(std::string(key), defaultVal);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<double> cv_flann_IndexParams_getDouble_const_const_StringR_double(const cv::flann::IndexParams* instance, const char* key, double defaultVal) {
		try {
			double ret = instance->getDouble(std::string(key), defaultVal);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_flann_IndexParams_setString_const_StringR_const_StringR(cv::flann::IndexParams* instance, const char* key, const char* value) {
		try {
			instance->setString(std::string(key), std::string(value));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_flann_IndexParams_setInt_const_StringR_int(cv::flann::IndexParams* instance, const char* key, int value) {
		try {
			instance->setInt(std::string(key), value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_flann_IndexParams_setDouble_const_StringR_double(cv::flann::IndexParams* instance, const char* key, double value) {
		try {
			instance->setDouble(std::string(key), value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_flann_IndexParams_setFloat_const_StringR_float(cv::flann::IndexParams* instance, const char* key, float value) {
		try {
			instance->setFloat(std::string(key), value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_flann_IndexParams_setBool_const_StringR_bool(cv::flann::IndexParams* instance, const char* key, bool value) {
		try {
			instance->setBool(std::string(key), value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_flann_IndexParams_setAlgorithm_int(cv::flann::IndexParams* instance, int value) {
		try {
			instance->setAlgorithm(value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_flann_IndexParams_getAll_const_vector_String_R_vector_FlannIndexType_R_vector_String_R_vector_double_R(const cv::flann::IndexParams* instance, std::vector<cv::String>* names, std::vector<cv::flann::FlannIndexType>* types, std::vector<cv::String>* strValues, std::vector<double>* numValues) {
		try {
			instance->getAll(*names, *types, *strValues, *numValues);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_KDTreeIndexParams_delete(cv::flann::KDTreeIndexParams* instance) {
		delete instance;
	}
	Result<cv::flann::KDTreeIndexParams*> cv_flann_KDTreeIndexParams_KDTreeIndexParams_int(int trees) {
		try {
			cv::flann::KDTreeIndexParams* ret = new cv::flann::KDTreeIndexParams(trees);
			return Ok<cv::flann::KDTreeIndexParams*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::flann::KDTreeIndexParams*>))
	}
	
	void cv_KMeansIndexParams_delete(cv::flann::KMeansIndexParams* instance) {
		delete instance;
	}
	Result<cv::flann::KMeansIndexParams*> cv_flann_KMeansIndexParams_KMeansIndexParams_int_int_flann_centers_init_t_float(int branching, int iterations, cvflann::flann_centers_init_t centers_init, float cb_index) {
		try {
			cv::flann::KMeansIndexParams* ret = new cv::flann::KMeansIndexParams(branching, iterations, centers_init, cb_index);
			return Ok<cv::flann::KMeansIndexParams*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::flann::KMeansIndexParams*>))
	}
	
	void cv_LinearIndexParams_delete(cv::flann::LinearIndexParams* instance) {
		delete instance;
	}
	Result<cv::flann::LinearIndexParams*> cv_flann_LinearIndexParams_LinearIndexParams() {
		try {
			cv::flann::LinearIndexParams* ret = new cv::flann::LinearIndexParams();
			return Ok<cv::flann::LinearIndexParams*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::flann::LinearIndexParams*>))
	}
	
	void cv_LshIndexParams_delete(cv::flann::LshIndexParams* instance) {
		delete instance;
	}
	Result<cv::flann::LshIndexParams*> cv_flann_LshIndexParams_LshIndexParams_int_int_int(int table_number, int key_size, int multi_probe_level) {
		try {
			cv::flann::LshIndexParams* ret = new cv::flann::LshIndexParams(table_number, key_size, multi_probe_level);
			return Ok<cv::flann::LshIndexParams*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::flann::LshIndexParams*>))
	}
	
	void cv_SavedIndexParams_delete(cv::flann::SavedIndexParams* instance) {
		delete instance;
	}
	Result<cv::flann::SavedIndexParams*> cv_flann_SavedIndexParams_SavedIndexParams_const_StringR(const char* filename) {
		try {
			cv::flann::SavedIndexParams* ret = new cv::flann::SavedIndexParams(std::string(filename));
			return Ok<cv::flann::SavedIndexParams*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::flann::SavedIndexParams*>))
	}
	
	void cv_SearchParams_delete(cv::flann::SearchParams* instance) {
		delete instance;
	}
	Result<cv::flann::SearchParams*> cv_flann_SearchParams_SearchParams_int_float_bool(int checks, float eps, bool sorted) {
		try {
			cv::flann::SearchParams* ret = new cv::flann::SearchParams(checks, eps, sorted);
			return Ok<cv::flann::SearchParams*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::flann::SearchParams*>))
	}
	
}
