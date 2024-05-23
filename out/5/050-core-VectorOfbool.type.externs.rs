// std::vector<bool>::new() generated
// ("std::vector<bool>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLboolG_new_const() -> *mut c_void;
// std::vector<bool>::delete() generated
// ("std::vector<bool>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLboolG_delete(instance: *mut c_void);
// std::vector<bool>::len() generated
// ("std::vector<bool>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLboolG_len_const(instance: *const c_void) -> size_t;
// std::vector<bool>::isEmpty() generated
// ("std::vector<bool>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLboolG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<bool>::capacity() generated
// ("std::vector<bool>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLboolG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<bool>::shrinkToFit() generated
// ("std::vector<bool>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLboolG_shrinkToFit(instance: *mut c_void);
// std::vector<bool>::reserve(Primitive) generated
// ("std::vector<bool>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLboolG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<bool>::remove(Primitive) generated
// ("std::vector<bool>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLboolG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<bool>::swap(Primitive, Primitive) generated
// ("std::vector<bool>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLboolG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<bool>::clear() generated
// ("std::vector<bool>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLboolG_clear(instance: *mut c_void);
// std::vector<bool>::push(Primitive) generated
// ("std::vector<bool>::push", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn std_vectorLboolG_push_const_bool(instance: *mut c_void, val: bool);
// std::vector<bool>::insert(Primitive, Primitive) generated
// ("std::vector<bool>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const bool"]), _)]),
pub fn std_vectorLboolG_insert_size_t_const_bool(instance: *mut c_void, index: size_t, val: bool);
// std::vector<bool>::get(Primitive) generated
// ("std::vector<bool>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLboolG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut bool);
// std::vector<bool>::set(Primitive, Primitive) generated
// ("std::vector<bool>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const bool"]), _)]),
pub fn std_vectorLboolG_set_size_t_const_bool(instance: *mut c_void, index: size_t, val: bool);
// std::vector<bool>::inputArray() generated
// ("std::vector<bool>::inputArray", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLboolG_inputArray_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
