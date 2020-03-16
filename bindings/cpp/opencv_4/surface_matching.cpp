#include "common.hpp"
#include <opencv2/surface_matching.hpp>
#include "surface_matching_types.hpp"

extern "C" {
	void cv_ICP_delete(cv::ppf_match_3d::ICP* instance) {
		delete instance;
	}
	Result<cv::ppf_match_3d::ICP*> cv_ppf_match_3d_ICP_ICP() {
		try {
			cv::ppf_match_3d::ICP* ret = new cv::ppf_match_3d::ICP();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::ppf_match_3d::ICP*>)
	}
	
	Result<cv::ppf_match_3d::ICP*> cv_ppf_match_3d_ICP_ICP_int_float_float_int_int_int(int iterations, float tolerence, float rejectionScale, int numLevels, int sampleType, int numMaxCorr) {
		try {
			cv::ppf_match_3d::ICP* ret = new cv::ppf_match_3d::ICP(iterations, tolerence, rejectionScale, numLevels, sampleType, numMaxCorr);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::ppf_match_3d::ICP*>)
	}
	
	Result<int> cv_ppf_match_3d_ICP_registerModelToScene_const_MatX_const_MatX_doubleX_Matx44dX(cv::ppf_match_3d::ICP* instance, const cv::Mat* srcPC, const cv::Mat* dstPC, double* residual, cv::Matx44d* pose) {
		try {
			int ret = instance->registerModelToScene(*srcPC, *dstPC, *residual, *pose);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ppf_match_3d_ICP_registerModelToScene_const_MatX_const_MatX_vector_Pose3DPtr_X(cv::ppf_match_3d::ICP* instance, const cv::Mat* srcPC, const cv::Mat* dstPC, std::vector<cv::ppf_match_3d::Pose3DPtr>* poses) {
		try {
			int ret = instance->registerModelToScene(*srcPC, *dstPC, *poses);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	void cv_PPF3DDetector_delete(cv::ppf_match_3d::PPF3DDetector* instance) {
		delete instance;
	}
	Result<cv::ppf_match_3d::PPF3DDetector*> cv_ppf_match_3d_PPF3DDetector_PPF3DDetector() {
		try {
			cv::ppf_match_3d::PPF3DDetector* ret = new cv::ppf_match_3d::PPF3DDetector();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::ppf_match_3d::PPF3DDetector*>)
	}
	
	Result<cv::ppf_match_3d::PPF3DDetector*> cv_ppf_match_3d_PPF3DDetector_PPF3DDetector_double_double_double(double relativeSamplingStep, double relativeDistanceStep, double numAngles) {
		try {
			cv::ppf_match_3d::PPF3DDetector* ret = new cv::ppf_match_3d::PPF3DDetector(relativeSamplingStep, relativeDistanceStep, numAngles);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::ppf_match_3d::PPF3DDetector*>)
	}
	
	Result_void cv_ppf_match_3d_PPF3DDetector_setSearchParams_double_double_bool(cv::ppf_match_3d::PPF3DDetector* instance, double positionThreshold, double rotationThreshold, bool useWeightedClustering) {
		try {
			instance->setSearchParams(positionThreshold, rotationThreshold, useWeightedClustering);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ppf_match_3d_PPF3DDetector_trainModel_const_MatX(cv::ppf_match_3d::PPF3DDetector* instance, const cv::Mat* Model) {
		try {
			instance->trainModel(*Model);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ppf_match_3d_PPF3DDetector_match_const_MatX_vector_Pose3DPtr_X_double_double(cv::ppf_match_3d::PPF3DDetector* instance, const cv::Mat* scene, std::vector<cv::ppf_match_3d::Pose3DPtr>* results, double relativeSceneSampleStep, double relativeSceneDistance) {
		try {
			instance->match(*scene, *results, relativeSceneSampleStep, relativeSceneDistance);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ppf_match_3d_PPF3DDetector_read_const_FileNodeX(cv::ppf_match_3d::PPF3DDetector* instance, const cv::FileNode* fn) {
		try {
			instance->read(*fn);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ppf_match_3d_PPF3DDetector_write_const_FileStorageX(const cv::ppf_match_3d::PPF3DDetector* instance, cv::FileStorage* fs) {
		try {
			instance->write(*fs);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ppf_match_3d_Pose3D_alpha_const(const cv::ppf_match_3d::Pose3D* instance) {
		try {
			double ret = instance->alpha;
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ppf_match_3d_Pose3D_setAlpha_double(cv::ppf_match_3d::Pose3D* instance, double val) {
		try {
			instance->alpha = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ppf_match_3d_Pose3D_residual_const(const cv::ppf_match_3d::Pose3D* instance) {
		try {
			double ret = instance->residual;
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ppf_match_3d_Pose3D_setResidual_double(cv::ppf_match_3d::Pose3D* instance, double val) {
		try {
			instance->residual = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<size_t> cv_ppf_match_3d_Pose3D_modelIndex_const(const cv::ppf_match_3d::Pose3D* instance) {
		try {
			size_t ret = instance->modelIndex;
			return Ok(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result_void cv_ppf_match_3d_Pose3D_setModelIndex_size_t(cv::ppf_match_3d::Pose3D* instance, size_t val) {
		try {
			instance->modelIndex = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<size_t> cv_ppf_match_3d_Pose3D_numVotes_const(const cv::ppf_match_3d::Pose3D* instance) {
		try {
			size_t ret = instance->numVotes;
			return Ok(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result_void cv_ppf_match_3d_Pose3D_setNumVotes_size_t(cv::ppf_match_3d::Pose3D* instance, size_t val) {
		try {
			instance->numVotes = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Matx44d> cv_ppf_match_3d_Pose3D_pose_const(const cv::ppf_match_3d::Pose3D* instance) {
		try {
			cv::Matx44d ret = instance->pose;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Matx44d>)
	}
	
	Result_void cv_ppf_match_3d_Pose3D_setPose_Matx44d(cv::ppf_match_3d::Pose3D* instance, const cv::Matx44d* val) {
		try {
			instance->pose = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ppf_match_3d_Pose3D_angle_const(const cv::ppf_match_3d::Pose3D* instance) {
		try {
			double ret = instance->angle;
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ppf_match_3d_Pose3D_setAngle_double(cv::ppf_match_3d::Pose3D* instance, double val) {
		try {
			instance->angle = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Vec3d> cv_ppf_match_3d_Pose3D_t_const(const cv::ppf_match_3d::Pose3D* instance) {
		try {
			cv::Vec3d ret = instance->t;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Vec3d>)
	}
	
	Result_void cv_ppf_match_3d_Pose3D_setT_Vec3d(cv::ppf_match_3d::Pose3D* instance, const cv::Vec3d* val) {
		try {
			instance->t = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Vec4d> cv_ppf_match_3d_Pose3D_q_const(const cv::ppf_match_3d::Pose3D* instance) {
		try {
			cv::Vec4d ret = instance->q;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Vec4d>)
	}
	
	Result_void cv_ppf_match_3d_Pose3D_setQ_Vec4d(cv::ppf_match_3d::Pose3D* instance, const cv::Vec4d* val) {
		try {
			instance->q = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_Pose3D_delete(cv::ppf_match_3d::Pose3D* instance) {
		delete instance;
	}
	Result<cv::ppf_match_3d::Pose3D*> cv_ppf_match_3d_Pose3D_Pose3D() {
		try {
			cv::ppf_match_3d::Pose3D* ret = new cv::ppf_match_3d::Pose3D();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::ppf_match_3d::Pose3D*>)
	}
	
	Result<cv::ppf_match_3d::Pose3D*> cv_ppf_match_3d_Pose3D_Pose3D_double_size_t_size_t(double Alpha, size_t ModelIndex, size_t NumVotes) {
		try {
			cv::ppf_match_3d::Pose3D* ret = new cv::ppf_match_3d::Pose3D(Alpha, ModelIndex, NumVotes);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::ppf_match_3d::Pose3D*>)
	}
	
	Result_void cv_ppf_match_3d_Pose3D_updatePose_Matx44dX(cv::ppf_match_3d::Pose3D* instance, cv::Matx44d* NewPose) {
		try {
			instance->updatePose(*NewPose);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ppf_match_3d_Pose3D_updatePose_Matx33dX_Vec3dX(cv::ppf_match_3d::Pose3D* instance, cv::Matx33d* NewR, cv::Vec3d* NewT) {
		try {
			instance->updatePose(*NewR, *NewT);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ppf_match_3d_Pose3D_updatePoseQuat_Vec4dX_Vec3dX(cv::ppf_match_3d::Pose3D* instance, cv::Vec4d* Q, cv::Vec3d* NewT) {
		try {
			instance->updatePoseQuat(*Q, *NewT);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ppf_match_3d_Pose3D_appendPose_Matx44dX(cv::ppf_match_3d::Pose3D* instance, cv::Matx44d* IncrementalPose) {
		try {
			instance->appendPose(*IncrementalPose);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ppf_match_3d_Pose3D_printPose(cv::ppf_match_3d::Pose3D* instance) {
		try {
			instance->printPose();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::ppf_match_3d::Pose3DPtr*> cv_ppf_match_3d_Pose3D_clone(cv::ppf_match_3d::Pose3D* instance) {
		try {
			cv::ppf_match_3d::Pose3DPtr ret = instance->clone();
			return Ok(new cv::ppf_match_3d::Pose3DPtr(ret));
		} OCVRS_CATCH(Result<cv::ppf_match_3d::Pose3DPtr*>)
	}
	
	Result<int> cv_ppf_match_3d_Pose3D_writePose_const_stringX(cv::ppf_match_3d::Pose3D* instance, const char* FileName) {
		try {
			int ret = instance->writePose(std::string(FileName));
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ppf_match_3d_Pose3D_readPose_const_stringX(cv::ppf_match_3d::Pose3D* instance, const char* FileName) {
		try {
			int ret = instance->readPose(std::string(FileName));
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<std::vector<cv::ppf_match_3d::Pose3DPtr>*> cv_ppf_match_3d_PoseCluster3D_poseList(cv::ppf_match_3d::PoseCluster3D* instance) {
		try {
			std::vector<cv::ppf_match_3d::Pose3DPtr> ret = instance->poseList;
			return Ok(new std::vector<cv::ppf_match_3d::Pose3DPtr>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::ppf_match_3d::Pose3DPtr>*>)
	}
	
	Result_void cv_ppf_match_3d_PoseCluster3D_setPoseList_vector_Pose3DPtr_(cv::ppf_match_3d::PoseCluster3D* instance, std::vector<cv::ppf_match_3d::Pose3DPtr>* val) {
		try {
			instance->poseList = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<size_t> cv_ppf_match_3d_PoseCluster3D_numVotes_const(const cv::ppf_match_3d::PoseCluster3D* instance) {
		try {
			size_t ret = instance->numVotes;
			return Ok(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result_void cv_ppf_match_3d_PoseCluster3D_setNumVotes_size_t(cv::ppf_match_3d::PoseCluster3D* instance, size_t val) {
		try {
			instance->numVotes = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ppf_match_3d_PoseCluster3D_id_const(const cv::ppf_match_3d::PoseCluster3D* instance) {
		try {
			int ret = instance->id;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ppf_match_3d_PoseCluster3D_setId_int(cv::ppf_match_3d::PoseCluster3D* instance, int val) {
		try {
			instance->id = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_PoseCluster3D_delete(cv::ppf_match_3d::PoseCluster3D* instance) {
		delete instance;
	}
	Result<cv::ppf_match_3d::PoseCluster3D*> cv_ppf_match_3d_PoseCluster3D_PoseCluster3D() {
		try {
			cv::ppf_match_3d::PoseCluster3D* ret = new cv::ppf_match_3d::PoseCluster3D();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::ppf_match_3d::PoseCluster3D*>)
	}
	
	Result<cv::ppf_match_3d::PoseCluster3D*> cv_ppf_match_3d_PoseCluster3D_PoseCluster3D_Pose3DPtr(cv::ppf_match_3d::Pose3DPtr* newPose) {
		try {
			cv::ppf_match_3d::PoseCluster3D* ret = new cv::ppf_match_3d::PoseCluster3D(*newPose);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::ppf_match_3d::PoseCluster3D*>)
	}
	
	Result<cv::ppf_match_3d::PoseCluster3D*> cv_ppf_match_3d_PoseCluster3D_PoseCluster3D_Pose3DPtr_int(cv::ppf_match_3d::Pose3DPtr* newPose, int newId) {
		try {
			cv::ppf_match_3d::PoseCluster3D* ret = new cv::ppf_match_3d::PoseCluster3D(*newPose, newId);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::ppf_match_3d::PoseCluster3D*>)
	}
	
	Result_void cv_ppf_match_3d_PoseCluster3D_addPose_Pose3DPtr(cv::ppf_match_3d::PoseCluster3D* instance, cv::ppf_match_3d::Pose3DPtr* newPose) {
		try {
			instance->addPose(*newPose);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ppf_match_3d_PoseCluster3D_writePoseCluster_const_stringX(cv::ppf_match_3d::PoseCluster3D* instance, const char* FileName) {
		try {
			int ret = instance->writePoseCluster(std::string(FileName));
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ppf_match_3d_PoseCluster3D_readPoseCluster_const_stringX(cv::ppf_match_3d::PoseCluster3D* instance, const char* FileName) {
		try {
			int ret = instance->readPoseCluster(std::string(FileName));
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
}
