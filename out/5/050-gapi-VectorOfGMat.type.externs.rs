// std::vector<cv::GMat>::new() generated
// ("std::vector<cv::GMat>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_GMatG_new_const() -> *mut c_void;
// std::vector<cv::GMat>::delete() generated
// ("std::vector<cv::GMat>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_GMatG_delete(instance: *mut c_void);
// std::vector<cv::GMat>::len() generated
// ("std::vector<cv::GMat>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_GMatG_len_const(instance: *const c_void) -> size_t;
// std::vector<cv::GMat>::isEmpty() generated
// ("std::vector<cv::GMat>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_GMatG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<cv::GMat>::capacity() generated
// ("std::vector<cv::GMat>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLcv_GMatG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<cv::GMat>::shrinkToFit() generated
// ("std::vector<cv::GMat>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_GMatG_shrinkToFit(instance: *mut c_void);
// std::vector<cv::GMat>::reserve(Primitive) generated
// ("std::vector<cv::GMat>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLcv_GMatG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<cv::GMat>::remove(Primitive) generated
// ("std::vector<cv::GMat>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_GMatG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<cv::GMat>::swap(Primitive, Primitive) generated
// ("std::vector<cv::GMat>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLcv_GMatG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<cv::GMat>::clear() generated
// ("std::vector<cv::GMat>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLcv_GMatG_clear(instance: *mut c_void);
// std::vector<cv::GMat>::push(TraitClass) generated
// ("std::vector<cv::GMat>::push", vec![(pred!(mut, ["val"], ["const cv::GMat"]), _)]),
pub fn std_vectorLcv_GMatG_push_const_GMat(instance: *mut c_void, val: *const c_void);
// std::vector<cv::GMat>::insert(Primitive, TraitClass) generated
// ("std::vector<cv::GMat>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::GMat"]), _)]),
pub fn std_vectorLcv_GMatG_insert_size_t_const_GMat(instance: *mut c_void, index: size_t, val: *const c_void);
// std::vector<cv::GMat>::get(Primitive) generated
// ("std::vector<cv::GMat>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLcv_GMatG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut *mut c_void);
// std::vector<cv::GMat>::set(Primitive, TraitClass) generated
// ("std::vector<cv::GMat>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const cv::GMat"]), _)]),
pub fn std_vectorLcv_GMatG_set_size_t_const_GMat(instance: *mut c_void, index: size_t, val: *const c_void);
