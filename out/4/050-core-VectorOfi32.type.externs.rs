// std::vector<int>::new() generated
// ("std::vector<int>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLintG_new_const() -> *mut c_void;
// std::vector<int>::delete() generated
// ("std::vector<int>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLintG_delete(instance: *mut c_void);
// std::vector<int>::len() generated
// ("std::vector<int>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLintG_len_const(instance: *const c_void) -> size_t;
// std::vector<int>::isEmpty() generated
// ("std::vector<int>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLintG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<int>::capacity() generated
// ("std::vector<int>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLintG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<int>::shrinkToFit() generated
// ("std::vector<int>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLintG_shrinkToFit(instance: *mut c_void);
// std::vector<int>::reserve(Primitive) generated
// ("std::vector<int>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLintG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<int>::remove(Primitive) generated
// ("std::vector<int>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLintG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<int>::swap(Primitive, Primitive) generated
// ("std::vector<int>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLintG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<int>::clear() generated
// ("std::vector<int>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLintG_clear(instance: *mut c_void);
// std::vector<int>::push(Primitive) generated
// ("std::vector<int>::push", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn std_vectorLintG_push_const_int(instance: *mut c_void, val: i32);
// std::vector<int>::insert(Primitive, Primitive) generated
// ("std::vector<int>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const int"]), _)]),
pub fn std_vectorLintG_insert_size_t_const_int(instance: *mut c_void, index: size_t, val: i32);
// std::vector<int>::get(Primitive) generated
// ("std::vector<int>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLintG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut i32);
// std::vector<int>::set(Primitive, Primitive) generated
// ("std::vector<int>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const int"]), _)]),
pub fn std_vectorLintG_set_size_t_const_int(instance: *mut c_void, index: size_t, val: i32);
// std::vector<int>::clone() generated
// ("std::vector<int>::clone", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLintG_clone_const(instance: *const c_void) -> *mut c_void;
// std::vector<int>::data() generated
// ("std::vector<int>::data", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLintG_data_const(instance: *const c_void) -> *const i32;
// std::vector<int>::dataMut() generated
// ("std::vector<int>::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLintG_dataMut(instance: *mut c_void) -> *mut i32;
// cv::fromSlice(Indirect, Primitive) generated
// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const int*", "size_t"]), _)]),
pub fn cv_fromSlice_const_const_intX_size_t(data: *const i32, len: size_t) -> *mut c_void;
// std::vector<int>::inputArray() generated
// ("std::vector<int>::inputArray", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLintG_inputArray_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<int>::outputArray() generated
// ("std::vector<int>::outputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLintG_outputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<int>::inputOutputArray() generated
// ("std::vector<int>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLintG_inputOutputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
