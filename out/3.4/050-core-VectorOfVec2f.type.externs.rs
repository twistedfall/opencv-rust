// std::vector<cv::Vec2f>::new() generated
// ("std::vector<cv::Vec2f>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec2fG_new_const() -> *mut c_void;
// std::vector<cv::Vec2f>::delete() generated
// ("std::vector<cv::Vec2f>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec2fG_delete(instance: *mut c_void);
// std::vector<cv::Vec2f>::len() generated
// ("std::vector<cv::Vec2f>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec2fG_len_const(instance: *const c_void) -> size_t;
// std::vector<cv::Vec2f>::isEmpty() generated
// ("std::vector<cv::Vec2f>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec2fG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<cv::Vec2f>::capacity() generated
// ("std::vector<cv::Vec2f>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec2fG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<cv::Vec2f>::shrinkToFit() generated
// ("std::vector<cv::Vec2f>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec2fG_shrinkToFit(instance: *mut c_void);
// std::vector<cv::Vec2f>::reserve(Primitive) generated
// ("std::vector<cv::Vec2f>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLcv_Vec2fG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<cv::Vec2f>::remove(Primitive) generated
// ("std::vector<cv::Vec2f>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_Vec2fG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<cv::Vec2f>::swap(Primitive, Primitive) generated
// ("std::vector<cv::Vec2f>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLcv_Vec2fG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<cv::Vec2f>::clear() generated
// ("std::vector<cv::Vec2f>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec2fG_clear(instance: *mut c_void);
// std::vector<cv::Vec2f>::push(SimpleClass) generated
// ("std::vector<cv::Vec2f>::push", vec![(pred!(mut, ["val"], ["const cv::Vec2f"]), _)]),
pub fn std_vectorLcv_Vec2fG_push_const_Vec2f(instance: *mut c_void, val: *const core::Vec2f);
// std::vector<cv::Vec2f>::insert(Primitive, SimpleClass) generated
// ("std::vector<cv::Vec2f>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Vec2f"]), _)]),
pub fn std_vectorLcv_Vec2fG_insert_size_t_const_Vec2f(instance: *mut c_void, index: size_t, val: *const core::Vec2f);
// std::vector<cv::Vec2f>::get(Primitive) generated
// ("std::vector<cv::Vec2f>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_Vec2fG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut core::Vec2f);
// std::vector<cv::Vec2f>::set(Primitive, SimpleClass) generated
// ("std::vector<cv::Vec2f>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Vec2f"]), _)]),
pub fn std_vectorLcv_Vec2fG_set_size_t_const_Vec2f(instance: *mut c_void, index: size_t, val: *const core::Vec2f);
// std::vector<cv::Vec2f>::clone() generated
// ("std::vector<cv::Vec2f>::clone", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec2fG_clone_const(instance: *const c_void) -> *mut c_void;
// std::vector<cv::Vec2f>::data() generated
// ("std::vector<cv::Vec2f>::data", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec2fG_data_const(instance: *const c_void) -> *const core::Vec2f;
// std::vector<cv::Vec2f>::dataMut() generated
// ("std::vector<cv::Vec2f>::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec2fG_dataMut(instance: *mut c_void) -> *mut core::Vec2f;
// cv::fromSlice(SimpleClass, Primitive) generated
// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::Vec2f*", "size_t"]), _)]),
pub fn cv_fromSlice_const_const_Vec2fX_size_t(data: *const core::Vec2f, len: size_t) -> *mut c_void;
// std::vector<cv::Vec2f>::inputArray() generated
// ("std::vector<cv::Vec2f>::inputArray", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec2fG_inputArray_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<cv::Vec2f>::outputArray() generated
// ("std::vector<cv::Vec2f>::outputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec2fG_outputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<cv::Vec2f>::inputOutputArray() generated
// ("std::vector<cv::Vec2f>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec2fG_inputOutputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
