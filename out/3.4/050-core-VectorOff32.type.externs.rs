// std::vector<float>::new() generated
// ("std::vector<float>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLfloatG_new_const() -> *mut c_void;
// std::vector<float>::delete() generated
// ("std::vector<float>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLfloatG_delete(instance: *mut c_void);
// std::vector<float>::len() generated
// ("std::vector<float>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLfloatG_len_const(instance: *const c_void) -> size_t;
// std::vector<float>::isEmpty() generated
// ("std::vector<float>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLfloatG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<float>::capacity() generated
// ("std::vector<float>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLfloatG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<float>::shrinkToFit() generated
// ("std::vector<float>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLfloatG_shrinkToFit(instance: *mut c_void);
// std::vector<float>::reserve(Primitive) generated
// ("std::vector<float>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLfloatG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<float>::remove(Primitive) generated
// ("std::vector<float>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLfloatG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<float>::swap(Primitive, Primitive) generated
// ("std::vector<float>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLfloatG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<float>::clear() generated
// ("std::vector<float>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLfloatG_clear(instance: *mut c_void);
// std::vector<float>::push(Primitive) generated
// ("std::vector<float>::push", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn std_vectorLfloatG_push_const_float(instance: *mut c_void, val: f32);
// std::vector<float>::insert(Primitive, Primitive) generated
// ("std::vector<float>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const float"]), _)]),
pub fn std_vectorLfloatG_insert_size_t_const_float(instance: *mut c_void, index: size_t, val: f32);
// std::vector<float>::get(Primitive) generated
// ("std::vector<float>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLfloatG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut f32);
// std::vector<float>::set(Primitive, Primitive) generated
// ("std::vector<float>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const float"]), _)]),
pub fn std_vectorLfloatG_set_size_t_const_float(instance: *mut c_void, index: size_t, val: f32);
// std::vector<float>::clone() generated
// ("std::vector<float>::clone", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLfloatG_clone_const(instance: *const c_void) -> *mut c_void;
// std::vector<float>::data() generated
// ("std::vector<float>::data", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLfloatG_data_const(instance: *const c_void) -> *const f32;
// std::vector<float>::dataMut() generated
// ("std::vector<float>::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLfloatG_dataMut(instance: *mut c_void) -> *mut f32;
// cv::fromSlice(Indirect, Primitive) generated
// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const float*", "size_t"]), _)]),
pub fn cv_fromSlice_const_const_floatX_size_t(data: *const f32, len: size_t) -> *mut c_void;
// std::vector<float>::inputArray() generated
// ("std::vector<float>::inputArray", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLfloatG_inputArray_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<float>::outputArray() generated
// ("std::vector<float>::outputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLfloatG_outputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<float>::inputOutputArray() generated
// ("std::vector<float>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLfloatG_inputOutputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
