// std::vector<cvflann::lsh::FeatureIndex>::new() generated
// ("std::vector<cvflann::lsh::FeatureIndex>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcvflann_lsh_FeatureIndexG_new_const() -> *mut c_void;
// std::vector<cvflann::lsh::FeatureIndex>::delete() generated
// ("std::vector<cvflann::lsh::FeatureIndex>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcvflann_lsh_FeatureIndexG_delete(instance: *mut c_void);
// std::vector<cvflann::lsh::FeatureIndex>::len() generated
// ("std::vector<cvflann::lsh::FeatureIndex>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcvflann_lsh_FeatureIndexG_len_const(instance: *const c_void) -> size_t;
// std::vector<cvflann::lsh::FeatureIndex>::isEmpty() generated
// ("std::vector<cvflann::lsh::FeatureIndex>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcvflann_lsh_FeatureIndexG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<cvflann::lsh::FeatureIndex>::capacity() generated
// ("std::vector<cvflann::lsh::FeatureIndex>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcvflann_lsh_FeatureIndexG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<cvflann::lsh::FeatureIndex>::shrinkToFit() generated
// ("std::vector<cvflann::lsh::FeatureIndex>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcvflann_lsh_FeatureIndexG_shrinkToFit(instance: *mut c_void);
// std::vector<cvflann::lsh::FeatureIndex>::reserve(Primitive) generated
// ("std::vector<cvflann::lsh::FeatureIndex>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLcvflann_lsh_FeatureIndexG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<cvflann::lsh::FeatureIndex>::remove(Primitive) generated
// ("std::vector<cvflann::lsh::FeatureIndex>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcvflann_lsh_FeatureIndexG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<cvflann::lsh::FeatureIndex>::swap(Primitive, Primitive) generated
// ("std::vector<cvflann::lsh::FeatureIndex>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLcvflann_lsh_FeatureIndexG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<cvflann::lsh::FeatureIndex>::clear() generated
// ("std::vector<cvflann::lsh::FeatureIndex>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcvflann_lsh_FeatureIndexG_clear(instance: *mut c_void);
// std::vector<cvflann::lsh::FeatureIndex>::push(Primitive) generated
// ("std::vector<cvflann::lsh::FeatureIndex>::push", vec![(pred!(mut, ["val"], ["const cvflann::lsh::FeatureIndex"]), _)]),
pub fn std_vectorLcvflann_lsh_FeatureIndexG_push_const_FeatureIndex(instance: *mut c_void, val: crate::flann::FeatureIndex);
// std::vector<cvflann::lsh::FeatureIndex>::insert(Primitive, Primitive) generated
// ("std::vector<cvflann::lsh::FeatureIndex>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cvflann::lsh::FeatureIndex"]), _)]),
pub fn std_vectorLcvflann_lsh_FeatureIndexG_insert_size_t_const_FeatureIndex(instance: *mut c_void, index: size_t, val: crate::flann::FeatureIndex);
// std::vector<cvflann::lsh::FeatureIndex>::get(Primitive) generated
// ("std::vector<cvflann::lsh::FeatureIndex>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcvflann_lsh_FeatureIndexG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut crate::flann::FeatureIndex);
// std::vector<cvflann::lsh::FeatureIndex>::set(Primitive, Primitive) generated
// ("std::vector<cvflann::lsh::FeatureIndex>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cvflann::lsh::FeatureIndex"]), _)]),
pub fn std_vectorLcvflann_lsh_FeatureIndexG_set_size_t_const_FeatureIndex(instance: *mut c_void, index: size_t, val: crate::flann::FeatureIndex);
// std::vector<cvflann::lsh::FeatureIndex>::clone() generated
// ("std::vector<cvflann::lsh::FeatureIndex>::clone", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcvflann_lsh_FeatureIndexG_clone_const(instance: *const c_void) -> *mut c_void;
// std::vector<cvflann::lsh::FeatureIndex>::data() generated
// ("std::vector<cvflann::lsh::FeatureIndex>::data", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcvflann_lsh_FeatureIndexG_data_const(instance: *const c_void) -> *const crate::flann::FeatureIndex;
// std::vector<cvflann::lsh::FeatureIndex>::dataMut() generated
// ("std::vector<cvflann::lsh::FeatureIndex>::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcvflann_lsh_FeatureIndexG_dataMut(instance: *mut c_void) -> *mut crate::flann::FeatureIndex;
// cv::fromSlice(Indirect, Primitive) generated
// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cvflann::lsh::FeatureIndex*", "size_t"]), _)]),
pub fn cv_fromSlice_const_const_FeatureIndexX_size_t(data: *const crate::flann::FeatureIndex, len: size_t) -> *mut c_void;
#[cfg(ocvrs_opencv_branch_5)]
// std::vector<cvflann::lsh::FeatureIndex>::inputArray() generated
// ("std::vector<cvflann::lsh::FeatureIndex>::inputArray", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcvflann_lsh_FeatureIndexG_inputArray_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
#[cfg(ocvrs_opencv_branch_5)]
// std::vector<cvflann::lsh::FeatureIndex>::outputArray() generated
// ("std::vector<cvflann::lsh::FeatureIndex>::outputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcvflann_lsh_FeatureIndexG_outputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
#[cfg(ocvrs_opencv_branch_5)]
// std::vector<cvflann::lsh::FeatureIndex>::inputOutputArray() generated
// ("std::vector<cvflann::lsh::FeatureIndex>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcvflann_lsh_FeatureIndexG_inputOutputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
