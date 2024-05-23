// std::vector<cv::dnn::MatType>::new() generated
// ("std::vector<cv::dnn::MatType>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_dnn_MatTypeG_new_const() -> *mut c_void;
// std::vector<cv::dnn::MatType>::delete() generated
// ("std::vector<cv::dnn::MatType>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_dnn_MatTypeG_delete(instance: *mut c_void);
// std::vector<cv::dnn::MatType>::len() generated
// ("std::vector<cv::dnn::MatType>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_dnn_MatTypeG_len_const(instance: *const c_void) -> size_t;
// std::vector<cv::dnn::MatType>::isEmpty() generated
// ("std::vector<cv::dnn::MatType>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_dnn_MatTypeG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<cv::dnn::MatType>::capacity() generated
// ("std::vector<cv::dnn::MatType>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_dnn_MatTypeG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<cv::dnn::MatType>::shrinkToFit() generated
// ("std::vector<cv::dnn::MatType>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_dnn_MatTypeG_shrinkToFit(instance: *mut c_void);
// std::vector<cv::dnn::MatType>::reserve(Primitive) generated
// ("std::vector<cv::dnn::MatType>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLcv_dnn_MatTypeG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<cv::dnn::MatType>::remove(Primitive) generated
// ("std::vector<cv::dnn::MatType>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_dnn_MatTypeG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<cv::dnn::MatType>::swap(Primitive, Primitive) generated
// ("std::vector<cv::dnn::MatType>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLcv_dnn_MatTypeG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<cv::dnn::MatType>::clear() generated
// ("std::vector<cv::dnn::MatType>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_dnn_MatTypeG_clear(instance: *mut c_void);
// std::vector<cv::dnn::MatType>::push(Primitive) generated
// ("std::vector<cv::dnn::MatType>::push", vec![(pred!(mut, ["val"], ["const cv::dnn::MatType"]), _)]),
pub fn std_vectorLcv_dnn_MatTypeG_push_const_MatType(instance: *mut c_void, val: crate::dnn::MatType);
// std::vector<cv::dnn::MatType>::insert(Primitive, Primitive) generated
// ("std::vector<cv::dnn::MatType>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::dnn::MatType"]), _)]),
pub fn std_vectorLcv_dnn_MatTypeG_insert_size_t_const_MatType(instance: *mut c_void, index: size_t, val: crate::dnn::MatType);
// std::vector<cv::dnn::MatType>::get(Primitive) generated
// ("std::vector<cv::dnn::MatType>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_dnn_MatTypeG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut crate::dnn::MatType);
// std::vector<cv::dnn::MatType>::set(Primitive, Primitive) generated
// ("std::vector<cv::dnn::MatType>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::dnn::MatType"]), _)]),
pub fn std_vectorLcv_dnn_MatTypeG_set_size_t_const_MatType(instance: *mut c_void, index: size_t, val: crate::dnn::MatType);
// std::vector<cv::dnn::MatType>::clone() generated
// ("std::vector<cv::dnn::MatType>::clone", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_dnn_MatTypeG_clone_const(instance: *const c_void) -> *mut c_void;
// std::vector<cv::dnn::MatType>::data() generated
// ("std::vector<cv::dnn::MatType>::data", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_dnn_MatTypeG_data_const(instance: *const c_void) -> *const crate::dnn::MatType;
// std::vector<cv::dnn::MatType>::dataMut() generated
// ("std::vector<cv::dnn::MatType>::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_dnn_MatTypeG_dataMut(instance: *mut c_void) -> *mut crate::dnn::MatType;
// cv::fromSlice(Indirect, Primitive) generated
// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::dnn::MatType*", "size_t"]), _)]),
pub fn cv_fromSlice_const_const_MatTypeX_size_t(data: *const crate::dnn::MatType, len: size_t) -> *mut c_void;
// std::vector<cv::dnn::MatType>::inputArray() generated
// ("std::vector<cv::dnn::MatType>::inputArray", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_dnn_MatTypeG_inputArray_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<cv::dnn::MatType>::outputArray() generated
// ("std::vector<cv::dnn::MatType>::outputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_dnn_MatTypeG_outputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<cv::dnn::MatType>::inputOutputArray() generated
// ("std::vector<cv::dnn::MatType>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_dnn_MatTypeG_inputOutputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
