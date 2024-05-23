// std::vector<cv::Vec3i>::new() generated
// ("std::vector<cv::Vec3i>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec3iG_new_const() -> *mut c_void;
// std::vector<cv::Vec3i>::delete() generated
// ("std::vector<cv::Vec3i>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec3iG_delete(instance: *mut c_void);
// std::vector<cv::Vec3i>::len() generated
// ("std::vector<cv::Vec3i>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec3iG_len_const(instance: *const c_void) -> size_t;
// std::vector<cv::Vec3i>::isEmpty() generated
// ("std::vector<cv::Vec3i>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec3iG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<cv::Vec3i>::capacity() generated
// ("std::vector<cv::Vec3i>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec3iG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<cv::Vec3i>::shrinkToFit() generated
// ("std::vector<cv::Vec3i>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec3iG_shrinkToFit(instance: *mut c_void);
// std::vector<cv::Vec3i>::reserve(Primitive) generated
// ("std::vector<cv::Vec3i>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLcv_Vec3iG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<cv::Vec3i>::remove(Primitive) generated
// ("std::vector<cv::Vec3i>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_Vec3iG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<cv::Vec3i>::swap(Primitive, Primitive) generated
// ("std::vector<cv::Vec3i>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLcv_Vec3iG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<cv::Vec3i>::clear() generated
// ("std::vector<cv::Vec3i>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec3iG_clear(instance: *mut c_void);
// std::vector<cv::Vec3i>::push(SimpleClass) generated
// ("std::vector<cv::Vec3i>::push", vec![(pred!(mut, ["val"], ["const cv::Vec3i"]), _)]),
pub fn std_vectorLcv_Vec3iG_push_const_Vec3i(instance: *mut c_void, val: *const core::Vec3i);
// std::vector<cv::Vec3i>::insert(Primitive, SimpleClass) generated
// ("std::vector<cv::Vec3i>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Vec3i"]), _)]),
pub fn std_vectorLcv_Vec3iG_insert_size_t_const_Vec3i(instance: *mut c_void, index: size_t, val: *const core::Vec3i);
// std::vector<cv::Vec3i>::get(Primitive) generated
// ("std::vector<cv::Vec3i>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_Vec3iG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut core::Vec3i);
// std::vector<cv::Vec3i>::set(Primitive, SimpleClass) generated
// ("std::vector<cv::Vec3i>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Vec3i"]), _)]),
pub fn std_vectorLcv_Vec3iG_set_size_t_const_Vec3i(instance: *mut c_void, index: size_t, val: *const core::Vec3i);
// std::vector<cv::Vec3i>::clone() generated
// ("std::vector<cv::Vec3i>::clone", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec3iG_clone_const(instance: *const c_void) -> *mut c_void;
// std::vector<cv::Vec3i>::data() generated
// ("std::vector<cv::Vec3i>::data", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec3iG_data_const(instance: *const c_void) -> *const core::Vec3i;
// std::vector<cv::Vec3i>::dataMut() generated
// ("std::vector<cv::Vec3i>::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec3iG_dataMut(instance: *mut c_void) -> *mut core::Vec3i;
// cv::fromSlice(SimpleClass, Primitive) generated
// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::Vec3i*", "size_t"]), _)]),
pub fn cv_fromSlice_const_const_Vec3iX_size_t(data: *const core::Vec3i, len: size_t) -> *mut c_void;
// std::vector<cv::Vec3i>::inputArray() generated
// ("std::vector<cv::Vec3i>::inputArray", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec3iG_inputArray_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<cv::Vec3i>::outputArray() generated
// ("std::vector<cv::Vec3i>::outputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec3iG_outputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<cv::Vec3i>::inputOutputArray() generated
// ("std::vector<cv::Vec3i>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec3iG_inputOutputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
