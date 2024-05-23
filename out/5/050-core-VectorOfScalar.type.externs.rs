// std::vector<cv::Scalar>::new() generated
// ("std::vector<cv::Scalar>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_ScalarG_new_const() -> *mut c_void;
// std::vector<cv::Scalar>::delete() generated
// ("std::vector<cv::Scalar>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_ScalarG_delete(instance: *mut c_void);
// std::vector<cv::Scalar>::len() generated
// ("std::vector<cv::Scalar>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_ScalarG_len_const(instance: *const c_void) -> size_t;
// std::vector<cv::Scalar>::isEmpty() generated
// ("std::vector<cv::Scalar>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_ScalarG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<cv::Scalar>::capacity() generated
// ("std::vector<cv::Scalar>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_ScalarG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<cv::Scalar>::shrinkToFit() generated
// ("std::vector<cv::Scalar>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_ScalarG_shrinkToFit(instance: *mut c_void);
// std::vector<cv::Scalar>::reserve(Primitive) generated
// ("std::vector<cv::Scalar>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLcv_ScalarG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<cv::Scalar>::remove(Primitive) generated
// ("std::vector<cv::Scalar>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_ScalarG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<cv::Scalar>::swap(Primitive, Primitive) generated
// ("std::vector<cv::Scalar>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLcv_ScalarG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<cv::Scalar>::clear() generated
// ("std::vector<cv::Scalar>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_ScalarG_clear(instance: *mut c_void);
// std::vector<cv::Scalar>::push(SimpleClass) generated
// ("std::vector<cv::Scalar>::push", vec![(pred!(mut, ["val"], ["const cv::Scalar"]), _)]),
pub fn std_vectorLcv_ScalarG_push_const_Scalar(instance: *mut c_void, val: *const core::Scalar);
// std::vector<cv::Scalar>::insert(Primitive, SimpleClass) generated
// ("std::vector<cv::Scalar>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Scalar"]), _)]),
pub fn std_vectorLcv_ScalarG_insert_size_t_const_Scalar(instance: *mut c_void, index: size_t, val: *const core::Scalar);
// std::vector<cv::Scalar>::get(Primitive) generated
// ("std::vector<cv::Scalar>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_ScalarG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut core::Scalar);
// std::vector<cv::Scalar>::set(Primitive, SimpleClass) generated
// ("std::vector<cv::Scalar>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Scalar"]), _)]),
pub fn std_vectorLcv_ScalarG_set_size_t_const_Scalar(instance: *mut c_void, index: size_t, val: *const core::Scalar);
// std::vector<cv::Scalar>::clone() generated
// ("std::vector<cv::Scalar>::clone", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_ScalarG_clone_const(instance: *const c_void) -> *mut c_void;
// std::vector<cv::Scalar>::data() generated
// ("std::vector<cv::Scalar>::data", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_ScalarG_data_const(instance: *const c_void) -> *const core::Scalar;
// std::vector<cv::Scalar>::dataMut() generated
// ("std::vector<cv::Scalar>::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_ScalarG_dataMut(instance: *mut c_void) -> *mut core::Scalar;
// cv::fromSlice(SimpleClass, Primitive) generated
// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::Scalar*", "size_t"]), _)]),
pub fn cv_fromSlice_const_const_ScalarX_size_t(data: *const core::Scalar, len: size_t) -> *mut c_void;
// std::vector<cv::Scalar>::inputArray() generated
// ("std::vector<cv::Scalar>::inputArray", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_ScalarG_inputArray_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<cv::Scalar>::outputArray() generated
// ("std::vector<cv::Scalar>::outputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_ScalarG_outputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<cv::Scalar>::inputOutputArray() generated
// ("std::vector<cv::Scalar>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_ScalarG_inputOutputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
