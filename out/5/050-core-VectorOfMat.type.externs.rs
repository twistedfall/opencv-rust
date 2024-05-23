// std::vector<cv::Mat>::new() generated
// ("std::vector<cv::Mat>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_MatG_new_const() -> *mut c_void;
// std::vector<cv::Mat>::delete() generated
// ("std::vector<cv::Mat>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_MatG_delete(instance: *mut c_void);
// std::vector<cv::Mat>::len() generated
// ("std::vector<cv::Mat>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_MatG_len_const(instance: *const c_void) -> size_t;
// std::vector<cv::Mat>::isEmpty() generated
// ("std::vector<cv::Mat>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_MatG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<cv::Mat>::capacity() generated
// ("std::vector<cv::Mat>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_MatG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<cv::Mat>::shrinkToFit() generated
// ("std::vector<cv::Mat>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_MatG_shrinkToFit(instance: *mut c_void);
// std::vector<cv::Mat>::reserve(Primitive) generated
// ("std::vector<cv::Mat>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLcv_MatG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<cv::Mat>::remove(Primitive) generated
// ("std::vector<cv::Mat>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_MatG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<cv::Mat>::swap(Primitive, Primitive) generated
// ("std::vector<cv::Mat>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLcv_MatG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<cv::Mat>::clear() generated
// ("std::vector<cv::Mat>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_MatG_clear(instance: *mut c_void);
// std::vector<cv::Mat>::push(TraitClass) generated
// ("std::vector<cv::Mat>::push", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn std_vectorLcv_MatG_push_const_Mat(instance: *mut c_void, val: *const c_void);
// std::vector<cv::Mat>::insert(Primitive, TraitClass) generated
// ("std::vector<cv::Mat>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Mat"]), _)]),
pub fn std_vectorLcv_MatG_insert_size_t_const_Mat(instance: *mut c_void, index: size_t, val: *const c_void);
// std::vector<cv::Mat>::get(Primitive) generated
// ("std::vector<cv::Mat>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_MatG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut *mut c_void);
// std::vector<cv::Mat>::set(Primitive, TraitClass) generated
// ("std::vector<cv::Mat>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::Mat"]), _)]),
pub fn std_vectorLcv_MatG_set_size_t_const_Mat(instance: *mut c_void, index: size_t, val: *const c_void);
