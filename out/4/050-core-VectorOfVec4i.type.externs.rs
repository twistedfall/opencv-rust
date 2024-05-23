// std::vector<cv::Vec4i>::new() generated
// ("std::vector<cv::Vec4i>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec4iG_new_const() -> *mut c_void;
// std::vector<cv::Vec4i>::delete() generated
// ("std::vector<cv::Vec4i>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec4iG_delete(instance: *mut c_void);
// std::vector<cv::Vec4i>::len() generated
// ("std::vector<cv::Vec4i>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec4iG_len_const(instance: *const c_void) -> size_t;
// std::vector<cv::Vec4i>::isEmpty() generated
// ("std::vector<cv::Vec4i>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec4iG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<cv::Vec4i>::capacity() generated
// ("std::vector<cv::Vec4i>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec4iG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<cv::Vec4i>::shrinkToFit() generated
// ("std::vector<cv::Vec4i>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec4iG_shrinkToFit(instance: *mut c_void);
// std::vector<cv::Vec4i>::reserve(Primitive) generated
// ("std::vector<cv::Vec4i>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLcv_Vec4iG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<cv::Vec4i>::remove(Primitive) generated
// ("std::vector<cv::Vec4i>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_Vec4iG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<cv::Vec4i>::swap(Primitive, Primitive) generated
// ("std::vector<cv::Vec4i>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLcv_Vec4iG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<cv::Vec4i>::clear() generated
// ("std::vector<cv::Vec4i>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec4iG_clear(instance: *mut c_void);
// std::vector<cv::Vec4i>::push(SimpleClass) generated
// ("std::vector<cv::Vec4i>::push", vec![(pred!(mut, ["val"], ["const cv::Vec4i"]), _)]),
pub fn std_vectorLcv_Vec4iG_push_const_Vec4i(instance: *mut c_void, val: *const core::Vec4i);
// std::vector<cv::Vec4i>::insert(Primitive, SimpleClass) generated
// ("std::vector<cv::Vec4i>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Vec4i"]), _)]),
pub fn std_vectorLcv_Vec4iG_insert_size_t_const_Vec4i(instance: *mut c_void, index: size_t, val: *const core::Vec4i);
// std::vector<cv::Vec4i>::get(Primitive) generated
// ("std::vector<cv::Vec4i>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_Vec4iG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut core::Vec4i);
// std::vector<cv::Vec4i>::set(Primitive, SimpleClass) generated
// ("std::vector<cv::Vec4i>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Vec4i"]), _)]),
pub fn std_vectorLcv_Vec4iG_set_size_t_const_Vec4i(instance: *mut c_void, index: size_t, val: *const core::Vec4i);
// std::vector<cv::Vec4i>::clone() generated
// ("std::vector<cv::Vec4i>::clone", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec4iG_clone_const(instance: *const c_void) -> *mut c_void;
// std::vector<cv::Vec4i>::data() generated
// ("std::vector<cv::Vec4i>::data", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec4iG_data_const(instance: *const c_void) -> *const core::Vec4i;
// std::vector<cv::Vec4i>::dataMut() generated
// ("std::vector<cv::Vec4i>::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec4iG_dataMut(instance: *mut c_void) -> *mut core::Vec4i;
// cv::fromSlice(SimpleClass, Primitive) generated
// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::Vec4i*", "size_t"]), _)]),
pub fn cv_fromSlice_const_const_Vec4iX_size_t(data: *const core::Vec4i, len: size_t) -> *mut c_void;
// std::vector<cv::Vec4i>::inputArray() generated
// ("std::vector<cv::Vec4i>::inputArray", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec4iG_inputArray_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<cv::Vec4i>::outputArray() generated
// ("std::vector<cv::Vec4i>::outputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec4iG_outputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<cv::Vec4i>::inputOutputArray() generated
// ("std::vector<cv::Vec4i>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec4iG_inputOutputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
