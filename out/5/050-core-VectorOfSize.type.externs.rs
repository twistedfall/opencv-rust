// std::vector<cv::Size>::new() generated
// ("std::vector<cv::Size>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_SizeG_new_const() -> *mut c_void;
// std::vector<cv::Size>::delete() generated
// ("std::vector<cv::Size>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_SizeG_delete(instance: *mut c_void);
// std::vector<cv::Size>::len() generated
// ("std::vector<cv::Size>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_SizeG_len_const(instance: *const c_void) -> size_t;
// std::vector<cv::Size>::isEmpty() generated
// ("std::vector<cv::Size>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_SizeG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<cv::Size>::capacity() generated
// ("std::vector<cv::Size>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_SizeG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<cv::Size>::shrinkToFit() generated
// ("std::vector<cv::Size>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_SizeG_shrinkToFit(instance: *mut c_void);
// std::vector<cv::Size>::reserve(Primitive) generated
// ("std::vector<cv::Size>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLcv_SizeG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<cv::Size>::remove(Primitive) generated
// ("std::vector<cv::Size>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_SizeG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<cv::Size>::swap(Primitive, Primitive) generated
// ("std::vector<cv::Size>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLcv_SizeG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<cv::Size>::clear() generated
// ("std::vector<cv::Size>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_SizeG_clear(instance: *mut c_void);
// std::vector<cv::Size>::push(SimpleClass) generated
// ("std::vector<cv::Size>::push", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn std_vectorLcv_SizeG_push_const_Size(instance: *mut c_void, val: *const core::Size);
// std::vector<cv::Size>::insert(Primitive, SimpleClass) generated
// ("std::vector<cv::Size>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Size"]), _)]),
pub fn std_vectorLcv_SizeG_insert_size_t_const_Size(instance: *mut c_void, index: size_t, val: *const core::Size);
// std::vector<cv::Size>::get(Primitive) generated
// ("std::vector<cv::Size>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_SizeG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut core::Size);
// std::vector<cv::Size>::set(Primitive, SimpleClass) generated
// ("std::vector<cv::Size>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Size"]), _)]),
pub fn std_vectorLcv_SizeG_set_size_t_const_Size(instance: *mut c_void, index: size_t, val: *const core::Size);
// std::vector<cv::Size>::clone() generated
// ("std::vector<cv::Size>::clone", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_SizeG_clone_const(instance: *const c_void) -> *mut c_void;
// std::vector<cv::Size>::data() generated
// ("std::vector<cv::Size>::data", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_SizeG_data_const(instance: *const c_void) -> *const core::Size;
// std::vector<cv::Size>::dataMut() generated
// ("std::vector<cv::Size>::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_SizeG_dataMut(instance: *mut c_void) -> *mut core::Size;
// cv::fromSlice(SimpleClass, Primitive) generated
// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::Size*", "size_t"]), _)]),
pub fn cv_fromSlice_const_const_SizeX_size_t(data: *const core::Size, len: size_t) -> *mut c_void;
// std::vector<cv::Size>::inputArray() generated
// ("std::vector<cv::Size>::inputArray", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_SizeG_inputArray_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<cv::Size>::outputArray() generated
// ("std::vector<cv::Size>::outputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_SizeG_outputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<cv::Size>::inputOutputArray() generated
// ("std::vector<cv::Size>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_SizeG_inputOutputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
