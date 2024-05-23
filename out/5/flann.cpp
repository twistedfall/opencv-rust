#include "ocvrs_common.hpp"
#include <opencv2/flann.hpp>
#include "flann_types.hpp"

extern "C" {
	// flann_distance_type()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann.hpp:61
	// ("cvflann::flann_distance_type", vec![(pred!(mut, [], []), _)]),
	void cvflann_flann_distance_type(Result<cvflann::flann_distance_t>* ocvrs_return) {
		try {
			cvflann::flann_distance_t ret = cvflann::flann_distance_type();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// set_distance_type(flann_distance_t, int)(Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann.hpp:62
	// ("cvflann::set_distance_type", vec![(pred!(mut, ["distance_type", "order"], ["cvflann::flann_distance_t", "int"]), _)]),
	void cvflann_set_distance_type_flann_distance_t_int(cvflann::flann_distance_t distance_type, int order, ResultVoid* ocvrs_return) {
		try {
			cvflann::set_distance_type(distance_type, order);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// AutotunedIndexParams(float, float, float, float)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:118
	// ("cv::flann::AutotunedIndexParams::AutotunedIndexParams", vec![(pred!(mut, ["target_precision", "build_weight", "memory_weight", "sample_fraction"], ["float", "float", "float", "float"]), _)]),
	void cv_flann_AutotunedIndexParams_AutotunedIndexParams_float_float_float_float(float target_precision, float build_weight, float memory_weight, float sample_fraction, Result<cv::flann::AutotunedIndexParams*>* ocvrs_return) {
		try {
			cv::flann::AutotunedIndexParams* ret = new cv::flann::AutotunedIndexParams(target_precision, build_weight, memory_weight, sample_fraction);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::flann::AutotunedIndexParams::AutotunedIndexParams() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:118
	// ("cv::flann::AutotunedIndexParams::AutotunedIndexParams", vec![(pred!(mut, [], []), _)]),
	void cv_flann_AutotunedIndexParams_AutotunedIndexParams(Result<cv::flann::AutotunedIndexParams*>* ocvrs_return) {
		try {
			cv::flann::AutotunedIndexParams* ret = new cv::flann::AutotunedIndexParams();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::flann::AutotunedIndexParams::to_IndexParams() generated
	// ("cv::flann::AutotunedIndexParams::to_IndexParams", vec![(pred!(mut, [], []), _)]),
	cv::flann::IndexParams* cv_flann_AutotunedIndexParams_to_IndexParams(cv::flann::AutotunedIndexParams* instance) {
			return dynamic_cast<cv::flann::IndexParams*>(instance);
	}

	// cv::flann::AutotunedIndexParams::delete() generated
	// ("cv::flann::AutotunedIndexParams::delete", vec![(pred!(mut, [], []), _)]),
	void cv_flann_AutotunedIndexParams_delete(cv::flann::AutotunedIndexParams* instance) {
			delete instance;
	}

	// CompositeIndexParams(int, int, int, cvflann::flann_centers_init_t, float)(Primitive, Primitive, Primitive, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:112
	// ("cv::flann::CompositeIndexParams::CompositeIndexParams", vec![(pred!(mut, ["trees", "branching", "iterations", "centers_init", "cb_index"], ["int", "int", "int", "cvflann::flann_centers_init_t", "float"]), _)]),
	void cv_flann_CompositeIndexParams_CompositeIndexParams_int_int_int_flann_centers_init_t_float(int trees, int branching, int iterations, cvflann::flann_centers_init_t centers_init, float cb_index, Result<cv::flann::CompositeIndexParams*>* ocvrs_return) {
		try {
			cv::flann::CompositeIndexParams* ret = new cv::flann::CompositeIndexParams(trees, branching, iterations, centers_init, cb_index);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::flann::CompositeIndexParams::CompositeIndexParams() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:112
	// ("cv::flann::CompositeIndexParams::CompositeIndexParams", vec![(pred!(mut, [], []), _)]),
	void cv_flann_CompositeIndexParams_CompositeIndexParams(Result<cv::flann::CompositeIndexParams*>* ocvrs_return) {
		try {
			cv::flann::CompositeIndexParams* ret = new cv::flann::CompositeIndexParams();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::flann::CompositeIndexParams::to_IndexParams() generated
	// ("cv::flann::CompositeIndexParams::to_IndexParams", vec![(pred!(mut, [], []), _)]),
	cv::flann::IndexParams* cv_flann_CompositeIndexParams_to_IndexParams(cv::flann::CompositeIndexParams* instance) {
			return dynamic_cast<cv::flann::IndexParams*>(instance);
	}

	// cv::flann::CompositeIndexParams::delete() generated
	// ("cv::flann::CompositeIndexParams::delete", vec![(pred!(mut, [], []), _)]),
	void cv_flann_CompositeIndexParams_delete(cv::flann::CompositeIndexParams* instance) {
			delete instance;
	}

	// HierarchicalClusteringIndexParams(int, cvflann::flann_centers_init_t, int, int)(Primitive, Enum, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:124
	// ("cv::flann::HierarchicalClusteringIndexParams::HierarchicalClusteringIndexParams", vec![(pred!(mut, ["branching", "centers_init", "trees", "leaf_size"], ["int", "cvflann::flann_centers_init_t", "int", "int"]), _)]),
	void cv_flann_HierarchicalClusteringIndexParams_HierarchicalClusteringIndexParams_int_flann_centers_init_t_int_int(int branching, cvflann::flann_centers_init_t centers_init, int trees, int leaf_size, Result<cv::flann::HierarchicalClusteringIndexParams*>* ocvrs_return) {
		try {
			cv::flann::HierarchicalClusteringIndexParams* ret = new cv::flann::HierarchicalClusteringIndexParams(branching, centers_init, trees, leaf_size);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::flann::HierarchicalClusteringIndexParams::HierarchicalClusteringIndexParams() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:124
	// ("cv::flann::HierarchicalClusteringIndexParams::HierarchicalClusteringIndexParams", vec![(pred!(mut, [], []), _)]),
	void cv_flann_HierarchicalClusteringIndexParams_HierarchicalClusteringIndexParams(Result<cv::flann::HierarchicalClusteringIndexParams*>* ocvrs_return) {
		try {
			cv::flann::HierarchicalClusteringIndexParams* ret = new cv::flann::HierarchicalClusteringIndexParams();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::flann::HierarchicalClusteringIndexParams::to_IndexParams() generated
	// ("cv::flann::HierarchicalClusteringIndexParams::to_IndexParams", vec![(pred!(mut, [], []), _)]),
	cv::flann::IndexParams* cv_flann_HierarchicalClusteringIndexParams_to_IndexParams(cv::flann::HierarchicalClusteringIndexParams* instance) {
			return dynamic_cast<cv::flann::IndexParams*>(instance);
	}

	// cv::flann::HierarchicalClusteringIndexParams::delete() generated
	// ("cv::flann::HierarchicalClusteringIndexParams::delete", vec![(pred!(mut, [], []), _)]),
	void cv_flann_HierarchicalClusteringIndexParams_delete(cv::flann::HierarchicalClusteringIndexParams* instance) {
			delete instance;
	}

	// Index()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:153
	// ("cv::flann::Index::Index", vec![(pred!(mut, [], []), _)]),
	void cv_flann_Index_Index(Result<cv::flann::Index*>* ocvrs_return) {
		try {
			cv::flann::Index* ret = new cv::flann::Index();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Index(InputArray, const IndexParams &, cvflann::flann_distance_t)(InputArray, TraitClass, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:154
	// ("cv::flann::Index::Index", vec![(pred!(mut, ["features", "params", "distType"], ["const cv::_InputArray*", "const cv::flann::IndexParams*", "cvflann::flann_distance_t"]), _)]),
	void cv_flann_Index_Index_const__InputArrayR_const_IndexParamsR_flann_distance_t(const cv::_InputArray* features, const cv::flann::IndexParams* params, cvflann::flann_distance_t distType, Result<cv::flann::Index*>* ocvrs_return) {
		try {
			cv::flann::Index* ret = new cv::flann::Index(*features, *params, distType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::flann::Index::Index(InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:154
	// ("cv::flann::Index::Index", vec![(pred!(mut, ["features", "params"], ["const cv::_InputArray*", "const cv::flann::IndexParams*"]), _)]),
	void cv_flann_Index_Index_const__InputArrayR_const_IndexParamsR(const cv::_InputArray* features, const cv::flann::IndexParams* params, Result<cv::flann::Index*>* ocvrs_return) {
		try {
			cv::flann::Index* ret = new cv::flann::Index(*features, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// build(InputArray, const IndexParams &, cvflann::flann_distance_t)(InputArray, TraitClass, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:157
	// ("cv::flann::Index::build", vec![(pred!(mut, ["features", "params", "distType"], ["const cv::_InputArray*", "const cv::flann::IndexParams*", "cvflann::flann_distance_t"]), _)]),
	void cv_flann_Index_build_const__InputArrayR_const_IndexParamsR_flann_distance_t(cv::flann::Index* instance, const cv::_InputArray* features, const cv::flann::IndexParams* params, cvflann::flann_distance_t distType, ResultVoid* ocvrs_return) {
		try {
			instance->build(*features, *params, distType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::flann::Index::build(InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:157
	// ("cv::flann::Index::build", vec![(pred!(mut, ["features", "params"], ["const cv::_InputArray*", "const cv::flann::IndexParams*"]), _)]),
	void cv_flann_Index_build_const__InputArrayR_const_IndexParamsR(cv::flann::Index* instance, const cv::_InputArray* features, const cv::flann::IndexParams* params, ResultVoid* ocvrs_return) {
		try {
			instance->build(*features, *params);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// knnSearch(InputArray, OutputArray, OutputArray, int, const SearchParams &)(InputArray, OutputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:158
	// ("cv::flann::Index::knnSearch", vec![(pred!(mut, ["query", "indices", "dists", "knn", "params"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "const cv::flann::SearchParams*"]), _)]),
	void cv_flann_Index_knnSearch_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_const_SearchParamsR(cv::flann::Index* instance, const cv::_InputArray* query, const cv::_OutputArray* indices, const cv::_OutputArray* dists, int knn, const cv::flann::SearchParams* params, ResultVoid* ocvrs_return) {
		try {
			instance->knnSearch(*query, *indices, *dists, knn, *params);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::flann::Index::knnSearch(InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:158
	// ("cv::flann::Index::knnSearch", vec![(pred!(mut, ["query", "indices", "dists", "knn"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_flann_Index_knnSearch_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int(cv::flann::Index* instance, const cv::_InputArray* query, const cv::_OutputArray* indices, const cv::_OutputArray* dists, int knn, ResultVoid* ocvrs_return) {
		try {
			instance->knnSearch(*query, *indices, *dists, knn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// radiusSearch(InputArray, OutputArray, OutputArray, double, int, const SearchParams &)(InputArray, OutputArray, OutputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:161
	// ("cv::flann::Index::radiusSearch", vec![(pred!(mut, ["query", "indices", "dists", "radius", "maxResults", "params"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "int", "const cv::flann::SearchParams*"]), _)]),
	void cv_flann_Index_radiusSearch_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_int_const_SearchParamsR(cv::flann::Index* instance, const cv::_InputArray* query, const cv::_OutputArray* indices, const cv::_OutputArray* dists, double radius, int maxResults, const cv::flann::SearchParams* params, Result<int>* ocvrs_return) {
		try {
			int ret = instance->radiusSearch(*query, *indices, *dists, radius, maxResults, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::flann::Index::radiusSearch(InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:161
	// ("cv::flann::Index::radiusSearch", vec![(pred!(mut, ["query", "indices", "dists", "radius", "maxResults"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "int"]), _)]),
	void cv_flann_Index_radiusSearch_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_int(cv::flann::Index* instance, const cv::_InputArray* query, const cv::_OutputArray* indices, const cv::_OutputArray* dists, double radius, int maxResults, Result<int>* ocvrs_return) {
		try {
			int ret = instance->radiusSearch(*query, *indices, *dists, radius, maxResults);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// save(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:165
	// ("cv::flann::Index::save", vec![(pred!(const, ["filename"], ["const cv::String*"]), _)]),
	void cv_flann_Index_save_const_const_StringR(const cv::flann::Index* instance, const char* filename, ResultVoid* ocvrs_return) {
		try {
			instance->save(std::string(filename));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// load(InputArray, const String &)(InputArray, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:166
	// ("cv::flann::Index::load", vec![(pred!(mut, ["features", "filename"], ["const cv::_InputArray*", "const cv::String*"]), _)]),
	void cv_flann_Index_load_const__InputArrayR_const_StringR(cv::flann::Index* instance, const cv::_InputArray* features, const char* filename, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->load(*features, std::string(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// release()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:167
	// ("cv::flann::Index::release", vec![(pred!(mut, [], []), _)]),
	void cv_flann_Index_release(cv::flann::Index* instance, ResultVoid* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDistance()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:168
	// ("cv::flann::Index::getDistance", vec![(pred!(const, [], []), _)]),
	void cv_flann_Index_getDistance_const(const cv::flann::Index* instance, Result<cvflann::flann_distance_t>* ocvrs_return) {
		try {
			cvflann::flann_distance_t ret = instance->getDistance();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAlgorithm()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:169
	// ("cv::flann::Index::getAlgorithm", vec![(pred!(const, [], []), _)]),
	void cv_flann_Index_getAlgorithm_const(const cv::flann::Index* instance, Result<cvflann::flann_algorithm_t>* ocvrs_return) {
		try {
			cvflann::flann_algorithm_t ret = instance->getAlgorithm();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::flann::Index::delete() generated
	// ("cv::flann::Index::delete", vec![(pred!(mut, [], []), _)]),
	void cv_flann_Index_delete(cv::flann::Index* instance) {
			delete instance;
	}

	// IndexParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:73
	// ("cv::flann::IndexParams::IndexParams", vec![(pred!(mut, [], []), _)]),
	void cv_flann_IndexParams_IndexParams(Result<cv::flann::IndexParams*>* ocvrs_return) {
		try {
			cv::flann::IndexParams* ret = new cv::flann::IndexParams();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getString(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:76
	// ("cv::flann::IndexParams::getString", vec![(pred!(const, ["key", "defaultVal"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_flann_IndexParams_getString_const_const_StringR_const_StringR(const cv::flann::IndexParams* instance, const char* key, const char* defaultVal, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getString(std::string(key), std::string(defaultVal));
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::flann::IndexParams::getString(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:76
	// ("cv::flann::IndexParams::getString", vec![(pred!(const, ["key"], ["const cv::String*"]), _)]),
	void cv_flann_IndexParams_getString_const_const_StringR(const cv::flann::IndexParams* instance, const char* key, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getString(std::string(key));
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getInt(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:77
	// ("cv::flann::IndexParams::getInt", vec![(pred!(const, ["key", "defaultVal"], ["const cv::String*", "int"]), _)]),
	void cv_flann_IndexParams_getInt_const_const_StringR_int(const cv::flann::IndexParams* instance, const char* key, int defaultVal, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getInt(std::string(key), defaultVal);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::flann::IndexParams::getInt(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:77
	// ("cv::flann::IndexParams::getInt", vec![(pred!(const, ["key"], ["const cv::String*"]), _)]),
	void cv_flann_IndexParams_getInt_const_const_StringR(const cv::flann::IndexParams* instance, const char* key, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getInt(std::string(key));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDouble(const String &, double)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:78
	// ("cv::flann::IndexParams::getDouble", vec![(pred!(const, ["key", "defaultVal"], ["const cv::String*", "double"]), _)]),
	void cv_flann_IndexParams_getDouble_const_const_StringR_double(const cv::flann::IndexParams* instance, const char* key, double defaultVal, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getDouble(std::string(key), defaultVal);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::flann::IndexParams::getDouble(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:78
	// ("cv::flann::IndexParams::getDouble", vec![(pred!(const, ["key"], ["const cv::String*"]), _)]),
	void cv_flann_IndexParams_getDouble_const_const_StringR(const cv::flann::IndexParams* instance, const char* key, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getDouble(std::string(key));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setString(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:80
	// ("cv::flann::IndexParams::setString", vec![(pred!(mut, ["key", "value"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_flann_IndexParams_setString_const_StringR_const_StringR(cv::flann::IndexParams* instance, const char* key, const char* value, ResultVoid* ocvrs_return) {
		try {
			instance->setString(std::string(key), std::string(value));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInt(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:81
	// ("cv::flann::IndexParams::setInt", vec![(pred!(mut, ["key", "value"], ["const cv::String*", "int"]), _)]),
	void cv_flann_IndexParams_setInt_const_StringR_int(cv::flann::IndexParams* instance, const char* key, int value, ResultVoid* ocvrs_return) {
		try {
			instance->setInt(std::string(key), value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDouble(const String &, double)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:82
	// ("cv::flann::IndexParams::setDouble", vec![(pred!(mut, ["key", "value"], ["const cv::String*", "double"]), _)]),
	void cv_flann_IndexParams_setDouble_const_StringR_double(cv::flann::IndexParams* instance, const char* key, double value, ResultVoid* ocvrs_return) {
		try {
			instance->setDouble(std::string(key), value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFloat(const String &, float)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:83
	// ("cv::flann::IndexParams::setFloat", vec![(pred!(mut, ["key", "value"], ["const cv::String*", "float"]), _)]),
	void cv_flann_IndexParams_setFloat_const_StringR_float(cv::flann::IndexParams* instance, const char* key, float value, ResultVoid* ocvrs_return) {
		try {
			instance->setFloat(std::string(key), value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBool(const String &, bool)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:84
	// ("cv::flann::IndexParams::setBool", vec![(pred!(mut, ["key", "value"], ["const cv::String*", "bool"]), _)]),
	void cv_flann_IndexParams_setBool_const_StringR_bool(cv::flann::IndexParams* instance, const char* key, bool value, ResultVoid* ocvrs_return) {
		try {
			instance->setBool(std::string(key), value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setAlgorithm(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:85
	// ("cv::flann::IndexParams::setAlgorithm", vec![(pred!(mut, ["value"], ["int"]), _)]),
	void cv_flann_IndexParams_setAlgorithm_int(cv::flann::IndexParams* instance, int value, ResultVoid* ocvrs_return) {
		try {
			instance->setAlgorithm(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAll(std::vector<String> &, std::vector<FlannIndexType> &, std::vector<String> &, std::vector<double> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:88
	// ("cv::flann::IndexParams::getAll", vec![(pred!(const, ["names", "types", "strValues", "numValues"], ["std::vector<cv::String>*", "std::vector<cv::flann::FlannIndexType>*", "std::vector<cv::String>*", "std::vector<double>*"]), _)]),
	void cv_flann_IndexParams_getAll_const_vectorLStringGR_vectorLFlannIndexTypeGR_vectorLStringGR_vectorLdoubleGR(const cv::flann::IndexParams* instance, std::vector<cv::String>* names, std::vector<cv::flann::FlannIndexType>* types, std::vector<cv::String>* strValues, std::vector<double>* numValues, ResultVoid* ocvrs_return) {
		try {
			instance->getAll(*names, *types, *strValues, *numValues);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::flann::IndexParams::params() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:93
	// ("cv::flann::IndexParams::params", vec![(pred!(mut, [], []), _)]),
	void* cv_flann_IndexParams_propParams(cv::flann::IndexParams* instance) {
			void* ret = instance->params;
			return ret;
	}

	// cv::flann::IndexParams::setParams(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:93
	// ("cv::flann::IndexParams::setParams", vec![(pred!(mut, ["val"], ["void*"]), _)]),
	void cv_flann_IndexParams_propParams_voidX(cv::flann::IndexParams* instance, void* const val) {
			instance->params = val;
	}

	// cv::flann::IndexParams::delete() generated
	// ("cv::flann::IndexParams::delete", vec![(pred!(mut, [], []), _)]),
	void cv_flann_IndexParams_delete(cv::flann::IndexParams* instance) {
			delete instance;
	}

	// KDTreeIndexParams(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:102
	// ("cv::flann::KDTreeIndexParams::KDTreeIndexParams", vec![(pred!(mut, ["trees"], ["int"]), _)]),
	void cv_flann_KDTreeIndexParams_KDTreeIndexParams_int(int trees, Result<cv::flann::KDTreeIndexParams*>* ocvrs_return) {
		try {
			cv::flann::KDTreeIndexParams* ret = new cv::flann::KDTreeIndexParams(trees);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::flann::KDTreeIndexParams::KDTreeIndexParams() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:102
	// ("cv::flann::KDTreeIndexParams::KDTreeIndexParams", vec![(pred!(mut, [], []), _)]),
	void cv_flann_KDTreeIndexParams_KDTreeIndexParams(Result<cv::flann::KDTreeIndexParams*>* ocvrs_return) {
		try {
			cv::flann::KDTreeIndexParams* ret = new cv::flann::KDTreeIndexParams();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::flann::KDTreeIndexParams::to_IndexParams() generated
	// ("cv::flann::KDTreeIndexParams::to_IndexParams", vec![(pred!(mut, [], []), _)]),
	cv::flann::IndexParams* cv_flann_KDTreeIndexParams_to_IndexParams(cv::flann::KDTreeIndexParams* instance) {
			return dynamic_cast<cv::flann::IndexParams*>(instance);
	}

	// cv::flann::KDTreeIndexParams::delete() generated
	// ("cv::flann::KDTreeIndexParams::delete", vec![(pred!(mut, [], []), _)]),
	void cv_flann_KDTreeIndexParams_delete(cv::flann::KDTreeIndexParams* instance) {
			delete instance;
	}

	// KMeansIndexParams(int, int, cvflann::flann_centers_init_t, float)(Primitive, Primitive, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:130
	// ("cv::flann::KMeansIndexParams::KMeansIndexParams", vec![(pred!(mut, ["branching", "iterations", "centers_init", "cb_index"], ["int", "int", "cvflann::flann_centers_init_t", "float"]), _)]),
	void cv_flann_KMeansIndexParams_KMeansIndexParams_int_int_flann_centers_init_t_float(int branching, int iterations, cvflann::flann_centers_init_t centers_init, float cb_index, Result<cv::flann::KMeansIndexParams*>* ocvrs_return) {
		try {
			cv::flann::KMeansIndexParams* ret = new cv::flann::KMeansIndexParams(branching, iterations, centers_init, cb_index);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::flann::KMeansIndexParams::KMeansIndexParams() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:130
	// ("cv::flann::KMeansIndexParams::KMeansIndexParams", vec![(pred!(mut, [], []), _)]),
	void cv_flann_KMeansIndexParams_KMeansIndexParams(Result<cv::flann::KMeansIndexParams*>* ocvrs_return) {
		try {
			cv::flann::KMeansIndexParams* ret = new cv::flann::KMeansIndexParams();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::flann::KMeansIndexParams::to_IndexParams() generated
	// ("cv::flann::KMeansIndexParams::to_IndexParams", vec![(pred!(mut, [], []), _)]),
	cv::flann::IndexParams* cv_flann_KMeansIndexParams_to_IndexParams(cv::flann::KMeansIndexParams* instance) {
			return dynamic_cast<cv::flann::IndexParams*>(instance);
	}

	// cv::flann::KMeansIndexParams::delete() generated
	// ("cv::flann::KMeansIndexParams::delete", vec![(pred!(mut, [], []), _)]),
	void cv_flann_KMeansIndexParams_delete(cv::flann::KMeansIndexParams* instance) {
			delete instance;
	}

	// LinearIndexParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:107
	// ("cv::flann::LinearIndexParams::LinearIndexParams", vec![(pred!(mut, [], []), _)]),
	void cv_flann_LinearIndexParams_LinearIndexParams(Result<cv::flann::LinearIndexParams*>* ocvrs_return) {
		try {
			cv::flann::LinearIndexParams* ret = new cv::flann::LinearIndexParams();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::flann::LinearIndexParams::to_IndexParams() generated
	// ("cv::flann::LinearIndexParams::to_IndexParams", vec![(pred!(mut, [], []), _)]),
	cv::flann::IndexParams* cv_flann_LinearIndexParams_to_IndexParams(cv::flann::LinearIndexParams* instance) {
			return dynamic_cast<cv::flann::IndexParams*>(instance);
	}

	// cv::flann::LinearIndexParams::delete() generated
	// ("cv::flann::LinearIndexParams::delete", vec![(pred!(mut, [], []), _)]),
	void cv_flann_LinearIndexParams_delete(cv::flann::LinearIndexParams* instance) {
			delete instance;
	}

	// LshIndexParams(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:136
	// ("cv::flann::LshIndexParams::LshIndexParams", vec![(pred!(mut, ["table_number", "key_size", "multi_probe_level"], ["int", "int", "int"]), _)]),
	void cv_flann_LshIndexParams_LshIndexParams_int_int_int(int table_number, int key_size, int multi_probe_level, Result<cv::flann::LshIndexParams*>* ocvrs_return) {
		try {
			cv::flann::LshIndexParams* ret = new cv::flann::LshIndexParams(table_number, key_size, multi_probe_level);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::flann::LshIndexParams::to_IndexParams() generated
	// ("cv::flann::LshIndexParams::to_IndexParams", vec![(pred!(mut, [], []), _)]),
	cv::flann::IndexParams* cv_flann_LshIndexParams_to_IndexParams(cv::flann::LshIndexParams* instance) {
			return dynamic_cast<cv::flann::IndexParams*>(instance);
	}

	// cv::flann::LshIndexParams::delete() generated
	// ("cv::flann::LshIndexParams::delete", vec![(pred!(mut, [], []), _)]),
	void cv_flann_LshIndexParams_delete(cv::flann::LshIndexParams* instance) {
			delete instance;
	}

	// SavedIndexParams(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:141
	// ("cv::flann::SavedIndexParams::SavedIndexParams", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_flann_SavedIndexParams_SavedIndexParams_const_StringR(const char* filename, Result<cv::flann::SavedIndexParams*>* ocvrs_return) {
		try {
			cv::flann::SavedIndexParams* ret = new cv::flann::SavedIndexParams(std::string(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::flann::SavedIndexParams::to_IndexParams() generated
	// ("cv::flann::SavedIndexParams::to_IndexParams", vec![(pred!(mut, [], []), _)]),
	cv::flann::IndexParams* cv_flann_SavedIndexParams_to_IndexParams(cv::flann::SavedIndexParams* instance) {
			return dynamic_cast<cv::flann::IndexParams*>(instance);
	}

	// cv::flann::SavedIndexParams::delete() generated
	// ("cv::flann::SavedIndexParams::delete", vec![(pred!(mut, [], []), _)]),
	void cv_flann_SavedIndexParams_delete(cv::flann::SavedIndexParams* instance) {
			delete instance;
	}

	// SearchParams(int, float, bool, bool)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:146
	// ("cv::flann::SearchParams::SearchParams", vec![(pred!(mut, ["checks", "eps", "sorted", "explore_all_trees"], ["int", "float", "bool", "bool"]), _)]),
	void cv_flann_SearchParams_SearchParams_int_float_bool_bool(int checks, float eps, bool sorted, bool explore_all_trees, Result<cv::flann::SearchParams*>* ocvrs_return) {
		try {
			cv::flann::SearchParams* ret = new cv::flann::SearchParams(checks, eps, sorted, explore_all_trees);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// SearchParams(int, float, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:147
	// ("cv::flann::SearchParams::SearchParams", vec![(pred!(mut, ["checks", "eps", "sorted"], ["int", "float", "bool"]), _)]),
	void cv_flann_SearchParams_SearchParams_int_float_bool(int checks, float eps, bool sorted, Result<cv::flann::SearchParams*>* ocvrs_return) {
		try {
			cv::flann::SearchParams* ret = new cv::flann::SearchParams(checks, eps, sorted);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::flann::SearchParams::SearchParams() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:147
	// ("cv::flann::SearchParams::SearchParams", vec![(pred!(mut, [], []), _)]),
	void cv_flann_SearchParams_SearchParams(Result<cv::flann::SearchParams*>* ocvrs_return) {
		try {
			cv::flann::SearchParams* ret = new cv::flann::SearchParams();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::flann::SearchParams::to_IndexParams() generated
	// ("cv::flann::SearchParams::to_IndexParams", vec![(pred!(mut, [], []), _)]),
	cv::flann::IndexParams* cv_flann_SearchParams_to_IndexParams(cv::flann::SearchParams* instance) {
			return dynamic_cast<cv::flann::IndexParams*>(instance);
	}

	// cv::flann::SearchParams::delete() generated
	// ("cv::flann::SearchParams::delete", vec![(pred!(mut, [], []), _)]),
	void cv_flann_SearchParams_delete(cv::flann::SearchParams* instance) {
			delete instance;
	}

}
