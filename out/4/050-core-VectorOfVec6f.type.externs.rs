// std::vector<cv::Vec6f>::new() generated
// ("std::vector<cv::Vec6f>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec6fG_new_const() -> *mut c_void;
// std::vector<cv::Vec6f>::delete() generated
// ("std::vector<cv::Vec6f>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec6fG_delete(instance: *mut c_void);
// std::vector<cv::Vec6f>::len() generated
// ("std::vector<cv::Vec6f>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec6fG_len_const(instance: *const c_void) -> size_t;
// std::vector<cv::Vec6f>::isEmpty() generated
// ("std::vector<cv::Vec6f>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec6fG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<cv::Vec6f>::capacity() generated
// ("std::vector<cv::Vec6f>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec6fG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<cv::Vec6f>::shrinkToFit() generated
// ("std::vector<cv::Vec6f>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec6fG_shrinkToFit(instance: *mut c_void);
// std::vector<cv::Vec6f>::reserve(Primitive) generated
// ("std::vector<cv::Vec6f>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLcv_Vec6fG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<cv::Vec6f>::remove(Primitive) generated
// ("std::vector<cv::Vec6f>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_Vec6fG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<cv::Vec6f>::swap(Primitive, Primitive) generated
// ("std::vector<cv::Vec6f>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLcv_Vec6fG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<cv::Vec6f>::clear() generated
// ("std::vector<cv::Vec6f>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec6fG_clear(instance: *mut c_void);
// std::vector<cv::Vec6f>::push(SimpleClass) generated
// ("std::vector<cv::Vec6f>::push", vec![(pred!(mut, ["val"], ["const cv::Vec6f"]), _)]),
pub fn std_vectorLcv_Vec6fG_push_const_Vec6f(instance: *mut c_void, val: *const core::Vec6f);
// std::vector<cv::Vec6f>::insert(Primitive, SimpleClass) generated
// ("std::vector<cv::Vec6f>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Vec6f"]), _)]),
pub fn std_vectorLcv_Vec6fG_insert_size_t_const_Vec6f(instance: *mut c_void, index: size_t, val: *const core::Vec6f);
// std::vector<cv::Vec6f>::get(Primitive) generated
// ("std::vector<cv::Vec6f>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_Vec6fG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut core::Vec6f);
// std::vector<cv::Vec6f>::set(Primitive, SimpleClass) generated
// ("std::vector<cv::Vec6f>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Vec6f"]), _)]),
pub fn std_vectorLcv_Vec6fG_set_size_t_const_Vec6f(instance: *mut c_void, index: size_t, val: *const core::Vec6f);
// std::vector<cv::Vec6f>::clone() generated
// ("std::vector<cv::Vec6f>::clone", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec6fG_clone_const(instance: *const c_void) -> *mut c_void;
// std::vector<cv::Vec6f>::data() generated
// ("std::vector<cv::Vec6f>::data", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec6fG_data_const(instance: *const c_void) -> *const core::Vec6f;
// std::vector<cv::Vec6f>::dataMut() generated
// ("std::vector<cv::Vec6f>::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec6fG_dataMut(instance: *mut c_void) -> *mut core::Vec6f;
// cv::fromSlice(SimpleClass, Primitive) generated
// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::Vec6f*", "size_t"]), _)]),
pub fn cv_fromSlice_const_const_Vec6fX_size_t(data: *const core::Vec6f, len: size_t) -> *mut c_void;
// std::vector<cv::Vec6f>::inputArray() generated
// ("std::vector<cv::Vec6f>::inputArray", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec6fG_inputArray_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<cv::Vec6f>::outputArray() generated
// ("std::vector<cv::Vec6f>::outputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec6fG_outputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<cv::Vec6f>::inputOutputArray() generated
// ("std::vector<cv::Vec6f>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec6fG_inputOutputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
