// computeNormals(const Mesh &, OutputArray)(TraitClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/vizcore.hpp:214
// ("cv::viz::computeNormals", vec![(pred!(mut, ["mesh", "normals"], ["const cv::viz::Mesh*", "const cv::_OutputArray*"]), _)]),
pub fn cv_viz_computeNormals_const_MeshR_const__OutputArrayR(mesh: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<()>);
// getWindowByName(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/vizcore.hpp:97
// ("cv::viz::getWindowByName", vec![(pred!(mut, ["window_name"], ["const cv::String*"]), _)]),
pub fn cv_viz_getWindowByName_const_StringR(window_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::imshow(InString, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/vizcore.hpp:103
// ("cv::viz::imshow", vec![(pred!(mut, ["window_name", "image"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
pub fn cv_viz_imshow_const_StringR_const__InputArrayR(window_name: *const c_char, image: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// imshow(const String &, InputArray, const Size &)(InString, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/vizcore.hpp:103
// ("cv::viz::imshow", vec![(pred!(mut, ["window_name", "image", "window_size"], ["const cv::String*", "const cv::_InputArray*", "const cv::Size*"]), _)]),
pub fn cv_viz_imshow_const_StringR_const__InputArrayR_const_SizeR(window_name: *const c_char, image: *const c_void, window_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// makeCameraPose(const Vec3d &, const Vec3d &, const Vec3d &)(SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/vizcore.hpp:84
// ("cv::viz::makeCameraPose", vec![(pred!(mut, ["position", "focal_point", "y_dir"], ["const cv::Vec3d*", "const cv::Vec3d*", "const cv::Vec3d*"]), _)]),
pub fn cv_viz_makeCameraPose_const_Vec3dR_const_Vec3dR_const_Vec3dR(position: *const core::Vec3d, focal_point: *const core::Vec3d, y_dir: *const core::Vec3d, ocvrs_return: *mut Result<core::Affine3d>);
// cv::viz::makeTransformToGlobal(SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/vizcore.hpp:73
// ("cv::viz::makeTransformToGlobal", vec![(pred!(mut, ["axis_x", "axis_y", "axis_z"], ["const cv::Vec3d*", "const cv::Vec3d*", "const cv::Vec3d*"]), _)]),
pub fn cv_viz_makeTransformToGlobal_const_Vec3dR_const_Vec3dR_const_Vec3dR(axis_x: *const core::Vec3d, axis_y: *const core::Vec3d, axis_z: *const core::Vec3d, ocvrs_return: *mut Result<core::Affine3d>);
// makeTransformToGlobal(const Vec3d &, const Vec3d &, const Vec3d &, const Vec3d &)(SimpleClass, SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/vizcore.hpp:73
// ("cv::viz::makeTransformToGlobal", vec![(pred!(mut, ["axis_x", "axis_y", "axis_z", "origin"], ["const cv::Vec3d*", "const cv::Vec3d*", "const cv::Vec3d*", "const cv::Vec3d*"]), _)]),
pub fn cv_viz_makeTransformToGlobal_const_Vec3dR_const_Vec3dR_const_Vec3dR_const_Vec3dR(axis_x: *const core::Vec3d, axis_y: *const core::Vec3d, axis_z: *const core::Vec3d, origin: *const core::Vec3d, ocvrs_return: *mut Result<core::Affine3d>);
// cv::viz::readCloud(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/vizcore.hpp:160
// ("cv::viz::readCloud", vec![(pred!(mut, ["file"], ["const cv::String*"]), _)]),
pub fn cv_viz_readCloud_const_StringR(file: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// readCloud(const String &, OutputArray, OutputArray)(InString, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/vizcore.hpp:160
// ("cv::viz::readCloud", vec![(pred!(mut, ["file", "colors", "normals"], ["const cv::String*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_viz_readCloud_const_StringR_const__OutputArrayR_const__OutputArrayR(file: *const c_char, colors: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// readMesh(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/vizcore.hpp:165
// ("cv::viz::readMesh", vec![(pred!(mut, ["file"], ["const cv::String*"]), _)]),
pub fn cv_viz_readMesh_const_StringR(file: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::readPose(InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/vizcore.hpp:175
// ("cv::viz::readPose", vec![(pred!(mut, ["file", "pose"], ["const cv::String*", "cv::Affine3d*"]), _)]),
pub fn cv_viz_readPose_const_StringR_Affine3dR(file: *const c_char, pose: *mut core::Affine3d, ocvrs_return: *mut Result<bool>);
// readPose(const String &, Affine3d &, const String &)(InString, SimpleClass, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/vizcore.hpp:175
// ("cv::viz::readPose", vec![(pred!(mut, ["file", "pose", "tag"], ["const cv::String*", "cv::Affine3d*", "const cv::String*"]), _)]),
pub fn cv_viz_readPose_const_StringR_Affine3dR_const_StringR(file: *const c_char, pose: *mut core::Affine3d, tag: *const c_char, ocvrs_return: *mut Result<bool>);
// cv::viz::readTrajectory(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/vizcore.hpp:206
// ("cv::viz::readTrajectory", vec![(pred!(mut, ["traj"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_viz_readTrajectory_const__OutputArrayR(traj: *const c_void, ocvrs_return: *mut Result<()>);
// readTrajectory(OutputArray, const String &, int, int, const String &)(OutputArray, InString, Primitive, Primitive, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/vizcore.hpp:206
// ("cv::viz::readTrajectory", vec![(pred!(mut, ["traj", "files_format", "start", "end", "tag"], ["const cv::_OutputArray*", "const cv::String*", "int", "int", "const cv::String*"]), _)]),
pub fn cv_viz_readTrajectory_const__OutputArrayR_const_StringR_int_int_const_StringR(traj: *const c_void, files_format: *const c_char, start: i32, end: i32, tag: *const c_char, ocvrs_return: *mut Result<()>);
// unregisterAllWindows()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/vizcore.hpp:100
// ("cv::viz::unregisterAllWindows", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_unregisterAllWindows(ocvrs_return: *mut Result<()>);
// cv::viz::writeCloud(InString, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/vizcore.hpp:151
// ("cv::viz::writeCloud", vec![(pred!(mut, ["file", "cloud"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
pub fn cv_viz_writeCloud_const_StringR_const__InputArrayR(file: *const c_char, cloud: *const c_void, ocvrs_return: *mut Result<()>);
// writeCloud(const String &, InputArray, InputArray, InputArray, bool)(InString, InputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/vizcore.hpp:151
// ("cv::viz::writeCloud", vec![(pred!(mut, ["file", "cloud", "colors", "normals", "binary"], ["const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "bool"]), _)]),
pub fn cv_viz_writeCloud_const_StringR_const__InputArrayR_const__InputArrayR_const__InputArrayR_bool(file: *const c_char, cloud: *const c_void, colors: *const c_void, normals: *const c_void, binary: bool, ocvrs_return: *mut Result<()>);
// cv::viz::writePose(InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/vizcore.hpp:181
// ("cv::viz::writePose", vec![(pred!(mut, ["file", "pose"], ["const cv::String*", "const cv::Affine3d*"]), _)]),
pub fn cv_viz_writePose_const_StringR_const_Affine3dR(file: *const c_char, pose: *const core::Affine3d, ocvrs_return: *mut Result<()>);
// writePose(const String &, const Affine3d &, const String &)(InString, SimpleClass, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/vizcore.hpp:181
// ("cv::viz::writePose", vec![(pred!(mut, ["file", "pose", "tag"], ["const cv::String*", "const cv::Affine3d*", "const cv::String*"]), _)]),
pub fn cv_viz_writePose_const_StringR_const_Affine3dR_const_StringR(file: *const c_char, pose: *const core::Affine3d, tag: *const c_char, ocvrs_return: *mut Result<()>);
// cv::viz::writeTrajectory(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/vizcore.hpp:193
// ("cv::viz::writeTrajectory", vec![(pred!(mut, ["traj"], ["const cv::_InputArray*"]), _)]),
pub fn cv_viz_writeTrajectory_const__InputArrayR(traj: *const c_void, ocvrs_return: *mut Result<()>);
// writeTrajectory(InputArray, const String &, int, const String &)(InputArray, InString, Primitive, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/vizcore.hpp:193
// ("cv::viz::writeTrajectory", vec![(pred!(mut, ["traj", "files_format", "start", "tag"], ["const cv::_InputArray*", "const cv::String*", "int", "const cv::String*"]), _)]),
pub fn cv_viz_writeTrajectory_const__InputArrayR_const_StringR_int_const_StringR(traj: *const c_void, files_format: *const c_char, start: i32, tag: *const c_char, ocvrs_return: *mut Result<()>);
// Camera(double, double, double, double, const Size &)(Primitive, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:176
// ("cv::viz::Camera::Camera", vec![(pred!(mut, ["fx", "fy", "cx", "cy", "window_size"], ["double", "double", "double", "double", "const cv::Size*"]), _)]),
pub fn cv_viz_Camera_Camera_double_double_double_double_const_SizeR(fx: f64, fy: f64, cx: f64, cy: f64, window_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// Camera(const Vec2d &, const Size &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:183
// ("cv::viz::Camera::Camera", vec![(pred!(mut, ["fov", "window_size"], ["const cv::Vec2d*", "const cv::Size*"]), _)]),
pub fn cv_viz_Camera_Camera_const_Vec2dR_const_SizeR(fov: *const core::Vec2d, window_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// Camera(const Matx33d &, const Size &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:197
// ("cv::viz::Camera::Camera", vec![(pred!(mut, ["K", "window_size"], ["const cv::Matx33d*", "const cv::Size*"]), _)]),
pub fn cv_viz_Camera_Camera_const_Matx33dR_const_SizeR(k: *const core::Matx33d, window_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// Camera(const Matx44d &, const Size &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:213
// ("cv::viz::Camera::Camera", vec![(pred!(mut, ["proj", "window_size"], ["const cv::Matx44d*", "const cv::Size*"]), _)]),
pub fn cv_viz_Camera_Camera_const_Matx44dR_const_SizeR(proj: *const core::Matx44d, window_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// getClip()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:215
// ("cv::viz::Camera::getClip", vec![(pred!(const, [], []), _)]),
pub fn cv_viz_Camera_getClip_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec2d>);
// setClip(const Vec2d &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:216
// ("cv::viz::Camera::setClip", vec![(pred!(mut, ["clip"], ["const cv::Vec2d*"]), _)]),
pub fn cv_viz_Camera_setClip_const_Vec2dR(instance: *mut c_void, clip: *const core::Vec2d, ocvrs_return: *mut Result<()>);
// getWindowSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:218
// ("cv::viz::Camera::getWindowSize", vec![(pred!(const, [], []), _)]),
pub fn cv_viz_Camera_getWindowSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// setWindowSize(const Size &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:219
// ("cv::viz::Camera::setWindowSize", vec![(pred!(mut, ["window_size"], ["const cv::Size*"]), _)]),
pub fn cv_viz_Camera_setWindowSize_const_SizeR(instance: *mut c_void, window_size: *const core::Size, ocvrs_return: *mut Result<()>);
// getFov()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:221
// ("cv::viz::Camera::getFov", vec![(pred!(const, [], []), _)]),
pub fn cv_viz_Camera_getFov_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec2d>);
// setFov(const Vec2d &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:222
// ("cv::viz::Camera::setFov", vec![(pred!(mut, ["fov"], ["const cv::Vec2d*"]), _)]),
pub fn cv_viz_Camera_setFov_const_Vec2dR(instance: *mut c_void, fov: *const core::Vec2d, ocvrs_return: *mut Result<()>);
// getPrincipalPoint()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:224
// ("cv::viz::Camera::getPrincipalPoint", vec![(pred!(const, [], []), _)]),
pub fn cv_viz_Camera_getPrincipalPoint_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec2d>);
// getFocalLength()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:225
// ("cv::viz::Camera::getFocalLength", vec![(pred!(const, [], []), _)]),
pub fn cv_viz_Camera_getFocalLength_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec2d>);
// computeProjectionMatrix(Matx44d &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:240
// ("cv::viz::Camera::computeProjectionMatrix", vec![(pred!(const, ["proj"], ["cv::Matx44d*"]), _)]),
pub fn cv_viz_Camera_computeProjectionMatrix_const_Matx44dR(instance: *const c_void, proj: *mut core::Matx44d, ocvrs_return: *mut Result<()>);
// KinectCamera(const Size &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:250
// ("cv::viz::Camera::KinectCamera", vec![(pred!(mut, ["window_size"], ["const cv::Size*"]), _)]),
pub fn cv_viz_Camera_KinectCamera_const_SizeR(window_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::Camera::delete() generated
// ("cv::viz::Camera::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Camera_delete(instance: *mut c_void);
// Color()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:66
// ("cv::viz::Color::Color", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_Color(ocvrs_return: *mut Result<*mut c_void>);
// Color(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:68
// ("cv::viz::Color::Color", vec![(pred!(mut, ["gray"], ["double"]), _)]),
pub fn cv_viz_Color_Color_double(gray: f64, ocvrs_return: *mut Result<*mut c_void>);
// Color(double, double, double)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:69
// ("cv::viz::Color::Color", vec![(pred!(mut, ["blue", "green", "red"], ["double", "double", "double"]), _)]),
pub fn cv_viz_Color_Color_double_double_double(blue: f64, green: f64, red: f64, ocvrs_return: *mut Result<*mut c_void>);
// Color(const Scalar &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:71
// ("cv::viz::Color::Color", vec![(pred!(mut, ["color"], ["const cv::Scalar*"]), _)]),
pub fn cv_viz_Color_Color_const_ScalarR(color: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// operator Vec()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:73
// ("cv::viz::Color::operator cv::Vec3b", vec![(pred!(const, [], []), _)]),
pub fn cv_viz_Color_operator_cv_Vec3b_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec3b>);
// black()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:75
// ("cv::viz::Color::black", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_black(ocvrs_return: *mut Result<*mut c_void>);
// blue()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:76
// ("cv::viz::Color::blue", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_blue(ocvrs_return: *mut Result<*mut c_void>);
// green()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:77
// ("cv::viz::Color::green", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_green(ocvrs_return: *mut Result<*mut c_void>);
// red()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:78
// ("cv::viz::Color::red", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_red(ocvrs_return: *mut Result<*mut c_void>);
// cyan()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:79
// ("cv::viz::Color::cyan", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_cyan(ocvrs_return: *mut Result<*mut c_void>);
// yellow()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:80
// ("cv::viz::Color::yellow", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_yellow(ocvrs_return: *mut Result<*mut c_void>);
// magenta()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:81
// ("cv::viz::Color::magenta", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_magenta(ocvrs_return: *mut Result<*mut c_void>);
// white()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:82
// ("cv::viz::Color::white", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_white(ocvrs_return: *mut Result<*mut c_void>);
// gray()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:84
// ("cv::viz::Color::gray", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_gray(ocvrs_return: *mut Result<*mut c_void>);
// silver()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:85
// ("cv::viz::Color::silver", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_silver(ocvrs_return: *mut Result<*mut c_void>);
// mlab()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:87
// ("cv::viz::Color::mlab", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_mlab(ocvrs_return: *mut Result<*mut c_void>);
// navy()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:89
// ("cv::viz::Color::navy", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_navy(ocvrs_return: *mut Result<*mut c_void>);
// maroon()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:90
// ("cv::viz::Color::maroon", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_maroon(ocvrs_return: *mut Result<*mut c_void>);
// teal()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:91
// ("cv::viz::Color::teal", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_teal(ocvrs_return: *mut Result<*mut c_void>);
// olive()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:92
// ("cv::viz::Color::olive", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_olive(ocvrs_return: *mut Result<*mut c_void>);
// purple()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:93
// ("cv::viz::Color::purple", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_purple(ocvrs_return: *mut Result<*mut c_void>);
// azure()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:94
// ("cv::viz::Color::azure", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_azure(ocvrs_return: *mut Result<*mut c_void>);
// chartreuse()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:95
// ("cv::viz::Color::chartreuse", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_chartreuse(ocvrs_return: *mut Result<*mut c_void>);
// rose()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:96
// ("cv::viz::Color::rose", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_rose(ocvrs_return: *mut Result<*mut c_void>);
// lime()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:98
// ("cv::viz::Color::lime", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_lime(ocvrs_return: *mut Result<*mut c_void>);
// gold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:99
// ("cv::viz::Color::gold", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_gold(ocvrs_return: *mut Result<*mut c_void>);
// orange()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:100
// ("cv::viz::Color::orange", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_orange(ocvrs_return: *mut Result<*mut c_void>);
// orange_red()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:101
// ("cv::viz::Color::orange_red", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_orange_red(ocvrs_return: *mut Result<*mut c_void>);
// indigo()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:102
// ("cv::viz::Color::indigo", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_indigo(ocvrs_return: *mut Result<*mut c_void>);
// brown()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:104
// ("cv::viz::Color::brown", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_brown(ocvrs_return: *mut Result<*mut c_void>);
// apricot()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:105
// ("cv::viz::Color::apricot", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_apricot(ocvrs_return: *mut Result<*mut c_void>);
// pink()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:106
// ("cv::viz::Color::pink", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_pink(ocvrs_return: *mut Result<*mut c_void>);
// raspberry()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:107
// ("cv::viz::Color::raspberry", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_raspberry(ocvrs_return: *mut Result<*mut c_void>);
// cherry()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:108
// ("cv::viz::Color::cherry", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_cherry(ocvrs_return: *mut Result<*mut c_void>);
// violet()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:109
// ("cv::viz::Color::violet", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_violet(ocvrs_return: *mut Result<*mut c_void>);
// amethyst()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:110
// ("cv::viz::Color::amethyst", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_amethyst(ocvrs_return: *mut Result<*mut c_void>);
// bluberry()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:111
// ("cv::viz::Color::bluberry", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_bluberry(ocvrs_return: *mut Result<*mut c_void>);
// celestial_blue()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:112
// ("cv::viz::Color::celestial_blue", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_celestial_blue(ocvrs_return: *mut Result<*mut c_void>);
// turquoise()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:113
// ("cv::viz::Color::turquoise", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_turquoise(ocvrs_return: *mut Result<*mut c_void>);
// not_set()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:115
// ("cv::viz::Color::not_set", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_not_set(ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::Color::delete() generated
// ("cv::viz::Color::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Color_delete(instance: *mut c_void);
// KeyboardEvent(Action, const String &, unsigned char, int)(Enum, InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:300
// ("cv::viz::KeyboardEvent::KeyboardEvent", vec![(pred!(mut, ["action", "symbol", "code", "modifiers"], ["cv::viz::KeyboardEvent::Action", "const cv::String*", "unsigned char", "int"]), _)]),
pub fn cv_viz_KeyboardEvent_KeyboardEvent_Action_const_StringR_unsigned_char_int(action: crate::viz::KeyboardEvent_Action, symbol: *const c_char, code: u8, modifiers: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::KeyboardEvent::action() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:302
// ("cv::viz::KeyboardEvent::action", vec![(pred!(const, [], []), _)]),
pub fn cv_viz_KeyboardEvent_propAction_const(instance: *const c_void, ocvrs_return: *mut crate::viz::KeyboardEvent_Action);
// cv::viz::KeyboardEvent::setAction(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:302
// ("cv::viz::KeyboardEvent::setAction", vec![(pred!(mut, ["val"], ["const cv::viz::KeyboardEvent::Action"]), _)]),
pub fn cv_viz_KeyboardEvent_propAction_const_Action(instance: *mut c_void, val: crate::viz::KeyboardEvent_Action);
// cv::viz::KeyboardEvent::symbol() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:303
// ("cv::viz::KeyboardEvent::symbol", vec![(pred!(const, [], []), _)]),
pub fn cv_viz_KeyboardEvent_propSymbol_const(instance: *const c_void) -> *mut c_void;
// cv::viz::KeyboardEvent::setSymbol(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:303
// ("cv::viz::KeyboardEvent::setSymbol", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
pub fn cv_viz_KeyboardEvent_propSymbol_const_String(instance: *mut c_void, val: *const c_char);
// cv::viz::KeyboardEvent::code() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:304
// ("cv::viz::KeyboardEvent::code", vec![(pred!(const, [], []), _)]),
pub fn cv_viz_KeyboardEvent_propCode_const(instance: *const c_void) -> u8;
// cv::viz::KeyboardEvent::setCode(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:304
// ("cv::viz::KeyboardEvent::setCode", vec![(pred!(mut, ["val"], ["const unsigned char"]), _)]),
pub fn cv_viz_KeyboardEvent_propCode_const_unsigned_char(instance: *mut c_void, val: u8);
// cv::viz::KeyboardEvent::modifiers() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:305
// ("cv::viz::KeyboardEvent::modifiers", vec![(pred!(const, [], []), _)]),
pub fn cv_viz_KeyboardEvent_propModifiers_const(instance: *const c_void) -> i32;
// cv::viz::KeyboardEvent::setModifiers(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:305
// ("cv::viz::KeyboardEvent::setModifiers", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_viz_KeyboardEvent_propModifiers_const_int(instance: *mut c_void, val: i32);
// cv::viz::KeyboardEvent::delete() generated
// ("cv::viz::KeyboardEvent::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_KeyboardEvent_delete(instance: *mut c_void);
// Mesh()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:140
// ("cv::viz::Mesh::Mesh", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Mesh_Mesh(ocvrs_return: *mut Result<*mut c_void>);
// load(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:154
// ("cv::viz::Mesh::load", vec![(pred!(mut, ["file", "type"], ["const cv::String*", "int"]), _)]),
pub fn cv_viz_Mesh_load_const_StringR_int(file: *const c_char, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::Mesh::load(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:154
// ("cv::viz::Mesh::load", vec![(pred!(mut, ["file"], ["const cv::String*"]), _)]),
pub fn cv_viz_Mesh_load_const_StringR(file: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::Mesh::implicitClone() generated
// ("cv::viz::Mesh::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_viz_Mesh_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::viz::Mesh::cloud() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:129
// ("cv::viz::Mesh::cloud", vec![(pred!(const, [], []), _)]),
pub fn cv_viz_Mesh_propCloud_const(instance: *const c_void) -> *mut c_void;
// cv::viz::Mesh::setCloud(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:129
// ("cv::viz::Mesh::setCloud", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_viz_Mesh_propCloud_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::viz::Mesh::colors() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:130
// ("cv::viz::Mesh::colors", vec![(pred!(const, [], []), _)]),
pub fn cv_viz_Mesh_propColors_const(instance: *const c_void) -> *mut c_void;
// cv::viz::Mesh::setColors(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:130
// ("cv::viz::Mesh::setColors", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_viz_Mesh_propColors_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::viz::Mesh::normals() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:131
// ("cv::viz::Mesh::normals", vec![(pred!(const, [], []), _)]),
pub fn cv_viz_Mesh_propNormals_const(instance: *const c_void) -> *mut c_void;
// cv::viz::Mesh::setNormals(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:131
// ("cv::viz::Mesh::setNormals", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_viz_Mesh_propNormals_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::viz::Mesh::polygons() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:135
// ("cv::viz::Mesh::polygons", vec![(pred!(const, [], []), _)]),
pub fn cv_viz_Mesh_propPolygons_const(instance: *const c_void) -> *mut c_void;
// cv::viz::Mesh::setPolygons(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:135
// ("cv::viz::Mesh::setPolygons", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_viz_Mesh_propPolygons_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::viz::Mesh::texture() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:137
// ("cv::viz::Mesh::texture", vec![(pred!(const, [], []), _)]),
pub fn cv_viz_Mesh_propTexture_const(instance: *const c_void) -> *mut c_void;
// cv::viz::Mesh::setTexture(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:137
// ("cv::viz::Mesh::setTexture", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_viz_Mesh_propTexture_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::viz::Mesh::tcoords() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:138
// ("cv::viz::Mesh::tcoords", vec![(pred!(const, [], []), _)]),
pub fn cv_viz_Mesh_propTcoords_const(instance: *const c_void) -> *mut c_void;
// cv::viz::Mesh::setTcoords(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:138
// ("cv::viz::Mesh::setTcoords", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_viz_Mesh_propTcoords_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::viz::Mesh::delete() generated
// ("cv::viz::Mesh::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Mesh_delete(instance: *mut c_void);
// MouseEvent(const Type &, const MouseButton &, const Point &, int)(Enum, Enum, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:325
// ("cv::viz::MouseEvent::MouseEvent", vec![(pred!(mut, ["type", "button", "pointer", "modifiers"], ["const cv::viz::MouseEvent::Type*", "const cv::viz::MouseEvent::MouseButton*", "const cv::Point*", "int"]), _)]),
pub fn cv_viz_MouseEvent_MouseEvent_const_TypeR_const_MouseButtonR_const_PointR_int(typ: crate::viz::MouseEvent_Type, button: crate::viz::MouseEvent_MouseButton, pointer: *const core::Point, modifiers: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::MouseEvent::type() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:327
// ("cv::viz::MouseEvent::type", vec![(pred!(const, [], []), _)]),
pub fn cv_viz_MouseEvent_propType_const(instance: *const c_void, ocvrs_return: *mut crate::viz::MouseEvent_Type);
// cv::viz::MouseEvent::setType(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:327
// ("cv::viz::MouseEvent::setType", vec![(pred!(mut, ["val"], ["const cv::viz::MouseEvent::Type"]), _)]),
pub fn cv_viz_MouseEvent_propType_const_Type(instance: *mut c_void, val: crate::viz::MouseEvent_Type);
// cv::viz::MouseEvent::button() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:328
// ("cv::viz::MouseEvent::button", vec![(pred!(const, [], []), _)]),
pub fn cv_viz_MouseEvent_propButton_const(instance: *const c_void, ocvrs_return: *mut crate::viz::MouseEvent_MouseButton);
// cv::viz::MouseEvent::setButton(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:328
// ("cv::viz::MouseEvent::setButton", vec![(pred!(mut, ["val"], ["const cv::viz::MouseEvent::MouseButton"]), _)]),
pub fn cv_viz_MouseEvent_propButton_const_MouseButton(instance: *mut c_void, val: crate::viz::MouseEvent_MouseButton);
// cv::viz::MouseEvent::pointer() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:329
// ("cv::viz::MouseEvent::pointer", vec![(pred!(const, [], []), _)]),
pub fn cv_viz_MouseEvent_propPointer_const(instance: *const c_void, ocvrs_return: *mut core::Point);
// cv::viz::MouseEvent::setPointer(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:329
// ("cv::viz::MouseEvent::setPointer", vec![(pred!(mut, ["val"], ["const cv::Point"]), _)]),
pub fn cv_viz_MouseEvent_propPointer_const_Point(instance: *mut c_void, val: *const core::Point);
// cv::viz::MouseEvent::modifiers() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:330
// ("cv::viz::MouseEvent::modifiers", vec![(pred!(const, [], []), _)]),
pub fn cv_viz_MouseEvent_propModifiers_const(instance: *const c_void) -> i32;
// cv::viz::MouseEvent::setModifiers(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/types.hpp:330
// ("cv::viz::MouseEvent::setModifiers", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_viz_MouseEvent_propModifiers_const_int(instance: *mut c_void, val: i32);
// cv::viz::MouseEvent::delete() generated
// ("cv::viz::MouseEvent::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_MouseEvent_delete(instance: *mut c_void);
// Viz3d(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:78
// ("cv::viz::Viz3d::Viz3d", vec![(pred!(mut, ["window_name"], ["const cv::String*"]), _)]),
pub fn cv_viz_Viz3d_Viz3d_const_StringR(window_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::Viz3d::Viz3d() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:78
// ("cv::viz::Viz3d::Viz3d", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Viz3d_Viz3d(ocvrs_return: *mut Result<*mut c_void>);
// Viz3d(const Viz3d &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:79
// ("cv::viz::Viz3d::Viz3d", vec![(pred!(mut, ["unnamed"], ["const cv::viz::Viz3d*"]), _)]),
pub fn cv_viz_Viz3d_Viz3d_const_Viz3dR(unnamed: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(const Viz3d &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:80
// ("cv::viz::Viz3d::operator=", vec![(pred!(mut, ["unnamed"], ["const cv::viz::Viz3d*"]), _)]),
pub fn cv_viz_Viz3d_operatorST_const_Viz3dR(instance: *mut c_void, unnamed: *const c_void, ocvrs_return: *mut Result<()>);
// showWidget(const String &, const Widget &, const Affine3d &)(InString, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:88
// ("cv::viz::Viz3d::showWidget", vec![(pred!(mut, ["id", "widget", "pose"], ["const cv::String*", "const cv::viz::Widget*", "const cv::Affine3d*"]), _)]),
pub fn cv_viz_Viz3d_showWidget_const_StringR_const_WidgetR_const_Affine3dR(instance: *mut c_void, id: *const c_char, widget: *const c_void, pose: *const core::Affine3d, ocvrs_return: *mut Result<()>);
// cv::viz::Viz3d::showWidget(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:88
// ("cv::viz::Viz3d::showWidget", vec![(pred!(mut, ["id", "widget"], ["const cv::String*", "const cv::viz::Widget*"]), _)]),
pub fn cv_viz_Viz3d_showWidget_const_StringR_const_WidgetR(instance: *mut c_void, id: *const c_char, widget: *const c_void, ocvrs_return: *mut Result<()>);
// removeWidget(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:94
// ("cv::viz::Viz3d::removeWidget", vec![(pred!(mut, ["id"], ["const cv::String*"]), _)]),
pub fn cv_viz_Viz3d_removeWidget_const_StringR(instance: *mut c_void, id: *const c_char, ocvrs_return: *mut Result<()>);
// getWidget(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:103
// ("cv::viz::Viz3d::getWidget", vec![(pred!(const, ["id"], ["const cv::String*"]), _)]),
pub fn cv_viz_Viz3d_getWidget_const_const_StringR(instance: *const c_void, id: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// removeAllWidgets()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:107
// ("cv::viz::Viz3d::removeAllWidgets", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Viz3d_removeAllWidgets(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// showImage(InputArray, const Size &)(InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:114
// ("cv::viz::Viz3d::showImage", vec![(pred!(mut, ["image", "window_size"], ["const cv::_InputArray*", "const cv::Size*"]), _)]),
pub fn cv_viz_Viz3d_showImage_const__InputArrayR_const_SizeR(instance: *mut c_void, image: *const c_void, window_size: *const core::Size, ocvrs_return: *mut Result<()>);
// cv::viz::Viz3d::showImage(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:114
// ("cv::viz::Viz3d::showImage", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
pub fn cv_viz_Viz3d_showImage_const__InputArrayR(instance: *mut c_void, image: *const c_void, ocvrs_return: *mut Result<()>);
// setWidgetPose(const String &, const Affine3d &)(InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:120
// ("cv::viz::Viz3d::setWidgetPose", vec![(pred!(mut, ["id", "pose"], ["const cv::String*", "const cv::Affine3d*"]), _)]),
pub fn cv_viz_Viz3d_setWidgetPose_const_StringR_const_Affine3dR(instance: *mut c_void, id: *const c_char, pose: *const core::Affine3d, ocvrs_return: *mut Result<()>);
// updateWidgetPose(const String &, const Affine3d &)(InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:127
// ("cv::viz::Viz3d::updateWidgetPose", vec![(pred!(mut, ["id", "pose"], ["const cv::String*", "const cv::Affine3d*"]), _)]),
pub fn cv_viz_Viz3d_updateWidgetPose_const_StringR_const_Affine3dR(instance: *mut c_void, id: *const c_char, pose: *const core::Affine3d, ocvrs_return: *mut Result<()>);
// getWidgetPose(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:133
// ("cv::viz::Viz3d::getWidgetPose", vec![(pred!(const, ["id"], ["const cv::String*"]), _)]),
pub fn cv_viz_Viz3d_getWidgetPose_const_const_StringR(instance: *const c_void, id: *const c_char, ocvrs_return: *mut Result<core::Affine3d>);
// setCamera(const Camera &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:139
// ("cv::viz::Viz3d::setCamera", vec![(pred!(mut, ["camera"], ["const cv::viz::Camera*"]), _)]),
pub fn cv_viz_Viz3d_setCamera_const_CameraR(instance: *mut c_void, camera: *const c_void, ocvrs_return: *mut Result<()>);
// getCamera()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:143
// ("cv::viz::Viz3d::getCamera", vec![(pred!(const, [], []), _)]),
pub fn cv_viz_Viz3d_getCamera_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getViewerPose()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:147
// ("cv::viz::Viz3d::getViewerPose", vec![(pred!(const, [], []), _)]),
pub fn cv_viz_Viz3d_getViewerPose_const(instance: *const c_void, ocvrs_return: *mut Result<core::Affine3d>);
// setViewerPose(const Affine3d &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:153
// ("cv::viz::Viz3d::setViewerPose", vec![(pred!(mut, ["pose"], ["const cv::Affine3d*"]), _)]),
pub fn cv_viz_Viz3d_setViewerPose_const_Affine3dR(instance: *mut c_void, pose: *const core::Affine3d, ocvrs_return: *mut Result<()>);
// resetCameraViewpoint(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:159
// ("cv::viz::Viz3d::resetCameraViewpoint", vec![(pred!(mut, ["id"], ["const cv::String*"]), _)]),
pub fn cv_viz_Viz3d_resetCameraViewpoint_const_StringR(instance: *mut c_void, id: *const c_char, ocvrs_return: *mut Result<()>);
// resetCamera()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:163
// ("cv::viz::Viz3d::resetCamera", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Viz3d_resetCamera(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// convertToWindowCoordinates(const Point3d &, Point3d &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:170
// ("cv::viz::Viz3d::convertToWindowCoordinates", vec![(pred!(mut, ["pt", "window_coord"], ["const cv::Point3d*", "cv::Point3d*"]), _)]),
pub fn cv_viz_Viz3d_convertToWindowCoordinates_const_Point3dR_Point3dR(instance: *mut c_void, pt: *const core::Point3d, window_coord: *mut core::Point3d, ocvrs_return: *mut Result<()>);
// converTo3DRay(const Point3d &, Point3d &, Vec3d &)(SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:177
// ("cv::viz::Viz3d::converTo3DRay", vec![(pred!(mut, ["window_coord", "origin", "direction"], ["const cv::Point3d*", "cv::Point3d*", "cv::Vec3d*"]), _)]),
pub fn cv_viz_Viz3d_converTo3DRay_const_Point3dR_Point3dR_Vec3dR(instance: *mut c_void, window_coord: *const core::Point3d, origin: *mut core::Point3d, direction: *mut core::Vec3d, ocvrs_return: *mut Result<()>);
// getWindowSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:181
// ("cv::viz::Viz3d::getWindowSize", vec![(pred!(const, [], []), _)]),
pub fn cv_viz_Viz3d_getWindowSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
// setWindowSize(const Size &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:186
// ("cv::viz::Viz3d::setWindowSize", vec![(pred!(mut, ["window_size"], ["const cv::Size*"]), _)]),
pub fn cv_viz_Viz3d_setWindowSize_const_SizeR(instance: *mut c_void, window_size: *const core::Size, ocvrs_return: *mut Result<()>);
// getWindowName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:191
// ("cv::viz::Viz3d::getWindowName", vec![(pred!(const, [], []), _)]),
pub fn cv_viz_Viz3d_getWindowName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getScreenshot()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:195
// ("cv::viz::Viz3d::getScreenshot", vec![(pred!(const, [], []), _)]),
pub fn cv_viz_Viz3d_getScreenshot_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// saveScreenshot(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:201
// ("cv::viz::Viz3d::saveScreenshot", vec![(pred!(mut, ["file"], ["const cv::String*"]), _)]),
pub fn cv_viz_Viz3d_saveScreenshot_const_StringR(instance: *mut c_void, file: *const c_char, ocvrs_return: *mut Result<()>);
// setWindowPosition(const Point &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:207
// ("cv::viz::Viz3d::setWindowPosition", vec![(pred!(mut, ["window_position"], ["const cv::Point*"]), _)]),
pub fn cv_viz_Viz3d_setWindowPosition_const_PointR(instance: *mut c_void, window_position: *const core::Point, ocvrs_return: *mut Result<()>);
// setFullScreen(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:213
// ("cv::viz::Viz3d::setFullScreen", vec![(pred!(mut, ["mode"], ["bool"]), _)]),
pub fn cv_viz_Viz3d_setFullScreen_bool(instance: *mut c_void, mode: bool, ocvrs_return: *mut Result<()>);
// cv::viz::Viz3d::setFullScreen() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:213
// ("cv::viz::Viz3d::setFullScreen", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Viz3d_setFullScreen(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// setBackgroundColor(const Color &, const Color &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:217
// ("cv::viz::Viz3d::setBackgroundColor", vec![(pred!(mut, ["color", "color2"], ["const cv::viz::Viz3d::Color*", "const cv::viz::Viz3d::Color*"]), _)]),
pub fn cv_viz_Viz3d_setBackgroundColor_const_ColorR_const_ColorR(instance: *mut c_void, color: *const c_void, color2: *const c_void, ocvrs_return: *mut Result<()>);
// cv::viz::Viz3d::setBackgroundColor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:217
// ("cv::viz::Viz3d::setBackgroundColor", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Viz3d_setBackgroundColor(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// setBackgroundTexture(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:218
// ("cv::viz::Viz3d::setBackgroundTexture", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
pub fn cv_viz_Viz3d_setBackgroundTexture_const__InputArrayR(instance: *mut c_void, image: *const c_void, ocvrs_return: *mut Result<()>);
// cv::viz::Viz3d::setBackgroundTexture() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:218
// ("cv::viz::Viz3d::setBackgroundTexture", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Viz3d_setBackgroundTexture(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// setBackgroundMeshLab()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:219
// ("cv::viz::Viz3d::setBackgroundMeshLab", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Viz3d_setBackgroundMeshLab(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// spin()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:223
// ("cv::viz::Viz3d::spin", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Viz3d_spin(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// spinOnce(int, bool)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:230
// ("cv::viz::Viz3d::spinOnce", vec![(pred!(mut, ["time", "force_redraw"], ["int", "bool"]), _)]),
pub fn cv_viz_Viz3d_spinOnce_int_bool(instance: *mut c_void, time: i32, force_redraw: bool, ocvrs_return: *mut Result<()>);
// cv::viz::Viz3d::spinOnce() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:230
// ("cv::viz::Viz3d::spinOnce", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Viz3d_spinOnce(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// setOffScreenRendering()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:234
// ("cv::viz::Viz3d::setOffScreenRendering", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Viz3d_setOffScreenRendering(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// removeAllLights()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:238
// ("cv::viz::Viz3d::removeAllLights", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Viz3d_removeAllLights(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// addLight(const Vec3d &, const Vec3d &, const Color &, const Color &, const Color &, const Color &)(SimpleClass, SimpleClass, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:249
// ("cv::viz::Viz3d::addLight", vec![(pred!(mut, ["position", "focalPoint", "color", "diffuseColor", "ambientColor", "specularColor"], ["const cv::Vec3d*", "const cv::Vec3d*", "const cv::viz::Viz3d::Color*", "const cv::viz::Viz3d::Color*", "const cv::viz::Viz3d::Color*", "const cv::viz::Viz3d::Color*"]), _)]),
pub fn cv_viz_Viz3d_addLight_const_Vec3dR_const_Vec3dR_const_ColorR_const_ColorR_const_ColorR_const_ColorR(instance: *mut c_void, position: *const core::Vec3d, focal_point: *const core::Vec3d, color: *const c_void, diffuse_color: *const c_void, ambient_color: *const c_void, specular_color: *const c_void, ocvrs_return: *mut Result<()>);
// cv::viz::Viz3d::addLight(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:249
// ("cv::viz::Viz3d::addLight", vec![(pred!(mut, ["position"], ["const cv::Vec3d*"]), _)]),
pub fn cv_viz_Viz3d_addLight_const_Vec3dR(instance: *mut c_void, position: *const core::Vec3d, ocvrs_return: *mut Result<()>);
// wasStopped()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:255
// ("cv::viz::Viz3d::wasStopped", vec![(pred!(const, [], []), _)]),
pub fn cv_viz_Viz3d_wasStopped_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// close()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:256
// ("cv::viz::Viz3d::close", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Viz3d_close(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// registerKeyboardCallback(KeyboardCallback, void *)(Function, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:264
// ("cv::viz::Viz3d::registerKeyboardCallback", vec![(pred!(mut, ["callback", "cookie"], ["cv::viz::Viz3d::KeyboardCallback", "void*"]), _)]),
pub fn cv_viz_Viz3d_registerKeyboardCallback_KeyboardCallback_voidX(instance: *mut c_void, callback: Option<unsafe extern "C" fn(*const c_void, *mut c_void) -> ()>, cookie: *mut c_void, ocvrs_return: *mut Result<()>);
// registerMouseCallback(MouseCallback, void *)(Function, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:271
// ("cv::viz::Viz3d::registerMouseCallback", vec![(pred!(mut, ["callback", "cookie"], ["cv::viz::Viz3d::MouseCallback", "void*"]), _)]),
pub fn cv_viz_Viz3d_registerMouseCallback_MouseCallback_voidX(instance: *mut c_void, callback: Option<unsafe extern "C" fn(*const c_void, *mut c_void) -> ()>, cookie: *mut c_void, ocvrs_return: *mut Result<()>);
// setRenderingProperty(const String &, int, double)(InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:299
// ("cv::viz::Viz3d::setRenderingProperty", vec![(pred!(mut, ["id", "property", "value"], ["const cv::String*", "int", "double"]), _)]),
pub fn cv_viz_Viz3d_setRenderingProperty_const_StringR_int_double(instance: *mut c_void, id: *const c_char, property: i32, value: f64, ocvrs_return: *mut Result<()>);
// getRenderingProperty(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:325
// ("cv::viz::Viz3d::getRenderingProperty", vec![(pred!(mut, ["id", "property"], ["const cv::String*", "int"]), _)]),
pub fn cv_viz_Viz3d_getRenderingProperty_const_StringR_int(instance: *mut c_void, id: *const c_char, property: i32, ocvrs_return: *mut Result<f64>);
// setRepresentation(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:334
// ("cv::viz::Viz3d::setRepresentation", vec![(pred!(mut, ["representation"], ["int"]), _)]),
pub fn cv_viz_Viz3d_setRepresentation_int(instance: *mut c_void, representation: i32, ocvrs_return: *mut Result<()>);
// setGlobalWarnings(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:336
// ("cv::viz::Viz3d::setGlobalWarnings", vec![(pred!(mut, ["enabled"], ["bool"]), _)]),
pub fn cv_viz_Viz3d_setGlobalWarnings_bool(instance: *mut c_void, enabled: bool, ocvrs_return: *mut Result<()>);
// cv::viz::Viz3d::setGlobalWarnings() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/viz3d.hpp:336
// ("cv::viz::Viz3d::setGlobalWarnings", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Viz3d_setGlobalWarnings(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::viz::Viz3d::delete() generated
// ("cv::viz::Viz3d::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Viz3d_delete(instance: *mut c_void);
// WArrow(const Point3d &, const Point3d &, double, const Color &)(SimpleClass, SimpleClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:301
// ("cv::viz::WArrow::WArrow", vec![(pred!(mut, ["pt1", "pt2", "thickness", "color"], ["const cv::Point3d*", "const cv::Point3d*", "double", "const cv::viz::Color*"]), _)]),
pub fn cv_viz_WArrow_WArrow_const_Point3dR_const_Point3dR_double_const_ColorR(pt1: *const core::Point3d, pt2: *const core::Point3d, thickness: f64, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WArrow::WArrow(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:301
// ("cv::viz::WArrow::WArrow", vec![(pred!(mut, ["pt1", "pt2"], ["const cv::Point3d*", "const cv::Point3d*"]), _)]),
pub fn cv_viz_WArrow_WArrow_const_Point3dR_const_Point3dR(pt1: *const core::Point3d, pt2: *const core::Point3d, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WArrow::to_Widget() generated
// ("cv::viz::WArrow::to_Widget", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WArrow_to_Widget(instance: *mut c_void) -> *mut c_void;
// cv::viz::WArrow::to_Widget3D() generated
// ("cv::viz::WArrow::to_Widget3D", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WArrow_to_Widget3D(instance: *mut c_void) -> *mut c_void;
// cv::viz::WArrow::delete() generated
// ("cv::viz::WArrow::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WArrow_delete(instance: *mut c_void);
// WCameraPosition(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:550
// ("cv::viz::WCameraPosition::WCameraPosition", vec![(pred!(mut, ["scale"], ["double"]), _)]),
pub fn cv_viz_WCameraPosition_WCameraPosition_double(scale: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WCameraPosition::WCameraPosition() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:550
// ("cv::viz::WCameraPosition::WCameraPosition", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCameraPosition_WCameraPosition(ocvrs_return: *mut Result<*mut c_void>);
// WCameraPosition(const Matx33d &, double, const Color &)(SimpleClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:560
// ("cv::viz::WCameraPosition::WCameraPosition", vec![(pred!(mut, ["K", "scale", "color"], ["const cv::Matx33d*", "double", "const cv::viz::Color*"]), _)]),
pub fn cv_viz_WCameraPosition_WCameraPosition_const_Matx33dR_double_const_ColorR(k: *const core::Matx33d, scale: f64, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WCameraPosition::WCameraPosition(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:560
// ("cv::viz::WCameraPosition::WCameraPosition", vec![(pred!(mut, ["K"], ["const cv::Matx33d*"]), _)]),
pub fn cv_viz_WCameraPosition_WCameraPosition_const_Matx33dR(k: *const core::Matx33d, ocvrs_return: *mut Result<*mut c_void>);
// WCameraPosition(const Vec2d &, double, const Color &)(SimpleClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:570
// ("cv::viz::WCameraPosition::WCameraPosition", vec![(pred!(mut, ["fov", "scale", "color"], ["const cv::Vec2d*", "double", "const cv::viz::Color*"]), _)]),
pub fn cv_viz_WCameraPosition_WCameraPosition_const_Vec2dR_double_const_ColorR(fov: *const core::Vec2d, scale: f64, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WCameraPosition::WCameraPosition(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:570
// ("cv::viz::WCameraPosition::WCameraPosition", vec![(pred!(mut, ["fov"], ["const cv::Vec2d*"]), _)]),
pub fn cv_viz_WCameraPosition_WCameraPosition_const_Vec2dR(fov: *const core::Vec2d, ocvrs_return: *mut Result<*mut c_void>);
// WCameraPosition(const Matx33d &, InputArray, double, const Color &)(SimpleClass, InputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:583
// ("cv::viz::WCameraPosition::WCameraPosition", vec![(pred!(mut, ["K", "image", "scale", "color"], ["const cv::Matx33d*", "const cv::_InputArray*", "double", "const cv::viz::Color*"]), _)]),
pub fn cv_viz_WCameraPosition_WCameraPosition_const_Matx33dR_const__InputArrayR_double_const_ColorR(k: *const core::Matx33d, image: *const c_void, scale: f64, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WCameraPosition::WCameraPosition(SimpleClass, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:583
// ("cv::viz::WCameraPosition::WCameraPosition", vec![(pred!(mut, ["K", "image"], ["const cv::Matx33d*", "const cv::_InputArray*"]), _)]),
pub fn cv_viz_WCameraPosition_WCameraPosition_const_Matx33dR_const__InputArrayR(k: *const core::Matx33d, image: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// WCameraPosition(const Vec2d &, InputArray, double, const Color &)(SimpleClass, InputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:596
// ("cv::viz::WCameraPosition::WCameraPosition", vec![(pred!(mut, ["fov", "image", "scale", "color"], ["const cv::Vec2d*", "const cv::_InputArray*", "double", "const cv::viz::Color*"]), _)]),
pub fn cv_viz_WCameraPosition_WCameraPosition_const_Vec2dR_const__InputArrayR_double_const_ColorR(fov: *const core::Vec2d, image: *const c_void, scale: f64, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WCameraPosition::WCameraPosition(SimpleClass, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:596
// ("cv::viz::WCameraPosition::WCameraPosition", vec![(pred!(mut, ["fov", "image"], ["const cv::Vec2d*", "const cv::_InputArray*"]), _)]),
pub fn cv_viz_WCameraPosition_WCameraPosition_const_Vec2dR_const__InputArrayR(fov: *const core::Vec2d, image: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WCameraPosition::to_Widget() generated
// ("cv::viz::WCameraPosition::to_Widget", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCameraPosition_to_Widget(instance: *mut c_void) -> *mut c_void;
// cv::viz::WCameraPosition::to_Widget3D() generated
// ("cv::viz::WCameraPosition::to_Widget3D", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCameraPosition_to_Widget3D(instance: *mut c_void) -> *mut c_void;
// cv::viz::WCameraPosition::delete() generated
// ("cv::viz::WCameraPosition::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCameraPosition_delete(instance: *mut c_void);
// WCircle(double, double, const Color &)(Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:315
// ("cv::viz::WCircle::WCircle", vec![(pred!(mut, ["radius", "thickness", "color"], ["double", "double", "const cv::viz::Color*"]), _)]),
pub fn cv_viz_WCircle_WCircle_double_double_const_ColorR(radius: f64, thickness: f64, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WCircle::WCircle(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:315
// ("cv::viz::WCircle::WCircle", vec![(pred!(mut, ["radius"], ["double"]), _)]),
pub fn cv_viz_WCircle_WCircle_double(radius: f64, ocvrs_return: *mut Result<*mut c_void>);
// WCircle(double, const Point3d &, const Vec3d &, double, const Color &)(Primitive, SimpleClass, SimpleClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:325
// ("cv::viz::WCircle::WCircle", vec![(pred!(mut, ["radius", "center", "normal", "thickness", "color"], ["double", "const cv::Point3d*", "const cv::Vec3d*", "double", "const cv::viz::Color*"]), _)]),
pub fn cv_viz_WCircle_WCircle_double_const_Point3dR_const_Vec3dR_double_const_ColorR(radius: f64, center: *const core::Point3d, normal: *const core::Vec3d, thickness: f64, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WCircle::WCircle(Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:325
// ("cv::viz::WCircle::WCircle", vec![(pred!(mut, ["radius", "center", "normal"], ["double", "const cv::Point3d*", "const cv::Vec3d*"]), _)]),
pub fn cv_viz_WCircle_WCircle_double_const_Point3dR_const_Vec3dR(radius: f64, center: *const core::Point3d, normal: *const core::Vec3d, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WCircle::to_Widget() generated
// ("cv::viz::WCircle::to_Widget", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCircle_to_Widget(instance: *mut c_void) -> *mut c_void;
// cv::viz::WCircle::to_Widget3D() generated
// ("cv::viz::WCircle::to_Widget3D", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCircle_to_Widget3D(instance: *mut c_void) -> *mut c_void;
// cv::viz::WCircle::delete() generated
// ("cv::viz::WCircle::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCircle_delete(instance: *mut c_void);
// WCloud(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:690
// ("cv::viz::WCloud::WCloud", vec![(pred!(mut, ["cloud", "colors"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_viz_WCloud_WCloud_const__InputArrayR_const__InputArrayR(cloud: *const c_void, colors: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// WCloud(InputArray, const Color &)(InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:698
// ("cv::viz::WCloud::WCloud", vec![(pred!(mut, ["cloud", "color"], ["const cv::_InputArray*", "const cv::viz::Color*"]), _)]),
pub fn cv_viz_WCloud_WCloud_const__InputArrayR_const_ColorR(cloud: *const c_void, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WCloud::WCloud(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:698
// ("cv::viz::WCloud::WCloud", vec![(pred!(mut, ["cloud"], ["const cv::_InputArray*"]), _)]),
pub fn cv_viz_WCloud_WCloud_const__InputArrayR(cloud: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// WCloud(InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:707
// ("cv::viz::WCloud::WCloud", vec![(pred!(mut, ["cloud", "colors", "normals"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_viz_WCloud_WCloud_const__InputArrayR_const__InputArrayR_const__InputArrayR(cloud: *const c_void, colors: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// WCloud(InputArray, const Color &, InputArray)(InputArray, TraitClass, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:717
// ("cv::viz::WCloud::WCloud", vec![(pred!(mut, ["cloud", "color", "normals"], ["const cv::_InputArray*", "const cv::viz::Color*", "const cv::_InputArray*"]), _)]),
pub fn cv_viz_WCloud_WCloud_const__InputArrayR_const_ColorR_const__InputArrayR(cloud: *const c_void, color: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WCloud::to_Widget() generated
// ("cv::viz::WCloud::to_Widget", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCloud_to_Widget(instance: *mut c_void) -> *mut c_void;
// cv::viz::WCloud::to_Widget3D() generated
// ("cv::viz::WCloud::to_Widget3D", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCloud_to_Widget3D(instance: *mut c_void) -> *mut c_void;
// cv::viz::WCloud::delete() generated
// ("cv::viz::WCloud::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCloud_delete(instance: *mut c_void);
// WCloudCollection()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:739
// ("cv::viz::WCloudCollection::WCloudCollection", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCloudCollection_WCloudCollection(ocvrs_return: *mut Result<*mut c_void>);
// addCloud(InputArray, InputArray, const Affine3d &)(InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:747
// ("cv::viz::WCloudCollection::addCloud", vec![(pred!(mut, ["cloud", "colors", "pose"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Affine3d*"]), _)]),
pub fn cv_viz_WCloudCollection_addCloud_const__InputArrayR_const__InputArrayR_const_Affine3dR(instance: *mut c_void, cloud: *const c_void, colors: *const c_void, pose: *const core::Affine3d, ocvrs_return: *mut Result<()>);
// cv::viz::WCloudCollection::addCloud(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:747
// ("cv::viz::WCloudCollection::addCloud", vec![(pred!(mut, ["cloud", "colors"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_viz_WCloudCollection_addCloud_const__InputArrayR_const__InputArrayR(instance: *mut c_void, cloud: *const c_void, colors: *const c_void, ocvrs_return: *mut Result<()>);
// addCloud(InputArray, const Color &, const Affine3d &)(InputArray, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:754
// ("cv::viz::WCloudCollection::addCloud", vec![(pred!(mut, ["cloud", "color", "pose"], ["const cv::_InputArray*", "const cv::viz::Color*", "const cv::Affine3d*"]), _)]),
pub fn cv_viz_WCloudCollection_addCloud_const__InputArrayR_const_ColorR_const_Affine3dR(instance: *mut c_void, cloud: *const c_void, color: *const c_void, pose: *const core::Affine3d, ocvrs_return: *mut Result<()>);
// cv::viz::WCloudCollection::addCloud(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:754
// ("cv::viz::WCloudCollection::addCloud", vec![(pred!(mut, ["cloud"], ["const cv::_InputArray*"]), _)]),
pub fn cv_viz_WCloudCollection_addCloud_const__InputArrayR(instance: *mut c_void, cloud: *const c_void, ocvrs_return: *mut Result<()>);
// finalize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:759
// ("cv::viz::WCloudCollection::finalize", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCloudCollection_finalize(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::viz::WCloudCollection::to_Widget() generated
// ("cv::viz::WCloudCollection::to_Widget", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCloudCollection_to_Widget(instance: *mut c_void) -> *mut c_void;
// cv::viz::WCloudCollection::to_Widget3D() generated
// ("cv::viz::WCloudCollection::to_Widget3D", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCloudCollection_to_Widget3D(instance: *mut c_void) -> *mut c_void;
// cv::viz::WCloudCollection::delete() generated
// ("cv::viz::WCloudCollection::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCloudCollection_delete(instance: *mut c_void);
// WCloudNormals(InputArray, InputArray, int, double, const Color &)(InputArray, InputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:777
// ("cv::viz::WCloudNormals::WCloudNormals", vec![(pred!(mut, ["cloud", "normals", "level", "scale", "color"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "double", "const cv::viz::Color*"]), _)]),
pub fn cv_viz_WCloudNormals_WCloudNormals_const__InputArrayR_const__InputArrayR_int_double_const_ColorR(cloud: *const c_void, normals: *const c_void, level: i32, scale: f64, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WCloudNormals::WCloudNormals(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:777
// ("cv::viz::WCloudNormals::WCloudNormals", vec![(pred!(mut, ["cloud", "normals"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_viz_WCloudNormals_WCloudNormals_const__InputArrayR_const__InputArrayR(cloud: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WCloudNormals::to_Widget() generated
// ("cv::viz::WCloudNormals::to_Widget", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCloudNormals_to_Widget(instance: *mut c_void) -> *mut c_void;
// cv::viz::WCloudNormals::to_Widget3D() generated
// ("cv::viz::WCloudNormals::to_Widget3D", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCloudNormals_to_Widget3D(instance: *mut c_void) -> *mut c_void;
// cv::viz::WCloudNormals::delete() generated
// ("cv::viz::WCloudNormals::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCloudNormals_delete(instance: *mut c_void);
// WCone(double, double, int, const Color &)(Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:340
// ("cv::viz::WCone::WCone", vec![(pred!(mut, ["length", "radius", "resolution", "color"], ["double", "double", "int", "const cv::viz::Color*"]), _)]),
pub fn cv_viz_WCone_WCone_double_double_int_const_ColorR(length: f64, radius: f64, resolution: i32, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WCone::WCone(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:340
// ("cv::viz::WCone::WCone", vec![(pred!(mut, ["length", "radius"], ["double", "double"]), _)]),
pub fn cv_viz_WCone_WCone_double_double(length: f64, radius: f64, ocvrs_return: *mut Result<*mut c_void>);
// WCone(double, const Point3d &, const Point3d &, int, const Color &)(Primitive, SimpleClass, SimpleClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:351
// ("cv::viz::WCone::WCone", vec![(pred!(mut, ["radius", "center", "tip", "resolution", "color"], ["double", "const cv::Point3d*", "const cv::Point3d*", "int", "const cv::viz::Color*"]), _)]),
pub fn cv_viz_WCone_WCone_double_const_Point3dR_const_Point3dR_int_const_ColorR(radius: f64, center: *const core::Point3d, tip: *const core::Point3d, resolution: i32, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WCone::WCone(Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:351
// ("cv::viz::WCone::WCone", vec![(pred!(mut, ["radius", "center", "tip"], ["double", "const cv::Point3d*", "const cv::Point3d*"]), _)]),
pub fn cv_viz_WCone_WCone_double_const_Point3dR_const_Point3dR(radius: f64, center: *const core::Point3d, tip: *const core::Point3d, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WCone::to_Widget() generated
// ("cv::viz::WCone::to_Widget", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCone_to_Widget(instance: *mut c_void) -> *mut c_void;
// cv::viz::WCone::to_Widget3D() generated
// ("cv::viz::WCone::to_Widget3D", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCone_to_Widget3D(instance: *mut c_void) -> *mut c_void;
// cv::viz::WCone::delete() generated
// ("cv::viz::WCone::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCone_delete(instance: *mut c_void);
// WCoordinateSystem(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:520
// ("cv::viz::WCoordinateSystem::WCoordinateSystem", vec![(pred!(mut, ["scale"], ["double"]), _)]),
pub fn cv_viz_WCoordinateSystem_WCoordinateSystem_double(scale: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WCoordinateSystem::WCoordinateSystem() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:520
// ("cv::viz::WCoordinateSystem::WCoordinateSystem", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCoordinateSystem_WCoordinateSystem(ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WCoordinateSystem::to_Widget() generated
// ("cv::viz::WCoordinateSystem::to_Widget", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCoordinateSystem_to_Widget(instance: *mut c_void) -> *mut c_void;
// cv::viz::WCoordinateSystem::to_Widget3D() generated
// ("cv::viz::WCoordinateSystem::to_Widget3D", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCoordinateSystem_to_Widget3D(instance: *mut c_void) -> *mut c_void;
// cv::viz::WCoordinateSystem::delete() generated
// ("cv::viz::WCoordinateSystem::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCoordinateSystem_delete(instance: *mut c_void);
// WCube(const Point3d &, const Point3d &, bool, const Color &)(SimpleClass, SimpleClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:384
// ("cv::viz::WCube::WCube", vec![(pred!(mut, ["min_point", "max_point", "wire_frame", "color"], ["const cv::Point3d*", "const cv::Point3d*", "bool", "const cv::viz::Color*"]), _)]),
pub fn cv_viz_WCube_WCube_const_Point3dR_const_Point3dR_bool_const_ColorR(min_point: *const core::Point3d, max_point: *const core::Point3d, wire_frame: bool, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WCube::WCube() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:384
// ("cv::viz::WCube::WCube", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCube_WCube(ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WCube::to_Widget() generated
// ("cv::viz::WCube::to_Widget", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCube_to_Widget(instance: *mut c_void) -> *mut c_void;
// cv::viz::WCube::to_Widget3D() generated
// ("cv::viz::WCube::to_Widget3D", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCube_to_Widget3D(instance: *mut c_void) -> *mut c_void;
// cv::viz::WCube::delete() generated
// ("cv::viz::WCube::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCube_delete(instance: *mut c_void);
// WCylinder(const Point3d &, const Point3d &, double, int, const Color &)(SimpleClass, SimpleClass, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:367
// ("cv::viz::WCylinder::WCylinder", vec![(pred!(mut, ["axis_point1", "axis_point2", "radius", "numsides", "color"], ["const cv::Point3d*", "const cv::Point3d*", "double", "int", "const cv::viz::Color*"]), _)]),
pub fn cv_viz_WCylinder_WCylinder_const_Point3dR_const_Point3dR_double_int_const_ColorR(axis_point1: *const core::Point3d, axis_point2: *const core::Point3d, radius: f64, numsides: i32, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WCylinder::WCylinder(SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:367
// ("cv::viz::WCylinder::WCylinder", vec![(pred!(mut, ["axis_point1", "axis_point2", "radius"], ["const cv::Point3d*", "const cv::Point3d*", "double"]), _)]),
pub fn cv_viz_WCylinder_WCylinder_const_Point3dR_const_Point3dR_double(axis_point1: *const core::Point3d, axis_point2: *const core::Point3d, radius: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WCylinder::to_Widget() generated
// ("cv::viz::WCylinder::to_Widget", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCylinder_to_Widget(instance: *mut c_void) -> *mut c_void;
// cv::viz::WCylinder::to_Widget3D() generated
// ("cv::viz::WCylinder::to_Widget3D", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCylinder_to_Widget3D(instance: *mut c_void) -> *mut c_void;
// cv::viz::WCylinder::delete() generated
// ("cv::viz::WCylinder::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WCylinder_delete(instance: *mut c_void);
// WGrid(const Vec2i &, const Vec2d &, const Color &)(SimpleClass, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:534
// ("cv::viz::WGrid::WGrid", vec![(pred!(mut, ["cells", "cells_spacing", "color"], ["const cv::Vec2i*", "const cv::Vec2d*", "const cv::viz::Color*"]), _)]),
pub fn cv_viz_WGrid_WGrid_const_Vec2iR_const_Vec2dR_const_ColorR(cells: *const core::Vec2i, cells_spacing: *const core::Vec2d, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WGrid::WGrid() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:534
// ("cv::viz::WGrid::WGrid", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WGrid_WGrid(ocvrs_return: *mut Result<*mut c_void>);
// WGrid(const Point3d &, const Vec3d &, const Vec3d &, const Vec2i &, const Vec2d &, const Color &)(SimpleClass, SimpleClass, SimpleClass, SimpleClass, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:537
// ("cv::viz::WGrid::WGrid", vec![(pred!(mut, ["center", "normal", "new_yaxis", "cells", "cells_spacing", "color"], ["const cv::Point3d*", "const cv::Vec3d*", "const cv::Vec3d*", "const cv::Vec2i*", "const cv::Vec2d*", "const cv::viz::Color*"]), _)]),
pub fn cv_viz_WGrid_WGrid_const_Point3dR_const_Vec3dR_const_Vec3dR_const_Vec2iR_const_Vec2dR_const_ColorR(center: *const core::Point3d, normal: *const core::Vec3d, new_yaxis: *const core::Vec3d, cells: *const core::Vec2i, cells_spacing: *const core::Vec2d, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WGrid::WGrid(SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:537
// ("cv::viz::WGrid::WGrid", vec![(pred!(mut, ["center", "normal", "new_yaxis"], ["const cv::Point3d*", "const cv::Vec3d*", "const cv::Vec3d*"]), _)]),
pub fn cv_viz_WGrid_WGrid_const_Point3dR_const_Vec3dR_const_Vec3dR(center: *const core::Point3d, normal: *const core::Vec3d, new_yaxis: *const core::Vec3d, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WGrid::to_Widget() generated
// ("cv::viz::WGrid::to_Widget", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WGrid_to_Widget(instance: *mut c_void) -> *mut c_void;
// cv::viz::WGrid::to_Widget3D() generated
// ("cv::viz::WGrid::to_Widget3D", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WGrid_to_Widget3D(instance: *mut c_void) -> *mut c_void;
// cv::viz::WGrid::delete() generated
// ("cv::viz::WGrid::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WGrid_delete(instance: *mut c_void);
// WImage3D(InputArray, const Size2d &)(InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:483
// ("cv::viz::WImage3D::WImage3D", vec![(pred!(mut, ["image", "size"], ["const cv::_InputArray*", "const cv::Size2d*"]), _)]),
pub fn cv_viz_WImage3D_WImage3D_const__InputArrayR_const_Size2dR(image: *const c_void, size: *const core::Size2d, ocvrs_return: *mut Result<*mut c_void>);
// WImage3D(InputArray, const Size2d &, const Vec3d &, const Vec3d &, const Vec3d &)(InputArray, SimpleClass, SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:493
// ("cv::viz::WImage3D::WImage3D", vec![(pred!(mut, ["image", "size", "center", "normal", "up_vector"], ["const cv::_InputArray*", "const cv::Size2d*", "const cv::Vec3d*", "const cv::Vec3d*", "const cv::Vec3d*"]), _)]),
pub fn cv_viz_WImage3D_WImage3D_const__InputArrayR_const_Size2dR_const_Vec3dR_const_Vec3dR_const_Vec3dR(image: *const c_void, size: *const core::Size2d, center: *const core::Vec3d, normal: *const core::Vec3d, up_vector: *const core::Vec3d, ocvrs_return: *mut Result<*mut c_void>);
// setImage(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:499
// ("cv::viz::WImage3D::setImage", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
pub fn cv_viz_WImage3D_setImage_const__InputArrayR(instance: *mut c_void, image: *const c_void, ocvrs_return: *mut Result<()>);
// setSize(const Size &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:505
// ("cv::viz::WImage3D::setSize", vec![(pred!(mut, ["size"], ["const cv::Size*"]), _)]),
pub fn cv_viz_WImage3D_setSize_const_SizeR(instance: *mut c_void, size: *const core::Size, ocvrs_return: *mut Result<()>);
// cv::viz::WImage3D::to_Widget() generated
// ("cv::viz::WImage3D::to_Widget", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WImage3D_to_Widget(instance: *mut c_void) -> *mut c_void;
// cv::viz::WImage3D::to_Widget3D() generated
// ("cv::viz::WImage3D::to_Widget3D", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WImage3D_to_Widget3D(instance: *mut c_void) -> *mut c_void;
// cv::viz::WImage3D::delete() generated
// ("cv::viz::WImage3D::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WImage3D_delete(instance: *mut c_void);
// WImageOverlay(InputArray, const Rect &)(InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:465
// ("cv::viz::WImageOverlay::WImageOverlay", vec![(pred!(mut, ["image", "rect"], ["const cv::_InputArray*", "const cv::Rect*"]), _)]),
pub fn cv_viz_WImageOverlay_WImageOverlay_const__InputArrayR_const_RectR(image: *const c_void, rect: *const core::Rect, ocvrs_return: *mut Result<*mut c_void>);
// setImage(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:470
// ("cv::viz::WImageOverlay::setImage", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
pub fn cv_viz_WImageOverlay_setImage_const__InputArrayR(instance: *mut c_void, image: *const c_void, ocvrs_return: *mut Result<()>);
// cv::viz::WImageOverlay::to_Widget() generated
// ("cv::viz::WImageOverlay::to_Widget", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WImageOverlay_to_Widget(instance: *mut c_void) -> *mut c_void;
// cv::viz::WImageOverlay::to_Widget2D() generated
// ("cv::viz::WImageOverlay::to_Widget2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WImageOverlay_to_Widget2D(instance: *mut c_void) -> *mut c_void;
// cv::viz::WImageOverlay::delete() generated
// ("cv::viz::WImageOverlay::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WImageOverlay_delete(instance: *mut c_void);
// WLine(const Point3d &, const Point3d &, const Color &)(SimpleClass, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:244
// ("cv::viz::WLine::WLine", vec![(pred!(mut, ["pt1", "pt2", "color"], ["const cv::Point3d*", "const cv::Point3d*", "const cv::viz::Color*"]), _)]),
pub fn cv_viz_WLine_WLine_const_Point3dR_const_Point3dR_const_ColorR(pt1: *const core::Point3d, pt2: *const core::Point3d, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WLine::WLine(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:244
// ("cv::viz::WLine::WLine", vec![(pred!(mut, ["pt1", "pt2"], ["const cv::Point3d*", "const cv::Point3d*"]), _)]),
pub fn cv_viz_WLine_WLine_const_Point3dR_const_Point3dR(pt1: *const core::Point3d, pt2: *const core::Point3d, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WLine::to_Widget() generated
// ("cv::viz::WLine::to_Widget", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WLine_to_Widget(instance: *mut c_void) -> *mut c_void;
// cv::viz::WLine::to_Widget3D() generated
// ("cv::viz::WLine::to_Widget3D", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WLine_to_Widget3D(instance: *mut c_void) -> *mut c_void;
// cv::viz::WLine::delete() generated
// ("cv::viz::WLine::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WLine_delete(instance: *mut c_void);
// WMesh(const Mesh &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:794
// ("cv::viz::WMesh::WMesh", vec![(pred!(mut, ["mesh"], ["const cv::viz::Mesh*"]), _)]),
pub fn cv_viz_WMesh_WMesh_const_MeshR(mesh: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// WMesh(InputArray, InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:795
// ("cv::viz::WMesh::WMesh", vec![(pred!(mut, ["cloud", "polygons", "colors", "normals"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_viz_WMesh_WMesh_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(cloud: *const c_void, polygons: *const c_void, colors: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WMesh::WMesh(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:795
// ("cv::viz::WMesh::WMesh", vec![(pred!(mut, ["cloud", "polygons"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_viz_WMesh_WMesh_const__InputArrayR_const__InputArrayR(cloud: *const c_void, polygons: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WMesh::to_Widget() generated
// ("cv::viz::WMesh::to_Widget", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WMesh_to_Widget(instance: *mut c_void) -> *mut c_void;
// cv::viz::WMesh::to_Widget3D() generated
// ("cv::viz::WMesh::to_Widget3D", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WMesh_to_Widget3D(instance: *mut c_void) -> *mut c_void;
// cv::viz::WMesh::delete() generated
// ("cv::viz::WMesh::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WMesh_delete(instance: *mut c_void);
// WPaintedCloud(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:724
// ("cv::viz::WPaintedCloud::WPaintedCloud", vec![(pred!(mut, ["cloud"], ["const cv::_InputArray*"]), _)]),
pub fn cv_viz_WPaintedCloud_WPaintedCloud_const__InputArrayR(cloud: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// WPaintedCloud(InputArray, const Point3d &, const Point3d &)(InputArray, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:727
// ("cv::viz::WPaintedCloud::WPaintedCloud", vec![(pred!(mut, ["cloud", "p1", "p2"], ["const cv::_InputArray*", "const cv::Point3d*", "const cv::Point3d*"]), _)]),
pub fn cv_viz_WPaintedCloud_WPaintedCloud_const__InputArrayR_const_Point3dR_const_Point3dR(cloud: *const c_void, p1: *const core::Point3d, p2: *const core::Point3d, ocvrs_return: *mut Result<*mut c_void>);
// WPaintedCloud(InputArray, const Point3d &, const Point3d &, const Color &, const Color)(InputArray, SimpleClass, SimpleClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:730
// ("cv::viz::WPaintedCloud::WPaintedCloud", vec![(pred!(mut, ["cloud", "p1", "p2", "c1", "c2"], ["const cv::_InputArray*", "const cv::Point3d*", "const cv::Point3d*", "const cv::viz::Color*", "const cv::viz::Color"]), _)]),
pub fn cv_viz_WPaintedCloud_WPaintedCloud_const__InputArrayR_const_Point3dR_const_Point3dR_const_ColorR_const_Color(cloud: *const c_void, p1: *const core::Point3d, p2: *const core::Point3d, c1: *const c_void, c2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WPaintedCloud::to_Widget() generated
// ("cv::viz::WPaintedCloud::to_Widget", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WPaintedCloud_to_Widget(instance: *mut c_void) -> *mut c_void;
// cv::viz::WPaintedCloud::to_Widget3D() generated
// ("cv::viz::WPaintedCloud::to_Widget3D", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WPaintedCloud_to_Widget3D(instance: *mut c_void) -> *mut c_void;
// cv::viz::WPaintedCloud::delete() generated
// ("cv::viz::WPaintedCloud::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WPaintedCloud_delete(instance: *mut c_void);
// WPlane(const Size2d &, const Color &)(SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:257
// ("cv::viz::WPlane::WPlane", vec![(pred!(mut, ["size", "color"], ["const cv::Size2d*", "const cv::viz::Color*"]), _)]),
pub fn cv_viz_WPlane_WPlane_const_Size2dR_const_ColorR(size: *const core::Size2d, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WPlane::WPlane() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:257
// ("cv::viz::WPlane::WPlane", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WPlane_WPlane(ocvrs_return: *mut Result<*mut c_void>);
// WPlane(const Point3d &, const Vec3d &, const Vec3d &, const Size2d &, const Color &)(SimpleClass, SimpleClass, SimpleClass, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:267
// ("cv::viz::WPlane::WPlane", vec![(pred!(mut, ["center", "normal", "new_yaxis", "size", "color"], ["const cv::Point3d*", "const cv::Vec3d*", "const cv::Vec3d*", "const cv::Size2d*", "const cv::viz::Color*"]), _)]),
pub fn cv_viz_WPlane_WPlane_const_Point3dR_const_Vec3dR_const_Vec3dR_const_Size2dR_const_ColorR(center: *const core::Point3d, normal: *const core::Vec3d, new_yaxis: *const core::Vec3d, size: *const core::Size2d, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WPlane::WPlane(SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:267
// ("cv::viz::WPlane::WPlane", vec![(pred!(mut, ["center", "normal", "new_yaxis"], ["const cv::Point3d*", "const cv::Vec3d*", "const cv::Vec3d*"]), _)]),
pub fn cv_viz_WPlane_WPlane_const_Point3dR_const_Vec3dR_const_Vec3dR(center: *const core::Point3d, normal: *const core::Vec3d, new_yaxis: *const core::Vec3d, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WPlane::to_Widget() generated
// ("cv::viz::WPlane::to_Widget", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WPlane_to_Widget(instance: *mut c_void) -> *mut c_void;
// cv::viz::WPlane::to_Widget3D() generated
// ("cv::viz::WPlane::to_Widget3D", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WPlane_to_Widget3D(instance: *mut c_void) -> *mut c_void;
// cv::viz::WPlane::delete() generated
// ("cv::viz::WPlane::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WPlane_delete(instance: *mut c_void);
// WPolyLine(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:393
// ("cv::viz::WPolyLine::WPolyLine", vec![(pred!(mut, ["points", "colors"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_viz_WPolyLine_WPolyLine_const__InputArrayR_const__InputArrayR(points: *const c_void, colors: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// WPolyLine(InputArray, const Color &)(InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:399
// ("cv::viz::WPolyLine::WPolyLine", vec![(pred!(mut, ["points", "color"], ["const cv::_InputArray*", "const cv::viz::Color*"]), _)]),
pub fn cv_viz_WPolyLine_WPolyLine_const__InputArrayR_const_ColorR(points: *const c_void, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WPolyLine::WPolyLine(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:399
// ("cv::viz::WPolyLine::WPolyLine", vec![(pred!(mut, ["points"], ["const cv::_InputArray*"]), _)]),
pub fn cv_viz_WPolyLine_WPolyLine_const__InputArrayR(points: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WPolyLine::to_Widget() generated
// ("cv::viz::WPolyLine::to_Widget", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WPolyLine_to_Widget(instance: *mut c_void) -> *mut c_void;
// cv::viz::WPolyLine::to_Widget3D() generated
// ("cv::viz::WPolyLine::to_Widget3D", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WPolyLine_to_Widget3D(instance: *mut c_void) -> *mut c_void;
// cv::viz::WPolyLine::delete() generated
// ("cv::viz::WPolyLine::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WPolyLine_delete(instance: *mut c_void);
// WSphere(const cv::Point3d &, double, int, const Color &)(SimpleClass, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:283
// ("cv::viz::WSphere::WSphere", vec![(pred!(mut, ["center", "radius", "sphere_resolution", "color"], ["const cv::Point3d*", "double", "int", "const cv::viz::Color*"]), _)]),
pub fn cv_viz_WSphere_WSphere_const_Point3dR_double_int_const_ColorR(center: *const core::Point3d, radius: f64, sphere_resolution: i32, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WSphere::WSphere(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:283
// ("cv::viz::WSphere::WSphere", vec![(pred!(mut, ["center", "radius"], ["const cv::Point3d*", "double"]), _)]),
pub fn cv_viz_WSphere_WSphere_const_Point3dR_double(center: *const core::Point3d, radius: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WSphere::to_Widget() generated
// ("cv::viz::WSphere::to_Widget", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WSphere_to_Widget(instance: *mut c_void) -> *mut c_void;
// cv::viz::WSphere::to_Widget3D() generated
// ("cv::viz::WSphere::to_Widget3D", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WSphere_to_Widget3D(instance: *mut c_void) -> *mut c_void;
// cv::viz::WSphere::delete() generated
// ("cv::viz::WSphere::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WSphere_delete(instance: *mut c_void);
// WText(const String &, const Point &, int, const Color &)(InString, SimpleClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:417
// ("cv::viz::WText::WText", vec![(pred!(mut, ["text", "pos", "font_size", "color"], ["const cv::String*", "const cv::Point*", "int", "const cv::viz::Color*"]), _)]),
pub fn cv_viz_WText_WText_const_StringR_const_PointR_int_const_ColorR(text: *const c_char, pos: *const core::Point, font_size: i32, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WText::WText(InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:417
// ("cv::viz::WText::WText", vec![(pred!(mut, ["text", "pos"], ["const cv::String*", "const cv::Point*"]), _)]),
pub fn cv_viz_WText_WText_const_StringR_const_PointR(text: *const c_char, pos: *const core::Point, ocvrs_return: *mut Result<*mut c_void>);
// setText(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:423
// ("cv::viz::WText::setText", vec![(pred!(mut, ["text"], ["const cv::String*"]), _)]),
pub fn cv_viz_WText_setText_const_StringR(instance: *mut c_void, text: *const c_char, ocvrs_return: *mut Result<()>);
// getText()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:426
// ("cv::viz::WText::getText", vec![(pred!(const, [], []), _)]),
pub fn cv_viz_WText_getText_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WText::to_Widget() generated
// ("cv::viz::WText::to_Widget", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WText_to_Widget(instance: *mut c_void) -> *mut c_void;
// cv::viz::WText::to_Widget2D() generated
// ("cv::viz::WText::to_Widget2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WText_to_Widget2D(instance: *mut c_void) -> *mut c_void;
// cv::viz::WText::delete() generated
// ("cv::viz::WText::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WText_delete(instance: *mut c_void);
// WText3D(const String &, const Point3d &, double, bool, const Color &)(InString, SimpleClass, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:442
// ("cv::viz::WText3D::WText3D", vec![(pred!(mut, ["text", "position", "text_scale", "face_camera", "color"], ["const cv::String*", "const cv::Point3d*", "double", "bool", "const cv::viz::Color*"]), _)]),
pub fn cv_viz_WText3D_WText3D_const_StringR_const_Point3dR_double_bool_const_ColorR(text: *const c_char, position: *const core::Point3d, text_scale: f64, face_camera: bool, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WText3D::WText3D(InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:442
// ("cv::viz::WText3D::WText3D", vec![(pred!(mut, ["text", "position"], ["const cv::String*", "const cv::Point3d*"]), _)]),
pub fn cv_viz_WText3D_WText3D_const_StringR_const_Point3dR(text: *const c_char, position: *const core::Point3d, ocvrs_return: *mut Result<*mut c_void>);
// setText(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:449
// ("cv::viz::WText3D::setText", vec![(pred!(mut, ["text"], ["const cv::String*"]), _)]),
pub fn cv_viz_WText3D_setText_const_StringR(instance: *mut c_void, text: *const c_char, ocvrs_return: *mut Result<()>);
// getText()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:452
// ("cv::viz::WText3D::getText", vec![(pred!(const, [], []), _)]),
pub fn cv_viz_WText3D_getText_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WText3D::to_Widget() generated
// ("cv::viz::WText3D::to_Widget", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WText3D_to_Widget(instance: *mut c_void) -> *mut c_void;
// cv::viz::WText3D::to_Widget3D() generated
// ("cv::viz::WText3D::to_Widget3D", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WText3D_to_Widget3D(instance: *mut c_void) -> *mut c_void;
// cv::viz::WText3D::delete() generated
// ("cv::viz::WText3D::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WText3D_delete(instance: *mut c_void);
// WTrajectory(InputArray, int, double, const Color &)(InputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:622
// ("cv::viz::WTrajectory::WTrajectory", vec![(pred!(mut, ["path", "display_mode", "scale", "color"], ["const cv::_InputArray*", "int", "double", "const cv::viz::Color*"]), _)]),
pub fn cv_viz_WTrajectory_WTrajectory_const__InputArrayR_int_double_const_ColorR(path: *const c_void, display_mode: i32, scale: f64, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WTrajectory::WTrajectory(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:622
// ("cv::viz::WTrajectory::WTrajectory", vec![(pred!(mut, ["path"], ["const cv::_InputArray*"]), _)]),
pub fn cv_viz_WTrajectory_WTrajectory_const__InputArrayR(path: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WTrajectory::to_Widget() generated
// ("cv::viz::WTrajectory::to_Widget", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WTrajectory_to_Widget(instance: *mut c_void) -> *mut c_void;
// cv::viz::WTrajectory::to_Widget3D() generated
// ("cv::viz::WTrajectory::to_Widget3D", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WTrajectory_to_Widget3D(instance: *mut c_void) -> *mut c_void;
// cv::viz::WTrajectory::delete() generated
// ("cv::viz::WTrajectory::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WTrajectory_delete(instance: *mut c_void);
// WTrajectoryFrustums(InputArray, const Matx33d &, double, const Color &)(InputArray, SimpleClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:639
// ("cv::viz::WTrajectoryFrustums::WTrajectoryFrustums", vec![(pred!(mut, ["path", "K", "scale", "color"], ["const cv::_InputArray*", "const cv::Matx33d*", "double", "const cv::viz::Color*"]), _)]),
pub fn cv_viz_WTrajectoryFrustums_WTrajectoryFrustums_const__InputArrayR_const_Matx33dR_double_const_ColorR(path: *const c_void, k: *const core::Matx33d, scale: f64, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WTrajectoryFrustums::WTrajectoryFrustums(InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:639
// ("cv::viz::WTrajectoryFrustums::WTrajectoryFrustums", vec![(pred!(mut, ["path", "K"], ["const cv::_InputArray*", "const cv::Matx33d*"]), _)]),
pub fn cv_viz_WTrajectoryFrustums_WTrajectoryFrustums_const__InputArrayR_const_Matx33dR(path: *const c_void, k: *const core::Matx33d, ocvrs_return: *mut Result<*mut c_void>);
// WTrajectoryFrustums(InputArray, const Vec2d &, double, const Color &)(InputArray, SimpleClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:650
// ("cv::viz::WTrajectoryFrustums::WTrajectoryFrustums", vec![(pred!(mut, ["path", "fov", "scale", "color"], ["const cv::_InputArray*", "const cv::Vec2d*", "double", "const cv::viz::Color*"]), _)]),
pub fn cv_viz_WTrajectoryFrustums_WTrajectoryFrustums_const__InputArrayR_const_Vec2dR_double_const_ColorR(path: *const c_void, fov: *const core::Vec2d, scale: f64, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WTrajectoryFrustums::WTrajectoryFrustums(InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:650
// ("cv::viz::WTrajectoryFrustums::WTrajectoryFrustums", vec![(pred!(mut, ["path", "fov"], ["const cv::_InputArray*", "const cv::Vec2d*"]), _)]),
pub fn cv_viz_WTrajectoryFrustums_WTrajectoryFrustums_const__InputArrayR_const_Vec2dR(path: *const c_void, fov: *const core::Vec2d, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WTrajectoryFrustums::to_Widget() generated
// ("cv::viz::WTrajectoryFrustums::to_Widget", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WTrajectoryFrustums_to_Widget(instance: *mut c_void) -> *mut c_void;
// cv::viz::WTrajectoryFrustums::to_Widget3D() generated
// ("cv::viz::WTrajectoryFrustums::to_Widget3D", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WTrajectoryFrustums_to_Widget3D(instance: *mut c_void) -> *mut c_void;
// cv::viz::WTrajectoryFrustums::delete() generated
// ("cv::viz::WTrajectoryFrustums::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WTrajectoryFrustums_delete(instance: *mut c_void);
// WTrajectorySpheres(InputArray, double, double, const Color &, const Color &)(InputArray, Primitive, Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:669
// ("cv::viz::WTrajectorySpheres::WTrajectorySpheres", vec![(pred!(mut, ["path", "line_length", "radius", "from", "to"], ["const cv::_InputArray*", "double", "double", "const cv::viz::Color*", "const cv::viz::Color*"]), _)]),
pub fn cv_viz_WTrajectorySpheres_WTrajectorySpheres_const__InputArrayR_double_double_const_ColorR_const_ColorR(path: *const c_void, line_length: f64, radius: f64, from: *const c_void, to: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WTrajectorySpheres::WTrajectorySpheres(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:669
// ("cv::viz::WTrajectorySpheres::WTrajectorySpheres", vec![(pred!(mut, ["path"], ["const cv::_InputArray*"]), _)]),
pub fn cv_viz_WTrajectorySpheres_WTrajectorySpheres_const__InputArrayR(path: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::viz::WTrajectorySpheres::to_Widget() generated
// ("cv::viz::WTrajectorySpheres::to_Widget", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WTrajectorySpheres_to_Widget(instance: *mut c_void) -> *mut c_void;
// cv::viz::WTrajectorySpheres::to_Widget3D() generated
// ("cv::viz::WTrajectorySpheres::to_Widget3D", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WTrajectorySpheres_to_Widget3D(instance: *mut c_void) -> *mut c_void;
// cv::viz::WTrajectorySpheres::delete() generated
// ("cv::viz::WTrajectorySpheres::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WTrajectorySpheres_delete(instance: *mut c_void);
// WWidgetMerger()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:808
// ("cv::viz::WWidgetMerger::WWidgetMerger", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WWidgetMerger_WWidgetMerger(ocvrs_return: *mut Result<*mut c_void>);
// addWidget(const Widget3D &, const Affine3d &)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:811
// ("cv::viz::WWidgetMerger::addWidget", vec![(pred!(mut, ["widget", "pose"], ["const cv::viz::Widget3D*", "const cv::Affine3d*"]), _)]),
pub fn cv_viz_WWidgetMerger_addWidget_const_Widget3DR_const_Affine3dR(instance: *mut c_void, widget: *const c_void, pose: *const core::Affine3d, ocvrs_return: *mut Result<()>);
// cv::viz::WWidgetMerger::addWidget(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:811
// ("cv::viz::WWidgetMerger::addWidget", vec![(pred!(mut, ["widget"], ["const cv::viz::Widget3D*"]), _)]),
pub fn cv_viz_WWidgetMerger_addWidget_const_Widget3DR(instance: *mut c_void, widget: *const c_void, ocvrs_return: *mut Result<()>);
// finalize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:814
// ("cv::viz::WWidgetMerger::finalize", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WWidgetMerger_finalize(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::viz::WWidgetMerger::to_Widget() generated
// ("cv::viz::WWidgetMerger::to_Widget", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WWidgetMerger_to_Widget(instance: *mut c_void) -> *mut c_void;
// cv::viz::WWidgetMerger::to_Widget3D() generated
// ("cv::viz::WWidgetMerger::to_Widget3D", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WWidgetMerger_to_Widget3D(instance: *mut c_void) -> *mut c_void;
// cv::viz::WWidgetMerger::delete() generated
// ("cv::viz::WWidgetMerger::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_WWidgetMerger_delete(instance: *mut c_void);
// Widget()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:95
// ("cv::viz::Widget::Widget", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Widget_Widget(ocvrs_return: *mut Result<*mut c_void>);
// Widget(const Widget &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:96
// ("cv::viz::Widget::Widget", vec![(pred!(mut, ["other"], ["const cv::viz::Widget*"]), _)]),
pub fn cv_viz_Widget_Widget_const_WidgetR(other: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(const Widget &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:97
// ("cv::viz::Widget::operator=", vec![(pred!(mut, ["other"], ["const cv::viz::Widget*"]), _)]),
pub fn cv_viz_Widget_operatorST_const_WidgetR(instance: *mut c_void, other: *const c_void, ocvrs_return: *mut Result<()>);
// fromPlyFile(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:104
// ("cv::viz::Widget::fromPlyFile", vec![(pred!(mut, ["file_name"], ["const cv::String*"]), _)]),
pub fn cv_viz_Widget_fromPlyFile_const_StringR(file_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// setRenderingProperty(int, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:131
// ("cv::viz::Widget::setRenderingProperty", vec![(pred!(mut, ["property", "value"], ["int", "double"]), _)]),
pub fn cv_viz_Widget_setRenderingProperty_int_double(instance: *mut c_void, property: i32, value: f64, ocvrs_return: *mut Result<()>);
// getRenderingProperty(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:157
// ("cv::viz::Widget::getRenderingProperty", vec![(pred!(const, ["property"], ["int"]), _)]),
pub fn cv_viz_Widget_getRenderingProperty_const_int(instance: *const c_void, property: i32, ocvrs_return: *mut Result<f64>);
// cv::viz::Widget::delete() generated
// ("cv::viz::Widget::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Widget_delete(instance: *mut c_void);
// Widget2D()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:221
// ("cv::viz::Widget2D::Widget2D", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Widget2D_Widget2D(ocvrs_return: *mut Result<*mut c_void>);
// setColor(const Color &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:227
// ("cv::viz::Widget2D::setColor", vec![(pred!(mut, ["color"], ["const cv::viz::Color*"]), _)]),
pub fn cv_viz_Widget2D_setColor_const_ColorR(instance: *mut c_void, color: *const c_void, ocvrs_return: *mut Result<()>);
// cv::viz::Widget2D::to_Widget() generated
// ("cv::viz::Widget2D::to_Widget", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Widget2D_to_Widget(instance: *mut c_void) -> *mut c_void;
// cv::viz::Widget2D::delete() generated
// ("cv::viz::Widget2D::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Widget2D_delete(instance: *mut c_void);
// Widget3D()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:184
// ("cv::viz::Widget3D::Widget3D", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Widget3D_Widget3D(ocvrs_return: *mut Result<*mut c_void>);
// setPose(const Affine3d &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:190
// ("cv::viz::Widget3D::setPose", vec![(pred!(mut, ["pose"], ["const cv::Affine3d*"]), _)]),
pub fn cv_viz_Widget3D_setPose_const_Affine3dR(instance: *mut c_void, pose: *const core::Affine3d, ocvrs_return: *mut Result<()>);
// updatePose(const Affine3d &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:195
// ("cv::viz::Widget3D::updatePose", vec![(pred!(mut, ["pose"], ["const cv::Affine3d*"]), _)]),
pub fn cv_viz_Widget3D_updatePose_const_Affine3dR(instance: *mut c_void, pose: *const core::Affine3d, ocvrs_return: *mut Result<()>);
// getPose()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:198
// ("cv::viz::Widget3D::getPose", vec![(pred!(const, [], []), _)]),
pub fn cv_viz_Widget3D_getPose_const(instance: *const c_void, ocvrs_return: *mut Result<core::Affine3d>);
// applyTransform(const Affine3d &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:204
// ("cv::viz::Widget3D::applyTransform", vec![(pred!(mut, ["transform"], ["const cv::Affine3d*"]), _)]),
pub fn cv_viz_Widget3D_applyTransform_const_Affine3dR(instance: *mut c_void, transform: *const core::Affine3d, ocvrs_return: *mut Result<()>);
// setColor(const Color &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/viz/widgets.hpp:210
// ("cv::viz::Widget3D::setColor", vec![(pred!(mut, ["color"], ["const cv::viz::Color*"]), _)]),
pub fn cv_viz_Widget3D_setColor_const_ColorR(instance: *mut c_void, color: *const c_void, ocvrs_return: *mut Result<()>);
// cv::viz::Widget3D::to_Widget() generated
// ("cv::viz::Widget3D::to_Widget", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Widget3D_to_Widget(instance: *mut c_void) -> *mut c_void;
// cv::viz::Widget3D::delete() generated
// ("cv::viz::Widget3D::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_viz_Widget3D_delete(instance: *mut c_void);
