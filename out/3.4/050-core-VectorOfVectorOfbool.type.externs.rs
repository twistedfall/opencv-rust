// std::vector<std::vector<bool>>::new() generated
// ("std::vector<std::vector<bool>>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLstd_vectorLboolGG_new_const() -> *mut c_void;
// std::vector<std::vector<bool>>::delete() generated
// ("std::vector<std::vector<bool>>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLstd_vectorLboolGG_delete(instance: *mut c_void);
// std::vector<std::vector<bool>>::len() generated
// ("std::vector<std::vector<bool>>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLstd_vectorLboolGG_len_const(instance: *const c_void) -> size_t;
// std::vector<std::vector<bool>>::isEmpty() generated
// ("std::vector<std::vector<bool>>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLstd_vectorLboolGG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<std::vector<bool>>::capacity() generated
// ("std::vector<std::vector<bool>>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLstd_vectorLboolGG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<std::vector<bool>>::shrinkToFit() generated
// ("std::vector<std::vector<bool>>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLstd_vectorLboolGG_shrinkToFit(instance: *mut c_void);
// std::vector<std::vector<bool>>::reserve(Primitive) generated
// ("std::vector<std::vector<bool>>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLstd_vectorLboolGG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<std::vector<bool>>::remove(Primitive) generated
// ("std::vector<std::vector<bool>>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLstd_vectorLboolGG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<std::vector<bool>>::swap(Primitive, Primitive) generated
// ("std::vector<std::vector<bool>>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLstd_vectorLboolGG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<std::vector<bool>>::clear() generated
// ("std::vector<std::vector<bool>>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLstd_vectorLboolGG_clear(instance: *mut c_void);
// std::vector<std::vector<bool>>::push(CppPassByVoidPtr) generated
// ("std::vector<std::vector<bool>>::push", vec![(pred!(mut, ["val"], ["const std::vector<bool>"]), _)]),
pub fn std_vectorLstd_vectorLboolGG_push_const_vectorLboolG(instance: *mut c_void, val: *const c_void);
// std::vector<std::vector<bool>>::insert(Primitive, CppPassByVoidPtr) generated
// ("std::vector<std::vector<bool>>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<bool>"]), _)]),
pub fn std_vectorLstd_vectorLboolGG_insert_size_t_const_vectorLboolG(instance: *mut c_void, index: size_t, val: *const c_void);
// std::vector<std::vector<bool>>::get(Primitive) generated
// ("std::vector<std::vector<bool>>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLstd_vectorLboolGG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut *mut c_void);
// std::vector<std::vector<bool>>::set(Primitive, CppPassByVoidPtr) generated
// ("std::vector<std::vector<bool>>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const std::vector<bool>"]), _)]),
pub fn std_vectorLstd_vectorLboolGG_set_size_t_const_vectorLboolG(instance: *mut c_void, index: size_t, val: *const c_void);
#[cfg(ocvrs_opencv_branch_5)]
// std::vector<std::vector<bool>>::inputArray() generated
// ("std::vector<std::vector<bool>>::inputArray", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLstd_vectorLboolGG_inputArray_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
#[cfg(ocvrs_opencv_branch_5)]
// std::vector<std::vector<bool>>::outputArray() generated
// ("std::vector<std::vector<bool>>::outputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLstd_vectorLboolGG_outputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
#[cfg(ocvrs_opencv_branch_5)]
// std::vector<std::vector<bool>>::inputOutputArray() generated
// ("std::vector<std::vector<bool>>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLstd_vectorLboolGG_inputOutputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
