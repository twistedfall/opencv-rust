// open(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:802
// ("cv::hdf::open", vec![(pred!(mut, ["HDF5Filename"], ["const cv::String*"]), _)]),
pub fn cv_hdf_open_const_StringR(hdf5_filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// close()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:73
// ("cv::hdf::HDF5::close", vec![(pred!(mut, [], []), _)]),
pub fn cv_hdf_HDF5_close(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// grcreate(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:95
// ("cv::hdf::HDF5::grcreate", vec![(pred!(mut, ["grlabel"], ["const cv::String*"]), _)]),
pub fn cv_hdf_HDF5_grcreate_const_StringR(instance: *mut c_void, grlabel: *const c_char, ocvrs_return: *mut Result<()>);
// hlexists(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:104
// ("cv::hdf::HDF5::hlexists", vec![(pred!(const, ["label"], ["const cv::String*"]), _)]),
pub fn cv_hdf_HDF5_hlexists_const_const_StringR(instance: *const c_void, label: *const c_char, ocvrs_return: *mut Result<bool>);
// atexists(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:114
// ("cv::hdf::HDF5::atexists", vec![(pred!(const, ["atlabel"], ["const cv::String*"]), _)]),
pub fn cv_hdf_HDF5_atexists_const_const_StringR(instance: *const c_void, atlabel: *const c_char, ocvrs_return: *mut Result<bool>);
// atdelete(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:126
// ("cv::hdf::HDF5::atdelete", vec![(pred!(mut, ["atlabel"], ["const cv::String*"]), _)]),
pub fn cv_hdf_HDF5_atdelete_const_StringR(instance: *mut c_void, atlabel: *const c_char, ocvrs_return: *mut Result<()>);
// atwrite(const int, const String &)(Primitive, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:144
// ("cv::hdf::HDF5::atwrite", vec![(pred!(mut, ["value", "atlabel"], ["const int", "const cv::String*"]), _)]),
pub fn cv_hdf_HDF5_atwrite_const_int_const_StringR(instance: *mut c_void, value: i32, atlabel: *const c_char, ocvrs_return: *mut Result<()>);
// atread(int *, const String &)(Indirect, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:161
// ("cv::hdf::HDF5::atread", vec![(pred!(mut, ["value", "atlabel"], ["int*", "const cv::String*"]), _)]),
pub fn cv_hdf_HDF5_atread_intX_const_StringR(instance: *mut c_void, value: *mut i32, atlabel: *const c_char, ocvrs_return: *mut Result<()>);
// atwrite(const double, const String &)(Primitive, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:164
// ("cv::hdf::HDF5::atwrite", vec![(pred!(mut, ["value", "atlabel"], ["const double", "const cv::String*"]), _)]),
pub fn cv_hdf_HDF5_atwrite_const_double_const_StringR(instance: *mut c_void, value: f64, atlabel: *const c_char, ocvrs_return: *mut Result<()>);
// atread(double *, const String &)(Indirect, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:167
// ("cv::hdf::HDF5::atread", vec![(pred!(mut, ["value", "atlabel"], ["double*", "const cv::String*"]), _)]),
pub fn cv_hdf_HDF5_atread_doubleX_const_StringR(instance: *mut c_void, value: *mut f64, atlabel: *const c_char, ocvrs_return: *mut Result<()>);
// atwrite(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:170
// ("cv::hdf::HDF5::atwrite", vec![(pred!(mut, ["value", "atlabel"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_hdf_HDF5_atwrite_const_StringR_const_StringR(instance: *mut c_void, value: *const c_char, atlabel: *const c_char, ocvrs_return: *mut Result<()>);
// atread(String *, const String &)(OutString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:173
// ("cv::hdf::HDF5::atread", vec![(pred!(mut, ["value", "atlabel"], ["cv::String*", "const cv::String*"]), _)]),
pub fn cv_hdf_HDF5_atread_StringX_const_StringR(instance: *mut c_void, value: *mut *mut c_void, atlabel: *const c_char, ocvrs_return: *mut Result<()>);
// atwrite(InputArray, const String &)(InputArray, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:187
// ("cv::hdf::HDF5::atwrite", vec![(pred!(mut, ["value", "atlabel"], ["const cv::_InputArray*", "const cv::String*"]), _)]),
pub fn cv_hdf_HDF5_atwrite_const__InputArrayR_const_StringR(instance: *mut c_void, value: *const c_void, atlabel: *const c_char, ocvrs_return: *mut Result<()>);
// atread(OutputArray, const String &)(OutputArray, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:200
// ("cv::hdf::HDF5::atread", vec![(pred!(mut, ["value", "atlabel"], ["const cv::_OutputArray*", "const cv::String*"]), _)]),
pub fn cv_hdf_HDF5_atread_const__OutputArrayR_const_StringR(instance: *mut c_void, value: *const c_void, atlabel: *const c_char, ocvrs_return: *mut Result<()>);
// dscreate(const int, const int, const int, const String &)(Primitive, Primitive, Primitive, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:203
// ("cv::hdf::HDF5::dscreate", vec![(pred!(const, ["rows", "cols", "type", "dslabel"], ["const int", "const int", "const int", "const cv::String*"]), _)]),
pub fn cv_hdf_HDF5_dscreate_const_const_int_const_int_const_int_const_StringR(instance: *const c_void, rows: i32, cols: i32, typ: i32, dslabel: *const c_char, ocvrs_return: *mut Result<()>);
// dscreate(const int, const int, const int, const String &, const int)(Primitive, Primitive, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:206
// ("cv::hdf::HDF5::dscreate", vec![(pred!(const, ["rows", "cols", "type", "dslabel", "compresslevel"], ["const int", "const int", "const int", "const cv::String*", "const int"]), _)]),
pub fn cv_hdf_HDF5_dscreate_const_const_int_const_int_const_int_const_StringR_const_int(instance: *const c_void, rows: i32, cols: i32, typ: i32, dslabel: *const c_char, compresslevel: i32, ocvrs_return: *mut Result<()>);
// dscreate(const int, const int, const int, const String &, const int, const vector<int> &)(Primitive, Primitive, Primitive, InString, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:209
// ("cv::hdf::HDF5::dscreate", vec![(pred!(const, ["rows", "cols", "type", "dslabel", "compresslevel", "dims_chunks"], ["const int", "const int", "const int", "const cv::String*", "const int", "const std::vector<int>*"]), _)]),
pub fn cv_hdf_HDF5_dscreate_const_const_int_const_int_const_int_const_StringR_const_int_const_vectorLintGR(instance: *const c_void, rows: i32, cols: i32, typ: i32, dslabel: *const c_char, compresslevel: i32, dims_chunks: *const c_void, ocvrs_return: *mut Result<()>);
// dscreate(const int, const int *, const int, const String &)(Primitive, VariableArray, Primitive, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:282
// ("cv::hdf::HDF5::dscreate", vec![(pred!(const, ["n_dims", "sizes", "type", "dslabel"], ["const int", "const int*", "const int", "const cv::String*"]), _)]),
pub fn cv_hdf_HDF5_dscreate_const_const_int_const_intX_const_int_const_StringR(instance: *const c_void, n_dims: i32, sizes: *const i32, typ: i32, dslabel: *const c_char, ocvrs_return: *mut Result<()>);
// dscreate(const int, const int *, const int, const String &, const int)(Primitive, VariableArray, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:285
// ("cv::hdf::HDF5::dscreate", vec![(pred!(const, ["n_dims", "sizes", "type", "dslabel", "compresslevel"], ["const int", "const int*", "const int", "const cv::String*", "const int"]), _)]),
pub fn cv_hdf_HDF5_dscreate_const_const_int_const_intX_const_int_const_StringR_const_int(instance: *const c_void, n_dims: i32, sizes: *const i32, typ: i32, dslabel: *const c_char, compresslevel: i32, ocvrs_return: *mut Result<()>);
// dscreate(const vector<int> &, const int, const String &, const int, const vector<int> &)(CppPassByVoidPtr, Primitive, InString, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:288
// ("cv::hdf::HDF5::dscreate", vec![(pred!(const, ["sizes", "type", "dslabel", "compresslevel", "dims_chunks"], ["const std::vector<int>*", "const int", "const cv::String*", "const int", "const std::vector<int>*"]), _)]),
pub fn cv_hdf_HDF5_dscreate_const_const_vectorLintGR_const_int_const_StringR_const_int_const_vectorLintGR(instance: *const c_void, sizes: *const c_void, typ: i32, dslabel: *const c_char, compresslevel: i32, dims_chunks: *const c_void, ocvrs_return: *mut Result<()>);
// cv::hdf::HDF5::dscreate(CppPassByVoidPtr, Primitive, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:288
// ("cv::hdf::HDF5::dscreate", vec![(pred!(const, ["sizes", "type", "dslabel"], ["const std::vector<int>*", "const int", "const cv::String*"]), _)]),
pub fn cv_hdf_HDF5_dscreate_const_const_vectorLintGR_const_int_const_StringR(instance: *const c_void, sizes: *const c_void, typ: i32, dslabel: *const c_char, ocvrs_return: *mut Result<()>);
// dscreate(const int, const int *, const int, const String &, const int, const int *)(Primitive, VariableArray, Primitive, InString, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:362
// ("cv::hdf::HDF5::dscreate", vec![(pred!(const, ["n_dims", "sizes", "type", "dslabel", "compresslevel", "dims_chunks"], ["const int", "const int*", "const int", "const cv::String*", "const int", "const int*"]), _)]),
pub fn cv_hdf_HDF5_dscreate_const_const_int_const_intX_const_int_const_StringR_const_int_const_intX(instance: *const c_void, n_dims: i32, sizes: *const i32, typ: i32, dslabel: *const c_char, compresslevel: i32, dims_chunks: *const i32, ocvrs_return: *mut Result<()>);
// dsgetsize(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:380
// ("cv::hdf::HDF5::dsgetsize", vec![(pred!(const, ["dslabel", "dims_flag"], ["const cv::String*", "int"]), _)]),
pub fn cv_hdf_HDF5_dsgetsize_const_const_StringR_int(instance: *const c_void, dslabel: *const c_char, dims_flag: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::hdf::HDF5::dsgetsize(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:380
// ("cv::hdf::HDF5::dsgetsize", vec![(pred!(const, ["dslabel"], ["const cv::String*"]), _)]),
pub fn cv_hdf_HDF5_dsgetsize_const_const_StringR(instance: *const c_void, dslabel: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// dsgettype(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:391
// ("cv::hdf::HDF5::dsgettype", vec![(pred!(const, ["dslabel"], ["const cv::String*"]), _)]),
pub fn cv_hdf_HDF5_dsgettype_const_const_StringR(instance: *const c_void, dslabel: *const c_char, ocvrs_return: *mut Result<i32>);
// dswrite(InputArray, const String &)(InputArray, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:394
// ("cv::hdf::HDF5::dswrite", vec![(pred!(const, ["Array", "dslabel"], ["const cv::_InputArray*", "const cv::String*"]), _)]),
pub fn cv_hdf_HDF5_dswrite_const_const__InputArrayR_const_StringR(instance: *const c_void, array: *const c_void, dslabel: *const c_char, ocvrs_return: *mut Result<()>);
// dswrite(InputArray, const String &, const vector<int> &, const vector<int> &)(InputArray, InString, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:399
// ("cv::hdf::HDF5::dswrite", vec![(pred!(const, ["Array", "dslabel", "dims_offset", "dims_counts"], ["const cv::_InputArray*", "const cv::String*", "const std::vector<int>*", "const std::vector<int>*"]), _)]),
pub fn cv_hdf_HDF5_dswrite_const_const__InputArrayR_const_StringR_const_vectorLintGR_const_vectorLintGR(instance: *const c_void, array: *const c_void, dslabel: *const c_char, dims_offset: *const c_void, dims_counts: *const c_void, ocvrs_return: *mut Result<()>);
// cv::hdf::HDF5::dswrite(InputArray, InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:399
// ("cv::hdf::HDF5::dswrite", vec![(pred!(const, ["Array", "dslabel", "dims_offset"], ["const cv::_InputArray*", "const cv::String*", "const std::vector<int>*"]), _)]),
pub fn cv_hdf_HDF5_dswrite_const_const__InputArrayR_const_StringR_const_vectorLintGR(instance: *const c_void, array: *const c_void, dslabel: *const c_char, dims_offset: *const c_void, ocvrs_return: *mut Result<()>);
// dsinsert(InputArray, const String &)(InputArray, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:468
// ("cv::hdf::HDF5::dsinsert", vec![(pred!(const, ["Array", "dslabel"], ["const cv::_InputArray*", "const cv::String*"]), _)]),
pub fn cv_hdf_HDF5_dsinsert_const_const__InputArrayR_const_StringR(instance: *const c_void, array: *const c_void, dslabel: *const c_char, ocvrs_return: *mut Result<()>);
// dsinsert(InputArray, const String &, const vector<int> &, const vector<int> &)(InputArray, InString, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:473
// ("cv::hdf::HDF5::dsinsert", vec![(pred!(const, ["Array", "dslabel", "dims_offset", "dims_counts"], ["const cv::_InputArray*", "const cv::String*", "const std::vector<int>*", "const std::vector<int>*"]), _)]),
pub fn cv_hdf_HDF5_dsinsert_const_const__InputArrayR_const_StringR_const_vectorLintGR_const_vectorLintGR(instance: *const c_void, array: *const c_void, dslabel: *const c_char, dims_offset: *const c_void, dims_counts: *const c_void, ocvrs_return: *mut Result<()>);
// cv::hdf::HDF5::dsinsert(InputArray, InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:473
// ("cv::hdf::HDF5::dsinsert", vec![(pred!(const, ["Array", "dslabel", "dims_offset"], ["const cv::_InputArray*", "const cv::String*", "const std::vector<int>*"]), _)]),
pub fn cv_hdf_HDF5_dsinsert_const_const__InputArrayR_const_StringR_const_vectorLintGR(instance: *const c_void, array: *const c_void, dslabel: *const c_char, dims_offset: *const c_void, ocvrs_return: *mut Result<()>);
// dsread(OutputArray, const String &)(OutputArray, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:528
// ("cv::hdf::HDF5::dsread", vec![(pred!(const, ["Array", "dslabel"], ["const cv::_OutputArray*", "const cv::String*"]), _)]),
pub fn cv_hdf_HDF5_dsread_const_const__OutputArrayR_const_StringR(instance: *const c_void, array: *const c_void, dslabel: *const c_char, ocvrs_return: *mut Result<()>);
// dsread(OutputArray, const String &, const vector<int> &, const vector<int> &)(OutputArray, InString, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:533
// ("cv::hdf::HDF5::dsread", vec![(pred!(const, ["Array", "dslabel", "dims_offset", "dims_counts"], ["const cv::_OutputArray*", "const cv::String*", "const std::vector<int>*", "const std::vector<int>*"]), _)]),
pub fn cv_hdf_HDF5_dsread_const_const__OutputArrayR_const_StringR_const_vectorLintGR_const_vectorLintGR(instance: *const c_void, array: *const c_void, dslabel: *const c_char, dims_offset: *const c_void, dims_counts: *const c_void, ocvrs_return: *mut Result<()>);
// cv::hdf::HDF5::dsread(OutputArray, InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:533
// ("cv::hdf::HDF5::dsread", vec![(pred!(const, ["Array", "dslabel", "dims_offset"], ["const cv::_OutputArray*", "const cv::String*", "const std::vector<int>*"]), _)]),
pub fn cv_hdf_HDF5_dsread_const_const__OutputArrayR_const_StringR_const_vectorLintGR(instance: *const c_void, array: *const c_void, dslabel: *const c_char, dims_offset: *const c_void, ocvrs_return: *mut Result<()>);
// kpgetsize(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:591
// ("cv::hdf::HDF5::kpgetsize", vec![(pred!(const, ["kplabel", "dims_flag"], ["const cv::String*", "int"]), _)]),
pub fn cv_hdf_HDF5_kpgetsize_const_const_StringR_int(instance: *const c_void, kplabel: *const c_char, dims_flag: i32, ocvrs_return: *mut Result<i32>);
// cv::hdf::HDF5::kpgetsize(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:591
// ("cv::hdf::HDF5::kpgetsize", vec![(pred!(const, ["kplabel"], ["const cv::String*"]), _)]),
pub fn cv_hdf_HDF5_kpgetsize_const_const_StringR(instance: *const c_void, kplabel: *const c_char, ocvrs_return: *mut Result<i32>);
// kpcreate(const int, const String &, const int, const int)(Primitive, InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:628
// ("cv::hdf::HDF5::kpcreate", vec![(pred!(const, ["size", "kplabel", "compresslevel", "chunks"], ["const int", "const cv::String*", "const int", "const int"]), _)]),
pub fn cv_hdf_HDF5_kpcreate_const_const_int_const_StringR_const_int_const_int(instance: *const c_void, size: i32, kplabel: *const c_char, compresslevel: i32, chunks: i32, ocvrs_return: *mut Result<()>);
// cv::hdf::HDF5::kpcreate(Primitive, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:628
// ("cv::hdf::HDF5::kpcreate", vec![(pred!(const, ["size", "kplabel"], ["const int", "const cv::String*"]), _)]),
pub fn cv_hdf_HDF5_kpcreate_const_const_int_const_StringR(instance: *const c_void, size: i32, kplabel: *const c_char, ocvrs_return: *mut Result<()>);
// kpwrite(const vector<KeyPoint>, const String &, const int, const int)(CppPassByVoidPtr, InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:681
// ("cv::hdf::HDF5::kpwrite", vec![(pred!(const, ["keypoints", "kplabel", "offset", "counts"], ["const std::vector<cv::KeyPoint>", "const cv::String*", "const int", "const int"]), _)]),
pub fn cv_hdf_HDF5_kpwrite_const_const_vectorLKeyPointG_const_StringR_const_int_const_int(instance: *const c_void, keypoints: *const c_void, kplabel: *const c_char, offset: i32, counts: i32, ocvrs_return: *mut Result<()>);
// cv::hdf::HDF5::kpwrite(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:681
// ("cv::hdf::HDF5::kpwrite", vec![(pred!(const, ["keypoints", "kplabel"], ["const std::vector<cv::KeyPoint>", "const cv::String*"]), _)]),
pub fn cv_hdf_HDF5_kpwrite_const_const_vectorLKeyPointG_const_StringR(instance: *const c_void, keypoints: *const c_void, kplabel: *const c_char, ocvrs_return: *mut Result<()>);
// kpinsert(const vector<KeyPoint>, const String &, const int, const int)(CppPassByVoidPtr, InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:716
// ("cv::hdf::HDF5::kpinsert", vec![(pred!(const, ["keypoints", "kplabel", "offset", "counts"], ["const std::vector<cv::KeyPoint>", "const cv::String*", "const int", "const int"]), _)]),
pub fn cv_hdf_HDF5_kpinsert_const_const_vectorLKeyPointG_const_StringR_const_int_const_int(instance: *const c_void, keypoints: *const c_void, kplabel: *const c_char, offset: i32, counts: i32, ocvrs_return: *mut Result<()>);
// cv::hdf::HDF5::kpinsert(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:716
// ("cv::hdf::HDF5::kpinsert", vec![(pred!(const, ["keypoints", "kplabel"], ["const std::vector<cv::KeyPoint>", "const cv::String*"]), _)]),
pub fn cv_hdf_HDF5_kpinsert_const_const_vectorLKeyPointG_const_StringR(instance: *const c_void, keypoints: *const c_void, kplabel: *const c_char, ocvrs_return: *mut Result<()>);
// kpread(vector<KeyPoint> &, const String &, const int, const int)(CppPassByVoidPtr, InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:754
// ("cv::hdf::HDF5::kpread", vec![(pred!(const, ["keypoints", "kplabel", "offset", "counts"], ["std::vector<cv::KeyPoint>*", "const cv::String*", "const int", "const int"]), _)]),
pub fn cv_hdf_HDF5_kpread_const_vectorLKeyPointGR_const_StringR_const_int_const_int(instance: *const c_void, keypoints: *mut c_void, kplabel: *const c_char, offset: i32, counts: i32, ocvrs_return: *mut Result<()>);
// cv::hdf::HDF5::kpread(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/hdf/hdf5.hpp:754
// ("cv::hdf::HDF5::kpread", vec![(pred!(const, ["keypoints", "kplabel"], ["std::vector<cv::KeyPoint>*", "const cv::String*"]), _)]),
pub fn cv_hdf_HDF5_kpread_const_vectorLKeyPointGR_const_StringR(instance: *const c_void, keypoints: *mut c_void, kplabel: *const c_char, ocvrs_return: *mut Result<()>);
// cv::hdf::HDF5::delete() generated
// ("cv::hdf::HDF5::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_hdf_HDF5_delete(instance: *mut c_void);
