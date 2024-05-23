// makeVolume(VolumeType, float, Matx44f, float, float, int, float, Vec3i)(Enum, Primitive, SimpleClass, Primitive, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:122
// ("cv::kinfu::makeVolume", vec![(pred!(mut, ["_volumeType", "_voxelSize", "_pose", "_raycastStepFactor", "_truncDist", "_maxWeight", "_truncateThreshold", "_resolution"], ["cv::kinfu::VolumeType", "float", "cv::Matx44f", "float", "float", "int", "float", "cv::Vec3i"]), _)]),
pub fn cv_kinfu_makeVolume_VolumeType_float_Matx44f_float_float_int_float_Vec3i(_volume_type: crate::rgbd::Kinfu_VolumeType, _voxel_size: f32, _pose: *const core::Matx44f, _raycast_step_factor: f32, _trunc_dist: f32, _max_weight: i32, _truncate_threshold: f32, _resolution: *const core::Vec3i, ocvrs_return: *mut Result<*mut c_void>);
// colormap(const Mat &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:245
// ("cv::linemod::colormap", vec![(pred!(mut, ["quantized", "dst"], ["const cv::Mat*", "cv::Mat*"]), _)]),
pub fn cv_linemod_colormap_const_MatR_MatR(quantized: *const c_void, dst: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::linemod::drawFeatures(InputOutputArray, CppPassByVoidPtr, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:254
// ("cv::linemod::drawFeatures", vec![(pred!(mut, ["img", "templates", "tl"], ["const cv::_InputOutputArray*", "const std::vector<cv::linemod::Template>*", "const cv::Point2i*"]), _)]),
pub fn cv_linemod_drawFeatures_const__InputOutputArrayR_const_vectorLTemplateGR_const_Point2iR(img: *const c_void, templates: *const c_void, tl: *const core::Point2i, ocvrs_return: *mut Result<()>);
// drawFeatures(InputOutputArray, const std::vector<Template> &, const Point2i &, int)(InputOutputArray, CppPassByVoidPtr, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:254
// ("cv::linemod::drawFeatures", vec![(pred!(mut, ["img", "templates", "tl", "size"], ["const cv::_InputOutputArray*", "const std::vector<cv::linemod::Template>*", "const cv::Point2i*", "int"]), _)]),
pub fn cv_linemod_drawFeatures_const__InputOutputArrayR_const_vectorLTemplateGR_const_Point2iR_int(img: *const c_void, templates: *const c_void, tl: *const core::Point2i, size: i32, ocvrs_return: *mut Result<()>);
// getDefaultLINE()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:420
// ("cv::linemod::getDefaultLINE", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_getDefaultLINE(ocvrs_return: *mut Result<*mut c_void>);
// getDefaultLINEMOD()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:428
// ("cv::linemod::getDefaultLINEMOD", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_getDefaultLINEMOD(ocvrs_return: *mut Result<*mut c_void>);
// depthTo3dSparse(InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:299
// ("cv::rgbd::depthTo3dSparse", vec![(pred!(mut, ["depth", "in_K", "in_points", "points3d"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_rgbd_depthTo3dSparse_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(depth: *const c_void, in_k: *const c_void, in_points: *const c_void, points3d: *const c_void, ocvrs_return: *mut Result<()>);
// cv::rgbd::depthTo3d(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:312
// ("cv::rgbd::depthTo3d", vec![(pred!(mut, ["depth", "K", "points3d"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_rgbd_depthTo3d_const__InputArrayR_const__InputArrayR_const__OutputArrayR(depth: *const c_void, k: *const c_void, points3d: *const c_void, ocvrs_return: *mut Result<()>);
// depthTo3d(InputArray, InputArray, OutputArray, InputArray)(InputArray, InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:312
// ("cv::rgbd::depthTo3d", vec![(pred!(mut, ["depth", "K", "points3d", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_rgbd_depthTo3d_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(depth: *const c_void, k: *const c_void, points3d: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// isValidDepth(const double &)(Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:33
// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const double*"]), _)]),
pub fn cv_rgbd_isValidDepth_const_doubleR(depth: *const f64, ocvrs_return: *mut Result<bool>);
// isValidDepth(const float &)(Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:27
// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const float*"]), _)]),
pub fn cv_rgbd_isValidDepth_const_floatR(depth: *const f32, ocvrs_return: *mut Result<bool>);
// isValidDepth(const int &)(Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:52
// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const int*"]), _)]),
pub fn cv_rgbd_isValidDepth_const_intR(depth: *const i32, ocvrs_return: *mut Result<bool>);
// isValidDepth(const short &)(Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:39
// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const short*"]), _)]),
pub fn cv_rgbd_isValidDepth_const_shortR(depth: *const i16, ocvrs_return: *mut Result<bool>);
// isValidDepth(const unsigned int &)(Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:58
// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const unsigned int*"]), _)]),
pub fn cv_rgbd_isValidDepth_const_unsigned_intR(depth: *const u32, ocvrs_return: *mut Result<bool>);
// isValidDepth(const unsigned short &)(Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:45
// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const unsigned short*"]), _)]),
pub fn cv_rgbd_isValidDepth_const_unsigned_shortR(depth: *const u16, ocvrs_return: *mut Result<bool>);
// cv::rgbd::registerDepth(InputArray, InputArray, InputArray, InputArray, InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:287
// ("cv::rgbd::registerDepth", vec![(pred!(mut, ["unregisteredCameraMatrix", "registeredCameraMatrix", "registeredDistCoeffs", "Rt", "unregisteredDepth", "outputImagePlaneSize", "registeredDepth"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_OutputArray*"]), _)]),
pub fn cv_rgbd_registerDepth_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__OutputArrayR(unregistered_camera_matrix: *const c_void, registered_camera_matrix: *const c_void, registered_dist_coeffs: *const c_void, rt: *const c_void, unregistered_depth: *const c_void, output_image_plane_size: *const core::Size, registered_depth: *const c_void, ocvrs_return: *mut Result<()>);
// registerDepth(InputArray, InputArray, InputArray, InputArray, InputArray, const Size &, OutputArray, bool)(InputArray, InputArray, InputArray, InputArray, InputArray, SimpleClass, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:287
// ("cv::rgbd::registerDepth", vec![(pred!(mut, ["unregisteredCameraMatrix", "registeredCameraMatrix", "registeredDistCoeffs", "Rt", "unregisteredDepth", "outputImagePlaneSize", "registeredDepth", "depthDilation"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_OutputArray*", "bool"]), _)]),
pub fn cv_rgbd_registerDepth_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__OutputArrayR_bool(unregistered_camera_matrix: *const c_void, registered_camera_matrix: *const c_void, registered_dist_coeffs: *const c_void, rt: *const c_void, unregistered_depth: *const c_void, output_image_plane_size: *const core::Size, registered_depth: *const c_void, depth_dilation: bool, ocvrs_return: *mut Result<()>);
// cv::rgbd::rescaleDepth(InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:325
// ("cv::rgbd::rescaleDepth", vec![(pred!(mut, ["in", "depth", "out"], ["const cv::_InputArray*", "int", "const cv::_OutputArray*"]), _)]),
pub fn cv_rgbd_rescaleDepth_const__InputArrayR_int_const__OutputArrayR(in_: *const c_void, depth: i32, out: *const c_void, ocvrs_return: *mut Result<()>);
// rescaleDepth(InputArray, int, OutputArray, double)(InputArray, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:325
// ("cv::rgbd::rescaleDepth", vec![(pred!(mut, ["in", "depth", "out", "depth_factor"], ["const cv::_InputArray*", "int", "const cv::_OutputArray*", "double"]), _)]),
pub fn cv_rgbd_rescaleDepth_const__InputArrayR_int_const__OutputArrayR_double(in_: *const c_void, depth: i32, out: *const c_void, depth_factor: f64, ocvrs_return: *mut Result<()>);
// cv::rgbd::warpFrame(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1179
// ("cv::rgbd::warpFrame", vec![(pred!(mut, ["image", "depth", "mask", "Rt", "cameraMatrix", "distCoeff", "warpedImage"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::_OutputArray*"]), _)]),
pub fn cv_rgbd_warpFrame_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const__OutputArrayR(image: *const c_void, depth: *const c_void, mask: *const c_void, rt: *const c_void, camera_matrix: *const c_void, dist_coeff: *const c_void, warped_image: *const c_void, ocvrs_return: *mut Result<()>);
// warpFrame(const Mat &, const Mat &, const Mat &, const Mat &, const Mat &, const Mat &, OutputArray, OutputArray, OutputArray)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1179
// ("cv::rgbd::warpFrame", vec![(pred!(mut, ["image", "depth", "mask", "Rt", "cameraMatrix", "distCoeff", "warpedImage", "warpedDepth", "warpedMask"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_rgbd_warpFrame_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(image: *const c_void, depth: *const c_void, mask: *const c_void, rt: *const c_void, camera_matrix: *const c_void, dist_coeff: *const c_void, warped_image: *const c_void, warped_depth: *const c_void, warped_mask: *const c_void, ocvrs_return: *mut Result<()>);
// create(const Ptr<Params> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:198
// ("cv::colored_kinfu::ColoredKinFu::create", vec![(pred!(mut, ["_params"], ["const cv::Ptr<cv::colored_kinfu::Params>*"]), _)]),
pub fn cv_colored_kinfu_ColoredKinFu_create_const_PtrLParamsGR(_params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:202
// ("cv::colored_kinfu::ColoredKinFu::getParams", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_ColoredKinFu_getParams_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// render(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:212
// ("cv::colored_kinfu::ColoredKinFu::render", vec![(pred!(const, ["image"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_colored_kinfu_ColoredKinFu_render_const_const__OutputArrayR(instance: *const c_void, image: *const c_void, ocvrs_return: *mut Result<()>);
// render(OutputArray, const Matx44f &)(OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:224
// ("cv::colored_kinfu::ColoredKinFu::render", vec![(pred!(const, ["image", "cameraPose"], ["const cv::_OutputArray*", "const cv::Matx44f*"]), _)]),
pub fn cv_colored_kinfu_ColoredKinFu_render_const_const__OutputArrayR_const_Matx44fR(instance: *const c_void, image: *const c_void, camera_pose: *const core::Matx44f, ocvrs_return: *mut Result<()>);
// getCloud(OutputArray, OutputArray, OutputArray)(OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:235
// ("cv::colored_kinfu::ColoredKinFu::getCloud", vec![(pred!(const, ["points", "normals", "colors"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_colored_kinfu_ColoredKinFu_getCloud_const_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, colors: *const c_void, ocvrs_return: *mut Result<()>);
// cv::colored_kinfu::ColoredKinFu::getCloud(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:235
// ("cv::colored_kinfu::ColoredKinFu::getCloud", vec![(pred!(const, ["points", "normals"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_colored_kinfu_ColoredKinFu_getCloud_const_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<()>);
// getPoints(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:243
// ("cv::colored_kinfu::ColoredKinFu::getPoints", vec![(pred!(const, ["points"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_colored_kinfu_ColoredKinFu_getPoints_const_const__OutputArrayR(instance: *const c_void, points: *const c_void, ocvrs_return: *mut Result<()>);
// getNormals(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:249
// ("cv::colored_kinfu::ColoredKinFu::getNormals", vec![(pred!(const, ["points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_colored_kinfu_ColoredKinFu_getNormals_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<()>);
// reset()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:255
// ("cv::colored_kinfu::ColoredKinFu::reset", vec![(pred!(mut, [], []), _)]),
pub fn cv_colored_kinfu_ColoredKinFu_reset(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// getPose()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:258
// ("cv::colored_kinfu::ColoredKinFu::getPose", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_ColoredKinFu_getPose_const(instance: *const c_void, ocvrs_return: *mut Result<core::Affine3f>);
// update(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:266
// ("cv::colored_kinfu::ColoredKinFu::update", vec![(pred!(mut, ["depth", "rgb"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_colored_kinfu_ColoredKinFu_update_const__InputArrayR_const__InputArrayR(instance: *mut c_void, depth: *const c_void, rgb: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::colored_kinfu::ColoredKinFu::delete() generated
// ("cv::colored_kinfu::ColoredKinFu::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_colored_kinfu_ColoredKinFu_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:22
// ("cv::colored_kinfu::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_colored_kinfu_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
// Params(Matx33f, Vec3f)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:30
// ("cv::colored_kinfu::Params::Params", vec![(pred!(mut, ["volumeInitialPoseRot", "volumeInitialPoseTransl"], ["cv::Matx33f", "cv::Vec3f"]), _)]),
pub fn cv_colored_kinfu_Params_Params_Matx33f_Vec3f(volume_initial_pose_rot: *const core::Matx33f, volume_initial_pose_transl: *const core::Vec3f, ocvrs_return: *mut Result<*mut c_void>);
// Params(Matx44f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:40
// ("cv::colored_kinfu::Params::Params", vec![(pred!(mut, ["volumeInitialPose"], ["cv::Matx44f"]), _)]),
pub fn cv_colored_kinfu_Params_Params_Matx44f(volume_initial_pose: *const core::Matx44f, ocvrs_return: *mut Result<*mut c_void>);
// setInitialVolumePose(Matx33f, Vec3f)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:51
// ("cv::colored_kinfu::Params::setInitialVolumePose", vec![(pred!(mut, ["R", "t"], ["cv::Matx33f", "cv::Vec3f"]), _)]),
pub fn cv_colored_kinfu_Params_setInitialVolumePose_Matx33f_Vec3f(instance: *mut c_void, r: *const core::Matx33f, t: *const core::Vec3f, ocvrs_return: *mut Result<()>);
// setInitialVolumePose(Matx44f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:58
// ("cv::colored_kinfu::Params::setInitialVolumePose", vec![(pred!(mut, ["homogen_tf"], ["cv::Matx44f"]), _)]),
pub fn cv_colored_kinfu_Params_setInitialVolumePose_Matx44f(instance: *mut c_void, homogen_tf: *const core::Matx44f, ocvrs_return: *mut Result<()>);
// defaultParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:64
// ("cv::colored_kinfu::Params::defaultParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_colored_kinfu_Params_defaultParams(ocvrs_return: *mut Result<*mut c_void>);
// coarseParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:70
// ("cv::colored_kinfu::Params::coarseParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_colored_kinfu_Params_coarseParams(ocvrs_return: *mut Result<*mut c_void>);
// hashTSDFParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:75
// ("cv::colored_kinfu::Params::hashTSDFParams", vec![(pred!(mut, ["isCoarse"], ["bool"]), _)]),
pub fn cv_colored_kinfu_Params_hashTSDFParams_bool(is_coarse: bool, ocvrs_return: *mut Result<*mut c_void>);
// coloredTSDFParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:80
// ("cv::colored_kinfu::Params::coloredTSDFParams", vec![(pred!(mut, ["isCoarse"], ["bool"]), _)]),
pub fn cv_colored_kinfu_Params_coloredTSDFParams_bool(is_coarse: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::colored_kinfu::Params::frameSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:83
// ("cv::colored_kinfu::Params::frameSize", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propFrameSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::colored_kinfu::Params::setFrameSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:83
// ("cv::colored_kinfu::Params::setFrameSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_colored_kinfu_Params_propFrameSize_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::colored_kinfu::Params::rgb_frameSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:86
// ("cv::colored_kinfu::Params::rgb_frameSize", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propRgb_frameSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::colored_kinfu::Params::setRgb_frameSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:86
// ("cv::colored_kinfu::Params::setRgb_frameSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_colored_kinfu_Params_propRgb_frameSize_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::colored_kinfu::Params::volumeType() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:88
// ("cv::colored_kinfu::Params::volumeType", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propVolumeType_const(instance: *const c_void, ocvrs_return: *mut crate::rgbd::Kinfu_VolumeType);
// cv::colored_kinfu::Params::setVolumeType(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:88
// ("cv::colored_kinfu::Params::setVolumeType", vec![(pred!(mut, ["val"], ["const cv::kinfu::VolumeType"]), _)]),
pub fn cv_colored_kinfu_Params_propVolumeType_const_VolumeType(instance: *mut c_void, val: crate::rgbd::Kinfu_VolumeType);
// cv::colored_kinfu::Params::intr() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:91
// ("cv::colored_kinfu::Params::intr", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propIntr_const(instance: *const c_void, ocvrs_return: *mut core::Matx33f);
// cv::colored_kinfu::Params::setIntr(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:91
// ("cv::colored_kinfu::Params::setIntr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
pub fn cv_colored_kinfu_Params_propIntr_const_Matx33f(instance: *mut c_void, val: *const core::Matx33f);
// cv::colored_kinfu::Params::rgb_intr() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:94
// ("cv::colored_kinfu::Params::rgb_intr", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propRgb_intr_const(instance: *const c_void, ocvrs_return: *mut core::Matx33f);
// cv::colored_kinfu::Params::setRgb_intr(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:94
// ("cv::colored_kinfu::Params::setRgb_intr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
pub fn cv_colored_kinfu_Params_propRgb_intr_const_Matx33f(instance: *mut c_void, val: *const core::Matx33f);
// cv::colored_kinfu::Params::depthFactor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:103
// ("cv::colored_kinfu::Params::depthFactor", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propDepthFactor_const(instance: *const c_void) -> f32;
// cv::colored_kinfu::Params::setDepthFactor(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:103
// ("cv::colored_kinfu::Params::setDepthFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_colored_kinfu_Params_propDepthFactor_const_float(instance: *mut c_void, val: f32);
// cv::colored_kinfu::Params::bilateral_sigma_depth() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:106
// ("cv::colored_kinfu::Params::bilateral_sigma_depth", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propBilateral_sigma_depth_const(instance: *const c_void) -> f32;
// cv::colored_kinfu::Params::setBilateral_sigma_depth(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:106
// ("cv::colored_kinfu::Params::setBilateral_sigma_depth", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_colored_kinfu_Params_propBilateral_sigma_depth_const_float(instance: *mut c_void, val: f32);
// cv::colored_kinfu::Params::bilateral_sigma_spatial() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:108
// ("cv::colored_kinfu::Params::bilateral_sigma_spatial", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propBilateral_sigma_spatial_const(instance: *const c_void) -> f32;
// cv::colored_kinfu::Params::setBilateral_sigma_spatial(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:108
// ("cv::colored_kinfu::Params::setBilateral_sigma_spatial", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_colored_kinfu_Params_propBilateral_sigma_spatial_const_float(instance: *mut c_void, val: f32);
// cv::colored_kinfu::Params::bilateral_kernel_size() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:110
// ("cv::colored_kinfu::Params::bilateral_kernel_size", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propBilateral_kernel_size_const(instance: *const c_void) -> i32;
// cv::colored_kinfu::Params::setBilateral_kernel_size(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:110
// ("cv::colored_kinfu::Params::setBilateral_kernel_size", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_colored_kinfu_Params_propBilateral_kernel_size_const_int(instance: *mut c_void, val: i32);
// cv::colored_kinfu::Params::pyramidLevels() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:113
// ("cv::colored_kinfu::Params::pyramidLevels", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propPyramidLevels_const(instance: *const c_void) -> i32;
// cv::colored_kinfu::Params::setPyramidLevels(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:113
// ("cv::colored_kinfu::Params::setPyramidLevels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_colored_kinfu_Params_propPyramidLevels_const_int(instance: *mut c_void, val: i32);
// cv::colored_kinfu::Params::volumeDims() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:119
// ("cv::colored_kinfu::Params::volumeDims", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propVolumeDims_const(instance: *const c_void, ocvrs_return: *mut core::Vec3i);
// cv::colored_kinfu::Params::setVolumeDims(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:119
// ("cv::colored_kinfu::Params::setVolumeDims", vec![(pred!(mut, ["val"], ["const cv::Vec3i"]), _)]),
pub fn cv_colored_kinfu_Params_propVolumeDims_const_Vec3i(instance: *mut c_void, val: *const core::Vec3i);
// cv::colored_kinfu::Params::voxelSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:121
// ("cv::colored_kinfu::Params::voxelSize", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propVoxelSize_const(instance: *const c_void) -> f32;
// cv::colored_kinfu::Params::setVoxelSize(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:121
// ("cv::colored_kinfu::Params::setVoxelSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_colored_kinfu_Params_propVoxelSize_const_float(instance: *mut c_void, val: f32);
// cv::colored_kinfu::Params::tsdf_min_camera_movement() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:127
// ("cv::colored_kinfu::Params::tsdf_min_camera_movement", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propTsdf_min_camera_movement_const(instance: *const c_void) -> f32;
// cv::colored_kinfu::Params::setTsdf_min_camera_movement(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:127
// ("cv::colored_kinfu::Params::setTsdf_min_camera_movement", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_colored_kinfu_Params_propTsdf_min_camera_movement_const_float(instance: *mut c_void, val: f32);
// cv::colored_kinfu::Params::volumePose() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:130
// ("cv::colored_kinfu::Params::volumePose", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propVolumePose_const(instance: *const c_void, ocvrs_return: *mut core::Affine3f);
// cv::colored_kinfu::Params::setVolumePose(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:130
// ("cv::colored_kinfu::Params::setVolumePose", vec![(pred!(mut, ["val"], ["const cv::Affine3f"]), _)]),
pub fn cv_colored_kinfu_Params_propVolumePose_const_Affine3f(instance: *mut c_void, val: *const core::Affine3f);
// cv::colored_kinfu::Params::tsdf_trunc_dist() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:136
// ("cv::colored_kinfu::Params::tsdf_trunc_dist", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propTsdf_trunc_dist_const(instance: *const c_void) -> f32;
// cv::colored_kinfu::Params::setTsdf_trunc_dist(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:136
// ("cv::colored_kinfu::Params::setTsdf_trunc_dist", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_colored_kinfu_Params_propTsdf_trunc_dist_const_float(instance: *mut c_void, val: f32);
// cv::colored_kinfu::Params::tsdf_max_weight() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:142
// ("cv::colored_kinfu::Params::tsdf_max_weight", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propTsdf_max_weight_const(instance: *const c_void) -> i32;
// cv::colored_kinfu::Params::setTsdf_max_weight(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:142
// ("cv::colored_kinfu::Params::setTsdf_max_weight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_colored_kinfu_Params_propTsdf_max_weight_const_int(instance: *mut c_void, val: i32);
// cv::colored_kinfu::Params::raycast_step_factor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:148
// ("cv::colored_kinfu::Params::raycast_step_factor", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propRaycast_step_factor_const(instance: *const c_void) -> f32;
// cv::colored_kinfu::Params::setRaycast_step_factor(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:148
// ("cv::colored_kinfu::Params::setRaycast_step_factor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_colored_kinfu_Params_propRaycast_step_factor_const_float(instance: *mut c_void, val: f32);
// cv::colored_kinfu::Params::lightPose() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:155
// ("cv::colored_kinfu::Params::lightPose", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propLightPose_const(instance: *const c_void, ocvrs_return: *mut core::Vec3f);
// cv::colored_kinfu::Params::setLightPose(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:155
// ("cv::colored_kinfu::Params::setLightPose", vec![(pred!(mut, ["val"], ["const cv::Vec3f"]), _)]),
pub fn cv_colored_kinfu_Params_propLightPose_const_Vec3f(instance: *mut c_void, val: *const core::Vec3f);
// cv::colored_kinfu::Params::icpDistThresh() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:158
// ("cv::colored_kinfu::Params::icpDistThresh", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propIcpDistThresh_const(instance: *const c_void) -> f32;
// cv::colored_kinfu::Params::setIcpDistThresh(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:158
// ("cv::colored_kinfu::Params::setIcpDistThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_colored_kinfu_Params_propIcpDistThresh_const_float(instance: *mut c_void, val: f32);
// cv::colored_kinfu::Params::icpAngleThresh() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:160
// ("cv::colored_kinfu::Params::icpAngleThresh", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propIcpAngleThresh_const(instance: *const c_void) -> f32;
// cv::colored_kinfu::Params::setIcpAngleThresh(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:160
// ("cv::colored_kinfu::Params::setIcpAngleThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_colored_kinfu_Params_propIcpAngleThresh_const_float(instance: *mut c_void, val: f32);
// cv::colored_kinfu::Params::icpIterations() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:162
// ("cv::colored_kinfu::Params::icpIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propIcpIterations_const(instance: *const c_void) -> *mut c_void;
// cv::colored_kinfu::Params::setIcpIterations(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:162
// ("cv::colored_kinfu::Params::setIcpIterations", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
pub fn cv_colored_kinfu_Params_propIcpIterations_const_vectorLintG(instance: *mut c_void, val: *const c_void);
// cv::colored_kinfu::Params::truncateThreshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:168
// ("cv::colored_kinfu::Params::truncateThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propTruncateThreshold_const(instance: *const c_void) -> f32;
// cv::colored_kinfu::Params::setTruncateThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:168
// ("cv::colored_kinfu::Params::setTruncateThreshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_colored_kinfu_Params_propTruncateThreshold_const_float(instance: *mut c_void, val: f32);
// cv::colored_kinfu::Params::delete() generated
// ("cv::colored_kinfu::Params::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_colored_kinfu_Params_delete(instance: *mut c_void);
// create(const Ptr<kinfu::Params> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:49
// ("cv::dynafu::DynaFu::create", vec![(pred!(mut, ["_params"], ["const cv::Ptr<cv::kinfu::Params>*"]), _)]),
pub fn cv_dynafu_DynaFu_create_const_PtrLParamsGR(_params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:53
// ("cv::dynafu::DynaFu::getParams", vec![(pred!(const, [], []), _)]),
pub fn cv_dynafu_DynaFu_getParams_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// render(OutputArray, const Matx44f &)(OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:65
// ("cv::dynafu::DynaFu::render", vec![(pred!(const, ["image", "cameraPose"], ["const cv::_OutputArray*", "const cv::Matx44f*"]), _)]),
pub fn cv_dynafu_DynaFu_render_const_const__OutputArrayR_const_Matx44fR(instance: *const c_void, image: *const c_void, camera_pose: *const core::Matx44f, ocvrs_return: *mut Result<()>);
// cv::dynafu::DynaFu::render(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:65
// ("cv::dynafu::DynaFu::render", vec![(pred!(const, ["image"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_dynafu_DynaFu_render_const_const__OutputArrayR(instance: *const c_void, image: *const c_void, ocvrs_return: *mut Result<()>);
// getCloud(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:75
// ("cv::dynafu::DynaFu::getCloud", vec![(pred!(const, ["points", "normals"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_dynafu_DynaFu_getCloud_const_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<()>);
// getPoints(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:83
// ("cv::dynafu::DynaFu::getPoints", vec![(pred!(const, ["points"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_dynafu_DynaFu_getPoints_const_const__OutputArrayR(instance: *const c_void, points: *const c_void, ocvrs_return: *mut Result<()>);
// getNormals(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:89
// ("cv::dynafu::DynaFu::getNormals", vec![(pred!(const, ["points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_dynafu_DynaFu_getNormals_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<()>);
// reset()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:95
// ("cv::dynafu::DynaFu::reset", vec![(pred!(mut, [], []), _)]),
pub fn cv_dynafu_DynaFu_reset(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// getPose()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:98
// ("cv::dynafu::DynaFu::getPose", vec![(pred!(const, [], []), _)]),
pub fn cv_dynafu_DynaFu_getPose_const(instance: *const c_void, ocvrs_return: *mut Result<core::Affine3f>);
// update(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:108
// ("cv::dynafu::DynaFu::update", vec![(pred!(mut, ["depth"], ["const cv::_InputArray*"]), _)]),
pub fn cv_dynafu_DynaFu_update_const__InputArrayR(instance: *mut c_void, depth: *const c_void, ocvrs_return: *mut Result<bool>);
// getNodesPos()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:110
// ("cv::dynafu::DynaFu::getNodesPos", vec![(pred!(const, [], []), _)]),
pub fn cv_dynafu_DynaFu_getNodesPos_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// marchCubes(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:112
// ("cv::dynafu::DynaFu::marchCubes", vec![(pred!(const, ["vertices", "edges"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_dynafu_DynaFu_marchCubes_const_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, vertices: *const c_void, edges: *const c_void, ocvrs_return: *mut Result<()>);
// renderSurface(OutputArray, OutputArray, OutputArray, bool)(OutputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:114
// ("cv::dynafu::DynaFu::renderSurface", vec![(pred!(mut, ["depthImage", "vertImage", "normImage", "warp"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool"]), _)]),
pub fn cv_dynafu_DynaFu_renderSurface_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_bool(instance: *mut c_void, depth_image: *const c_void, vert_image: *const c_void, norm_image: *const c_void, warp: bool, ocvrs_return: *mut Result<()>);
// cv::dynafu::DynaFu::renderSurface(OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:114
// ("cv::dynafu::DynaFu::renderSurface", vec![(pred!(mut, ["depthImage", "vertImage", "normImage"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_dynafu_DynaFu_renderSurface_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, depth_image: *const c_void, vert_image: *const c_void, norm_image: *const c_void, ocvrs_return: *mut Result<()>);
// cv::dynafu::DynaFu::delete() generated
// ("cv::dynafu::DynaFu::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dynafu_DynaFu_delete(instance: *mut c_void);
// Intr()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:61
// ("cv::kinfu::Intr::Intr", vec![(pred!(mut, [], []), _)]),
pub fn cv_kinfu_Intr_Intr(ocvrs_return: *mut Result<crate::rgbd::Kinfu_Intr>);
// Intr(float, float, float, float)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:62
// ("cv::kinfu::Intr::Intr", vec![(pred!(mut, ["_fx", "_fy", "_cx", "_cy"], ["float", "float", "float", "float"]), _)]),
pub fn cv_kinfu_Intr_Intr_float_float_float_float(_fx: f32, _fy: f32, _cx: f32, _cy: f32, ocvrs_return: *mut Result<crate::rgbd::Kinfu_Intr>);
// Intr(cv::Matx33f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:63
// ("cv::kinfu::Intr::Intr", vec![(pred!(mut, ["m"], ["cv::Matx33f"]), _)]),
pub fn cv_kinfu_Intr_Intr_Matx33f(m: *const core::Matx33f, ocvrs_return: *mut Result<crate::rgbd::Kinfu_Intr>);
// scale(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:65
// ("cv::kinfu::Intr::scale", vec![(pred!(const, ["pyr"], ["int"]), _)]),
pub fn cv_kinfu_Intr_scale_const_int(instance: *const crate::rgbd::Kinfu_Intr, pyr: i32, ocvrs_return: *mut Result<crate::rgbd::Kinfu_Intr>);
// makeReprojector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:70
// ("cv::kinfu::Intr::makeReprojector", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Intr_makeReprojector_const(instance: *const crate::rgbd::Kinfu_Intr, ocvrs_return: *mut Result<crate::rgbd::Kinfu_Intr_Reprojector>);
// makeProjector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:71
// ("cv::kinfu::Intr::makeProjector", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Intr_makeProjector_const(instance: *const crate::rgbd::Kinfu_Intr, ocvrs_return: *mut Result<crate::rgbd::Kinfu_Intr_Projector>);
// getMat()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:73
// ("cv::kinfu::Intr::getMat", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Intr_getMat_const(instance: *const crate::rgbd::Kinfu_Intr, ocvrs_return: *mut Result<core::Matx33f>);
// Projector(Intr)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:41
// ("cv::kinfu::Intr::Projector::Projector", vec![(pred!(mut, ["intr"], ["cv::kinfu::Intr"]), _)]),
pub fn cv_kinfu_Intr_Projector_Projector_Intr(intr: *const crate::rgbd::Kinfu_Intr, ocvrs_return: *mut Result<crate::rgbd::Kinfu_Intr_Projector>);
// Reprojector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:21
// ("cv::kinfu::Intr::Reprojector::Reprojector", vec![(pred!(mut, [], []), _)]),
pub fn cv_kinfu_Intr_Reprojector_Reprojector(ocvrs_return: *mut Result<crate::rgbd::Kinfu_Intr_Reprojector>);
// Reprojector(Intr)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:22
// ("cv::kinfu::Intr::Reprojector::Reprojector", vec![(pred!(mut, ["intr"], ["cv::kinfu::Intr"]), _)]),
pub fn cv_kinfu_Intr_Reprojector_Reprojector_Intr(intr: *const crate::rgbd::Kinfu_Intr, ocvrs_return: *mut Result<crate::rgbd::Kinfu_Intr_Reprojector>);
// create(const Ptr<Params> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:195
// ("cv::kinfu::KinFu::create", vec![(pred!(mut, ["_params"], ["const cv::Ptr<cv::kinfu::Params>*"]), _)]),
pub fn cv_kinfu_KinFu_create_const_PtrLParamsGR(_params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:199
// ("cv::kinfu::KinFu::getParams", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_KinFu_getParams_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// render(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:209
// ("cv::kinfu::KinFu::render", vec![(pred!(const, ["image"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_kinfu_KinFu_render_const_const__OutputArrayR(instance: *const c_void, image: *const c_void, ocvrs_return: *mut Result<()>);
// render(OutputArray, const Matx44f &)(OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:221
// ("cv::kinfu::KinFu::render", vec![(pred!(const, ["image", "cameraPose"], ["const cv::_OutputArray*", "const cv::Matx44f*"]), _)]),
pub fn cv_kinfu_KinFu_render_const_const__OutputArrayR_const_Matx44fR(instance: *const c_void, image: *const c_void, camera_pose: *const core::Matx44f, ocvrs_return: *mut Result<()>);
// getCloud(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:231
// ("cv::kinfu::KinFu::getCloud", vec![(pred!(const, ["points", "normals"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_kinfu_KinFu_getCloud_const_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<()>);
// getPoints(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:239
// ("cv::kinfu::KinFu::getPoints", vec![(pred!(const, ["points"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_kinfu_KinFu_getPoints_const_const__OutputArrayR(instance: *const c_void, points: *const c_void, ocvrs_return: *mut Result<()>);
// getNormals(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:245
// ("cv::kinfu::KinFu::getNormals", vec![(pred!(const, ["points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_kinfu_KinFu_getNormals_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<()>);
// reset()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:251
// ("cv::kinfu::KinFu::reset", vec![(pred!(mut, [], []), _)]),
pub fn cv_kinfu_KinFu_reset(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// getPose()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:254
// ("cv::kinfu::KinFu::getPose", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_KinFu_getPose_const(instance: *const c_void, ocvrs_return: *mut Result<core::Affine3f>);
// update(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:264
// ("cv::kinfu::KinFu::update", vec![(pred!(mut, ["depth"], ["const cv::_InputArray*"]), _)]),
pub fn cv_kinfu_KinFu_update_const__InputArrayR(instance: *mut c_void, depth: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::kinfu::KinFu::delete() generated
// ("cv::kinfu::KinFu::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_kinfu_KinFu_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:22
// ("cv::kinfu::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_kinfu_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
// Params(Matx33f, Vec3f)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:30
// ("cv::kinfu::Params::Params", vec![(pred!(mut, ["volumeInitialPoseRot", "volumeInitialPoseTransl"], ["cv::Matx33f", "cv::Vec3f"]), _)]),
pub fn cv_kinfu_Params_Params_Matx33f_Vec3f(volume_initial_pose_rot: *const core::Matx33f, volume_initial_pose_transl: *const core::Vec3f, ocvrs_return: *mut Result<*mut c_void>);
// Params(Matx44f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:40
// ("cv::kinfu::Params::Params", vec![(pred!(mut, ["volumeInitialPose"], ["cv::Matx44f"]), _)]),
pub fn cv_kinfu_Params_Params_Matx44f(volume_initial_pose: *const core::Matx44f, ocvrs_return: *mut Result<*mut c_void>);
// setInitialVolumePose(Matx33f, Vec3f)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:51
// ("cv::kinfu::Params::setInitialVolumePose", vec![(pred!(mut, ["R", "t"], ["cv::Matx33f", "cv::Vec3f"]), _)]),
pub fn cv_kinfu_Params_setInitialVolumePose_Matx33f_Vec3f(instance: *mut c_void, r: *const core::Matx33f, t: *const core::Vec3f, ocvrs_return: *mut Result<()>);
// setInitialVolumePose(Matx44f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:58
// ("cv::kinfu::Params::setInitialVolumePose", vec![(pred!(mut, ["homogen_tf"], ["cv::Matx44f"]), _)]),
pub fn cv_kinfu_Params_setInitialVolumePose_Matx44f(instance: *mut c_void, homogen_tf: *const core::Matx44f, ocvrs_return: *mut Result<()>);
// defaultParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:64
// ("cv::kinfu::Params::defaultParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_kinfu_Params_defaultParams(ocvrs_return: *mut Result<*mut c_void>);
// coarseParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:70
// ("cv::kinfu::Params::coarseParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_kinfu_Params_coarseParams(ocvrs_return: *mut Result<*mut c_void>);
// hashTSDFParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:75
// ("cv::kinfu::Params::hashTSDFParams", vec![(pred!(mut, ["isCoarse"], ["bool"]), _)]),
pub fn cv_kinfu_Params_hashTSDFParams_bool(is_coarse: bool, ocvrs_return: *mut Result<*mut c_void>);
// coloredTSDFParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:80
// ("cv::kinfu::Params::coloredTSDFParams", vec![(pred!(mut, ["isCoarse"], ["bool"]), _)]),
pub fn cv_kinfu_Params_coloredTSDFParams_bool(is_coarse: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::kinfu::Params::frameSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:83
// ("cv::kinfu::Params::frameSize", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propFrameSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::kinfu::Params::setFrameSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:83
// ("cv::kinfu::Params::setFrameSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_kinfu_Params_propFrameSize_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::kinfu::Params::volumeType() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:86
// ("cv::kinfu::Params::volumeType", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propVolumeType_const(instance: *const c_void, ocvrs_return: *mut crate::rgbd::Kinfu_VolumeType);
// cv::kinfu::Params::setVolumeType(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:86
// ("cv::kinfu::Params::setVolumeType", vec![(pred!(mut, ["val"], ["const cv::kinfu::VolumeType"]), _)]),
pub fn cv_kinfu_Params_propVolumeType_const_VolumeType(instance: *mut c_void, val: crate::rgbd::Kinfu_VolumeType);
// cv::kinfu::Params::intr() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:89
// ("cv::kinfu::Params::intr", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propIntr_const(instance: *const c_void, ocvrs_return: *mut core::Matx33f);
// cv::kinfu::Params::setIntr(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:89
// ("cv::kinfu::Params::setIntr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
pub fn cv_kinfu_Params_propIntr_const_Matx33f(instance: *mut c_void, val: *const core::Matx33f);
// cv::kinfu::Params::rgb_intr() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:92
// ("cv::kinfu::Params::rgb_intr", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propRgb_intr_const(instance: *const c_void, ocvrs_return: *mut core::Matx33f);
// cv::kinfu::Params::setRgb_intr(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:92
// ("cv::kinfu::Params::setRgb_intr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
pub fn cv_kinfu_Params_propRgb_intr_const_Matx33f(instance: *mut c_void, val: *const core::Matx33f);
// cv::kinfu::Params::depthFactor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:100
// ("cv::kinfu::Params::depthFactor", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propDepthFactor_const(instance: *const c_void) -> f32;
// cv::kinfu::Params::setDepthFactor(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:100
// ("cv::kinfu::Params::setDepthFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_kinfu_Params_propDepthFactor_const_float(instance: *mut c_void, val: f32);
// cv::kinfu::Params::bilateral_sigma_depth() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:103
// ("cv::kinfu::Params::bilateral_sigma_depth", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propBilateral_sigma_depth_const(instance: *const c_void) -> f32;
// cv::kinfu::Params::setBilateral_sigma_depth(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:103
// ("cv::kinfu::Params::setBilateral_sigma_depth", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_kinfu_Params_propBilateral_sigma_depth_const_float(instance: *mut c_void, val: f32);
// cv::kinfu::Params::bilateral_sigma_spatial() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:105
// ("cv::kinfu::Params::bilateral_sigma_spatial", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propBilateral_sigma_spatial_const(instance: *const c_void) -> f32;
// cv::kinfu::Params::setBilateral_sigma_spatial(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:105
// ("cv::kinfu::Params::setBilateral_sigma_spatial", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_kinfu_Params_propBilateral_sigma_spatial_const_float(instance: *mut c_void, val: f32);
// cv::kinfu::Params::bilateral_kernel_size() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:107
// ("cv::kinfu::Params::bilateral_kernel_size", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propBilateral_kernel_size_const(instance: *const c_void) -> i32;
// cv::kinfu::Params::setBilateral_kernel_size(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:107
// ("cv::kinfu::Params::setBilateral_kernel_size", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_kinfu_Params_propBilateral_kernel_size_const_int(instance: *mut c_void, val: i32);
// cv::kinfu::Params::pyramidLevels() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:110
// ("cv::kinfu::Params::pyramidLevels", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propPyramidLevels_const(instance: *const c_void) -> i32;
// cv::kinfu::Params::setPyramidLevels(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:110
// ("cv::kinfu::Params::setPyramidLevels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_kinfu_Params_propPyramidLevels_const_int(instance: *mut c_void, val: i32);
// cv::kinfu::Params::volumeDims() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:116
// ("cv::kinfu::Params::volumeDims", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propVolumeDims_const(instance: *const c_void, ocvrs_return: *mut core::Vec3i);
// cv::kinfu::Params::setVolumeDims(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:116
// ("cv::kinfu::Params::setVolumeDims", vec![(pred!(mut, ["val"], ["const cv::Vec3i"]), _)]),
pub fn cv_kinfu_Params_propVolumeDims_const_Vec3i(instance: *mut c_void, val: *const core::Vec3i);
// cv::kinfu::Params::voxelSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:118
// ("cv::kinfu::Params::voxelSize", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propVoxelSize_const(instance: *const c_void) -> f32;
// cv::kinfu::Params::setVoxelSize(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:118
// ("cv::kinfu::Params::setVoxelSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_kinfu_Params_propVoxelSize_const_float(instance: *mut c_void, val: f32);
// cv::kinfu::Params::tsdf_min_camera_movement() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:124
// ("cv::kinfu::Params::tsdf_min_camera_movement", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propTsdf_min_camera_movement_const(instance: *const c_void) -> f32;
// cv::kinfu::Params::setTsdf_min_camera_movement(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:124
// ("cv::kinfu::Params::setTsdf_min_camera_movement", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_kinfu_Params_propTsdf_min_camera_movement_const_float(instance: *mut c_void, val: f32);
// cv::kinfu::Params::volumePose() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:127
// ("cv::kinfu::Params::volumePose", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propVolumePose_const(instance: *const c_void, ocvrs_return: *mut core::Affine3f);
// cv::kinfu::Params::setVolumePose(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:127
// ("cv::kinfu::Params::setVolumePose", vec![(pred!(mut, ["val"], ["const cv::Affine3f"]), _)]),
pub fn cv_kinfu_Params_propVolumePose_const_Affine3f(instance: *mut c_void, val: *const core::Affine3f);
// cv::kinfu::Params::tsdf_trunc_dist() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:133
// ("cv::kinfu::Params::tsdf_trunc_dist", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propTsdf_trunc_dist_const(instance: *const c_void) -> f32;
// cv::kinfu::Params::setTsdf_trunc_dist(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:133
// ("cv::kinfu::Params::setTsdf_trunc_dist", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_kinfu_Params_propTsdf_trunc_dist_const_float(instance: *mut c_void, val: f32);
// cv::kinfu::Params::tsdf_max_weight() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:139
// ("cv::kinfu::Params::tsdf_max_weight", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propTsdf_max_weight_const(instance: *const c_void) -> i32;
// cv::kinfu::Params::setTsdf_max_weight(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:139
// ("cv::kinfu::Params::setTsdf_max_weight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_kinfu_Params_propTsdf_max_weight_const_int(instance: *mut c_void, val: i32);
// cv::kinfu::Params::raycast_step_factor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:145
// ("cv::kinfu::Params::raycast_step_factor", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propRaycast_step_factor_const(instance: *const c_void) -> f32;
// cv::kinfu::Params::setRaycast_step_factor(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:145
// ("cv::kinfu::Params::setRaycast_step_factor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_kinfu_Params_propRaycast_step_factor_const_float(instance: *mut c_void, val: f32);
// cv::kinfu::Params::lightPose() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:152
// ("cv::kinfu::Params::lightPose", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propLightPose_const(instance: *const c_void, ocvrs_return: *mut core::Vec3f);
// cv::kinfu::Params::setLightPose(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:152
// ("cv::kinfu::Params::setLightPose", vec![(pred!(mut, ["val"], ["const cv::Vec3f"]), _)]),
pub fn cv_kinfu_Params_propLightPose_const_Vec3f(instance: *mut c_void, val: *const core::Vec3f);
// cv::kinfu::Params::icpDistThresh() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:155
// ("cv::kinfu::Params::icpDistThresh", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propIcpDistThresh_const(instance: *const c_void) -> f32;
// cv::kinfu::Params::setIcpDistThresh(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:155
// ("cv::kinfu::Params::setIcpDistThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_kinfu_Params_propIcpDistThresh_const_float(instance: *mut c_void, val: f32);
// cv::kinfu::Params::icpAngleThresh() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:157
// ("cv::kinfu::Params::icpAngleThresh", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propIcpAngleThresh_const(instance: *const c_void) -> f32;
// cv::kinfu::Params::setIcpAngleThresh(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:157
// ("cv::kinfu::Params::setIcpAngleThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_kinfu_Params_propIcpAngleThresh_const_float(instance: *mut c_void, val: f32);
// cv::kinfu::Params::icpIterations() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:159
// ("cv::kinfu::Params::icpIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propIcpIterations_const(instance: *const c_void) -> *mut c_void;
// cv::kinfu::Params::setIcpIterations(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:159
// ("cv::kinfu::Params::setIcpIterations", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
pub fn cv_kinfu_Params_propIcpIterations_const_vectorLintG(instance: *mut c_void, val: *const c_void);
// cv::kinfu::Params::truncateThreshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:165
// ("cv::kinfu::Params::truncateThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propTruncateThreshold_const(instance: *const c_void) -> f32;
// cv::kinfu::Params::setTruncateThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:165
// ("cv::kinfu::Params::setTruncateThreshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_kinfu_Params_propTruncateThreshold_const_float(instance: *mut c_void, val: f32);
// cv::kinfu::Params::delete() generated
// ("cv::kinfu::Params::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_kinfu_Params_delete(instance: *mut c_void);
// integrate(InputArray, float, const Matx44f &, const kinfu::Intr &, const int)(InputArray, Primitive, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:31
// ("cv::kinfu::Volume::integrate", vec![(pred!(mut, ["_depth", "depthFactor", "cameraPose", "intrinsics", "frameId"], ["const cv::_InputArray*", "float", "const cv::Matx44f*", "const cv::kinfu::Intr*", "const int"]), _)]),
pub fn cv_kinfu_Volume_integrate_const__InputArrayR_float_const_Matx44fR_const_IntrR_const_int(instance: *mut c_void, _depth: *const c_void, depth_factor: f32, camera_pose: *const core::Matx44f, intrinsics: *const crate::rgbd::Kinfu_Intr, frame_id: i32, ocvrs_return: *mut Result<()>);
// cv::kinfu::Volume::integrate(InputArray, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:31
// ("cv::kinfu::Volume::integrate", vec![(pred!(mut, ["_depth", "depthFactor", "cameraPose", "intrinsics"], ["const cv::_InputArray*", "float", "const cv::Matx44f*", "const cv::kinfu::Intr*"]), _)]),
pub fn cv_kinfu_Volume_integrate_const__InputArrayR_float_const_Matx44fR_const_IntrR(instance: *mut c_void, _depth: *const c_void, depth_factor: f32, camera_pose: *const core::Matx44f, intrinsics: *const crate::rgbd::Kinfu_Intr, ocvrs_return: *mut Result<()>);
// integrate(InputArray, InputArray, float, const Matx44f &, const kinfu::Intr &, const Intr &, const int)(InputArray, InputArray, Primitive, SimpleClass, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:33
// ("cv::kinfu::Volume::integrate", vec![(pred!(mut, ["_depth", "_rgb", "depthFactor", "cameraPose", "intrinsics", "rgb_intrinsics", "frameId"], ["const cv::_InputArray*", "const cv::_InputArray*", "float", "const cv::Matx44f*", "const cv::kinfu::Intr*", "const cv::kinfu::Intr*", "const int"]), _)]),
pub fn cv_kinfu_Volume_integrate_const__InputArrayR_const__InputArrayR_float_const_Matx44fR_const_IntrR_const_IntrR_const_int(instance: *mut c_void, _depth: *const c_void, _rgb: *const c_void, depth_factor: f32, camera_pose: *const core::Matx44f, intrinsics: *const crate::rgbd::Kinfu_Intr, rgb_intrinsics: *const crate::rgbd::Kinfu_Intr, frame_id: i32, ocvrs_return: *mut Result<()>);
// cv::kinfu::Volume::integrate(InputArray, InputArray, Primitive, SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:33
// ("cv::kinfu::Volume::integrate", vec![(pred!(mut, ["_depth", "_rgb", "depthFactor", "cameraPose", "intrinsics", "rgb_intrinsics"], ["const cv::_InputArray*", "const cv::_InputArray*", "float", "const cv::Matx44f*", "const cv::kinfu::Intr*", "const cv::kinfu::Intr*"]), _)]),
pub fn cv_kinfu_Volume_integrate_const__InputArrayR_const__InputArrayR_float_const_Matx44fR_const_IntrR_const_IntrR(instance: *mut c_void, _depth: *const c_void, _rgb: *const c_void, depth_factor: f32, camera_pose: *const core::Matx44f, intrinsics: *const crate::rgbd::Kinfu_Intr, rgb_intrinsics: *const crate::rgbd::Kinfu_Intr, ocvrs_return: *mut Result<()>);
// raycast(const Matx44f &, const kinfu::Intr &, const Size &, OutputArray, OutputArray)(SimpleClass, SimpleClass, SimpleClass, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:36
// ("cv::kinfu::Volume::raycast", vec![(pred!(const, ["cameraPose", "intrinsics", "frameSize", "points", "normals"], ["const cv::Matx44f*", "const cv::kinfu::Intr*", "const cv::Size*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_kinfu_Volume_raycast_const_const_Matx44fR_const_IntrR_const_SizeR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, camera_pose: *const core::Matx44f, intrinsics: *const crate::rgbd::Kinfu_Intr, frame_size: *const core::Size, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<()>);
// raycast(const Matx44f &, const kinfu::Intr &, const Size &, OutputArray, OutputArray, OutputArray)(SimpleClass, SimpleClass, SimpleClass, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:38
// ("cv::kinfu::Volume::raycast", vec![(pred!(const, ["cameraPose", "intrinsics", "frameSize", "points", "normals", "colors"], ["const cv::Matx44f*", "const cv::kinfu::Intr*", "const cv::Size*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_kinfu_Volume_raycast_const_const_Matx44fR_const_IntrR_const_SizeR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, camera_pose: *const core::Matx44f, intrinsics: *const crate::rgbd::Kinfu_Intr, frame_size: *const core::Size, points: *const c_void, normals: *const c_void, colors: *const c_void, ocvrs_return: *mut Result<()>);
// fetchNormals(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:40
// ("cv::kinfu::Volume::fetchNormals", vec![(pred!(const, ["points", "_normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_kinfu_Volume_fetchNormals_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, _normals: *const c_void, ocvrs_return: *mut Result<()>);
// fetchPointsNormals(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:41
// ("cv::kinfu::Volume::fetchPointsNormals", vec![(pred!(const, ["points", "normals"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_kinfu_Volume_fetchPointsNormals_const_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<()>);
// fetchPointsNormalsColors(OutputArray, OutputArray, OutputArray)(OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:42
// ("cv::kinfu::Volume::fetchPointsNormalsColors", vec![(pred!(const, ["unnamed", "unnamed", "unnamed"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_kinfu_Volume_fetchPointsNormalsColors_const_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, unnamed: *const c_void, unnamed_1: *const c_void, unnamed_2: *const c_void, ocvrs_return: *mut Result<()>);
// reset()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:46
// ("cv::kinfu::Volume::reset", vec![(pred!(mut, [], []), _)]),
pub fn cv_kinfu_Volume_reset(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::kinfu::Volume::voxelSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:49
// ("cv::kinfu::Volume::voxelSize", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Volume_propVoxelSize_const(instance: *const c_void) -> f32;
// cv::kinfu::Volume::voxelSizeInv() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:50
// ("cv::kinfu::Volume::voxelSizeInv", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Volume_propVoxelSizeInv_const(instance: *const c_void) -> f32;
// cv::kinfu::Volume::pose() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:51
// ("cv::kinfu::Volume::pose", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Volume_propPose_const(instance: *const c_void, ocvrs_return: *mut core::Affine3f);
// cv::kinfu::Volume::raycastStepFactor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:52
// ("cv::kinfu::Volume::raycastStepFactor", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Volume_propRaycastStepFactor_const(instance: *const c_void) -> f32;
// cv::kinfu::Volume::delete() generated
// ("cv::kinfu::Volume::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_kinfu_Volume_delete(instance: *mut c_void);
// defaultParams(VolumeType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:112
// ("cv::kinfu::VolumeParams::defaultParams", vec![(pred!(mut, ["_volumeType"], ["cv::kinfu::VolumeType"]), _)]),
pub fn cv_kinfu_VolumeParams_defaultParams_VolumeType(_volume_type: crate::rgbd::Kinfu_VolumeType, ocvrs_return: *mut Result<*mut c_void>);
// coarseParams(VolumeType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:117
// ("cv::kinfu::VolumeParams::coarseParams", vec![(pred!(mut, ["_volumeType"], ["cv::kinfu::VolumeType"]), _)]),
pub fn cv_kinfu_VolumeParams_coarseParams_VolumeType(_volume_type: crate::rgbd::Kinfu_VolumeType, ocvrs_return: *mut Result<*mut c_void>);
// cv::kinfu::VolumeParams::defaultNew() generated
// ("cv::kinfu::VolumeParams::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_VolumeParams_defaultNew_const() -> *mut c_void;
// cv::kinfu::VolumeParams::type() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:67
// ("cv::kinfu::VolumeParams::type", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_VolumeParams_propType_const(instance: *const c_void, ocvrs_return: *mut crate::rgbd::Kinfu_VolumeType);
// cv::kinfu::VolumeParams::setType(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:67
// ("cv::kinfu::VolumeParams::setType", vec![(pred!(mut, ["val"], ["const cv::kinfu::VolumeType"]), _)]),
pub fn cv_kinfu_VolumeParams_propType_const_VolumeType(instance: *mut c_void, val: crate::rgbd::Kinfu_VolumeType);
// cv::kinfu::VolumeParams::resolution() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:74
// ("cv::kinfu::VolumeParams::resolution", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_VolumeParams_propResolution_const(instance: *const c_void, ocvrs_return: *mut core::Vec3i);
// cv::kinfu::VolumeParams::setResolution(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:74
// ("cv::kinfu::VolumeParams::setResolution", vec![(pred!(mut, ["val"], ["const cv::Vec3i"]), _)]),
pub fn cv_kinfu_VolumeParams_propResolution_const_Vec3i(instance: *mut c_void, val: *const core::Vec3i);
// cv::kinfu::VolumeParams::unitResolution() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:80
// ("cv::kinfu::VolumeParams::unitResolution", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_VolumeParams_propUnitResolution_const(instance: *const c_void) -> i32;
// cv::kinfu::VolumeParams::setUnitResolution(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:80
// ("cv::kinfu::VolumeParams::setUnitResolution", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_kinfu_VolumeParams_propUnitResolution_const_int(instance: *mut c_void, val: i32);
// cv::kinfu::VolumeParams::pose() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:83
// ("cv::kinfu::VolumeParams::pose", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_VolumeParams_propPose_const(instance: *const c_void, ocvrs_return: *mut core::Affine3f);
// cv::kinfu::VolumeParams::setPose(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:83
// ("cv::kinfu::VolumeParams::setPose", vec![(pred!(mut, ["val"], ["const cv::Affine3f"]), _)]),
pub fn cv_kinfu_VolumeParams_propPose_const_Affine3f(instance: *mut c_void, val: *const core::Affine3f);
// cv::kinfu::VolumeParams::voxelSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:86
// ("cv::kinfu::VolumeParams::voxelSize", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_VolumeParams_propVoxelSize_const(instance: *const c_void) -> f32;
// cv::kinfu::VolumeParams::setVoxelSize(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:86
// ("cv::kinfu::VolumeParams::setVoxelSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_kinfu_VolumeParams_propVoxelSize_const_float(instance: *mut c_void, val: f32);
// cv::kinfu::VolumeParams::tsdfTruncDist() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:91
// ("cv::kinfu::VolumeParams::tsdfTruncDist", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_VolumeParams_propTsdfTruncDist_const(instance: *const c_void) -> f32;
// cv::kinfu::VolumeParams::setTsdfTruncDist(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:91
// ("cv::kinfu::VolumeParams::setTsdfTruncDist", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_kinfu_VolumeParams_propTsdfTruncDist_const_float(instance: *mut c_void, val: f32);
// cv::kinfu::VolumeParams::maxWeight() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:97
// ("cv::kinfu::VolumeParams::maxWeight", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_VolumeParams_propMaxWeight_const(instance: *const c_void) -> i32;
// cv::kinfu::VolumeParams::setMaxWeight(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:97
// ("cv::kinfu::VolumeParams::setMaxWeight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_kinfu_VolumeParams_propMaxWeight_const_int(instance: *mut c_void, val: i32);
// cv::kinfu::VolumeParams::depthTruncThreshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:102
// ("cv::kinfu::VolumeParams::depthTruncThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_VolumeParams_propDepthTruncThreshold_const(instance: *const c_void) -> f32;
// cv::kinfu::VolumeParams::setDepthTruncThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:102
// ("cv::kinfu::VolumeParams::setDepthTruncThreshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_kinfu_VolumeParams_propDepthTruncThreshold_const_float(instance: *mut c_void, val: f32);
// cv::kinfu::VolumeParams::raycastStepFactor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:107
// ("cv::kinfu::VolumeParams::raycastStepFactor", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_VolumeParams_propRaycastStepFactor_const(instance: *const c_void) -> f32;
// cv::kinfu::VolumeParams::setRaycastStepFactor(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:107
// ("cv::kinfu::VolumeParams::setRaycastStepFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_kinfu_VolumeParams_propRaycastStepFactor_const_float(instance: *mut c_void, val: f32);
// cv::kinfu::VolumeParams::delete() generated
// ("cv::kinfu::VolumeParams::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_kinfu_VolumeParams_delete(instance: *mut c_void);
// create()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:29
// ("cv::kinfu::detail::PoseGraph::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_kinfu_detail_PoseGraph_create(ocvrs_return: *mut Result<*mut c_void>);
// addNode(size_t, const Affine3d &, bool)(Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:33
// ("cv::kinfu::detail::PoseGraph::addNode", vec![(pred!(mut, ["_nodeId", "_pose", "fixed"], ["size_t", "const cv::Affine3d*", "bool"]), _)]),
pub fn cv_kinfu_detail_PoseGraph_addNode_size_t_const_Affine3dR_bool(instance: *mut c_void, _node_id: size_t, _pose: *const core::Affine3d, fixed: bool, ocvrs_return: *mut Result<()>);
// isNodeExist(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:34
// ("cv::kinfu::detail::PoseGraph::isNodeExist", vec![(pred!(const, ["nodeId"], ["size_t"]), _)]),
pub fn cv_kinfu_detail_PoseGraph_isNodeExist_const_size_t(instance: *const c_void, node_id: size_t, ocvrs_return: *mut Result<bool>);
// setNodeFixed(size_t, bool)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:35
// ("cv::kinfu::detail::PoseGraph::setNodeFixed", vec![(pred!(mut, ["nodeId", "fixed"], ["size_t", "bool"]), _)]),
pub fn cv_kinfu_detail_PoseGraph_setNodeFixed_size_t_bool(instance: *mut c_void, node_id: size_t, fixed: bool, ocvrs_return: *mut Result<bool>);
// isNodeFixed(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:36
// ("cv::kinfu::detail::PoseGraph::isNodeFixed", vec![(pred!(const, ["nodeId"], ["size_t"]), _)]),
pub fn cv_kinfu_detail_PoseGraph_isNodeFixed_const_size_t(instance: *const c_void, node_id: size_t, ocvrs_return: *mut Result<bool>);
// getNodePose(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:37
// ("cv::kinfu::detail::PoseGraph::getNodePose", vec![(pred!(const, ["nodeId"], ["size_t"]), _)]),
pub fn cv_kinfu_detail_PoseGraph_getNodePose_const_size_t(instance: *const c_void, node_id: size_t, ocvrs_return: *mut Result<core::Affine3d>);
// getNodesIds()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:38
// ("cv::kinfu::detail::PoseGraph::getNodesIds", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_detail_PoseGraph_getNodesIds_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getNumNodes()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:39
// ("cv::kinfu::detail::PoseGraph::getNumNodes", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_detail_PoseGraph_getNumNodes_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// addEdge(size_t, size_t, const Affine3f &, const Matx66f &)(Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:42
// ("cv::kinfu::detail::PoseGraph::addEdge", vec![(pred!(mut, ["_sourceNodeId", "_targetNodeId", "_transformation", "_information"], ["size_t", "size_t", "const cv::Affine3f*", "const cv::Matx66f*"]), _)]),
pub fn cv_kinfu_detail_PoseGraph_addEdge_size_t_size_t_const_Affine3fR_const_Matx66fR(instance: *mut c_void, _source_node_id: size_t, _target_node_id: size_t, _transformation: *const core::Affine3f, _information: *const core::Matx66f, ocvrs_return: *mut Result<()>);
// cv::kinfu::detail::PoseGraph::addEdge(Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:42
// ("cv::kinfu::detail::PoseGraph::addEdge", vec![(pred!(mut, ["_sourceNodeId", "_targetNodeId", "_transformation"], ["size_t", "size_t", "const cv::Affine3f*"]), _)]),
pub fn cv_kinfu_detail_PoseGraph_addEdge_size_t_size_t_const_Affine3fR(instance: *mut c_void, _source_node_id: size_t, _target_node_id: size_t, _transformation: *const core::Affine3f, ocvrs_return: *mut Result<()>);
// getEdgeStart(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:44
// ("cv::kinfu::detail::PoseGraph::getEdgeStart", vec![(pred!(const, ["i"], ["size_t"]), _)]),
pub fn cv_kinfu_detail_PoseGraph_getEdgeStart_const_size_t(instance: *const c_void, i: size_t, ocvrs_return: *mut Result<size_t>);
// getEdgeEnd(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:45
// ("cv::kinfu::detail::PoseGraph::getEdgeEnd", vec![(pred!(const, ["i"], ["size_t"]), _)]),
pub fn cv_kinfu_detail_PoseGraph_getEdgeEnd_const_size_t(instance: *const c_void, i: size_t, ocvrs_return: *mut Result<size_t>);
// getNumEdges()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:46
// ("cv::kinfu::detail::PoseGraph::getNumEdges", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_detail_PoseGraph_getNumEdges_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// isValid()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:49
// ("cv::kinfu::detail::PoseGraph::isValid", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_detail_PoseGraph_isValid_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// optimize(const cv::TermCriteria &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:53
// ("cv::kinfu::detail::PoseGraph::optimize", vec![(pred!(mut, ["tc"], ["const cv::TermCriteria*"]), _)]),
pub fn cv_kinfu_detail_PoseGraph_optimize_const_TermCriteriaR(instance: *mut c_void, tc: *const core::TermCriteria, ocvrs_return: *mut Result<i32>);
// cv::kinfu::detail::PoseGraph::optimize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:53
// ("cv::kinfu::detail::PoseGraph::optimize", vec![(pred!(mut, [], []), _)]),
pub fn cv_kinfu_detail_PoseGraph_optimize(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
// calcEnergy()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:56
// ("cv::kinfu::detail::PoseGraph::calcEnergy", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_detail_PoseGraph_calcEnergy_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// cv::kinfu::detail::PoseGraph::delete() generated
// ("cv::kinfu::detail::PoseGraph::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_kinfu_detail_PoseGraph_delete(instance: *mut c_void);
// create(const Ptr<Params> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:123
// ("cv::large_kinfu::LargeKinfu::create", vec![(pred!(mut, ["_params"], ["const cv::Ptr<cv::large_kinfu::Params>*"]), _)]),
pub fn cv_large_kinfu_LargeKinfu_create_const_PtrLParamsGR(_params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:126
// ("cv::large_kinfu::LargeKinfu::getParams", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_LargeKinfu_getParams_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// render(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:128
// ("cv::large_kinfu::LargeKinfu::render", vec![(pred!(const, ["image"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_large_kinfu_LargeKinfu_render_const_const__OutputArrayR(instance: *const c_void, image: *const c_void, ocvrs_return: *mut Result<()>);
// render(OutputArray, const Matx44f &)(OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:129
// ("cv::large_kinfu::LargeKinfu::render", vec![(pred!(const, ["image", "cameraPose"], ["const cv::_OutputArray*", "const cv::Matx44f*"]), _)]),
pub fn cv_large_kinfu_LargeKinfu_render_const_const__OutputArrayR_const_Matx44fR(instance: *const c_void, image: *const c_void, camera_pose: *const core::Matx44f, ocvrs_return: *mut Result<()>);
// getCloud(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:131
// ("cv::large_kinfu::LargeKinfu::getCloud", vec![(pred!(const, ["points", "normals"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_large_kinfu_LargeKinfu_getCloud_const_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<()>);
// getPoints(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:133
// ("cv::large_kinfu::LargeKinfu::getPoints", vec![(pred!(const, ["points"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_large_kinfu_LargeKinfu_getPoints_const_const__OutputArrayR(instance: *const c_void, points: *const c_void, ocvrs_return: *mut Result<()>);
// getNormals(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:135
// ("cv::large_kinfu::LargeKinfu::getNormals", vec![(pred!(const, ["points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_large_kinfu_LargeKinfu_getNormals_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<()>);
// reset()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:137
// ("cv::large_kinfu::LargeKinfu::reset", vec![(pred!(mut, [], []), _)]),
pub fn cv_large_kinfu_LargeKinfu_reset(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// getPose()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:139
// ("cv::large_kinfu::LargeKinfu::getPose", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_LargeKinfu_getPose_const(instance: *const c_void, ocvrs_return: *mut Result<core::Affine3f>);
// update(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:141
// ("cv::large_kinfu::LargeKinfu::update", vec![(pred!(mut, ["depth"], ["const cv::_InputArray*"]), _)]),
pub fn cv_large_kinfu_LargeKinfu_update_const__InputArrayR(instance: *mut c_void, depth: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::large_kinfu::LargeKinfu::delete() generated
// ("cv::large_kinfu::LargeKinfu::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_large_kinfu_LargeKinfu_delete(instance: *mut c_void);
// defaultParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:25
// ("cv::large_kinfu::Params::defaultParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_large_kinfu_Params_defaultParams(ocvrs_return: *mut Result<*mut c_void>);
// coarseParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:31
// ("cv::large_kinfu::Params::coarseParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_large_kinfu_Params_coarseParams(ocvrs_return: *mut Result<*mut c_void>);
// hashTSDFParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:36
// ("cv::large_kinfu::Params::hashTSDFParams", vec![(pred!(mut, ["isCoarse"], ["bool"]), _)]),
pub fn cv_large_kinfu_Params_hashTSDFParams_bool(is_coarse: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::large_kinfu::Params::defaultNew() generated
// ("cv::large_kinfu::Params::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_defaultNew_const() -> *mut c_void;
// cv::large_kinfu::Params::frameSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:39
// ("cv::large_kinfu::Params::frameSize", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_propFrameSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::large_kinfu::Params::setFrameSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:39
// ("cv::large_kinfu::Params::setFrameSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_large_kinfu_Params_propFrameSize_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::large_kinfu::Params::intr() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:42
// ("cv::large_kinfu::Params::intr", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_propIntr_const(instance: *const c_void, ocvrs_return: *mut core::Matx33f);
// cv::large_kinfu::Params::setIntr(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:42
// ("cv::large_kinfu::Params::setIntr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
pub fn cv_large_kinfu_Params_propIntr_const_Matx33f(instance: *mut c_void, val: *const core::Matx33f);
// cv::large_kinfu::Params::rgb_intr() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:45
// ("cv::large_kinfu::Params::rgb_intr", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_propRgb_intr_const(instance: *const c_void, ocvrs_return: *mut core::Matx33f);
// cv::large_kinfu::Params::setRgb_intr(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:45
// ("cv::large_kinfu::Params::setRgb_intr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
pub fn cv_large_kinfu_Params_propRgb_intr_const_Matx33f(instance: *mut c_void, val: *const core::Matx33f);
// cv::large_kinfu::Params::depthFactor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:53
// ("cv::large_kinfu::Params::depthFactor", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_propDepthFactor_const(instance: *const c_void) -> f32;
// cv::large_kinfu::Params::setDepthFactor(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:53
// ("cv::large_kinfu::Params::setDepthFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_large_kinfu_Params_propDepthFactor_const_float(instance: *mut c_void, val: f32);
// cv::large_kinfu::Params::bilateral_sigma_depth() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:56
// ("cv::large_kinfu::Params::bilateral_sigma_depth", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_propBilateral_sigma_depth_const(instance: *const c_void) -> f32;
// cv::large_kinfu::Params::setBilateral_sigma_depth(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:56
// ("cv::large_kinfu::Params::setBilateral_sigma_depth", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_large_kinfu_Params_propBilateral_sigma_depth_const_float(instance: *mut c_void, val: f32);
// cv::large_kinfu::Params::bilateral_sigma_spatial() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:58
// ("cv::large_kinfu::Params::bilateral_sigma_spatial", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_propBilateral_sigma_spatial_const(instance: *const c_void) -> f32;
// cv::large_kinfu::Params::setBilateral_sigma_spatial(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:58
// ("cv::large_kinfu::Params::setBilateral_sigma_spatial", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_large_kinfu_Params_propBilateral_sigma_spatial_const_float(instance: *mut c_void, val: f32);
// cv::large_kinfu::Params::bilateral_kernel_size() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:60
// ("cv::large_kinfu::Params::bilateral_kernel_size", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_propBilateral_kernel_size_const(instance: *const c_void) -> i32;
// cv::large_kinfu::Params::setBilateral_kernel_size(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:60
// ("cv::large_kinfu::Params::setBilateral_kernel_size", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_large_kinfu_Params_propBilateral_kernel_size_const_int(instance: *mut c_void, val: i32);
// cv::large_kinfu::Params::pyramidLevels() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:63
// ("cv::large_kinfu::Params::pyramidLevels", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_propPyramidLevels_const(instance: *const c_void) -> i32;
// cv::large_kinfu::Params::setPyramidLevels(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:63
// ("cv::large_kinfu::Params::setPyramidLevels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_large_kinfu_Params_propPyramidLevels_const_int(instance: *mut c_void, val: i32);
// cv::large_kinfu::Params::tsdf_min_camera_movement() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:68
// ("cv::large_kinfu::Params::tsdf_min_camera_movement", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_propTsdf_min_camera_movement_const(instance: *const c_void) -> f32;
// cv::large_kinfu::Params::setTsdf_min_camera_movement(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:68
// ("cv::large_kinfu::Params::setTsdf_min_camera_movement", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_large_kinfu_Params_propTsdf_min_camera_movement_const_float(instance: *mut c_void, val: f32);
// cv::large_kinfu::Params::lightPose() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:71
// ("cv::large_kinfu::Params::lightPose", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_propLightPose_const(instance: *const c_void, ocvrs_return: *mut core::Vec3f);
// cv::large_kinfu::Params::setLightPose(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:71
// ("cv::large_kinfu::Params::setLightPose", vec![(pred!(mut, ["val"], ["const cv::Vec3f"]), _)]),
pub fn cv_large_kinfu_Params_propLightPose_const_Vec3f(instance: *mut c_void, val: *const core::Vec3f);
// cv::large_kinfu::Params::icpDistThresh() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:74
// ("cv::large_kinfu::Params::icpDistThresh", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_propIcpDistThresh_const(instance: *const c_void) -> f32;
// cv::large_kinfu::Params::setIcpDistThresh(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:74
// ("cv::large_kinfu::Params::setIcpDistThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_large_kinfu_Params_propIcpDistThresh_const_float(instance: *mut c_void, val: f32);
// cv::large_kinfu::Params::icpAngleThresh() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:76
// ("cv::large_kinfu::Params::icpAngleThresh", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_propIcpAngleThresh_const(instance: *const c_void) -> f32;
// cv::large_kinfu::Params::setIcpAngleThresh(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:76
// ("cv::large_kinfu::Params::setIcpAngleThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_large_kinfu_Params_propIcpAngleThresh_const_float(instance: *mut c_void, val: f32);
// cv::large_kinfu::Params::icpIterations() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:78
// ("cv::large_kinfu::Params::icpIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_propIcpIterations_const(instance: *const c_void) -> *mut c_void;
// cv::large_kinfu::Params::setIcpIterations(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:78
// ("cv::large_kinfu::Params::setIcpIterations", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
pub fn cv_large_kinfu_Params_propIcpIterations_const_vectorLintG(instance: *mut c_void, val: *const c_void);
// cv::large_kinfu::Params::truncateThreshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:83
// ("cv::large_kinfu::Params::truncateThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_propTruncateThreshold_const(instance: *const c_void) -> f32;
// cv::large_kinfu::Params::setTruncateThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:83
// ("cv::large_kinfu::Params::setTruncateThreshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_large_kinfu_Params_propTruncateThreshold_const_float(instance: *mut c_void, val: f32);
// cv::large_kinfu::Params::volumeParams() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:87
// ("cv::large_kinfu::Params::volumeParams", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_propVolumeParams_const(instance: *const c_void) -> *mut c_void;
// cv::large_kinfu::Params::setVolumeParams(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:87
// ("cv::large_kinfu::Params::setVolumeParams", vec![(pred!(mut, ["val"], ["const cv::kinfu::VolumeParams"]), _)]),
pub fn cv_large_kinfu_Params_propVolumeParams_const_VolumeParams(instance: *mut c_void, val: *const c_void);
// cv::large_kinfu::Params::delete() generated
// ("cv::large_kinfu::Params::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_large_kinfu_Params_delete(instance: *mut c_void);
// ColorGradient()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:172
// ("cv::linemod::ColorGradient::ColorGradient", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_ColorGradient_ColorGradient(ocvrs_return: *mut Result<*mut c_void>);
// ColorGradient(float, size_t, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:182
// ("cv::linemod::ColorGradient::ColorGradient", vec![(pred!(mut, ["weak_threshold", "num_features", "strong_threshold"], ["float", "size_t", "float"]), _)]),
pub fn cv_linemod_ColorGradient_ColorGradient_float_size_t_float(weak_threshold: f32, num_features: size_t, strong_threshold: f32, ocvrs_return: *mut Result<*mut c_void>);
// create(float, size_t, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:184
// ("cv::linemod::ColorGradient::create", vec![(pred!(mut, ["weak_threshold", "num_features", "strong_threshold"], ["float", "size_t", "float"]), _)]),
pub fn cv_linemod_ColorGradient_create_float_size_t_float(weak_threshold: f32, num_features: size_t, strong_threshold: f32, ocvrs_return: *mut Result<*mut c_void>);
// name()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:186
// ("cv::linemod::ColorGradient::name", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_ColorGradient_name_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:188
// ("cv::linemod::ColorGradient::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_linemod_ColorGradient_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:189
// ("cv::linemod::ColorGradient::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_linemod_ColorGradient_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::linemod::ColorGradient::weak_threshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:191
// ("cv::linemod::ColorGradient::weak_threshold", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_ColorGradient_propWeak_threshold_const(instance: *const c_void) -> f32;
// cv::linemod::ColorGradient::setWeak_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:191
// ("cv::linemod::ColorGradient::setWeak_threshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_linemod_ColorGradient_propWeak_threshold_const_float(instance: *mut c_void, val: f32);
// cv::linemod::ColorGradient::num_features() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:192
// ("cv::linemod::ColorGradient::num_features", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_ColorGradient_propNum_features_const(instance: *const c_void) -> size_t;
// cv::linemod::ColorGradient::setNum_features(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:192
// ("cv::linemod::ColorGradient::setNum_features", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
pub fn cv_linemod_ColorGradient_propNum_features_const_size_t(instance: *mut c_void, val: size_t);
// cv::linemod::ColorGradient::strong_threshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:193
// ("cv::linemod::ColorGradient::strong_threshold", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_ColorGradient_propStrong_threshold_const(instance: *const c_void) -> f32;
// cv::linemod::ColorGradient::setStrong_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:193
// ("cv::linemod::ColorGradient::setStrong_threshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_linemod_ColorGradient_propStrong_threshold_const_float(instance: *mut c_void, val: f32);
// cv::linemod::ColorGradient::to_LineMod_Modality() generated
// ("cv::linemod::ColorGradient::to_LineMod_Modality", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_ColorGradient_to_LineMod_Modality(instance: *mut c_void) -> *mut c_void;
// cv::linemod::ColorGradient::delete() generated
// ("cv::linemod::ColorGradient::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_ColorGradient_delete(instance: *mut c_void);
// DepthNormal()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:209
// ("cv::linemod::DepthNormal::DepthNormal", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_DepthNormal_DepthNormal(ocvrs_return: *mut Result<*mut c_void>);
// DepthNormal(int, int, size_t, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:221
// ("cv::linemod::DepthNormal::DepthNormal", vec![(pred!(mut, ["distance_threshold", "difference_threshold", "num_features", "extract_threshold"], ["int", "int", "size_t", "int"]), _)]),
pub fn cv_linemod_DepthNormal_DepthNormal_int_int_size_t_int(distance_threshold: i32, difference_threshold: i32, num_features: size_t, extract_threshold: i32, ocvrs_return: *mut Result<*mut c_void>);
// create(int, int, size_t, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:224
// ("cv::linemod::DepthNormal::create", vec![(pred!(mut, ["distance_threshold", "difference_threshold", "num_features", "extract_threshold"], ["int", "int", "size_t", "int"]), _)]),
pub fn cv_linemod_DepthNormal_create_int_int_size_t_int(distance_threshold: i32, difference_threshold: i32, num_features: size_t, extract_threshold: i32, ocvrs_return: *mut Result<*mut c_void>);
// name()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:227
// ("cv::linemod::DepthNormal::name", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_DepthNormal_name_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:229
// ("cv::linemod::DepthNormal::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_linemod_DepthNormal_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:230
// ("cv::linemod::DepthNormal::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_linemod_DepthNormal_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::linemod::DepthNormal::distance_threshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:232
// ("cv::linemod::DepthNormal::distance_threshold", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_DepthNormal_propDistance_threshold_const(instance: *const c_void) -> i32;
// cv::linemod::DepthNormal::setDistance_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:232
// ("cv::linemod::DepthNormal::setDistance_threshold", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_linemod_DepthNormal_propDistance_threshold_const_int(instance: *mut c_void, val: i32);
// cv::linemod::DepthNormal::difference_threshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:233
// ("cv::linemod::DepthNormal::difference_threshold", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_DepthNormal_propDifference_threshold_const(instance: *const c_void) -> i32;
// cv::linemod::DepthNormal::setDifference_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:233
// ("cv::linemod::DepthNormal::setDifference_threshold", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_linemod_DepthNormal_propDifference_threshold_const_int(instance: *mut c_void, val: i32);
// cv::linemod::DepthNormal::num_features() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:234
// ("cv::linemod::DepthNormal::num_features", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_DepthNormal_propNum_features_const(instance: *const c_void) -> size_t;
// cv::linemod::DepthNormal::setNum_features(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:234
// ("cv::linemod::DepthNormal::setNum_features", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
pub fn cv_linemod_DepthNormal_propNum_features_const_size_t(instance: *mut c_void, val: size_t);
// cv::linemod::DepthNormal::extract_threshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:235
// ("cv::linemod::DepthNormal::extract_threshold", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_DepthNormal_propExtract_threshold_const(instance: *const c_void) -> i32;
// cv::linemod::DepthNormal::setExtract_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:235
// ("cv::linemod::DepthNormal::setExtract_threshold", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_linemod_DepthNormal_propExtract_threshold_const_int(instance: *mut c_void, val: i32);
// cv::linemod::DepthNormal::to_LineMod_Modality() generated
// ("cv::linemod::DepthNormal::to_LineMod_Modality", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_DepthNormal_to_LineMod_Modality(instance: *mut c_void) -> *mut c_void;
// cv::linemod::DepthNormal::delete() generated
// ("cv::linemod::DepthNormal::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_DepthNormal_delete(instance: *mut c_void);
// Detector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:304
// ("cv::linemod::Detector::Detector", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_Detector_Detector(ocvrs_return: *mut Result<*mut c_void>);
// Detector(const std::vector<Ptr<Modality>> &, const std::vector<int> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:313
// ("cv::linemod::Detector::Detector", vec![(pred!(mut, ["modalities", "T_pyramid"], ["const std::vector<cv::Ptr<cv::linemod::Modality>>*", "const std::vector<int>*"]), _)]),
pub fn cv_linemod_Detector_Detector_const_vectorLPtrLModalityGGR_const_vectorLintGR(modalities: *const c_void, t_pyramid: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// match(const std::vector<Mat> &, float, std::vector<Match> &, const std::vector<String> &, OutputArrayOfArrays, const std::vector<Mat> &)(CppPassByVoidPtr, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:330
// ("cv::linemod::Detector::match", vec![(pred!(const, ["sources", "threshold", "matches", "class_ids", "quantized_images", "masks"], ["const std::vector<cv::Mat>*", "float", "std::vector<cv::linemod::Match>*", "const std::vector<cv::String>*", "const cv::_OutputArray*", "const std::vector<cv::Mat>*"]), _)]),
pub fn cv_linemod_Detector_match_const_const_vectorLMatGR_float_vectorLMatchGR_const_vectorLStringGR_const__OutputArrayR_const_vectorLMatGR(instance: *const c_void, sources: *const c_void, threshold: f32, matches: *mut c_void, class_ids: *const c_void, quantized_images: *const c_void, masks: *const c_void, ocvrs_return: *mut Result<()>);
// cv::linemod::Detector::match(CppPassByVoidPtr, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:330
// ("cv::linemod::Detector::match", vec![(pred!(const, ["sources", "threshold", "matches"], ["const std::vector<cv::Mat>*", "float", "std::vector<cv::linemod::Match>*"]), _)]),
pub fn cv_linemod_Detector_match_const_const_vectorLMatGR_float_vectorLMatchGR(instance: *const c_void, sources: *const c_void, threshold: f32, matches: *mut c_void, ocvrs_return: *mut Result<()>);
// addTemplate(const std::vector<Mat> &, const String &, const Mat &, Rect *)(CppPassByVoidPtr, InString, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:345
// ("cv::linemod::Detector::addTemplate", vec![(pred!(mut, ["sources", "class_id", "object_mask", "bounding_box"], ["const std::vector<cv::Mat>*", "const cv::String*", "const cv::Mat*", "cv::Rect*"]), _)]),
pub fn cv_linemod_Detector_addTemplate_const_vectorLMatGR_const_StringR_const_MatR_RectX(instance: *mut c_void, sources: *const c_void, class_id: *const c_char, object_mask: *const c_void, bounding_box: *mut core::Rect, ocvrs_return: *mut Result<i32>);
// cv::linemod::Detector::addTemplate(CppPassByVoidPtr, InString, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:345
// ("cv::linemod::Detector::addTemplate", vec![(pred!(mut, ["sources", "class_id", "object_mask"], ["const std::vector<cv::Mat>*", "const cv::String*", "const cv::Mat*"]), _)]),
pub fn cv_linemod_Detector_addTemplate_const_vectorLMatGR_const_StringR_const_MatR(instance: *mut c_void, sources: *const c_void, class_id: *const c_char, object_mask: *const c_void, ocvrs_return: *mut Result<i32>);
// addSyntheticTemplate(const std::vector<Template> &, const String &)(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:351
// ("cv::linemod::Detector::addSyntheticTemplate", vec![(pred!(mut, ["templates", "class_id"], ["const std::vector<cv::linemod::Template>*", "const cv::String*"]), _)]),
pub fn cv_linemod_Detector_addSyntheticTemplate_const_vectorLTemplateGR_const_StringR(instance: *mut c_void, templates: *const c_void, class_id: *const c_char, ocvrs_return: *mut Result<i32>);
// getModalities()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:359
// ("cv::linemod::Detector::getModalities", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Detector_getModalities_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getT(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:364
// ("cv::linemod::Detector::getT", vec![(pred!(const, ["pyramid_level"], ["int"]), _)]),
pub fn cv_linemod_Detector_getT_const_int(instance: *const c_void, pyramid_level: i32, ocvrs_return: *mut Result<i32>);
// pyramidLevels()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:369
// ("cv::linemod::Detector::pyramidLevels", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Detector_pyramidLevels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// getTemplates(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:377
// ("cv::linemod::Detector::getTemplates", vec![(pred!(const, ["class_id", "template_id"], ["const cv::String*", "int"]), _)]),
pub fn cv_linemod_Detector_getTemplates_const_const_StringR_int(instance: *const c_void, class_id: *const c_char, template_id: i32, ocvrs_return: *mut Result<*mut c_void>);
// numTemplates()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:379
// ("cv::linemod::Detector::numTemplates", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Detector_numTemplates_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// numTemplates(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:380
// ("cv::linemod::Detector::numTemplates", vec![(pred!(const, ["class_id"], ["const cv::String*"]), _)]),
pub fn cv_linemod_Detector_numTemplates_const_const_StringR(instance: *const c_void, class_id: *const c_char, ocvrs_return: *mut Result<i32>);
// numClasses()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:381
// ("cv::linemod::Detector::numClasses", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Detector_numClasses_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// classIds()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:383
// ("cv::linemod::Detector::classIds", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Detector_classIds_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:385
// ("cv::linemod::Detector::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_linemod_Detector_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:386
// ("cv::linemod::Detector::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_linemod_Detector_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// readClass(const FileNode &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:388
// ("cv::linemod::Detector::readClass", vec![(pred!(mut, ["fn", "class_id_override"], ["const cv::FileNode*", "const cv::String*"]), _)]),
pub fn cv_linemod_Detector_readClass_const_FileNodeR_const_StringR(instance: *mut c_void, fn_: *const c_void, class_id_override: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::linemod::Detector::readClass(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:388
// ("cv::linemod::Detector::readClass", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_linemod_Detector_readClass_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// writeClass(const String &, FileStorage &)(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:389
// ("cv::linemod::Detector::writeClass", vec![(pred!(const, ["class_id", "fs"], ["const cv::String*", "cv::FileStorage*"]), _)]),
pub fn cv_linemod_Detector_writeClass_const_const_StringR_FileStorageR(instance: *const c_void, class_id: *const c_char, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// readClasses(const std::vector<String> &, const String &)(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:391
// ("cv::linemod::Detector::readClasses", vec![(pred!(mut, ["class_ids", "format"], ["const std::vector<cv::String>*", "const cv::String*"]), _)]),
pub fn cv_linemod_Detector_readClasses_const_vectorLStringGR_const_StringR(instance: *mut c_void, class_ids: *const c_void, format: *const c_char, ocvrs_return: *mut Result<()>);
// cv::linemod::Detector::readClasses(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:391
// ("cv::linemod::Detector::readClasses", vec![(pred!(mut, ["class_ids"], ["const std::vector<cv::String>*"]), _)]),
pub fn cv_linemod_Detector_readClasses_const_vectorLStringGR(instance: *mut c_void, class_ids: *const c_void, ocvrs_return: *mut Result<()>);
// writeClasses(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:393
// ("cv::linemod::Detector::writeClasses", vec![(pred!(const, ["format"], ["const cv::String*"]), _)]),
pub fn cv_linemod_Detector_writeClasses_const_const_StringR(instance: *const c_void, format: *const c_char, ocvrs_return: *mut Result<()>);
// cv::linemod::Detector::writeClasses() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:393
// ("cv::linemod::Detector::writeClasses", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Detector_writeClasses_const(instance: *const c_void, ocvrs_return: *mut Result<()>);
// cv::linemod::Detector::delete() generated
// ("cv::linemod::Detector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_Detector_delete(instance: *mut c_void);
// Feature()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:32
// ("cv::linemod::Feature::Feature", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_Feature_Feature(ocvrs_return: *mut Result<crate::rgbd::LineMod_Feature>);
// Feature(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:33
// ("cv::linemod::Feature::Feature", vec![(pred!(mut, ["x", "y", "label"], ["int", "int", "int"]), _)]),
pub fn cv_linemod_Feature_Feature_int_int_int(x: i32, y: i32, label: i32, ocvrs_return: *mut Result<crate::rgbd::LineMod_Feature>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:35
// ("cv::linemod::Feature::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_linemod_Feature_read_const_FileNodeR(instance: *const crate::rgbd::LineMod_Feature, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:36
// ("cv::linemod::Feature::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_linemod_Feature_write_const_FileStorageR(instance: *const crate::rgbd::LineMod_Feature, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// Match()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:261
// ("cv::linemod::Match::Match", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_Match_Match(ocvrs_return: *mut Result<*mut c_void>);
// Match(int, int, float, const String &, int)(Primitive, Primitive, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:265
// ("cv::linemod::Match::Match", vec![(pred!(mut, ["x", "y", "similarity", "class_id", "template_id"], ["int", "int", "float", "const cv::String*", "int"]), _)]),
pub fn cv_linemod_Match_Match_int_int_float_const_StringR_int(x: i32, y: i32, similarity: f32, class_id: *const c_char, template_id: i32, ocvrs_return: *mut Result<*mut c_void>);
// operator<(const Match &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:268
// ("cv::linemod::Match::operator<", vec![(pred!(const, ["rhs"], ["const cv::linemod::Match*"]), _)]),
pub fn cv_linemod_Match_operatorL_const_const_MatchR(instance: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<bool>);
// operator==(const Match &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:277
// ("cv::linemod::Match::operator==", vec![(pred!(const, ["rhs"], ["const cv::linemod::Match*"]), _)]),
pub fn cv_linemod_Match_operatorEQ_const_const_MatchR(instance: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::linemod::Match::implicitClone() generated
// ("cv::linemod::Match::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Match_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::linemod::Match::x() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:282
// ("cv::linemod::Match::x", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Match_propX_const(instance: *const c_void) -> i32;
// cv::linemod::Match::setX(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:282
// ("cv::linemod::Match::setX", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_linemod_Match_propX_const_int(instance: *mut c_void, val: i32);
// cv::linemod::Match::y() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:283
// ("cv::linemod::Match::y", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Match_propY_const(instance: *const c_void) -> i32;
// cv::linemod::Match::setY(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:283
// ("cv::linemod::Match::setY", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_linemod_Match_propY_const_int(instance: *mut c_void, val: i32);
// cv::linemod::Match::similarity() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:284
// ("cv::linemod::Match::similarity", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Match_propSimilarity_const(instance: *const c_void) -> f32;
// cv::linemod::Match::setSimilarity(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:284
// ("cv::linemod::Match::setSimilarity", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_linemod_Match_propSimilarity_const_float(instance: *mut c_void, val: f32);
// cv::linemod::Match::class_id() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:285
// ("cv::linemod::Match::class_id", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Match_propClass_id_const(instance: *const c_void) -> *mut c_void;
// cv::linemod::Match::setClass_id(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:285
// ("cv::linemod::Match::setClass_id", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
pub fn cv_linemod_Match_propClass_id_const_String(instance: *mut c_void, val: *const c_char);
// cv::linemod::Match::template_id() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:286
// ("cv::linemod::Match::template_id", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Match_propTemplate_id_const(instance: *const c_void) -> i32;
// cv::linemod::Match::setTemplate_id(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:286
// ("cv::linemod::Match::setTemplate_id", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_linemod_Match_propTemplate_id_const_int(instance: *mut c_void, val: i32);
// cv::linemod::Match::delete() generated
// ("cv::linemod::Match::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_Match_delete(instance: *mut c_void);
// process(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:132
// ("cv::linemod::Modality::process", vec![(pred!(const, ["src", "mask"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_linemod_Modality_process_const_const_MatR_const_MatR(instance: *const c_void, src: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::linemod::Modality::process(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:132
// ("cv::linemod::Modality::process", vec![(pred!(const, ["src"], ["const cv::Mat*"]), _)]),
pub fn cv_linemod_Modality_process_const_const_MatR(instance: *const c_void, src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// name()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:138
// ("cv::linemod::Modality::name", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Modality_name_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:140
// ("cv::linemod::Modality::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_linemod_Modality_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:141
// ("cv::linemod::Modality::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_linemod_Modality_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// create(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:150
// ("cv::linemod::Modality::create", vec![(pred!(mut, ["modality_type"], ["const cv::String*"]), _)]),
pub fn cv_linemod_Modality_create_const_StringR(modality_type: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// create(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:155
// ("cv::linemod::Modality::create", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_linemod_Modality_create_const_FileNodeR(fn_: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::linemod::Modality::to_LineMod_ColorGradient() generated
// ("cv::linemod::Modality::to_LineMod_ColorGradient", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_Modality_to_LineMod_ColorGradient(instance: *mut c_void) -> *mut c_void;
// cv::linemod::Modality::to_LineMod_DepthNormal() generated
// ("cv::linemod::Modality::to_LineMod_DepthNormal", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_Modality_to_LineMod_DepthNormal(instance: *mut c_void) -> *mut c_void;
// cv::linemod::Modality::delete() generated
// ("cv::linemod::Modality::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_Modality_delete(instance: *mut c_void);
// quantize(Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:67
// ("cv::linemod::QuantizedPyramid::quantize", vec![(pred!(const, ["dst"], ["cv::Mat*"]), _)]),
pub fn cv_linemod_QuantizedPyramid_quantize_const_MatR(instance: *const c_void, dst: *mut c_void, ocvrs_return: *mut Result<()>);
// extractTemplate(Template &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:74
// ("cv::linemod::QuantizedPyramid::extractTemplate", vec![(pred!(const, ["templ"], ["cv::linemod::Template*"]), _)]),
pub fn cv_linemod_QuantizedPyramid_extractTemplate_const_TemplateR(instance: *const c_void, templ: *mut c_void, ocvrs_return: *mut Result<bool>);
// pyrDown()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:81
// ("cv::linemod::QuantizedPyramid::pyrDown", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_QuantizedPyramid_pyrDown(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::linemod::QuantizedPyramid::delete() generated
// ("cv::linemod::QuantizedPyramid::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_QuantizedPyramid_delete(instance: *mut c_void);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:48
// ("cv::linemod::Template::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_linemod_Template_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:49
// ("cv::linemod::Template::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_linemod_Template_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::linemod::Template::implicitClone() generated
// ("cv::linemod::Template::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Template_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::linemod::Template::defaultNew() generated
// ("cv::linemod::Template::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Template_defaultNew_const() -> *mut c_void;
// cv::linemod::Template::width() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:43
// ("cv::linemod::Template::width", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Template_propWidth_const(instance: *const c_void) -> i32;
// cv::linemod::Template::setWidth(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:43
// ("cv::linemod::Template::setWidth", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_linemod_Template_propWidth_const_int(instance: *mut c_void, val: i32);
// cv::linemod::Template::height() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:44
// ("cv::linemod::Template::height", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Template_propHeight_const(instance: *const c_void) -> i32;
// cv::linemod::Template::setHeight(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:44
// ("cv::linemod::Template::setHeight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_linemod_Template_propHeight_const_int(instance: *mut c_void, val: i32);
// cv::linemod::Template::pyramid_level() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:45
// ("cv::linemod::Template::pyramid_level", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Template_propPyramid_level_const(instance: *const c_void) -> i32;
// cv::linemod::Template::setPyramid_level(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:45
// ("cv::linemod::Template::setPyramid_level", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_linemod_Template_propPyramid_level_const_int(instance: *mut c_void, val: i32);
// cv::linemod::Template::features() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:46
// ("cv::linemod::Template::features", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Template_propFeatures_const(instance: *const c_void) -> *mut c_void;
// cv::linemod::Template::setFeatures(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:46
// ("cv::linemod::Template::setFeatures", vec![(pred!(mut, ["val"], ["const std::vector<cv::linemod::Feature>"]), _)]),
pub fn cv_linemod_Template_propFeatures_const_vectorLFeatureG(instance: *mut c_void, val: *const c_void);
// cv::linemod::Template::delete() generated
// ("cv::linemod::Template::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_Template_delete(instance: *mut c_void);
// DepthCleaner()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:198
// ("cv::rgbd::DepthCleaner::DepthCleaner", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_DepthCleaner_DepthCleaner(ocvrs_return: *mut Result<*mut c_void>);
// DepthCleaner(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:212
// ("cv::rgbd::DepthCleaner::DepthCleaner", vec![(pred!(mut, ["depth", "window_size", "method"], ["int", "int", "int"]), _)]),
pub fn cv_rgbd_DepthCleaner_DepthCleaner_int_int_int(depth: i32, window_size: i32, method: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::DepthCleaner::DepthCleaner(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:212
// ("cv::rgbd::DepthCleaner::DepthCleaner", vec![(pred!(mut, ["depth"], ["int"]), _)]),
pub fn cv_rgbd_DepthCleaner_DepthCleaner_int(depth: i32, ocvrs_return: *mut Result<*mut c_void>);
// create(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:216
// ("cv::rgbd::DepthCleaner::create", vec![(pred!(mut, ["depth", "window_size", "method"], ["int", "int", "int"]), _)]),
pub fn cv_rgbd_DepthCleaner_create_int_int_int(depth: i32, window_size: i32, method: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::DepthCleaner::create(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:216
// ("cv::rgbd::DepthCleaner::create", vec![(pred!(mut, ["depth"], ["int"]), _)]),
pub fn cv_rgbd_DepthCleaner_create_int(depth: i32, ocvrs_return: *mut Result<*mut c_void>);
// operator()(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:223
// ("cv::rgbd::DepthCleaner::operator()", vec![(pred!(const, ["points", "depth"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_rgbd_DepthCleaner_operator___const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, depth: *const c_void, ocvrs_return: *mut Result<()>);
// initialize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:229
// ("cv::rgbd::DepthCleaner::initialize", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_DepthCleaner_initialize_const(instance: *const c_void, ocvrs_return: *mut Result<()>);
// getWindowSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:231
// ("cv::rgbd::DepthCleaner::getWindowSize", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_DepthCleaner_getWindowSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setWindowSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:235
// ("cv::rgbd::DepthCleaner::setWindowSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_rgbd_DepthCleaner_setWindowSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getDepth()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:239
// ("cv::rgbd::DepthCleaner::getDepth", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_DepthCleaner_getDepth_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setDepth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:243
// ("cv::rgbd::DepthCleaner::setDepth", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_rgbd_DepthCleaner_setDepth_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getMethod()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:247
// ("cv::rgbd::DepthCleaner::getMethod", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_DepthCleaner_getMethod_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMethod(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:251
// ("cv::rgbd::DepthCleaner::setMethod", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_rgbd_DepthCleaner_setMethod_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// cv::rgbd::DepthCleaner::to_Algorithm() generated
// ("cv::rgbd::DepthCleaner::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_DepthCleaner_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::DepthCleaner::delete() generated
// ("cv::rgbd::DepthCleaner::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_DepthCleaner_delete(instance: *mut c_void);
// FastICPOdometry()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1042
// ("cv::rgbd::FastICPOdometry::FastICPOdometry", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_FastICPOdometry_FastICPOdometry(ocvrs_return: *mut Result<*mut c_void>);
// FastICPOdometry(const Mat &, float, float, float, float, int, const std::vector<int> &)(TraitClass, Primitive, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1054
// ("cv::rgbd::FastICPOdometry::FastICPOdometry", vec![(pred!(mut, ["cameraMatrix", "maxDistDiff", "angleThreshold", "sigmaDepth", "sigmaSpatial", "kernelSize", "iterCounts"], ["const cv::Mat*", "float", "float", "float", "float", "int", "const std::vector<int>*"]), _)]),
pub fn cv_rgbd_FastICPOdometry_FastICPOdometry_const_MatR_float_float_float_float_int_const_vectorLintGR(camera_matrix: *const c_void, max_dist_diff: f32, angle_threshold: f32, sigma_depth: f32, sigma_spatial: f32, kernel_size: i32, iter_counts: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::FastICPOdometry::FastICPOdometry(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1054
// ("cv::rgbd::FastICPOdometry::FastICPOdometry", vec![(pred!(mut, ["cameraMatrix"], ["const cv::Mat*"]), _)]),
pub fn cv_rgbd_FastICPOdometry_FastICPOdometry_const_MatR(camera_matrix: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create(const Mat &, float, float, float, float, int, const std::vector<int> &)(TraitClass, Primitive, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1062
// ("cv::rgbd::FastICPOdometry::create", vec![(pred!(mut, ["cameraMatrix", "maxDistDiff", "angleThreshold", "sigmaDepth", "sigmaSpatial", "kernelSize", "iterCounts"], ["const cv::Mat*", "float", "float", "float", "float", "int", "const std::vector<int>*"]), _)]),
pub fn cv_rgbd_FastICPOdometry_create_const_MatR_float_float_float_float_int_const_vectorLintGR(camera_matrix: *const c_void, max_dist_diff: f32, angle_threshold: f32, sigma_depth: f32, sigma_spatial: f32, kernel_size: i32, iter_counts: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::FastICPOdometry::create(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1062
// ("cv::rgbd::FastICPOdometry::create", vec![(pred!(mut, ["cameraMatrix"], ["const cv::Mat*"]), _)]),
pub fn cv_rgbd_FastICPOdometry_create_const_MatR(camera_matrix: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// prepareFrameCache(Ptr<OdometryFrame> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1070
// ("cv::rgbd::FastICPOdometry::prepareFrameCache", vec![(pred!(const, ["frame", "cacheType"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "int"]), _)]),
pub fn cv_rgbd_FastICPOdometry_prepareFrameCache_const_PtrLOdometryFrameGR_int(instance: *const c_void, frame: *mut c_void, cache_type: i32, ocvrs_return: *mut Result<core::Size>);
// getCameraMatrix()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1072
// ("cv::rgbd::FastICPOdometry::getCameraMatrix", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_FastICPOdometry_getCameraMatrix_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setCameraMatrix(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1076
// ("cv::rgbd::FastICPOdometry::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
pub fn cv_rgbd_FastICPOdometry_setCameraMatrix_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getMaxDistDiff()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1080
// ("cv::rgbd::FastICPOdometry::getMaxDistDiff", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_FastICPOdometry_getMaxDistDiff_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxDistDiff(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1084
// ("cv::rgbd::FastICPOdometry::setMaxDistDiff", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_rgbd_FastICPOdometry_setMaxDistDiff_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getAngleThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1088
// ("cv::rgbd::FastICPOdometry::getAngleThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_FastICPOdometry_getAngleThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setAngleThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1092
// ("cv::rgbd::FastICPOdometry::setAngleThreshold", vec![(pred!(mut, ["f"], ["float"]), _)]),
pub fn cv_rgbd_FastICPOdometry_setAngleThreshold_float(instance: *mut c_void, f: f32, ocvrs_return: *mut Result<()>);
// getSigmaDepth()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1096
// ("cv::rgbd::FastICPOdometry::getSigmaDepth", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_FastICPOdometry_getSigmaDepth_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setSigmaDepth(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1100
// ("cv::rgbd::FastICPOdometry::setSigmaDepth", vec![(pred!(mut, ["f"], ["float"]), _)]),
pub fn cv_rgbd_FastICPOdometry_setSigmaDepth_float(instance: *mut c_void, f: f32, ocvrs_return: *mut Result<()>);
// getSigmaSpatial()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1104
// ("cv::rgbd::FastICPOdometry::getSigmaSpatial", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_FastICPOdometry_getSigmaSpatial_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setSigmaSpatial(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1108
// ("cv::rgbd::FastICPOdometry::setSigmaSpatial", vec![(pred!(mut, ["f"], ["float"]), _)]),
pub fn cv_rgbd_FastICPOdometry_setSigmaSpatial_float(instance: *mut c_void, f: f32, ocvrs_return: *mut Result<()>);
// getKernelSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1112
// ("cv::rgbd::FastICPOdometry::getKernelSize", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_FastICPOdometry_getKernelSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setKernelSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1116
// ("cv::rgbd::FastICPOdometry::setKernelSize", vec![(pred!(mut, ["f"], ["int"]), _)]),
pub fn cv_rgbd_FastICPOdometry_setKernelSize_int(instance: *mut c_void, f: i32, ocvrs_return: *mut Result<()>);
// getIterationCounts()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1120
// ("cv::rgbd::FastICPOdometry::getIterationCounts", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_FastICPOdometry_getIterationCounts_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setIterationCounts(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1124
// ("cv::rgbd::FastICPOdometry::setIterationCounts", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
pub fn cv_rgbd_FastICPOdometry_setIterationCounts_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getTransformType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1128
// ("cv::rgbd::FastICPOdometry::getTransformType", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_FastICPOdometry_getTransformType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setTransformType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1132
// ("cv::rgbd::FastICPOdometry::setTransformType", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_rgbd_FastICPOdometry_setTransformType_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// cv::rgbd::FastICPOdometry::to_Algorithm() generated
// ("cv::rgbd::FastICPOdometry::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_FastICPOdometry_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::FastICPOdometry::to_Odometry() generated
// ("cv::rgbd::FastICPOdometry::to_Odometry", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_FastICPOdometry_to_Odometry(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::FastICPOdometry::delete() generated
// ("cv::rgbd::FastICPOdometry::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_FastICPOdometry_delete(instance: *mut c_void);
// ICPOdometry()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:762
// ("cv::rgbd::ICPOdometry::ICPOdometry", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_ICPOdometry_ICPOdometry(ocvrs_return: *mut Result<*mut c_void>);
// ICPOdometry(const Mat &, float, float, float, float, const std::vector<int> &, int)(TraitClass, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:773
// ("cv::rgbd::ICPOdometry::ICPOdometry", vec![(pred!(mut, ["cameraMatrix", "minDepth", "maxDepth", "maxDepthDiff", "maxPointsPart", "iterCounts", "transformType"], ["const cv::Mat*", "float", "float", "float", "float", "const std::vector<int>*", "int"]), _)]),
pub fn cv_rgbd_ICPOdometry_ICPOdometry_const_MatR_float_float_float_float_const_vectorLintGR_int(camera_matrix: *const c_void, min_depth: f32, max_depth: f32, max_depth_diff: f32, max_points_part: f32, iter_counts: *const c_void, transform_type: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::ICPOdometry::ICPOdometry(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:773
// ("cv::rgbd::ICPOdometry::ICPOdometry", vec![(pred!(mut, ["cameraMatrix"], ["const cv::Mat*"]), _)]),
pub fn cv_rgbd_ICPOdometry_ICPOdometry_const_MatR(camera_matrix: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create(const Mat &, float, float, float, float, const std::vector<int> &, int)(TraitClass, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:777
// ("cv::rgbd::ICPOdometry::create", vec![(pred!(mut, ["cameraMatrix", "minDepth", "maxDepth", "maxDepthDiff", "maxPointsPart", "iterCounts", "transformType"], ["const cv::Mat*", "float", "float", "float", "float", "const std::vector<int>*", "int"]), _)]),
pub fn cv_rgbd_ICPOdometry_create_const_MatR_float_float_float_float_const_vectorLintGR_int(camera_matrix: *const c_void, min_depth: f32, max_depth: f32, max_depth_diff: f32, max_points_part: f32, iter_counts: *const c_void, transform_type: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::ICPOdometry::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:777
// ("cv::rgbd::ICPOdometry::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_ICPOdometry_create(ocvrs_return: *mut Result<*mut c_void>);
// prepareFrameCache(Ptr<OdometryFrame> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:781
// ("cv::rgbd::ICPOdometry::prepareFrameCache", vec![(pred!(const, ["frame", "cacheType"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "int"]), _)]),
pub fn cv_rgbd_ICPOdometry_prepareFrameCache_const_PtrLOdometryFrameGR_int(instance: *const c_void, frame: *mut c_void, cache_type: i32, ocvrs_return: *mut Result<core::Size>);
// getCameraMatrix()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:783
// ("cv::rgbd::ICPOdometry::getCameraMatrix", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_ICPOdometry_getCameraMatrix_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setCameraMatrix(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:787
// ("cv::rgbd::ICPOdometry::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
pub fn cv_rgbd_ICPOdometry_setCameraMatrix_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getMinDepth()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:791
// ("cv::rgbd::ICPOdometry::getMinDepth", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_ICPOdometry_getMinDepth_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMinDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:795
// ("cv::rgbd::ICPOdometry::setMinDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_ICPOdometry_setMinDepth_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getMaxDepth()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:799
// ("cv::rgbd::ICPOdometry::getMaxDepth", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_ICPOdometry_getMaxDepth_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:803
// ("cv::rgbd::ICPOdometry::setMaxDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_ICPOdometry_setMaxDepth_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getMaxDepthDiff()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:807
// ("cv::rgbd::ICPOdometry::getMaxDepthDiff", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_ICPOdometry_getMaxDepthDiff_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxDepthDiff(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:811
// ("cv::rgbd::ICPOdometry::setMaxDepthDiff", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_ICPOdometry_setMaxDepthDiff_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getIterationCounts()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:815
// ("cv::rgbd::ICPOdometry::getIterationCounts", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_ICPOdometry_getIterationCounts_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setIterationCounts(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:819
// ("cv::rgbd::ICPOdometry::setIterationCounts", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
pub fn cv_rgbd_ICPOdometry_setIterationCounts_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getMaxPointsPart()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:823
// ("cv::rgbd::ICPOdometry::getMaxPointsPart", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_ICPOdometry_getMaxPointsPart_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxPointsPart(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:827
// ("cv::rgbd::ICPOdometry::setMaxPointsPart", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_ICPOdometry_setMaxPointsPart_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getTransformType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:831
// ("cv::rgbd::ICPOdometry::getTransformType", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_ICPOdometry_getTransformType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setTransformType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:835
// ("cv::rgbd::ICPOdometry::setTransformType", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_rgbd_ICPOdometry_setTransformType_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getMaxTranslation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:839
// ("cv::rgbd::ICPOdometry::getMaxTranslation", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_ICPOdometry_getMaxTranslation_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxTranslation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:843
// ("cv::rgbd::ICPOdometry::setMaxTranslation", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_ICPOdometry_setMaxTranslation_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getMaxRotation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:847
// ("cv::rgbd::ICPOdometry::getMaxRotation", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_ICPOdometry_getMaxRotation_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxRotation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:851
// ("cv::rgbd::ICPOdometry::setMaxRotation", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_ICPOdometry_setMaxRotation_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getNormalsComputer()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:855
// ("cv::rgbd::ICPOdometry::getNormalsComputer", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_ICPOdometry_getNormalsComputer_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::ICPOdometry::to_Algorithm() generated
// ("cv::rgbd::ICPOdometry::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_ICPOdometry_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::ICPOdometry::to_Odometry() generated
// ("cv::rgbd::ICPOdometry::to_Odometry", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_ICPOdometry_to_Odometry(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::ICPOdometry::delete() generated
// ("cv::rgbd::ICPOdometry::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_ICPOdometry_delete(instance: *mut c_void);
// DEFAULT_MIN_DEPTH()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:535
// ("cv::rgbd::Odometry::DEFAULT_MIN_DEPTH", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_Odometry_DEFAULT_MIN_DEPTH(ocvrs_return: *mut Result<f32>);
// DEFAULT_MAX_DEPTH()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:540
// ("cv::rgbd::Odometry::DEFAULT_MAX_DEPTH", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_Odometry_DEFAULT_MAX_DEPTH(ocvrs_return: *mut Result<f32>);
// DEFAULT_MAX_DEPTH_DIFF()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:545
// ("cv::rgbd::Odometry::DEFAULT_MAX_DEPTH_DIFF", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_Odometry_DEFAULT_MAX_DEPTH_DIFF(ocvrs_return: *mut Result<f32>);
// DEFAULT_MAX_POINTS_PART()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:550
// ("cv::rgbd::Odometry::DEFAULT_MAX_POINTS_PART", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_Odometry_DEFAULT_MAX_POINTS_PART(ocvrs_return: *mut Result<f32>);
// DEFAULT_MAX_TRANSLATION()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:555
// ("cv::rgbd::Odometry::DEFAULT_MAX_TRANSLATION", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_Odometry_DEFAULT_MAX_TRANSLATION(ocvrs_return: *mut Result<f32>);
// DEFAULT_MAX_ROTATION()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:560
// ("cv::rgbd::Odometry::DEFAULT_MAX_ROTATION", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_Odometry_DEFAULT_MAX_ROTATION(ocvrs_return: *mut Result<f32>);
// compute(const Mat &, const Mat &, const Mat &, const Mat &, const Mat &, const Mat &, OutputArray, const Mat &)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:584
// ("cv::rgbd::Odometry::compute", vec![(pred!(const, ["srcImage", "srcDepth", "srcMask", "dstImage", "dstDepth", "dstMask", "Rt", "initRt"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::_OutputArray*", "const cv::Mat*"]), _)]),
pub fn cv_rgbd_Odometry_compute_const_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const__OutputArrayR_const_MatR(instance: *const c_void, src_image: *const c_void, src_depth: *const c_void, src_mask: *const c_void, dst_image: *const c_void, dst_depth: *const c_void, dst_mask: *const c_void, rt: *const c_void, init_rt: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::rgbd::Odometry::compute(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:584
// ("cv::rgbd::Odometry::compute", vec![(pred!(const, ["srcImage", "srcDepth", "srcMask", "dstImage", "dstDepth", "dstMask", "Rt"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::_OutputArray*"]), _)]),
pub fn cv_rgbd_Odometry_compute_const_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const__OutputArrayR(instance: *const c_void, src_image: *const c_void, src_depth: *const c_void, src_mask: *const c_void, dst_image: *const c_void, dst_depth: *const c_void, dst_mask: *const c_void, rt: *const c_void, ocvrs_return: *mut Result<bool>);
// compute(Ptr<OdometryFrame> &, Ptr<OdometryFrame> &, OutputArray, const Mat &)(CppPassByVoidPtr, CppPassByVoidPtr, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:591
// ("cv::rgbd::Odometry::compute", vec![(pred!(const, ["srcFrame", "dstFrame", "Rt", "initRt"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "cv::Ptr<cv::rgbd::OdometryFrame>*", "const cv::_OutputArray*", "const cv::Mat*"]), _)]),
pub fn cv_rgbd_Odometry_compute_const_PtrLOdometryFrameGR_PtrLOdometryFrameGR_const__OutputArrayR_const_MatR(instance: *const c_void, src_frame: *mut c_void, dst_frame: *mut c_void, rt: *const c_void, init_rt: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::rgbd::Odometry::compute(CppPassByVoidPtr, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:591
// ("cv::rgbd::Odometry::compute", vec![(pred!(const, ["srcFrame", "dstFrame", "Rt"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "cv::Ptr<cv::rgbd::OdometryFrame>*", "const cv::_OutputArray*"]), _)]),
pub fn cv_rgbd_Odometry_compute_const_PtrLOdometryFrameGR_PtrLOdometryFrameGR_const__OutputArrayR(instance: *const c_void, src_frame: *mut c_void, dst_frame: *mut c_void, rt: *const c_void, ocvrs_return: *mut Result<bool>);
// prepareFrameCache(Ptr<OdometryFrame> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:599
// ("cv::rgbd::Odometry::prepareFrameCache", vec![(pred!(const, ["frame", "cacheType"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "int"]), _)]),
pub fn cv_rgbd_Odometry_prepareFrameCache_const_PtrLOdometryFrameGR_int(instance: *const c_void, frame: *mut c_void, cache_type: i32, ocvrs_return: *mut Result<core::Size>);
// create(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:601
// ("cv::rgbd::Odometry::create", vec![(pred!(mut, ["odometryType"], ["const cv::String*"]), _)]),
pub fn cv_rgbd_Odometry_create_const_StringR(odometry_type: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// getCameraMatrix()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:604
// ("cv::rgbd::Odometry::getCameraMatrix", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_Odometry_getCameraMatrix_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setCameraMatrix(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:606
// ("cv::rgbd::Odometry::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
pub fn cv_rgbd_Odometry_setCameraMatrix_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getTransformType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:608
// ("cv::rgbd::Odometry::getTransformType", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_Odometry_getTransformType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setTransformType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:610
// ("cv::rgbd::Odometry::setTransformType", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_rgbd_Odometry_setTransformType_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// cv::rgbd::Odometry::to_FastICPOdometry() generated
// ("cv::rgbd::Odometry::to_FastICPOdometry", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_Odometry_to_FastICPOdometry(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::Odometry::to_ICPOdometry() generated
// ("cv::rgbd::Odometry::to_ICPOdometry", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_Odometry_to_ICPOdometry(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::Odometry::to_RgbdICPOdometry() generated
// ("cv::rgbd::Odometry::to_RgbdICPOdometry", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_Odometry_to_RgbdICPOdometry(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::Odometry::to_RgbdOdometry() generated
// ("cv::rgbd::Odometry::to_RgbdOdometry", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_Odometry_to_RgbdOdometry(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::Odometry::to_Algorithm() generated
// ("cv::rgbd::Odometry::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_Odometry_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::Odometry::delete() generated
// ("cv::rgbd::Odometry::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_Odometry_delete(instance: *mut c_void);
// OdometryFrame()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:497
// ("cv::rgbd::OdometryFrame::OdometryFrame", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_OdometryFrame_OdometryFrame(ocvrs_return: *mut Result<*mut c_void>);
// OdometryFrame(const Mat &, const Mat &, const Mat &, const Mat &, int)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:498
// ("cv::rgbd::OdometryFrame::OdometryFrame", vec![(pred!(mut, ["image", "depth", "mask", "normals", "ID"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "int"]), _)]),
pub fn cv_rgbd_OdometryFrame_OdometryFrame_const_MatR_const_MatR_const_MatR_const_MatR_int(image: *const c_void, depth: *const c_void, mask: *const c_void, normals: *const c_void, id: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::OdometryFrame::OdometryFrame(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:498
// ("cv::rgbd::OdometryFrame::OdometryFrame", vec![(pred!(mut, ["image", "depth"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_rgbd_OdometryFrame_OdometryFrame_const_MatR_const_MatR(image: *const c_void, depth: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create(const Mat &, const Mat &, const Mat &, const Mat &, int)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:500
// ("cv::rgbd::OdometryFrame::create", vec![(pred!(mut, ["image", "depth", "mask", "normals", "ID"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "int"]), _)]),
pub fn cv_rgbd_OdometryFrame_create_const_MatR_const_MatR_const_MatR_const_MatR_int(image: *const c_void, depth: *const c_void, mask: *const c_void, normals: *const c_void, id: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::OdometryFrame::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:500
// ("cv::rgbd::OdometryFrame::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_OdometryFrame_create(ocvrs_return: *mut Result<*mut c_void>);
// release()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:503
// ("cv::rgbd::OdometryFrame::release", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_OdometryFrame_release(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// releasePyramids()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:506
// ("cv::rgbd::OdometryFrame::releasePyramids", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_OdometryFrame_releasePyramids(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::rgbd::OdometryFrame::pyramidImage() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:508
// ("cv::rgbd::OdometryFrame::pyramidImage", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramidImage_const(instance: *const c_void) -> *mut c_void;
// cv::rgbd::OdometryFrame::setPyramidImage(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:508
// ("cv::rgbd::OdometryFrame::setPyramidImage", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramidImage_const_vectorLMatG(instance: *mut c_void, val: *const c_void);
// cv::rgbd::OdometryFrame::pyramidDepth() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:509
// ("cv::rgbd::OdometryFrame::pyramidDepth", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramidDepth_const(instance: *const c_void) -> *mut c_void;
// cv::rgbd::OdometryFrame::setPyramidDepth(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:509
// ("cv::rgbd::OdometryFrame::setPyramidDepth", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramidDepth_const_vectorLMatG(instance: *mut c_void, val: *const c_void);
// cv::rgbd::OdometryFrame::pyramidMask() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:510
// ("cv::rgbd::OdometryFrame::pyramidMask", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramidMask_const(instance: *const c_void) -> *mut c_void;
// cv::rgbd::OdometryFrame::setPyramidMask(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:510
// ("cv::rgbd::OdometryFrame::setPyramidMask", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramidMask_const_vectorLMatG(instance: *mut c_void, val: *const c_void);
// cv::rgbd::OdometryFrame::pyramidCloud() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:512
// ("cv::rgbd::OdometryFrame::pyramidCloud", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramidCloud_const(instance: *const c_void) -> *mut c_void;
// cv::rgbd::OdometryFrame::setPyramidCloud(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:512
// ("cv::rgbd::OdometryFrame::setPyramidCloud", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramidCloud_const_vectorLMatG(instance: *mut c_void, val: *const c_void);
// cv::rgbd::OdometryFrame::pyramid_dI_dx() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:514
// ("cv::rgbd::OdometryFrame::pyramid_dI_dx", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramid_dI_dx_const(instance: *const c_void) -> *mut c_void;
// cv::rgbd::OdometryFrame::setPyramid_dI_dx(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:514
// ("cv::rgbd::OdometryFrame::setPyramid_dI_dx", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramid_dI_dx_const_vectorLMatG(instance: *mut c_void, val: *const c_void);
// cv::rgbd::OdometryFrame::pyramid_dI_dy() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:515
// ("cv::rgbd::OdometryFrame::pyramid_dI_dy", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramid_dI_dy_const(instance: *const c_void) -> *mut c_void;
// cv::rgbd::OdometryFrame::setPyramid_dI_dy(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:515
// ("cv::rgbd::OdometryFrame::setPyramid_dI_dy", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramid_dI_dy_const_vectorLMatG(instance: *mut c_void, val: *const c_void);
// cv::rgbd::OdometryFrame::pyramidTexturedMask() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:516
// ("cv::rgbd::OdometryFrame::pyramidTexturedMask", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramidTexturedMask_const(instance: *const c_void) -> *mut c_void;
// cv::rgbd::OdometryFrame::setPyramidTexturedMask(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:516
// ("cv::rgbd::OdometryFrame::setPyramidTexturedMask", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramidTexturedMask_const_vectorLMatG(instance: *mut c_void, val: *const c_void);
// cv::rgbd::OdometryFrame::pyramidNormals() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:518
// ("cv::rgbd::OdometryFrame::pyramidNormals", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramidNormals_const(instance: *const c_void) -> *mut c_void;
// cv::rgbd::OdometryFrame::setPyramidNormals(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:518
// ("cv::rgbd::OdometryFrame::setPyramidNormals", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramidNormals_const_vectorLMatG(instance: *mut c_void, val: *const c_void);
// cv::rgbd::OdometryFrame::pyramidNormalsMask() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:519
// ("cv::rgbd::OdometryFrame::pyramidNormalsMask", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramidNormalsMask_const(instance: *const c_void) -> *mut c_void;
// cv::rgbd::OdometryFrame::setPyramidNormalsMask(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:519
// ("cv::rgbd::OdometryFrame::setPyramidNormalsMask", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
pub fn cv_rgbd_OdometryFrame_propPyramidNormalsMask_const_vectorLMatG(instance: *mut c_void, val: *const c_void);
// cv::rgbd::OdometryFrame::to_RgbdFrame() generated
// ("cv::rgbd::OdometryFrame::to_RgbdFrame", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_OdometryFrame_to_RgbdFrame(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::OdometryFrame::delete() generated
// ("cv::rgbd::OdometryFrame::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_OdometryFrame_delete(instance: *mut c_void);
// RgbdFrame()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:463
// ("cv::rgbd::RgbdFrame::RgbdFrame", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdFrame_RgbdFrame(ocvrs_return: *mut Result<*mut c_void>);
// RgbdFrame(const Mat &, const Mat &, const Mat &, const Mat &, int)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:464
// ("cv::rgbd::RgbdFrame::RgbdFrame", vec![(pred!(mut, ["image", "depth", "mask", "normals", "ID"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "int"]), _)]),
pub fn cv_rgbd_RgbdFrame_RgbdFrame_const_MatR_const_MatR_const_MatR_const_MatR_int(image: *const c_void, depth: *const c_void, mask: *const c_void, normals: *const c_void, id: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::RgbdFrame::RgbdFrame(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:464
// ("cv::rgbd::RgbdFrame::RgbdFrame", vec![(pred!(mut, ["image", "depth"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_rgbd_RgbdFrame_RgbdFrame_const_MatR_const_MatR(image: *const c_void, depth: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create(const Mat &, const Mat &, const Mat &, const Mat &, int)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:467
// ("cv::rgbd::RgbdFrame::create", vec![(pred!(mut, ["image", "depth", "mask", "normals", "ID"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "int"]), _)]),
pub fn cv_rgbd_RgbdFrame_create_const_MatR_const_MatR_const_MatR_const_MatR_int(image: *const c_void, depth: *const c_void, mask: *const c_void, normals: *const c_void, id: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::RgbdFrame::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:467
// ("cv::rgbd::RgbdFrame::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdFrame_create(ocvrs_return: *mut Result<*mut c_void>);
// release()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:470
// ("cv::rgbd::RgbdFrame::release", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdFrame_release(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::rgbd::RgbdFrame::ID() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:472
// ("cv::rgbd::RgbdFrame::ID", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdFrame_propID_const(instance: *const c_void) -> i32;
// cv::rgbd::RgbdFrame::setID(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:472
// ("cv::rgbd::RgbdFrame::setID", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_rgbd_RgbdFrame_propID_const_int(instance: *mut c_void, val: i32);
// cv::rgbd::RgbdFrame::image() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:473
// ("cv::rgbd::RgbdFrame::image", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdFrame_propImage_const(instance: *const c_void) -> *mut c_void;
// cv::rgbd::RgbdFrame::setImage(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:473
// ("cv::rgbd::RgbdFrame::setImage", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_rgbd_RgbdFrame_propImage_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::rgbd::RgbdFrame::depth() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:474
// ("cv::rgbd::RgbdFrame::depth", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdFrame_propDepth_const(instance: *const c_void) -> *mut c_void;
// cv::rgbd::RgbdFrame::setDepth(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:474
// ("cv::rgbd::RgbdFrame::setDepth", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_rgbd_RgbdFrame_propDepth_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::rgbd::RgbdFrame::mask() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:475
// ("cv::rgbd::RgbdFrame::mask", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdFrame_propMask_const(instance: *const c_void) -> *mut c_void;
// cv::rgbd::RgbdFrame::setMask(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:475
// ("cv::rgbd::RgbdFrame::setMask", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_rgbd_RgbdFrame_propMask_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::rgbd::RgbdFrame::normals() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:476
// ("cv::rgbd::RgbdFrame::normals", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdFrame_propNormals_const(instance: *const c_void) -> *mut c_void;
// cv::rgbd::RgbdFrame::setNormals(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:476
// ("cv::rgbd::RgbdFrame::setNormals", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
pub fn cv_rgbd_RgbdFrame_propNormals_const_Mat(instance: *mut c_void, val: *const c_void);
// cv::rgbd::RgbdFrame::to_OdometryFrame() generated
// ("cv::rgbd::RgbdFrame::to_OdometryFrame", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdFrame_to_OdometryFrame(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::RgbdFrame::delete() generated
// ("cv::rgbd::RgbdFrame::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdFrame_delete(instance: *mut c_void);
// RgbdICPOdometry()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:890
// ("cv::rgbd::RgbdICPOdometry::RgbdICPOdometry", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_RgbdICPOdometry(ocvrs_return: *mut Result<*mut c_void>);
// RgbdICPOdometry(const Mat &, float, float, float, float, const std::vector<int> &, const std::vector<float> &, int)(TraitClass, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:903
// ("cv::rgbd::RgbdICPOdometry::RgbdICPOdometry", vec![(pred!(mut, ["cameraMatrix", "minDepth", "maxDepth", "maxDepthDiff", "maxPointsPart", "iterCounts", "minGradientMagnitudes", "transformType"], ["const cv::Mat*", "float", "float", "float", "float", "const std::vector<int>*", "const std::vector<float>*", "int"]), _)]),
pub fn cv_rgbd_RgbdICPOdometry_RgbdICPOdometry_const_MatR_float_float_float_float_const_vectorLintGR_const_vectorLfloatGR_int(camera_matrix: *const c_void, min_depth: f32, max_depth: f32, max_depth_diff: f32, max_points_part: f32, iter_counts: *const c_void, min_gradient_magnitudes: *const c_void, transform_type: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::RgbdICPOdometry::RgbdICPOdometry(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:903
// ("cv::rgbd::RgbdICPOdometry::RgbdICPOdometry", vec![(pred!(mut, ["cameraMatrix"], ["const cv::Mat*"]), _)]),
pub fn cv_rgbd_RgbdICPOdometry_RgbdICPOdometry_const_MatR(camera_matrix: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create(const Mat &, float, float, float, float, const std::vector<int> &, const std::vector<float> &, int)(TraitClass, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:909
// ("cv::rgbd::RgbdICPOdometry::create", vec![(pred!(mut, ["cameraMatrix", "minDepth", "maxDepth", "maxDepthDiff", "maxPointsPart", "iterCounts", "minGradientMagnitudes", "transformType"], ["const cv::Mat*", "float", "float", "float", "float", "const std::vector<int>*", "const std::vector<float>*", "int"]), _)]),
pub fn cv_rgbd_RgbdICPOdometry_create_const_MatR_float_float_float_float_const_vectorLintGR_const_vectorLfloatGR_int(camera_matrix: *const c_void, min_depth: f32, max_depth: f32, max_depth_diff: f32, max_points_part: f32, iter_counts: *const c_void, min_gradient_magnitudes: *const c_void, transform_type: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::RgbdICPOdometry::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:909
// ("cv::rgbd::RgbdICPOdometry::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_create(ocvrs_return: *mut Result<*mut c_void>);
// prepareFrameCache(Ptr<OdometryFrame> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:915
// ("cv::rgbd::RgbdICPOdometry::prepareFrameCache", vec![(pred!(const, ["frame", "cacheType"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "int"]), _)]),
pub fn cv_rgbd_RgbdICPOdometry_prepareFrameCache_const_PtrLOdometryFrameGR_int(instance: *const c_void, frame: *mut c_void, cache_type: i32, ocvrs_return: *mut Result<core::Size>);
// getCameraMatrix()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:917
// ("cv::rgbd::RgbdICPOdometry::getCameraMatrix", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_getCameraMatrix_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setCameraMatrix(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:921
// ("cv::rgbd::RgbdICPOdometry::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
pub fn cv_rgbd_RgbdICPOdometry_setCameraMatrix_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getMinDepth()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:925
// ("cv::rgbd::RgbdICPOdometry::getMinDepth", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_getMinDepth_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMinDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:929
// ("cv::rgbd::RgbdICPOdometry::setMinDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdICPOdometry_setMinDepth_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getMaxDepth()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:933
// ("cv::rgbd::RgbdICPOdometry::getMaxDepth", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_getMaxDepth_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:937
// ("cv::rgbd::RgbdICPOdometry::setMaxDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdICPOdometry_setMaxDepth_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getMaxDepthDiff()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:941
// ("cv::rgbd::RgbdICPOdometry::getMaxDepthDiff", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_getMaxDepthDiff_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxDepthDiff(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:945
// ("cv::rgbd::RgbdICPOdometry::setMaxDepthDiff", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdICPOdometry_setMaxDepthDiff_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getMaxPointsPart()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:949
// ("cv::rgbd::RgbdICPOdometry::getMaxPointsPart", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_getMaxPointsPart_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxPointsPart(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:953
// ("cv::rgbd::RgbdICPOdometry::setMaxPointsPart", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdICPOdometry_setMaxPointsPart_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getIterationCounts()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:957
// ("cv::rgbd::RgbdICPOdometry::getIterationCounts", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_getIterationCounts_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setIterationCounts(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:961
// ("cv::rgbd::RgbdICPOdometry::setIterationCounts", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
pub fn cv_rgbd_RgbdICPOdometry_setIterationCounts_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getMinGradientMagnitudes()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:965
// ("cv::rgbd::RgbdICPOdometry::getMinGradientMagnitudes", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_getMinGradientMagnitudes_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setMinGradientMagnitudes(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:969
// ("cv::rgbd::RgbdICPOdometry::setMinGradientMagnitudes", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
pub fn cv_rgbd_RgbdICPOdometry_setMinGradientMagnitudes_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getTransformType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:973
// ("cv::rgbd::RgbdICPOdometry::getTransformType", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_getTransformType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setTransformType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:977
// ("cv::rgbd::RgbdICPOdometry::setTransformType", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_rgbd_RgbdICPOdometry_setTransformType_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getMaxTranslation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:981
// ("cv::rgbd::RgbdICPOdometry::getMaxTranslation", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_getMaxTranslation_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxTranslation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:985
// ("cv::rgbd::RgbdICPOdometry::setMaxTranslation", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdICPOdometry_setMaxTranslation_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getMaxRotation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:989
// ("cv::rgbd::RgbdICPOdometry::getMaxRotation", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_getMaxRotation_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxRotation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:993
// ("cv::rgbd::RgbdICPOdometry::setMaxRotation", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdICPOdometry_setMaxRotation_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getNormalsComputer()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:997
// ("cv::rgbd::RgbdICPOdometry::getNormalsComputer", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_getNormalsComputer_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::RgbdICPOdometry::to_Algorithm() generated
// ("cv::rgbd::RgbdICPOdometry::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::RgbdICPOdometry::to_Odometry() generated
// ("cv::rgbd::RgbdICPOdometry::to_Odometry", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_to_Odometry(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::RgbdICPOdometry::delete() generated
// ("cv::rgbd::RgbdICPOdometry::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdICPOdometry_delete(instance: *mut c_void);
// RgbdNormals()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:83
// ("cv::rgbd::RgbdNormals::RgbdNormals", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdNormals_RgbdNormals(ocvrs_return: *mut Result<*mut c_void>);
// RgbdNormals(int, int, int, InputArray, int, int)(Primitive, Primitive, Primitive, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:103
// ("cv::rgbd::RgbdNormals::RgbdNormals", vec![(pred!(mut, ["rows", "cols", "depth", "K", "window_size", "method"], ["int", "int", "int", "const cv::_InputArray*", "int", "int"]), _)]),
pub fn cv_rgbd_RgbdNormals_RgbdNormals_int_int_int_const__InputArrayR_int_int(rows: i32, cols: i32, depth: i32, k: *const c_void, window_size: i32, method: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::RgbdNormals::RgbdNormals(Primitive, Primitive, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:103
// ("cv::rgbd::RgbdNormals::RgbdNormals", vec![(pred!(mut, ["rows", "cols", "depth", "K"], ["int", "int", "int", "const cv::_InputArray*"]), _)]),
pub fn cv_rgbd_RgbdNormals_RgbdNormals_int_int_int_const__InputArrayR(rows: i32, cols: i32, depth: i32, k: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create(int, int, int, InputArray, int, int)(Primitive, Primitive, Primitive, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:108
// ("cv::rgbd::RgbdNormals::create", vec![(pred!(mut, ["rows", "cols", "depth", "K", "window_size", "method"], ["int", "int", "int", "const cv::_InputArray*", "int", "int"]), _)]),
pub fn cv_rgbd_RgbdNormals_create_int_int_int_const__InputArrayR_int_int(rows: i32, cols: i32, depth: i32, k: *const c_void, window_size: i32, method: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::RgbdNormals::create(Primitive, Primitive, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:108
// ("cv::rgbd::RgbdNormals::create", vec![(pred!(mut, ["rows", "cols", "depth", "K"], ["int", "int", "int", "const cv::_InputArray*"]), _)]),
pub fn cv_rgbd_RgbdNormals_create_int_int_int_const__InputArrayR(rows: i32, cols: i32, depth: i32, k: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator()(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:116
// ("cv::rgbd::RgbdNormals::operator()", vec![(pred!(const, ["points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_rgbd_RgbdNormals_operator___const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<()>);
// initialize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:122
// ("cv::rgbd::RgbdNormals::initialize", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdNormals_initialize_const(instance: *const c_void, ocvrs_return: *mut Result<()>);
// getRows()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:124
// ("cv::rgbd::RgbdNormals::getRows", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdNormals_getRows_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setRows(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:128
// ("cv::rgbd::RgbdNormals::setRows", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_rgbd_RgbdNormals_setRows_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getCols()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:132
// ("cv::rgbd::RgbdNormals::getCols", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdNormals_getCols_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setCols(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:136
// ("cv::rgbd::RgbdNormals::setCols", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_rgbd_RgbdNormals_setCols_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getWindowSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:140
// ("cv::rgbd::RgbdNormals::getWindowSize", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdNormals_getWindowSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setWindowSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:144
// ("cv::rgbd::RgbdNormals::setWindowSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_rgbd_RgbdNormals_setWindowSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getDepth()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:148
// ("cv::rgbd::RgbdNormals::getDepth", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdNormals_getDepth_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setDepth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:152
// ("cv::rgbd::RgbdNormals::setDepth", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_rgbd_RgbdNormals_setDepth_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getK()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:156
// ("cv::rgbd::RgbdNormals::getK", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdNormals_getK_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setK(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:160
// ("cv::rgbd::RgbdNormals::setK", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
pub fn cv_rgbd_RgbdNormals_setK_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getMethod()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:164
// ("cv::rgbd::RgbdNormals::getMethod", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdNormals_getMethod_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMethod(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:168
// ("cv::rgbd::RgbdNormals::setMethod", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_rgbd_RgbdNormals_setMethod_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// cv::rgbd::RgbdNormals::to_Algorithm() generated
// ("cv::rgbd::RgbdNormals::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdNormals_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::RgbdNormals::delete() generated
// ("cv::rgbd::RgbdNormals::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdNormals_delete(instance: *mut c_void);
// RgbdOdometry()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:627
// ("cv::rgbd::RgbdOdometry::RgbdOdometry", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdOdometry_RgbdOdometry(ocvrs_return: *mut Result<*mut c_void>);
// RgbdOdometry(const Mat &, float, float, float, const std::vector<int> &, const std::vector<float> &, float, int)(TraitClass, Primitive, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:640
// ("cv::rgbd::RgbdOdometry::RgbdOdometry", vec![(pred!(mut, ["cameraMatrix", "minDepth", "maxDepth", "maxDepthDiff", "iterCounts", "minGradientMagnitudes", "maxPointsPart", "transformType"], ["const cv::Mat*", "float", "float", "float", "const std::vector<int>*", "const std::vector<float>*", "float", "int"]), _)]),
pub fn cv_rgbd_RgbdOdometry_RgbdOdometry_const_MatR_float_float_float_const_vectorLintGR_const_vectorLfloatGR_float_int(camera_matrix: *const c_void, min_depth: f32, max_depth: f32, max_depth_diff: f32, iter_counts: *const c_void, min_gradient_magnitudes: *const c_void, max_points_part: f32, transform_type: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::RgbdOdometry::RgbdOdometry(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:640
// ("cv::rgbd::RgbdOdometry::RgbdOdometry", vec![(pred!(mut, ["cameraMatrix"], ["const cv::Mat*"]), _)]),
pub fn cv_rgbd_RgbdOdometry_RgbdOdometry_const_MatR(camera_matrix: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create(const Mat &, float, float, float, const std::vector<int> &, const std::vector<float> &, float, int)(TraitClass, Primitive, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:645
// ("cv::rgbd::RgbdOdometry::create", vec![(pred!(mut, ["cameraMatrix", "minDepth", "maxDepth", "maxDepthDiff", "iterCounts", "minGradientMagnitudes", "maxPointsPart", "transformType"], ["const cv::Mat*", "float", "float", "float", "const std::vector<int>*", "const std::vector<float>*", "float", "int"]), _)]),
pub fn cv_rgbd_RgbdOdometry_create_const_MatR_float_float_float_const_vectorLintGR_const_vectorLfloatGR_float_int(camera_matrix: *const c_void, min_depth: f32, max_depth: f32, max_depth_diff: f32, iter_counts: *const c_void, min_gradient_magnitudes: *const c_void, max_points_part: f32, transform_type: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::RgbdOdometry::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:645
// ("cv::rgbd::RgbdOdometry::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdOdometry_create(ocvrs_return: *mut Result<*mut c_void>);
// prepareFrameCache(Ptr<OdometryFrame> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:650
// ("cv::rgbd::RgbdOdometry::prepareFrameCache", vec![(pred!(const, ["frame", "cacheType"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "int"]), _)]),
pub fn cv_rgbd_RgbdOdometry_prepareFrameCache_const_PtrLOdometryFrameGR_int(instance: *const c_void, frame: *mut c_void, cache_type: i32, ocvrs_return: *mut Result<core::Size>);
// getCameraMatrix()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:652
// ("cv::rgbd::RgbdOdometry::getCameraMatrix", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdOdometry_getCameraMatrix_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setCameraMatrix(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:656
// ("cv::rgbd::RgbdOdometry::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
pub fn cv_rgbd_RgbdOdometry_setCameraMatrix_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getMinDepth()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:660
// ("cv::rgbd::RgbdOdometry::getMinDepth", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdOdometry_getMinDepth_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMinDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:664
// ("cv::rgbd::RgbdOdometry::setMinDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdOdometry_setMinDepth_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getMaxDepth()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:668
// ("cv::rgbd::RgbdOdometry::getMaxDepth", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdOdometry_getMaxDepth_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:672
// ("cv::rgbd::RgbdOdometry::setMaxDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdOdometry_setMaxDepth_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getMaxDepthDiff()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:676
// ("cv::rgbd::RgbdOdometry::getMaxDepthDiff", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdOdometry_getMaxDepthDiff_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxDepthDiff(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:680
// ("cv::rgbd::RgbdOdometry::setMaxDepthDiff", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdOdometry_setMaxDepthDiff_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getIterationCounts()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:684
// ("cv::rgbd::RgbdOdometry::getIterationCounts", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdOdometry_getIterationCounts_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setIterationCounts(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:688
// ("cv::rgbd::RgbdOdometry::setIterationCounts", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
pub fn cv_rgbd_RgbdOdometry_setIterationCounts_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getMinGradientMagnitudes()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:692
// ("cv::rgbd::RgbdOdometry::getMinGradientMagnitudes", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdOdometry_getMinGradientMagnitudes_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setMinGradientMagnitudes(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:696
// ("cv::rgbd::RgbdOdometry::setMinGradientMagnitudes", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
pub fn cv_rgbd_RgbdOdometry_setMinGradientMagnitudes_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getMaxPointsPart()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:700
// ("cv::rgbd::RgbdOdometry::getMaxPointsPart", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdOdometry_getMaxPointsPart_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxPointsPart(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:704
// ("cv::rgbd::RgbdOdometry::setMaxPointsPart", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdOdometry_setMaxPointsPart_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getTransformType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:708
// ("cv::rgbd::RgbdOdometry::getTransformType", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdOdometry_getTransformType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setTransformType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:712
// ("cv::rgbd::RgbdOdometry::setTransformType", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_rgbd_RgbdOdometry_setTransformType_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getMaxTranslation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:716
// ("cv::rgbd::RgbdOdometry::getMaxTranslation", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdOdometry_getMaxTranslation_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxTranslation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:720
// ("cv::rgbd::RgbdOdometry::setMaxTranslation", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdOdometry_setMaxTranslation_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getMaxRotation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:724
// ("cv::rgbd::RgbdOdometry::getMaxRotation", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdOdometry_getMaxRotation_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxRotation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:728
// ("cv::rgbd::RgbdOdometry::setMaxRotation", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdOdometry_setMaxRotation_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// cv::rgbd::RgbdOdometry::to_Algorithm() generated
// ("cv::rgbd::RgbdOdometry::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdOdometry_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::RgbdOdometry::to_Odometry() generated
// ("cv::rgbd::RgbdOdometry::to_Odometry", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdOdometry_to_Odometry(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::RgbdOdometry::delete() generated
// ("cv::rgbd::RgbdOdometry::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdOdometry_delete(instance: *mut c_void);
// RgbdPlane(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:337
// ("cv::rgbd::RgbdPlane::RgbdPlane", vec![(pred!(mut, ["method"], ["int"]), _)]),
pub fn cv_rgbd_RgbdPlane_RgbdPlane_int(method: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::RgbdPlane::RgbdPlane() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:337
// ("cv::rgbd::RgbdPlane::RgbdPlane", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdPlane_RgbdPlane(ocvrs_return: *mut Result<*mut c_void>);
// RgbdPlane(int, int, int, double, double, double, double)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:358
// ("cv::rgbd::RgbdPlane::RgbdPlane", vec![(pred!(mut, ["method", "block_size", "min_size", "threshold", "sensor_error_a", "sensor_error_b", "sensor_error_c"], ["int", "int", "int", "double", "double", "double", "double"]), _)]),
pub fn cv_rgbd_RgbdPlane_RgbdPlane_int_int_int_double_double_double_double(method: i32, block_size: i32, min_size: i32, threshold: f64, sensor_error_a: f64, sensor_error_b: f64, sensor_error_c: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::RgbdPlane::RgbdPlane(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:358
// ("cv::rgbd::RgbdPlane::RgbdPlane", vec![(pred!(mut, ["method", "block_size", "min_size", "threshold"], ["int", "int", "int", "double"]), _)]),
pub fn cv_rgbd_RgbdPlane_RgbdPlane_int_int_int_double(method: i32, block_size: i32, min_size: i32, threshold: f64, ocvrs_return: *mut Result<*mut c_void>);
// create(int, int, int, double, double, double, double)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:364
// ("cv::rgbd::RgbdPlane::create", vec![(pred!(mut, ["method", "block_size", "min_size", "threshold", "sensor_error_a", "sensor_error_b", "sensor_error_c"], ["int", "int", "int", "double", "double", "double", "double"]), _)]),
pub fn cv_rgbd_RgbdPlane_create_int_int_int_double_double_double_double(method: i32, block_size: i32, min_size: i32, threshold: f64, sensor_error_a: f64, sensor_error_b: f64, sensor_error_c: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::rgbd::RgbdPlane::create(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:364
// ("cv::rgbd::RgbdPlane::create", vec![(pred!(mut, ["method", "block_size", "min_size", "threshold"], ["int", "int", "int", "double"]), _)]),
pub fn cv_rgbd_RgbdPlane_create_int_int_int_double(method: i32, block_size: i32, min_size: i32, threshold: f64, ocvrs_return: *mut Result<*mut c_void>);
// operator()(InputArray, InputArray, OutputArray, OutputArray)(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:377
// ("cv::rgbd::RgbdPlane::operator()", vec![(pred!(mut, ["points3d", "normals", "mask", "plane_coefficients"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_rgbd_RgbdPlane_operator___const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, points3d: *const c_void, normals: *const c_void, mask: *const c_void, plane_coefficients: *const c_void, ocvrs_return: *mut Result<()>);
// operator()(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:387
// ("cv::rgbd::RgbdPlane::operator()", vec![(pred!(mut, ["points3d", "mask", "plane_coefficients"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_rgbd_RgbdPlane_operator___const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, points3d: *const c_void, mask: *const c_void, plane_coefficients: *const c_void, ocvrs_return: *mut Result<()>);
// getBlockSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:389
// ("cv::rgbd::RgbdPlane::getBlockSize", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdPlane_getBlockSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setBlockSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:393
// ("cv::rgbd::RgbdPlane::setBlockSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_rgbd_RgbdPlane_setBlockSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getMinSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:397
// ("cv::rgbd::RgbdPlane::getMinSize", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdPlane_getMinSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMinSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:401
// ("cv::rgbd::RgbdPlane::setMinSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_rgbd_RgbdPlane_setMinSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getMethod()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:405
// ("cv::rgbd::RgbdPlane::getMethod", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdPlane_getMethod_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMethod(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:409
// ("cv::rgbd::RgbdPlane::setMethod", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_rgbd_RgbdPlane_setMethod_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:413
// ("cv::rgbd::RgbdPlane::getThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdPlane_getThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:417
// ("cv::rgbd::RgbdPlane::setThreshold", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdPlane_setThreshold_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getSensorErrorA()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:421
// ("cv::rgbd::RgbdPlane::getSensorErrorA", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdPlane_getSensorErrorA_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setSensorErrorA(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:425
// ("cv::rgbd::RgbdPlane::setSensorErrorA", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdPlane_setSensorErrorA_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getSensorErrorB()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:429
// ("cv::rgbd::RgbdPlane::getSensorErrorB", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdPlane_getSensorErrorB_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setSensorErrorB(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:433
// ("cv::rgbd::RgbdPlane::setSensorErrorB", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdPlane_setSensorErrorB_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getSensorErrorC()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:437
// ("cv::rgbd::RgbdPlane::getSensorErrorC", vec![(pred!(const, [], []), _)]),
pub fn cv_rgbd_RgbdPlane_getSensorErrorC_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setSensorErrorC(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:441
// ("cv::rgbd::RgbdPlane::setSensorErrorC", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_rgbd_RgbdPlane_setSensorErrorC_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// cv::rgbd::RgbdPlane::to_Algorithm() generated
// ("cv::rgbd::RgbdPlane::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdPlane_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::rgbd::RgbdPlane::delete() generated
// ("cv::rgbd::RgbdPlane::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_rgbd_RgbdPlane_delete(instance: *mut c_void);
