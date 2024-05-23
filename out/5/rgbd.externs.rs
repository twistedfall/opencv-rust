// colormap(const Mat &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:245
// ("cv::linemod::colormap", vec![(pred!(mut, ["quantized", "dst"], ["const cv::Mat*", "cv::Mat*"]), _)]),
pub fn cv_linemod_colormap_const_MatR_MatR(quantized: *const c_void, dst: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::linemod::drawFeatures(InputOutputArray, CppPassByVoidPtr, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:254
// ("cv::linemod::drawFeatures", vec![(pred!(mut, ["img", "templates", "tl"], ["const cv::_InputOutputArray*", "const std::vector<cv::linemod::Template>*", "const cv::Point2i*"]), _)]),
pub fn cv_linemod_drawFeatures_const__InputOutputArrayR_const_vectorLTemplateGR_const_Point2iR(img: *const c_void, templates: *const c_void, tl: *const core::Point2i, ocvrs_return: *mut Result<()>);
// drawFeatures(InputOutputArray, const std::vector<Template> &, const Point2i &, int)(InputOutputArray, CppPassByVoidPtr, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:254
// ("cv::linemod::drawFeatures", vec![(pred!(mut, ["img", "templates", "tl", "size"], ["const cv::_InputOutputArray*", "const std::vector<cv::linemod::Template>*", "const cv::Point2i*", "int"]), _)]),
pub fn cv_linemod_drawFeatures_const__InputOutputArrayR_const_vectorLTemplateGR_const_Point2iR_int(img: *const c_void, templates: *const c_void, tl: *const core::Point2i, size: i32, ocvrs_return: *mut Result<()>);
// getDefaultLINE()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:420
// ("cv::linemod::getDefaultLINE", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_getDefaultLINE(ocvrs_return: *mut Result<*mut c_void>);
// getDefaultLINEMOD()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:428
// ("cv::linemod::getDefaultLINEMOD", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_getDefaultLINEMOD(ocvrs_return: *mut Result<*mut c_void>);
// create(const Ptr<Params> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:200
// ("cv::colored_kinfu::ColoredKinFu::create", vec![(pred!(mut, ["_params"], ["const cv::Ptr<cv::colored_kinfu::Params>*"]), _)]),
pub fn cv_colored_kinfu_ColoredKinFu_create_const_PtrLParamsGR(_params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:204
// ("cv::colored_kinfu::ColoredKinFu::getParams", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_ColoredKinFu_getParams_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// render(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:214
// ("cv::colored_kinfu::ColoredKinFu::render", vec![(pred!(const, ["image"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_colored_kinfu_ColoredKinFu_render_const_const__OutputArrayR(instance: *const c_void, image: *const c_void, ocvrs_return: *mut Result<()>);
// render(OutputArray, const Matx44f &)(OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:226
// ("cv::colored_kinfu::ColoredKinFu::render", vec![(pred!(const, ["image", "cameraPose"], ["const cv::_OutputArray*", "const cv::Matx44f*"]), _)]),
pub fn cv_colored_kinfu_ColoredKinFu_render_const_const__OutputArrayR_const_Matx44fR(instance: *const c_void, image: *const c_void, camera_pose: *const core::Matx44f, ocvrs_return: *mut Result<()>);
// getCloud(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:236
// ("cv::colored_kinfu::ColoredKinFu::getCloud", vec![(pred!(const, ["points", "normals"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_colored_kinfu_ColoredKinFu_getCloud_const_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<()>);
// getPoints(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:244
// ("cv::colored_kinfu::ColoredKinFu::getPoints", vec![(pred!(const, ["points"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_colored_kinfu_ColoredKinFu_getPoints_const_const__OutputArrayR(instance: *const c_void, points: *const c_void, ocvrs_return: *mut Result<()>);
// getNormals(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:250
// ("cv::colored_kinfu::ColoredKinFu::getNormals", vec![(pred!(const, ["points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_colored_kinfu_ColoredKinFu_getNormals_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<()>);
// reset()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:256
// ("cv::colored_kinfu::ColoredKinFu::reset", vec![(pred!(mut, [], []), _)]),
pub fn cv_colored_kinfu_ColoredKinFu_reset(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// getPose()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:259
// ("cv::colored_kinfu::ColoredKinFu::getPose", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_ColoredKinFu_getPose_const(instance: *const c_void, ocvrs_return: *mut Result<core::Affine3f>);
// update(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:267
// ("cv::colored_kinfu::ColoredKinFu::update", vec![(pred!(mut, ["depth", "rgb"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_colored_kinfu_ColoredKinFu_update_const__InputArrayR_const__InputArrayR(instance: *mut c_void, depth: *const c_void, rgb: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::colored_kinfu::ColoredKinFu::delete() generated
// ("cv::colored_kinfu::ColoredKinFu::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_colored_kinfu_ColoredKinFu_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:21
// ("cv::colored_kinfu::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_colored_kinfu_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
// Params(Matx33f, Vec3f)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:32
// ("cv::colored_kinfu::Params::Params", vec![(pred!(mut, ["volumeInitialPoseRot", "volumeInitialPoseTransl"], ["cv::Matx33f", "cv::Vec3f"]), _)]),
pub fn cv_colored_kinfu_Params_Params_Matx33f_Vec3f(volume_initial_pose_rot: *const core::Matx33f, volume_initial_pose_transl: *const core::Vec3f, ocvrs_return: *mut Result<*mut c_void>);
// Params(Matx44f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:42
// ("cv::colored_kinfu::Params::Params", vec![(pred!(mut, ["volumeInitialPose"], ["cv::Matx44f"]), _)]),
pub fn cv_colored_kinfu_Params_Params_Matx44f(volume_initial_pose: *const core::Matx44f, ocvrs_return: *mut Result<*mut c_void>);
// setInitialVolumePose(Matx33f, Vec3f)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:53
// ("cv::colored_kinfu::Params::setInitialVolumePose", vec![(pred!(mut, ["R", "t"], ["cv::Matx33f", "cv::Vec3f"]), _)]),
pub fn cv_colored_kinfu_Params_setInitialVolumePose_Matx33f_Vec3f(instance: *mut c_void, r: *const core::Matx33f, t: *const core::Vec3f, ocvrs_return: *mut Result<()>);
// setInitialVolumePose(Matx44f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:60
// ("cv::colored_kinfu::Params::setInitialVolumePose", vec![(pred!(mut, ["homogen_tf"], ["cv::Matx44f"]), _)]),
pub fn cv_colored_kinfu_Params_setInitialVolumePose_Matx44f(instance: *mut c_void, homogen_tf: *const core::Matx44f, ocvrs_return: *mut Result<()>);
// defaultParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:66
// ("cv::colored_kinfu::Params::defaultParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_colored_kinfu_Params_defaultParams(ocvrs_return: *mut Result<*mut c_void>);
// coarseParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:72
// ("cv::colored_kinfu::Params::coarseParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_colored_kinfu_Params_coarseParams(ocvrs_return: *mut Result<*mut c_void>);
// hashTSDFParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:77
// ("cv::colored_kinfu::Params::hashTSDFParams", vec![(pred!(mut, ["isCoarse"], ["bool"]), _)]),
pub fn cv_colored_kinfu_Params_hashTSDFParams_bool(is_coarse: bool, ocvrs_return: *mut Result<*mut c_void>);
// coloredTSDFParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:82
// ("cv::colored_kinfu::Params::coloredTSDFParams", vec![(pred!(mut, ["isCoarse"], ["bool"]), _)]),
pub fn cv_colored_kinfu_Params_coloredTSDFParams_bool(is_coarse: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::colored_kinfu::Params::frameSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:85
// ("cv::colored_kinfu::Params::frameSize", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propFrameSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::colored_kinfu::Params::setFrameSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:85
// ("cv::colored_kinfu::Params::setFrameSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_colored_kinfu_Params_propFrameSize_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::colored_kinfu::Params::rgb_frameSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:88
// ("cv::colored_kinfu::Params::rgb_frameSize", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propRgb_frameSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::colored_kinfu::Params::setRgb_frameSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:88
// ("cv::colored_kinfu::Params::setRgb_frameSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_colored_kinfu_Params_propRgb_frameSize_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::colored_kinfu::Params::volumeKind() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:90
// ("cv::colored_kinfu::Params::volumeKind", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propVolumeKind_const(instance: *const c_void, ocvrs_return: *mut crate::mod_3d::VolumeType);
// cv::colored_kinfu::Params::setVolumeKind(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:90
// ("cv::colored_kinfu::Params::setVolumeKind", vec![(pred!(mut, ["val"], ["const cv::VolumeType"]), _)]),
pub fn cv_colored_kinfu_Params_propVolumeKind_const_VolumeType(instance: *mut c_void, val: crate::mod_3d::VolumeType);
// cv::colored_kinfu::Params::intr() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:93
// ("cv::colored_kinfu::Params::intr", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propIntr_const(instance: *const c_void, ocvrs_return: *mut core::Matx33f);
// cv::colored_kinfu::Params::setIntr(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:93
// ("cv::colored_kinfu::Params::setIntr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
pub fn cv_colored_kinfu_Params_propIntr_const_Matx33f(instance: *mut c_void, val: *const core::Matx33f);
// cv::colored_kinfu::Params::rgb_intr() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:96
// ("cv::colored_kinfu::Params::rgb_intr", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propRgb_intr_const(instance: *const c_void, ocvrs_return: *mut core::Matx33f);
// cv::colored_kinfu::Params::setRgb_intr(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:96
// ("cv::colored_kinfu::Params::setRgb_intr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
pub fn cv_colored_kinfu_Params_propRgb_intr_const_Matx33f(instance: *mut c_void, val: *const core::Matx33f);
// cv::colored_kinfu::Params::depthFactor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:105
// ("cv::colored_kinfu::Params::depthFactor", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propDepthFactor_const(instance: *const c_void) -> f32;
// cv::colored_kinfu::Params::setDepthFactor(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:105
// ("cv::colored_kinfu::Params::setDepthFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_colored_kinfu_Params_propDepthFactor_const_float(instance: *mut c_void, val: f32);
// cv::colored_kinfu::Params::bilateral_sigma_depth() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:108
// ("cv::colored_kinfu::Params::bilateral_sigma_depth", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propBilateral_sigma_depth_const(instance: *const c_void) -> f32;
// cv::colored_kinfu::Params::setBilateral_sigma_depth(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:108
// ("cv::colored_kinfu::Params::setBilateral_sigma_depth", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_colored_kinfu_Params_propBilateral_sigma_depth_const_float(instance: *mut c_void, val: f32);
// cv::colored_kinfu::Params::bilateral_sigma_spatial() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:110
// ("cv::colored_kinfu::Params::bilateral_sigma_spatial", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propBilateral_sigma_spatial_const(instance: *const c_void) -> f32;
// cv::colored_kinfu::Params::setBilateral_sigma_spatial(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:110
// ("cv::colored_kinfu::Params::setBilateral_sigma_spatial", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_colored_kinfu_Params_propBilateral_sigma_spatial_const_float(instance: *mut c_void, val: f32);
// cv::colored_kinfu::Params::bilateral_kernel_size() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:112
// ("cv::colored_kinfu::Params::bilateral_kernel_size", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propBilateral_kernel_size_const(instance: *const c_void) -> i32;
// cv::colored_kinfu::Params::setBilateral_kernel_size(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:112
// ("cv::colored_kinfu::Params::setBilateral_kernel_size", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_colored_kinfu_Params_propBilateral_kernel_size_const_int(instance: *mut c_void, val: i32);
// cv::colored_kinfu::Params::pyramidLevels() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:115
// ("cv::colored_kinfu::Params::pyramidLevels", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propPyramidLevels_const(instance: *const c_void) -> i32;
// cv::colored_kinfu::Params::setPyramidLevels(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:115
// ("cv::colored_kinfu::Params::setPyramidLevels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_colored_kinfu_Params_propPyramidLevels_const_int(instance: *mut c_void, val: i32);
// cv::colored_kinfu::Params::volumeDims() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:121
// ("cv::colored_kinfu::Params::volumeDims", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propVolumeDims_const(instance: *const c_void, ocvrs_return: *mut core::Vec3i);
// cv::colored_kinfu::Params::setVolumeDims(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:121
// ("cv::colored_kinfu::Params::setVolumeDims", vec![(pred!(mut, ["val"], ["const cv::Vec3i"]), _)]),
pub fn cv_colored_kinfu_Params_propVolumeDims_const_Vec3i(instance: *mut c_void, val: *const core::Vec3i);
// cv::colored_kinfu::Params::voxelSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:123
// ("cv::colored_kinfu::Params::voxelSize", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propVoxelSize_const(instance: *const c_void) -> f32;
// cv::colored_kinfu::Params::setVoxelSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:123
// ("cv::colored_kinfu::Params::setVoxelSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_colored_kinfu_Params_propVoxelSize_const_float(instance: *mut c_void, val: f32);
// cv::colored_kinfu::Params::tsdf_min_camera_movement() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:129
// ("cv::colored_kinfu::Params::tsdf_min_camera_movement", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propTsdf_min_camera_movement_const(instance: *const c_void) -> f32;
// cv::colored_kinfu::Params::setTsdf_min_camera_movement(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:129
// ("cv::colored_kinfu::Params::setTsdf_min_camera_movement", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_colored_kinfu_Params_propTsdf_min_camera_movement_const_float(instance: *mut c_void, val: f32);
// cv::colored_kinfu::Params::volumePose() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:132
// ("cv::colored_kinfu::Params::volumePose", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propVolumePose_const(instance: *const c_void, ocvrs_return: *mut core::Matx44f);
// cv::colored_kinfu::Params::setVolumePose(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:132
// ("cv::colored_kinfu::Params::setVolumePose", vec![(pred!(mut, ["val"], ["const cv::Matx44f"]), _)]),
pub fn cv_colored_kinfu_Params_propVolumePose_const_Matx44f(instance: *mut c_void, val: *const core::Matx44f);
// cv::colored_kinfu::Params::tsdf_trunc_dist() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:138
// ("cv::colored_kinfu::Params::tsdf_trunc_dist", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propTsdf_trunc_dist_const(instance: *const c_void) -> f32;
// cv::colored_kinfu::Params::setTsdf_trunc_dist(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:138
// ("cv::colored_kinfu::Params::setTsdf_trunc_dist", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_colored_kinfu_Params_propTsdf_trunc_dist_const_float(instance: *mut c_void, val: f32);
// cv::colored_kinfu::Params::tsdf_max_weight() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:144
// ("cv::colored_kinfu::Params::tsdf_max_weight", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propTsdf_max_weight_const(instance: *const c_void) -> i32;
// cv::colored_kinfu::Params::setTsdf_max_weight(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:144
// ("cv::colored_kinfu::Params::setTsdf_max_weight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_colored_kinfu_Params_propTsdf_max_weight_const_int(instance: *mut c_void, val: i32);
// cv::colored_kinfu::Params::raycast_step_factor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:150
// ("cv::colored_kinfu::Params::raycast_step_factor", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propRaycast_step_factor_const(instance: *const c_void) -> f32;
// cv::colored_kinfu::Params::setRaycast_step_factor(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:150
// ("cv::colored_kinfu::Params::setRaycast_step_factor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_colored_kinfu_Params_propRaycast_step_factor_const_float(instance: *mut c_void, val: f32);
// cv::colored_kinfu::Params::lightPose() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:157
// ("cv::colored_kinfu::Params::lightPose", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propLightPose_const(instance: *const c_void, ocvrs_return: *mut core::Vec3f);
// cv::colored_kinfu::Params::setLightPose(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:157
// ("cv::colored_kinfu::Params::setLightPose", vec![(pred!(mut, ["val"], ["const cv::Vec3f"]), _)]),
pub fn cv_colored_kinfu_Params_propLightPose_const_Vec3f(instance: *mut c_void, val: *const core::Vec3f);
// cv::colored_kinfu::Params::icpDistThresh() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:160
// ("cv::colored_kinfu::Params::icpDistThresh", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propIcpDistThresh_const(instance: *const c_void) -> f32;
// cv::colored_kinfu::Params::setIcpDistThresh(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:160
// ("cv::colored_kinfu::Params::setIcpDistThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_colored_kinfu_Params_propIcpDistThresh_const_float(instance: *mut c_void, val: f32);
// cv::colored_kinfu::Params::icpAngleThresh() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:162
// ("cv::colored_kinfu::Params::icpAngleThresh", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propIcpAngleThresh_const(instance: *const c_void) -> f32;
// cv::colored_kinfu::Params::setIcpAngleThresh(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:162
// ("cv::colored_kinfu::Params::setIcpAngleThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_colored_kinfu_Params_propIcpAngleThresh_const_float(instance: *mut c_void, val: f32);
// cv::colored_kinfu::Params::icpIterations() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:164
// ("cv::colored_kinfu::Params::icpIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propIcpIterations_const(instance: *const c_void) -> *mut c_void;
// cv::colored_kinfu::Params::setIcpIterations(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:164
// ("cv::colored_kinfu::Params::setIcpIterations", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
pub fn cv_colored_kinfu_Params_propIcpIterations_const_vectorLintG(instance: *mut c_void, val: *const c_void);
// cv::colored_kinfu::Params::truncateThreshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:170
// ("cv::colored_kinfu::Params::truncateThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_colored_kinfu_Params_propTruncateThreshold_const(instance: *const c_void) -> f32;
// cv::colored_kinfu::Params::setTruncateThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:170
// ("cv::colored_kinfu::Params::setTruncateThreshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_colored_kinfu_Params_propTruncateThreshold_const_float(instance: *mut c_void, val: f32);
// cv::colored_kinfu::Params::delete() generated
// ("cv::colored_kinfu::Params::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_colored_kinfu_Params_delete(instance: *mut c_void);
// create(const Ptr<kinfu::Params> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:49
// ("cv::dynafu::DynaFu::create", vec![(pred!(mut, ["_params"], ["const cv::Ptr<cv::kinfu::Params>*"]), _)]),
pub fn cv_dynafu_DynaFu_create_const_PtrLParamsGR(_params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:53
// ("cv::dynafu::DynaFu::getParams", vec![(pred!(const, [], []), _)]),
pub fn cv_dynafu_DynaFu_getParams_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// render(OutputArray, const Matx44f &)(OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:65
// ("cv::dynafu::DynaFu::render", vec![(pred!(const, ["image", "cameraPose"], ["const cv::_OutputArray*", "const cv::Matx44f*"]), _)]),
pub fn cv_dynafu_DynaFu_render_const_const__OutputArrayR_const_Matx44fR(instance: *const c_void, image: *const c_void, camera_pose: *const core::Matx44f, ocvrs_return: *mut Result<()>);
// cv::dynafu::DynaFu::render(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:65
// ("cv::dynafu::DynaFu::render", vec![(pred!(const, ["image"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_dynafu_DynaFu_render_const_const__OutputArrayR(instance: *const c_void, image: *const c_void, ocvrs_return: *mut Result<()>);
// getCloud(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:75
// ("cv::dynafu::DynaFu::getCloud", vec![(pred!(const, ["points", "normals"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_dynafu_DynaFu_getCloud_const_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<()>);
// getPoints(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:83
// ("cv::dynafu::DynaFu::getPoints", vec![(pred!(const, ["points"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_dynafu_DynaFu_getPoints_const_const__OutputArrayR(instance: *const c_void, points: *const c_void, ocvrs_return: *mut Result<()>);
// getNormals(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:89
// ("cv::dynafu::DynaFu::getNormals", vec![(pred!(const, ["points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_dynafu_DynaFu_getNormals_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<()>);
// reset()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:95
// ("cv::dynafu::DynaFu::reset", vec![(pred!(mut, [], []), _)]),
pub fn cv_dynafu_DynaFu_reset(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// getPose()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:98
// ("cv::dynafu::DynaFu::getPose", vec![(pred!(const, [], []), _)]),
pub fn cv_dynafu_DynaFu_getPose_const(instance: *const c_void, ocvrs_return: *mut Result<core::Affine3f>);
// update(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:108
// ("cv::dynafu::DynaFu::update", vec![(pred!(mut, ["depth"], ["const cv::_InputArray*"]), _)]),
pub fn cv_dynafu_DynaFu_update_const__InputArrayR(instance: *mut c_void, depth: *const c_void, ocvrs_return: *mut Result<bool>);
// getNodesPos()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:110
// ("cv::dynafu::DynaFu::getNodesPos", vec![(pred!(const, [], []), _)]),
pub fn cv_dynafu_DynaFu_getNodesPos_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// marchCubes(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:112
// ("cv::dynafu::DynaFu::marchCubes", vec![(pred!(const, ["vertices", "edges"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_dynafu_DynaFu_marchCubes_const_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, vertices: *const c_void, edges: *const c_void, ocvrs_return: *mut Result<()>);
// renderSurface(OutputArray, OutputArray, OutputArray, bool)(OutputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:114
// ("cv::dynafu::DynaFu::renderSurface", vec![(pred!(mut, ["depthImage", "vertImage", "normImage", "warp"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool"]), _)]),
pub fn cv_dynafu_DynaFu_renderSurface_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_bool(instance: *mut c_void, depth_image: *const c_void, vert_image: *const c_void, norm_image: *const c_void, warp: bool, ocvrs_return: *mut Result<()>);
// cv::dynafu::DynaFu::renderSurface(OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:114
// ("cv::dynafu::DynaFu::renderSurface", vec![(pred!(mut, ["depthImage", "vertImage", "normImage"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_dynafu_DynaFu_renderSurface_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, depth_image: *const c_void, vert_image: *const c_void, norm_image: *const c_void, ocvrs_return: *mut Result<()>);
// cv::dynafu::DynaFu::delete() generated
// ("cv::dynafu::DynaFu::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dynafu_DynaFu_delete(instance: *mut c_void);
// create(const Ptr<Params> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:252
// ("cv::kinfu::KinFu::create", vec![(pred!(mut, ["_params"], ["const cv::Ptr<cv::kinfu::Params>*"]), _)]),
pub fn cv_kinfu_KinFu_create_const_PtrLParamsGR(_params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:256
// ("cv::kinfu::KinFu::getParams", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_KinFu_getParams_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// render(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:266
// ("cv::kinfu::KinFu::render", vec![(pred!(const, ["image"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_kinfu_KinFu_render_const_const__OutputArrayR(instance: *const c_void, image: *const c_void, ocvrs_return: *mut Result<()>);
// render(OutputArray, const Matx44f &)(OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:278
// ("cv::kinfu::KinFu::render", vec![(pred!(const, ["image", "cameraPose"], ["const cv::_OutputArray*", "const cv::Matx44f*"]), _)]),
pub fn cv_kinfu_KinFu_render_const_const__OutputArrayR_const_Matx44fR(instance: *const c_void, image: *const c_void, camera_pose: *const core::Matx44f, ocvrs_return: *mut Result<()>);
// getCloud(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:288
// ("cv::kinfu::KinFu::getCloud", vec![(pred!(const, ["points", "normals"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_kinfu_KinFu_getCloud_const_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<()>);
// getPoints(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:296
// ("cv::kinfu::KinFu::getPoints", vec![(pred!(const, ["points"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_kinfu_KinFu_getPoints_const_const__OutputArrayR(instance: *const c_void, points: *const c_void, ocvrs_return: *mut Result<()>);
// getNormals(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:302
// ("cv::kinfu::KinFu::getNormals", vec![(pred!(const, ["points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_kinfu_KinFu_getNormals_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<()>);
// reset()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:308
// ("cv::kinfu::KinFu::reset", vec![(pred!(mut, [], []), _)]),
pub fn cv_kinfu_KinFu_reset(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// getPose()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:311
// ("cv::kinfu::KinFu::getPose", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_KinFu_getPose_const(instance: *const c_void, ocvrs_return: *mut Result<core::Affine3f>);
// update(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:321
// ("cv::kinfu::KinFu::update", vec![(pred!(mut, ["depth"], ["const cv::_InputArray*"]), _)]),
pub fn cv_kinfu_KinFu_update_const__InputArrayR(instance: *mut c_void, depth: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::kinfu::KinFu::delete() generated
// ("cv::kinfu::KinFu::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_kinfu_KinFu_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:76
// ("cv::kinfu::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_kinfu_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
// Params(Matx33f, Vec3f)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:87
// ("cv::kinfu::Params::Params", vec![(pred!(mut, ["volumeInitialPoseRot", "volumeInitialPoseTransl"], ["cv::Matx33f", "cv::Vec3f"]), _)]),
pub fn cv_kinfu_Params_Params_Matx33f_Vec3f(volume_initial_pose_rot: *const core::Matx33f, volume_initial_pose_transl: *const core::Vec3f, ocvrs_return: *mut Result<*mut c_void>);
// Params(Matx44f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:97
// ("cv::kinfu::Params::Params", vec![(pred!(mut, ["volumeInitialPose"], ["cv::Matx44f"]), _)]),
pub fn cv_kinfu_Params_Params_Matx44f(volume_initial_pose: *const core::Matx44f, ocvrs_return: *mut Result<*mut c_void>);
// setInitialVolumePose(Matx33f, Vec3f)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:108
// ("cv::kinfu::Params::setInitialVolumePose", vec![(pred!(mut, ["R", "t"], ["cv::Matx33f", "cv::Vec3f"]), _)]),
pub fn cv_kinfu_Params_setInitialVolumePose_Matx33f_Vec3f(instance: *mut c_void, r: *const core::Matx33f, t: *const core::Vec3f, ocvrs_return: *mut Result<()>);
// setInitialVolumePose(Matx44f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:115
// ("cv::kinfu::Params::setInitialVolumePose", vec![(pred!(mut, ["homogen_tf"], ["cv::Matx44f"]), _)]),
pub fn cv_kinfu_Params_setInitialVolumePose_Matx44f(instance: *mut c_void, homogen_tf: *const core::Matx44f, ocvrs_return: *mut Result<()>);
// defaultParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:121
// ("cv::kinfu::Params::defaultParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_kinfu_Params_defaultParams(ocvrs_return: *mut Result<*mut c_void>);
// coarseParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:127
// ("cv::kinfu::Params::coarseParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_kinfu_Params_coarseParams(ocvrs_return: *mut Result<*mut c_void>);
// hashTSDFParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:132
// ("cv::kinfu::Params::hashTSDFParams", vec![(pred!(mut, ["isCoarse"], ["bool"]), _)]),
pub fn cv_kinfu_Params_hashTSDFParams_bool(is_coarse: bool, ocvrs_return: *mut Result<*mut c_void>);
// coloredTSDFParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:137
// ("cv::kinfu::Params::coloredTSDFParams", vec![(pred!(mut, ["isCoarse"], ["bool"]), _)]),
pub fn cv_kinfu_Params_coloredTSDFParams_bool(is_coarse: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::kinfu::Params::frameSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:140
// ("cv::kinfu::Params::frameSize", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propFrameSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::kinfu::Params::setFrameSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:140
// ("cv::kinfu::Params::setFrameSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_kinfu_Params_propFrameSize_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::kinfu::Params::volumeKind() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:143
// ("cv::kinfu::Params::volumeKind", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propVolumeKind_const(instance: *const c_void, ocvrs_return: *mut crate::mod_3d::VolumeType);
// cv::kinfu::Params::setVolumeKind(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:143
// ("cv::kinfu::Params::setVolumeKind", vec![(pred!(mut, ["val"], ["const cv::VolumeType"]), _)]),
pub fn cv_kinfu_Params_propVolumeKind_const_VolumeType(instance: *mut c_void, val: crate::mod_3d::VolumeType);
// cv::kinfu::Params::intr() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:146
// ("cv::kinfu::Params::intr", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propIntr_const(instance: *const c_void, ocvrs_return: *mut core::Matx33f);
// cv::kinfu::Params::setIntr(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:146
// ("cv::kinfu::Params::setIntr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
pub fn cv_kinfu_Params_propIntr_const_Matx33f(instance: *mut c_void, val: *const core::Matx33f);
// cv::kinfu::Params::rgb_intr() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:149
// ("cv::kinfu::Params::rgb_intr", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propRgb_intr_const(instance: *const c_void, ocvrs_return: *mut core::Matx33f);
// cv::kinfu::Params::setRgb_intr(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:149
// ("cv::kinfu::Params::setRgb_intr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
pub fn cv_kinfu_Params_propRgb_intr_const_Matx33f(instance: *mut c_void, val: *const core::Matx33f);
// cv::kinfu::Params::depthFactor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:157
// ("cv::kinfu::Params::depthFactor", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propDepthFactor_const(instance: *const c_void) -> f32;
// cv::kinfu::Params::setDepthFactor(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:157
// ("cv::kinfu::Params::setDepthFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_kinfu_Params_propDepthFactor_const_float(instance: *mut c_void, val: f32);
// cv::kinfu::Params::bilateral_sigma_depth() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:160
// ("cv::kinfu::Params::bilateral_sigma_depth", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propBilateral_sigma_depth_const(instance: *const c_void) -> f32;
// cv::kinfu::Params::setBilateral_sigma_depth(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:160
// ("cv::kinfu::Params::setBilateral_sigma_depth", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_kinfu_Params_propBilateral_sigma_depth_const_float(instance: *mut c_void, val: f32);
// cv::kinfu::Params::bilateral_sigma_spatial() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:162
// ("cv::kinfu::Params::bilateral_sigma_spatial", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propBilateral_sigma_spatial_const(instance: *const c_void) -> f32;
// cv::kinfu::Params::setBilateral_sigma_spatial(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:162
// ("cv::kinfu::Params::setBilateral_sigma_spatial", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_kinfu_Params_propBilateral_sigma_spatial_const_float(instance: *mut c_void, val: f32);
// cv::kinfu::Params::bilateral_kernel_size() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:164
// ("cv::kinfu::Params::bilateral_kernel_size", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propBilateral_kernel_size_const(instance: *const c_void) -> i32;
// cv::kinfu::Params::setBilateral_kernel_size(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:164
// ("cv::kinfu::Params::setBilateral_kernel_size", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_kinfu_Params_propBilateral_kernel_size_const_int(instance: *mut c_void, val: i32);
// cv::kinfu::Params::pyramidLevels() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:167
// ("cv::kinfu::Params::pyramidLevels", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propPyramidLevels_const(instance: *const c_void) -> i32;
// cv::kinfu::Params::setPyramidLevels(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:167
// ("cv::kinfu::Params::setPyramidLevels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_kinfu_Params_propPyramidLevels_const_int(instance: *mut c_void, val: i32);
// cv::kinfu::Params::volumeDims() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:173
// ("cv::kinfu::Params::volumeDims", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propVolumeDims_const(instance: *const c_void, ocvrs_return: *mut core::Vec3i);
// cv::kinfu::Params::setVolumeDims(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:173
// ("cv::kinfu::Params::setVolumeDims", vec![(pred!(mut, ["val"], ["const cv::Vec3i"]), _)]),
pub fn cv_kinfu_Params_propVolumeDims_const_Vec3i(instance: *mut c_void, val: *const core::Vec3i);
// cv::kinfu::Params::voxelSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:175
// ("cv::kinfu::Params::voxelSize", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propVoxelSize_const(instance: *const c_void) -> f32;
// cv::kinfu::Params::setVoxelSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:175
// ("cv::kinfu::Params::setVoxelSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_kinfu_Params_propVoxelSize_const_float(instance: *mut c_void, val: f32);
// cv::kinfu::Params::tsdf_min_camera_movement() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:181
// ("cv::kinfu::Params::tsdf_min_camera_movement", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propTsdf_min_camera_movement_const(instance: *const c_void) -> f32;
// cv::kinfu::Params::setTsdf_min_camera_movement(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:181
// ("cv::kinfu::Params::setTsdf_min_camera_movement", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_kinfu_Params_propTsdf_min_camera_movement_const_float(instance: *mut c_void, val: f32);
// cv::kinfu::Params::volumePose() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:184
// ("cv::kinfu::Params::volumePose", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propVolumePose_const(instance: *const c_void, ocvrs_return: *mut core::Matx44f);
// cv::kinfu::Params::setVolumePose(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:184
// ("cv::kinfu::Params::setVolumePose", vec![(pred!(mut, ["val"], ["const cv::Matx44f"]), _)]),
pub fn cv_kinfu_Params_propVolumePose_const_Matx44f(instance: *mut c_void, val: *const core::Matx44f);
// cv::kinfu::Params::tsdf_trunc_dist() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:190
// ("cv::kinfu::Params::tsdf_trunc_dist", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propTsdf_trunc_dist_const(instance: *const c_void) -> f32;
// cv::kinfu::Params::setTsdf_trunc_dist(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:190
// ("cv::kinfu::Params::setTsdf_trunc_dist", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_kinfu_Params_propTsdf_trunc_dist_const_float(instance: *mut c_void, val: f32);
// cv::kinfu::Params::tsdf_max_weight() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:196
// ("cv::kinfu::Params::tsdf_max_weight", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propTsdf_max_weight_const(instance: *const c_void) -> i32;
// cv::kinfu::Params::setTsdf_max_weight(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:196
// ("cv::kinfu::Params::setTsdf_max_weight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_kinfu_Params_propTsdf_max_weight_const_int(instance: *mut c_void, val: i32);
// cv::kinfu::Params::raycast_step_factor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:202
// ("cv::kinfu::Params::raycast_step_factor", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propRaycast_step_factor_const(instance: *const c_void) -> f32;
// cv::kinfu::Params::setRaycast_step_factor(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:202
// ("cv::kinfu::Params::setRaycast_step_factor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_kinfu_Params_propRaycast_step_factor_const_float(instance: *mut c_void, val: f32);
// cv::kinfu::Params::lightPose() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:209
// ("cv::kinfu::Params::lightPose", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propLightPose_const(instance: *const c_void, ocvrs_return: *mut core::Vec3f);
// cv::kinfu::Params::setLightPose(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:209
// ("cv::kinfu::Params::setLightPose", vec![(pred!(mut, ["val"], ["const cv::Vec3f"]), _)]),
pub fn cv_kinfu_Params_propLightPose_const_Vec3f(instance: *mut c_void, val: *const core::Vec3f);
// cv::kinfu::Params::icpDistThresh() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:212
// ("cv::kinfu::Params::icpDistThresh", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propIcpDistThresh_const(instance: *const c_void) -> f32;
// cv::kinfu::Params::setIcpDistThresh(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:212
// ("cv::kinfu::Params::setIcpDistThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_kinfu_Params_propIcpDistThresh_const_float(instance: *mut c_void, val: f32);
// cv::kinfu::Params::icpAngleThresh() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:214
// ("cv::kinfu::Params::icpAngleThresh", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propIcpAngleThresh_const(instance: *const c_void) -> f32;
// cv::kinfu::Params::setIcpAngleThresh(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:214
// ("cv::kinfu::Params::setIcpAngleThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_kinfu_Params_propIcpAngleThresh_const_float(instance: *mut c_void, val: f32);
// cv::kinfu::Params::icpIterations() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:216
// ("cv::kinfu::Params::icpIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propIcpIterations_const(instance: *const c_void) -> *mut c_void;
// cv::kinfu::Params::setIcpIterations(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:216
// ("cv::kinfu::Params::setIcpIterations", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
pub fn cv_kinfu_Params_propIcpIterations_const_vectorLintG(instance: *mut c_void, val: *const c_void);
// cv::kinfu::Params::truncateThreshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:222
// ("cv::kinfu::Params::truncateThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_Params_propTruncateThreshold_const(instance: *const c_void) -> f32;
// cv::kinfu::Params::setTruncateThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:222
// ("cv::kinfu::Params::setTruncateThreshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_kinfu_Params_propTruncateThreshold_const_float(instance: *mut c_void, val: f32);
// cv::kinfu::Params::delete() generated
// ("cv::kinfu::Params::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_kinfu_Params_delete(instance: *mut c_void);
// cv::kinfu::VolumeParams::defaultNew() generated
// ("cv::kinfu::VolumeParams::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_VolumeParams_defaultNew_const() -> *mut c_void;
// cv::kinfu::VolumeParams::kind() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:24
// ("cv::kinfu::VolumeParams::kind", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_VolumeParams_propKind_const(instance: *const c_void, ocvrs_return: *mut crate::mod_3d::VolumeType);
// cv::kinfu::VolumeParams::setKind(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:24
// ("cv::kinfu::VolumeParams::setKind", vec![(pred!(mut, ["val"], ["const cv::VolumeType"]), _)]),
pub fn cv_kinfu_VolumeParams_propKind_const_VolumeType(instance: *mut c_void, val: crate::mod_3d::VolumeType);
// cv::kinfu::VolumeParams::resolutionX() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:31
// ("cv::kinfu::VolumeParams::resolutionX", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_VolumeParams_propResolutionX_const(instance: *const c_void) -> i32;
// cv::kinfu::VolumeParams::setResolutionX(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:31
// ("cv::kinfu::VolumeParams::setResolutionX", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_kinfu_VolumeParams_propResolutionX_const_int(instance: *mut c_void, val: i32);
// cv::kinfu::VolumeParams::resolutionY() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:32
// ("cv::kinfu::VolumeParams::resolutionY", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_VolumeParams_propResolutionY_const(instance: *const c_void) -> i32;
// cv::kinfu::VolumeParams::setResolutionY(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:32
// ("cv::kinfu::VolumeParams::setResolutionY", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_kinfu_VolumeParams_propResolutionY_const_int(instance: *mut c_void, val: i32);
// cv::kinfu::VolumeParams::resolutionZ() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:33
// ("cv::kinfu::VolumeParams::resolutionZ", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_VolumeParams_propResolutionZ_const(instance: *const c_void) -> i32;
// cv::kinfu::VolumeParams::setResolutionZ(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:33
// ("cv::kinfu::VolumeParams::setResolutionZ", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_kinfu_VolumeParams_propResolutionZ_const_int(instance: *mut c_void, val: i32);
// cv::kinfu::VolumeParams::unitResolution() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:39
// ("cv::kinfu::VolumeParams::unitResolution", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_VolumeParams_propUnitResolution_const(instance: *const c_void) -> i32;
// cv::kinfu::VolumeParams::setUnitResolution(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:39
// ("cv::kinfu::VolumeParams::setUnitResolution", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_kinfu_VolumeParams_propUnitResolution_const_int(instance: *mut c_void, val: i32);
// cv::kinfu::VolumeParams::volumSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:42
// ("cv::kinfu::VolumeParams::volumSize", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_VolumeParams_propVolumSize_const(instance: *const c_void) -> f32;
// cv::kinfu::VolumeParams::setVolumSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:42
// ("cv::kinfu::VolumeParams::setVolumSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_kinfu_VolumeParams_propVolumSize_const_float(instance: *mut c_void, val: f32);
// cv::kinfu::VolumeParams::pose() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:45
// ("cv::kinfu::VolumeParams::pose", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_VolumeParams_propPose_const(instance: *const c_void, ocvrs_return: *mut core::Matx44f);
// cv::kinfu::VolumeParams::setPose(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:45
// ("cv::kinfu::VolumeParams::setPose", vec![(pred!(mut, ["val"], ["const cv::Matx44f"]), _)]),
pub fn cv_kinfu_VolumeParams_propPose_const_Matx44f(instance: *mut c_void, val: *const core::Matx44f);
// cv::kinfu::VolumeParams::voxelSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:48
// ("cv::kinfu::VolumeParams::voxelSize", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_VolumeParams_propVoxelSize_const(instance: *const c_void) -> f32;
// cv::kinfu::VolumeParams::setVoxelSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:48
// ("cv::kinfu::VolumeParams::setVoxelSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_kinfu_VolumeParams_propVoxelSize_const_float(instance: *mut c_void, val: f32);
// cv::kinfu::VolumeParams::tsdfTruncDist() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:53
// ("cv::kinfu::VolumeParams::tsdfTruncDist", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_VolumeParams_propTsdfTruncDist_const(instance: *const c_void) -> f32;
// cv::kinfu::VolumeParams::setTsdfTruncDist(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:53
// ("cv::kinfu::VolumeParams::setTsdfTruncDist", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_kinfu_VolumeParams_propTsdfTruncDist_const_float(instance: *mut c_void, val: f32);
// cv::kinfu::VolumeParams::maxWeight() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:59
// ("cv::kinfu::VolumeParams::maxWeight", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_VolumeParams_propMaxWeight_const(instance: *const c_void) -> i32;
// cv::kinfu::VolumeParams::setMaxWeight(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:59
// ("cv::kinfu::VolumeParams::setMaxWeight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_kinfu_VolumeParams_propMaxWeight_const_int(instance: *mut c_void, val: i32);
// cv::kinfu::VolumeParams::depthTruncThreshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:64
// ("cv::kinfu::VolumeParams::depthTruncThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_VolumeParams_propDepthTruncThreshold_const(instance: *const c_void) -> f32;
// cv::kinfu::VolumeParams::setDepthTruncThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:64
// ("cv::kinfu::VolumeParams::setDepthTruncThreshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_kinfu_VolumeParams_propDepthTruncThreshold_const_float(instance: *mut c_void, val: f32);
// cv::kinfu::VolumeParams::raycastStepFactor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:69
// ("cv::kinfu::VolumeParams::raycastStepFactor", vec![(pred!(const, [], []), _)]),
pub fn cv_kinfu_VolumeParams_propRaycastStepFactor_const(instance: *const c_void) -> f32;
// cv::kinfu::VolumeParams::setRaycastStepFactor(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:69
// ("cv::kinfu::VolumeParams::setRaycastStepFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_kinfu_VolumeParams_propRaycastStepFactor_const_float(instance: *mut c_void, val: f32);
// cv::kinfu::VolumeParams::delete() generated
// ("cv::kinfu::VolumeParams::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_kinfu_VolumeParams_delete(instance: *mut c_void);
// create(const Ptr<Params> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:195
// ("cv::large_kinfu::LargeKinfu::create", vec![(pred!(mut, ["_params"], ["const cv::Ptr<cv::large_kinfu::Params>*"]), _)]),
pub fn cv_large_kinfu_LargeKinfu_create_const_PtrLParamsGR(_params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:198
// ("cv::large_kinfu::LargeKinfu::getParams", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_LargeKinfu_getParams_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// render(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:200
// ("cv::large_kinfu::LargeKinfu::render", vec![(pred!(const, ["image"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_large_kinfu_LargeKinfu_render_const_const__OutputArrayR(instance: *const c_void, image: *const c_void, ocvrs_return: *mut Result<()>);
// render(OutputArray, const Matx44f &)(OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:201
// ("cv::large_kinfu::LargeKinfu::render", vec![(pred!(const, ["image", "cameraPose"], ["const cv::_OutputArray*", "const cv::Matx44f*"]), _)]),
pub fn cv_large_kinfu_LargeKinfu_render_const_const__OutputArrayR_const_Matx44fR(instance: *const c_void, image: *const c_void, camera_pose: *const core::Matx44f, ocvrs_return: *mut Result<()>);
// getCloud(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:203
// ("cv::large_kinfu::LargeKinfu::getCloud", vec![(pred!(const, ["points", "normals"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_large_kinfu_LargeKinfu_getCloud_const_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<()>);
// getPoints(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:205
// ("cv::large_kinfu::LargeKinfu::getPoints", vec![(pred!(const, ["points"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_large_kinfu_LargeKinfu_getPoints_const_const__OutputArrayR(instance: *const c_void, points: *const c_void, ocvrs_return: *mut Result<()>);
// getNormals(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:207
// ("cv::large_kinfu::LargeKinfu::getNormals", vec![(pred!(const, ["points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_large_kinfu_LargeKinfu_getNormals_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<()>);
// reset()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:209
// ("cv::large_kinfu::LargeKinfu::reset", vec![(pred!(mut, [], []), _)]),
pub fn cv_large_kinfu_LargeKinfu_reset(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// getPose()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:211
// ("cv::large_kinfu::LargeKinfu::getPose", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_LargeKinfu_getPose_const(instance: *const c_void, ocvrs_return: *mut Result<core::Affine3f>);
// update(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:213
// ("cv::large_kinfu::LargeKinfu::update", vec![(pred!(mut, ["depth"], ["const cv::_InputArray*"]), _)]),
pub fn cv_large_kinfu_LargeKinfu_update_const__InputArrayR(instance: *mut c_void, depth: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::large_kinfu::LargeKinfu::delete() generated
// ("cv::large_kinfu::LargeKinfu::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_large_kinfu_LargeKinfu_delete(instance: *mut c_void);
// defaultParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:92
// ("cv::large_kinfu::Params::defaultParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_large_kinfu_Params_defaultParams(ocvrs_return: *mut Result<*mut c_void>);
// coarseParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:99
// ("cv::large_kinfu::Params::coarseParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_large_kinfu_Params_coarseParams(ocvrs_return: *mut Result<*mut c_void>);
// hashTSDFParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:105
// ("cv::large_kinfu::Params::hashTSDFParams", vec![(pred!(mut, ["isCoarse"], ["bool"]), _)]),
pub fn cv_large_kinfu_Params_hashTSDFParams_bool(is_coarse: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::large_kinfu::Params::defaultNew() generated
// ("cv::large_kinfu::Params::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_defaultNew_const() -> *mut c_void;
// cv::large_kinfu::Params::frameSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:108
// ("cv::large_kinfu::Params::frameSize", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_propFrameSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::large_kinfu::Params::setFrameSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:108
// ("cv::large_kinfu::Params::setFrameSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_large_kinfu_Params_propFrameSize_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::large_kinfu::Params::intr() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:111
// ("cv::large_kinfu::Params::intr", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_propIntr_const(instance: *const c_void, ocvrs_return: *mut core::Matx33f);
// cv::large_kinfu::Params::setIntr(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:111
// ("cv::large_kinfu::Params::setIntr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
pub fn cv_large_kinfu_Params_propIntr_const_Matx33f(instance: *mut c_void, val: *const core::Matx33f);
// cv::large_kinfu::Params::rgb_intr() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:114
// ("cv::large_kinfu::Params::rgb_intr", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_propRgb_intr_const(instance: *const c_void, ocvrs_return: *mut core::Matx33f);
// cv::large_kinfu::Params::setRgb_intr(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:114
// ("cv::large_kinfu::Params::setRgb_intr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
pub fn cv_large_kinfu_Params_propRgb_intr_const_Matx33f(instance: *mut c_void, val: *const core::Matx33f);
// cv::large_kinfu::Params::depthFactor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:123
// ("cv::large_kinfu::Params::depthFactor", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_propDepthFactor_const(instance: *const c_void) -> f32;
// cv::large_kinfu::Params::setDepthFactor(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:123
// ("cv::large_kinfu::Params::setDepthFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_large_kinfu_Params_propDepthFactor_const_float(instance: *mut c_void, val: f32);
// cv::large_kinfu::Params::bilateral_sigma_depth() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:126
// ("cv::large_kinfu::Params::bilateral_sigma_depth", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_propBilateral_sigma_depth_const(instance: *const c_void) -> f32;
// cv::large_kinfu::Params::setBilateral_sigma_depth(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:126
// ("cv::large_kinfu::Params::setBilateral_sigma_depth", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_large_kinfu_Params_propBilateral_sigma_depth_const_float(instance: *mut c_void, val: f32);
// cv::large_kinfu::Params::bilateral_sigma_spatial() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:128
// ("cv::large_kinfu::Params::bilateral_sigma_spatial", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_propBilateral_sigma_spatial_const(instance: *const c_void) -> f32;
// cv::large_kinfu::Params::setBilateral_sigma_spatial(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:128
// ("cv::large_kinfu::Params::setBilateral_sigma_spatial", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_large_kinfu_Params_propBilateral_sigma_spatial_const_float(instance: *mut c_void, val: f32);
// cv::large_kinfu::Params::bilateral_kernel_size() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:130
// ("cv::large_kinfu::Params::bilateral_kernel_size", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_propBilateral_kernel_size_const(instance: *const c_void) -> i32;
// cv::large_kinfu::Params::setBilateral_kernel_size(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:130
// ("cv::large_kinfu::Params::setBilateral_kernel_size", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_large_kinfu_Params_propBilateral_kernel_size_const_int(instance: *mut c_void, val: i32);
// cv::large_kinfu::Params::pyramidLevels() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:133
// ("cv::large_kinfu::Params::pyramidLevels", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_propPyramidLevels_const(instance: *const c_void) -> i32;
// cv::large_kinfu::Params::setPyramidLevels(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:133
// ("cv::large_kinfu::Params::setPyramidLevels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_large_kinfu_Params_propPyramidLevels_const_int(instance: *mut c_void, val: i32);
// cv::large_kinfu::Params::tsdf_min_camera_movement() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:139
// ("cv::large_kinfu::Params::tsdf_min_camera_movement", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_propTsdf_min_camera_movement_const(instance: *const c_void) -> f32;
// cv::large_kinfu::Params::setTsdf_min_camera_movement(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:139
// ("cv::large_kinfu::Params::setTsdf_min_camera_movement", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_large_kinfu_Params_propTsdf_min_camera_movement_const_float(instance: *mut c_void, val: f32);
// cv::large_kinfu::Params::lightPose() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:142
// ("cv::large_kinfu::Params::lightPose", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_propLightPose_const(instance: *const c_void, ocvrs_return: *mut core::Vec3f);
// cv::large_kinfu::Params::setLightPose(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:142
// ("cv::large_kinfu::Params::setLightPose", vec![(pred!(mut, ["val"], ["const cv::Vec3f"]), _)]),
pub fn cv_large_kinfu_Params_propLightPose_const_Vec3f(instance: *mut c_void, val: *const core::Vec3f);
// cv::large_kinfu::Params::icpDistThresh() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:145
// ("cv::large_kinfu::Params::icpDistThresh", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_propIcpDistThresh_const(instance: *const c_void) -> f32;
// cv::large_kinfu::Params::setIcpDistThresh(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:145
// ("cv::large_kinfu::Params::setIcpDistThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_large_kinfu_Params_propIcpDistThresh_const_float(instance: *mut c_void, val: f32);
// cv::large_kinfu::Params::icpAngleThresh() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:147
// ("cv::large_kinfu::Params::icpAngleThresh", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_propIcpAngleThresh_const(instance: *const c_void) -> f32;
// cv::large_kinfu::Params::setIcpAngleThresh(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:147
// ("cv::large_kinfu::Params::setIcpAngleThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_large_kinfu_Params_propIcpAngleThresh_const_float(instance: *mut c_void, val: f32);
// cv::large_kinfu::Params::icpIterations() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:149
// ("cv::large_kinfu::Params::icpIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_propIcpIterations_const(instance: *const c_void) -> *mut c_void;
// cv::large_kinfu::Params::setIcpIterations(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:149
// ("cv::large_kinfu::Params::setIcpIterations", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
pub fn cv_large_kinfu_Params_propIcpIterations_const_vectorLintG(instance: *mut c_void, val: *const c_void);
// cv::large_kinfu::Params::truncateThreshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:155
// ("cv::large_kinfu::Params::truncateThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_propTruncateThreshold_const(instance: *const c_void) -> f32;
// cv::large_kinfu::Params::setTruncateThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:155
// ("cv::large_kinfu::Params::setTruncateThreshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_large_kinfu_Params_propTruncateThreshold_const_float(instance: *mut c_void, val: f32);
// cv::large_kinfu::Params::volumeParams() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:159
// ("cv::large_kinfu::Params::volumeParams", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_Params_propVolumeParams_const(instance: *const c_void) -> *mut c_void;
// cv::large_kinfu::Params::setVolumeParams(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:159
// ("cv::large_kinfu::Params::setVolumeParams", vec![(pred!(mut, ["val"], ["const cv::large_kinfu::VolumeParams"]), _)]),
pub fn cv_large_kinfu_Params_propVolumeParams_const_VolumeParams(instance: *mut c_void, val: *const c_void);
// cv::large_kinfu::Params::delete() generated
// ("cv::large_kinfu::Params::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_large_kinfu_Params_delete(instance: *mut c_void);
// defaultParams(VolumeType)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:80
// ("cv::large_kinfu::VolumeParams::defaultParams", vec![(pred!(mut, ["volumeType"], ["cv::VolumeType"]), _)]),
pub fn cv_large_kinfu_VolumeParams_defaultParams_VolumeType(volume_type: crate::mod_3d::VolumeType, ocvrs_return: *mut Result<*mut c_void>);
// coarseParams(VolumeType)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:83
// ("cv::large_kinfu::VolumeParams::coarseParams", vec![(pred!(mut, ["volumeType"], ["cv::VolumeType"]), _)]),
pub fn cv_large_kinfu_VolumeParams_coarseParams_VolumeType(volume_type: crate::mod_3d::VolumeType, ocvrs_return: *mut Result<*mut c_void>);
// cv::large_kinfu::VolumeParams::defaultNew() generated
// ("cv::large_kinfu::VolumeParams::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_VolumeParams_defaultNew_const() -> *mut c_void;
// cv::large_kinfu::VolumeParams::kind() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:26
// ("cv::large_kinfu::VolumeParams::kind", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_VolumeParams_propKind_const(instance: *const c_void, ocvrs_return: *mut crate::mod_3d::VolumeType);
// cv::large_kinfu::VolumeParams::setKind(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:26
// ("cv::large_kinfu::VolumeParams::setKind", vec![(pred!(mut, ["val"], ["const cv::VolumeType"]), _)]),
pub fn cv_large_kinfu_VolumeParams_propKind_const_VolumeType(instance: *mut c_void, val: crate::mod_3d::VolumeType);
// cv::large_kinfu::VolumeParams::resolutionX() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:34
// ("cv::large_kinfu::VolumeParams::resolutionX", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_VolumeParams_propResolutionX_const(instance: *const c_void) -> i32;
// cv::large_kinfu::VolumeParams::setResolutionX(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:34
// ("cv::large_kinfu::VolumeParams::setResolutionX", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_large_kinfu_VolumeParams_propResolutionX_const_int(instance: *mut c_void, val: i32);
// cv::large_kinfu::VolumeParams::resolutionY() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:35
// ("cv::large_kinfu::VolumeParams::resolutionY", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_VolumeParams_propResolutionY_const(instance: *const c_void) -> i32;
// cv::large_kinfu::VolumeParams::setResolutionY(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:35
// ("cv::large_kinfu::VolumeParams::setResolutionY", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_large_kinfu_VolumeParams_propResolutionY_const_int(instance: *mut c_void, val: i32);
// cv::large_kinfu::VolumeParams::resolutionZ() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:36
// ("cv::large_kinfu::VolumeParams::resolutionZ", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_VolumeParams_propResolutionZ_const(instance: *const c_void) -> i32;
// cv::large_kinfu::VolumeParams::setResolutionZ(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:36
// ("cv::large_kinfu::VolumeParams::setResolutionZ", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_large_kinfu_VolumeParams_propResolutionZ_const_int(instance: *mut c_void, val: i32);
// cv::large_kinfu::VolumeParams::unitResolution() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:43
// ("cv::large_kinfu::VolumeParams::unitResolution", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_VolumeParams_propUnitResolution_const(instance: *const c_void) -> i32;
// cv::large_kinfu::VolumeParams::setUnitResolution(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:43
// ("cv::large_kinfu::VolumeParams::setUnitResolution", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_large_kinfu_VolumeParams_propUnitResolution_const_int(instance: *mut c_void, val: i32);
// cv::large_kinfu::VolumeParams::volumSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:46
// ("cv::large_kinfu::VolumeParams::volumSize", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_VolumeParams_propVolumSize_const(instance: *const c_void) -> f32;
// cv::large_kinfu::VolumeParams::setVolumSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:46
// ("cv::large_kinfu::VolumeParams::setVolumSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_large_kinfu_VolumeParams_propVolumSize_const_float(instance: *mut c_void, val: f32);
// cv::large_kinfu::VolumeParams::pose() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:49
// ("cv::large_kinfu::VolumeParams::pose", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_VolumeParams_propPose_const(instance: *const c_void, ocvrs_return: *mut core::Matx44f);
// cv::large_kinfu::VolumeParams::setPose(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:49
// ("cv::large_kinfu::VolumeParams::setPose", vec![(pred!(mut, ["val"], ["const cv::Matx44f"]), _)]),
pub fn cv_large_kinfu_VolumeParams_propPose_const_Matx44f(instance: *mut c_void, val: *const core::Matx44f);
// cv::large_kinfu::VolumeParams::voxelSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:52
// ("cv::large_kinfu::VolumeParams::voxelSize", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_VolumeParams_propVoxelSize_const(instance: *const c_void) -> f32;
// cv::large_kinfu::VolumeParams::setVoxelSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:52
// ("cv::large_kinfu::VolumeParams::setVoxelSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_large_kinfu_VolumeParams_propVoxelSize_const_float(instance: *mut c_void, val: f32);
// cv::large_kinfu::VolumeParams::tsdfTruncDist() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:58
// ("cv::large_kinfu::VolumeParams::tsdfTruncDist", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_VolumeParams_propTsdfTruncDist_const(instance: *const c_void) -> f32;
// cv::large_kinfu::VolumeParams::setTsdfTruncDist(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:58
// ("cv::large_kinfu::VolumeParams::setTsdfTruncDist", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_large_kinfu_VolumeParams_propTsdfTruncDist_const_float(instance: *mut c_void, val: f32);
// cv::large_kinfu::VolumeParams::maxWeight() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:65
// ("cv::large_kinfu::VolumeParams::maxWeight", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_VolumeParams_propMaxWeight_const(instance: *const c_void) -> i32;
// cv::large_kinfu::VolumeParams::setMaxWeight(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:65
// ("cv::large_kinfu::VolumeParams::setMaxWeight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_large_kinfu_VolumeParams_propMaxWeight_const_int(instance: *mut c_void, val: i32);
// cv::large_kinfu::VolumeParams::depthTruncThreshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:71
// ("cv::large_kinfu::VolumeParams::depthTruncThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_VolumeParams_propDepthTruncThreshold_const(instance: *const c_void) -> f32;
// cv::large_kinfu::VolumeParams::setDepthTruncThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:71
// ("cv::large_kinfu::VolumeParams::setDepthTruncThreshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_large_kinfu_VolumeParams_propDepthTruncThreshold_const_float(instance: *mut c_void, val: f32);
// cv::large_kinfu::VolumeParams::raycastStepFactor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:77
// ("cv::large_kinfu::VolumeParams::raycastStepFactor", vec![(pred!(const, [], []), _)]),
pub fn cv_large_kinfu_VolumeParams_propRaycastStepFactor_const(instance: *const c_void) -> f32;
// cv::large_kinfu::VolumeParams::setRaycastStepFactor(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:77
// ("cv::large_kinfu::VolumeParams::setRaycastStepFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_large_kinfu_VolumeParams_propRaycastStepFactor_const_float(instance: *mut c_void, val: f32);
// cv::large_kinfu::VolumeParams::delete() generated
// ("cv::large_kinfu::VolumeParams::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_large_kinfu_VolumeParams_delete(instance: *mut c_void);
// ColorGradient()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:172
// ("cv::linemod::ColorGradient::ColorGradient", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_ColorGradient_ColorGradient(ocvrs_return: *mut Result<*mut c_void>);
// ColorGradient(float, size_t, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:182
// ("cv::linemod::ColorGradient::ColorGradient", vec![(pred!(mut, ["weak_threshold", "num_features", "strong_threshold"], ["float", "size_t", "float"]), _)]),
pub fn cv_linemod_ColorGradient_ColorGradient_float_size_t_float(weak_threshold: f32, num_features: size_t, strong_threshold: f32, ocvrs_return: *mut Result<*mut c_void>);
// create(float, size_t, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:184
// ("cv::linemod::ColorGradient::create", vec![(pred!(mut, ["weak_threshold", "num_features", "strong_threshold"], ["float", "size_t", "float"]), _)]),
pub fn cv_linemod_ColorGradient_create_float_size_t_float(weak_threshold: f32, num_features: size_t, strong_threshold: f32, ocvrs_return: *mut Result<*mut c_void>);
// name()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:186
// ("cv::linemod::ColorGradient::name", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_ColorGradient_name_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:188
// ("cv::linemod::ColorGradient::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_linemod_ColorGradient_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:189
// ("cv::linemod::ColorGradient::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_linemod_ColorGradient_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::linemod::ColorGradient::weak_threshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:191
// ("cv::linemod::ColorGradient::weak_threshold", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_ColorGradient_propWeak_threshold_const(instance: *const c_void) -> f32;
// cv::linemod::ColorGradient::setWeak_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:191
// ("cv::linemod::ColorGradient::setWeak_threshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_linemod_ColorGradient_propWeak_threshold_const_float(instance: *mut c_void, val: f32);
// cv::linemod::ColorGradient::num_features() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:192
// ("cv::linemod::ColorGradient::num_features", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_ColorGradient_propNum_features_const(instance: *const c_void) -> size_t;
// cv::linemod::ColorGradient::setNum_features(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:192
// ("cv::linemod::ColorGradient::setNum_features", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
pub fn cv_linemod_ColorGradient_propNum_features_const_size_t(instance: *mut c_void, val: size_t);
// cv::linemod::ColorGradient::strong_threshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:193
// ("cv::linemod::ColorGradient::strong_threshold", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_ColorGradient_propStrong_threshold_const(instance: *const c_void) -> f32;
// cv::linemod::ColorGradient::setStrong_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:193
// ("cv::linemod::ColorGradient::setStrong_threshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_linemod_ColorGradient_propStrong_threshold_const_float(instance: *mut c_void, val: f32);
// cv::linemod::ColorGradient::to_LineMod_Modality() generated
// ("cv::linemod::ColorGradient::to_LineMod_Modality", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_ColorGradient_to_LineMod_Modality(instance: *mut c_void) -> *mut c_void;
// cv::linemod::ColorGradient::delete() generated
// ("cv::linemod::ColorGradient::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_ColorGradient_delete(instance: *mut c_void);
// DepthNormal()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:209
// ("cv::linemod::DepthNormal::DepthNormal", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_DepthNormal_DepthNormal(ocvrs_return: *mut Result<*mut c_void>);
// DepthNormal(int, int, size_t, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:221
// ("cv::linemod::DepthNormal::DepthNormal", vec![(pred!(mut, ["distance_threshold", "difference_threshold", "num_features", "extract_threshold"], ["int", "int", "size_t", "int"]), _)]),
pub fn cv_linemod_DepthNormal_DepthNormal_int_int_size_t_int(distance_threshold: i32, difference_threshold: i32, num_features: size_t, extract_threshold: i32, ocvrs_return: *mut Result<*mut c_void>);
// create(int, int, size_t, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:224
// ("cv::linemod::DepthNormal::create", vec![(pred!(mut, ["distance_threshold", "difference_threshold", "num_features", "extract_threshold"], ["int", "int", "size_t", "int"]), _)]),
pub fn cv_linemod_DepthNormal_create_int_int_size_t_int(distance_threshold: i32, difference_threshold: i32, num_features: size_t, extract_threshold: i32, ocvrs_return: *mut Result<*mut c_void>);
// name()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:227
// ("cv::linemod::DepthNormal::name", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_DepthNormal_name_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:229
// ("cv::linemod::DepthNormal::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_linemod_DepthNormal_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:230
// ("cv::linemod::DepthNormal::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_linemod_DepthNormal_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::linemod::DepthNormal::distance_threshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:232
// ("cv::linemod::DepthNormal::distance_threshold", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_DepthNormal_propDistance_threshold_const(instance: *const c_void) -> i32;
// cv::linemod::DepthNormal::setDistance_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:232
// ("cv::linemod::DepthNormal::setDistance_threshold", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_linemod_DepthNormal_propDistance_threshold_const_int(instance: *mut c_void, val: i32);
// cv::linemod::DepthNormal::difference_threshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:233
// ("cv::linemod::DepthNormal::difference_threshold", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_DepthNormal_propDifference_threshold_const(instance: *const c_void) -> i32;
// cv::linemod::DepthNormal::setDifference_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:233
// ("cv::linemod::DepthNormal::setDifference_threshold", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_linemod_DepthNormal_propDifference_threshold_const_int(instance: *mut c_void, val: i32);
// cv::linemod::DepthNormal::num_features() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:234
// ("cv::linemod::DepthNormal::num_features", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_DepthNormal_propNum_features_const(instance: *const c_void) -> size_t;
// cv::linemod::DepthNormal::setNum_features(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:234
// ("cv::linemod::DepthNormal::setNum_features", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
pub fn cv_linemod_DepthNormal_propNum_features_const_size_t(instance: *mut c_void, val: size_t);
// cv::linemod::DepthNormal::extract_threshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:235
// ("cv::linemod::DepthNormal::extract_threshold", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_DepthNormal_propExtract_threshold_const(instance: *const c_void) -> i32;
// cv::linemod::DepthNormal::setExtract_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:235
// ("cv::linemod::DepthNormal::setExtract_threshold", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_linemod_DepthNormal_propExtract_threshold_const_int(instance: *mut c_void, val: i32);
// cv::linemod::DepthNormal::to_LineMod_Modality() generated
// ("cv::linemod::DepthNormal::to_LineMod_Modality", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_DepthNormal_to_LineMod_Modality(instance: *mut c_void) -> *mut c_void;
// cv::linemod::DepthNormal::delete() generated
// ("cv::linemod::DepthNormal::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_DepthNormal_delete(instance: *mut c_void);
// Detector()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:304
// ("cv::linemod::Detector::Detector", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_Detector_Detector(ocvrs_return: *mut Result<*mut c_void>);
// Detector(const std::vector<Ptr<Modality>> &, const std::vector<int> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:313
// ("cv::linemod::Detector::Detector", vec![(pred!(mut, ["modalities", "T_pyramid"], ["const std::vector<cv::Ptr<cv::linemod::Modality>>*", "const std::vector<int>*"]), _)]),
pub fn cv_linemod_Detector_Detector_const_vectorLPtrLModalityGGR_const_vectorLintGR(modalities: *const c_void, t_pyramid: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// match(const std::vector<Mat> &, float, std::vector<Match> &, const std::vector<String> &, OutputArrayOfArrays, const std::vector<Mat> &)(CppPassByVoidPtr, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:330
// ("cv::linemod::Detector::match", vec![(pred!(const, ["sources", "threshold", "matches", "class_ids", "quantized_images", "masks"], ["const std::vector<cv::Mat>*", "float", "std::vector<cv::linemod::Match>*", "const std::vector<cv::String>*", "const cv::_OutputArray*", "const std::vector<cv::Mat>*"]), _)]),
pub fn cv_linemod_Detector_match_const_const_vectorLMatGR_float_vectorLMatchGR_const_vectorLStringGR_const__OutputArrayR_const_vectorLMatGR(instance: *const c_void, sources: *const c_void, threshold: f32, matches: *mut c_void, class_ids: *const c_void, quantized_images: *const c_void, masks: *const c_void, ocvrs_return: *mut Result<()>);
// cv::linemod::Detector::match(CppPassByVoidPtr, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:330
// ("cv::linemod::Detector::match", vec![(pred!(const, ["sources", "threshold", "matches"], ["const std::vector<cv::Mat>*", "float", "std::vector<cv::linemod::Match>*"]), _)]),
pub fn cv_linemod_Detector_match_const_const_vectorLMatGR_float_vectorLMatchGR(instance: *const c_void, sources: *const c_void, threshold: f32, matches: *mut c_void, ocvrs_return: *mut Result<()>);
// addTemplate(const std::vector<Mat> &, const String &, const Mat &, Rect *)(CppPassByVoidPtr, InString, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:345
// ("cv::linemod::Detector::addTemplate", vec![(pred!(mut, ["sources", "class_id", "object_mask", "bounding_box"], ["const std::vector<cv::Mat>*", "const cv::String*", "const cv::Mat*", "cv::Rect*"]), _)]),
pub fn cv_linemod_Detector_addTemplate_const_vectorLMatGR_const_StringR_const_MatR_RectX(instance: *mut c_void, sources: *const c_void, class_id: *const c_char, object_mask: *const c_void, bounding_box: *mut core::Rect, ocvrs_return: *mut Result<i32>);
// cv::linemod::Detector::addTemplate(CppPassByVoidPtr, InString, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:345
// ("cv::linemod::Detector::addTemplate", vec![(pred!(mut, ["sources", "class_id", "object_mask"], ["const std::vector<cv::Mat>*", "const cv::String*", "const cv::Mat*"]), _)]),
pub fn cv_linemod_Detector_addTemplate_const_vectorLMatGR_const_StringR_const_MatR(instance: *mut c_void, sources: *const c_void, class_id: *const c_char, object_mask: *const c_void, ocvrs_return: *mut Result<i32>);
// addSyntheticTemplate(const std::vector<Template> &, const String &)(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:351
// ("cv::linemod::Detector::addSyntheticTemplate", vec![(pred!(mut, ["templates", "class_id"], ["const std::vector<cv::linemod::Template>*", "const cv::String*"]), _)]),
pub fn cv_linemod_Detector_addSyntheticTemplate_const_vectorLTemplateGR_const_StringR(instance: *mut c_void, templates: *const c_void, class_id: *const c_char, ocvrs_return: *mut Result<i32>);
// getModalities()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:359
// ("cv::linemod::Detector::getModalities", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Detector_getModalities_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getT(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:364
// ("cv::linemod::Detector::getT", vec![(pred!(const, ["pyramid_level"], ["int"]), _)]),
pub fn cv_linemod_Detector_getT_const_int(instance: *const c_void, pyramid_level: i32, ocvrs_return: *mut Result<i32>);
// pyramidLevels()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:369
// ("cv::linemod::Detector::pyramidLevels", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Detector_pyramidLevels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// getTemplates(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:377
// ("cv::linemod::Detector::getTemplates", vec![(pred!(const, ["class_id", "template_id"], ["const cv::String*", "int"]), _)]),
pub fn cv_linemod_Detector_getTemplates_const_const_StringR_int(instance: *const c_void, class_id: *const c_char, template_id: i32, ocvrs_return: *mut Result<*mut c_void>);
// numTemplates()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:379
// ("cv::linemod::Detector::numTemplates", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Detector_numTemplates_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// numTemplates(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:380
// ("cv::linemod::Detector::numTemplates", vec![(pred!(const, ["class_id"], ["const cv::String*"]), _)]),
pub fn cv_linemod_Detector_numTemplates_const_const_StringR(instance: *const c_void, class_id: *const c_char, ocvrs_return: *mut Result<i32>);
// numClasses()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:381
// ("cv::linemod::Detector::numClasses", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Detector_numClasses_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// classIds()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:383
// ("cv::linemod::Detector::classIds", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Detector_classIds_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:385
// ("cv::linemod::Detector::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_linemod_Detector_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:386
// ("cv::linemod::Detector::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_linemod_Detector_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// readClass(const FileNode &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:388
// ("cv::linemod::Detector::readClass", vec![(pred!(mut, ["fn", "class_id_override"], ["const cv::FileNode*", "const cv::String*"]), _)]),
pub fn cv_linemod_Detector_readClass_const_FileNodeR_const_StringR(instance: *mut c_void, fn_: *const c_void, class_id_override: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::linemod::Detector::readClass(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:388
// ("cv::linemod::Detector::readClass", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_linemod_Detector_readClass_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// writeClass(const String &, FileStorage &)(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:389
// ("cv::linemod::Detector::writeClass", vec![(pred!(const, ["class_id", "fs"], ["const cv::String*", "cv::FileStorage*"]), _)]),
pub fn cv_linemod_Detector_writeClass_const_const_StringR_FileStorageR(instance: *const c_void, class_id: *const c_char, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// readClasses(const std::vector<String> &, const String &)(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:391
// ("cv::linemod::Detector::readClasses", vec![(pred!(mut, ["class_ids", "format"], ["const std::vector<cv::String>*", "const cv::String*"]), _)]),
pub fn cv_linemod_Detector_readClasses_const_vectorLStringGR_const_StringR(instance: *mut c_void, class_ids: *const c_void, format: *const c_char, ocvrs_return: *mut Result<()>);
// cv::linemod::Detector::readClasses(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:391
// ("cv::linemod::Detector::readClasses", vec![(pred!(mut, ["class_ids"], ["const std::vector<cv::String>*"]), _)]),
pub fn cv_linemod_Detector_readClasses_const_vectorLStringGR(instance: *mut c_void, class_ids: *const c_void, ocvrs_return: *mut Result<()>);
// writeClasses(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:393
// ("cv::linemod::Detector::writeClasses", vec![(pred!(const, ["format"], ["const cv::String*"]), _)]),
pub fn cv_linemod_Detector_writeClasses_const_const_StringR(instance: *const c_void, format: *const c_char, ocvrs_return: *mut Result<()>);
// cv::linemod::Detector::writeClasses() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:393
// ("cv::linemod::Detector::writeClasses", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Detector_writeClasses_const(instance: *const c_void, ocvrs_return: *mut Result<()>);
// cv::linemod::Detector::delete() generated
// ("cv::linemod::Detector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_Detector_delete(instance: *mut c_void);
// Feature()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:32
// ("cv::linemod::Feature::Feature", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_Feature_Feature(ocvrs_return: *mut Result<crate::rgbd::LineMod_Feature>);
// Feature(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:33
// ("cv::linemod::Feature::Feature", vec![(pred!(mut, ["x", "y", "label"], ["int", "int", "int"]), _)]),
pub fn cv_linemod_Feature_Feature_int_int_int(x: i32, y: i32, label: i32, ocvrs_return: *mut Result<crate::rgbd::LineMod_Feature>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:35
// ("cv::linemod::Feature::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_linemod_Feature_read_const_FileNodeR(instance: *const crate::rgbd::LineMod_Feature, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:36
// ("cv::linemod::Feature::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_linemod_Feature_write_const_FileStorageR(instance: *const crate::rgbd::LineMod_Feature, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// Match()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:261
// ("cv::linemod::Match::Match", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_Match_Match(ocvrs_return: *mut Result<*mut c_void>);
// Match(int, int, float, const String &, int)(Primitive, Primitive, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:265
// ("cv::linemod::Match::Match", vec![(pred!(mut, ["x", "y", "similarity", "class_id", "template_id"], ["int", "int", "float", "const cv::String*", "int"]), _)]),
pub fn cv_linemod_Match_Match_int_int_float_const_StringR_int(x: i32, y: i32, similarity: f32, class_id: *const c_char, template_id: i32, ocvrs_return: *mut Result<*mut c_void>);
// operator<(const Match &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:268
// ("cv::linemod::Match::operator<", vec![(pred!(const, ["rhs"], ["const cv::linemod::Match*"]), _)]),
pub fn cv_linemod_Match_operatorL_const_const_MatchR(instance: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<bool>);
// operator==(const Match &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:277
// ("cv::linemod::Match::operator==", vec![(pred!(const, ["rhs"], ["const cv::linemod::Match*"]), _)]),
pub fn cv_linemod_Match_operatorEQ_const_const_MatchR(instance: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::linemod::Match::implicitClone() generated
// ("cv::linemod::Match::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Match_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::linemod::Match::x() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:282
// ("cv::linemod::Match::x", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Match_propX_const(instance: *const c_void) -> i32;
// cv::linemod::Match::setX(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:282
// ("cv::linemod::Match::setX", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_linemod_Match_propX_const_int(instance: *mut c_void, val: i32);
// cv::linemod::Match::y() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:283
// ("cv::linemod::Match::y", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Match_propY_const(instance: *const c_void) -> i32;
// cv::linemod::Match::setY(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:283
// ("cv::linemod::Match::setY", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_linemod_Match_propY_const_int(instance: *mut c_void, val: i32);
// cv::linemod::Match::similarity() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:284
// ("cv::linemod::Match::similarity", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Match_propSimilarity_const(instance: *const c_void) -> f32;
// cv::linemod::Match::setSimilarity(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:284
// ("cv::linemod::Match::setSimilarity", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_linemod_Match_propSimilarity_const_float(instance: *mut c_void, val: f32);
// cv::linemod::Match::class_id() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:285
// ("cv::linemod::Match::class_id", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Match_propClass_id_const(instance: *const c_void) -> *mut c_void;
// cv::linemod::Match::setClass_id(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:285
// ("cv::linemod::Match::setClass_id", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
pub fn cv_linemod_Match_propClass_id_const_String(instance: *mut c_void, val: *const c_char);
// cv::linemod::Match::template_id() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:286
// ("cv::linemod::Match::template_id", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Match_propTemplate_id_const(instance: *const c_void) -> i32;
// cv::linemod::Match::setTemplate_id(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:286
// ("cv::linemod::Match::setTemplate_id", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_linemod_Match_propTemplate_id_const_int(instance: *mut c_void, val: i32);
// cv::linemod::Match::delete() generated
// ("cv::linemod::Match::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_Match_delete(instance: *mut c_void);
// process(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:132
// ("cv::linemod::Modality::process", vec![(pred!(const, ["src", "mask"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_linemod_Modality_process_const_const_MatR_const_MatR(instance: *const c_void, src: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::linemod::Modality::process(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:132
// ("cv::linemod::Modality::process", vec![(pred!(const, ["src"], ["const cv::Mat*"]), _)]),
pub fn cv_linemod_Modality_process_const_const_MatR(instance: *const c_void, src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// name()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:138
// ("cv::linemod::Modality::name", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Modality_name_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:140
// ("cv::linemod::Modality::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_linemod_Modality_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:141
// ("cv::linemod::Modality::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_linemod_Modality_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// create(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:150
// ("cv::linemod::Modality::create", vec![(pred!(mut, ["modality_type"], ["const cv::String*"]), _)]),
pub fn cv_linemod_Modality_create_const_StringR(modality_type: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// create(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:155
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
// quantize(Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:67
// ("cv::linemod::QuantizedPyramid::quantize", vec![(pred!(const, ["dst"], ["cv::Mat*"]), _)]),
pub fn cv_linemod_QuantizedPyramid_quantize_const_MatR(instance: *const c_void, dst: *mut c_void, ocvrs_return: *mut Result<()>);
// extractTemplate(Template &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:74
// ("cv::linemod::QuantizedPyramid::extractTemplate", vec![(pred!(const, ["templ"], ["cv::linemod::Template*"]), _)]),
pub fn cv_linemod_QuantizedPyramid_extractTemplate_const_TemplateR(instance: *const c_void, templ: *mut c_void, ocvrs_return: *mut Result<bool>);
// pyrDown()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:81
// ("cv::linemod::QuantizedPyramid::pyrDown", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_QuantizedPyramid_pyrDown(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::linemod::QuantizedPyramid::delete() generated
// ("cv::linemod::QuantizedPyramid::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_QuantizedPyramid_delete(instance: *mut c_void);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:48
// ("cv::linemod::Template::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_linemod_Template_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:49
// ("cv::linemod::Template::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_linemod_Template_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::linemod::Template::implicitClone() generated
// ("cv::linemod::Template::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Template_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::linemod::Template::defaultNew() generated
// ("cv::linemod::Template::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Template_defaultNew_const() -> *mut c_void;
// cv::linemod::Template::width() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:43
// ("cv::linemod::Template::width", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Template_propWidth_const(instance: *const c_void) -> i32;
// cv::linemod::Template::setWidth(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:43
// ("cv::linemod::Template::setWidth", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_linemod_Template_propWidth_const_int(instance: *mut c_void, val: i32);
// cv::linemod::Template::height() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:44
// ("cv::linemod::Template::height", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Template_propHeight_const(instance: *const c_void) -> i32;
// cv::linemod::Template::setHeight(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:44
// ("cv::linemod::Template::setHeight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_linemod_Template_propHeight_const_int(instance: *mut c_void, val: i32);
// cv::linemod::Template::pyramid_level() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:45
// ("cv::linemod::Template::pyramid_level", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Template_propPyramid_level_const(instance: *const c_void) -> i32;
// cv::linemod::Template::setPyramid_level(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:45
// ("cv::linemod::Template::setPyramid_level", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_linemod_Template_propPyramid_level_const_int(instance: *mut c_void, val: i32);
// cv::linemod::Template::features() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:46
// ("cv::linemod::Template::features", vec![(pred!(const, [], []), _)]),
pub fn cv_linemod_Template_propFeatures_const(instance: *const c_void) -> *mut c_void;
// cv::linemod::Template::setFeatures(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:46
// ("cv::linemod::Template::setFeatures", vec![(pred!(mut, ["val"], ["const std::vector<cv::linemod::Feature>"]), _)]),
pub fn cv_linemod_Template_propFeatures_const_vectorLFeatureG(instance: *mut c_void, val: *const c_void);
// cv::linemod::Template::delete() generated
// ("cv::linemod::Template::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_linemod_Template_delete(instance: *mut c_void);
