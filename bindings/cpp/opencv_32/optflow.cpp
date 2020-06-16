#include "common.hpp"
#include <opencv2/optflow.hpp>
#include "optflow_types.hpp"

extern "C" {
	Result<double> cv_motempl_calcGlobalOrientation_const__InputArrayR_const__InputArrayR_const__InputArrayR_double_double(const cv::_InputArray* orientation, const cv::_InputArray* mask, const cv::_InputArray* mhi, double timestamp, double duration) {
		try {
			double ret = cv::motempl::calcGlobalOrientation(*orientation, *mask, *mhi, timestamp, duration);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_motempl_calcMotionGradient_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_double_int(const cv::_InputArray* mhi, const cv::_OutputArray* mask, const cv::_OutputArray* orientation, double delta1, double delta2, int apertureSize) {
		try {
			cv::motempl::calcMotionGradient(*mhi, *mask, *orientation, delta1, delta2, apertureSize);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_motempl_segmentMotion_const__InputArrayR_const__OutputArrayR_vector_Rect_R_double_double(const cv::_InputArray* mhi, const cv::_OutputArray* segmask, std::vector<cv::Rect>* boundingRects, double timestamp, double segThresh) {
		try {
			cv::motempl::segmentMotion(*mhi, *segmask, *boundingRects, timestamp, segThresh);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_motempl_updateMotionHistory_const__InputArrayR_const__InputOutputArrayR_double_double(const cv::_InputArray* silhouette, const cv::_InputOutputArray* mhi, double timestamp, double duration) {
		try {
			cv::motempl::updateMotionHistory(*silhouette, *mhi, timestamp, duration);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_optflow_calcOpticalFlowSF_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* from, const cv::_InputArray* to, const cv::_OutputArray* flow, int layers, int averaging_block_size, int max_flow) {
		try {
			cv::optflow::calcOpticalFlowSF(*from, *to, *flow, layers, averaging_block_size, max_flow);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_optflow_calcOpticalFlowSF_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_int_double_double_int_double_double_double_int_double_double_double(const cv::_InputArray* from, const cv::_InputArray* to, const cv::_OutputArray* flow, int layers, int averaging_block_size, int max_flow, double sigma_dist, double sigma_color, int postprocess_window, double sigma_dist_fix, double sigma_color_fix, double occ_thr, int upscale_averaging_radius, double upscale_sigma_dist, double upscale_sigma_color, double speed_up_thr) {
		try {
			cv::optflow::calcOpticalFlowSF(*from, *to, *flow, layers, averaging_block_size, max_flow, sigma_dist, sigma_color, postprocess_window, sigma_dist_fix, sigma_color_fix, occ_thr, upscale_averaging_radius, upscale_sigma_dist, upscale_sigma_color, speed_up_thr);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_optflow_calcOpticalFlowSparseToDense_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_float_bool_float_float(const cv::_InputArray* from, const cv::_InputArray* to, const cv::_OutputArray* flow, int grid_step, int k, float sigma, bool use_post_proc, float fgs_lambda, float fgs_sigma) {
		try {
			cv::optflow::calcOpticalFlowSparseToDense(*from, *to, *flow, grid_step, k, sigma, use_post_proc, fgs_lambda, fgs_sigma);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::optflow::DISOpticalFlow>*> cv_optflow_createOptFlow_DIS_int(int preset) {
		try {
			cv::Ptr<cv::optflow::DISOpticalFlow> ret = cv::optflow::createOptFlow_DIS(preset);
			return Ok(new cv::Ptr<cv::optflow::DISOpticalFlow>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::optflow::DISOpticalFlow>*>))
	}
	
	Result<cv::Ptr<cv::DenseOpticalFlow>*> cv_optflow_createOptFlow_DeepFlow() {
		try {
			cv::Ptr<cv::DenseOpticalFlow> ret = cv::optflow::createOptFlow_DeepFlow();
			return Ok(new cv::Ptr<cv::DenseOpticalFlow>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::DenseOpticalFlow>*>))
	}
	
	Result<cv::Ptr<cv::DenseOpticalFlow>*> cv_optflow_createOptFlow_Farneback() {
		try {
			cv::Ptr<cv::DenseOpticalFlow> ret = cv::optflow::createOptFlow_Farneback();
			return Ok(new cv::Ptr<cv::DenseOpticalFlow>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::DenseOpticalFlow>*>))
	}
	
	Result<cv::Ptr<cv::DenseOpticalFlow>*> cv_optflow_createOptFlow_PCAFlow() {
		try {
			cv::Ptr<cv::DenseOpticalFlow> ret = cv::optflow::createOptFlow_PCAFlow();
			return Ok(new cv::Ptr<cv::DenseOpticalFlow>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::DenseOpticalFlow>*>))
	}
	
	Result<cv::Ptr<cv::DenseOpticalFlow>*> cv_optflow_createOptFlow_SimpleFlow() {
		try {
			cv::Ptr<cv::DenseOpticalFlow> ret = cv::optflow::createOptFlow_SimpleFlow();
			return Ok(new cv::Ptr<cv::DenseOpticalFlow>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::DenseOpticalFlow>*>))
	}
	
	Result<cv::Ptr<cv::DenseOpticalFlow>*> cv_optflow_createOptFlow_SparseToDense() {
		try {
			cv::Ptr<cv::DenseOpticalFlow> ret = cv::optflow::createOptFlow_SparseToDense();
			return Ok(new cv::Ptr<cv::DenseOpticalFlow>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::DenseOpticalFlow>*>))
	}
	
	Result<cv::Ptr<cv::optflow::VariationalRefinement>*> cv_optflow_createVariationalFlowRefinement() {
		try {
			cv::Ptr<cv::optflow::VariationalRefinement> ret = cv::optflow::createVariationalFlowRefinement();
			return Ok(new cv::Ptr<cv::optflow::VariationalRefinement>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::optflow::VariationalRefinement>*>))
	}
	
	Result<cv::Mat*> cv_optflow_readOpticalFlow_const_StringR(const char* path) {
		try {
			cv::Mat ret = cv::optflow::readOpticalFlow(cv::String(path));
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<bool> cv_optflow_writeOpticalFlow_const_StringR_const__InputArrayR(const char* path, const cv::_InputArray* flow) {
		try {
			bool ret = cv::optflow::writeOpticalFlow(cv::String(path), *flow);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_read_const_FileNodeR_NodeR_Node(const cv::FileNode* fn, cv::optflow::GPCTree::Node* node, const cv::optflow::GPCTree::Node* unnamed) {
		try {
			cv::read(*fn, *node, *unnamed);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_write_FileStorageR_const_StringR_const_NodeR(cv::FileStorage* fs, const char* name, const cv::optflow::GPCTree::Node* node) {
		try {
			cv::write(*fs, cv::String(name), *node);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_optflow_DISOpticalFlow_getFinestScale_const(const cv::optflow::DISOpticalFlow* instance) {
		try {
			int ret = instance->getFinestScale();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_optflow_DISOpticalFlow_setFinestScale_int(cv::optflow::DISOpticalFlow* instance, int val) {
		try {
			instance->setFinestScale(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_optflow_DISOpticalFlow_getPatchSize_const(const cv::optflow::DISOpticalFlow* instance) {
		try {
			int ret = instance->getPatchSize();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_optflow_DISOpticalFlow_setPatchSize_int(cv::optflow::DISOpticalFlow* instance, int val) {
		try {
			instance->setPatchSize(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_optflow_DISOpticalFlow_getPatchStride_const(const cv::optflow::DISOpticalFlow* instance) {
		try {
			int ret = instance->getPatchStride();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_optflow_DISOpticalFlow_setPatchStride_int(cv::optflow::DISOpticalFlow* instance, int val) {
		try {
			instance->setPatchStride(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_optflow_DISOpticalFlow_getGradientDescentIterations_const(const cv::optflow::DISOpticalFlow* instance) {
		try {
			int ret = instance->getGradientDescentIterations();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_optflow_DISOpticalFlow_setGradientDescentIterations_int(cv::optflow::DISOpticalFlow* instance, int val) {
		try {
			instance->setGradientDescentIterations(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_optflow_DISOpticalFlow_getVariationalRefinementIterations_const(const cv::optflow::DISOpticalFlow* instance) {
		try {
			int ret = instance->getVariationalRefinementIterations();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_optflow_DISOpticalFlow_setVariationalRefinementIterations_int(cv::optflow::DISOpticalFlow* instance, int val) {
		try {
			instance->setVariationalRefinementIterations(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_optflow_DISOpticalFlow_getVariationalRefinementAlpha_const(const cv::optflow::DISOpticalFlow* instance) {
		try {
			float ret = instance->getVariationalRefinementAlpha();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_optflow_DISOpticalFlow_setVariationalRefinementAlpha_float(cv::optflow::DISOpticalFlow* instance, float val) {
		try {
			instance->setVariationalRefinementAlpha(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_optflow_DISOpticalFlow_getVariationalRefinementDelta_const(const cv::optflow::DISOpticalFlow* instance) {
		try {
			float ret = instance->getVariationalRefinementDelta();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_optflow_DISOpticalFlow_setVariationalRefinementDelta_float(cv::optflow::DISOpticalFlow* instance, float val) {
		try {
			instance->setVariationalRefinementDelta(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_optflow_DISOpticalFlow_getVariationalRefinementGamma_const(const cv::optflow::DISOpticalFlow* instance) {
		try {
			float ret = instance->getVariationalRefinementGamma();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_optflow_DISOpticalFlow_setVariationalRefinementGamma_float(cv::optflow::DISOpticalFlow* instance, float val) {
		try {
			instance->setVariationalRefinementGamma(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_optflow_DISOpticalFlow_getUseMeanNormalization_const(const cv::optflow::DISOpticalFlow* instance) {
		try {
			bool ret = instance->getUseMeanNormalization();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_optflow_DISOpticalFlow_setUseMeanNormalization_bool(cv::optflow::DISOpticalFlow* instance, bool val) {
		try {
			instance->setUseMeanNormalization(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_optflow_DISOpticalFlow_getUseSpatialPropagation_const(const cv::optflow::DISOpticalFlow* instance) {
		try {
			bool ret = instance->getUseSpatialPropagation();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_optflow_DISOpticalFlow_setUseSpatialPropagation_bool(cv::optflow::DISOpticalFlow* instance, bool val) {
		try {
			instance->setUseSpatialPropagation(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_GPCDetails_delete(cv::optflow::GPCDetails* instance) {
		delete instance;
	}
	Result_void cv_optflow_GPCDetails_getAllDescriptorsForImage_const_MatX_vector_GPCPatchDescriptor_R_const_GPCMatchingParamsR_int(const cv::Mat* imgCh, std::vector<cv::optflow::GPCPatchDescriptor>* descr, const cv::optflow::GPCMatchingParams* mp, int type) {
		try {
			cv::optflow::GPCDetails::getAllDescriptorsForImage(imgCh, *descr, *mp, type);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_optflow_GPCDetails_getCoordinatesFromIndex_size_t_Size_intR_intR(size_t index, const cv::Size* sz, int* x, int* y) {
		try {
			cv::optflow::GPCDetails::getCoordinatesFromIndex(index, *sz, *x, *y);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::optflow::GPCMatchingParams> cv_optflow_GPCMatchingParams_GPCMatchingParams_bool(bool _useOpenCL) {
		try {
			cv::optflow::GPCMatchingParams ret(_useOpenCL);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::GPCMatchingParams>))
	}
	
	Result<cv::optflow::GPCMatchingParams> cv_optflow_GPCMatchingParams_GPCMatchingParams_const_GPCMatchingParamsR(const cv::optflow::GPCMatchingParams* params) {
		try {
			cv::optflow::GPCMatchingParams ret(*params);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::GPCMatchingParams>))
	}
	
	Result<cv::Vec<double, 18>> cv_optflow_GPCPatchDescriptor_getPropFeature_const(const cv::optflow::GPCPatchDescriptor* instance) {
		try {
			cv::Vec<double, 18> ret = instance->feature;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec<double, 18>>))
	}
	
	Result_void cv_optflow_GPCPatchDescriptor_setPropFeature_Vec_double__18_(cv::optflow::GPCPatchDescriptor* instance, const cv::Vec<double, 18>* val) {
		try {
			instance->feature = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_GPCPatchDescriptor_delete(cv::optflow::GPCPatchDescriptor* instance) {
		delete instance;
	}
	Result<double> cv_optflow_GPCPatchDescriptor_dot_const_const_Vec_double__18_R(const cv::optflow::GPCPatchDescriptor* instance, const cv::Vec<double, 18>* coef) {
		try {
			double ret = instance->dot(*coef);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_optflow_GPCPatchDescriptor_markAsSeparated(cv::optflow::GPCPatchDescriptor* instance) {
		try {
			instance->markAsSeparated();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_optflow_GPCPatchDescriptor_isSeparated_const(const cv::optflow::GPCPatchDescriptor* instance) {
		try {
			bool ret = instance->isSeparated();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<cv::optflow::GPCPatchDescriptor*> cv_optflow_GPCPatchSample_getPropRef(cv::optflow::GPCPatchSample* instance) {
		try {
			cv::optflow::GPCPatchDescriptor ret = instance->ref;
			return Ok(new cv::optflow::GPCPatchDescriptor(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::GPCPatchDescriptor*>))
	}
	
	Result_void cv_optflow_GPCPatchSample_setPropRef_GPCPatchDescriptor(cv::optflow::GPCPatchSample* instance, cv::optflow::GPCPatchDescriptor* val) {
		try {
			instance->ref = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::optflow::GPCPatchDescriptor*> cv_optflow_GPCPatchSample_getPropPos(cv::optflow::GPCPatchSample* instance) {
		try {
			cv::optflow::GPCPatchDescriptor ret = instance->pos;
			return Ok(new cv::optflow::GPCPatchDescriptor(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::GPCPatchDescriptor*>))
	}
	
	Result_void cv_optflow_GPCPatchSample_setPropPos_GPCPatchDescriptor(cv::optflow::GPCPatchSample* instance, cv::optflow::GPCPatchDescriptor* val) {
		try {
			instance->pos = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::optflow::GPCPatchDescriptor*> cv_optflow_GPCPatchSample_getPropNeg(cv::optflow::GPCPatchSample* instance) {
		try {
			cv::optflow::GPCPatchDescriptor ret = instance->neg;
			return Ok(new cv::optflow::GPCPatchDescriptor(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::GPCPatchDescriptor*>))
	}
	
	Result_void cv_optflow_GPCPatchSample_setPropNeg_GPCPatchDescriptor(cv::optflow::GPCPatchSample* instance, cv::optflow::GPCPatchDescriptor* val) {
		try {
			instance->neg = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_GPCPatchSample_delete(cv::optflow::GPCPatchSample* instance) {
		delete instance;
	}
	Result_void cv_optflow_GPCPatchSample_getDirections_const_boolR_boolR_boolR_const_Vec_double__18_R_double(const cv::optflow::GPCPatchSample* instance, bool* refdir, bool* posdir, bool* negdir, const cv::Vec<double, 18>* coef, double rhs) {
		try {
			instance->getDirections(*refdir, *posdir, *negdir, *coef, rhs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::optflow::GPCTrainingParams> cv_optflow_GPCTrainingParams_GPCTrainingParams_unsigned_int_int_GPCDescType_bool(unsigned int _maxTreeDepth, int _minNumberOfSamples, cv::optflow::GPCDescType _descriptorType, bool _printProgress) {
		try {
			cv::optflow::GPCTrainingParams ret(_maxTreeDepth, _minNumberOfSamples, _descriptorType, _printProgress);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::GPCTrainingParams>))
	}
	
	Result<cv::optflow::GPCTrainingParams> cv_optflow_GPCTrainingParams_GPCTrainingParams_const_GPCTrainingParamsR(const cv::optflow::GPCTrainingParams* params) {
		try {
			cv::optflow::GPCTrainingParams ret(*params);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::GPCTrainingParams>))
	}
	
	Result<bool> cv_optflow_GPCTrainingParams_check_const(const cv::optflow::GPCTrainingParams instance) {
		try {
			bool ret = instance.check();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	void cv_GPCTrainingSamples_delete(cv::optflow::GPCTrainingSamples* instance) {
		delete instance;
	}
	Result<cv::Ptr<cv::optflow::GPCTrainingSamples>*> cv_optflow_GPCTrainingSamples_create_const_vector_String_R_const_vector_String_R_const_vector_String_R_int(const std::vector<cv::String>* imagesFrom, const std::vector<cv::String>* imagesTo, const std::vector<cv::String>* gt, int descriptorType) {
		try {
			cv::Ptr<cv::optflow::GPCTrainingSamples> ret = cv::optflow::GPCTrainingSamples::create(*imagesFrom, *imagesTo, *gt, descriptorType);
			return Ok(new cv::Ptr<cv::optflow::GPCTrainingSamples>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::optflow::GPCTrainingSamples>*>))
	}
	
	Result<cv::Ptr<cv::optflow::GPCTrainingSamples>*> cv_optflow_GPCTrainingSamples_create_const__InputArrayR_const__InputArrayR_const__InputArrayR_int(const cv::_InputArray* imagesFrom, const cv::_InputArray* imagesTo, const cv::_InputArray* gt, int descriptorType) {
		try {
			cv::Ptr<cv::optflow::GPCTrainingSamples> ret = cv::optflow::GPCTrainingSamples::create(*imagesFrom, *imagesTo, *gt, descriptorType);
			return Ok(new cv::Ptr<cv::optflow::GPCTrainingSamples>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::optflow::GPCTrainingSamples>*>))
	}
	
	Result<size_t> cv_optflow_GPCTrainingSamples_size_const(const cv::optflow::GPCTrainingSamples* instance) {
		try {
			size_t ret = instance->size();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<int> cv_optflow_GPCTrainingSamples_type_const(const cv::optflow::GPCTrainingSamples* instance) {
		try {
			int ret = instance->type();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	void cv_GPCTree_delete(cv::optflow::GPCTree* instance) {
		delete instance;
	}
	Result_void cv_optflow_GPCTree_train_GPCTrainingSamplesR_GPCTrainingParams(cv::optflow::GPCTree* instance, cv::optflow::GPCTrainingSamples* samples, const cv::optflow::GPCTrainingParams* params) {
		try {
			instance->train(*samples, *params);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_optflow_GPCTree_write_const_FileStorageR(const cv::optflow::GPCTree* instance, cv::FileStorage* fs) {
		try {
			instance->write(*fs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_optflow_GPCTree_read_const_FileNodeR(cv::optflow::GPCTree* instance, const cv::FileNode* fn) {
		try {
			instance->read(*fn);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<unsigned int> cv_optflow_GPCTree_findLeafForPatch_const_const_GPCPatchDescriptorR(const cv::optflow::GPCTree* instance, const cv::optflow::GPCPatchDescriptor* descr) {
		try {
			unsigned int ret = instance->findLeafForPatch(*descr);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned int>))
	}
	
	Result<cv::Ptr<cv::optflow::GPCTree>*> cv_optflow_GPCTree_create() {
		try {
			cv::Ptr<cv::optflow::GPCTree> ret = cv::optflow::GPCTree::create();
			return Ok(new cv::Ptr<cv::optflow::GPCTree>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::optflow::GPCTree>*>))
	}
	
	Result<int> cv_optflow_GPCTree_getDescriptorType_const(const cv::optflow::GPCTree* instance) {
		try {
			int ret = instance->getDescriptorType();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	void cv_OpticalFlowPCAFlow_delete(cv::optflow::OpticalFlowPCAFlow* instance) {
		delete instance;
	}
	Result<cv::optflow::OpticalFlowPCAFlow*> cv_optflow_OpticalFlowPCAFlow_OpticalFlowPCAFlow_Ptr_PCAPrior__Size_float_float_float_float_float(const cv::Ptr<cv::optflow::PCAPrior>* _prior, const cv::Size* _basisSize, float _sparseRate, float _retainedCornersFraction, float _occlusionsThreshold, float _dampingFactor, float _claheClip) {
		try {
			cv::optflow::OpticalFlowPCAFlow* ret = new cv::optflow::OpticalFlowPCAFlow(*_prior, *_basisSize, _sparseRate, _retainedCornersFraction, _occlusionsThreshold, _dampingFactor, _claheClip);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::OpticalFlowPCAFlow*>))
	}
	
	Result_void cv_optflow_OpticalFlowPCAFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(cv::optflow::OpticalFlowPCAFlow* instance, const cv::_InputArray* I0, const cv::_InputArray* I1, const cv::_InputOutputArray* flow) {
		try {
			instance->calc(*I0, *I1, *flow);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_optflow_OpticalFlowPCAFlow_collectGarbage(cv::optflow::OpticalFlowPCAFlow* instance) {
		try {
			instance->collectGarbage();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_PCAPrior_delete(cv::optflow::PCAPrior* instance) {
		delete instance;
	}
	Result<cv::optflow::PCAPrior*> cv_optflow_PCAPrior_PCAPrior_const_charX(const char* pathToPrior) {
		try {
			cv::optflow::PCAPrior* ret = new cv::optflow::PCAPrior(pathToPrior);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::PCAPrior*>))
	}
	
	Result<int> cv_optflow_PCAPrior_getPadding_const(const cv::optflow::PCAPrior* instance) {
		try {
			int ret = instance->getPadding();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_optflow_PCAPrior_getBasisSize_const(const cv::optflow::PCAPrior* instance) {
		try {
			int ret = instance->getBasisSize();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_optflow_PCAPrior_fillConstraints_const_floatX_floatX_floatX_floatX(const cv::optflow::PCAPrior* instance, float* A1, float* A2, float* b1, float* b2) {
		try {
			instance->fillConstraints(A1, A2, b1, b2);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_optflow_VariationalRefinement_calcUV_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(cv::optflow::VariationalRefinement* instance, const cv::_InputArray* I0, const cv::_InputArray* I1, const cv::_InputOutputArray* flow_u, const cv::_InputOutputArray* flow_v) {
		try {
			instance->calcUV(*I0, *I1, *flow_u, *flow_v);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_optflow_VariationalRefinement_getFixedPointIterations_const(const cv::optflow::VariationalRefinement* instance) {
		try {
			int ret = instance->getFixedPointIterations();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_optflow_VariationalRefinement_setFixedPointIterations_int(cv::optflow::VariationalRefinement* instance, int val) {
		try {
			instance->setFixedPointIterations(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_optflow_VariationalRefinement_getSorIterations_const(const cv::optflow::VariationalRefinement* instance) {
		try {
			int ret = instance->getSorIterations();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_optflow_VariationalRefinement_setSorIterations_int(cv::optflow::VariationalRefinement* instance, int val) {
		try {
			instance->setSorIterations(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_optflow_VariationalRefinement_getOmega_const(const cv::optflow::VariationalRefinement* instance) {
		try {
			float ret = instance->getOmega();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_optflow_VariationalRefinement_setOmega_float(cv::optflow::VariationalRefinement* instance, float val) {
		try {
			instance->setOmega(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_optflow_VariationalRefinement_getAlpha_const(const cv::optflow::VariationalRefinement* instance) {
		try {
			float ret = instance->getAlpha();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_optflow_VariationalRefinement_setAlpha_float(cv::optflow::VariationalRefinement* instance, float val) {
		try {
			instance->setAlpha(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_optflow_VariationalRefinement_getDelta_const(const cv::optflow::VariationalRefinement* instance) {
		try {
			float ret = instance->getDelta();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_optflow_VariationalRefinement_setDelta_float(cv::optflow::VariationalRefinement* instance, float val) {
		try {
			instance->setDelta(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_optflow_VariationalRefinement_getGamma_const(const cv::optflow::VariationalRefinement* instance) {
		try {
			float ret = instance->getGamma();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_optflow_VariationalRefinement_setGamma_float(cv::optflow::VariationalRefinement* instance, float val) {
		try {
			instance->setGamma(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
