// std::vector<cv::DMatch>::new() generated
// ("std::vector<cv::DMatch>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_DMatchG_new_const() -> *mut c_void;
// std::vector<cv::DMatch>::delete() generated
// ("std::vector<cv::DMatch>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_DMatchG_delete(instance: *mut c_void);
// std::vector<cv::DMatch>::len() generated
// ("std::vector<cv::DMatch>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_DMatchG_len_const(instance: *const c_void) -> size_t;
// std::vector<cv::DMatch>::isEmpty() generated
// ("std::vector<cv::DMatch>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_DMatchG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<cv::DMatch>::capacity() generated
// ("std::vector<cv::DMatch>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_DMatchG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<cv::DMatch>::shrinkToFit() generated
// ("std::vector<cv::DMatch>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_DMatchG_shrinkToFit(instance: *mut c_void);
// std::vector<cv::DMatch>::reserve(Primitive) generated
// ("std::vector<cv::DMatch>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLcv_DMatchG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<cv::DMatch>::remove(Primitive) generated
// ("std::vector<cv::DMatch>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_DMatchG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<cv::DMatch>::swap(Primitive, Primitive) generated
// ("std::vector<cv::DMatch>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLcv_DMatchG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<cv::DMatch>::clear() generated
// ("std::vector<cv::DMatch>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_DMatchG_clear(instance: *mut c_void);
// std::vector<cv::DMatch>::push(SimpleClass) generated
// ("std::vector<cv::DMatch>::push", vec![(pred!(mut, ["val"], ["const cv::DMatch"]), _)]),
pub fn std_vectorLcv_DMatchG_push_const_DMatch(instance: *mut c_void, val: *const core::DMatch);
// std::vector<cv::DMatch>::insert(Primitive, SimpleClass) generated
// ("std::vector<cv::DMatch>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::DMatch"]), _)]),
pub fn std_vectorLcv_DMatchG_insert_size_t_const_DMatch(instance: *mut c_void, index: size_t, val: *const core::DMatch);
// std::vector<cv::DMatch>::get(Primitive) generated
// ("std::vector<cv::DMatch>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_DMatchG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut core::DMatch);
// std::vector<cv::DMatch>::set(Primitive, SimpleClass) generated
// ("std::vector<cv::DMatch>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::DMatch"]), _)]),
pub fn std_vectorLcv_DMatchG_set_size_t_const_DMatch(instance: *mut c_void, index: size_t, val: *const core::DMatch);
// std::vector<cv::DMatch>::clone() generated
// ("std::vector<cv::DMatch>::clone", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_DMatchG_clone_const(instance: *const c_void) -> *mut c_void;
// std::vector<cv::DMatch>::data() generated
// ("std::vector<cv::DMatch>::data", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_DMatchG_data_const(instance: *const c_void) -> *const core::DMatch;
// std::vector<cv::DMatch>::dataMut() generated
// ("std::vector<cv::DMatch>::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_DMatchG_dataMut(instance: *mut c_void) -> *mut core::DMatch;
// cv::fromSlice(SimpleClass, Primitive) generated
// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const cv::DMatch*", "size_t"]), _)]),
pub fn cv_fromSlice_const_const_DMatchX_size_t(data: *const core::DMatch, len: size_t) -> *mut c_void;
