// std::vector<cv::Point>::new() generated
// ("std::vector<cv::Point>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_PointG_new_const() -> *mut c_void;
// std::vector<cv::Point>::delete() generated
// ("std::vector<cv::Point>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_PointG_delete(instance: *mut c_void);
// std::vector<cv::Point>::len() generated
// ("std::vector<cv::Point>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_PointG_len_const(instance: *const c_void) -> size_t;
// std::vector<cv::Point>::isEmpty() generated
// ("std::vector<cv::Point>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_PointG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<cv::Point>::capacity() generated
// ("std::vector<cv::Point>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_PointG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<cv::Point>::shrinkToFit() generated
// ("std::vector<cv::Point>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_PointG_shrinkToFit(instance: *mut c_void);
// std::vector<cv::Point>::reserve(Primitive) generated
// ("std::vector<cv::Point>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLcv_PointG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<cv::Point>::remove(Primitive) generated
// ("std::vector<cv::Point>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_PointG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<cv::Point>::swap(Primitive, Primitive) generated
// ("std::vector<cv::Point>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLcv_PointG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<cv::Point>::clear() generated
// ("std::vector<cv::Point>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_PointG_clear(instance: *mut c_void);
// std::vector<cv::Point>::push(SimpleClass) generated
// ("std::vector<cv::Point>::push", vec![(pred!(mut, ["val"], ["const cv::Point"]), _)]),
pub fn std_vectorLcv_PointG_push_const_Point(instance: *mut c_void, val: *const core::Point);
// std::vector<cv::Point>::insert(Primitive, SimpleClass) generated
// ("std::vector<cv::Point>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Point"]), _)]),
pub fn std_vectorLcv_PointG_insert_size_t_const_Point(instance: *mut c_void, index: size_t, val: *const core::Point);
// std::vector<cv::Point>::get(Primitive) generated
// ("std::vector<cv::Point>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_PointG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut core::Point);
// std::vector<cv::Point>::set(Primitive, SimpleClass) generated
// ("std::vector<cv::Point>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Point"]), _)]),
pub fn std_vectorLcv_PointG_set_size_t_const_Point(instance: *mut c_void, index: size_t, val: *const core::Point);
// std::vector<cv::Point>::clone() generated
// ("std::vector<cv::Point>::clone", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_PointG_clone_const(instance: *const c_void) -> *mut c_void;
// std::vector<cv::Point>::data() generated
// ("std::vector<cv::Point>::data", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_PointG_data_const(instance: *const c_void) -> *const core::Point;
// std::vector<cv::Point>::dataMut() generated
// ("std::vector<cv::Point>::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_PointG_dataMut(instance: *mut c_void) -> *mut core::Point;
// cv::fromSlice(SimpleClass, Primitive) generated
// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::Point*", "size_t"]), _)]),
pub fn cv_fromSlice_const_const_PointX_size_t(data: *const core::Point, len: size_t) -> *mut c_void;
// std::vector<cv::Point>::inputArray() generated
// ("std::vector<cv::Point>::inputArray", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_PointG_inputArray_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<cv::Point>::outputArray() generated
// ("std::vector<cv::Point>::outputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_PointG_outputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<cv::Point>::inputOutputArray() generated
// ("std::vector<cv::Point>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_PointG_inputOutputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
