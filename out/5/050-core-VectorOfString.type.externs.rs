// std::vector<cv::String>::new() generated
// ("std::vector<cv::String>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_StringG_new_const() -> *mut c_void;
// std::vector<cv::String>::delete() generated
// ("std::vector<cv::String>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_StringG_delete(instance: *mut c_void);
// std::vector<cv::String>::len() generated
// ("std::vector<cv::String>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_StringG_len_const(instance: *const c_void) -> size_t;
// std::vector<cv::String>::isEmpty() generated
// ("std::vector<cv::String>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_StringG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<cv::String>::capacity() generated
// ("std::vector<cv::String>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_StringG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<cv::String>::shrinkToFit() generated
// ("std::vector<cv::String>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_StringG_shrinkToFit(instance: *mut c_void);
// std::vector<cv::String>::reserve(Primitive) generated
// ("std::vector<cv::String>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLcv_StringG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<cv::String>::remove(Primitive) generated
// ("std::vector<cv::String>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_StringG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<cv::String>::swap(Primitive, Primitive) generated
// ("std::vector<cv::String>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLcv_StringG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<cv::String>::clear() generated
// ("std::vector<cv::String>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_StringG_clear(instance: *mut c_void);
// std::vector<cv::String>::push(InString) generated
// ("std::vector<cv::String>::push", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
pub fn std_vectorLcv_StringG_push_const_String(instance: *mut c_void, val: *const c_char);
// std::vector<cv::String>::insert(Primitive, InString) generated
// ("std::vector<cv::String>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::String"]), _)]),
pub fn std_vectorLcv_StringG_insert_size_t_const_String(instance: *mut c_void, index: size_t, val: *const c_char);
// std::vector<cv::String>::get(Primitive) generated
// ("std::vector<cv::String>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_StringG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut *mut c_void);
// std::vector<cv::String>::set(Primitive, InString) generated
// ("std::vector<cv::String>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::String"]), _)]),
pub fn std_vectorLcv_StringG_set_size_t_const_String(instance: *mut c_void, index: size_t, val: *const c_char);
