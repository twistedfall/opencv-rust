// std::vector<double>::new() generated
// ("std::vector<double>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLdoubleG_new_const() -> *mut c_void;
// std::vector<double>::delete() generated
// ("std::vector<double>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLdoubleG_delete(instance: *mut c_void);
// std::vector<double>::len() generated
// ("std::vector<double>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLdoubleG_len_const(instance: *const c_void) -> size_t;
// std::vector<double>::isEmpty() generated
// ("std::vector<double>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLdoubleG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<double>::capacity() generated
// ("std::vector<double>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLdoubleG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<double>::shrinkToFit() generated
// ("std::vector<double>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLdoubleG_shrinkToFit(instance: *mut c_void);
// std::vector<double>::reserve(Primitive) generated
// ("std::vector<double>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLdoubleG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<double>::remove(Primitive) generated
// ("std::vector<double>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLdoubleG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<double>::swap(Primitive, Primitive) generated
// ("std::vector<double>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLdoubleG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<double>::clear() generated
// ("std::vector<double>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLdoubleG_clear(instance: *mut c_void);
// std::vector<double>::push(Primitive) generated
// ("std::vector<double>::push", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn std_vectorLdoubleG_push_const_double(instance: *mut c_void, val: f64);
// std::vector<double>::insert(Primitive, Primitive) generated
// ("std::vector<double>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const double"]), _)]),
pub fn std_vectorLdoubleG_insert_size_t_const_double(instance: *mut c_void, index: size_t, val: f64);
// std::vector<double>::get(Primitive) generated
// ("std::vector<double>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLdoubleG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut f64);
// std::vector<double>::set(Primitive, Primitive) generated
// ("std::vector<double>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const double"]), _)]),
pub fn std_vectorLdoubleG_set_size_t_const_double(instance: *mut c_void, index: size_t, val: f64);
// std::vector<double>::clone() generated
// ("std::vector<double>::clone", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLdoubleG_clone_const(instance: *const c_void) -> *mut c_void;
// std::vector<double>::data() generated
// ("std::vector<double>::data", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLdoubleG_data_const(instance: *const c_void) -> *const f64;
// std::vector<double>::dataMut() generated
// ("std::vector<double>::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLdoubleG_dataMut(instance: *mut c_void) -> *mut f64;
// cv::fromSlice(Indirect, Primitive) generated
// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const double*", "size_t"]), _)]),
pub fn cv_fromSlice_const_const_doubleX_size_t(data: *const f64, len: size_t) -> *mut c_void;
// std::vector<double>::inputArray() generated
// ("std::vector<double>::inputArray", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLdoubleG_inputArray_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<double>::outputArray() generated
// ("std::vector<double>::outputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLdoubleG_outputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<double>::inputOutputArray() generated
// ("std::vector<double>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLdoubleG_inputOutputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
