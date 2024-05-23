// std::vector<cv::linemod::Feature>::new() generated
// ("std::vector<cv::linemod::Feature>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_linemod_FeatureG_new_const() -> *mut c_void;
// std::vector<cv::linemod::Feature>::delete() generated
// ("std::vector<cv::linemod::Feature>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_linemod_FeatureG_delete(instance: *mut c_void);
// std::vector<cv::linemod::Feature>::len() generated
// ("std::vector<cv::linemod::Feature>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_linemod_FeatureG_len_const(instance: *const c_void) -> size_t;
// std::vector<cv::linemod::Feature>::isEmpty() generated
// ("std::vector<cv::linemod::Feature>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_linemod_FeatureG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<cv::linemod::Feature>::capacity() generated
// ("std::vector<cv::linemod::Feature>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_linemod_FeatureG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<cv::linemod::Feature>::shrinkToFit() generated
// ("std::vector<cv::linemod::Feature>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_linemod_FeatureG_shrinkToFit(instance: *mut c_void);
// std::vector<cv::linemod::Feature>::reserve(Primitive) generated
// ("std::vector<cv::linemod::Feature>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLcv_linemod_FeatureG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<cv::linemod::Feature>::remove(Primitive) generated
// ("std::vector<cv::linemod::Feature>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_linemod_FeatureG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<cv::linemod::Feature>::swap(Primitive, Primitive) generated
// ("std::vector<cv::linemod::Feature>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLcv_linemod_FeatureG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<cv::linemod::Feature>::clear() generated
// ("std::vector<cv::linemod::Feature>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_linemod_FeatureG_clear(instance: *mut c_void);
// std::vector<cv::linemod::Feature>::push(SimpleClass) generated
// ("std::vector<cv::linemod::Feature>::push", vec![(pred!(mut, ["val"], ["const cv::linemod::Feature"]), _)]),
pub fn std_vectorLcv_linemod_FeatureG_push_const_Feature(instance: *mut c_void, val: *const crate::rgbd::LineMod_Feature);
// std::vector<cv::linemod::Feature>::insert(Primitive, SimpleClass) generated
// ("std::vector<cv::linemod::Feature>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::linemod::Feature"]), _)]),
pub fn std_vectorLcv_linemod_FeatureG_insert_size_t_const_Feature(instance: *mut c_void, index: size_t, val: *const crate::rgbd::LineMod_Feature);
// std::vector<cv::linemod::Feature>::get(Primitive) generated
// ("std::vector<cv::linemod::Feature>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_linemod_FeatureG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut crate::rgbd::LineMod_Feature);
// std::vector<cv::linemod::Feature>::set(Primitive, SimpleClass) generated
// ("std::vector<cv::linemod::Feature>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::linemod::Feature"]), _)]),
pub fn std_vectorLcv_linemod_FeatureG_set_size_t_const_Feature(instance: *mut c_void, index: size_t, val: *const crate::rgbd::LineMod_Feature);
// std::vector<cv::linemod::Feature>::clone() generated
// ("std::vector<cv::linemod::Feature>::clone", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_linemod_FeatureG_clone_const(instance: *const c_void) -> *mut c_void;
// std::vector<cv::linemod::Feature>::data() generated
// ("std::vector<cv::linemod::Feature>::data", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_linemod_FeatureG_data_const(instance: *const c_void) -> *const crate::rgbd::LineMod_Feature;
// std::vector<cv::linemod::Feature>::dataMut() generated
// ("std::vector<cv::linemod::Feature>::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_linemod_FeatureG_dataMut(instance: *mut c_void) -> *mut crate::rgbd::LineMod_Feature;
// cv::fromSlice(SimpleClass, Primitive) generated
// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::linemod::Feature*", "size_t"]), _)]),
pub fn cv_fromSlice_const_const_FeatureX_size_t(data: *const crate::rgbd::LineMod_Feature, len: size_t) -> *mut c_void;
