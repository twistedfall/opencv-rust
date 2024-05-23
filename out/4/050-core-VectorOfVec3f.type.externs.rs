// std::vector<cv::Vec3f>::new() generated
// ("std::vector<cv::Vec3f>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec3fG_new_const() -> *mut c_void;
// std::vector<cv::Vec3f>::delete() generated
// ("std::vector<cv::Vec3f>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec3fG_delete(instance: *mut c_void);
// std::vector<cv::Vec3f>::len() generated
// ("std::vector<cv::Vec3f>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec3fG_len_const(instance: *const c_void) -> size_t;
// std::vector<cv::Vec3f>::isEmpty() generated
// ("std::vector<cv::Vec3f>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec3fG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<cv::Vec3f>::capacity() generated
// ("std::vector<cv::Vec3f>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec3fG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<cv::Vec3f>::shrinkToFit() generated
// ("std::vector<cv::Vec3f>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec3fG_shrinkToFit(instance: *mut c_void);
// std::vector<cv::Vec3f>::reserve(Primitive) generated
// ("std::vector<cv::Vec3f>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLcv_Vec3fG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<cv::Vec3f>::remove(Primitive) generated
// ("std::vector<cv::Vec3f>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_Vec3fG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<cv::Vec3f>::swap(Primitive, Primitive) generated
// ("std::vector<cv::Vec3f>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLcv_Vec3fG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<cv::Vec3f>::clear() generated
// ("std::vector<cv::Vec3f>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec3fG_clear(instance: *mut c_void);
// std::vector<cv::Vec3f>::push(SimpleClass) generated
// ("std::vector<cv::Vec3f>::push", vec![(pred!(mut, ["val"], ["const cv::Vec3f"]), _)]),
pub fn std_vectorLcv_Vec3fG_push_const_Vec3f(instance: *mut c_void, val: *const core::Vec3f);
// std::vector<cv::Vec3f>::insert(Primitive, SimpleClass) generated
// ("std::vector<cv::Vec3f>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Vec3f"]), _)]),
pub fn std_vectorLcv_Vec3fG_insert_size_t_const_Vec3f(instance: *mut c_void, index: size_t, val: *const core::Vec3f);
// std::vector<cv::Vec3f>::get(Primitive) generated
// ("std::vector<cv::Vec3f>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_Vec3fG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut core::Vec3f);
// std::vector<cv::Vec3f>::set(Primitive, SimpleClass) generated
// ("std::vector<cv::Vec3f>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Vec3f"]), _)]),
pub fn std_vectorLcv_Vec3fG_set_size_t_const_Vec3f(instance: *mut c_void, index: size_t, val: *const core::Vec3f);
// std::vector<cv::Vec3f>::clone() generated
// ("std::vector<cv::Vec3f>::clone", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec3fG_clone_const(instance: *const c_void) -> *mut c_void;
// std::vector<cv::Vec3f>::data() generated
// ("std::vector<cv::Vec3f>::data", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec3fG_data_const(instance: *const c_void) -> *const core::Vec3f;
// std::vector<cv::Vec3f>::dataMut() generated
// ("std::vector<cv::Vec3f>::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec3fG_dataMut(instance: *mut c_void) -> *mut core::Vec3f;
// cv::fromSlice(SimpleClass, Primitive) generated
// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::Vec3f*", "size_t"]), _)]),
pub fn cv_fromSlice_const_const_Vec3fX_size_t(data: *const core::Vec3f, len: size_t) -> *mut c_void;
// std::vector<cv::Vec3f>::inputArray() generated
// ("std::vector<cv::Vec3f>::inputArray", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Vec3fG_inputArray_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<cv::Vec3f>::outputArray() generated
// ("std::vector<cv::Vec3f>::outputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec3fG_outputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<cv::Vec3f>::inputOutputArray() generated
// ("std::vector<cv::Vec3f>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Vec3fG_inputOutputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
