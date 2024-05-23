// std::vector<unsigned char>::new() generated
// ("std::vector<unsigned char>::new", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLunsigned_charG_new_const() -> *mut c_void;
// std::vector<unsigned char>::delete() generated
// ("std::vector<unsigned char>::delete", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLunsigned_charG_delete(instance: *mut c_void);
// std::vector<unsigned char>::len() generated
// ("std::vector<unsigned char>::len", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLunsigned_charG_len_const(instance: *const c_void) -> size_t;
// std::vector<unsigned char>::isEmpty() generated
// ("std::vector<unsigned char>::isEmpty", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLunsigned_charG_isEmpty_const(instance: *const c_void) -> bool;
// std::vector<unsigned char>::capacity() generated
// ("std::vector<unsigned char>::capacity", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLunsigned_charG_capacity_const(instance: *const c_void) -> size_t;
// std::vector<unsigned char>::shrinkToFit() generated
// ("std::vector<unsigned char>::shrinkToFit", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLunsigned_charG_shrinkToFit(instance: *mut c_void);
// std::vector<unsigned char>::reserve(Primitive) generated
// ("std::vector<unsigned char>::reserve", vec![(pred!(mut, ["additional"], ["size_t"]), _)]),
pub fn std_vectorLunsigned_charG_reserve_size_t(instance: *mut c_void, additional: size_t);
// std::vector<unsigned char>::remove(Primitive) generated
// ("std::vector<unsigned char>::remove", vec![(pred!(mut, ["index"], ["size_t"]), _)]),
pub fn std_vectorLunsigned_charG_remove_size_t(instance: *mut c_void, index: size_t);
// std::vector<unsigned char>::swap(Primitive, Primitive) generated
// ("std::vector<unsigned char>::swap", vec![(pred!(mut, ["index1", "index2"], ["size_t", "size_t"]), _)]),
pub fn std_vectorLunsigned_charG_swap_size_t_size_t(instance: *mut c_void, index1: size_t, index2: size_t);
// std::vector<unsigned char>::clear() generated
// ("std::vector<unsigned char>::clear", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLunsigned_charG_clear(instance: *mut c_void);
// std::vector<unsigned char>::push(Primitive) generated
// ("std::vector<unsigned char>::push", vec![(pred!(mut, ["val"], ["const unsigned char"]), _)]),
pub fn std_vectorLunsigned_charG_push_const_unsigned_char(instance: *mut c_void, val: u8);
// std::vector<unsigned char>::insert(Primitive, Primitive) generated
// ("std::vector<unsigned char>::insert", vec![(pred!(mut, ["index", "val"], ["size_t", "const unsigned char"]), _)]),
pub fn std_vectorLunsigned_charG_insert_size_t_const_unsigned_char(instance: *mut c_void, index: size_t, val: u8);
// std::vector<unsigned char>::get(Primitive) generated
// ("std::vector<unsigned char>::get", vec![(pred!(const, ["index"], ["size_t"]), _)]),
pub fn std_vectorLunsigned_charG_get_const_size_t(instance: *const c_void, index: size_t, ocvrs_return: *mut u8);
// std::vector<unsigned char>::set(Primitive, Primitive) generated
// ("std::vector<unsigned char>::set", vec![(pred!(mut, ["index", "val"], ["size_t", "const unsigned char"]), _)]),
pub fn std_vectorLunsigned_charG_set_size_t_const_unsigned_char(instance: *mut c_void, index: size_t, val: u8);
// std::vector<unsigned char>::clone() generated
// ("std::vector<unsigned char>::clone", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLunsigned_charG_clone_const(instance: *const c_void) -> *mut c_void;
// std::vector<unsigned char>::data() generated
// ("std::vector<unsigned char>::data", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLunsigned_charG_data_const(instance: *const c_void) -> *const u8;
// std::vector<unsigned char>::dataMut() generated
// ("std::vector<unsigned char>::dataMut", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLunsigned_charG_dataMut(instance: *mut c_void) -> *mut u8;
// cv::fromSlice(Indirect, Primitive) generated
// ("cv::fromSlice", vec![(pred!(const, ["data", "len"], ["const unsigned char*", "size_t"]), _)]),
pub fn cv_fromSlice_const_const_unsigned_charX_size_t(data: *const u8, len: size_t) -> *mut c_void;
// std::vector<unsigned char>::inputArray() generated
// ("std::vector<unsigned char>::inputArray", vec![(pred!(const, [], []), _)]),
pub fn std_vectorLunsigned_charG_inputArray_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<unsigned char>::outputArray() generated
// ("std::vector<unsigned char>::outputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLunsigned_charG_outputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// std::vector<unsigned char>::inputOutputArray() generated
// ("std::vector<unsigned char>::inputOutputArray", vec![(pred!(mut, [], []), _)]),
pub fn std_vectorLunsigned_charG_inputOutputArray(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
