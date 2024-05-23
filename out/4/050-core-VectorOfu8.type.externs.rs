// std::vector<uint8_t>::new() generated
// ("std::vector<uint8_t>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLuint8_tG_new_const() -> *mut c_void;
// std::vector<uint8_t>::delete() generated
// ("std::vector<uint8_t>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLuint8_tG_delete(instance: *mut c_void);
// std::vector<uint8_t>::len() generated
// ("std::vector<uint8_t>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLuint8_tG_len_const(instance: *const c_void) -> size_t;
// std::vector<uint8_t>::isEmpty() generated
// ("std::vector<uint8_t>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLuint8_tG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<uint8_t>::capacity() generated
// ("std::vector<uint8_t>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLuint8_tG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<uint8_t>::shrinkToFit() generated
// ("std::vector<uint8_t>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLuint8_tG_shrinkToFit(instance: *mut c_void);
// std::vector<uint8_t>::reserve(Primitive) generated
// ("std::vector<uint8_t>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLuint8_tG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<uint8_t>::remove(Primitive) generated
// ("std::vector<uint8_t>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLuint8_tG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<uint8_t>::swap(Primitive, Primitive) generated
// ("std::vector<uint8_t>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLuint8_tG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<uint8_t>::clear() generated
// ("std::vector<uint8_t>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLuint8_tG_clear(instance: *mut c_void);
// std::vector<uint8_t>::push(Primitive) generated
// ("std::vector<uint8_t>::push", vec![(pred!(mut, ["val"], ["const uint8_t"]), _)]),
pub fn std_vectorLuint8_tG_push_const_uint8_t(instance: *mut c_void, val: u8);
// std::vector<uint8_t>::insert(Primitive, Primitive) generated
// ("std::vector<uint8_t>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const uint8_t"]), _)]),
pub fn std_vectorLuint8_tG_insert_size_t_const_uint8_t(instance: *mut c_void, index: size_t, val: u8);
// std::vector<uint8_t>::get(Primitive) generated
// ("std::vector<uint8_t>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLuint8_tG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut u8);
// std::vector<uint8_t>::set(Primitive, Primitive) generated
// ("std::vector<uint8_t>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const uint8_t"]), _)]),
pub fn std_vectorLuint8_tG_set_size_t_const_uint8_t(instance: *mut c_void, index: size_t, val: u8);
// std::vector<uint8_t>::clone() generated
// ("std::vector<uint8_t>::clone", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLuint8_tG_clone_const(instance: *const c_void) -> *mut c_void;
// std::vector<uint8_t>::data() generated
// ("std::vector<uint8_t>::data", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLuint8_tG_data_const(instance: *const c_void) -> *const u8;
// std::vector<uint8_t>::dataMut() generated
// ("std::vector<uint8_t>::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLuint8_tG_dataMut(instance: *mut c_void) -> *mut u8;
// cv::fromSlice(Indirect, Primitive) generated
// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const uint8_t*", "size_t"]), _)]),
pub fn cv_fromSlice_const_const_uint8_tX_size_t(data: *const u8, len: size_t) -> *mut c_void;
// std::vector<uint8_t>::inputArray() generated
// ("std::vector<uint8_t>::inputArray", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLuint8_tG_inputArray_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<uint8_t>::outputArray() generated
// ("std::vector<uint8_t>::outputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLuint8_tG_outputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<uint8_t>::inputOutputArray() generated
// ("std::vector<uint8_t>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLuint8_tG_inputOutputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
