// getBackendName(VideoCaptureAPIs)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio/registry.hpp:27
// ("cv::videoio_registry::getBackendName", vec![(pred!(mut, ["api"], ["cv::VideoCaptureAPIs"]), _)]),
pub fn cv_videoio_registry_getBackendName_VideoCaptureAPIs(api: crate::videoio::VideoCaptureAPIs, ocvrs_return: *mut Result<*mut c_void>);
// getBackends()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio/registry.hpp:30
// ("cv::videoio_registry::getBackends", vec![(pred!(mut, [], []), _)]),
pub fn cv_videoio_registry_getBackends(ocvrs_return: *mut Result<*mut c_void>);
// getCameraBackends()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio/registry.hpp:33
// ("cv::videoio_registry::getCameraBackends", vec![(pred!(mut, [], []), _)]),
pub fn cv_videoio_registry_getCameraBackends(ocvrs_return: *mut Result<*mut c_void>);
// getStreamBackends()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio/registry.hpp:36
// ("cv::videoio_registry::getStreamBackends", vec![(pred!(mut, [], []), _)]),
pub fn cv_videoio_registry_getStreamBackends(ocvrs_return: *mut Result<*mut c_void>);
// getWriterBackends()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio/registry.hpp:39
// ("cv::videoio_registry::getWriterBackends", vec![(pred!(mut, [], []), _)]),
pub fn cv_videoio_registry_getWriterBackends(ocvrs_return: *mut Result<*mut c_void>);
// VideoCapture()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:630
// ("cv::VideoCapture::VideoCapture", vec![(pred!(mut, [], []), _)]),
pub fn cv_VideoCapture_VideoCapture(ocvrs_return: *mut Result<*mut c_void>);
// VideoCapture(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:637
// ("cv::VideoCapture::VideoCapture", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_VideoCapture_VideoCapture_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// VideoCapture(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:654
// ("cv::VideoCapture::VideoCapture", vec![(pred!(mut, ["filename", "apiPreference"], ["const cv::String*", "int"]), _)]),
pub fn cv_VideoCapture_VideoCapture_const_StringR_int(filename: *const c_char, api_preference: i32, ocvrs_return: *mut Result<*mut c_void>);
// VideoCapture(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:665
// ("cv::VideoCapture::VideoCapture", vec![(pred!(mut, ["index"], ["int"]), _)]),
pub fn cv_VideoCapture_VideoCapture_int(index: i32, ocvrs_return: *mut Result<*mut c_void>);
// VideoCapture(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:677
// ("cv::VideoCapture::VideoCapture", vec![(pred!(mut, ["index", "apiPreference"], ["int", "int"]), _)]),
pub fn cv_VideoCapture_VideoCapture_int_int(index: i32, api_preference: i32, ocvrs_return: *mut Result<*mut c_void>);
// open(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:694
// ("cv::VideoCapture::open", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_VideoCapture_open_const_StringR(instance: *mut c_void, filename: *const c_char, ocvrs_return: *mut Result<bool>);
// open(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:705
// ("cv::VideoCapture::open", vec![(pred!(mut, ["index"], ["int"]), _)]),
pub fn cv_VideoCapture_open_int(instance: *mut c_void, index: i32, ocvrs_return: *mut Result<bool>);
// open(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:715
// ("cv::VideoCapture::open", vec![(pred!(mut, ["cameraNum", "apiPreference"], ["int", "int"]), _)]),
pub fn cv_VideoCapture_open_int_int(instance: *mut c_void, camera_num: i32, api_preference: i32, ocvrs_return: *mut Result<bool>);
// isOpened()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:722
// ("cv::VideoCapture::isOpened", vec![(pred!(const, [], []), _)]),
pub fn cv_VideoCapture_isOpened_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// release()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:731
// ("cv::VideoCapture::release", vec![(pred!(mut, [], []), _)]),
pub fn cv_VideoCapture_release(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// grab()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:752
// ("cv::VideoCapture::grab", vec![(pred!(mut, [], []), _)]),
pub fn cv_VideoCapture_grab(instance: *mut c_void, ocvrs_return: *mut Result<bool>);
// retrieve(OutputArray, int)(OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:770
// ("cv::VideoCapture::retrieve", vec![(pred!(mut, ["image", "flag"], ["const cv::_OutputArray*", "int"]), _)]),
pub fn cv_VideoCapture_retrieve_const__OutputArrayR_int(instance: *mut c_void, image: *const c_void, flag: i32, ocvrs_return: *mut Result<bool>);
// cv::VideoCapture::retrieve(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:770
// ("cv::VideoCapture::retrieve", vec![(pred!(mut, ["image"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_VideoCapture_retrieve_const__OutputArrayR(instance: *mut c_void, image: *const c_void, ocvrs_return: *mut Result<bool>);
// read(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:796
// ("cv::VideoCapture::read", vec![(pred!(mut, ["image"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_VideoCapture_read_const__OutputArrayR(instance: *mut c_void, image: *const c_void, ocvrs_return: *mut Result<bool>);
// set(int, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:807
// ("cv::VideoCapture::set", vec![(pred!(mut, ["propId", "value"], ["int", "double"]), _)]),
pub fn cv_VideoCapture_set_int_double(instance: *mut c_void, prop_id: i32, value: f64, ocvrs_return: *mut Result<bool>);
// get(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:826
// ("cv::VideoCapture::get", vec![(pred!(const, ["propId"], ["int"]), _)]),
pub fn cv_VideoCapture_get_const_int(instance: *const c_void, prop_id: i32, ocvrs_return: *mut Result<f64>);
// open(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:837
// ("cv::VideoCapture::open", vec![(pred!(mut, ["filename", "apiPreference"], ["const cv::String*", "int"]), _)]),
pub fn cv_VideoCapture_open_const_StringR_int(instance: *mut c_void, filename: *const c_char, api_preference: i32, ocvrs_return: *mut Result<bool>);
// getBackendName()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:843
// ("cv::VideoCapture::getBackendName", vec![(pred!(const, [], []), _)]),
pub fn cv_VideoCapture_getBackendName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::VideoCapture::delete() generated
// ("cv::VideoCapture::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_VideoCapture_delete(instance: *mut c_void);
// VideoWriter()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:874
// ("cv::VideoWriter::VideoWriter", vec![(pred!(mut, [], []), _)]),
pub fn cv_VideoWriter_VideoWriter(ocvrs_return: *mut Result<*mut c_void>);
// VideoWriter(const String &, int, double, Size, bool)(InString, Primitive, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:899
// ("cv::VideoWriter::VideoWriter", vec![(pred!(mut, ["filename", "fourcc", "fps", "frameSize", "isColor"], ["const cv::String*", "int", "double", "cv::Size", "bool"]), _)]),
pub fn cv_VideoWriter_VideoWriter_const_StringR_int_double_Size_bool(filename: *const c_char, fourcc: i32, fps: f64, frame_size: *const core::Size, is_color: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::VideoWriter::VideoWriter(InString, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:899
// ("cv::VideoWriter::VideoWriter", vec![(pred!(mut, ["filename", "fourcc", "fps", "frameSize"], ["const cv::String*", "int", "double", "cv::Size"]), _)]),
pub fn cv_VideoWriter_VideoWriter_const_StringR_int_double_Size(filename: *const c_char, fourcc: i32, fps: f64, frame_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// VideoWriter(const String &, int, int, double, Size, bool)(InString, Primitive, Primitive, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:906
// ("cv::VideoWriter::VideoWriter", vec![(pred!(mut, ["filename", "apiPreference", "fourcc", "fps", "frameSize", "isColor"], ["const cv::String*", "int", "int", "double", "cv::Size", "bool"]), _)]),
pub fn cv_VideoWriter_VideoWriter_const_StringR_int_int_double_Size_bool(filename: *const c_char, api_preference: i32, fourcc: i32, fps: f64, frame_size: *const core::Size, is_color: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::VideoWriter::VideoWriter(InString, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:906
// ("cv::VideoWriter::VideoWriter", vec![(pred!(mut, ["filename", "apiPreference", "fourcc", "fps", "frameSize"], ["const cv::String*", "int", "int", "double", "cv::Size"]), _)]),
pub fn cv_VideoWriter_VideoWriter_const_StringR_int_int_double_Size(filename: *const c_char, api_preference: i32, fourcc: i32, fps: f64, frame_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// open(const String &, int, double, Size, bool)(InString, Primitive, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:923
// ("cv::VideoWriter::open", vec![(pred!(mut, ["filename", "fourcc", "fps", "frameSize", "isColor"], ["const cv::String*", "int", "double", "cv::Size", "bool"]), _)]),
pub fn cv_VideoWriter_open_const_StringR_int_double_Size_bool(instance: *mut c_void, filename: *const c_char, fourcc: i32, fps: f64, frame_size: *const core::Size, is_color: bool, ocvrs_return: *mut Result<bool>);
// cv::VideoWriter::open(InString, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:923
// ("cv::VideoWriter::open", vec![(pred!(mut, ["filename", "fourcc", "fps", "frameSize"], ["const cv::String*", "int", "double", "cv::Size"]), _)]),
pub fn cv_VideoWriter_open_const_StringR_int_double_Size(instance: *mut c_void, filename: *const c_char, fourcc: i32, fps: f64, frame_size: *const core::Size, ocvrs_return: *mut Result<bool>);
// open(const String &, int, int, double, Size, bool)(InString, Primitive, Primitive, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:928
// ("cv::VideoWriter::open", vec![(pred!(mut, ["filename", "apiPreference", "fourcc", "fps", "frameSize", "isColor"], ["const cv::String*", "int", "int", "double", "cv::Size", "bool"]), _)]),
pub fn cv_VideoWriter_open_const_StringR_int_int_double_Size_bool(instance: *mut c_void, filename: *const c_char, api_preference: i32, fourcc: i32, fps: f64, frame_size: *const core::Size, is_color: bool, ocvrs_return: *mut Result<bool>);
// cv::VideoWriter::open(InString, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:928
// ("cv::VideoWriter::open", vec![(pred!(mut, ["filename", "apiPreference", "fourcc", "fps", "frameSize"], ["const cv::String*", "int", "int", "double", "cv::Size"]), _)]),
pub fn cv_VideoWriter_open_const_StringR_int_int_double_Size(instance: *mut c_void, filename: *const c_char, api_preference: i32, fourcc: i32, fps: f64, frame_size: *const core::Size, ocvrs_return: *mut Result<bool>);
// isOpened()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:933
// ("cv::VideoWriter::isOpened", vec![(pred!(const, [], []), _)]),
pub fn cv_VideoWriter_isOpened_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// release()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:940
// ("cv::VideoWriter::release", vec![(pred!(mut, [], []), _)]),
pub fn cv_VideoWriter_release(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// write(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:954
// ("cv::VideoWriter::write", vec![(pred!(mut, ["image"], ["const cv::Mat*"]), _)]),
pub fn cv_VideoWriter_write_const_MatR(instance: *mut c_void, image: *const c_void, ocvrs_return: *mut Result<()>);
// set(int, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:964
// ("cv::VideoWriter::set", vec![(pred!(mut, ["propId", "value"], ["int", "double"]), _)]),
pub fn cv_VideoWriter_set_int_double(instance: *mut c_void, prop_id: i32, value: f64, ocvrs_return: *mut Result<bool>);
// get(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:974
// ("cv::VideoWriter::get", vec![(pred!(const, ["propId"], ["int"]), _)]),
pub fn cv_VideoWriter_get_const_int(instance: *const c_void, prop_id: i32, ocvrs_return: *mut Result<f64>);
// fourcc(char, char, char, char)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:983
// ("cv::VideoWriter::fourcc", vec![(pred!(mut, ["c1", "c2", "c3", "c4"], ["char", "char", "char", "char"]), _)]),
pub fn cv_VideoWriter_fourcc_char_char_char_char(c1: c_char, c2: c_char, c3: c_char, c4: c_char, ocvrs_return: *mut Result<i32>);
// getBackendName()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:989
// ("cv::VideoWriter::getBackendName", vec![(pred!(const, [], []), _)]),
pub fn cv_VideoWriter_getBackendName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::VideoWriter::delete() generated
// ("cv::VideoWriter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_VideoWriter_delete(instance: *mut c_void);
