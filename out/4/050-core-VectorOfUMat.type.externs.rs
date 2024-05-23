// std::vector<cv::UMat>::new() generated
// ("std::vector<cv::UMat>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_UMatG_new_const() -> *mut c_void;
// std::vector<cv::UMat>::delete() generated
// ("std::vector<cv::UMat>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_UMatG_delete(instance: *mut c_void);
// std::vector<cv::UMat>::len() generated
// ("std::vector<cv::UMat>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_UMatG_len_const(instance: *const c_void) -> size_t;
// std::vector<cv::UMat>::isEmpty() generated
// ("std::vector<cv::UMat>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_UMatG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<cv::UMat>::capacity() generated
// ("std::vector<cv::UMat>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_UMatG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<cv::UMat>::shrinkToFit() generated
// ("std::vector<cv::UMat>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_UMatG_shrinkToFit(instance: *mut c_void);
// std::vector<cv::UMat>::reserve(Primitive) generated
// ("std::vector<cv::UMat>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLcv_UMatG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<cv::UMat>::remove(Primitive) generated
// ("std::vector<cv::UMat>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_UMatG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<cv::UMat>::swap(Primitive, Primitive) generated
// ("std::vector<cv::UMat>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLcv_UMatG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<cv::UMat>::clear() generated
// ("std::vector<cv::UMat>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_UMatG_clear(instance: *mut c_void);
// std::vector<cv::UMat>::push(TraitClass) generated
// ("std::vector<cv::UMat>::push", vec![(pred!(mut, ["val"], ["const cv::UMat"]), _)]),
pub fn std_vectorLcv_UMatG_push_const_UMat(instance: *mut c_void, val: *const c_void);
// std::vector<cv::UMat>::insert(Primitive, TraitClass) generated
// ("std::vector<cv::UMat>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::UMat"]), _)]),
pub fn std_vectorLcv_UMatG_insert_size_t_const_UMat(instance: *mut c_void, index: size_t, val: *const c_void);
// std::vector<cv::UMat>::get(Primitive) generated
// ("std::vector<cv::UMat>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_UMatG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut *mut c_void);
// std::vector<cv::UMat>::set(Primitive, TraitClass) generated
// ("std::vector<cv::UMat>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::UMat"]), _)]),
pub fn std_vectorLcv_UMatG_set_size_t_const_UMat(instance: *mut c_void, index: size_t, val: *const c_void);
