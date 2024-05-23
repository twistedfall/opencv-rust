// debugDMatch(cv::InputArray, std::vector<cv::KeyPoint>, cv::InputArray, std::vector<cv::KeyPoint>, std::vector<cv::DMatch>, const CallMetaData &, const char *, const char *, bool)(InputArray, CppPassByVoidPtr, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, TraitClass, InString, InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/dmatch.hpp:24
// ("cvv::impl::debugDMatch", vec![(pred!(mut, ["img1", "keypoints1", "img2", "keypoints2", "matches", "data", "description", "view", "useTrainDescriptor"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>", "const cv::_InputArray*", "std::vector<cv::KeyPoint>", "std::vector<cv::DMatch>", "const cvv::impl::CallMetaData*", "const char*", "const char*", "bool"]), _)]),
pub fn cvv_impl_debugDMatch_const__InputArrayR_vectorLKeyPointG_const__InputArrayR_vectorLKeyPointG_vectorLDMatchG_const_CallMetaDataR_const_charX_const_charX_bool(img1: *const c_void, keypoints1: *mut c_void, img2: *const c_void, keypoints2: *mut c_void, matches: *mut c_void, data: *const c_void, description: *const c_char, view: *const c_char, use_train_descriptor: bool, ocvrs_return: *mut Result<()>);
// debugFilter(cv::InputArray, cv::InputArray, const CallMetaData &, const char *, const char *)(InputArray, InputArray, TraitClass, InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/filter.hpp:24
// ("cvv::impl::debugFilter", vec![(pred!(mut, ["original", "result", "data", "description", "view"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cvv::impl::CallMetaData*", "const char*", "const char*"]), _)]),
pub fn cvv_impl_debugFilter_const__InputArrayR_const__InputArrayR_const_CallMetaDataR_const_charX_const_charX(original: *const c_void, result: *const c_void, data: *const c_void, description: *const c_char, view: *const c_char, ocvrs_return: *mut Result<()>);
// finalShow()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/final_show.hpp:15
// ("cvv::impl::finalShow", vec![(pred!(mut, [], []), _)]),
pub fn cvv_impl_finalShow(ocvrs_return: *mut Result<()>);
// showImage(cv::InputArray, const CallMetaData &, const char *, const char *)(InputArray, TraitClass, InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/show_image.hpp:24
// ("cvv::impl::showImage", vec![(pred!(mut, ["img", "data", "description", "view"], ["const cv::_InputArray*", "const cvv::impl::CallMetaData*", "const char*", "const char*"]), _)]),
pub fn cvv_impl_showImage_const__InputArrayR_const_CallMetaDataR_const_charX_const_charX(img: *const c_void, data: *const c_void, description: *const c_char, view: *const c_char, ocvrs_return: *mut Result<()>);
// CallMetaData()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/call_meta_data.hpp:26
// ("cvv::impl::CallMetaData::CallMetaData", vec![(pred!(mut, [], []), _)]),
pub fn cvv_impl_CallMetaData_CallMetaData(ocvrs_return: *mut Result<*mut c_void>);
// CallMetaData(const char *, size_t, const char *)(InString, Primitive, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/call_meta_data.hpp:36
// ("cvv::impl::CallMetaData::CallMetaData", vec![(pred!(mut, ["file", "line", "function"], ["const char*", "size_t", "const char*"]), _)]),
pub fn cvv_impl_CallMetaData_CallMetaData_const_charX_size_t_const_charX(file: *const c_char, line: size_t, function: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// operator bool()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/call_meta_data.hpp:40
// ("cvv::impl::CallMetaData::operator bool", vec![(pred!(mut, [], []), _)]),
pub fn cvv_impl_CallMetaData_operator_bool(instance: *mut c_void, ocvrs_return: *mut Result<bool>);
// cvv::impl::CallMetaData::implicitClone() generated
// ("cvv::impl::CallMetaData::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cvv_impl_CallMetaData_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cvv::impl::CallMetaData::file() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/call_meta_data.hpp:46
// ("cvv::impl::CallMetaData::file", vec![(pred!(const, [], []), _)]),
pub fn cvv_impl_CallMetaData_propFile_const(instance: *const c_void) -> *mut c_void;
// cvv::impl::CallMetaData::line() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/call_meta_data.hpp:47
// ("cvv::impl::CallMetaData::line", vec![(pred!(const, [], []), _)]),
pub fn cvv_impl_CallMetaData_propLine_const(instance: *const c_void) -> size_t;
// cvv::impl::CallMetaData::function() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/call_meta_data.hpp:48
// ("cvv::impl::CallMetaData::function", vec![(pred!(const, [], []), _)]),
pub fn cvv_impl_CallMetaData_propFunction_const(instance: *const c_void) -> *mut c_void;
// cvv::impl::CallMetaData::isKnown() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/call_meta_data.hpp:53
// ("cvv::impl::CallMetaData::isKnown", vec![(pred!(const, [], []), _)]),
pub fn cvv_impl_CallMetaData_propIsKnown_const(instance: *const c_void) -> bool;
// cvv::impl::CallMetaData::delete() generated
// ("cvv::impl::CallMetaData::delete", vec![(pred!(mut, [], []), _)]),
pub fn cvv_impl_CallMetaData_delete(instance: *mut c_void);
