// getBackendName(VideoCaptureAPIs)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio/registry.hpp:27
// ("cv::videoio_registry::getBackendName", vec![(pred!(mut, ["api"], ["cv::VideoCaptureAPIs"]), _)]),
pub fn cv_videoio_registry_getBackendName_VideoCaptureAPIs(api: crate::videoio::VideoCaptureAPIs, ocvrs_return: *mut Result<*mut c_void>);
// getBackends()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio/registry.hpp:30
// ("cv::videoio_registry::getBackends", vec![(pred!(mut, [], []), _)]),
pub fn cv_videoio_registry_getBackends(ocvrs_return: *mut Result<*mut c_void>);
// getCameraBackendPluginVersion(VideoCaptureAPIs, int &, int &)(Enum, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio/registry.hpp:48
// ("cv::videoio_registry::getCameraBackendPluginVersion", vec![(pred!(mut, ["api", "version_ABI", "version_API"], ["cv::VideoCaptureAPIs", "int*", "int*"]), _)]),
pub fn cv_videoio_registry_getCameraBackendPluginVersion_VideoCaptureAPIs_intR_intR(api: crate::videoio::VideoCaptureAPIs, version_abi: *mut i32, version_api: *mut i32, ocvrs_return: *mut Result<*mut c_void>);
// getCameraBackends()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio/registry.hpp:33
// ("cv::videoio_registry::getCameraBackends", vec![(pred!(mut, [], []), _)]),
pub fn cv_videoio_registry_getCameraBackends(ocvrs_return: *mut Result<*mut c_void>);
// getStreamBackendPluginVersion(VideoCaptureAPIs, int &, int &)(Enum, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio/registry.hpp:55
// ("cv::videoio_registry::getStreamBackendPluginVersion", vec![(pred!(mut, ["api", "version_ABI", "version_API"], ["cv::VideoCaptureAPIs", "int*", "int*"]), _)]),
pub fn cv_videoio_registry_getStreamBackendPluginVersion_VideoCaptureAPIs_intR_intR(api: crate::videoio::VideoCaptureAPIs, version_abi: *mut i32, version_api: *mut i32, ocvrs_return: *mut Result<*mut c_void>);
// getStreamBackends()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio/registry.hpp:36
// ("cv::videoio_registry::getStreamBackends", vec![(pred!(mut, [], []), _)]),
pub fn cv_videoio_registry_getStreamBackends(ocvrs_return: *mut Result<*mut c_void>);
// getWriterBackendPluginVersion(VideoCaptureAPIs, int &, int &)(Enum, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio/registry.hpp:62
// ("cv::videoio_registry::getWriterBackendPluginVersion", vec![(pred!(mut, ["api", "version_ABI", "version_API"], ["cv::VideoCaptureAPIs", "int*", "int*"]), _)]),
pub fn cv_videoio_registry_getWriterBackendPluginVersion_VideoCaptureAPIs_intR_intR(api: crate::videoio::VideoCaptureAPIs, version_abi: *mut i32, version_api: *mut i32, ocvrs_return: *mut Result<*mut c_void>);
// getWriterBackends()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio/registry.hpp:39
// ("cv::videoio_registry::getWriterBackends", vec![(pred!(mut, [], []), _)]),
pub fn cv_videoio_registry_getWriterBackends(ocvrs_return: *mut Result<*mut c_void>);
// hasBackend(VideoCaptureAPIs)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio/registry.hpp:42
// ("cv::videoio_registry::hasBackend", vec![(pred!(mut, ["api"], ["cv::VideoCaptureAPIs"]), _)]),
pub fn cv_videoio_registry_hasBackend_VideoCaptureAPIs(api: crate::videoio::VideoCaptureAPIs, ocvrs_return: *mut Result<bool>);
// isBackendBuiltIn(VideoCaptureAPIs)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio/registry.hpp:45
// ("cv::videoio_registry::isBackendBuiltIn", vec![(pred!(mut, ["api"], ["cv::VideoCaptureAPIs"]), _)]),
pub fn cv_videoio_registry_isBackendBuiltIn_VideoCaptureAPIs(api: crate::videoio::VideoCaptureAPIs, ocvrs_return: *mut Result<bool>);
// VideoCapture()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:731
// ("cv::VideoCapture::VideoCapture", vec![(pred!(mut, [], []), _)]),
pub fn cv_VideoCapture_VideoCapture(ocvrs_return: *mut Result<*mut c_void>);
// VideoCapture(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:748
// ("cv::VideoCapture::VideoCapture", vec![(pred!(mut, ["filename", "apiPreference"], ["const cv::String*", "int"]), _)]),
pub fn cv_VideoCapture_VideoCapture_const_StringR_int(filename: *const c_char, api_preference: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::VideoCapture::VideoCapture(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:748
// ("cv::VideoCapture::VideoCapture", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_VideoCapture_VideoCapture_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// VideoCapture(const String &, int, const std::vector<int> &)(InString, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:756
// ("cv::VideoCapture::VideoCapture", vec![(pred!(mut, ["filename", "apiPreference", "params"], ["const cv::String*", "int", "const std::vector<int>*"]), _)]),
pub fn cv_VideoCapture_VideoCapture_const_StringR_int_const_vectorLintGR(filename: *const c_char, api_preference: i32, params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// VideoCapture(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:768
// ("cv::VideoCapture::VideoCapture", vec![(pred!(mut, ["index", "apiPreference"], ["int", "int"]), _)]),
pub fn cv_VideoCapture_VideoCapture_int_int(index: i32, api_preference: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::VideoCapture::VideoCapture(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:768
// ("cv::VideoCapture::VideoCapture", vec![(pred!(mut, ["index"], ["int"]), _)]),
pub fn cv_VideoCapture_VideoCapture_int(index: i32, ocvrs_return: *mut Result<*mut c_void>);
// VideoCapture(int, int, const std::vector<int> &)(Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:776
// ("cv::VideoCapture::VideoCapture", vec![(pred!(mut, ["index", "apiPreference", "params"], ["int", "int", "const std::vector<int>*"]), _)]),
pub fn cv_VideoCapture_VideoCapture_int_int_const_vectorLintGR(index: i32, api_preference: i32, params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// open(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:793
// ("cv::VideoCapture::open", vec![(pred!(mut, ["filename", "apiPreference"], ["const cv::String*", "int"]), _)]),
pub fn cv_VideoCapture_open_const_StringR_int(instance: *mut c_void, filename: *const c_char, api_preference: i32, ocvrs_return: *mut Result<bool>);
// cv::VideoCapture::open(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:793
// ("cv::VideoCapture::open", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_VideoCapture_open_const_StringR(instance: *mut c_void, filename: *const c_char, ocvrs_return: *mut Result<bool>);
// open(const String &, int, const std::vector<int> &)(InString, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:806
// ("cv::VideoCapture::open", vec![(pred!(mut, ["filename", "apiPreference", "params"], ["const cv::String*", "int", "const std::vector<int>*"]), _)]),
pub fn cv_VideoCapture_open_const_StringR_int_const_vectorLintGR(instance: *mut c_void, filename: *const c_char, api_preference: i32, params: *const c_void, ocvrs_return: *mut Result<bool>);
// open(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:817
// ("cv::VideoCapture::open", vec![(pred!(mut, ["index", "apiPreference"], ["int", "int"]), _)]),
pub fn cv_VideoCapture_open_int_int(instance: *mut c_void, index: i32, api_preference: i32, ocvrs_return: *mut Result<bool>);
// cv::VideoCapture::open(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:817
// ("cv::VideoCapture::open", vec![(pred!(mut, ["index"], ["int"]), _)]),
pub fn cv_VideoCapture_open_int(instance: *mut c_void, index: i32, ocvrs_return: *mut Result<bool>);
// open(int, int, const std::vector<int> &)(Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:830
// ("cv::VideoCapture::open", vec![(pred!(mut, ["index", "apiPreference", "params"], ["int", "int", "const std::vector<int>*"]), _)]),
pub fn cv_VideoCapture_open_int_int_const_vectorLintGR(instance: *mut c_void, index: i32, api_preference: i32, params: *const c_void, ocvrs_return: *mut Result<bool>);
// isOpened()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:837
// ("cv::VideoCapture::isOpened", vec![(pred!(const, [], []), _)]),
pub fn cv_VideoCapture_isOpened_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// release()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:846
// ("cv::VideoCapture::release", vec![(pred!(mut, [], []), _)]),
pub fn cv_VideoCapture_release(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// grab()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:867
// ("cv::VideoCapture::grab", vec![(pred!(mut, [], []), _)]),
pub fn cv_VideoCapture_grab(instance: *mut c_void, ocvrs_return: *mut Result<bool>);
// retrieve(OutputArray, int)(OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:881
// ("cv::VideoCapture::retrieve", vec![(pred!(mut, ["image", "flag"], ["const cv::_OutputArray*", "int"]), _)]),
pub fn cv_VideoCapture_retrieve_const__OutputArrayR_int(instance: *mut c_void, image: *const c_void, flag: i32, ocvrs_return: *mut Result<bool>);
// cv::VideoCapture::retrieve(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:881
// ("cv::VideoCapture::retrieve", vec![(pred!(mut, ["image"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_VideoCapture_retrieve_const__OutputArrayR(instance: *mut c_void, image: *const c_void, ocvrs_return: *mut Result<bool>);
// read(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:903
// ("cv::VideoCapture::read", vec![(pred!(mut, ["image"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_VideoCapture_read_const__OutputArrayR(instance: *mut c_void, image: *const c_void, ocvrs_return: *mut Result<bool>);
// set(int, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:914
// ("cv::VideoCapture::set", vec![(pred!(mut, ["propId", "value"], ["int", "double"]), _)]),
pub fn cv_VideoCapture_set_int_double(instance: *mut c_void, prop_id: i32, value: f64, ocvrs_return: *mut Result<bool>);
// get(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:933
// ("cv::VideoCapture::get", vec![(pred!(const, ["propId"], ["int"]), _)]),
pub fn cv_VideoCapture_get_const_int(instance: *const c_void, prop_id: i32, ocvrs_return: *mut Result<f64>);
// getBackendName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:939
// ("cv::VideoCapture::getBackendName", vec![(pred!(const, [], []), _)]),
pub fn cv_VideoCapture_getBackendName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setExceptionMode(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:945
// ("cv::VideoCapture::setExceptionMode", vec![(pred!(mut, ["enable"], ["bool"]), _)]),
pub fn cv_VideoCapture_setExceptionMode_bool(instance: *mut c_void, enable: bool, ocvrs_return: *mut Result<()>);
// getExceptionMode()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:948
// ("cv::VideoCapture::getExceptionMode", vec![(pred!(const, [], []), _)]),
pub fn cv_VideoCapture_getExceptionMode_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// waitAny(const std::vector<VideoCapture> &, std::vector<int> &, int64)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:966
// ("cv::VideoCapture::waitAny", vec![(pred!(mut, ["streams", "readyIndex", "timeoutNs"], ["const std::vector<cv::VideoCapture>*", "std::vector<int>*", "int64_t"]), _)]),
pub fn cv_VideoCapture_waitAny_const_vectorLVideoCaptureGR_vectorLintGR_int64_t(streams: *const c_void, ready_index: *mut c_void, timeout_ns: i64, ocvrs_return: *mut Result<bool>);
// cv::VideoCapture::waitAny(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:966
// ("cv::VideoCapture::waitAny", vec![(pred!(mut, ["streams", "readyIndex"], ["const std::vector<cv::VideoCapture>*", "std::vector<int>*"]), _)]),
pub fn cv_VideoCapture_waitAny_const_vectorLVideoCaptureGR_vectorLintGR(streams: *const c_void, ready_index: *mut c_void, ocvrs_return: *mut Result<bool>);
// cv::VideoCapture::delete() generated
// ("cv::VideoCapture::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_VideoCapture_delete(instance: *mut c_void);
// VideoWriter()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:1003
// ("cv::VideoWriter::VideoWriter", vec![(pred!(mut, [], []), _)]),
pub fn cv_VideoWriter_VideoWriter(ocvrs_return: *mut Result<*mut c_void>);
// VideoWriter(const String &, int, double, Size, bool)(InString, Primitive, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:1031
// ("cv::VideoWriter::VideoWriter", vec![(pred!(mut, ["filename", "fourcc", "fps", "frameSize", "isColor"], ["const cv::String*", "int", "double", "cv::Size", "bool"]), _)]),
pub fn cv_VideoWriter_VideoWriter_const_StringR_int_double_Size_bool(filename: *const c_char, fourcc: i32, fps: f64, frame_size: *const core::Size, is_color: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::VideoWriter::VideoWriter(InString, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:1031
// ("cv::VideoWriter::VideoWriter", vec![(pred!(mut, ["filename", "fourcc", "fps", "frameSize"], ["const cv::String*", "int", "double", "cv::Size"]), _)]),
pub fn cv_VideoWriter_VideoWriter_const_StringR_int_double_Size(filename: *const c_char, fourcc: i32, fps: f64, frame_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// VideoWriter(const String &, int, int, double, Size, bool)(InString, Primitive, Primitive, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:1038
// ("cv::VideoWriter::VideoWriter", vec![(pred!(mut, ["filename", "apiPreference", "fourcc", "fps", "frameSize", "isColor"], ["const cv::String*", "int", "int", "double", "cv::Size", "bool"]), _)]),
pub fn cv_VideoWriter_VideoWriter_const_StringR_int_int_double_Size_bool(filename: *const c_char, api_preference: i32, fourcc: i32, fps: f64, frame_size: *const core::Size, is_color: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::VideoWriter::VideoWriter(InString, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:1038
// ("cv::VideoWriter::VideoWriter", vec![(pred!(mut, ["filename", "apiPreference", "fourcc", "fps", "frameSize"], ["const cv::String*", "int", "int", "double", "cv::Size"]), _)]),
pub fn cv_VideoWriter_VideoWriter_const_StringR_int_int_double_Size(filename: *const c_char, api_preference: i32, fourcc: i32, fps: f64, frame_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// VideoWriter(const String &, int, double, const Size &, const std::vector<int> &)(InString, Primitive, Primitive, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:1045
// ("cv::VideoWriter::VideoWriter", vec![(pred!(mut, ["filename", "fourcc", "fps", "frameSize", "params"], ["const cv::String*", "int", "double", "const cv::Size*", "const std::vector<int>*"]), _)]),
pub fn cv_VideoWriter_VideoWriter_const_StringR_int_double_const_SizeR_const_vectorLintGR(filename: *const c_char, fourcc: i32, fps: f64, frame_size: *const core::Size, params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// VideoWriter(const String &, int, int, double, const Size &, const std::vector<int> &)(InString, Primitive, Primitive, Primitive, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:1050
// ("cv::VideoWriter::VideoWriter", vec![(pred!(mut, ["filename", "apiPreference", "fourcc", "fps", "frameSize", "params"], ["const cv::String*", "int", "int", "double", "const cv::Size*", "const std::vector<int>*"]), _)]),
pub fn cv_VideoWriter_VideoWriter_const_StringR_int_int_double_const_SizeR_const_vectorLintGR(filename: *const c_char, api_preference: i32, fourcc: i32, fps: f64, frame_size: *const core::Size, params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// open(const String &, int, double, Size, bool)(InString, Primitive, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:1067
// ("cv::VideoWriter::open", vec![(pred!(mut, ["filename", "fourcc", "fps", "frameSize", "isColor"], ["const cv::String*", "int", "double", "cv::Size", "bool"]), _)]),
pub fn cv_VideoWriter_open_const_StringR_int_double_Size_bool(instance: *mut c_void, filename: *const c_char, fourcc: i32, fps: f64, frame_size: *const core::Size, is_color: bool, ocvrs_return: *mut Result<bool>);
// cv::VideoWriter::open(InString, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:1067
// ("cv::VideoWriter::open", vec![(pred!(mut, ["filename", "fourcc", "fps", "frameSize"], ["const cv::String*", "int", "double", "cv::Size"]), _)]),
pub fn cv_VideoWriter_open_const_StringR_int_double_Size(instance: *mut c_void, filename: *const c_char, fourcc: i32, fps: f64, frame_size: *const core::Size, ocvrs_return: *mut Result<bool>);
// open(const String &, int, int, double, Size, bool)(InString, Primitive, Primitive, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:1072
// ("cv::VideoWriter::open", vec![(pred!(mut, ["filename", "apiPreference", "fourcc", "fps", "frameSize", "isColor"], ["const cv::String*", "int", "int", "double", "cv::Size", "bool"]), _)]),
pub fn cv_VideoWriter_open_const_StringR_int_int_double_Size_bool(instance: *mut c_void, filename: *const c_char, api_preference: i32, fourcc: i32, fps: f64, frame_size: *const core::Size, is_color: bool, ocvrs_return: *mut Result<bool>);
// cv::VideoWriter::open(InString, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:1072
// ("cv::VideoWriter::open", vec![(pred!(mut, ["filename", "apiPreference", "fourcc", "fps", "frameSize"], ["const cv::String*", "int", "int", "double", "cv::Size"]), _)]),
pub fn cv_VideoWriter_open_const_StringR_int_int_double_Size(instance: *mut c_void, filename: *const c_char, api_preference: i32, fourcc: i32, fps: f64, frame_size: *const core::Size, ocvrs_return: *mut Result<bool>);
// open(const String &, int, double, const Size &, const std::vector<int> &)(InString, Primitive, Primitive, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:1077
// ("cv::VideoWriter::open", vec![(pred!(mut, ["filename", "fourcc", "fps", "frameSize", "params"], ["const cv::String*", "int", "double", "const cv::Size*", "const std::vector<int>*"]), _)]),
pub fn cv_VideoWriter_open_const_StringR_int_double_const_SizeR_const_vectorLintGR(instance: *mut c_void, filename: *const c_char, fourcc: i32, fps: f64, frame_size: *const core::Size, params: *const c_void, ocvrs_return: *mut Result<bool>);
// open(const String &, int, int, double, const Size &, const std::vector<int> &)(InString, Primitive, Primitive, Primitive, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:1082
// ("cv::VideoWriter::open", vec![(pred!(mut, ["filename", "apiPreference", "fourcc", "fps", "frameSize", "params"], ["const cv::String*", "int", "int", "double", "const cv::Size*", "const std::vector<int>*"]), _)]),
pub fn cv_VideoWriter_open_const_StringR_int_int_double_const_SizeR_const_vectorLintGR(instance: *mut c_void, filename: *const c_char, api_preference: i32, fourcc: i32, fps: f64, frame_size: *const core::Size, params: *const c_void, ocvrs_return: *mut Result<bool>);
// isOpened()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:1087
// ("cv::VideoWriter::isOpened", vec![(pred!(const, [], []), _)]),
pub fn cv_VideoWriter_isOpened_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// release()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:1094
// ("cv::VideoWriter::release", vec![(pred!(mut, [], []), _)]),
pub fn cv_VideoWriter_release(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// write(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:1113
// ("cv::VideoWriter::write", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
pub fn cv_VideoWriter_write_const__InputArrayR(instance: *mut c_void, image: *const c_void, ocvrs_return: *mut Result<()>);
// set(int, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:1123
// ("cv::VideoWriter::set", vec![(pred!(mut, ["propId", "value"], ["int", "double"]), _)]),
pub fn cv_VideoWriter_set_int_double(instance: *mut c_void, prop_id: i32, value: f64, ocvrs_return: *mut Result<bool>);
// get(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:1133
// ("cv::VideoWriter::get", vec![(pred!(const, ["propId"], ["int"]), _)]),
pub fn cv_VideoWriter_get_const_int(instance: *const c_void, prop_id: i32, ocvrs_return: *mut Result<f64>);
// fourcc(char, char, char, char)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:1142
// ("cv::VideoWriter::fourcc", vec![(pred!(mut, ["c1", "c2", "c3", "c4"], ["char", "char", "char", "char"]), _)]),
pub fn cv_VideoWriter_fourcc_char_char_char_char(c1: c_char, c2: c_char, c3: c_char, c4: c_char, ocvrs_return: *mut Result<i32>);
// getBackendName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/videoio.hpp:1148
// ("cv::VideoWriter::getBackendName", vec![(pred!(const, [], []), _)]),
pub fn cv_VideoWriter_getBackendName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::VideoWriter::delete() generated
// ("cv::VideoWriter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_VideoWriter_delete(instance: *mut c_void);
