// std::vector<cv::Vec4f>::new() generated
// ("std::vector<cv::Vec4f>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec4fG_new_const() -> *mut c_void;
// std::vector<cv::Vec4f>::delete() generated
// ("std::vector<cv::Vec4f>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec4fG_delete(instance: *mut c_void);
// std::vector<cv::Vec4f>::len() generated
// ("std::vector<cv::Vec4f>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec4fG_len_const(instance: *const c_void) -> size_t;
// std::vector<cv::Vec4f>::isEmpty() generated
// ("std::vector<cv::Vec4f>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec4fG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<cv::Vec4f>::capacity() generated
// ("std::vector<cv::Vec4f>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec4fG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<cv::Vec4f>::shrinkToFit() generated
// ("std::vector<cv::Vec4f>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec4fG_shrinkToFit(instance: *mut c_void);
// std::vector<cv::Vec4f>::reserve(Primitive) generated
// ("std::vector<cv::Vec4f>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLcv_Vec4fG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<cv::Vec4f>::remove(Primitive) generated
// ("std::vector<cv::Vec4f>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_Vec4fG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<cv::Vec4f>::swap(Primitive, Primitive) generated
// ("std::vector<cv::Vec4f>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLcv_Vec4fG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<cv::Vec4f>::clear() generated
// ("std::vector<cv::Vec4f>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec4fG_clear(instance: *mut c_void);
// std::vector<cv::Vec4f>::push(SimpleClass) generated
// ("std::vector<cv::Vec4f>::push", vec![(pred!(mut, ["val"], ["const cv::Vec4f"]), _)]),
pub fn std_vectorLcv_Vec4fG_push_const_Vec4f(instance: *mut c_void, val: *const core::Vec4f);
// std::vector<cv::Vec4f>::insert(Primitive, SimpleClass) generated
// ("std::vector<cv::Vec4f>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Vec4f"]), _)]),
pub fn std_vectorLcv_Vec4fG_insert_size_t_const_Vec4f(instance: *mut c_void, index: size_t, val: *const core::Vec4f);
// std::vector<cv::Vec4f>::get(Primitive) generated
// ("std::vector<cv::Vec4f>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_Vec4fG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut core::Vec4f);
// std::vector<cv::Vec4f>::set(Primitive, SimpleClass) generated
// ("std::vector<cv::Vec4f>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Vec4f"]), _)]),
pub fn std_vectorLcv_Vec4fG_set_size_t_const_Vec4f(instance: *mut c_void, index: size_t, val: *const core::Vec4f);
// std::vector<cv::Vec4f>::clone() generated
// ("std::vector<cv::Vec4f>::clone", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec4fG_clone_const(instance: *const c_void) -> *mut c_void;
// std::vector<cv::Vec4f>::data() generated
// ("std::vector<cv::Vec4f>::data", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec4fG_data_const(instance: *const c_void) -> *const core::Vec4f;
// std::vector<cv::Vec4f>::dataMut() generated
// ("std::vector<cv::Vec4f>::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec4fG_dataMut(instance: *mut c_void) -> *mut core::Vec4f;
// cv::fromSlice(SimpleClass, Primitive) generated
// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::Vec4f*", "size_t"]), _)]),
pub fn cv_fromSlice_const_const_Vec4fX_size_t(data: *const core::Vec4f, len: size_t) -> *mut c_void;
// std::vector<cv::Vec4f>::inputArray() generated
// ("std::vector<cv::Vec4f>::inputArray", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec4fG_inputArray_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<cv::Vec4f>::outputArray() generated
// ("std::vector<cv::Vec4f>::outputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec4fG_outputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<cv::Vec4f>::inputOutputArray() generated
// ("std::vector<cv::Vec4f>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec4fG_inputOutputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
