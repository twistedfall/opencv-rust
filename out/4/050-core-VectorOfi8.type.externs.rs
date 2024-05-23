// std::vector<signed char>::new() generated
// ("std::vector<signed char>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLsigned_charG_new_const() -> *mut c_void;
// std::vector<signed char>::delete() generated
// ("std::vector<signed char>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLsigned_charG_delete(instance: *mut c_void);
// std::vector<signed char>::len() generated
// ("std::vector<signed char>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLsigned_charG_len_const(instance: *const c_void) -> size_t;
// std::vector<signed char>::isEmpty() generated
// ("std::vector<signed char>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLsigned_charG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<signed char>::capacity() generated
// ("std::vector<signed char>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLsigned_charG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<signed char>::shrinkToFit() generated
// ("std::vector<signed char>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLsigned_charG_shrinkToFit(instance: *mut c_void);
// std::vector<signed char>::reserve(Primitive) generated
// ("std::vector<signed char>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLsigned_charG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<signed char>::remove(Primitive) generated
// ("std::vector<signed char>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLsigned_charG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<signed char>::swap(Primitive, Primitive) generated
// ("std::vector<signed char>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLsigned_charG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<signed char>::clear() generated
// ("std::vector<signed char>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLsigned_charG_clear(instance: *mut c_void);
// std::vector<signed char>::push(Primitive) generated
// ("std::vector<signed char>::push", vec![(pred!(mut, ["val"], ["const signed char"]), _)]),
pub fn std_vectorLsigned_charG_push_const_signed_char(instance: *mut c_void, val: i8);
// std::vector<signed char>::insert(Primitive, Primitive) generated
// ("std::vector<signed char>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const signed char"]), _)]),
pub fn std_vectorLsigned_charG_insert_size_t_const_signed_char(instance: *mut c_void, index: size_t, val: i8);
// std::vector<signed char>::get(Primitive) generated
// ("std::vector<signed char>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLsigned_charG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut i8);
// std::vector<signed char>::set(Primitive, Primitive) generated
// ("std::vector<signed char>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const signed char"]), _)]),
pub fn std_vectorLsigned_charG_set_size_t_const_signed_char(instance: *mut c_void, index: size_t, val: i8);
// std::vector<signed char>::clone() generated
// ("std::vector<signed char>::clone", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLsigned_charG_clone_const(instance: *const c_void) -> *mut c_void;
// std::vector<signed char>::data() generated
// ("std::vector<signed char>::data", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLsigned_charG_data_const(instance: *const c_void) -> *const i8;
// std::vector<signed char>::dataMut() generated
// ("std::vector<signed char>::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLsigned_charG_dataMut(instance: *mut c_void) -> *mut i8;
// cv::fromSlice(Indirect, Primitive) generated
// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const signed char*", "size_t"]), _)]),
pub fn cv_fromSlice_const_const_signed_charX_size_t(data: *const i8, len: size_t) -> *mut c_void;
