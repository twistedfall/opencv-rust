// flann_distance_type()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann.hpp:61
// ("cvflann::flann_distance_type", vec![(pred!(mut, [], []), _)]),
pub fn cvflann_flann_distance_type(ocvrs_return: *mut Result<crate::flann::flann_distance_t>);
// set_distance_type(flann_distance_t, int)(Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann.hpp:62
// ("cvflann::set_distance_type", vec![(pred!(mut, ["distance_type", "order"], ["cvflann::flann_distance_t", "int"]), _)]),
pub fn cvflann_set_distance_type_flann_distance_t_int(distance_type: crate::flann::flann_distance_t, order: i32, ocvrs_return: *mut Result<()>);
// AutotunedIndexParams(float, float, float, float)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:118
// ("cv::flann::AutotunedIndexParams::AutotunedIndexParams", vec![(pred!(mut, ["target_precision", "build_weight", "memory_weight", "sample_fraction"], ["float", "float", "float", "float"]), _)]),
pub fn cv_flann_AutotunedIndexParams_AutotunedIndexParams_float_float_float_float(target_precision: f32, build_weight: f32, memory_weight: f32, sample_fraction: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::flann::AutotunedIndexParams::AutotunedIndexParams() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:118
// ("cv::flann::AutotunedIndexParams::AutotunedIndexParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_flann_AutotunedIndexParams_AutotunedIndexParams(ocvrs_return: *mut Result<*mut c_void>);
// cv::flann::AutotunedIndexParams::to_IndexParams() generated
// ("cv::flann::AutotunedIndexParams::to_IndexParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_flann_AutotunedIndexParams_to_IndexParams(instance: *mut c_void) -> *mut c_void;
// cv::flann::AutotunedIndexParams::delete() generated
// ("cv::flann::AutotunedIndexParams::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_flann_AutotunedIndexParams_delete(instance: *mut c_void);
// CompositeIndexParams(int, int, int, cvflann::flann_centers_init_t, float)(Primitive, Primitive, Primitive, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:112
// ("cv::flann::CompositeIndexParams::CompositeIndexParams", vec![(pred!(mut, ["trees", "branching", "iterations", "centers_init", "cb_index"], ["int", "int", "int", "cvflann::flann_centers_init_t", "float"]), _)]),
pub fn cv_flann_CompositeIndexParams_CompositeIndexParams_int_int_int_flann_centers_init_t_float(trees: i32, branching: i32, iterations: i32, centers_init: crate::flann::flann_centers_init_t, cb_index: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::flann::CompositeIndexParams::CompositeIndexParams() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:112
// ("cv::flann::CompositeIndexParams::CompositeIndexParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_flann_CompositeIndexParams_CompositeIndexParams(ocvrs_return: *mut Result<*mut c_void>);
// cv::flann::CompositeIndexParams::to_IndexParams() generated
// ("cv::flann::CompositeIndexParams::to_IndexParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_flann_CompositeIndexParams_to_IndexParams(instance: *mut c_void) -> *mut c_void;
// cv::flann::CompositeIndexParams::delete() generated
// ("cv::flann::CompositeIndexParams::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_flann_CompositeIndexParams_delete(instance: *mut c_void);
// HierarchicalClusteringIndexParams(int, cvflann::flann_centers_init_t, int, int)(Primitive, Enum, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:124
// ("cv::flann::HierarchicalClusteringIndexParams::HierarchicalClusteringIndexParams", vec![(pred!(mut, ["branching", "centers_init", "trees", "leaf_size"], ["int", "cvflann::flann_centers_init_t", "int", "int"]), _)]),
pub fn cv_flann_HierarchicalClusteringIndexParams_HierarchicalClusteringIndexParams_int_flann_centers_init_t_int_int(branching: i32, centers_init: crate::flann::flann_centers_init_t, trees: i32, leaf_size: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::flann::HierarchicalClusteringIndexParams::HierarchicalClusteringIndexParams() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:124
// ("cv::flann::HierarchicalClusteringIndexParams::HierarchicalClusteringIndexParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_flann_HierarchicalClusteringIndexParams_HierarchicalClusteringIndexParams(ocvrs_return: *mut Result<*mut c_void>);
// cv::flann::HierarchicalClusteringIndexParams::to_IndexParams() generated
// ("cv::flann::HierarchicalClusteringIndexParams::to_IndexParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_flann_HierarchicalClusteringIndexParams_to_IndexParams(instance: *mut c_void) -> *mut c_void;
// cv::flann::HierarchicalClusteringIndexParams::delete() generated
// ("cv::flann::HierarchicalClusteringIndexParams::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_flann_HierarchicalClusteringIndexParams_delete(instance: *mut c_void);
// Index()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:153
// ("cv::flann::Index::Index", vec![(pred!(mut, [], []), _)]),
pub fn cv_flann_Index_Index(ocvrs_return: *mut Result<*mut c_void>);
// Index(InputArray, const IndexParams &, cvflann::flann_distance_t)(InputArray, TraitClass, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:154
// ("cv::flann::Index::Index", vec![(pred!(mut, ["features", "params", "distType"], ["const cv::_InputArray*", "const cv::flann::IndexParams*", "cvflann::flann_distance_t"]), _)]),
pub fn cv_flann_Index_Index_const__InputArrayR_const_IndexParamsR_flann_distance_t(features: *const c_void, params: *const c_void, dist_type: crate::flann::flann_distance_t, ocvrs_return: *mut Result<*mut c_void>);
// cv::flann::Index::Index(InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:154
// ("cv::flann::Index::Index", vec![(pred!(mut, ["features", "params"], ["const cv::_InputArray*", "const cv::flann::IndexParams*"]), _)]),
pub fn cv_flann_Index_Index_const__InputArrayR_const_IndexParamsR(features: *const c_void, params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// build(InputArray, const IndexParams &, cvflann::flann_distance_t)(InputArray, TraitClass, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:157
// ("cv::flann::Index::build", vec![(pred!(mut, ["features", "params", "distType"], ["const cv::_InputArray*", "const cv::flann::IndexParams*", "cvflann::flann_distance_t"]), _)]),
pub fn cv_flann_Index_build_const__InputArrayR_const_IndexParamsR_flann_distance_t(instance: *mut c_void, features: *const c_void, params: *const c_void, dist_type: crate::flann::flann_distance_t, ocvrs_return: *mut Result<()>);
// cv::flann::Index::build(InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:157
// ("cv::flann::Index::build", vec![(pred!(mut, ["features", "params"], ["const cv::_InputArray*", "const cv::flann::IndexParams*"]), _)]),
pub fn cv_flann_Index_build_const__InputArrayR_const_IndexParamsR(instance: *mut c_void, features: *const c_void, params: *const c_void, ocvrs_return: *mut Result<()>);
// knnSearch(InputArray, OutputArray, OutputArray, int, const SearchParams &)(InputArray, OutputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:158
// ("cv::flann::Index::knnSearch", vec![(pred!(mut, ["query", "indices", "dists", "knn", "params"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "const cv::flann::SearchParams*"]), _)]),
pub fn cv_flann_Index_knnSearch_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_const_SearchParamsR(instance: *mut c_void, query: *const c_void, indices: *const c_void, dists: *const c_void, knn: i32, params: *const c_void, ocvrs_return: *mut Result<()>);
// cv::flann::Index::knnSearch(InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:158
// ("cv::flann::Index::knnSearch", vec![(pred!(mut, ["query", "indices", "dists", "knn"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_flann_Index_knnSearch_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int(instance: *mut c_void, query: *const c_void, indices: *const c_void, dists: *const c_void, knn: i32, ocvrs_return: *mut Result<()>);
// radiusSearch(InputArray, OutputArray, OutputArray, double, int, const SearchParams &)(InputArray, OutputArray, OutputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:161
// ("cv::flann::Index::radiusSearch", vec![(pred!(mut, ["query", "indices", "dists", "radius", "maxResults", "params"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "int", "const cv::flann::SearchParams*"]), _)]),
pub fn cv_flann_Index_radiusSearch_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_int_const_SearchParamsR(instance: *mut c_void, query: *const c_void, indices: *const c_void, dists: *const c_void, radius: f64, max_results: i32, params: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::flann::Index::radiusSearch(InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:161
// ("cv::flann::Index::radiusSearch", vec![(pred!(mut, ["query", "indices", "dists", "radius", "maxResults"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "int"]), _)]),
pub fn cv_flann_Index_radiusSearch_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_int(instance: *mut c_void, query: *const c_void, indices: *const c_void, dists: *const c_void, radius: f64, max_results: i32, ocvrs_return: *mut Result<i32>);
// save(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:165
// ("cv::flann::Index::save", vec![(pred!(const, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_flann_Index_save_const_const_StringR(instance: *const c_void, filename: *const c_char, ocvrs_return: *mut Result<()>);
// load(InputArray, const String &)(InputArray, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:166
// ("cv::flann::Index::load", vec![(pred!(mut, ["features", "filename"], ["const cv::_InputArray*", "const cv::String*"]), _)]),
pub fn cv_flann_Index_load_const__InputArrayR_const_StringR(instance: *mut c_void, features: *const c_void, filename: *const c_char, ocvrs_return: *mut Result<bool>);
// release()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:167
// ("cv::flann::Index::release", vec![(pred!(mut, [], []), _)]),
pub fn cv_flann_Index_release(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// getDistance()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:168
// ("cv::flann::Index::getDistance", vec![(pred!(const, [], []), _)]),
pub fn cv_flann_Index_getDistance_const(instance: *const c_void, ocvrs_return: *mut Result<crate::flann::flann_distance_t>);
// getAlgorithm()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:169
// ("cv::flann::Index::getAlgorithm", vec![(pred!(const, [], []), _)]),
pub fn cv_flann_Index_getAlgorithm_const(instance: *const c_void, ocvrs_return: *mut Result<crate::flann::flann_algorithm_t>);
// cv::flann::Index::delete() generated
// ("cv::flann::Index::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_flann_Index_delete(instance: *mut c_void);
// IndexParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:73
// ("cv::flann::IndexParams::IndexParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_flann_IndexParams_IndexParams(ocvrs_return: *mut Result<*mut c_void>);
// getString(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:76
// ("cv::flann::IndexParams::getString", vec![(pred!(const, ["key", "defaultVal"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_flann_IndexParams_getString_const_const_StringR_const_StringR(instance: *const c_void, key: *const c_char, default_val: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::flann::IndexParams::getString(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:76
// ("cv::flann::IndexParams::getString", vec![(pred!(const, ["key"], ["const cv::String*"]), _)]),
pub fn cv_flann_IndexParams_getString_const_const_StringR(instance: *const c_void, key: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// getInt(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:77
// ("cv::flann::IndexParams::getInt", vec![(pred!(const, ["key", "defaultVal"], ["const cv::String*", "int"]), _)]),
pub fn cv_flann_IndexParams_getInt_const_const_StringR_int(instance: *const c_void, key: *const c_char, default_val: i32, ocvrs_return: *mut Result<i32>);
// cv::flann::IndexParams::getInt(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:77
// ("cv::flann::IndexParams::getInt", vec![(pred!(const, ["key"], ["const cv::String*"]), _)]),
pub fn cv_flann_IndexParams_getInt_const_const_StringR(instance: *const c_void, key: *const c_char, ocvrs_return: *mut Result<i32>);
// getDouble(const String &, double)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:78
// ("cv::flann::IndexParams::getDouble", vec![(pred!(const, ["key", "defaultVal"], ["const cv::String*", "double"]), _)]),
pub fn cv_flann_IndexParams_getDouble_const_const_StringR_double(instance: *const c_void, key: *const c_char, default_val: f64, ocvrs_return: *mut Result<f64>);
// cv::flann::IndexParams::getDouble(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:78
// ("cv::flann::IndexParams::getDouble", vec![(pred!(const, ["key"], ["const cv::String*"]), _)]),
pub fn cv_flann_IndexParams_getDouble_const_const_StringR(instance: *const c_void, key: *const c_char, ocvrs_return: *mut Result<f64>);
// setString(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:80
// ("cv::flann::IndexParams::setString", vec![(pred!(mut, ["key", "value"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_flann_IndexParams_setString_const_StringR_const_StringR(instance: *mut c_void, key: *const c_char, value: *const c_char, ocvrs_return: *mut Result<()>);
// setInt(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:81
// ("cv::flann::IndexParams::setInt", vec![(pred!(mut, ["key", "value"], ["const cv::String*", "int"]), _)]),
pub fn cv_flann_IndexParams_setInt_const_StringR_int(instance: *mut c_void, key: *const c_char, value: i32, ocvrs_return: *mut Result<()>);
// setDouble(const String &, double)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:82
// ("cv::flann::IndexParams::setDouble", vec![(pred!(mut, ["key", "value"], ["const cv::String*", "double"]), _)]),
pub fn cv_flann_IndexParams_setDouble_const_StringR_double(instance: *mut c_void, key: *const c_char, value: f64, ocvrs_return: *mut Result<()>);
// setFloat(const String &, float)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:83
// ("cv::flann::IndexParams::setFloat", vec![(pred!(mut, ["key", "value"], ["const cv::String*", "float"]), _)]),
pub fn cv_flann_IndexParams_setFloat_const_StringR_float(instance: *mut c_void, key: *const c_char, value: f32, ocvrs_return: *mut Result<()>);
// setBool(const String &, bool)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:84
// ("cv::flann::IndexParams::setBool", vec![(pred!(mut, ["key", "value"], ["const cv::String*", "bool"]), _)]),
pub fn cv_flann_IndexParams_setBool_const_StringR_bool(instance: *mut c_void, key: *const c_char, value: bool, ocvrs_return: *mut Result<()>);
// setAlgorithm(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:85
// ("cv::flann::IndexParams::setAlgorithm", vec![(pred!(mut, ["value"], ["int"]), _)]),
pub fn cv_flann_IndexParams_setAlgorithm_int(instance: *mut c_void, value: i32, ocvrs_return: *mut Result<()>);
// getAll(std::vector<String> &, std::vector<FlannIndexType> &, std::vector<String> &, std::vector<double> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:88
// ("cv::flann::IndexParams::getAll", vec![(pred!(const, ["names", "types", "strValues", "numValues"], ["std::vector<cv::String>*", "std::vector<cv::flann::FlannIndexType>*", "std::vector<cv::String>*", "std::vector<double>*"]), _)]),
pub fn cv_flann_IndexParams_getAll_const_vectorLStringGR_vectorLFlannIndexTypeGR_vectorLStringGR_vectorLdoubleGR(instance: *const c_void, names: *mut c_void, types: *mut c_void, str_values: *mut c_void, num_values: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::flann::IndexParams::params() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:93
// ("cv::flann::IndexParams::params", vec![(pred!(mut, [], []), _)]),
pub fn cv_flann_IndexParams_propParams(instance: *mut c_void) -> *mut c_void;
// cv::flann::IndexParams::setParams(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:93
// ("cv::flann::IndexParams::setParams", vec![(pred!(mut, ["val"], ["void*"]), _)]),
pub fn cv_flann_IndexParams_propParams_voidX(instance: *mut c_void, val: *const c_void);
// cv::flann::IndexParams::delete() generated
// ("cv::flann::IndexParams::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_flann_IndexParams_delete(instance: *mut c_void);
// KDTreeIndexParams(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:102
// ("cv::flann::KDTreeIndexParams::KDTreeIndexParams", vec![(pred!(mut, ["trees"], ["int"]), _)]),
pub fn cv_flann_KDTreeIndexParams_KDTreeIndexParams_int(trees: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::flann::KDTreeIndexParams::KDTreeIndexParams() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:102
// ("cv::flann::KDTreeIndexParams::KDTreeIndexParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_flann_KDTreeIndexParams_KDTreeIndexParams(ocvrs_return: *mut Result<*mut c_void>);
// cv::flann::KDTreeIndexParams::to_IndexParams() generated
// ("cv::flann::KDTreeIndexParams::to_IndexParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_flann_KDTreeIndexParams_to_IndexParams(instance: *mut c_void) -> *mut c_void;
// cv::flann::KDTreeIndexParams::delete() generated
// ("cv::flann::KDTreeIndexParams::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_flann_KDTreeIndexParams_delete(instance: *mut c_void);
// KMeansIndexParams(int, int, cvflann::flann_centers_init_t, float)(Primitive, Primitive, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:130
// ("cv::flann::KMeansIndexParams::KMeansIndexParams", vec![(pred!(mut, ["branching", "iterations", "centers_init", "cb_index"], ["int", "int", "cvflann::flann_centers_init_t", "float"]), _)]),
pub fn cv_flann_KMeansIndexParams_KMeansIndexParams_int_int_flann_centers_init_t_float(branching: i32, iterations: i32, centers_init: crate::flann::flann_centers_init_t, cb_index: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::flann::KMeansIndexParams::KMeansIndexParams() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:130
// ("cv::flann::KMeansIndexParams::KMeansIndexParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_flann_KMeansIndexParams_KMeansIndexParams(ocvrs_return: *mut Result<*mut c_void>);
// cv::flann::KMeansIndexParams::to_IndexParams() generated
// ("cv::flann::KMeansIndexParams::to_IndexParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_flann_KMeansIndexParams_to_IndexParams(instance: *mut c_void) -> *mut c_void;
// cv::flann::KMeansIndexParams::delete() generated
// ("cv::flann::KMeansIndexParams::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_flann_KMeansIndexParams_delete(instance: *mut c_void);
// LinearIndexParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:107
// ("cv::flann::LinearIndexParams::LinearIndexParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_flann_LinearIndexParams_LinearIndexParams(ocvrs_return: *mut Result<*mut c_void>);
// cv::flann::LinearIndexParams::to_IndexParams() generated
// ("cv::flann::LinearIndexParams::to_IndexParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_flann_LinearIndexParams_to_IndexParams(instance: *mut c_void) -> *mut c_void;
// cv::flann::LinearIndexParams::delete() generated
// ("cv::flann::LinearIndexParams::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_flann_LinearIndexParams_delete(instance: *mut c_void);
// LshIndexParams(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:136
// ("cv::flann::LshIndexParams::LshIndexParams", vec![(pred!(mut, ["table_number", "key_size", "multi_probe_level"], ["int", "int", "int"]), _)]),
pub fn cv_flann_LshIndexParams_LshIndexParams_int_int_int(table_number: i32, key_size: i32, multi_probe_level: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::flann::LshIndexParams::to_IndexParams() generated
// ("cv::flann::LshIndexParams::to_IndexParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_flann_LshIndexParams_to_IndexParams(instance: *mut c_void) -> *mut c_void;
// cv::flann::LshIndexParams::delete() generated
// ("cv::flann::LshIndexParams::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_flann_LshIndexParams_delete(instance: *mut c_void);
// SavedIndexParams(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:141
// ("cv::flann::SavedIndexParams::SavedIndexParams", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_flann_SavedIndexParams_SavedIndexParams_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::flann::SavedIndexParams::to_IndexParams() generated
// ("cv::flann::SavedIndexParams::to_IndexParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_flann_SavedIndexParams_to_IndexParams(instance: *mut c_void) -> *mut c_void;
// cv::flann::SavedIndexParams::delete() generated
// ("cv::flann::SavedIndexParams::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_flann_SavedIndexParams_delete(instance: *mut c_void);
// SearchParams(int, float, bool, bool)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:146
// ("cv::flann::SearchParams::SearchParams", vec![(pred!(mut, ["checks", "eps", "sorted", "explore_all_trees"], ["int", "float", "bool", "bool"]), _)]),
pub fn cv_flann_SearchParams_SearchParams_int_float_bool_bool(checks: i32, eps: f32, sorted: bool, explore_all_trees: bool, ocvrs_return: *mut Result<*mut c_void>);
// SearchParams(int, float, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:147
// ("cv::flann::SearchParams::SearchParams", vec![(pred!(mut, ["checks", "eps", "sorted"], ["int", "float", "bool"]), _)]),
pub fn cv_flann_SearchParams_SearchParams_int_float_bool(checks: i32, eps: f32, sorted: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::flann::SearchParams::SearchParams() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/flann/miniflann.hpp:147
// ("cv::flann::SearchParams::SearchParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_flann_SearchParams_SearchParams(ocvrs_return: *mut Result<*mut c_void>);
// cv::flann::SearchParams::to_IndexParams() generated
// ("cv::flann::SearchParams::to_IndexParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_flann_SearchParams_to_IndexParams(instance: *mut c_void) -> *mut c_void;
// cv::flann::SearchParams::delete() generated
// ("cv::flann::SearchParams::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_flann_SearchParams_delete(instance: *mut c_void);
