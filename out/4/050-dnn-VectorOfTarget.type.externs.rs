// std::vector<cv::dnn::Target>::new() generated
// ("std::vector<cv::dnn::Target>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_dnn_TargetG_new_const() -> *mut c_void;
// std::vector<cv::dnn::Target>::delete() generated
// ("std::vector<cv::dnn::Target>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_dnn_TargetG_delete(instance: *mut c_void);
// std::vector<cv::dnn::Target>::len() generated
// ("std::vector<cv::dnn::Target>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_dnn_TargetG_len_const(instance: *const c_void) -> size_t;
// std::vector<cv::dnn::Target>::isEmpty() generated
// ("std::vector<cv::dnn::Target>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_dnn_TargetG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<cv::dnn::Target>::capacity() generated
// ("std::vector<cv::dnn::Target>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_dnn_TargetG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<cv::dnn::Target>::shrinkToFit() generated
// ("std::vector<cv::dnn::Target>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_dnn_TargetG_shrinkToFit(instance: *mut c_void);
// std::vector<cv::dnn::Target>::reserve(Primitive) generated
// ("std::vector<cv::dnn::Target>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLcv_dnn_TargetG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<cv::dnn::Target>::remove(Primitive) generated
// ("std::vector<cv::dnn::Target>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_dnn_TargetG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<cv::dnn::Target>::swap(Primitive, Primitive) generated
// ("std::vector<cv::dnn::Target>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLcv_dnn_TargetG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<cv::dnn::Target>::clear() generated
// ("std::vector<cv::dnn::Target>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_dnn_TargetG_clear(instance: *mut c_void);
// std::vector<cv::dnn::Target>::push(Enum) generated
// ("std::vector<cv::dnn::Target>::push", vec![(pred!(mut, ["val"], ["const cv::dnn::Target"]), _)]),
pub fn std_vectorLcv_dnn_TargetG_push_const_Target(instance: *mut c_void, val: crate::dnn::Target);
// std::vector<cv::dnn::Target>::insert(Primitive, Enum) generated
// ("std::vector<cv::dnn::Target>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::dnn::Target"]), _)]),
pub fn std_vectorLcv_dnn_TargetG_insert_size_t_const_Target(instance: *mut c_void, index: size_t, val: crate::dnn::Target);
// std::vector<cv::dnn::Target>::get(Primitive) generated
// ("std::vector<cv::dnn::Target>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_dnn_TargetG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut crate::dnn::Target);
// std::vector<cv::dnn::Target>::set(Primitive, Enum) generated
// ("std::vector<cv::dnn::Target>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::dnn::Target"]), _)]),
pub fn std_vectorLcv_dnn_TargetG_set_size_t_const_Target(instance: *mut c_void, index: size_t, val: crate::dnn::Target);
// std::vector<cv::dnn::Target>::clone() generated
// ("std::vector<cv::dnn::Target>::clone", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_dnn_TargetG_clone_const(instance: *const c_void) -> *mut c_void;
// std::vector<cv::dnn::Target>::data() generated
// ("std::vector<cv::dnn::Target>::data", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_dnn_TargetG_data_const(instance: *const c_void) -> *const crate::dnn::Target;
// std::vector<cv::dnn::Target>::dataMut() generated
// ("std::vector<cv::dnn::Target>::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_dnn_TargetG_dataMut(instance: *mut c_void) -> *mut crate::dnn::Target;
// cv::fromSlice(Enum, Primitive) generated
// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::dnn::Target*", "size_t"]), _)]),
pub fn cv_fromSlice_const_const_TargetX_size_t(data: *const crate::dnn::Target, len: size_t) -> *mut c_void;
