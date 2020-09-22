#include "ocvrs_common.hpp"
#include <opencv2/surface_matching.hpp>
#include "surface_matching_types.hpp"

extern "C" {
	void cv_ICP_delete(cv::ppf_match_3d::ICP* instance) {
		delete instance;
	}
	Result<cv::ppf_match_3d::ICP*> cv_ppf_match_3d_ICP_ICP() {
		try {
			cv::ppf_match_3d::ICP* ret = new cv::ppf_match_3d::ICP();
			return Ok<cv::ppf_match_3d::ICP*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ppf_match_3d::ICP*>))
	}
	
	Result<cv::ppf_match_3d::ICP*> cv_ppf_match_3d_ICP_ICP_const_int_const_float_const_float_const_int_const_ICP_SAMPLING_TYPE_const_int(const int iterations, const float tolerence, const float rejectionScale, const int numLevels, const cv::ppf_match_3d::ICP::ICP_SAMPLING_TYPE sampleType, const int numMaxCorr) {
		try {
			cv::ppf_match_3d::ICP* ret = new cv::ppf_match_3d::ICP(iterations, tolerence, rejectionScale, numLevels, sampleType, numMaxCorr);
			return Ok<cv::ppf_match_3d::ICP*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ppf_match_3d::ICP*>))
	}
	
	Result<int> cv_ppf_match_3d_ICP_registerModelToScene_const_MatR_const_MatR_doubleR_double_X__16_(cv::ppf_match_3d::ICP* instance, const cv::Mat* srcPC, const cv::Mat* dstPC, double* residual, double(*pose)[16]) {
		try {
			int ret = instance->registerModelToScene(*srcPC, *dstPC, *residual, *pose);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_ppf_match_3d_ICP_registerModelToScene_const_MatR_const_MatR_vector_Pose3DPtr_R(cv::ppf_match_3d::ICP* instance, const cv::Mat* srcPC, const cv::Mat* dstPC, std::vector<cv::ppf_match_3d::Pose3DPtr>* poses) {
		try {
			int ret = instance->registerModelToScene(*srcPC, *dstPC, *poses);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	void cv_PPF3DDetector_delete(cv::ppf_match_3d::PPF3DDetector* instance) {
		delete instance;
	}
	Result<cv::ppf_match_3d::PPF3DDetector*> cv_ppf_match_3d_PPF3DDetector_PPF3DDetector() {
		try {
			cv::ppf_match_3d::PPF3DDetector* ret = new cv::ppf_match_3d::PPF3DDetector();
			return Ok<cv::ppf_match_3d::PPF3DDetector*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ppf_match_3d::PPF3DDetector*>))
	}
	
	Result<cv::ppf_match_3d::PPF3DDetector*> cv_ppf_match_3d_PPF3DDetector_PPF3DDetector_const_double_const_double_const_double(const double relativeSamplingStep, const double relativeDistanceStep, const double numAngles) {
		try {
			cv::ppf_match_3d::PPF3DDetector* ret = new cv::ppf_match_3d::PPF3DDetector(relativeSamplingStep, relativeDistanceStep, numAngles);
			return Ok<cv::ppf_match_3d::PPF3DDetector*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ppf_match_3d::PPF3DDetector*>))
	}
	
	Result_void cv_ppf_match_3d_PPF3DDetector_setSearchParams_const_double_const_double_const_bool(cv::ppf_match_3d::PPF3DDetector* instance, const double positionThreshold, const double rotationThreshold, const bool useWeightedClustering) {
		try {
			instance->setSearchParams(positionThreshold, rotationThreshold, useWeightedClustering);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ppf_match_3d_PPF3DDetector_trainModel_const_MatR(cv::ppf_match_3d::PPF3DDetector* instance, const cv::Mat* Model) {
		try {
			instance->trainModel(*Model);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ppf_match_3d_PPF3DDetector_match_const_MatR_vector_Pose3DPtr_R_const_double_const_double(cv::ppf_match_3d::PPF3DDetector* instance, const cv::Mat* scene, std::vector<cv::ppf_match_3d::Pose3DPtr>* results, const double relativeSceneSampleStep, const double relativeSceneDistance) {
		try {
			instance->match(*scene, *results, relativeSceneSampleStep, relativeSceneDistance);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_ppf_match_3d_Pose3D_getPropAlpha_const(const cv::ppf_match_3d::Pose3D* instance) {
		try {
			double ret = instance->alpha;
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_ppf_match_3d_Pose3D_setPropAlpha_double(cv::ppf_match_3d::Pose3D* instance, double val) {
		try {
			instance->alpha = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_ppf_match_3d_Pose3D_getPropResidual_const(const cv::ppf_match_3d::Pose3D* instance) {
		try {
			double ret = instance->residual;
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_ppf_match_3d_Pose3D_setPropResidual_double(cv::ppf_match_3d::Pose3D* instance, double val) {
		try {
			instance->residual = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<unsigned int> cv_ppf_match_3d_Pose3D_getPropModelIndex_const(const cv::ppf_match_3d::Pose3D* instance) {
		try {
			unsigned int ret = instance->modelIndex;
			return Ok<unsigned int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned int>))
	}
	
	Result_void cv_ppf_match_3d_Pose3D_setPropModelIndex_unsigned_int(cv::ppf_match_3d::Pose3D* instance, unsigned int val) {
		try {
			instance->modelIndex = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<unsigned int> cv_ppf_match_3d_Pose3D_getPropNumVotes_const(const cv::ppf_match_3d::Pose3D* instance) {
		try {
			unsigned int ret = instance->numVotes;
			return Ok<unsigned int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned int>))
	}
	
	Result_void cv_ppf_match_3d_Pose3D_setPropNumVotes_unsigned_int(cv::ppf_match_3d::Pose3D* instance, unsigned int val) {
		try {
			instance->numVotes = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double(*)[16]> cv_ppf_match_3d_Pose3D_getPropPose(cv::ppf_match_3d::Pose3D* instance) {
		try {
			double(*ret)[16] = &instance->pose;
			return Ok<double(*)[16]>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double(*)[16]>))
	}
	
	Result<double> cv_ppf_match_3d_Pose3D_getPropAngle_const(const cv::ppf_match_3d::Pose3D* instance) {
		try {
			double ret = instance->angle;
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_ppf_match_3d_Pose3D_setPropAngle_double(cv::ppf_match_3d::Pose3D* instance, double val) {
		try {
			instance->angle = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double(*)[3]> cv_ppf_match_3d_Pose3D_getPropT(cv::ppf_match_3d::Pose3D* instance) {
		try {
			double(*ret)[3] = &instance->t;
			return Ok<double(*)[3]>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double(*)[3]>))
	}
	
	Result<double(*)[4]> cv_ppf_match_3d_Pose3D_getPropQ(cv::ppf_match_3d::Pose3D* instance) {
		try {
			double(*ret)[4] = &instance->q;
			return Ok<double(*)[4]>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double(*)[4]>))
	}
	
	void cv_Pose3D_delete(cv::ppf_match_3d::Pose3D* instance) {
		delete instance;
	}
	Result<cv::ppf_match_3d::Pose3D*> cv_ppf_match_3d_Pose3D_Pose3D() {
		try {
			cv::ppf_match_3d::Pose3D* ret = new cv::ppf_match_3d::Pose3D();
			return Ok<cv::ppf_match_3d::Pose3D*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ppf_match_3d::Pose3D*>))
	}
	
	Result<cv::ppf_match_3d::Pose3D*> cv_ppf_match_3d_Pose3D_Pose3D_double_unsigned_int_unsigned_int(double Alpha, unsigned int ModelIndex, unsigned int NumVotes) {
		try {
			cv::ppf_match_3d::Pose3D* ret = new cv::ppf_match_3d::Pose3D(Alpha, ModelIndex, NumVotes);
			return Ok<cv::ppf_match_3d::Pose3D*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ppf_match_3d::Pose3D*>))
	}
	
	Result_void cv_ppf_match_3d_Pose3D_updatePose_double_X__16_(cv::ppf_match_3d::Pose3D* instance, double(*NewPose)[16]) {
		try {
			instance->updatePose(*NewPose);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ppf_match_3d_Pose3D_updatePose_double_X__9__double_X__3_(cv::ppf_match_3d::Pose3D* instance, double(*NewR)[9], double(*NewT)[3]) {
		try {
			instance->updatePose(*NewR, *NewT);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ppf_match_3d_Pose3D_updatePoseQuat_double_X__4__double_X__3_(cv::ppf_match_3d::Pose3D* instance, double(*Q)[4], double(*NewT)[3]) {
		try {
			instance->updatePoseQuat(*Q, *NewT);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ppf_match_3d_Pose3D_appendPose_double_X__16_(cv::ppf_match_3d::Pose3D* instance, double(*IncrementalPose)[16]) {
		try {
			instance->appendPose(*IncrementalPose);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ppf_match_3d_Pose3D_printPose(cv::ppf_match_3d::Pose3D* instance) {
		try {
			instance->printPose();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::ppf_match_3d::Pose3DPtr*> cv_ppf_match_3d_Pose3D_clone(cv::ppf_match_3d::Pose3D* instance) {
		try {
			cv::ppf_match_3d::Pose3DPtr ret = instance->clone();
			return Ok(new cv::ppf_match_3d::Pose3DPtr(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ppf_match_3d::Pose3DPtr*>))
	}
	
	Result<int> cv_ppf_match_3d_Pose3D_writePose_const_stringR(cv::ppf_match_3d::Pose3D* instance, const char* FileName) {
		try {
			int ret = instance->writePose(std::string(FileName));
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_ppf_match_3d_Pose3D_readPose_const_stringR(cv::ppf_match_3d::Pose3D* instance, const char* FileName) {
		try {
			int ret = instance->readPose(std::string(FileName));
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<std::vector<cv::ppf_match_3d::Pose3DPtr>*> cv_ppf_match_3d_PoseCluster3D_getPropPoseList(cv::ppf_match_3d::PoseCluster3D* instance) {
		try {
			std::vector<cv::ppf_match_3d::Pose3DPtr> ret = instance->poseList;
			return Ok(new std::vector<cv::ppf_match_3d::Pose3DPtr>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::ppf_match_3d::Pose3DPtr>*>))
	}
	
	Result_void cv_ppf_match_3d_PoseCluster3D_setPropPoseList_vector_Pose3DPtr_(cv::ppf_match_3d::PoseCluster3D* instance, std::vector<cv::ppf_match_3d::Pose3DPtr>* val) {
		try {
			instance->poseList = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_ppf_match_3d_PoseCluster3D_getPropNumVotes_const(const cv::ppf_match_3d::PoseCluster3D* instance) {
		try {
			int ret = instance->numVotes;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_ppf_match_3d_PoseCluster3D_setPropNumVotes_int(cv::ppf_match_3d::PoseCluster3D* instance, int val) {
		try {
			instance->numVotes = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_ppf_match_3d_PoseCluster3D_getPropId_const(const cv::ppf_match_3d::PoseCluster3D* instance) {
		try {
			int ret = instance->id;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_ppf_match_3d_PoseCluster3D_setPropId_int(cv::ppf_match_3d::PoseCluster3D* instance, int val) {
		try {
			instance->id = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_PoseCluster3D_delete(cv::ppf_match_3d::PoseCluster3D* instance) {
		delete instance;
	}
	Result<cv::ppf_match_3d::PoseCluster3D*> cv_ppf_match_3d_PoseCluster3D_PoseCluster3D() {
		try {
			cv::ppf_match_3d::PoseCluster3D* ret = new cv::ppf_match_3d::PoseCluster3D();
			return Ok<cv::ppf_match_3d::PoseCluster3D*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ppf_match_3d::PoseCluster3D*>))
	}
	
	Result<cv::ppf_match_3d::PoseCluster3D*> cv_ppf_match_3d_PoseCluster3D_PoseCluster3D_Pose3DPtr(cv::ppf_match_3d::Pose3DPtr* newPose) {
		try {
			cv::ppf_match_3d::PoseCluster3D* ret = new cv::ppf_match_3d::PoseCluster3D(*newPose);
			return Ok<cv::ppf_match_3d::PoseCluster3D*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ppf_match_3d::PoseCluster3D*>))
	}
	
	Result<cv::ppf_match_3d::PoseCluster3D*> cv_ppf_match_3d_PoseCluster3D_PoseCluster3D_Pose3DPtr_int(cv::ppf_match_3d::Pose3DPtr* newPose, int newId) {
		try {
			cv::ppf_match_3d::PoseCluster3D* ret = new cv::ppf_match_3d::PoseCluster3D(*newPose, newId);
			return Ok<cv::ppf_match_3d::PoseCluster3D*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ppf_match_3d::PoseCluster3D*>))
	}
	
	Result_void cv_ppf_match_3d_PoseCluster3D_addPose_Pose3DPtr(cv::ppf_match_3d::PoseCluster3D* instance, cv::ppf_match_3d::Pose3DPtr* newPose) {
		try {
			instance->addPose(*newPose);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_ppf_match_3d_PoseCluster3D_writePoseCluster_const_stringR(cv::ppf_match_3d::PoseCluster3D* instance, const char* FileName) {
		try {
			int ret = instance->writePoseCluster(std::string(FileName));
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_ppf_match_3d_PoseCluster3D_readPoseCluster_const_stringR(cv::ppf_match_3d::PoseCluster3D* instance, const char* FileName) {
		try {
			int ret = instance->readPoseCluster(std::string(FileName));
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
}
