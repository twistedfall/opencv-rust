// std::vector<cv::Point3d>::new() generated
// ("std::vector<cv::Point3d>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Point3dG_new_const() -> *mut c_void;
// std::vector<cv::Point3d>::delete() generated
// ("std::vector<cv::Point3d>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Point3dG_delete(instance: *mut c_void);
// std::vector<cv::Point3d>::len() generated
// ("std::vector<cv::Point3d>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Point3dG_len_const(instance: *const c_void) -> size_t;
// std::vector<cv::Point3d>::isEmpty() generated
// ("std::vector<cv::Point3d>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Point3dG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<cv::Point3d>::capacity() generated
// ("std::vector<cv::Point3d>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Point3dG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<cv::Point3d>::shrinkToFit() generated
// ("std::vector<cv::Point3d>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Point3dG_shrinkToFit(instance: *mut c_void);
// std::vector<cv::Point3d>::reserve(Primitive) generated
// ("std::vector<cv::Point3d>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLcv_Point3dG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<cv::Point3d>::remove(Primitive) generated
// ("std::vector<cv::Point3d>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_Point3dG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<cv::Point3d>::swap(Primitive, Primitive) generated
// ("std::vector<cv::Point3d>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLcv_Point3dG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<cv::Point3d>::clear() generated
// ("std::vector<cv::Point3d>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Point3dG_clear(instance: *mut c_void);
// std::vector<cv::Point3d>::push(SimpleClass) generated
// ("std::vector<cv::Point3d>::push", vec![(pred!(mut, ["val"], ["const cv::Point3d"]), _)]),
pub fn std_vectorLcv_Point3dG_push_const_Point3d(instance: *mut c_void, val: *const core::Point3d);
// std::vector<cv::Point3d>::insert(Primitive, SimpleClass) generated
// ("std::vector<cv::Point3d>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Point3d"]), _)]),
pub fn std_vectorLcv_Point3dG_insert_size_t_const_Point3d(instance: *mut c_void, index: size_t, val: *const core::Point3d);
// std::vector<cv::Point3d>::get(Primitive) generated
// ("std::vector<cv::Point3d>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_Point3dG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut core::Point3d);
// std::vector<cv::Point3d>::set(Primitive, SimpleClass) generated
// ("std::vector<cv::Point3d>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Point3d"]), _)]),
pub fn std_vectorLcv_Point3dG_set_size_t_const_Point3d(instance: *mut c_void, index: size_t, val: *const core::Point3d);
// std::vector<cv::Point3d>::clone() generated
// ("std::vector<cv::Point3d>::clone", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Point3dG_clone_const(instance: *const c_void) -> *mut c_void;
// std::vector<cv::Point3d>::data() generated
// ("std::vector<cv::Point3d>::data", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Point3dG_data_const(instance: *const c_void) -> *const core::Point3d;
// std::vector<cv::Point3d>::dataMut() generated
// ("std::vector<cv::Point3d>::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Point3dG_dataMut(instance: *mut c_void) -> *mut core::Point3d;
// cv::fromSlice(SimpleClass, Primitive) generated
// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::Point3d*", "size_t"]), _)]),
pub fn cv_fromSlice_const_const_Point3dX_size_t(data: *const core::Point3d, len: size_t) -> *mut c_void;
// std::vector<cv::Point3d>::inputArray() generated
// ("std::vector<cv::Point3d>::inputArray", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_Point3dG_inputArray_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<cv::Point3d>::outputArray() generated
// ("std::vector<cv::Point3d>::outputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Point3dG_outputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<cv::Point3d>::inputOutputArray() generated
// ("std::vector<cv::Point3d>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_Point3dG_inputOutputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
