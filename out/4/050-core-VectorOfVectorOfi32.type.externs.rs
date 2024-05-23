// std::vector<std::vector<int>>::new() generated
// ("std::vector<std::vector<int>>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLstd_vectorLintGG_new_const() -> *mut c_void;
// std::vector<std::vector<int>>::delete() generated
// ("std::vector<std::vector<int>>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLstd_vectorLintGG_delete(instance: *mut c_void);
// std::vector<std::vector<int>>::len() generated
// ("std::vector<std::vector<int>>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLstd_vectorLintGG_len_const(instance: *const c_void) -> size_t;
// std::vector<std::vector<int>>::isEmpty() generated
// ("std::vector<std::vector<int>>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLstd_vectorLintGG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<std::vector<int>>::capacity() generated
// ("std::vector<std::vector<int>>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLstd_vectorLintGG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<std::vector<int>>::shrinkToFit() generated
// ("std::vector<std::vector<int>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLstd_vectorLintGG_shrinkToFit(instance: *mut c_void);
// std::vector<std::vector<int>>::reserve(Primitive) generated
// ("std::vector<std::vector<int>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLstd_vectorLintGG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<std::vector<int>>::remove(Primitive) generated
// ("std::vector<std::vector<int>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLstd_vectorLintGG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<std::vector<int>>::swap(Primitive, Primitive) generated
// ("std::vector<std::vector<int>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLstd_vectorLintGG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<std::vector<int>>::clear() generated
// ("std::vector<std::vector<int>>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLstd_vectorLintGG_clear(instance: *mut c_void);
// std::vector<std::vector<int>>::push(CppPassByVoidPtr) generated
// ("std::vector<std::vector<int>>::push", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
pub fn std_vectorLstd_vectorLintGG_push_const_vectorLintG(instance: *mut c_void, val: *const c_void);
// std::vector<std::vector<int>>::insert(Primitive, CppPassByVoidPtr) generated
// ("std::vector<std::vector<int>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<int>"]), _)]),
pub fn std_vectorLstd_vectorLintGG_insert_size_t_const_vectorLintG(instance: *mut c_void, index: size_t, val: *const c_void);
// std::vector<std::vector<int>>::get(Primitive) generated
// ("std::vector<std::vector<int>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLstd_vectorLintGG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut *mut c_void);
// std::vector<std::vector<int>>::set(Primitive, CppPassByVoidPtr) generated
// ("std::vector<std::vector<int>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<int>"]), _)]),
pub fn std_vectorLstd_vectorLintGG_set_size_t_const_vectorLintG(instance: *mut c_void, index: size_t, val: *const c_void);
// std::vector<std::vector<int>>::inputArray() generated
// ("std::vector<std::vector<int>>::inputArray", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLstd_vectorLintGG_inputArray_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<std::vector<int>>::outputArray() generated
// ("std::vector<std::vector<int>>::outputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLstd_vectorLintGG_outputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<std::vector<int>>::inputOutputArray() generated
// ("std::vector<std::vector<int>>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLstd_vectorLintGG_inputOutputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
