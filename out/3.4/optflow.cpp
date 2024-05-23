#include "ocvrs_common.hpp"
#include <opencv2/optflow.hpp>
#include "optflow_types.hpp"

extern "C" {
	// calcGlobalOrientation(InputArray, InputArray, InputArray, double, double)(InputArray, InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/motempl.hpp:119
	// ("cv::motempl::calcGlobalOrientation", vec![(pred!(mut, ["orientation", "mask", "mhi", "timestamp", "duration"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "double", "double"]), _)]),
	void cv_motempl_calcGlobalOrientation_const__InputArrayR_const__InputArrayR_const__InputArrayR_double_double(const cv::_InputArray* orientation, const cv::_InputArray* mask, const cv::_InputArray* mhi, double timestamp, double duration, Result<double>* ocvrs_return) {
		try {
			double ret = cv::motempl::calcGlobalOrientation(*orientation, *mask, *mhi, timestamp, duration);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::motempl::calcMotionGradient(InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/motempl.hpp:102
	// ("cv::motempl::calcMotionGradient", vec![(pred!(mut, ["mhi", "mask", "orientation", "delta1", "delta2"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
	void cv_motempl_calcMotionGradient_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_double(const cv::_InputArray* mhi, const cv::_OutputArray* mask, const cv::_OutputArray* orientation, double delta1, double delta2, ResultVoid* ocvrs_return) {
		try {
			cv::motempl::calcMotionGradient(*mhi, *mask, *orientation, delta1, delta2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calcMotionGradient(InputArray, OutputArray, OutputArray, double, double, int)(InputArray, OutputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/motempl.hpp:102
	// ("cv::motempl::calcMotionGradient", vec![(pred!(mut, ["mhi", "mask", "orientation", "delta1", "delta2", "apertureSize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "double", "int"]), _)]),
	void cv_motempl_calcMotionGradient_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_double_int(const cv::_InputArray* mhi, const cv::_OutputArray* mask, const cv::_OutputArray* orientation, double delta1, double delta2, int apertureSize, ResultVoid* ocvrs_return) {
		try {
			cv::motempl::calcMotionGradient(*mhi, *mask, *orientation, delta1, delta2, apertureSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// segmentMotion(InputArray, OutputArray, std::vector<Rect> &, double, double)(InputArray, OutputArray, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/motempl.hpp:137
	// ("cv::motempl::segmentMotion", vec![(pred!(mut, ["mhi", "segmask", "boundingRects", "timestamp", "segThresh"], ["const cv::_InputArray*", "const cv::_OutputArray*", "std::vector<cv::Rect>*", "double", "double"]), _)]),
	void cv_motempl_segmentMotion_const__InputArrayR_const__OutputArrayR_vectorLRectGR_double_double(const cv::_InputArray* mhi, const cv::_OutputArray* segmask, std::vector<cv::Rect>* boundingRects, double timestamp, double segThresh, ResultVoid* ocvrs_return) {
		try {
			cv::motempl::segmentMotion(*mhi, *segmask, *boundingRects, timestamp, segThresh);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// updateMotionHistory(InputArray, InputOutputArray, double, double)(InputArray, InputOutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/motempl.hpp:71
	// ("cv::motempl::updateMotionHistory", vec![(pred!(mut, ["silhouette", "mhi", "timestamp", "duration"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "double", "double"]), _)]),
	void cv_motempl_updateMotionHistory_const__InputArrayR_const__InputOutputArrayR_double_double(const cv::_InputArray* silhouette, const cv::_InputOutputArray* mhi, double timestamp, double duration, ResultVoid* ocvrs_return) {
		try {
			cv::motempl::updateMotionHistory(*silhouette, *mhi, timestamp, duration);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calcOpticalFlowSF(InputArray, InputArray, OutputArray, int, int, int)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:81
	// ("cv::optflow::calcOpticalFlowSF", vec![(pred!(mut, ["from", "to", "flow", "layers", "averaging_block_size", "max_flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
	void cv_optflow_calcOpticalFlowSF_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* from, const cv::_InputArray* to, const cv::_OutputArray* flow, int layers, int averaging_block_size, int max_flow, ResultVoid* ocvrs_return) {
		try {
			cv::optflow::calcOpticalFlowSF(*from, *to, *flow, layers, averaging_block_size, max_flow);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calcOpticalFlowSF(InputArray, InputArray, OutputArray, int, int, int, double, double, int, double, double, double, int, double, double, double)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:110
	// ("cv::optflow::calcOpticalFlowSF", vec![(pred!(mut, ["from", "to", "flow", "layers", "averaging_block_size", "max_flow", "sigma_dist", "sigma_color", "postprocess_window", "sigma_dist_fix", "sigma_color_fix", "occ_thr", "upscale_averaging_radius", "upscale_sigma_dist", "upscale_sigma_color", "speed_up_thr"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int", "double", "double", "int", "double", "double", "double", "int", "double", "double", "double"]), _)]),
	void cv_optflow_calcOpticalFlowSF_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_int_double_double_int_double_double_double_int_double_double_double(const cv::_InputArray* from, const cv::_InputArray* to, const cv::_OutputArray* flow, int layers, int averaging_block_size, int max_flow, double sigma_dist, double sigma_color, int postprocess_window, double sigma_dist_fix, double sigma_color_fix, double occ_thr, int upscale_averaging_radius, double upscale_sigma_dist, double upscale_sigma_color, double speed_up_thr, ResultVoid* ocvrs_return) {
		try {
			cv::optflow::calcOpticalFlowSF(*from, *to, *flow, layers, averaging_block_size, max_flow, sigma_dist, sigma_color, postprocess_window, sigma_dist_fix, sigma_color_fix, occ_thr, upscale_averaging_radius, upscale_sigma_dist, upscale_sigma_color, speed_up_thr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::optflow::calcOpticalFlowSparseToDense(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:135
	// ("cv::optflow::calcOpticalFlowSparseToDense", vec![(pred!(mut, ["from", "to", "flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_optflow_calcOpticalFlowSparseToDense_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* from, const cv::_InputArray* to, const cv::_OutputArray* flow, ResultVoid* ocvrs_return) {
		try {
			cv::optflow::calcOpticalFlowSparseToDense(*from, *to, *flow);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calcOpticalFlowSparseToDense(InputArray, InputArray, OutputArray, int, int, float, bool, float, float)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:135
	// ("cv::optflow::calcOpticalFlowSparseToDense", vec![(pred!(mut, ["from", "to", "flow", "grid_step", "k", "sigma", "use_post_proc", "fgs_lambda", "fgs_sigma"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "float", "bool", "float", "float"]), _)]),
	void cv_optflow_calcOpticalFlowSparseToDense_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_float_bool_float_float(const cv::_InputArray* from, const cv::_InputArray* to, const cv::_OutputArray* flow, int grid_step, int k, float sigma, bool use_post_proc, float fgs_lambda, float fgs_sigma, ResultVoid* ocvrs_return) {
		try {
			cv::optflow::calcOpticalFlowSparseToDense(*from, *to, *flow, grid_step, k, sigma, use_post_proc, fgs_lambda, fgs_sigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::optflow::createOptFlow_DIS() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:355
	// ("cv::optflow::createOptFlow_DIS", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_createOptFlow_DIS(Result<cv::Ptr<cv::optflow::DISOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::optflow::DISOpticalFlow> ret = cv::optflow::createOptFlow_DIS();
			Ok(new cv::Ptr<cv::optflow::DISOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createOptFlow_DIS(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:355
	// ("cv::optflow::createOptFlow_DIS", vec![(pred!(mut, ["preset"], ["int"]), _)]),
	void cv_optflow_createOptFlow_DIS_int(int preset, Result<cv::Ptr<cv::optflow::DISOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::optflow::DISOpticalFlow> ret = cv::optflow::createOptFlow_DIS(preset);
			Ok(new cv::Ptr<cv::optflow::DISOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createOptFlow_DeepFlow()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:244
	// ("cv::optflow::createOptFlow_DeepFlow", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_createOptFlow_DeepFlow(Result<cv::Ptr<cv::DenseOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DenseOpticalFlow> ret = cv::optflow::createOptFlow_DeepFlow();
			Ok(new cv::Ptr<cv::DenseOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createOptFlow_Farneback()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:250
	// ("cv::optflow::createOptFlow_Farneback", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_createOptFlow_Farneback(Result<cv::Ptr<cv::DenseOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DenseOpticalFlow> ret = cv::optflow::createOptFlow_Farneback();
			Ok(new cv::Ptr<cv::DenseOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createOptFlow_PCAFlow()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/pcaflow.hpp:142
	// ("cv::optflow::createOptFlow_PCAFlow", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_createOptFlow_PCAFlow(Result<cv::Ptr<cv::DenseOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DenseOpticalFlow> ret = cv::optflow::createOptFlow_PCAFlow();
			Ok(new cv::Ptr<cv::DenseOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createOptFlow_SimpleFlow()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:247
	// ("cv::optflow::createOptFlow_SimpleFlow", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_createOptFlow_SimpleFlow(Result<cv::Ptr<cv::DenseOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DenseOpticalFlow> ret = cv::optflow::createOptFlow_SimpleFlow();
			Ok(new cv::Ptr<cv::DenseOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createOptFlow_SparseToDense()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:253
	// ("cv::optflow::createOptFlow_SparseToDense", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_createOptFlow_SparseToDense(Result<cv::Ptr<cv::DenseOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DenseOpticalFlow> ret = cv::optflow::createOptFlow_SparseToDense();
			Ok(new cv::Ptr<cv::DenseOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createVariationalFlowRefinement()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:217
	// ("cv::optflow::createVariationalFlowRefinement", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_createVariationalFlowRefinement(Result<cv::Ptr<cv::optflow::VariationalRefinement>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::optflow::VariationalRefinement> ret = cv::optflow::createVariationalFlowRefinement();
			Ok(new cv::Ptr<cv::optflow::VariationalRefinement>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readOpticalFlow(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:148
	// ("cv::optflow::readOpticalFlow", vec![(pred!(mut, ["path"], ["const cv::String*"]), _)]),
	void cv_optflow_readOpticalFlow_const_StringR(const char* path, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::optflow::readOpticalFlow(cv::String(path));
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writeOpticalFlow(const String &, InputArray)(InString, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:158
	// ("cv::optflow::writeOpticalFlow", vec![(pred!(mut, ["path", "flow"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
	void cv_optflow_writeOpticalFlow_const_StringR_const__InputArrayR(const char* path, const cv::_InputArray* flow, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::optflow::writeOpticalFlow(cv::String(path), *flow);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &, optflow::GPCTree::Node &, optflow::GPCTree::Node)(TraitClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:369
	// ("cv::read", vec![(pred!(mut, ["fn", "node", "unnamed"], ["const cv::FileNode*", "cv::optflow::GPCTree::Node*", "cv::optflow::GPCTree::Node"]), _)]),
	void cv_read_const_FileNodeR_NodeR_Node(const cv::FileNode* fn, cv::optflow::GPCTree::Node* node, cv::optflow::GPCTree::Node* unnamed, ResultVoid* ocvrs_return) {
		try {
			cv::read(*fn, *node, *unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &, const String &, const optflow::GPCTree::Node &)(TraitClass, InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:367
	// ("cv::write", vec![(pred!(mut, ["fs", "name", "node"], ["cv::FileStorage*", "const cv::String*", "const cv::optflow::GPCTree::Node*"]), _)]),
	void cv_write_FileStorageR_const_StringR_const_NodeR(cv::FileStorage* fs, const char* name, const cv::optflow::GPCTree::Node* node, ResultVoid* ocvrs_return) {
		try {
			cv::write(*fs, cv::String(name), *node);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFinestScale()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:280
	// ("cv::optflow::DISOpticalFlow::getFinestScale", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DISOpticalFlow_getFinestScale_const(const cv::optflow::DISOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFinestScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFinestScale(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:282
	// ("cv::optflow::DISOpticalFlow::setFinestScale", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_optflow_DISOpticalFlow_setFinestScale_int(cv::optflow::DISOpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setFinestScale(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPatchSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:287
	// ("cv::optflow::DISOpticalFlow::getPatchSize", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DISOpticalFlow_getPatchSize_const(const cv::optflow::DISOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPatchSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPatchSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:289
	// ("cv::optflow::DISOpticalFlow::setPatchSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_optflow_DISOpticalFlow_setPatchSize_int(cv::optflow::DISOpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setPatchSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPatchStride()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:294
	// ("cv::optflow::DISOpticalFlow::getPatchStride", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DISOpticalFlow_getPatchStride_const(const cv::optflow::DISOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPatchStride();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPatchStride(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:296
	// ("cv::optflow::DISOpticalFlow::setPatchStride", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_optflow_DISOpticalFlow_setPatchStride_int(cv::optflow::DISOpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setPatchStride(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGradientDescentIterations()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:301
	// ("cv::optflow::DISOpticalFlow::getGradientDescentIterations", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DISOpticalFlow_getGradientDescentIterations_const(const cv::optflow::DISOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getGradientDescentIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setGradientDescentIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:303
	// ("cv::optflow::DISOpticalFlow::setGradientDescentIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_optflow_DISOpticalFlow_setGradientDescentIterations_int(cv::optflow::DISOpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setGradientDescentIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVariationalRefinementIterations()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:309
	// ("cv::optflow::DISOpticalFlow::getVariationalRefinementIterations", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DISOpticalFlow_getVariationalRefinementIterations_const(const cv::optflow::DISOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getVariationalRefinementIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setVariationalRefinementIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:311
	// ("cv::optflow::DISOpticalFlow::setVariationalRefinementIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_optflow_DISOpticalFlow_setVariationalRefinementIterations_int(cv::optflow::DISOpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setVariationalRefinementIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVariationalRefinementAlpha()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:315
	// ("cv::optflow::DISOpticalFlow::getVariationalRefinementAlpha", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DISOpticalFlow_getVariationalRefinementAlpha_const(const cv::optflow::DISOpticalFlow* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getVariationalRefinementAlpha();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setVariationalRefinementAlpha(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:317
	// ("cv::optflow::DISOpticalFlow::setVariationalRefinementAlpha", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_optflow_DISOpticalFlow_setVariationalRefinementAlpha_float(cv::optflow::DISOpticalFlow* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setVariationalRefinementAlpha(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVariationalRefinementDelta()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:321
	// ("cv::optflow::DISOpticalFlow::getVariationalRefinementDelta", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DISOpticalFlow_getVariationalRefinementDelta_const(const cv::optflow::DISOpticalFlow* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getVariationalRefinementDelta();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setVariationalRefinementDelta(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:323
	// ("cv::optflow::DISOpticalFlow::setVariationalRefinementDelta", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_optflow_DISOpticalFlow_setVariationalRefinementDelta_float(cv::optflow::DISOpticalFlow* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setVariationalRefinementDelta(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVariationalRefinementGamma()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:327
	// ("cv::optflow::DISOpticalFlow::getVariationalRefinementGamma", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DISOpticalFlow_getVariationalRefinementGamma_const(const cv::optflow::DISOpticalFlow* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getVariationalRefinementGamma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setVariationalRefinementGamma(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:329
	// ("cv::optflow::DISOpticalFlow::setVariationalRefinementGamma", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_optflow_DISOpticalFlow_setVariationalRefinementGamma_float(cv::optflow::DISOpticalFlow* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setVariationalRefinementGamma(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUseMeanNormalization()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:337
	// ("cv::optflow::DISOpticalFlow::getUseMeanNormalization", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DISOpticalFlow_getUseMeanNormalization_const(const cv::optflow::DISOpticalFlow* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseMeanNormalization();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseMeanNormalization(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:339
	// ("cv::optflow::DISOpticalFlow::setUseMeanNormalization", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	void cv_optflow_DISOpticalFlow_setUseMeanNormalization_bool(cv::optflow::DISOpticalFlow* instance, bool val, ResultVoid* ocvrs_return) {
		try {
			instance->setUseMeanNormalization(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUseSpatialPropagation()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:346
	// ("cv::optflow::DISOpticalFlow::getUseSpatialPropagation", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DISOpticalFlow_getUseSpatialPropagation_const(const cv::optflow::DISOpticalFlow* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseSpatialPropagation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseSpatialPropagation(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:348
	// ("cv::optflow::DISOpticalFlow::setUseSpatialPropagation", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	void cv_optflow_DISOpticalFlow_setUseSpatialPropagation_bool(cv::optflow::DISOpticalFlow* instance, bool val, ResultVoid* ocvrs_return) {
		try {
			instance->setUseSpatialPropagation(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::optflow::DISOpticalFlow::to_Algorithm() generated
	// ("cv::optflow::DISOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_optflow_DISOpticalFlow_to_Algorithm(cv::optflow::DISOpticalFlow* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::optflow::DISOpticalFlow::to_DenseOpticalFlow() generated
	// ("cv::optflow::DISOpticalFlow::to_DenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::DenseOpticalFlow* cv_optflow_DISOpticalFlow_to_DenseOpticalFlow(cv::optflow::DISOpticalFlow* instance) {
			return dynamic_cast<cv::DenseOpticalFlow*>(instance);
	}

	// cv::optflow::DISOpticalFlow::delete() generated
	// ("cv::optflow::DISOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_DISOpticalFlow_delete(cv::optflow::DISOpticalFlow* instance) {
			delete instance;
	}

	// dropOutliers(std::vector<std::pair<Point2i, Point2i>> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:304
	// ("cv::optflow::GPCDetails::dropOutliers", vec![(pred!(mut, ["corr"], ["std::vector<std::pair<cv::Point2i, cv::Point2i>>*"]), _)]),
	void cv_optflow_GPCDetails_dropOutliers_vectorLpairLcv_Point2i__cv_Point2iGGR(std::vector<std::pair<cv::Point2i, cv::Point2i>>* corr, ResultVoid* ocvrs_return) {
		try {
			cv::optflow::GPCDetails::dropOutliers(*corr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAllDescriptorsForImage(const Mat *, std::vector<GPCPatchDescriptor> &, const GPCMatchingParams &, int)(TraitClass, CppPassByVoidPtr, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:306
	// ("cv::optflow::GPCDetails::getAllDescriptorsForImage", vec![(pred!(mut, ["imgCh", "descr", "mp", "type"], ["const cv::Mat*", "std::vector<cv::optflow::GPCPatchDescriptor>*", "const cv::optflow::GPCMatchingParams*", "int"]), _)]),
	void cv_optflow_GPCDetails_getAllDescriptorsForImage_const_MatX_vectorLGPCPatchDescriptorGR_const_GPCMatchingParamsR_int(const cv::Mat* imgCh, std::vector<cv::optflow::GPCPatchDescriptor>* descr, const cv::optflow::GPCMatchingParams* mp, int type, ResultVoid* ocvrs_return) {
		try {
			cv::optflow::GPCDetails::getAllDescriptorsForImage(imgCh, *descr, *mp, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCoordinatesFromIndex(size_t, Size, int &, int &)(Primitive, SimpleClass, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:309
	// ("cv::optflow::GPCDetails::getCoordinatesFromIndex", vec![(pred!(mut, ["index", "sz", "x", "y"], ["size_t", "cv::Size", "int*", "int*"]), _)]),
	void cv_optflow_GPCDetails_getCoordinatesFromIndex_size_t_Size_intR_intR(size_t index, cv::Size* sz, int* x, int* y, ResultVoid* ocvrs_return) {
		try {
			cv::optflow::GPCDetails::getCoordinatesFromIndex(index, *sz, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::optflow::GPCDetails::defaultNew() generated
	// ("cv::optflow::GPCDetails::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::optflow::GPCDetails* cv_optflow_GPCDetails_defaultNew_const() {
			cv::optflow::GPCDetails* ret = new cv::optflow::GPCDetails();
			return ret;
	}

	// cv::optflow::GPCDetails::delete() generated
	// ("cv::optflow::GPCDetails::delete", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_GPCDetails_delete(cv::optflow::GPCDetails* instance) {
			delete instance;
	}

	// GPCMatchingParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:147
	// ("cv::optflow::GPCMatchingParams::GPCMatchingParams", vec![(pred!(mut, ["_useOpenCL"], ["bool"]), _)]),
	void cv_optflow_GPCMatchingParams_GPCMatchingParams_bool(bool _useOpenCL, Result<cv::optflow::GPCMatchingParams>* ocvrs_return) {
		try {
			cv::optflow::GPCMatchingParams ret(_useOpenCL);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::optflow::GPCMatchingParams::GPCMatchingParams() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:147
	// ("cv::optflow::GPCMatchingParams::GPCMatchingParams", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_GPCMatchingParams_GPCMatchingParams(Result<cv::optflow::GPCMatchingParams>* ocvrs_return) {
		try {
			cv::optflow::GPCMatchingParams ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GPCMatchingParams(const GPCMatchingParams &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:149
	// ("cv::optflow::GPCMatchingParams::GPCMatchingParams", vec![(pred!(mut, ["params"], ["const cv::optflow::GPCMatchingParams*"]), _)]),
	void cv_optflow_GPCMatchingParams_GPCMatchingParams_const_GPCMatchingParamsR(const cv::optflow::GPCMatchingParams* params, Result<cv::optflow::GPCMatchingParams>* ocvrs_return) {
		try {
			cv::optflow::GPCMatchingParams ret(*params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dot(const Vec<double, nFeatures> &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:70
	// ("cv::optflow::GPCPatchDescriptor::dot", vec![(pred!(const, ["coef"], ["const cv::Vec<double, 18>*"]), _)]),
	void cv_optflow_GPCPatchDescriptor_dot_const_const_VecLdouble__18GR(const cv::optflow::GPCPatchDescriptor* instance, const cv::Vec<double, 18>* coef, Result<double>* ocvrs_return) {
		try {
			double ret = instance->dot(*coef);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// markAsSeparated()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:72
	// ("cv::optflow::GPCPatchDescriptor::markAsSeparated", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_GPCPatchDescriptor_markAsSeparated(cv::optflow::GPCPatchDescriptor* instance, ResultVoid* ocvrs_return) {
		try {
			instance->markAsSeparated();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isSeparated()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:74
	// ("cv::optflow::GPCPatchDescriptor::isSeparated", vec![(pred!(const, [], []), _)]),
	void cv_optflow_GPCPatchDescriptor_isSeparated_const(const cv::optflow::GPCPatchDescriptor* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isSeparated();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::optflow::GPCPatchDescriptor::defaultNew() generated
	// ("cv::optflow::GPCPatchDescriptor::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::optflow::GPCPatchDescriptor* cv_optflow_GPCPatchDescriptor_defaultNew_const() {
			cv::optflow::GPCPatchDescriptor* ret = new cv::optflow::GPCPatchDescriptor();
			return ret;
	}

	// cv::optflow::GPCPatchDescriptor::feature() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:68
	// ("cv::optflow::GPCPatchDescriptor::feature", vec![(pred!(const, [], []), _)]),
	void cv_optflow_GPCPatchDescriptor_propFeature_const(const cv::optflow::GPCPatchDescriptor* instance, cv::Vec<double, 18>* ocvrs_return) {
			cv::Vec<double, 18> ret = instance->feature;
			*ocvrs_return = ret;
	}

	// cv::optflow::GPCPatchDescriptor::setFeature(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:68
	// ("cv::optflow::GPCPatchDescriptor::setFeature", vec![(pred!(mut, ["val"], ["const cv::Vec<double, 18>"]), _)]),
	void cv_optflow_GPCPatchDescriptor_propFeature_const_VecLdouble__18G(cv::optflow::GPCPatchDescriptor* instance, const cv::Vec<double, 18>* val) {
			instance->feature = *val;
	}

	// cv::optflow::GPCPatchDescriptor::delete() generated
	// ("cv::optflow::GPCPatchDescriptor::delete", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_GPCPatchDescriptor_delete(cv::optflow::GPCPatchDescriptor* instance) {
			delete instance;
	}

	// getDirections(bool &, bool &, bool &, const Vec<double, GPCPatchDescriptor::nFeatures> &, double)(Indirect, Indirect, Indirect, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:83
	// ("cv::optflow::GPCPatchSample::getDirections", vec![(pred!(const, ["refdir", "posdir", "negdir", "coef", "rhs"], ["bool*", "bool*", "bool*", "const cv::Vec<double, 18>*", "double"]), _)]),
	void cv_optflow_GPCPatchSample_getDirections_const_boolR_boolR_boolR_const_VecLdouble__18GR_double(const cv::optflow::GPCPatchSample* instance, bool* refdir, bool* posdir, bool* negdir, const cv::Vec<double, 18>* coef, double rhs, ResultVoid* ocvrs_return) {
		try {
			instance->getDirections(*refdir, *posdir, *negdir, *coef, rhs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::optflow::GPCPatchSample::defaultNew() generated
	// ("cv::optflow::GPCPatchSample::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::optflow::GPCPatchSample* cv_optflow_GPCPatchSample_defaultNew_const() {
			cv::optflow::GPCPatchSample* ret = new cv::optflow::GPCPatchSample();
			return ret;
	}

	// cv::optflow::GPCPatchSample::ref() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:79
	// ("cv::optflow::GPCPatchSample::ref", vec![(pred!(const, [], []), _)]),
	cv::optflow::GPCPatchDescriptor* cv_optflow_GPCPatchSample_propRef_const(const cv::optflow::GPCPatchSample* instance) {
			cv::optflow::GPCPatchDescriptor ret = instance->ref;
			return new cv::optflow::GPCPatchDescriptor(ret);
	}

	// cv::optflow::GPCPatchSample::setRef(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:79
	// ("cv::optflow::GPCPatchSample::setRef", vec![(pred!(mut, ["val"], ["const cv::optflow::GPCPatchDescriptor"]), _)]),
	void cv_optflow_GPCPatchSample_propRef_const_GPCPatchDescriptor(cv::optflow::GPCPatchSample* instance, const cv::optflow::GPCPatchDescriptor* val) {
			instance->ref = *val;
	}

	// cv::optflow::GPCPatchSample::pos() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:80
	// ("cv::optflow::GPCPatchSample::pos", vec![(pred!(const, [], []), _)]),
	cv::optflow::GPCPatchDescriptor* cv_optflow_GPCPatchSample_propPos_const(const cv::optflow::GPCPatchSample* instance) {
			cv::optflow::GPCPatchDescriptor ret = instance->pos;
			return new cv::optflow::GPCPatchDescriptor(ret);
	}

	// cv::optflow::GPCPatchSample::setPos(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:80
	// ("cv::optflow::GPCPatchSample::setPos", vec![(pred!(mut, ["val"], ["const cv::optflow::GPCPatchDescriptor"]), _)]),
	void cv_optflow_GPCPatchSample_propPos_const_GPCPatchDescriptor(cv::optflow::GPCPatchSample* instance, const cv::optflow::GPCPatchDescriptor* val) {
			instance->pos = *val;
	}

	// cv::optflow::GPCPatchSample::neg() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:81
	// ("cv::optflow::GPCPatchSample::neg", vec![(pred!(const, [], []), _)]),
	cv::optflow::GPCPatchDescriptor* cv_optflow_GPCPatchSample_propNeg_const(const cv::optflow::GPCPatchSample* instance) {
			cv::optflow::GPCPatchDescriptor ret = instance->neg;
			return new cv::optflow::GPCPatchDescriptor(ret);
	}

	// cv::optflow::GPCPatchSample::setNeg(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:81
	// ("cv::optflow::GPCPatchSample::setNeg", vec![(pred!(mut, ["val"], ["const cv::optflow::GPCPatchDescriptor"]), _)]),
	void cv_optflow_GPCPatchSample_propNeg_const_GPCPatchDescriptor(cv::optflow::GPCPatchSample* instance, const cv::optflow::GPCPatchDescriptor* val) {
			instance->neg = *val;
	}

	// cv::optflow::GPCPatchSample::delete() generated
	// ("cv::optflow::GPCPatchSample::delete", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_GPCPatchSample_delete(cv::optflow::GPCPatchSample* instance) {
			delete instance;
	}

	// GPCTrainingParams(unsigned int, int, GPCDescType, bool)(Primitive, Primitive, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:130
	// ("cv::optflow::GPCTrainingParams::GPCTrainingParams", vec![(pred!(mut, ["_maxTreeDepth", "_minNumberOfSamples", "_descriptorType", "_printProgress"], ["unsigned int", "int", "cv::optflow::GPCDescType", "bool"]), _)]),
	void cv_optflow_GPCTrainingParams_GPCTrainingParams_unsigned_int_int_GPCDescType_bool(unsigned int _maxTreeDepth, int _minNumberOfSamples, cv::optflow::GPCDescType _descriptorType, bool _printProgress, Result<cv::optflow::GPCTrainingParams>* ocvrs_return) {
		try {
			cv::optflow::GPCTrainingParams ret(_maxTreeDepth, _minNumberOfSamples, _descriptorType, _printProgress);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::optflow::GPCTrainingParams::GPCTrainingParams() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:130
	// ("cv::optflow::GPCTrainingParams::GPCTrainingParams", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_GPCTrainingParams_GPCTrainingParams(Result<cv::optflow::GPCTrainingParams>* ocvrs_return) {
		try {
			cv::optflow::GPCTrainingParams ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// check()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:138
	// ("cv::optflow::GPCTrainingParams::check", vec![(pred!(const, [], []), _)]),
	void cv_optflow_GPCTrainingParams_check_const(const cv::optflow::GPCTrainingParams* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->check();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const std::vector<String> &, const std::vector<String> &, const std::vector<String> &, int)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:108
	// ("cv::optflow::GPCTrainingSamples::create", vec![(pred!(mut, ["imagesFrom", "imagesTo", "gt", "descriptorType"], ["const std::vector<cv::String>*", "const std::vector<cv::String>*", "const std::vector<cv::String>*", "int"]), _)]),
	void cv_optflow_GPCTrainingSamples_create_const_vectorLStringGR_const_vectorLStringGR_const_vectorLStringGR_int(const std::vector<cv::String>* imagesFrom, const std::vector<cv::String>* imagesTo, const std::vector<cv::String>* gt, int descriptorType, Result<cv::Ptr<cv::optflow::GPCTrainingSamples>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::optflow::GPCTrainingSamples> ret = cv::optflow::GPCTrainingSamples::create(*imagesFrom, *imagesTo, *gt, descriptorType);
			Ok(new cv::Ptr<cv::optflow::GPCTrainingSamples>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(InputArrayOfArrays, InputArrayOfArrays, InputArrayOfArrays, int)(InputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:111
	// ("cv::optflow::GPCTrainingSamples::create", vec![(pred!(mut, ["imagesFrom", "imagesTo", "gt", "descriptorType"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
	void cv_optflow_GPCTrainingSamples_create_const__InputArrayR_const__InputArrayR_const__InputArrayR_int(const cv::_InputArray* imagesFrom, const cv::_InputArray* imagesTo, const cv::_InputArray* gt, int descriptorType, Result<cv::Ptr<cv::optflow::GPCTrainingSamples>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::optflow::GPCTrainingSamples> ret = cv::optflow::GPCTrainingSamples::create(*imagesFrom, *imagesTo, *gt, descriptorType);
			Ok(new cv::Ptr<cv::optflow::GPCTrainingSamples>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// size()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:114
	// ("cv::optflow::GPCTrainingSamples::size", vec![(pred!(const, [], []), _)]),
	void cv_optflow_GPCTrainingSamples_size_const(const cv::optflow::GPCTrainingSamples* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->size();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// type()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:116
	// ("cv::optflow::GPCTrainingSamples::type", vec![(pred!(const, [], []), _)]),
	void cv_optflow_GPCTrainingSamples_type_const(const cv::optflow::GPCTrainingSamples* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->type();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::optflow::GPCTrainingSamples::defaultNew() generated
	// ("cv::optflow::GPCTrainingSamples::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::optflow::GPCTrainingSamples* cv_optflow_GPCTrainingSamples_defaultNew_const() {
			cv::optflow::GPCTrainingSamples* ret = new cv::optflow::GPCTrainingSamples();
			return ret;
	}

	// cv::optflow::GPCTrainingSamples::delete() generated
	// ("cv::optflow::GPCTrainingSamples::delete", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_GPCTrainingSamples_delete(cv::optflow::GPCTrainingSamples* instance) {
			delete instance;
	}

	// train(GPCTrainingSamples &, const GPCTrainingParams)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:176
	// ("cv::optflow::GPCTree::train", vec![(pred!(mut, ["samples", "params"], ["cv::optflow::GPCTrainingSamples*", "const cv::optflow::GPCTrainingParams"]), _)]),
	void cv_optflow_GPCTree_train_GPCTrainingSamplesR_const_GPCTrainingParams(cv::optflow::GPCTree* instance, cv::optflow::GPCTrainingSamples* samples, const cv::optflow::GPCTrainingParams* params, ResultVoid* ocvrs_return) {
		try {
			instance->train(*samples, *params);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::optflow::GPCTree::train(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:176
	// ("cv::optflow::GPCTree::train", vec![(pred!(mut, ["samples"], ["cv::optflow::GPCTrainingSamples*"]), _)]),
	void cv_optflow_GPCTree_train_GPCTrainingSamplesR(cv::optflow::GPCTree* instance, cv::optflow::GPCTrainingSamples* samples, ResultVoid* ocvrs_return) {
		try {
			instance->train(*samples);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:178
	// ("cv::optflow::GPCTree::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_optflow_GPCTree_write_const_FileStorageR(const cv::optflow::GPCTree* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:180
	// ("cv::optflow::GPCTree::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_optflow_GPCTree_read_const_FileNodeR(cv::optflow::GPCTree* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findLeafForPatch(const GPCPatchDescriptor &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:182
	// ("cv::optflow::GPCTree::findLeafForPatch", vec![(pred!(const, ["descr"], ["const cv::optflow::GPCPatchDescriptor*"]), _)]),
	void cv_optflow_GPCTree_findLeafForPatch_const_const_GPCPatchDescriptorR(const cv::optflow::GPCTree* instance, const cv::optflow::GPCPatchDescriptor* descr, Result<unsigned int>* ocvrs_return) {
		try {
			unsigned int ret = instance->findLeafForPatch(*descr);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:184
	// ("cv::optflow::GPCTree::create", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_GPCTree_create(Result<cv::Ptr<cv::optflow::GPCTree>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::optflow::GPCTree> ret = cv::optflow::GPCTree::create();
			Ok(new cv::Ptr<cv::optflow::GPCTree>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator==(const GPCTree &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:186
	// ("cv::optflow::GPCTree::operator==", vec![(pred!(const, ["t"], ["const cv::optflow::GPCTree*"]), _)]),
	void cv_optflow_GPCTree_operatorEQ_const_const_GPCTreeR(const cv::optflow::GPCTree* instance, const cv::optflow::GPCTree* t, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator==(*t);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDescriptorType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:188
	// ("cv::optflow::GPCTree::getDescriptorType", vec![(pred!(const, [], []), _)]),
	void cv_optflow_GPCTree_getDescriptorType_const(const cv::optflow::GPCTree* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDescriptorType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::optflow::GPCTree::defaultNew() generated
	// ("cv::optflow::GPCTree::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::optflow::GPCTree* cv_optflow_GPCTree_defaultNew_const() {
			cv::optflow::GPCTree* ret = new cv::optflow::GPCTree();
			return ret;
	}

	// cv::optflow::GPCTree::to_Algorithm() generated
	// ("cv::optflow::GPCTree::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_optflow_GPCTree_to_Algorithm(cv::optflow::GPCTree* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::optflow::GPCTree::delete() generated
	// ("cv::optflow::GPCTree::delete", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_GPCTree_delete(cv::optflow::GPCTree* instance) {
			delete instance;
	}

	// operator==(const Node &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:164
	// ("cv::optflow::GPCTree::Node::operator==", vec![(pred!(const, ["n"], ["const cv::optflow::GPCTree::Node*"]), _)]),
	void cv_optflow_GPCTree_Node_operatorEQ_const_const_NodeR(const cv::optflow::GPCTree::Node* instance, const cv::optflow::GPCTree::Node* n, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator==(*n);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// OpticalFlowPCAFlow(Ptr<const PCAPrior>, const Size, float, float, float, float, float)(CppPassByVoidPtr, SimpleClass, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/pcaflow.hpp:116
	// ("cv::optflow::OpticalFlowPCAFlow::OpticalFlowPCAFlow", vec![(pred!(mut, ["_prior", "_basisSize", "_sparseRate", "_retainedCornersFraction", "_occlusionsThreshold", "_dampingFactor", "_claheClip"], ["cv::Ptr<const cv::optflow::PCAPrior>", "const cv::Size", "float", "float", "float", "float", "float"]), _)]),
	void cv_optflow_OpticalFlowPCAFlow_OpticalFlowPCAFlow_PtrLconst_PCAPriorG_const_Size_float_float_float_float_float(const cv::Ptr<const cv::optflow::PCAPrior>* _prior, const cv::Size* _basisSize, float _sparseRate, float _retainedCornersFraction, float _occlusionsThreshold, float _dampingFactor, float _claheClip, Result<cv::optflow::OpticalFlowPCAFlow*>* ocvrs_return) {
		try {
			cv::optflow::OpticalFlowPCAFlow* ret = new cv::optflow::OpticalFlowPCAFlow(*_prior, *_basisSize, _sparseRate, _retainedCornersFraction, _occlusionsThreshold, _dampingFactor, _claheClip);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::optflow::OpticalFlowPCAFlow::OpticalFlowPCAFlow() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/pcaflow.hpp:116
	// ("cv::optflow::OpticalFlowPCAFlow::OpticalFlowPCAFlow", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_OpticalFlowPCAFlow_OpticalFlowPCAFlow(Result<cv::optflow::OpticalFlowPCAFlow*>* ocvrs_return) {
		try {
			cv::optflow::OpticalFlowPCAFlow* ret = new cv::optflow::OpticalFlowPCAFlow();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calc(InputArray, InputArray, InputOutputArray)(InputArray, InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/pcaflow.hpp:120
	// ("cv::optflow::OpticalFlowPCAFlow::calc", vec![(pred!(mut, ["I0", "I1", "flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_optflow_OpticalFlowPCAFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(cv::optflow::OpticalFlowPCAFlow* instance, const cv::_InputArray* I0, const cv::_InputArray* I1, const cv::_InputOutputArray* flow, ResultVoid* ocvrs_return) {
		try {
			instance->calc(*I0, *I1, *flow);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// collectGarbage()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/pcaflow.hpp:121
	// ("cv::optflow::OpticalFlowPCAFlow::collectGarbage", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_OpticalFlowPCAFlow_collectGarbage(cv::optflow::OpticalFlowPCAFlow* instance, ResultVoid* ocvrs_return) {
		try {
			instance->collectGarbage();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::optflow::OpticalFlowPCAFlow::to_Algorithm() generated
	// ("cv::optflow::OpticalFlowPCAFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_optflow_OpticalFlowPCAFlow_to_Algorithm(cv::optflow::OpticalFlowPCAFlow* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::optflow::OpticalFlowPCAFlow::to_DenseOpticalFlow() generated
	// ("cv::optflow::OpticalFlowPCAFlow::to_DenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::DenseOpticalFlow* cv_optflow_OpticalFlowPCAFlow_to_DenseOpticalFlow(cv::optflow::OpticalFlowPCAFlow* instance) {
			return dynamic_cast<cv::DenseOpticalFlow*>(instance);
	}

	// cv::optflow::OpticalFlowPCAFlow::delete() generated
	// ("cv::optflow::OpticalFlowPCAFlow::delete", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_OpticalFlowPCAFlow_delete(cv::optflow::OpticalFlowPCAFlow* instance) {
			delete instance;
	}

	// PCAPrior(const char *)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/pcaflow.hpp:83
	// ("cv::optflow::PCAPrior::PCAPrior", vec![(pred!(mut, ["pathToPrior"], ["const char*"]), _)]),
	void cv_optflow_PCAPrior_PCAPrior_const_charX(const char* pathToPrior, Result<cv::optflow::PCAPrior*>* ocvrs_return) {
		try {
			cv::optflow::PCAPrior* ret = new cv::optflow::PCAPrior(pathToPrior);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPadding()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/pcaflow.hpp:85
	// ("cv::optflow::PCAPrior::getPadding", vec![(pred!(const, [], []), _)]),
	void cv_optflow_PCAPrior_getPadding_const(const cv::optflow::PCAPrior* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPadding();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBasisSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/pcaflow.hpp:87
	// ("cv::optflow::PCAPrior::getBasisSize", vec![(pred!(const, [], []), _)]),
	void cv_optflow_PCAPrior_getBasisSize_const(const cv::optflow::PCAPrior* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getBasisSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fillConstraints(float *, float *, float *, float *)(Indirect, Indirect, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/pcaflow.hpp:89
	// ("cv::optflow::PCAPrior::fillConstraints", vec![(pred!(const, ["A1", "A2", "b1", "b2"], ["float*", "float*", "float*", "float*"]), _)]),
	void cv_optflow_PCAPrior_fillConstraints_const_floatX_floatX_floatX_floatX(const cv::optflow::PCAPrior* instance, float* A1, float* A2, float* b1, float* b2, ResultVoid* ocvrs_return) {
		try {
			instance->fillConstraints(A1, A2, b1, b2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::optflow::PCAPrior::delete() generated
	// ("cv::optflow::PCAPrior::delete", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_PCAPrior_delete(cv::optflow::PCAPrior* instance) {
			delete instance;
	}

	// calcUV(InputArray, InputArray, InputOutputArray, InputOutputArray)(InputArray, InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:175
	// ("cv::optflow::VariationalRefinement::calcUV", vec![(pred!(mut, ["I0", "I1", "flow_u", "flow_v"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_optflow_VariationalRefinement_calcUV_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(cv::optflow::VariationalRefinement* instance, const cv::_InputArray* I0, const cv::_InputArray* I1, const cv::_InputOutputArray* flow_u, const cv::_InputOutputArray* flow_v, ResultVoid* ocvrs_return) {
		try {
			instance->calcUV(*I0, *I1, *flow_u, *flow_v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFixedPointIterations()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:179
	// ("cv::optflow::VariationalRefinement::getFixedPointIterations", vec![(pred!(const, [], []), _)]),
	void cv_optflow_VariationalRefinement_getFixedPointIterations_const(const cv::optflow::VariationalRefinement* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFixedPointIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFixedPointIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:181
	// ("cv::optflow::VariationalRefinement::setFixedPointIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_optflow_VariationalRefinement_setFixedPointIterations_int(cv::optflow::VariationalRefinement* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setFixedPointIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSorIterations()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:186
	// ("cv::optflow::VariationalRefinement::getSorIterations", vec![(pred!(const, [], []), _)]),
	void cv_optflow_VariationalRefinement_getSorIterations_const(const cv::optflow::VariationalRefinement* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSorIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSorIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:188
	// ("cv::optflow::VariationalRefinement::setSorIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_optflow_VariationalRefinement_setSorIterations_int(cv::optflow::VariationalRefinement* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setSorIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOmega()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:192
	// ("cv::optflow::VariationalRefinement::getOmega", vec![(pred!(const, [], []), _)]),
	void cv_optflow_VariationalRefinement_getOmega_const(const cv::optflow::VariationalRefinement* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getOmega();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setOmega(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:194
	// ("cv::optflow::VariationalRefinement::setOmega", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_optflow_VariationalRefinement_setOmega_float(cv::optflow::VariationalRefinement* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setOmega(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAlpha()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:198
	// ("cv::optflow::VariationalRefinement::getAlpha", vec![(pred!(const, [], []), _)]),
	void cv_optflow_VariationalRefinement_getAlpha_const(const cv::optflow::VariationalRefinement* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getAlpha();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setAlpha(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:200
	// ("cv::optflow::VariationalRefinement::setAlpha", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_optflow_VariationalRefinement_setAlpha_float(cv::optflow::VariationalRefinement* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setAlpha(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDelta()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:204
	// ("cv::optflow::VariationalRefinement::getDelta", vec![(pred!(const, [], []), _)]),
	void cv_optflow_VariationalRefinement_getDelta_const(const cv::optflow::VariationalRefinement* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getDelta();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDelta(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:206
	// ("cv::optflow::VariationalRefinement::setDelta", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_optflow_VariationalRefinement_setDelta_float(cv::optflow::VariationalRefinement* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setDelta(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGamma()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:210
	// ("cv::optflow::VariationalRefinement::getGamma", vec![(pred!(const, [], []), _)]),
	void cv_optflow_VariationalRefinement_getGamma_const(const cv::optflow::VariationalRefinement* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getGamma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setGamma(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:212
	// ("cv::optflow::VariationalRefinement::setGamma", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_optflow_VariationalRefinement_setGamma_float(cv::optflow::VariationalRefinement* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setGamma(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::optflow::VariationalRefinement::to_Algorithm() generated
	// ("cv::optflow::VariationalRefinement::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_optflow_VariationalRefinement_to_Algorithm(cv::optflow::VariationalRefinement* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::optflow::VariationalRefinement::to_DenseOpticalFlow() generated
	// ("cv::optflow::VariationalRefinement::to_DenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::DenseOpticalFlow* cv_optflow_VariationalRefinement_to_DenseOpticalFlow(cv::optflow::VariationalRefinement* instance) {
			return dynamic_cast<cv::DenseOpticalFlow*>(instance);
	}

	// cv::optflow::VariationalRefinement::delete() generated
	// ("cv::optflow::VariationalRefinement::delete", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_VariationalRefinement_delete(cv::optflow::VariationalRefinement* instance) {
			delete instance;
	}

}
