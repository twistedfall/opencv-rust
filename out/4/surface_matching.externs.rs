// ICP()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/icp.hpp:90
// ("cv::ppf_match_3d::ICP::ICP", vec![(pred!(mut, [], []), _)]),
pub fn cv_ppf_match_3d_ICP_ICP(ocvrs_return: *mut Result<*mut c_void>);
// ICP(const int, const float, const float, const int, const int, const int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/icp.hpp:117
// ("cv::ppf_match_3d::ICP::ICP", vec![(pred!(mut, ["iterations", "tolerence", "rejectionScale", "numLevels", "sampleType", "numMaxCorr"], ["const int", "const float", "const float", "const int", "const int", "const int"]), _)]),
pub fn cv_ppf_match_3d_ICP_ICP_const_int_const_float_const_float_const_int_const_int_const_int(iterations: i32, tolerence: f32, rejection_scale: f32, num_levels: i32, sample_type: i32, num_max_corr: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::ppf_match_3d::ICP::ICP(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/icp.hpp:117
// ("cv::ppf_match_3d::ICP::ICP", vec![(pred!(mut, ["iterations"], ["const int"]), _)]),
pub fn cv_ppf_match_3d_ICP_ICP_const_int(iterations: i32, ocvrs_return: *mut Result<*mut c_void>);
// registerModelToScene(const Mat &, const Mat &, double &, Matx44d &)(TraitClass, TraitClass, Indirect, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/icp.hpp:139
// ("cv::ppf_match_3d::ICP::registerModelToScene", vec![(pred!(mut, ["srcPC", "dstPC", "residual", "pose"], ["const cv::Mat*", "const cv::Mat*", "double*", "cv::Matx44d*"]), _)]),
pub fn cv_ppf_match_3d_ICP_registerModelToScene_const_MatR_const_MatR_doubleR_Matx44dR(instance: *mut c_void, src_pc: *const c_void, dst_pc: *const c_void, residual: *mut f64, pose: *mut core::Matx44d, ocvrs_return: *mut Result<i32>);
// registerModelToScene(const Mat &, const Mat &, std::vector<Pose3DPtr> &)(TraitClass, TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/icp.hpp:152
// ("cv::ppf_match_3d::ICP::registerModelToScene", vec![(pred!(mut, ["srcPC", "dstPC", "poses"], ["const cv::Mat*", "const cv::Mat*", "std::vector<cv::ppf_match_3d::Pose3DPtr>*"]), _)]),
pub fn cv_ppf_match_3d_ICP_registerModelToScene_const_MatR_const_MatR_vectorLPose3DPtrGR(instance: *mut c_void, src_pc: *const c_void, dst_pc: *const c_void, poses: *mut c_void, ocvrs_return: *mut Result<i32>);
// cv::ppf_match_3d::ICP::delete() generated
// ("cv::ppf_match_3d::ICP::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ppf_match_3d_ICP_delete(instance: *mut c_void);
// PPF3DDetector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/ppf_match_3d.hpp:104
// ("cv::ppf_match_3d::PPF3DDetector::PPF3DDetector", vec![(pred!(mut, [], []), _)]),
pub fn cv_ppf_match_3d_PPF3DDetector_PPF3DDetector(ocvrs_return: *mut Result<*mut c_void>);
// PPF3DDetector(const double, const double, const double)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/ppf_match_3d.hpp:112
// ("cv::ppf_match_3d::PPF3DDetector::PPF3DDetector", vec![(pred!(mut, ["relativeSamplingStep", "relativeDistanceStep", "numAngles"], ["const double", "const double", "const double"]), _)]),
pub fn cv_ppf_match_3d_PPF3DDetector_PPF3DDetector_const_double_const_double_const_double(relative_sampling_step: f64, relative_distance_step: f64, num_angles: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::ppf_match_3d::PPF3DDetector::PPF3DDetector(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/ppf_match_3d.hpp:112
// ("cv::ppf_match_3d::PPF3DDetector::PPF3DDetector", vec![(pred!(mut, ["relativeSamplingStep"], ["const double"]), _)]),
pub fn cv_ppf_match_3d_PPF3DDetector_PPF3DDetector_const_double(relative_sampling_step: f64, ocvrs_return: *mut Result<*mut c_void>);
// setSearchParams(const double, const double, const bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/ppf_match_3d.hpp:122
// ("cv::ppf_match_3d::PPF3DDetector::setSearchParams", vec![(pred!(mut, ["positionThreshold", "rotationThreshold", "useWeightedClustering"], ["const double", "const double", "const bool"]), _)]),
pub fn cv_ppf_match_3d_PPF3DDetector_setSearchParams_const_double_const_double_const_bool(instance: *mut c_void, position_threshold: f64, rotation_threshold: f64, use_weighted_clustering: bool, ocvrs_return: *mut Result<()>);
// cv::ppf_match_3d::PPF3DDetector::setSearchParams() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/ppf_match_3d.hpp:122
// ("cv::ppf_match_3d::PPF3DDetector::setSearchParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_ppf_match_3d_PPF3DDetector_setSearchParams(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// trainModel(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/ppf_match_3d.hpp:131
// ("cv::ppf_match_3d::PPF3DDetector::trainModel", vec![(pred!(mut, ["Model"], ["const cv::Mat*"]), _)]),
pub fn cv_ppf_match_3d_PPF3DDetector_trainModel_const_MatR(instance: *mut c_void, model: *const c_void, ocvrs_return: *mut Result<()>);
// match(const Mat &, std::vector<Pose3DPtr> &, const double, const double)(TraitClass, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/ppf_match_3d.hpp:141
// ("cv::ppf_match_3d::PPF3DDetector::match", vec![(pred!(mut, ["scene", "results", "relativeSceneSampleStep", "relativeSceneDistance"], ["const cv::Mat*", "std::vector<cv::ppf_match_3d::Pose3DPtr>*", "const double", "const double"]), _)]),
pub fn cv_ppf_match_3d_PPF3DDetector_match_const_MatR_vectorLPose3DPtrGR_const_double_const_double(instance: *mut c_void, scene: *const c_void, results: *mut c_void, relative_scene_sample_step: f64, relative_scene_distance: f64, ocvrs_return: *mut Result<()>);
// cv::ppf_match_3d::PPF3DDetector::match(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/ppf_match_3d.hpp:141
// ("cv::ppf_match_3d::PPF3DDetector::match", vec![(pred!(mut, ["scene", "results"], ["const cv::Mat*", "std::vector<cv::ppf_match_3d::Pose3DPtr>*"]), _)]),
pub fn cv_ppf_match_3d_PPF3DDetector_match_const_MatR_vectorLPose3DPtrGR(instance: *mut c_void, scene: *const c_void, results: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::ppf_match_3d::PPF3DDetector::delete() generated
// ("cv::ppf_match_3d::PPF3DDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ppf_match_3d_PPF3DDetector_delete(instance: *mut c_void);
// Pose3D()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:73
// ("cv::ppf_match_3d::Pose3D::Pose3D", vec![(pred!(mut, [], []), _)]),
pub fn cv_ppf_match_3d_Pose3D_Pose3D(ocvrs_return: *mut Result<*mut c_void>);
// Pose3D(double, size_t, size_t)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:83
// ("cv::ppf_match_3d::Pose3D::Pose3D", vec![(pred!(mut, ["Alpha", "ModelIndex", "NumVotes"], ["double", "size_t", "size_t"]), _)]),
pub fn cv_ppf_match_3d_Pose3D_Pose3D_double_size_t_size_t(alpha: f64, model_index: size_t, num_votes: size_t, ocvrs_return: *mut Result<*mut c_void>);
// cv::ppf_match_3d::Pose3D::Pose3D(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:83
// ("cv::ppf_match_3d::Pose3D::Pose3D", vec![(pred!(mut, ["Alpha"], ["double"]), _)]),
pub fn cv_ppf_match_3d_Pose3D_Pose3D_double(alpha: f64, ocvrs_return: *mut Result<*mut c_void>);
// updatePose(Matx44d &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:97
// ("cv::ppf_match_3d::Pose3D::updatePose", vec![(pred!(mut, ["NewPose"], ["cv::Matx44d*"]), _)]),
pub fn cv_ppf_match_3d_Pose3D_updatePose_Matx44dR(instance: *mut c_void, new_pose: *mut core::Matx44d, ocvrs_return: *mut Result<()>);
// updatePose(Matx33d &, Vec3d &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:102
// ("cv::ppf_match_3d::Pose3D::updatePose", vec![(pred!(mut, ["NewR", "NewT"], ["cv::Matx33d*", "cv::Vec3d*"]), _)]),
pub fn cv_ppf_match_3d_Pose3D_updatePose_Matx33dR_Vec3dR(instance: *mut c_void, new_r: *mut core::Matx33d, new_t: *mut core::Vec3d, ocvrs_return: *mut Result<()>);
// updatePoseQuat(Vec4d &, Vec3d &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:107
// ("cv::ppf_match_3d::Pose3D::updatePoseQuat", vec![(pred!(mut, ["Q", "NewT"], ["cv::Vec4d*", "cv::Vec3d*"]), _)]),
pub fn cv_ppf_match_3d_Pose3D_updatePoseQuat_Vec4dR_Vec3dR(instance: *mut c_void, q: *mut core::Vec4d, new_t: *mut core::Vec3d, ocvrs_return: *mut Result<()>);
// appendPose(Matx44d &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:113
// ("cv::ppf_match_3d::Pose3D::appendPose", vec![(pred!(mut, ["IncrementalPose"], ["cv::Matx44d*"]), _)]),
pub fn cv_ppf_match_3d_Pose3D_appendPose_Matx44dR(instance: *mut c_void, incremental_pose: *mut core::Matx44d, ocvrs_return: *mut Result<()>);
// printPose()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:114
// ("cv::ppf_match_3d::Pose3D::printPose", vec![(pred!(mut, [], []), _)]),
pub fn cv_ppf_match_3d_Pose3D_printPose(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// clone()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:116
// ("cv::ppf_match_3d::Pose3D::clone", vec![(pred!(mut, [], []), _)]),
pub fn cv_ppf_match_3d_Pose3D_clone(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// writePose(const std::string &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:120
// ("cv::ppf_match_3d::Pose3D::writePose", vec![(pred!(mut, ["FileName"], ["const std::string*"]), _)]),
pub fn cv_ppf_match_3d_Pose3D_writePose_const_stringR(instance: *mut c_void, file_name: *const c_char, ocvrs_return: *mut Result<i32>);
// readPose(const std::string &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:121
// ("cv::ppf_match_3d::Pose3D::readPose", vec![(pred!(mut, ["FileName"], ["const std::string*"]), _)]),
pub fn cv_ppf_match_3d_Pose3D_readPose_const_stringR(instance: *mut c_void, file_name: *const c_char, ocvrs_return: *mut Result<i32>);
// cv::ppf_match_3d::Pose3D::alpha() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:125
// ("cv::ppf_match_3d::Pose3D::alpha", vec![(pred!(const, [], []), _)]),
pub fn cv_ppf_match_3d_Pose3D_propAlpha_const(instance: *const c_void) -> f64;
// cv::ppf_match_3d::Pose3D::setAlpha(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:125
// ("cv::ppf_match_3d::Pose3D::setAlpha", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_ppf_match_3d_Pose3D_propAlpha_const_double(instance: *mut c_void, val: f64);
// cv::ppf_match_3d::Pose3D::residual() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:125
// ("cv::ppf_match_3d::Pose3D::residual", vec![(pred!(const, [], []), _)]),
pub fn cv_ppf_match_3d_Pose3D_propResidual_const(instance: *const c_void) -> f64;
// cv::ppf_match_3d::Pose3D::setResidual(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:125
// ("cv::ppf_match_3d::Pose3D::setResidual", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_ppf_match_3d_Pose3D_propResidual_const_double(instance: *mut c_void, val: f64);
// cv::ppf_match_3d::Pose3D::modelIndex() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:126
// ("cv::ppf_match_3d::Pose3D::modelIndex", vec![(pred!(const, [], []), _)]),
pub fn cv_ppf_match_3d_Pose3D_propModelIndex_const(instance: *const c_void) -> size_t;
// cv::ppf_match_3d::Pose3D::setModelIndex(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:126
// ("cv::ppf_match_3d::Pose3D::setModelIndex", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
pub fn cv_ppf_match_3d_Pose3D_propModelIndex_const_size_t(instance: *mut c_void, val: size_t);
// cv::ppf_match_3d::Pose3D::numVotes() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:126
// ("cv::ppf_match_3d::Pose3D::numVotes", vec![(pred!(const, [], []), _)]),
pub fn cv_ppf_match_3d_Pose3D_propNumVotes_const(instance: *const c_void) -> size_t;
// cv::ppf_match_3d::Pose3D::setNumVotes(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:126
// ("cv::ppf_match_3d::Pose3D::setNumVotes", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
pub fn cv_ppf_match_3d_Pose3D_propNumVotes_const_size_t(instance: *mut c_void, val: size_t);
// cv::ppf_match_3d::Pose3D::pose() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:127
// ("cv::ppf_match_3d::Pose3D::pose", vec![(pred!(const, [], []), _)]),
pub fn cv_ppf_match_3d_Pose3D_propPose_const(instance: *const c_void, ocvrs_return: *mut core::Matx44d);
// cv::ppf_match_3d::Pose3D::setPose(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:127
// ("cv::ppf_match_3d::Pose3D::setPose", vec![(pred!(mut, ["val"], ["const cv::Matx44d"]), _)]),
pub fn cv_ppf_match_3d_Pose3D_propPose_const_Matx44d(instance: *mut c_void, val: *const core::Matx44d);
// cv::ppf_match_3d::Pose3D::angle() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:128
// ("cv::ppf_match_3d::Pose3D::angle", vec![(pred!(const, [], []), _)]),
pub fn cv_ppf_match_3d_Pose3D_propAngle_const(instance: *const c_void) -> f64;
// cv::ppf_match_3d::Pose3D::setAngle(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:128
// ("cv::ppf_match_3d::Pose3D::setAngle", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_ppf_match_3d_Pose3D_propAngle_const_double(instance: *mut c_void, val: f64);
// cv::ppf_match_3d::Pose3D::t() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:129
// ("cv::ppf_match_3d::Pose3D::t", vec![(pred!(const, [], []), _)]),
pub fn cv_ppf_match_3d_Pose3D_propT_const(instance: *const c_void, ocvrs_return: *mut core::Vec3d);
// cv::ppf_match_3d::Pose3D::setT(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:129
// ("cv::ppf_match_3d::Pose3D::setT", vec![(pred!(mut, ["val"], ["const cv::Vec3d"]), _)]),
pub fn cv_ppf_match_3d_Pose3D_propT_const_Vec3d(instance: *mut c_void, val: *const core::Vec3d);
// cv::ppf_match_3d::Pose3D::q() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:130
// ("cv::ppf_match_3d::Pose3D::q", vec![(pred!(const, [], []), _)]),
pub fn cv_ppf_match_3d_Pose3D_propQ_const(instance: *const c_void, ocvrs_return: *mut core::Vec4d);
// cv::ppf_match_3d::Pose3D::setQ(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:130
// ("cv::ppf_match_3d::Pose3D::setQ", vec![(pred!(mut, ["val"], ["const cv::Vec4d"]), _)]),
pub fn cv_ppf_match_3d_Pose3D_propQ_const_Vec4d(instance: *mut c_void, val: *const core::Vec4d);
// cv::ppf_match_3d::Pose3D::delete() generated
// ("cv::ppf_match_3d::Pose3D::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ppf_match_3d_Pose3D_delete(instance: *mut c_void);
// PoseCluster3D()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:141
// ("cv::ppf_match_3d::PoseCluster3D::PoseCluster3D", vec![(pred!(mut, [], []), _)]),
pub fn cv_ppf_match_3d_PoseCluster3D_PoseCluster3D(ocvrs_return: *mut Result<*mut c_void>);
// PoseCluster3D(Pose3DPtr)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:147
// ("cv::ppf_match_3d::PoseCluster3D::PoseCluster3D", vec![(pred!(mut, ["newPose"], ["cv::ppf_match_3d::Pose3DPtr"]), _)]),
pub fn cv_ppf_match_3d_PoseCluster3D_PoseCluster3D_Pose3DPtr(new_pose: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// PoseCluster3D(Pose3DPtr, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:155
// ("cv::ppf_match_3d::PoseCluster3D::PoseCluster3D", vec![(pred!(mut, ["newPose", "newId"], ["cv::ppf_match_3d::Pose3DPtr", "int"]), _)]),
pub fn cv_ppf_match_3d_PoseCluster3D_PoseCluster3D_Pose3DPtr_int(new_pose: *mut c_void, new_id: i32, ocvrs_return: *mut Result<*mut c_void>);
// addPose(Pose3DPtr)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:170
// ("cv::ppf_match_3d::PoseCluster3D::addPose", vec![(pred!(mut, ["newPose"], ["cv::ppf_match_3d::Pose3DPtr"]), _)]),
pub fn cv_ppf_match_3d_PoseCluster3D_addPose_Pose3DPtr(instance: *mut c_void, new_pose: *mut c_void, ocvrs_return: *mut Result<()>);
// writePoseCluster(const std::string &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:174
// ("cv::ppf_match_3d::PoseCluster3D::writePoseCluster", vec![(pred!(mut, ["FileName"], ["const std::string*"]), _)]),
pub fn cv_ppf_match_3d_PoseCluster3D_writePoseCluster_const_stringR(instance: *mut c_void, file_name: *const c_char, ocvrs_return: *mut Result<i32>);
// readPoseCluster(const std::string &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:175
// ("cv::ppf_match_3d::PoseCluster3D::readPoseCluster", vec![(pred!(mut, ["FileName"], ["const std::string*"]), _)]),
pub fn cv_ppf_match_3d_PoseCluster3D_readPoseCluster_const_stringR(instance: *mut c_void, file_name: *const c_char, ocvrs_return: *mut Result<i32>);
// cv::ppf_match_3d::PoseCluster3D::poseList() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:177
// ("cv::ppf_match_3d::PoseCluster3D::poseList", vec![(pred!(const, [], []), _)]),
pub fn cv_ppf_match_3d_PoseCluster3D_propPoseList_const(instance: *const c_void) -> *mut c_void;
// cv::ppf_match_3d::PoseCluster3D::setPoseList(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:177
// ("cv::ppf_match_3d::PoseCluster3D::setPoseList", vec![(pred!(mut, ["val"], ["const std::vector<cv::ppf_match_3d::Pose3DPtr>"]), _)]),
pub fn cv_ppf_match_3d_PoseCluster3D_propPoseList_const_vectorLPose3DPtrG(instance: *mut c_void, val: *const c_void);
// cv::ppf_match_3d::PoseCluster3D::numVotes() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:178
// ("cv::ppf_match_3d::PoseCluster3D::numVotes", vec![(pred!(const, [], []), _)]),
pub fn cv_ppf_match_3d_PoseCluster3D_propNumVotes_const(instance: *const c_void) -> size_t;
// cv::ppf_match_3d::PoseCluster3D::setNumVotes(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:178
// ("cv::ppf_match_3d::PoseCluster3D::setNumVotes", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
pub fn cv_ppf_match_3d_PoseCluster3D_propNumVotes_const_size_t(instance: *mut c_void, val: size_t);
// cv::ppf_match_3d::PoseCluster3D::id() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:179
// ("cv::ppf_match_3d::PoseCluster3D::id", vec![(pred!(const, [], []), _)]),
pub fn cv_ppf_match_3d_PoseCluster3D_propId_const(instance: *const c_void) -> i32;
// cv::ppf_match_3d::PoseCluster3D::setId(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/surface_matching/pose_3d.hpp:179
// ("cv::ppf_match_3d::PoseCluster3D::setId", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_ppf_match_3d_PoseCluster3D_propId_const_int(instance: *mut c_void, val: i32);
// cv::ppf_match_3d::PoseCluster3D::delete() generated
// ("cv::ppf_match_3d::PoseCluster3D::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ppf_match_3d_PoseCluster3D_delete(instance: *mut c_void);
