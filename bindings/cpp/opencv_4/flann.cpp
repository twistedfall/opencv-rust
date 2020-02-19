#include "common.hpp"
#include <opencv2/flann.hpp>
#include "flann_types.hpp"

extern "C" {
	Result<cvflann::flann_distance_t> cvflann_flann_distance_type() {
		try {
			cvflann::flann_distance_t ret = cvflann::flann_distance_type();
			return Ok<cvflann::flann_distance_t>(ret);
		} OCVRS_CATCH(Result<cvflann::flann_distance_t>)
	}
	
	Result_void cvflann_set_distance_type_flann_distance_t_int(cvflann::flann_distance_t distance_type, int order) {
		try {
			cvflann::set_distance_type(distance_type, order);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_AutotunedIndexParams_delete(cv::flann::AutotunedIndexParams* instance) {
		delete instance;
	}
	Result<void*> cv_flann_AutotunedIndexParams_AutotunedIndexParams_float_float_float_float(float target_precision, float build_weight, float memory_weight, float sample_fraction) {
		try {
			cv::flann::AutotunedIndexParams* ret = new cv::flann::AutotunedIndexParams(target_precision, build_weight, memory_weight, sample_fraction);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_CompositeIndexParams_delete(cv::flann::CompositeIndexParams* instance) {
		delete instance;
	}
	Result<void*> cv_flann_CompositeIndexParams_CompositeIndexParams_int_int_int_flann_centers_init_t_float(int trees, int branching, int iterations, cvflann::flann_centers_init_t centers_init, float cb_index) {
		try {
			cv::flann::CompositeIndexParams* ret = new cv::flann::CompositeIndexParams(trees, branching, iterations, centers_init, cb_index);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_HierarchicalClusteringIndexParams_delete(cv::flann::HierarchicalClusteringIndexParams* instance) {
		delete instance;
	}
	Result<void*> cv_flann_HierarchicalClusteringIndexParams_HierarchicalClusteringIndexParams_int_flann_centers_init_t_int_int(int branching, cvflann::flann_centers_init_t centers_init, int trees, int leaf_size) {
		try {
			cv::flann::HierarchicalClusteringIndexParams* ret = new cv::flann::HierarchicalClusteringIndexParams(branching, centers_init, trees, leaf_size);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_Index_delete(cv::flann::Index* instance) {
		delete instance;
	}
	Result<void*> cv_flann_Index_Index() {
		try {
			cv::flann::Index* ret = new cv::flann::Index();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_flann_Index_Index_const__InputArrayX_const_IndexParamsX_flann_distance_t(void* features, void* params, cvflann::flann_distance_t distType) {
		try {
			cv::flann::Index* ret = new cv::flann::Index(*reinterpret_cast<const cv::_InputArray*>(features), *reinterpret_cast<const cv::flann::IndexParams*>(params), distType);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_flann_Index_build_const__InputArrayX_const_IndexParamsX_flann_distance_t(void* instance, void* features, void* params, cvflann::flann_distance_t distType) {
		try {
			reinterpret_cast<cv::flann::Index*>(instance)->build(*reinterpret_cast<const cv::_InputArray*>(features), *reinterpret_cast<const cv::flann::IndexParams*>(params), distType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_flann_Index_knnSearch_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_int_const_SearchParamsX(void* instance, void* query, void* indices, void* dists, int knn, void* params) {
		try {
			reinterpret_cast<cv::flann::Index*>(instance)->knnSearch(*reinterpret_cast<const cv::_InputArray*>(query), *reinterpret_cast<const cv::_OutputArray*>(indices), *reinterpret_cast<const cv::_OutputArray*>(dists), knn, *reinterpret_cast<const cv::flann::SearchParams*>(params));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_flann_Index_radiusSearch_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_double_int_const_SearchParamsX(void* instance, void* query, void* indices, void* dists, double radius, int maxResults, void* params) {
		try {
			int ret = reinterpret_cast<cv::flann::Index*>(instance)->radiusSearch(*reinterpret_cast<const cv::_InputArray*>(query), *reinterpret_cast<const cv::_OutputArray*>(indices), *reinterpret_cast<const cv::_OutputArray*>(dists), radius, maxResults, *reinterpret_cast<const cv::flann::SearchParams*>(params));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_flann_Index_save_const_const_StringX(void* instance, const char* filename) {
		try {
			reinterpret_cast<cv::flann::Index*>(instance)->save(std::string(filename));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_flann_Index_load_const__InputArrayX_const_StringX(void* instance, void* features, const char* filename) {
		try {
			bool ret = reinterpret_cast<cv::flann::Index*>(instance)->load(*reinterpret_cast<const cv::_InputArray*>(features), std::string(filename));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_flann_Index_release(void* instance) {
		try {
			reinterpret_cast<cv::flann::Index*>(instance)->release();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cvflann::flann_distance_t> cv_flann_Index_getDistance_const(void* instance) {
		try {
			cvflann::flann_distance_t ret = reinterpret_cast<cv::flann::Index*>(instance)->getDistance();
			return Ok<cvflann::flann_distance_t>(ret);
		} OCVRS_CATCH(Result<cvflann::flann_distance_t>)
	}
	
	Result<cvflann::flann_algorithm_t> cv_flann_Index_getAlgorithm_const(void* instance) {
		try {
			cvflann::flann_algorithm_t ret = reinterpret_cast<cv::flann::Index*>(instance)->getAlgorithm();
			return Ok<cvflann::flann_algorithm_t>(ret);
		} OCVRS_CATCH(Result<cvflann::flann_algorithm_t>)
	}
	
	Result<void*> cv_flann_IndexParams_params(void* instance) {
		try {
			void* ret = reinterpret_cast<cv::flann::IndexParams*>(instance)->params;
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_flann_IndexParams_setParams_voidX(void* instance, void* val) {
		try {
			reinterpret_cast<cv::flann::IndexParams*>(instance)->params = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_IndexParams_delete(cv::flann::IndexParams* instance) {
		delete instance;
	}
	Result<void*> cv_flann_IndexParams_IndexParams() {
		try {
			cv::flann::IndexParams* ret = new cv::flann::IndexParams();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_flann_IndexParams_getString_const_const_StringX_const_StringX(void* instance, const char* key, const char* defaultVal) {
		try {
			cv::String ret = reinterpret_cast<cv::flann::IndexParams*>(instance)->getString(std::string(key), std::string(defaultVal));
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_flann_IndexParams_getInt_const_const_StringX_int(void* instance, const char* key, int defaultVal) {
		try {
			int ret = reinterpret_cast<cv::flann::IndexParams*>(instance)->getInt(std::string(key), defaultVal);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<double> cv_flann_IndexParams_getDouble_const_const_StringX_double(void* instance, const char* key, double defaultVal) {
		try {
			double ret = reinterpret_cast<cv::flann::IndexParams*>(instance)->getDouble(std::string(key), defaultVal);
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_flann_IndexParams_setString_const_StringX_const_StringX(void* instance, const char* key, const char* value) {
		try {
			reinterpret_cast<cv::flann::IndexParams*>(instance)->setString(std::string(key), std::string(value));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_flann_IndexParams_setInt_const_StringX_int(void* instance, const char* key, int value) {
		try {
			reinterpret_cast<cv::flann::IndexParams*>(instance)->setInt(std::string(key), value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_flann_IndexParams_setDouble_const_StringX_double(void* instance, const char* key, double value) {
		try {
			reinterpret_cast<cv::flann::IndexParams*>(instance)->setDouble(std::string(key), value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_flann_IndexParams_setFloat_const_StringX_float(void* instance, const char* key, float value) {
		try {
			reinterpret_cast<cv::flann::IndexParams*>(instance)->setFloat(std::string(key), value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_flann_IndexParams_setBool_const_StringX_bool(void* instance, const char* key, bool value) {
		try {
			reinterpret_cast<cv::flann::IndexParams*>(instance)->setBool(std::string(key), value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_flann_IndexParams_setAlgorithm_int(void* instance, int value) {
		try {
			reinterpret_cast<cv::flann::IndexParams*>(instance)->setAlgorithm(value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_flann_IndexParams_getAll_const_vector_String_X_vector_FlannIndexType_X_vector_String_X_vector_double_X(void* instance, void* names, void* types, void* strValues, void* numValues) {
		try {
			reinterpret_cast<cv::flann::IndexParams*>(instance)->getAll(*reinterpret_cast<std::vector<cv::String>*>(names), *reinterpret_cast<std::vector<cv::flann::FlannIndexType>*>(types), *reinterpret_cast<std::vector<cv::String>*>(strValues), *reinterpret_cast<std::vector<double>*>(numValues));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_KDTreeIndexParams_delete(cv::flann::KDTreeIndexParams* instance) {
		delete instance;
	}
	Result<void*> cv_flann_KDTreeIndexParams_KDTreeIndexParams_int(int trees) {
		try {
			cv::flann::KDTreeIndexParams* ret = new cv::flann::KDTreeIndexParams(trees);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_KMeansIndexParams_delete(cv::flann::KMeansIndexParams* instance) {
		delete instance;
	}
	Result<void*> cv_flann_KMeansIndexParams_KMeansIndexParams_int_int_flann_centers_init_t_float(int branching, int iterations, cvflann::flann_centers_init_t centers_init, float cb_index) {
		try {
			cv::flann::KMeansIndexParams* ret = new cv::flann::KMeansIndexParams(branching, iterations, centers_init, cb_index);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_LinearIndexParams_delete(cv::flann::LinearIndexParams* instance) {
		delete instance;
	}
	Result<void*> cv_flann_LinearIndexParams_LinearIndexParams() {
		try {
			cv::flann::LinearIndexParams* ret = new cv::flann::LinearIndexParams();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_LshIndexParams_delete(cv::flann::LshIndexParams* instance) {
		delete instance;
	}
	Result<void*> cv_flann_LshIndexParams_LshIndexParams_int_int_int(int table_number, int key_size, int multi_probe_level) {
		try {
			cv::flann::LshIndexParams* ret = new cv::flann::LshIndexParams(table_number, key_size, multi_probe_level);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_SavedIndexParams_delete(cv::flann::SavedIndexParams* instance) {
		delete instance;
	}
	Result<void*> cv_flann_SavedIndexParams_SavedIndexParams_const_StringX(const char* filename) {
		try {
			cv::flann::SavedIndexParams* ret = new cv::flann::SavedIndexParams(std::string(filename));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_SearchParams_delete(cv::flann::SearchParams* instance) {
		delete instance;
	}
	Result<void*> cv_flann_SearchParams_SearchParams_int_float_bool(int checks, float eps, bool sorted) {
		try {
			cv::flann::SearchParams* ret = new cv::flann::SearchParams(checks, eps, sorted);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
}
