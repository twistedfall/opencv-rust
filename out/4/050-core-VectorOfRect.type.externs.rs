// std::vector<cv::Rect>::new() generated
// ("std::vector<cv::Rect>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_RectG_new_const() -> *mut c_void;
// std::vector<cv::Rect>::delete() generated
// ("std::vector<cv::Rect>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_RectG_delete(instance: *mut c_void);
// std::vector<cv::Rect>::len() generated
// ("std::vector<cv::Rect>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_RectG_len_const(instance: *const c_void) -> size_t;
// std::vector<cv::Rect>::isEmpty() generated
// ("std::vector<cv::Rect>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_RectG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<cv::Rect>::capacity() generated
// ("std::vector<cv::Rect>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_RectG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<cv::Rect>::shrinkToFit() generated
// ("std::vector<cv::Rect>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_RectG_shrinkToFit(instance: *mut c_void);
// std::vector<cv::Rect>::reserve(Primitive) generated
// ("std::vector<cv::Rect>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLcv_RectG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<cv::Rect>::remove(Primitive) generated
// ("std::vector<cv::Rect>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_RectG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<cv::Rect>::swap(Primitive, Primitive) generated
// ("std::vector<cv::Rect>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLcv_RectG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<cv::Rect>::clear() generated
// ("std::vector<cv::Rect>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_RectG_clear(instance: *mut c_void);
// std::vector<cv::Rect>::push(SimpleClass) generated
// ("std::vector<cv::Rect>::push", vec![(pred!(mut, ["val"], ["const cv::Rect"]), _)]),
pub fn std_vectorLcv_RectG_push_const_Rect(instance: *mut c_void, val: *const core::Rect);
// std::vector<cv::Rect>::insert(Primitive, SimpleClass) generated
// ("std::vector<cv::Rect>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Rect"]), _)]),
pub fn std_vectorLcv_RectG_insert_size_t_const_Rect(instance: *mut c_void, index: size_t, val: *const core::Rect);
// std::vector<cv::Rect>::get(Primitive) generated
// ("std::vector<cv::Rect>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_RectG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut core::Rect);
// std::vector<cv::Rect>::set(Primitive, SimpleClass) generated
// ("std::vector<cv::Rect>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Rect"]), _)]),
pub fn std_vectorLcv_RectG_set_size_t_const_Rect(instance: *mut c_void, index: size_t, val: *const core::Rect);
// std::vector<cv::Rect>::clone() generated
// ("std::vector<cv::Rect>::clone", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_RectG_clone_const(instance: *const c_void) -> *mut c_void;
// std::vector<cv::Rect>::data() generated
// ("std::vector<cv::Rect>::data", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_RectG_data_const(instance: *const c_void) -> *const core::Rect;
// std::vector<cv::Rect>::dataMut() generated
// ("std::vector<cv::Rect>::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_RectG_dataMut(instance: *mut c_void) -> *mut core::Rect;
// cv::fromSlice(SimpleClass, Primitive) generated
// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::Rect*", "size_t"]), _)]),
pub fn cv_fromSlice_const_const_RectX_size_t(data: *const core::Rect, len: size_t) -> *mut c_void;
// std::vector<cv::Rect>::inputArray() generated
// ("std::vector<cv::Rect>::inputArray", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_RectG_inputArray_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<cv::Rect>::outputArray() generated
// ("std::vector<cv::Rect>::outputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_RectG_outputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<cv::Rect>::inputOutputArray() generated
// ("std::vector<cv::Rect>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_RectG_inputOutputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
