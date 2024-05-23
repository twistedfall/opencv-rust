// std::vector<cv::Point3f>::new() generated
// ("std::vector<cv::Point3f>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Point3fG_new_const() -> *mut c_void;
// std::vector<cv::Point3f>::delete() generated
// ("std::vector<cv::Point3f>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Point3fG_delete(instance: *mut c_void);
// std::vector<cv::Point3f>::len() generated
// ("std::vector<cv::Point3f>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Point3fG_len_const(instance: *const c_void) -> size_t;
// std::vector<cv::Point3f>::isEmpty() generated
// ("std::vector<cv::Point3f>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Point3fG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<cv::Point3f>::capacity() generated
// ("std::vector<cv::Point3f>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Point3fG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<cv::Point3f>::shrinkToFit() generated
// ("std::vector<cv::Point3f>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Point3fG_shrinkToFit(instance: *mut c_void);
// std::vector<cv::Point3f>::reserve(Primitive) generated
// ("std::vector<cv::Point3f>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLcv_Point3fG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<cv::Point3f>::remove(Primitive) generated
// ("std::vector<cv::Point3f>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_Point3fG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<cv::Point3f>::swap(Primitive, Primitive) generated
// ("std::vector<cv::Point3f>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLcv_Point3fG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<cv::Point3f>::clear() generated
// ("std::vector<cv::Point3f>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Point3fG_clear(instance: *mut c_void);
// std::vector<cv::Point3f>::push(SimpleClass) generated
// ("std::vector<cv::Point3f>::push", vec![(pred!(mut, ["val"], ["const cv::Point3f"]), _)]),
pub fn std_vectorLcv_Point3fG_push_const_Point3f(instance: *mut c_void, val: *const core::Point3f);
// std::vector<cv::Point3f>::insert(Primitive, SimpleClass) generated
// ("std::vector<cv::Point3f>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Point3f"]), _)]),
pub fn std_vectorLcv_Point3fG_insert_size_t_const_Point3f(instance: *mut c_void, index: size_t, val: *const core::Point3f);
// std::vector<cv::Point3f>::get(Primitive) generated
// ("std::vector<cv::Point3f>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_Point3fG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut core::Point3f);
// std::vector<cv::Point3f>::set(Primitive, SimpleClass) generated
// ("std::vector<cv::Point3f>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Point3f"]), _)]),
pub fn std_vectorLcv_Point3fG_set_size_t_const_Point3f(instance: *mut c_void, index: size_t, val: *const core::Point3f);
// std::vector<cv::Point3f>::clone() generated
// ("std::vector<cv::Point3f>::clone", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Point3fG_clone_const(instance: *const c_void) -> *mut c_void;
// std::vector<cv::Point3f>::data() generated
// ("std::vector<cv::Point3f>::data", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Point3fG_data_const(instance: *const c_void) -> *const core::Point3f;
// std::vector<cv::Point3f>::dataMut() generated
// ("std::vector<cv::Point3f>::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Point3fG_dataMut(instance: *mut c_void) -> *mut core::Point3f;
// cv::fromSlice(SimpleClass, Primitive) generated
// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::Point3f*", "size_t"]), _)]),
pub fn cv_fromSlice_const_const_Point3fX_size_t(data: *const core::Point3f, len: size_t) -> *mut c_void;
// std::vector<cv::Point3f>::inputArray() generated
// ("std::vector<cv::Point3f>::inputArray", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Point3fG_inputArray_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<cv::Point3f>::outputArray() generated
// ("std::vector<cv::Point3f>::outputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Point3fG_outputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<cv::Point3f>::inputOutputArray() generated
// ("std::vector<cv::Point3f>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Point3fG_inputOutputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
