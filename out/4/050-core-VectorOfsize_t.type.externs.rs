// std::vector<size_t>::new() generated
// ("std::vector<size_t>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLsize_tG_new_const() -> *mut c_void;
// std::vector<size_t>::delete() generated
// ("std::vector<size_t>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLsize_tG_delete(instance: *mut c_void);
// std::vector<size_t>::len() generated
// ("std::vector<size_t>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLsize_tG_len_const(instance: *const c_void) -> size_t;
// std::vector<size_t>::isEmpty() generated
// ("std::vector<size_t>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLsize_tG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<size_t>::capacity() generated
// ("std::vector<size_t>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLsize_tG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<size_t>::shrinkToFit() generated
// ("std::vector<size_t>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLsize_tG_shrinkToFit(instance: *mut c_void);
// std::vector<size_t>::reserve(Primitive) generated
// ("std::vector<size_t>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLsize_tG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<size_t>::remove(Primitive) generated
// ("std::vector<size_t>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLsize_tG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<size_t>::swap(Primitive, Primitive) generated
// ("std::vector<size_t>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLsize_tG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<size_t>::clear() generated
// ("std::vector<size_t>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLsize_tG_clear(instance: *mut c_void);
// std::vector<size_t>::push(Primitive) generated
// ("std::vector<size_t>::push", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
pub fn std_vectorLsize_tG_push_const_size_t(instance: *mut c_void, val: size_t);
// std::vector<size_t>::insert(Primitive, Primitive) generated
// ("std::vector<size_t>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const size_t"]), _)]),
pub fn std_vectorLsize_tG_insert_size_t_const_size_t(instance: *mut c_void, index: size_t, val: size_t);
// std::vector<size_t>::get(Primitive) generated
// ("std::vector<size_t>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLsize_tG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut size_t);
// std::vector<size_t>::set(Primitive, Primitive) generated
// ("std::vector<size_t>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const size_t"]), _)]),
pub fn std_vectorLsize_tG_set_size_t_const_size_t(instance: *mut c_void, index: size_t, val: size_t);
// std::vector<size_t>::clone() generated
// ("std::vector<size_t>::clone", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLsize_tG_clone_const(instance: *const c_void) -> *mut c_void;
// std::vector<size_t>::data() generated
// ("std::vector<size_t>::data", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLsize_tG_data_const(instance: *const c_void) -> *const size_t;
// std::vector<size_t>::dataMut() generated
// ("std::vector<size_t>::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLsize_tG_dataMut(instance: *mut c_void) -> *mut size_t;
// cv::fromSlice(Indirect, Primitive) generated
// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const size_t*", "size_t"]), _)]),
pub fn cv_fromSlice_const_const_size_tX_size_t(data: *const size_t, len: size_t) -> *mut c_void;
