// std::vector<cv::Vec2i>::new() generated
// ("std::vector<cv::Vec2i>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec2iG_new_const() -> *mut c_void;
// std::vector<cv::Vec2i>::delete() generated
// ("std::vector<cv::Vec2i>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec2iG_delete(instance: *mut c_void);
// std::vector<cv::Vec2i>::len() generated
// ("std::vector<cv::Vec2i>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec2iG_len_const(instance: *const c_void) -> size_t;
// std::vector<cv::Vec2i>::isEmpty() generated
// ("std::vector<cv::Vec2i>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec2iG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<cv::Vec2i>::capacity() generated
// ("std::vector<cv::Vec2i>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec2iG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<cv::Vec2i>::shrinkToFit() generated
// ("std::vector<cv::Vec2i>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec2iG_shrinkToFit(instance: *mut c_void);
// std::vector<cv::Vec2i>::reserve(Primitive) generated
// ("std::vector<cv::Vec2i>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLcv_Vec2iG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<cv::Vec2i>::remove(Primitive) generated
// ("std::vector<cv::Vec2i>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_Vec2iG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<cv::Vec2i>::swap(Primitive, Primitive) generated
// ("std::vector<cv::Vec2i>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLcv_Vec2iG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<cv::Vec2i>::clear() generated
// ("std::vector<cv::Vec2i>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec2iG_clear(instance: *mut c_void);
// std::vector<cv::Vec2i>::push(SimpleClass) generated
// ("std::vector<cv::Vec2i>::push", vec![(pred!(mut, ["val"], ["const cv::Vec2i"]), _)]),
pub fn std_vectorLcv_Vec2iG_push_const_Vec2i(instance: *mut c_void, val: *const core::Vec2i);
// std::vector<cv::Vec2i>::insert(Primitive, SimpleClass) generated
// ("std::vector<cv::Vec2i>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Vec2i"]), _)]),
pub fn std_vectorLcv_Vec2iG_insert_size_t_const_Vec2i(instance: *mut c_void, index: size_t, val: *const core::Vec2i);
// std::vector<cv::Vec2i>::get(Primitive) generated
// ("std::vector<cv::Vec2i>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_Vec2iG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut core::Vec2i);
// std::vector<cv::Vec2i>::set(Primitive, SimpleClass) generated
// ("std::vector<cv::Vec2i>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Vec2i"]), _)]),
pub fn std_vectorLcv_Vec2iG_set_size_t_const_Vec2i(instance: *mut c_void, index: size_t, val: *const core::Vec2i);
// std::vector<cv::Vec2i>::clone() generated
// ("std::vector<cv::Vec2i>::clone", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec2iG_clone_const(instance: *const c_void) -> *mut c_void;
// std::vector<cv::Vec2i>::data() generated
// ("std::vector<cv::Vec2i>::data", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec2iG_data_const(instance: *const c_void) -> *const core::Vec2i;
// std::vector<cv::Vec2i>::dataMut() generated
// ("std::vector<cv::Vec2i>::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec2iG_dataMut(instance: *mut c_void) -> *mut core::Vec2i;
// cv::fromSlice(SimpleClass, Primitive) generated
// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::Vec2i*", "size_t"]), _)]),
pub fn cv_fromSlice_const_const_Vec2iX_size_t(data: *const core::Vec2i, len: size_t) -> *mut c_void;
// std::vector<cv::Vec2i>::inputArray() generated
// ("std::vector<cv::Vec2i>::inputArray", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec2iG_inputArray_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<cv::Vec2i>::outputArray() generated
// ("std::vector<cv::Vec2i>::outputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec2iG_outputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<cv::Vec2i>::inputOutputArray() generated
// ("std::vector<cv::Vec2i>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec2iG_inputOutputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
