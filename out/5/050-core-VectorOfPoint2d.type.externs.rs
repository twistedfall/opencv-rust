// std::vector<cv::Point2d>::new() generated
// ("std::vector<cv::Point2d>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Point2dG_new_const() -> *mut c_void;
// std::vector<cv::Point2d>::delete() generated
// ("std::vector<cv::Point2d>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Point2dG_delete(instance: *mut c_void);
// std::vector<cv::Point2d>::len() generated
// ("std::vector<cv::Point2d>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Point2dG_len_const(instance: *const c_void) -> size_t;
// std::vector<cv::Point2d>::isEmpty() generated
// ("std::vector<cv::Point2d>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Point2dG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<cv::Point2d>::capacity() generated
// ("std::vector<cv::Point2d>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Point2dG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<cv::Point2d>::shrinkToFit() generated
// ("std::vector<cv::Point2d>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Point2dG_shrinkToFit(instance: *mut c_void);
// std::vector<cv::Point2d>::reserve(Primitive) generated
// ("std::vector<cv::Point2d>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLcv_Point2dG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<cv::Point2d>::remove(Primitive) generated
// ("std::vector<cv::Point2d>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_Point2dG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<cv::Point2d>::swap(Primitive, Primitive) generated
// ("std::vector<cv::Point2d>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLcv_Point2dG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<cv::Point2d>::clear() generated
// ("std::vector<cv::Point2d>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Point2dG_clear(instance: *mut c_void);
// std::vector<cv::Point2d>::push(SimpleClass) generated
// ("std::vector<cv::Point2d>::push", vec![(pred!(mut, ["val"], ["const cv::Point2d"]), _)]),
pub fn std_vectorLcv_Point2dG_push_const_Point2d(instance: *mut c_void, val: *const core::Point2d);
// std::vector<cv::Point2d>::insert(Primitive, SimpleClass) generated
// ("std::vector<cv::Point2d>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Point2d"]), _)]),
pub fn std_vectorLcv_Point2dG_insert_size_t_const_Point2d(instance: *mut c_void, index: size_t, val: *const core::Point2d);
// std::vector<cv::Point2d>::get(Primitive) generated
// ("std::vector<cv::Point2d>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_Point2dG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut core::Point2d);
// std::vector<cv::Point2d>::set(Primitive, SimpleClass) generated
// ("std::vector<cv::Point2d>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Point2d"]), _)]),
pub fn std_vectorLcv_Point2dG_set_size_t_const_Point2d(instance: *mut c_void, index: size_t, val: *const core::Point2d);
// std::vector<cv::Point2d>::clone() generated
// ("std::vector<cv::Point2d>::clone", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Point2dG_clone_const(instance: *const c_void) -> *mut c_void;
// std::vector<cv::Point2d>::data() generated
// ("std::vector<cv::Point2d>::data", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Point2dG_data_const(instance: *const c_void) -> *const core::Point2d;
// std::vector<cv::Point2d>::dataMut() generated
// ("std::vector<cv::Point2d>::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Point2dG_dataMut(instance: *mut c_void) -> *mut core::Point2d;
// cv::fromSlice(SimpleClass, Primitive) generated
// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::Point2d*", "size_t"]), _)]),
pub fn cv_fromSlice_const_const_Point2dX_size_t(data: *const core::Point2d, len: size_t) -> *mut c_void;
// std::vector<cv::Point2d>::inputArray() generated
// ("std::vector<cv::Point2d>::inputArray", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Point2dG_inputArray_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<cv::Point2d>::outputArray() generated
// ("std::vector<cv::Point2d>::outputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Point2dG_outputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<cv::Point2d>::inputOutputArray() generated
// ("std::vector<cv::Point2d>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Point2dG_inputOutputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
