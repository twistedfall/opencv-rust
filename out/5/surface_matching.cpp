#include "ocvrs_common.hpp"
#include <opencv2/surface_matching.hpp>
#include "surface_matching_types.hpp"

extern "C" {
	// ICP()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/icp.hpp:90
	// ("cv::ppf_match_3d::ICP::ICP", vec![(pred!(mut, [], []), _)]),
	void cv_ppf_match_3d_ICP_ICP(Result<cv::ppf_match_3d::ICP*>* ocvrs_return) {
		try {
			cv::ppf_match_3d::ICP* ret = new cv::ppf_match_3d::ICP();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ICP(const int, const float, const float, const int, const int, const int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/icp.hpp:117
	// ("cv::ppf_match_3d::ICP::ICP", vec![(pred!(mut, ["iterations", "tolerence", "rejectionScale", "numLevels", "sampleType", "numMaxCorr"], ["const int", "const float", "const float", "const int", "const int", "const int"]), _)]),
	void cv_ppf_match_3d_ICP_ICP_const_int_const_float_const_float_const_int_const_int_const_int(const int iterations, const float tolerence, const float rejectionScale, const int numLevels, const int sampleType, const int numMaxCorr, Result<cv::ppf_match_3d::ICP*>* ocvrs_return) {
		try {
			cv::ppf_match_3d::ICP* ret = new cv::ppf_match_3d::ICP(iterations, tolerence, rejectionScale, numLevels, sampleType, numMaxCorr);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ppf_match_3d::ICP::ICP(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/icp.hpp:117
	// ("cv::ppf_match_3d::ICP::ICP", vec![(pred!(mut, ["iterations"], ["const int"]), _)]),
	void cv_ppf_match_3d_ICP_ICP_const_int(const int iterations, Result<cv::ppf_match_3d::ICP*>* ocvrs_return) {
		try {
			cv::ppf_match_3d::ICP* ret = new cv::ppf_match_3d::ICP(iterations);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// registerModelToScene(const Mat &, const Mat &, double &, Matx44d &)(TraitClass, TraitClass, Indirect, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/icp.hpp:139
	// ("cv::ppf_match_3d::ICP::registerModelToScene", vec![(pred!(mut, ["srcPC", "dstPC", "residual", "pose"], ["const cv::Mat*", "const cv::Mat*", "double*", "cv::Matx44d*"]), _)]),
	void cv_ppf_match_3d_ICP_registerModelToScene_const_MatR_const_MatR_doubleR_Matx44dR(cv::ppf_match_3d::ICP* instance, const cv::Mat* srcPC, const cv::Mat* dstPC, double* residual, cv::Matx44d* pose, Result<int>* ocvrs_return) {
		try {
			int ret = instance->registerModelToScene(*srcPC, *dstPC, *residual, *pose);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// registerModelToScene(const Mat &, const Mat &, std::vector<Pose3DPtr> &)(TraitClass, TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/icp.hpp:152
	// ("cv::ppf_match_3d::ICP::registerModelToScene", vec![(pred!(mut, ["srcPC", "dstPC", "poses"], ["const cv::Mat*", "const cv::Mat*", "std::vector<cv::ppf_match_3d::Pose3DPtr>*"]), _)]),
	void cv_ppf_match_3d_ICP_registerModelToScene_const_MatR_const_MatR_vectorLPose3DPtrGR(cv::ppf_match_3d::ICP* instance, const cv::Mat* srcPC, const cv::Mat* dstPC, std::vector<cv::ppf_match_3d::Pose3DPtr>* poses, Result<int>* ocvrs_return) {
		try {
			int ret = instance->registerModelToScene(*srcPC, *dstPC, *poses);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ppf_match_3d::ICP::delete() generated
	// ("cv::ppf_match_3d::ICP::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ppf_match_3d_ICP_delete(cv::ppf_match_3d::ICP* instance) {
			delete instance;
	}

	// PPF3DDetector()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/ppf_match_3d.hpp:104
	// ("cv::ppf_match_3d::PPF3DDetector::PPF3DDetector", vec![(pred!(mut, [], []), _)]),
	void cv_ppf_match_3d_PPF3DDetector_PPF3DDetector(Result<cv::ppf_match_3d::PPF3DDetector*>* ocvrs_return) {
		try {
			cv::ppf_match_3d::PPF3DDetector* ret = new cv::ppf_match_3d::PPF3DDetector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// PPF3DDetector(const double, const double, const double)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/ppf_match_3d.hpp:112
	// ("cv::ppf_match_3d::PPF3DDetector::PPF3DDetector", vec![(pred!(mut, ["relativeSamplingStep", "relativeDistanceStep", "numAngles"], ["const double", "const double", "const double"]), _)]),
	void cv_ppf_match_3d_PPF3DDetector_PPF3DDetector_const_double_const_double_const_double(const double relativeSamplingStep, const double relativeDistanceStep, const double numAngles, Result<cv::ppf_match_3d::PPF3DDetector*>* ocvrs_return) {
		try {
			cv::ppf_match_3d::PPF3DDetector* ret = new cv::ppf_match_3d::PPF3DDetector(relativeSamplingStep, relativeDistanceStep, numAngles);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ppf_match_3d::PPF3DDetector::PPF3DDetector(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/ppf_match_3d.hpp:112
	// ("cv::ppf_match_3d::PPF3DDetector::PPF3DDetector", vec![(pred!(mut, ["relativeSamplingStep"], ["const double"]), _)]),
	void cv_ppf_match_3d_PPF3DDetector_PPF3DDetector_const_double(const double relativeSamplingStep, Result<cv::ppf_match_3d::PPF3DDetector*>* ocvrs_return) {
		try {
			cv::ppf_match_3d::PPF3DDetector* ret = new cv::ppf_match_3d::PPF3DDetector(relativeSamplingStep);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSearchParams(const double, const double, const bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/ppf_match_3d.hpp:122
	// ("cv::ppf_match_3d::PPF3DDetector::setSearchParams", vec![(pred!(mut, ["positionThreshold", "rotationThreshold", "useWeightedClustering"], ["const double", "const double", "const bool"]), _)]),
	void cv_ppf_match_3d_PPF3DDetector_setSearchParams_const_double_const_double_const_bool(cv::ppf_match_3d::PPF3DDetector* instance, const double positionThreshold, const double rotationThreshold, const bool useWeightedClustering, ResultVoid* ocvrs_return) {
		try {
			instance->setSearchParams(positionThreshold, rotationThreshold, useWeightedClustering);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ppf_match_3d::PPF3DDetector::setSearchParams() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/ppf_match_3d.hpp:122
	// ("cv::ppf_match_3d::PPF3DDetector::setSearchParams", vec![(pred!(mut, [], []), _)]),
	void cv_ppf_match_3d_PPF3DDetector_setSearchParams(cv::ppf_match_3d::PPF3DDetector* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setSearchParams();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// trainModel(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/ppf_match_3d.hpp:131
	// ("cv::ppf_match_3d::PPF3DDetector::trainModel", vec![(pred!(mut, ["Model"], ["const cv::Mat*"]), _)]),
	void cv_ppf_match_3d_PPF3DDetector_trainModel_const_MatR(cv::ppf_match_3d::PPF3DDetector* instance, const cv::Mat* Model, ResultVoid* ocvrs_return) {
		try {
			instance->trainModel(*Model);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// match(const Mat &, std::vector<Pose3DPtr> &, const double, const double)(TraitClass, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/ppf_match_3d.hpp:141
	// ("cv::ppf_match_3d::PPF3DDetector::match", vec![(pred!(mut, ["scene", "results", "relativeSceneSampleStep", "relativeSceneDistance"], ["const cv::Mat*", "std::vector<cv::ppf_match_3d::Pose3DPtr>*", "const double", "const double"]), _)]),
	void cv_ppf_match_3d_PPF3DDetector_match_const_MatR_vectorLPose3DPtrGR_const_double_const_double(cv::ppf_match_3d::PPF3DDetector* instance, const cv::Mat* scene, std::vector<cv::ppf_match_3d::Pose3DPtr>* results, const double relativeSceneSampleStep, const double relativeSceneDistance, ResultVoid* ocvrs_return) {
		try {
			instance->match(*scene, *results, relativeSceneSampleStep, relativeSceneDistance);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ppf_match_3d::PPF3DDetector::match(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/ppf_match_3d.hpp:141
	// ("cv::ppf_match_3d::PPF3DDetector::match", vec![(pred!(mut, ["scene", "results"], ["const cv::Mat*", "std::vector<cv::ppf_match_3d::Pose3DPtr>*"]), _)]),
	void cv_ppf_match_3d_PPF3DDetector_match_const_MatR_vectorLPose3DPtrGR(cv::ppf_match_3d::PPF3DDetector* instance, const cv::Mat* scene, std::vector<cv::ppf_match_3d::Pose3DPtr>* results, ResultVoid* ocvrs_return) {
		try {
			instance->match(*scene, *results);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ppf_match_3d::PPF3DDetector::delete() generated
	// ("cv::ppf_match_3d::PPF3DDetector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ppf_match_3d_PPF3DDetector_delete(cv::ppf_match_3d::PPF3DDetector* instance) {
			delete instance;
	}

	// Pose3D()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:73
	// ("cv::ppf_match_3d::Pose3D::Pose3D", vec![(pred!(mut, [], []), _)]),
	void cv_ppf_match_3d_Pose3D_Pose3D(Result<cv::ppf_match_3d::Pose3D*>* ocvrs_return) {
		try {
			cv::ppf_match_3d::Pose3D* ret = new cv::ppf_match_3d::Pose3D();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Pose3D(double, size_t, size_t)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:83
	// ("cv::ppf_match_3d::Pose3D::Pose3D", vec![(pred!(mut, ["Alpha", "ModelIndex", "NumVotes"], ["double", "size_t", "size_t"]), _)]),
	void cv_ppf_match_3d_Pose3D_Pose3D_double_size_t_size_t(double Alpha, size_t ModelIndex, size_t NumVotes, Result<cv::ppf_match_3d::Pose3D*>* ocvrs_return) {
		try {
			cv::ppf_match_3d::Pose3D* ret = new cv::ppf_match_3d::Pose3D(Alpha, ModelIndex, NumVotes);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ppf_match_3d::Pose3D::Pose3D(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:83
	// ("cv::ppf_match_3d::Pose3D::Pose3D", vec![(pred!(mut, ["Alpha"], ["double"]), _)]),
	void cv_ppf_match_3d_Pose3D_Pose3D_double(double Alpha, Result<cv::ppf_match_3d::Pose3D*>* ocvrs_return) {
		try {
			cv::ppf_match_3d::Pose3D* ret = new cv::ppf_match_3d::Pose3D(Alpha);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// updatePose(Matx44d &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:97
	// ("cv::ppf_match_3d::Pose3D::updatePose", vec![(pred!(mut, ["NewPose"], ["cv::Matx44d*"]), _)]),
	void cv_ppf_match_3d_Pose3D_updatePose_Matx44dR(cv::ppf_match_3d::Pose3D* instance, cv::Matx44d* NewPose, ResultVoid* ocvrs_return) {
		try {
			instance->updatePose(*NewPose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// updatePose(Matx33d &, Vec3d &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:102
	// ("cv::ppf_match_3d::Pose3D::updatePose", vec![(pred!(mut, ["NewR", "NewT"], ["cv::Matx33d*", "cv::Vec3d*"]), _)]),
	void cv_ppf_match_3d_Pose3D_updatePose_Matx33dR_Vec3dR(cv::ppf_match_3d::Pose3D* instance, cv::Matx33d* NewR, cv::Vec3d* NewT, ResultVoid* ocvrs_return) {
		try {
			instance->updatePose(*NewR, *NewT);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// updatePoseQuat(Vec4d &, Vec3d &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:107
	// ("cv::ppf_match_3d::Pose3D::updatePoseQuat", vec![(pred!(mut, ["Q", "NewT"], ["cv::Vec4d*", "cv::Vec3d*"]), _)]),
	void cv_ppf_match_3d_Pose3D_updatePoseQuat_Vec4dR_Vec3dR(cv::ppf_match_3d::Pose3D* instance, cv::Vec4d* Q, cv::Vec3d* NewT, ResultVoid* ocvrs_return) {
		try {
			instance->updatePoseQuat(*Q, *NewT);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// appendPose(Matx44d &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:113
	// ("cv::ppf_match_3d::Pose3D::appendPose", vec![(pred!(mut, ["IncrementalPose"], ["cv::Matx44d*"]), _)]),
	void cv_ppf_match_3d_Pose3D_appendPose_Matx44dR(cv::ppf_match_3d::Pose3D* instance, cv::Matx44d* IncrementalPose, ResultVoid* ocvrs_return) {
		try {
			instance->appendPose(*IncrementalPose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// printPose()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:114
	// ("cv::ppf_match_3d::Pose3D::printPose", vec![(pred!(mut, [], []), _)]),
	void cv_ppf_match_3d_Pose3D_printPose(cv::ppf_match_3d::Pose3D* instance, ResultVoid* ocvrs_return) {
		try {
			instance->printPose();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clone()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:116
	// ("cv::ppf_match_3d::Pose3D::clone", vec![(pred!(mut, [], []), _)]),
	void cv_ppf_match_3d_Pose3D_clone(cv::ppf_match_3d::Pose3D* instance, Result<cv::ppf_match_3d::Pose3DPtr*>* ocvrs_return) {
		try {
			cv::ppf_match_3d::Pose3DPtr ret = instance->clone();
			Ok(new cv::ppf_match_3d::Pose3DPtr(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writePose(const std::string &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:120
	// ("cv::ppf_match_3d::Pose3D::writePose", vec![(pred!(mut, ["FileName"], ["const std::string*"]), _)]),
	void cv_ppf_match_3d_Pose3D_writePose_const_stringR(cv::ppf_match_3d::Pose3D* instance, const char* FileName, Result<int>* ocvrs_return) {
		try {
			int ret = instance->writePose(std::string(FileName));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readPose(const std::string &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:121
	// ("cv::ppf_match_3d::Pose3D::readPose", vec![(pred!(mut, ["FileName"], ["const std::string*"]), _)]),
	void cv_ppf_match_3d_Pose3D_readPose_const_stringR(cv::ppf_match_3d::Pose3D* instance, const char* FileName, Result<int>* ocvrs_return) {
		try {
			int ret = instance->readPose(std::string(FileName));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ppf_match_3d::Pose3D::alpha() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:125
	// ("cv::ppf_match_3d::Pose3D::alpha", vec![(pred!(const, [], []), _)]),
	double cv_ppf_match_3d_Pose3D_propAlpha_const(const cv::ppf_match_3d::Pose3D* instance) {
			double ret = instance->alpha;
			return ret;
	}

	// cv::ppf_match_3d::Pose3D::setAlpha(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:125
	// ("cv::ppf_match_3d::Pose3D::setAlpha", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_ppf_match_3d_Pose3D_propAlpha_const_double(cv::ppf_match_3d::Pose3D* instance, const double val) {
			instance->alpha = val;
	}

	// cv::ppf_match_3d::Pose3D::residual() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:125
	// ("cv::ppf_match_3d::Pose3D::residual", vec![(pred!(const, [], []), _)]),
	double cv_ppf_match_3d_Pose3D_propResidual_const(const cv::ppf_match_3d::Pose3D* instance) {
			double ret = instance->residual;
			return ret;
	}

	// cv::ppf_match_3d::Pose3D::setResidual(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:125
	// ("cv::ppf_match_3d::Pose3D::setResidual", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_ppf_match_3d_Pose3D_propResidual_const_double(cv::ppf_match_3d::Pose3D* instance, const double val) {
			instance->residual = val;
	}

	// cv::ppf_match_3d::Pose3D::modelIndex() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:126
	// ("cv::ppf_match_3d::Pose3D::modelIndex", vec![(pred!(const, [], []), _)]),
	size_t cv_ppf_match_3d_Pose3D_propModelIndex_const(const cv::ppf_match_3d::Pose3D* instance) {
			size_t ret = instance->modelIndex;
			return ret;
	}

	// cv::ppf_match_3d::Pose3D::setModelIndex(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:126
	// ("cv::ppf_match_3d::Pose3D::setModelIndex", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
	void cv_ppf_match_3d_Pose3D_propModelIndex_const_size_t(cv::ppf_match_3d::Pose3D* instance, const size_t val) {
			instance->modelIndex = val;
	}

	// cv::ppf_match_3d::Pose3D::numVotes() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:126
	// ("cv::ppf_match_3d::Pose3D::numVotes", vec![(pred!(const, [], []), _)]),
	size_t cv_ppf_match_3d_Pose3D_propNumVotes_const(const cv::ppf_match_3d::Pose3D* instance) {
			size_t ret = instance->numVotes;
			return ret;
	}

	// cv::ppf_match_3d::Pose3D::setNumVotes(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:126
	// ("cv::ppf_match_3d::Pose3D::setNumVotes", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
	void cv_ppf_match_3d_Pose3D_propNumVotes_const_size_t(cv::ppf_match_3d::Pose3D* instance, const size_t val) {
			instance->numVotes = val;
	}

	// cv::ppf_match_3d::Pose3D::pose() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:127
	// ("cv::ppf_match_3d::Pose3D::pose", vec![(pred!(const, [], []), _)]),
	void cv_ppf_match_3d_Pose3D_propPose_const(const cv::ppf_match_3d::Pose3D* instance, cv::Matx44d* ocvrs_return) {
			cv::Matx44d ret = instance->pose;
			*ocvrs_return = ret;
	}

	// cv::ppf_match_3d::Pose3D::setPose(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:127
	// ("cv::ppf_match_3d::Pose3D::setPose", vec![(pred!(mut, ["val"], ["const cv::Matx44d"]), _)]),
	void cv_ppf_match_3d_Pose3D_propPose_const_Matx44d(cv::ppf_match_3d::Pose3D* instance, const cv::Matx44d* val) {
			instance->pose = *val;
	}

	// cv::ppf_match_3d::Pose3D::angle() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:128
	// ("cv::ppf_match_3d::Pose3D::angle", vec![(pred!(const, [], []), _)]),
	double cv_ppf_match_3d_Pose3D_propAngle_const(const cv::ppf_match_3d::Pose3D* instance) {
			double ret = instance->angle;
			return ret;
	}

	// cv::ppf_match_3d::Pose3D::setAngle(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:128
	// ("cv::ppf_match_3d::Pose3D::setAngle", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_ppf_match_3d_Pose3D_propAngle_const_double(cv::ppf_match_3d::Pose3D* instance, const double val) {
			instance->angle = val;
	}

	// cv::ppf_match_3d::Pose3D::t() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:129
	// ("cv::ppf_match_3d::Pose3D::t", vec![(pred!(const, [], []), _)]),
	void cv_ppf_match_3d_Pose3D_propT_const(const cv::ppf_match_3d::Pose3D* instance, cv::Vec3d* ocvrs_return) {
			cv::Vec3d ret = instance->t;
			*ocvrs_return = ret;
	}

	// cv::ppf_match_3d::Pose3D::setT(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:129
	// ("cv::ppf_match_3d::Pose3D::setT", vec![(pred!(mut, ["val"], ["const cv::Vec3d"]), _)]),
	void cv_ppf_match_3d_Pose3D_propT_const_Vec3d(cv::ppf_match_3d::Pose3D* instance, const cv::Vec3d* val) {
			instance->t = *val;
	}

	// cv::ppf_match_3d::Pose3D::q() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:130
	// ("cv::ppf_match_3d::Pose3D::q", vec![(pred!(const, [], []), _)]),
	void cv_ppf_match_3d_Pose3D_propQ_const(const cv::ppf_match_3d::Pose3D* instance, cv::Vec4d* ocvrs_return) {
			cv::Vec4d ret = instance->q;
			*ocvrs_return = ret;
	}

	// cv::ppf_match_3d::Pose3D::setQ(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:130
	// ("cv::ppf_match_3d::Pose3D::setQ", vec![(pred!(mut, ["val"], ["const cv::Vec4d"]), _)]),
	void cv_ppf_match_3d_Pose3D_propQ_const_Vec4d(cv::ppf_match_3d::Pose3D* instance, const cv::Vec4d* val) {
			instance->q = *val;
	}

	// cv::ppf_match_3d::Pose3D::delete() generated
	// ("cv::ppf_match_3d::Pose3D::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ppf_match_3d_Pose3D_delete(cv::ppf_match_3d::Pose3D* instance) {
			delete instance;
	}

	// PoseCluster3D()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:141
	// ("cv::ppf_match_3d::PoseCluster3D::PoseCluster3D", vec![(pred!(mut, [], []), _)]),
	void cv_ppf_match_3d_PoseCluster3D_PoseCluster3D(Result<cv::ppf_match_3d::PoseCluster3D*>* ocvrs_return) {
		try {
			cv::ppf_match_3d::PoseCluster3D* ret = new cv::ppf_match_3d::PoseCluster3D();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// PoseCluster3D(Pose3DPtr)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:147
	// ("cv::ppf_match_3d::PoseCluster3D::PoseCluster3D", vec![(pred!(mut, ["newPose"], ["cv::ppf_match_3d::Pose3DPtr"]), _)]),
	void cv_ppf_match_3d_PoseCluster3D_PoseCluster3D_Pose3DPtr(cv::ppf_match_3d::Pose3DPtr* newPose, Result<cv::ppf_match_3d::PoseCluster3D*>* ocvrs_return) {
		try {
			cv::ppf_match_3d::PoseCluster3D* ret = new cv::ppf_match_3d::PoseCluster3D(*newPose);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// PoseCluster3D(Pose3DPtr, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:155
	// ("cv::ppf_match_3d::PoseCluster3D::PoseCluster3D", vec![(pred!(mut, ["newPose", "newId"], ["cv::ppf_match_3d::Pose3DPtr", "int"]), _)]),
	void cv_ppf_match_3d_PoseCluster3D_PoseCluster3D_Pose3DPtr_int(cv::ppf_match_3d::Pose3DPtr* newPose, int newId, Result<cv::ppf_match_3d::PoseCluster3D*>* ocvrs_return) {
		try {
			cv::ppf_match_3d::PoseCluster3D* ret = new cv::ppf_match_3d::PoseCluster3D(*newPose, newId);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addPose(Pose3DPtr)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:170
	// ("cv::ppf_match_3d::PoseCluster3D::addPose", vec![(pred!(mut, ["newPose"], ["cv::ppf_match_3d::Pose3DPtr"]), _)]),
	void cv_ppf_match_3d_PoseCluster3D_addPose_Pose3DPtr(cv::ppf_match_3d::PoseCluster3D* instance, cv::ppf_match_3d::Pose3DPtr* newPose, ResultVoid* ocvrs_return) {
		try {
			instance->addPose(*newPose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writePoseCluster(const std::string &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:174
	// ("cv::ppf_match_3d::PoseCluster3D::writePoseCluster", vec![(pred!(mut, ["FileName"], ["const std::string*"]), _)]),
	void cv_ppf_match_3d_PoseCluster3D_writePoseCluster_const_stringR(cv::ppf_match_3d::PoseCluster3D* instance, const char* FileName, Result<int>* ocvrs_return) {
		try {
			int ret = instance->writePoseCluster(std::string(FileName));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readPoseCluster(const std::string &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:175
	// ("cv::ppf_match_3d::PoseCluster3D::readPoseCluster", vec![(pred!(mut, ["FileName"], ["const std::string*"]), _)]),
	void cv_ppf_match_3d_PoseCluster3D_readPoseCluster_const_stringR(cv::ppf_match_3d::PoseCluster3D* instance, const char* FileName, Result<int>* ocvrs_return) {
		try {
			int ret = instance->readPoseCluster(std::string(FileName));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ppf_match_3d::PoseCluster3D::poseList() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:177
	// ("cv::ppf_match_3d::PoseCluster3D::poseList", vec![(pred!(const, [], []), _)]),
	std::vector<cv::ppf_match_3d::Pose3DPtr>* cv_ppf_match_3d_PoseCluster3D_propPoseList_const(const cv::ppf_match_3d::PoseCluster3D* instance) {
			std::vector<cv::ppf_match_3d::Pose3DPtr> ret = instance->poseList;
			return new std::vector<cv::ppf_match_3d::Pose3DPtr>(ret);
	}

	// cv::ppf_match_3d::PoseCluster3D::setPoseList(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:177
	// ("cv::ppf_match_3d::PoseCluster3D::setPoseList", vec![(pred!(mut, ["val"], ["const std::vector<cv::ppf_match_3d::Pose3DPtr>"]), _)]),
	void cv_ppf_match_3d_PoseCluster3D_propPoseList_const_vectorLPose3DPtrG(cv::ppf_match_3d::PoseCluster3D* instance, const std::vector<cv::ppf_match_3d::Pose3DPtr>* val) {
			instance->poseList = *val;
	}

	// cv::ppf_match_3d::PoseCluster3D::numVotes() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:178
	// ("cv::ppf_match_3d::PoseCluster3D::numVotes", vec![(pred!(const, [], []), _)]),
	size_t cv_ppf_match_3d_PoseCluster3D_propNumVotes_const(const cv::ppf_match_3d::PoseCluster3D* instance) {
			size_t ret = instance->numVotes;
			return ret;
	}

	// cv::ppf_match_3d::PoseCluster3D::setNumVotes(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:178
	// ("cv::ppf_match_3d::PoseCluster3D::setNumVotes", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
	void cv_ppf_match_3d_PoseCluster3D_propNumVotes_const_size_t(cv::ppf_match_3d::PoseCluster3D* instance, const size_t val) {
			instance->numVotes = val;
	}

	// cv::ppf_match_3d::PoseCluster3D::id() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:179
	// ("cv::ppf_match_3d::PoseCluster3D::id", vec![(pred!(const, [], []), _)]),
	int cv_ppf_match_3d_PoseCluster3D_propId_const(const cv::ppf_match_3d::PoseCluster3D* instance) {
			int ret = instance->id;
			return ret;
	}

	// cv::ppf_match_3d::PoseCluster3D::setId(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/surface_matching/pose_3d.hpp:179
	// ("cv::ppf_match_3d::PoseCluster3D::setId", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_ppf_match_3d_PoseCluster3D_propId_const_int(cv::ppf_match_3d::PoseCluster3D* instance, const int val) {
			instance->id = val;
	}

	// cv::ppf_match_3d::PoseCluster3D::delete() generated
	// ("cv::ppf_match_3d::PoseCluster3D::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ppf_match_3d_PoseCluster3D_delete(cv::ppf_match_3d::PoseCluster3D* instance) {
			delete instance;
	}

}
