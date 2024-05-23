// std::vector<cv::GShape>::new() generated
// ("std::vector<cv::GShape>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_GShapeG_new_const() -> *mut c_void;
// std::vector<cv::GShape>::delete() generated
// ("std::vector<cv::GShape>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_GShapeG_delete(instance: *mut c_void);
// std::vector<cv::GShape>::len() generated
// ("std::vector<cv::GShape>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_GShapeG_len_const(instance: *const c_void) -> size_t;
// std::vector<cv::GShape>::isEmpty() generated
// ("std::vector<cv::GShape>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_GShapeG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<cv::GShape>::capacity() generated
// ("std::vector<cv::GShape>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_GShapeG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<cv::GShape>::shrinkToFit() generated
// ("std::vector<cv::GShape>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_GShapeG_shrinkToFit(instance: *mut c_void);
// std::vector<cv::GShape>::reserve(Primitive) generated
// ("std::vector<cv::GShape>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLcv_GShapeG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<cv::GShape>::remove(Primitive) generated
// ("std::vector<cv::GShape>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_GShapeG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<cv::GShape>::swap(Primitive, Primitive) generated
// ("std::vector<cv::GShape>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLcv_GShapeG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<cv::GShape>::clear() generated
// ("std::vector<cv::GShape>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_GShapeG_clear(instance: *mut c_void);
// std::vector<cv::GShape>::push(Enum) generated
// ("std::vector<cv::GShape>::push", vec![(pred!(mut, ["val"], ["const cv::GShape"]), _)]),
pub fn std_vectorLcv_GShapeG_push_const_GShape(instance: *mut c_void, val: crate::gapi::GShape);
// std::vector<cv::GShape>::insert(Primitive, Enum) generated
// ("std::vector<cv::GShape>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::GShape"]), _)]),
pub fn std_vectorLcv_GShapeG_insert_size_t_const_GShape(instance: *mut c_void, index: size_t, val: crate::gapi::GShape);
// std::vector<cv::GShape>::get(Primitive) generated
// ("std::vector<cv::GShape>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_GShapeG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut crate::gapi::GShape);
// std::vector<cv::GShape>::set(Primitive, Enum) generated
// ("std::vector<cv::GShape>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::GShape"]), _)]),
pub fn std_vectorLcv_GShapeG_set_size_t_const_GShape(instance: *mut c_void, index: size_t, val: crate::gapi::GShape);
// std::vector<cv::GShape>::clone() generated
// ("std::vector<cv::GShape>::clone", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_GShapeG_clone_const(instance: *const c_void) -> *mut c_void;
// std::vector<cv::GShape>::data() generated
// ("std::vector<cv::GShape>::data", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_GShapeG_data_const(instance: *const c_void) -> *const crate::gapi::GShape;
// std::vector<cv::GShape>::dataMut() generated
// ("std::vector<cv::GShape>::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_GShapeG_dataMut(instance: *mut c_void) -> *mut crate::gapi::GShape;
// cv::fromSlice(Enum, Primitive) generated
// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::GShape*", "size_t"]), _)]),
pub fn cv_fromSlice_const_const_GShapeX_size_t(data: *const crate::gapi::GShape, len: size_t) -> *mut c_void;
