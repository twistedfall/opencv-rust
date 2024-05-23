#include "ocvrs_common.hpp"
#include <opencv2/optflow.hpp>
#include "optflow_types.hpp"

extern "C" {
	// calcGlobalOrientation(InputArray, InputArray, InputArray, double, double)(InputArray, InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/motempl.hpp:119
	// ("cv::motempl::calcGlobalOrientation", vec![(pred!(mut, ["orientation", "mask", "mhi", "timestamp", "duration"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "double", "double"]), _)]),
	void cv_motempl_calcGlobalOrientation_const__InputArrayR_const__InputArrayR_const__InputArrayR_double_double(const cv::_InputArray* orientation, const cv::_InputArray* mask, const cv::_InputArray* mhi, double timestamp, double duration, Result<double>* ocvrs_return) {
		try {
			double ret = cv::motempl::calcGlobalOrientation(*orientation, *mask, *mhi, timestamp, duration);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::motempl::calcMotionGradient(InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/motempl.hpp:102
	// ("cv::motempl::calcMotionGradient", vec![(pred!(mut, ["mhi", "mask", "orientation", "delta1", "delta2"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
	void cv_motempl_calcMotionGradient_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_double(const cv::_InputArray* mhi, const cv::_OutputArray* mask, const cv::_OutputArray* orientation, double delta1, double delta2, ResultVoid* ocvrs_return) {
		try {
			cv::motempl::calcMotionGradient(*mhi, *mask, *orientation, delta1, delta2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calcMotionGradient(InputArray, OutputArray, OutputArray, double, double, int)(InputArray, OutputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/motempl.hpp:102
	// ("cv::motempl::calcMotionGradient", vec![(pred!(mut, ["mhi", "mask", "orientation", "delta1", "delta2", "apertureSize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "double", "int"]), _)]),
	void cv_motempl_calcMotionGradient_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_double_int(const cv::_InputArray* mhi, const cv::_OutputArray* mask, const cv::_OutputArray* orientation, double delta1, double delta2, int apertureSize, ResultVoid* ocvrs_return) {
		try {
			cv::motempl::calcMotionGradient(*mhi, *mask, *orientation, delta1, delta2, apertureSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// segmentMotion(InputArray, OutputArray, std::vector<Rect> &, double, double)(InputArray, OutputArray, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/motempl.hpp:137
	// ("cv::motempl::segmentMotion", vec![(pred!(mut, ["mhi", "segmask", "boundingRects", "timestamp", "segThresh"], ["const cv::_InputArray*", "const cv::_OutputArray*", "std::vector<cv::Rect>*", "double", "double"]), _)]),
	void cv_motempl_segmentMotion_const__InputArrayR_const__OutputArrayR_vectorLRectGR_double_double(const cv::_InputArray* mhi, const cv::_OutputArray* segmask, std::vector<cv::Rect>* boundingRects, double timestamp, double segThresh, ResultVoid* ocvrs_return) {
		try {
			cv::motempl::segmentMotion(*mhi, *segmask, *boundingRects, timestamp, segThresh);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// updateMotionHistory(InputArray, InputOutputArray, double, double)(InputArray, InputOutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/motempl.hpp:71
	// ("cv::motempl::updateMotionHistory", vec![(pred!(mut, ["silhouette", "mhi", "timestamp", "duration"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "double", "double"]), _)]),
	void cv_motempl_updateMotionHistory_const__InputArrayR_const__InputOutputArrayR_double_double(const cv::_InputArray* silhouette, const cv::_InputOutputArray* mhi, double timestamp, double duration, ResultVoid* ocvrs_return) {
		try {
			cv::motempl::updateMotionHistory(*silhouette, *mhi, timestamp, duration);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::optflow::calcOpticalFlowDenseRLOF(InputArray, InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:501
	// ("cv::optflow::calcOpticalFlowDenseRLOF", vec![(pred!(mut, ["I0", "I1", "flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_optflow_calcOpticalFlowDenseRLOF_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(const cv::_InputArray* I0, const cv::_InputArray* I1, const cv::_InputOutputArray* flow, ResultVoid* ocvrs_return) {
		try {
			cv::optflow::calcOpticalFlowDenseRLOF(*I0, *I1, *flow);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calcOpticalFlowDenseRLOF(InputArray, InputArray, InputOutputArray, Ptr<RLOFOpticalFlowParameter>, float, Size, InterpolationType, int, float, float, int, int, bool, float, float, bool)(InputArray, InputArray, InputOutputArray, CppPassByVoidPtr, Primitive, SimpleClass, Enum, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:501
	// ("cv::optflow::calcOpticalFlowDenseRLOF", vec![(pred!(mut, ["I0", "I1", "flow", "rlofParam", "forwardBackwardThreshold", "gridStep", "interp_type", "epicK", "epicSigma", "epicLambda", "ricSPSize", "ricSLICType", "use_post_proc", "fgsLambda", "fgsSigma", "use_variational_refinement"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>", "float", "cv::Size", "cv::optflow::InterpolationType", "int", "float", "float", "int", "int", "bool", "float", "float", "bool"]), _)]),
	void cv_optflow_calcOpticalFlowDenseRLOF_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_PtrLRLOFOpticalFlowParameterG_float_Size_InterpolationType_int_float_float_int_int_bool_float_float_bool(const cv::_InputArray* I0, const cv::_InputArray* I1, const cv::_InputOutputArray* flow, cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>* rlofParam, float forwardBackwardThreshold, cv::Size* gridStep, cv::optflow::InterpolationType interp_type, int epicK, float epicSigma, float epicLambda, int ricSPSize, int ricSLICType, bool use_post_proc, float fgsLambda, float fgsSigma, bool use_variational_refinement, ResultVoid* ocvrs_return) {
		try {
			cv::optflow::calcOpticalFlowDenseRLOF(*I0, *I1, *flow, *rlofParam, forwardBackwardThreshold, *gridStep, interp_type, epicK, epicSigma, epicLambda, ricSPSize, ricSLICType, use_post_proc, fgsLambda, fgsSigma, use_variational_refinement);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calcOpticalFlowSF(InputArray, InputArray, OutputArray, int, int, int)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:81
	// ("cv::optflow::calcOpticalFlowSF", vec![(pred!(mut, ["from", "to", "flow", "layers", "averaging_block_size", "max_flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
	void cv_optflow_calcOpticalFlowSF_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* from, const cv::_InputArray* to, const cv::_OutputArray* flow, int layers, int averaging_block_size, int max_flow, ResultVoid* ocvrs_return) {
		try {
			cv::optflow::calcOpticalFlowSF(*from, *to, *flow, layers, averaging_block_size, max_flow);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calcOpticalFlowSF(InputArray, InputArray, OutputArray, int, int, int, double, double, int, double, double, double, int, double, double, double)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:110
	// ("cv::optflow::calcOpticalFlowSF", vec![(pred!(mut, ["from", "to", "flow", "layers", "averaging_block_size", "max_flow", "sigma_dist", "sigma_color", "postprocess_window", "sigma_dist_fix", "sigma_color_fix", "occ_thr", "upscale_averaging_radius", "upscale_sigma_dist", "upscale_sigma_color", "speed_up_thr"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int", "double", "double", "int", "double", "double", "double", "int", "double", "double", "double"]), _)]),
	void cv_optflow_calcOpticalFlowSF_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_int_double_double_int_double_double_double_int_double_double_double(const cv::_InputArray* from, const cv::_InputArray* to, const cv::_OutputArray* flow, int layers, int averaging_block_size, int max_flow, double sigma_dist, double sigma_color, int postprocess_window, double sigma_dist_fix, double sigma_color_fix, double occ_thr, int upscale_averaging_radius, double upscale_sigma_dist, double upscale_sigma_color, double speed_up_thr, ResultVoid* ocvrs_return) {
		try {
			cv::optflow::calcOpticalFlowSF(*from, *to, *flow, layers, averaging_block_size, max_flow, sigma_dist, sigma_color, postprocess_window, sigma_dist_fix, sigma_color_fix, occ_thr, upscale_averaging_radius, upscale_sigma_dist, upscale_sigma_color, speed_up_thr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::optflow::calcOpticalFlowSparseRLOF(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:538
	// ("cv::optflow::calcOpticalFlowSparseRLOF", vec![(pred!(mut, ["prevImg", "nextImg", "prevPts", "nextPts", "status", "err"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_optflow_calcOpticalFlowSparseRLOF_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* prevImg, const cv::_InputArray* nextImg, const cv::_InputArray* prevPts, const cv::_InputOutputArray* nextPts, const cv::_OutputArray* status, const cv::_OutputArray* err, ResultVoid* ocvrs_return) {
		try {
			cv::optflow::calcOpticalFlowSparseRLOF(*prevImg, *nextImg, *prevPts, *nextPts, *status, *err);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calcOpticalFlowSparseRLOF(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray, Ptr<RLOFOpticalFlowParameter>, float)(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:538
	// ("cv::optflow::calcOpticalFlowSparseRLOF", vec![(pred!(mut, ["prevImg", "nextImg", "prevPts", "nextPts", "status", "err", "rlofParam", "forwardBackwardThreshold"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>", "float"]), _)]),
	void cv_optflow_calcOpticalFlowSparseRLOF_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_PtrLRLOFOpticalFlowParameterG_float(const cv::_InputArray* prevImg, const cv::_InputArray* nextImg, const cv::_InputArray* prevPts, const cv::_InputOutputArray* nextPts, const cv::_OutputArray* status, const cv::_OutputArray* err, cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>* rlofParam, float forwardBackwardThreshold, ResultVoid* ocvrs_return) {
		try {
			cv::optflow::calcOpticalFlowSparseRLOF(*prevImg, *nextImg, *prevPts, *nextPts, *status, *err, *rlofParam, forwardBackwardThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::optflow::calcOpticalFlowSparseToDense(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:135
	// ("cv::optflow::calcOpticalFlowSparseToDense", vec![(pred!(mut, ["from", "to", "flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_optflow_calcOpticalFlowSparseToDense_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* from, const cv::_InputArray* to, const cv::_OutputArray* flow, ResultVoid* ocvrs_return) {
		try {
			cv::optflow::calcOpticalFlowSparseToDense(*from, *to, *flow);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calcOpticalFlowSparseToDense(InputArray, InputArray, OutputArray, int, int, float, bool, float, float)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:135
	// ("cv::optflow::calcOpticalFlowSparseToDense", vec![(pred!(mut, ["from", "to", "flow", "grid_step", "k", "sigma", "use_post_proc", "fgs_lambda", "fgs_sigma"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "float", "bool", "float", "float"]), _)]),
	void cv_optflow_calcOpticalFlowSparseToDense_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_float_bool_float_float(const cv::_InputArray* from, const cv::_InputArray* to, const cv::_OutputArray* flow, int grid_step, int k, float sigma, bool use_post_proc, float fgs_lambda, float fgs_sigma, ResultVoid* ocvrs_return) {
		try {
			cv::optflow::calcOpticalFlowSparseToDense(*from, *to, *flow, grid_step, k, sigma, use_post_proc, fgs_lambda, fgs_sigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createOptFlow_DeepFlow()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:165
	// ("cv::optflow::createOptFlow_DeepFlow", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_createOptFlow_DeepFlow(Result<cv::Ptr<cv::DenseOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DenseOpticalFlow> ret = cv::optflow::createOptFlow_DeepFlow();
			Ok(new cv::Ptr<cv::DenseOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createOptFlow_DenseRLOF()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:545
	// ("cv::optflow::createOptFlow_DenseRLOF", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_createOptFlow_DenseRLOF(Result<cv::Ptr<cv::DenseOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DenseOpticalFlow> ret = cv::optflow::createOptFlow_DenseRLOF();
			Ok(new cv::Ptr<cv::DenseOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createOptFlow_DualTVL1()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:300
	// ("cv::optflow::createOptFlow_DualTVL1", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_createOptFlow_DualTVL1(Result<cv::Ptr<cv::optflow::DualTVL1OpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::optflow::DualTVL1OpticalFlow> ret = cv::optflow::createOptFlow_DualTVL1();
			Ok(new cv::Ptr<cv::optflow::DualTVL1OpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createOptFlow_Farneback()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:171
	// ("cv::optflow::createOptFlow_Farneback", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_createOptFlow_Farneback(Result<cv::Ptr<cv::DenseOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DenseOpticalFlow> ret = cv::optflow::createOptFlow_Farneback();
			Ok(new cv::Ptr<cv::DenseOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createOptFlow_PCAFlow()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/pcaflow.hpp:142
	// ("cv::optflow::createOptFlow_PCAFlow", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_createOptFlow_PCAFlow(Result<cv::Ptr<cv::DenseOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DenseOpticalFlow> ret = cv::optflow::createOptFlow_PCAFlow();
			Ok(new cv::Ptr<cv::DenseOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createOptFlow_SimpleFlow()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:168
	// ("cv::optflow::createOptFlow_SimpleFlow", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_createOptFlow_SimpleFlow(Result<cv::Ptr<cv::DenseOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DenseOpticalFlow> ret = cv::optflow::createOptFlow_SimpleFlow();
			Ok(new cv::Ptr<cv::DenseOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createOptFlow_SparseRLOF()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:548
	// ("cv::optflow::createOptFlow_SparseRLOF", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_createOptFlow_SparseRLOF(Result<cv::Ptr<cv::SparseOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::SparseOpticalFlow> ret = cv::optflow::createOptFlow_SparseRLOF();
			Ok(new cv::Ptr<cv::SparseOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createOptFlow_SparseToDense()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:174
	// ("cv::optflow::createOptFlow_SparseToDense", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_createOptFlow_SparseToDense(Result<cv::Ptr<cv::DenseOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DenseOpticalFlow> ret = cv::optflow::createOptFlow_SparseToDense();
			Ok(new cv::Ptr<cv::DenseOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &, optflow::GPCTree::Node &, optflow::GPCTree::Node)(TraitClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:369
	// ("cv::read", vec![(pred!(mut, ["fn", "node", "unnamed"], ["const cv::FileNode*", "cv::optflow::GPCTree::Node*", "cv::optflow::GPCTree::Node"]), _)]),
	void cv_read_const_FileNodeR_NodeR_Node(const cv::FileNode* fn, cv::optflow::GPCTree::Node* node, cv::optflow::GPCTree::Node* unnamed, ResultVoid* ocvrs_return) {
		try {
			cv::read(*fn, *node, *unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &, const String &, const optflow::GPCTree::Node &)(TraitClass, InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:367
	// ("cv::write", vec![(pred!(mut, ["fs", "name", "node"], ["cv::FileStorage*", "const cv::String*", "const cv::optflow::GPCTree::Node*"]), _)]),
	void cv_write_FileStorageR_const_StringR_const_NodeR(cv::FileStorage* fs, const char* name, const cv::optflow::GPCTree::Node* node, ResultVoid* ocvrs_return) {
		try {
			cv::write(*fs, std::string(name), *node);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRLOFOpticalFlowParameter(Ptr<RLOFOpticalFlowParameter>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:240
	// ("cv::optflow::DenseRLOFOpticalFlow::setRLOFOpticalFlowParameter", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>"]), _)]),
	void cv_optflow_DenseRLOFOpticalFlow_setRLOFOpticalFlowParameter_PtrLRLOFOpticalFlowParameterG(cv::optflow::DenseRLOFOpticalFlow* instance, cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>* val, ResultVoid* ocvrs_return) {
		try {
			instance->setRLOFOpticalFlowParameter(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRLOFOpticalFlowParameter()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:244
	// ("cv::optflow::DenseRLOFOpticalFlow::getRLOFOpticalFlowParameter", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DenseRLOFOpticalFlow_getRLOFOpticalFlowParameter_const(const cv::optflow::DenseRLOFOpticalFlow* instance, Result<cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::optflow::RLOFOpticalFlowParameter> ret = instance->getRLOFOpticalFlowParameter();
			Ok(new cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setForwardBackward(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:253
	// ("cv::optflow::DenseRLOFOpticalFlow::setForwardBackward", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_optflow_DenseRLOFOpticalFlow_setForwardBackward_float(cv::optflow::DenseRLOFOpticalFlow* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setForwardBackward(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getForwardBackward()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:257
	// ("cv::optflow::DenseRLOFOpticalFlow::getForwardBackward", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DenseRLOFOpticalFlow_getForwardBackward_const(const cv::optflow::DenseRLOFOpticalFlow* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getForwardBackward();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGridStep()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:263
	// ("cv::optflow::DenseRLOFOpticalFlow::getGridStep", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DenseRLOFOpticalFlow_getGridStep_const(const cv::optflow::DenseRLOFOpticalFlow* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getGridStep();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setGridStep(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:267
	// ("cv::optflow::DenseRLOFOpticalFlow::setGridStep", vec![(pred!(mut, ["val"], ["cv::Size"]), _)]),
	void cv_optflow_DenseRLOFOpticalFlow_setGridStep_Size(cv::optflow::DenseRLOFOpticalFlow* instance, cv::Size* val, ResultVoid* ocvrs_return) {
		try {
			instance->setGridStep(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInterpolation(InterpolationType)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:275
	// ("cv::optflow::DenseRLOFOpticalFlow::setInterpolation", vec![(pred!(mut, ["val"], ["cv::optflow::InterpolationType"]), _)]),
	void cv_optflow_DenseRLOFOpticalFlow_setInterpolation_InterpolationType(cv::optflow::DenseRLOFOpticalFlow* instance, cv::optflow::InterpolationType val, ResultVoid* ocvrs_return) {
		try {
			instance->setInterpolation(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getInterpolation()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:279
	// ("cv::optflow::DenseRLOFOpticalFlow::getInterpolation", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DenseRLOFOpticalFlow_getInterpolation_const(const cv::optflow::DenseRLOFOpticalFlow* instance, Result<cv::optflow::InterpolationType>* ocvrs_return) {
		try {
			cv::optflow::InterpolationType ret = instance->getInterpolation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEPICK()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:285
	// ("cv::optflow::DenseRLOFOpticalFlow::getEPICK", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DenseRLOFOpticalFlow_getEPICK_const(const cv::optflow::DenseRLOFOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getEPICK();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEPICK(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:289
	// ("cv::optflow::DenseRLOFOpticalFlow::setEPICK", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_optflow_DenseRLOFOpticalFlow_setEPICK_int(cv::optflow::DenseRLOFOpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setEPICK(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEPICSigma()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:296
	// ("cv::optflow::DenseRLOFOpticalFlow::getEPICSigma", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DenseRLOFOpticalFlow_getEPICSigma_const(const cv::optflow::DenseRLOFOpticalFlow* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getEPICSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEPICSigma(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:300
	// ("cv::optflow::DenseRLOFOpticalFlow::setEPICSigma", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_optflow_DenseRLOFOpticalFlow_setEPICSigma_float(cv::optflow::DenseRLOFOpticalFlow* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setEPICSigma(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEPICLambda()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:306
	// ("cv::optflow::DenseRLOFOpticalFlow::getEPICLambda", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DenseRLOFOpticalFlow_getEPICLambda_const(const cv::optflow::DenseRLOFOpticalFlow* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getEPICLambda();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEPICLambda(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:310
	// ("cv::optflow::DenseRLOFOpticalFlow::setEPICLambda", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_optflow_DenseRLOFOpticalFlow_setEPICLambda_float(cv::optflow::DenseRLOFOpticalFlow* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setEPICLambda(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFgsLambda()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:315
	// ("cv::optflow::DenseRLOFOpticalFlow::getFgsLambda", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DenseRLOFOpticalFlow_getFgsLambda_const(const cv::optflow::DenseRLOFOpticalFlow* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getFgsLambda();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFgsLambda(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:319
	// ("cv::optflow::DenseRLOFOpticalFlow::setFgsLambda", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_optflow_DenseRLOFOpticalFlow_setFgsLambda_float(cv::optflow::DenseRLOFOpticalFlow* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setFgsLambda(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFgsSigma()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:324
	// ("cv::optflow::DenseRLOFOpticalFlow::getFgsSigma", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DenseRLOFOpticalFlow_getFgsSigma_const(const cv::optflow::DenseRLOFOpticalFlow* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getFgsSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFgsSigma(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:328
	// ("cv::optflow::DenseRLOFOpticalFlow::setFgsSigma", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_optflow_DenseRLOFOpticalFlow_setFgsSigma_float(cv::optflow::DenseRLOFOpticalFlow* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setFgsSigma(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUsePostProc(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:333
	// ("cv::optflow::DenseRLOFOpticalFlow::setUsePostProc", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	void cv_optflow_DenseRLOFOpticalFlow_setUsePostProc_bool(cv::optflow::DenseRLOFOpticalFlow* instance, bool val, ResultVoid* ocvrs_return) {
		try {
			instance->setUsePostProc(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUsePostProc()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:337
	// ("cv::optflow::DenseRLOFOpticalFlow::getUsePostProc", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DenseRLOFOpticalFlow_getUsePostProc_const(const cv::optflow::DenseRLOFOpticalFlow* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUsePostProc();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseVariationalRefinement(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:342
	// ("cv::optflow::DenseRLOFOpticalFlow::setUseVariationalRefinement", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	void cv_optflow_DenseRLOFOpticalFlow_setUseVariationalRefinement_bool(cv::optflow::DenseRLOFOpticalFlow* instance, bool val, ResultVoid* ocvrs_return) {
		try {
			instance->setUseVariationalRefinement(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUseVariationalRefinement()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:346
	// ("cv::optflow::DenseRLOFOpticalFlow::getUseVariationalRefinement", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DenseRLOFOpticalFlow_getUseVariationalRefinement_const(const cv::optflow::DenseRLOFOpticalFlow* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseVariationalRefinement();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRICSPSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:351
	// ("cv::optflow::DenseRLOFOpticalFlow::setRICSPSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_optflow_DenseRLOFOpticalFlow_setRICSPSize_int(cv::optflow::DenseRLOFOpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setRICSPSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRICSPSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:355
	// ("cv::optflow::DenseRLOFOpticalFlow::getRICSPSize", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DenseRLOFOpticalFlow_getRICSPSize_const(const cv::optflow::DenseRLOFOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getRICSPSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRICSLICType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:362
	// ("cv::optflow::DenseRLOFOpticalFlow::setRICSLICType", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_optflow_DenseRLOFOpticalFlow_setRICSLICType_int(cv::optflow::DenseRLOFOpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setRICSLICType(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRICSLICType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:366
	// ("cv::optflow::DenseRLOFOpticalFlow::getRICSLICType", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DenseRLOFOpticalFlow_getRICSLICType_const(const cv::optflow::DenseRLOFOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getRICSLICType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(Ptr<RLOFOpticalFlowParameter>, float, Size, InterpolationType, int, float, float, int, int, bool, float, float, bool)(CppPassByVoidPtr, Primitive, SimpleClass, Enum, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:383
	// ("cv::optflow::DenseRLOFOpticalFlow::create", vec![(pred!(mut, ["rlofParam", "forwardBackwardThreshold", "gridStep", "interp_type", "epicK", "epicSigma", "epicLambda", "ricSPSize", "ricSLICType", "use_post_proc", "fgsLambda", "fgsSigma", "use_variational_refinement"], ["cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>", "float", "cv::Size", "cv::optflow::InterpolationType", "int", "float", "float", "int", "int", "bool", "float", "float", "bool"]), _)]),
	void cv_optflow_DenseRLOFOpticalFlow_create_PtrLRLOFOpticalFlowParameterG_float_Size_InterpolationType_int_float_float_int_int_bool_float_float_bool(cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>* rlofParam, float forwardBackwardThreshold, cv::Size* gridStep, cv::optflow::InterpolationType interp_type, int epicK, float epicSigma, float epicLambda, int ricSPSize, int ricSLICType, bool use_post_proc, float fgsLambda, float fgsSigma, bool use_variational_refinement, Result<cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::optflow::DenseRLOFOpticalFlow> ret = cv::optflow::DenseRLOFOpticalFlow::create(*rlofParam, forwardBackwardThreshold, *gridStep, interp_type, epicK, epicSigma, epicLambda, ricSPSize, ricSLICType, use_post_proc, fgsLambda, fgsSigma, use_variational_refinement);
			Ok(new cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::optflow::DenseRLOFOpticalFlow::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:383
	// ("cv::optflow::DenseRLOFOpticalFlow::create", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_DenseRLOFOpticalFlow_create(Result<cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::optflow::DenseRLOFOpticalFlow> ret = cv::optflow::DenseRLOFOpticalFlow::create();
			Ok(new cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::optflow::DenseRLOFOpticalFlow::to_Algorithm() generated
	// ("cv::optflow::DenseRLOFOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_optflow_DenseRLOFOpticalFlow_to_Algorithm(cv::optflow::DenseRLOFOpticalFlow* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::optflow::DenseRLOFOpticalFlow::to_DenseOpticalFlow() generated
	// ("cv::optflow::DenseRLOFOpticalFlow::to_DenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::DenseOpticalFlow* cv_optflow_DenseRLOFOpticalFlow_to_DenseOpticalFlow(cv::optflow::DenseRLOFOpticalFlow* instance) {
			return dynamic_cast<cv::DenseOpticalFlow*>(instance);
	}

	// cv::optflow::DenseRLOFOpticalFlow::delete() generated
	// ("cv::optflow::DenseRLOFOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_DenseRLOFOpticalFlow_delete(cv::optflow::DenseRLOFOpticalFlow* instance) {
			delete instance;
	}

	// getTau()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:223
	// ("cv::optflow::DualTVL1OpticalFlow::getTau", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DualTVL1OpticalFlow_getTau_const(const cv::optflow::DualTVL1OpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getTau();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTau(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:225
	// ("cv::optflow::DualTVL1OpticalFlow::setTau", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_optflow_DualTVL1OpticalFlow_setTau_double(cv::optflow::DualTVL1OpticalFlow* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setTau(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLambda()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:228
	// ("cv::optflow::DualTVL1OpticalFlow::getLambda", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DualTVL1OpticalFlow_getLambda_const(const cv::optflow::DualTVL1OpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getLambda();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setLambda(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:230
	// ("cv::optflow::DualTVL1OpticalFlow::setLambda", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_optflow_DualTVL1OpticalFlow_setLambda_double(cv::optflow::DualTVL1OpticalFlow* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setLambda(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTheta()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:233
	// ("cv::optflow::DualTVL1OpticalFlow::getTheta", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DualTVL1OpticalFlow_getTheta_const(const cv::optflow::DualTVL1OpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getTheta();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTheta(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:235
	// ("cv::optflow::DualTVL1OpticalFlow::setTheta", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_optflow_DualTVL1OpticalFlow_setTheta_double(cv::optflow::DualTVL1OpticalFlow* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setTheta(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGamma()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:238
	// ("cv::optflow::DualTVL1OpticalFlow::getGamma", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DualTVL1OpticalFlow_getGamma_const(const cv::optflow::DualTVL1OpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getGamma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setGamma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:240
	// ("cv::optflow::DualTVL1OpticalFlow::setGamma", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_optflow_DualTVL1OpticalFlow_setGamma_double(cv::optflow::DualTVL1OpticalFlow* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setGamma(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScalesNumber()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:243
	// ("cv::optflow::DualTVL1OpticalFlow::getScalesNumber", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DualTVL1OpticalFlow_getScalesNumber_const(const cv::optflow::DualTVL1OpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getScalesNumber();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScalesNumber(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:245
	// ("cv::optflow::DualTVL1OpticalFlow::setScalesNumber", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_optflow_DualTVL1OpticalFlow_setScalesNumber_int(cv::optflow::DualTVL1OpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setScalesNumber(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWarpingsNumber()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:248
	// ("cv::optflow::DualTVL1OpticalFlow::getWarpingsNumber", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DualTVL1OpticalFlow_getWarpingsNumber_const(const cv::optflow::DualTVL1OpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWarpingsNumber();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWarpingsNumber(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:250
	// ("cv::optflow::DualTVL1OpticalFlow::setWarpingsNumber", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_optflow_DualTVL1OpticalFlow_setWarpingsNumber_int(cv::optflow::DualTVL1OpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setWarpingsNumber(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEpsilon()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:253
	// ("cv::optflow::DualTVL1OpticalFlow::getEpsilon", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DualTVL1OpticalFlow_getEpsilon_const(const cv::optflow::DualTVL1OpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getEpsilon();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEpsilon(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:255
	// ("cv::optflow::DualTVL1OpticalFlow::setEpsilon", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_optflow_DualTVL1OpticalFlow_setEpsilon_double(cv::optflow::DualTVL1OpticalFlow* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setEpsilon(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getInnerIterations()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:258
	// ("cv::optflow::DualTVL1OpticalFlow::getInnerIterations", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DualTVL1OpticalFlow_getInnerIterations_const(const cv::optflow::DualTVL1OpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getInnerIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInnerIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:260
	// ("cv::optflow::DualTVL1OpticalFlow::setInnerIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_optflow_DualTVL1OpticalFlow_setInnerIterations_int(cv::optflow::DualTVL1OpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setInnerIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOuterIterations()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:263
	// ("cv::optflow::DualTVL1OpticalFlow::getOuterIterations", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DualTVL1OpticalFlow_getOuterIterations_const(const cv::optflow::DualTVL1OpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getOuterIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setOuterIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:265
	// ("cv::optflow::DualTVL1OpticalFlow::setOuterIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_optflow_DualTVL1OpticalFlow_setOuterIterations_int(cv::optflow::DualTVL1OpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setOuterIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUseInitialFlow()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:268
	// ("cv::optflow::DualTVL1OpticalFlow::getUseInitialFlow", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DualTVL1OpticalFlow_getUseInitialFlow_const(const cv::optflow::DualTVL1OpticalFlow* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseInitialFlow();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseInitialFlow(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:270
	// ("cv::optflow::DualTVL1OpticalFlow::setUseInitialFlow", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	void cv_optflow_DualTVL1OpticalFlow_setUseInitialFlow_bool(cv::optflow::DualTVL1OpticalFlow* instance, bool val, ResultVoid* ocvrs_return) {
		try {
			instance->setUseInitialFlow(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScaleStep()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:273
	// ("cv::optflow::DualTVL1OpticalFlow::getScaleStep", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DualTVL1OpticalFlow_getScaleStep_const(const cv::optflow::DualTVL1OpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getScaleStep();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScaleStep(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:275
	// ("cv::optflow::DualTVL1OpticalFlow::setScaleStep", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_optflow_DualTVL1OpticalFlow_setScaleStep_double(cv::optflow::DualTVL1OpticalFlow* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setScaleStep(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMedianFiltering()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:278
	// ("cv::optflow::DualTVL1OpticalFlow::getMedianFiltering", vec![(pred!(const, [], []), _)]),
	void cv_optflow_DualTVL1OpticalFlow_getMedianFiltering_const(const cv::optflow::DualTVL1OpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMedianFiltering();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMedianFiltering(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:280
	// ("cv::optflow::DualTVL1OpticalFlow::setMedianFiltering", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_optflow_DualTVL1OpticalFlow_setMedianFiltering_int(cv::optflow::DualTVL1OpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setMedianFiltering(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(double, double, double, int, int, double, int, int, double, double, int, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:283
	// ("cv::optflow::DualTVL1OpticalFlow::create", vec![(pred!(mut, ["tau", "lambda", "theta", "nscales", "warps", "epsilon", "innnerIterations", "outerIterations", "scaleStep", "gamma", "medianFiltering", "useInitialFlow"], ["double", "double", "double", "int", "int", "double", "int", "int", "double", "double", "int", "bool"]), _)]),
	void cv_optflow_DualTVL1OpticalFlow_create_double_double_double_int_int_double_int_int_double_double_int_bool(double tau, double lambda, double theta, int nscales, int warps, double epsilon, int innnerIterations, int outerIterations, double scaleStep, double gamma, int medianFiltering, bool useInitialFlow, Result<cv::Ptr<cv::optflow::DualTVL1OpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::optflow::DualTVL1OpticalFlow> ret = cv::optflow::DualTVL1OpticalFlow::create(tau, lambda, theta, nscales, warps, epsilon, innnerIterations, outerIterations, scaleStep, gamma, medianFiltering, useInitialFlow);
			Ok(new cv::Ptr<cv::optflow::DualTVL1OpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::optflow::DualTVL1OpticalFlow::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow.hpp:283
	// ("cv::optflow::DualTVL1OpticalFlow::create", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_DualTVL1OpticalFlow_create(Result<cv::Ptr<cv::optflow::DualTVL1OpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::optflow::DualTVL1OpticalFlow> ret = cv::optflow::DualTVL1OpticalFlow::create();
			Ok(new cv::Ptr<cv::optflow::DualTVL1OpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::optflow::DualTVL1OpticalFlow::to_Algorithm() generated
	// ("cv::optflow::DualTVL1OpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_optflow_DualTVL1OpticalFlow_to_Algorithm(cv::optflow::DualTVL1OpticalFlow* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::optflow::DualTVL1OpticalFlow::to_DenseOpticalFlow() generated
	// ("cv::optflow::DualTVL1OpticalFlow::to_DenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::DenseOpticalFlow* cv_optflow_DualTVL1OpticalFlow_to_DenseOpticalFlow(cv::optflow::DualTVL1OpticalFlow* instance) {
			return dynamic_cast<cv::DenseOpticalFlow*>(instance);
	}

	// cv::optflow::DualTVL1OpticalFlow::delete() generated
	// ("cv::optflow::DualTVL1OpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_DualTVL1OpticalFlow_delete(cv::optflow::DualTVL1OpticalFlow* instance) {
			delete instance;
	}

	// dropOutliers(std::vector<std::pair<Point2i, Point2i>> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:304
	// ("cv::optflow::GPCDetails::dropOutliers", vec![(pred!(mut, ["corr"], ["std::vector<std::pair<cv::Point2i, cv::Point2i>>*"]), _)]),
	void cv_optflow_GPCDetails_dropOutliers_vectorLpairLcv_Point2i__cv_Point2iGGR(std::vector<std::pair<cv::Point2i, cv::Point2i>>* corr, ResultVoid* ocvrs_return) {
		try {
			cv::optflow::GPCDetails::dropOutliers(*corr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAllDescriptorsForImage(const Mat *, std::vector<GPCPatchDescriptor> &, const GPCMatchingParams &, int)(TraitClass, CppPassByVoidPtr, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:306
	// ("cv::optflow::GPCDetails::getAllDescriptorsForImage", vec![(pred!(mut, ["imgCh", "descr", "mp", "type"], ["const cv::Mat*", "std::vector<cv::optflow::GPCPatchDescriptor>*", "const cv::optflow::GPCMatchingParams*", "int"]), _)]),
	void cv_optflow_GPCDetails_getAllDescriptorsForImage_const_MatX_vectorLGPCPatchDescriptorGR_const_GPCMatchingParamsR_int(const cv::Mat* imgCh, std::vector<cv::optflow::GPCPatchDescriptor>* descr, const cv::optflow::GPCMatchingParams* mp, int type, ResultVoid* ocvrs_return) {
		try {
			cv::optflow::GPCDetails::getAllDescriptorsForImage(imgCh, *descr, *mp, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCoordinatesFromIndex(size_t, Size, int &, int &)(Primitive, SimpleClass, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:309
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

	// GPCMatchingParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:147
	// ("cv::optflow::GPCMatchingParams::GPCMatchingParams", vec![(pred!(mut, ["_useOpenCL"], ["bool"]), _)]),
	void cv_optflow_GPCMatchingParams_GPCMatchingParams_bool(bool _useOpenCL, Result<cv::optflow::GPCMatchingParams>* ocvrs_return) {
		try {
			cv::optflow::GPCMatchingParams ret(_useOpenCL);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::optflow::GPCMatchingParams::GPCMatchingParams() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:147
	// ("cv::optflow::GPCMatchingParams::GPCMatchingParams", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_GPCMatchingParams_GPCMatchingParams(Result<cv::optflow::GPCMatchingParams>* ocvrs_return) {
		try {
			cv::optflow::GPCMatchingParams ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GPCMatchingParams(const GPCMatchingParams &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:149
	// ("cv::optflow::GPCMatchingParams::GPCMatchingParams", vec![(pred!(mut, ["params"], ["const cv::optflow::GPCMatchingParams*"]), _)]),
	void cv_optflow_GPCMatchingParams_GPCMatchingParams_const_GPCMatchingParamsR(const cv::optflow::GPCMatchingParams* params, Result<cv::optflow::GPCMatchingParams>* ocvrs_return) {
		try {
			cv::optflow::GPCMatchingParams ret(*params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dot(const Vec<double, nFeatures> &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:70
	// ("cv::optflow::GPCPatchDescriptor::dot", vec![(pred!(const, ["coef"], ["const cv::Vec<double, 18>*"]), _)]),
	void cv_optflow_GPCPatchDescriptor_dot_const_const_VecLdouble__18GR(const cv::optflow::GPCPatchDescriptor* instance, const cv::Vec<double, 18>* coef, Result<double>* ocvrs_return) {
		try {
			double ret = instance->dot(*coef);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// markAsSeparated()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:72
	// ("cv::optflow::GPCPatchDescriptor::markAsSeparated", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_GPCPatchDescriptor_markAsSeparated(cv::optflow::GPCPatchDescriptor* instance, ResultVoid* ocvrs_return) {
		try {
			instance->markAsSeparated();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isSeparated()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:74
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

	// cv::optflow::GPCPatchDescriptor::feature() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:68
	// ("cv::optflow::GPCPatchDescriptor::feature", vec![(pred!(const, [], []), _)]),
	void cv_optflow_GPCPatchDescriptor_propFeature_const(const cv::optflow::GPCPatchDescriptor* instance, cv::Vec<double, 18>* ocvrs_return) {
			cv::Vec<double, 18> ret = instance->feature;
			*ocvrs_return = ret;
	}

	// cv::optflow::GPCPatchDescriptor::setFeature(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:68
	// ("cv::optflow::GPCPatchDescriptor::setFeature", vec![(pred!(mut, ["val"], ["const cv::Vec<double, 18>"]), _)]),
	void cv_optflow_GPCPatchDescriptor_propFeature_const_VecLdouble__18G(cv::optflow::GPCPatchDescriptor* instance, const cv::Vec<double, 18>* val) {
			instance->feature = *val;
	}

	// cv::optflow::GPCPatchDescriptor::delete() generated
	// ("cv::optflow::GPCPatchDescriptor::delete", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_GPCPatchDescriptor_delete(cv::optflow::GPCPatchDescriptor* instance) {
			delete instance;
	}

	// getDirections(bool &, bool &, bool &, const Vec<double, GPCPatchDescriptor::nFeatures> &, double)(Indirect, Indirect, Indirect, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:83
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

	// cv::optflow::GPCPatchSample::ref() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:79
	// ("cv::optflow::GPCPatchSample::ref", vec![(pred!(const, [], []), _)]),
	cv::optflow::GPCPatchDescriptor* cv_optflow_GPCPatchSample_propRef_const(const cv::optflow::GPCPatchSample* instance) {
			cv::optflow::GPCPatchDescriptor ret = instance->ref;
			return new cv::optflow::GPCPatchDescriptor(ret);
	}

	// cv::optflow::GPCPatchSample::setRef(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:79
	// ("cv::optflow::GPCPatchSample::setRef", vec![(pred!(mut, ["val"], ["const cv::optflow::GPCPatchDescriptor"]), _)]),
	void cv_optflow_GPCPatchSample_propRef_const_GPCPatchDescriptor(cv::optflow::GPCPatchSample* instance, const cv::optflow::GPCPatchDescriptor* val) {
			instance->ref = *val;
	}

	// cv::optflow::GPCPatchSample::pos() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:80
	// ("cv::optflow::GPCPatchSample::pos", vec![(pred!(const, [], []), _)]),
	cv::optflow::GPCPatchDescriptor* cv_optflow_GPCPatchSample_propPos_const(const cv::optflow::GPCPatchSample* instance) {
			cv::optflow::GPCPatchDescriptor ret = instance->pos;
			return new cv::optflow::GPCPatchDescriptor(ret);
	}

	// cv::optflow::GPCPatchSample::setPos(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:80
	// ("cv::optflow::GPCPatchSample::setPos", vec![(pred!(mut, ["val"], ["const cv::optflow::GPCPatchDescriptor"]), _)]),
	void cv_optflow_GPCPatchSample_propPos_const_GPCPatchDescriptor(cv::optflow::GPCPatchSample* instance, const cv::optflow::GPCPatchDescriptor* val) {
			instance->pos = *val;
	}

	// cv::optflow::GPCPatchSample::neg() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:81
	// ("cv::optflow::GPCPatchSample::neg", vec![(pred!(const, [], []), _)]),
	cv::optflow::GPCPatchDescriptor* cv_optflow_GPCPatchSample_propNeg_const(const cv::optflow::GPCPatchSample* instance) {
			cv::optflow::GPCPatchDescriptor ret = instance->neg;
			return new cv::optflow::GPCPatchDescriptor(ret);
	}

	// cv::optflow::GPCPatchSample::setNeg(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:81
	// ("cv::optflow::GPCPatchSample::setNeg", vec![(pred!(mut, ["val"], ["const cv::optflow::GPCPatchDescriptor"]), _)]),
	void cv_optflow_GPCPatchSample_propNeg_const_GPCPatchDescriptor(cv::optflow::GPCPatchSample* instance, const cv::optflow::GPCPatchDescriptor* val) {
			instance->neg = *val;
	}

	// cv::optflow::GPCPatchSample::delete() generated
	// ("cv::optflow::GPCPatchSample::delete", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_GPCPatchSample_delete(cv::optflow::GPCPatchSample* instance) {
			delete instance;
	}

	// GPCTrainingParams(unsigned int, int, GPCDescType, bool)(Primitive, Primitive, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:130
	// ("cv::optflow::GPCTrainingParams::GPCTrainingParams", vec![(pred!(mut, ["_maxTreeDepth", "_minNumberOfSamples", "_descriptorType", "_printProgress"], ["unsigned int", "int", "cv::optflow::GPCDescType", "bool"]), _)]),
	void cv_optflow_GPCTrainingParams_GPCTrainingParams_unsigned_int_int_GPCDescType_bool(unsigned int _maxTreeDepth, int _minNumberOfSamples, cv::optflow::GPCDescType _descriptorType, bool _printProgress, Result<cv::optflow::GPCTrainingParams>* ocvrs_return) {
		try {
			cv::optflow::GPCTrainingParams ret(_maxTreeDepth, _minNumberOfSamples, _descriptorType, _printProgress);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::optflow::GPCTrainingParams::GPCTrainingParams() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:130
	// ("cv::optflow::GPCTrainingParams::GPCTrainingParams", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_GPCTrainingParams_GPCTrainingParams(Result<cv::optflow::GPCTrainingParams>* ocvrs_return) {
		try {
			cv::optflow::GPCTrainingParams ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// check()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:138
	// ("cv::optflow::GPCTrainingParams::check", vec![(pred!(const, [], []), _)]),
	void cv_optflow_GPCTrainingParams_check_const(const cv::optflow::GPCTrainingParams* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->check();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const std::vector<String> &, const std::vector<String> &, const std::vector<String> &, int)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:108
	// ("cv::optflow::GPCTrainingSamples::create", vec![(pred!(mut, ["imagesFrom", "imagesTo", "gt", "descriptorType"], ["const std::vector<cv::String>*", "const std::vector<cv::String>*", "const std::vector<cv::String>*", "int"]), _)]),
	void cv_optflow_GPCTrainingSamples_create_const_vectorLStringGR_const_vectorLStringGR_const_vectorLStringGR_int(const std::vector<cv::String>* imagesFrom, const std::vector<cv::String>* imagesTo, const std::vector<cv::String>* gt, int descriptorType, Result<cv::Ptr<cv::optflow::GPCTrainingSamples>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::optflow::GPCTrainingSamples> ret = cv::optflow::GPCTrainingSamples::create(*imagesFrom, *imagesTo, *gt, descriptorType);
			Ok(new cv::Ptr<cv::optflow::GPCTrainingSamples>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(InputArrayOfArrays, InputArrayOfArrays, InputArrayOfArrays, int)(InputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:111
	// ("cv::optflow::GPCTrainingSamples::create", vec![(pred!(mut, ["imagesFrom", "imagesTo", "gt", "descriptorType"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
	void cv_optflow_GPCTrainingSamples_create_const__InputArrayR_const__InputArrayR_const__InputArrayR_int(const cv::_InputArray* imagesFrom, const cv::_InputArray* imagesTo, const cv::_InputArray* gt, int descriptorType, Result<cv::Ptr<cv::optflow::GPCTrainingSamples>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::optflow::GPCTrainingSamples> ret = cv::optflow::GPCTrainingSamples::create(*imagesFrom, *imagesTo, *gt, descriptorType);
			Ok(new cv::Ptr<cv::optflow::GPCTrainingSamples>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// size()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:114
	// ("cv::optflow::GPCTrainingSamples::size", vec![(pred!(const, [], []), _)]),
	void cv_optflow_GPCTrainingSamples_size_const(const cv::optflow::GPCTrainingSamples* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->size();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// type()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:116
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

	// train(GPCTrainingSamples &, const GPCTrainingParams)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:176
	// ("cv::optflow::GPCTree::train", vec![(pred!(mut, ["samples", "params"], ["cv::optflow::GPCTrainingSamples*", "const cv::optflow::GPCTrainingParams"]), _)]),
	void cv_optflow_GPCTree_train_GPCTrainingSamplesR_const_GPCTrainingParams(cv::optflow::GPCTree* instance, cv::optflow::GPCTrainingSamples* samples, const cv::optflow::GPCTrainingParams* params, ResultVoid* ocvrs_return) {
		try {
			instance->train(*samples, *params);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::optflow::GPCTree::train(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:176
	// ("cv::optflow::GPCTree::train", vec![(pred!(mut, ["samples"], ["cv::optflow::GPCTrainingSamples*"]), _)]),
	void cv_optflow_GPCTree_train_GPCTrainingSamplesR(cv::optflow::GPCTree* instance, cv::optflow::GPCTrainingSamples* samples, ResultVoid* ocvrs_return) {
		try {
			instance->train(*samples);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:178
	// ("cv::optflow::GPCTree::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_optflow_GPCTree_write_const_FileStorageR(const cv::optflow::GPCTree* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:180
	// ("cv::optflow::GPCTree::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_optflow_GPCTree_read_const_FileNodeR(cv::optflow::GPCTree* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findLeafForPatch(const GPCPatchDescriptor &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:182
	// ("cv::optflow::GPCTree::findLeafForPatch", vec![(pred!(const, ["descr"], ["const cv::optflow::GPCPatchDescriptor*"]), _)]),
	void cv_optflow_GPCTree_findLeafForPatch_const_const_GPCPatchDescriptorR(const cv::optflow::GPCTree* instance, const cv::optflow::GPCPatchDescriptor* descr, Result<unsigned int>* ocvrs_return) {
		try {
			unsigned int ret = instance->findLeafForPatch(*descr);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:184
	// ("cv::optflow::GPCTree::create", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_GPCTree_create(Result<cv::Ptr<cv::optflow::GPCTree>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::optflow::GPCTree> ret = cv::optflow::GPCTree::create();
			Ok(new cv::Ptr<cv::optflow::GPCTree>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator==(const GPCTree &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:186
	// ("cv::optflow::GPCTree::operator==", vec![(pred!(const, ["t"], ["const cv::optflow::GPCTree*"]), _)]),
	void cv_optflow_GPCTree_operatorEQ_const_const_GPCTreeR(const cv::optflow::GPCTree* instance, const cv::optflow::GPCTree* t, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator==(*t);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDescriptorType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:188
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

	// operator==(const Node &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/sparse_matching_gpc.hpp:164
	// ("cv::optflow::GPCTree::Node::operator==", vec![(pred!(const, ["n"], ["const cv::optflow::GPCTree::Node*"]), _)]),
	void cv_optflow_GPCTree_Node_operatorEQ_const_const_NodeR(const cv::optflow::GPCTree::Node* instance, const cv::optflow::GPCTree::Node* n, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator==(*n);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// OpticalFlowPCAFlow(Ptr<const PCAPrior>, const Size, float, float, float, float, float)(CppPassByVoidPtr, SimpleClass, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/pcaflow.hpp:116
	// ("cv::optflow::OpticalFlowPCAFlow::OpticalFlowPCAFlow", vec![(pred!(mut, ["_prior", "_basisSize", "_sparseRate", "_retainedCornersFraction", "_occlusionsThreshold", "_dampingFactor", "_claheClip"], ["cv::Ptr<const cv::optflow::PCAPrior>", "const cv::Size", "float", "float", "float", "float", "float"]), _)]),
	void cv_optflow_OpticalFlowPCAFlow_OpticalFlowPCAFlow_PtrLconst_PCAPriorG_const_Size_float_float_float_float_float(const cv::Ptr<const cv::optflow::PCAPrior>* _prior, const cv::Size* _basisSize, float _sparseRate, float _retainedCornersFraction, float _occlusionsThreshold, float _dampingFactor, float _claheClip, Result<cv::optflow::OpticalFlowPCAFlow*>* ocvrs_return) {
		try {
			cv::optflow::OpticalFlowPCAFlow* ret = new cv::optflow::OpticalFlowPCAFlow(*_prior, *_basisSize, _sparseRate, _retainedCornersFraction, _occlusionsThreshold, _dampingFactor, _claheClip);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::optflow::OpticalFlowPCAFlow::OpticalFlowPCAFlow() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/pcaflow.hpp:116
	// ("cv::optflow::OpticalFlowPCAFlow::OpticalFlowPCAFlow", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_OpticalFlowPCAFlow_OpticalFlowPCAFlow(Result<cv::optflow::OpticalFlowPCAFlow*>* ocvrs_return) {
		try {
			cv::optflow::OpticalFlowPCAFlow* ret = new cv::optflow::OpticalFlowPCAFlow();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calc(InputArray, InputArray, InputOutputArray)(InputArray, InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/pcaflow.hpp:120
	// ("cv::optflow::OpticalFlowPCAFlow::calc", vec![(pred!(mut, ["I0", "I1", "flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_optflow_OpticalFlowPCAFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(cv::optflow::OpticalFlowPCAFlow* instance, const cv::_InputArray* I0, const cv::_InputArray* I1, const cv::_InputOutputArray* flow, ResultVoid* ocvrs_return) {
		try {
			instance->calc(*I0, *I1, *flow);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// collectGarbage()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/pcaflow.hpp:121
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

	// PCAPrior(const char *)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/pcaflow.hpp:83
	// ("cv::optflow::PCAPrior::PCAPrior", vec![(pred!(mut, ["pathToPrior"], ["const char*"]), _)]),
	void cv_optflow_PCAPrior_PCAPrior_const_charX(const char* pathToPrior, Result<cv::optflow::PCAPrior*>* ocvrs_return) {
		try {
			cv::optflow::PCAPrior* ret = new cv::optflow::PCAPrior(pathToPrior);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPadding()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/pcaflow.hpp:85
	// ("cv::optflow::PCAPrior::getPadding", vec![(pred!(const, [], []), _)]),
	void cv_optflow_PCAPrior_getPadding_const(const cv::optflow::PCAPrior* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPadding();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBasisSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/pcaflow.hpp:87
	// ("cv::optflow::PCAPrior::getBasisSize", vec![(pred!(const, [], []), _)]),
	void cv_optflow_PCAPrior_getBasisSize_const(const cv::optflow::PCAPrior* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getBasisSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fillConstraints(float *, float *, float *, float *)(Indirect, Indirect, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/pcaflow.hpp:89
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

	// RLOFOpticalFlowParameter()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:66
	// ("cv::optflow::RLOFOpticalFlowParameter::RLOFOpticalFlowParameter", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_RLOFOpticalFlowParameter(Result<cv::optflow::RLOFOpticalFlowParameter*>* ocvrs_return) {
		try {
			cv::optflow::RLOFOpticalFlowParameter* ret = new cv::optflow::RLOFOpticalFlowParameter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseMEstimator(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:160
	// ("cv::optflow::RLOFOpticalFlowParameter::setUseMEstimator", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_setUseMEstimator_bool(cv::optflow::RLOFOpticalFlowParameter* instance, bool val, ResultVoid* ocvrs_return) {
		try {
			instance->setUseMEstimator(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSolverType(SolverType)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:162
	// ("cv::optflow::RLOFOpticalFlowParameter::setSolverType", vec![(pred!(mut, ["val"], ["cv::optflow::SolverType"]), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_setSolverType_SolverType(cv::optflow::RLOFOpticalFlowParameter* instance, cv::optflow::SolverType val, ResultVoid* ocvrs_return) {
		try {
			instance->setSolverType(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSolverType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:163
	// ("cv::optflow::RLOFOpticalFlowParameter::getSolverType", vec![(pred!(const, [], []), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_getSolverType_const(const cv::optflow::RLOFOpticalFlowParameter* instance, Result<cv::optflow::SolverType>* ocvrs_return) {
		try {
			cv::optflow::SolverType ret = instance->getSolverType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSupportRegionType(SupportRegionType)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:165
	// ("cv::optflow::RLOFOpticalFlowParameter::setSupportRegionType", vec![(pred!(mut, ["val"], ["cv::optflow::SupportRegionType"]), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_setSupportRegionType_SupportRegionType(cv::optflow::RLOFOpticalFlowParameter* instance, cv::optflow::SupportRegionType val, ResultVoid* ocvrs_return) {
		try {
			instance->setSupportRegionType(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSupportRegionType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:166
	// ("cv::optflow::RLOFOpticalFlowParameter::getSupportRegionType", vec![(pred!(const, [], []), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_getSupportRegionType_const(const cv::optflow::RLOFOpticalFlowParameter* instance, Result<cv::optflow::SupportRegionType>* ocvrs_return) {
		try {
			cv::optflow::SupportRegionType ret = instance->getSupportRegionType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNormSigma0(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:168
	// ("cv::optflow::RLOFOpticalFlowParameter::setNormSigma0", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_setNormSigma0_float(cv::optflow::RLOFOpticalFlowParameter* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setNormSigma0(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNormSigma0()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:169
	// ("cv::optflow::RLOFOpticalFlowParameter::getNormSigma0", vec![(pred!(const, [], []), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_getNormSigma0_const(const cv::optflow::RLOFOpticalFlowParameter* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getNormSigma0();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNormSigma1(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:171
	// ("cv::optflow::RLOFOpticalFlowParameter::setNormSigma1", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_setNormSigma1_float(cv::optflow::RLOFOpticalFlowParameter* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setNormSigma1(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNormSigma1()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:172
	// ("cv::optflow::RLOFOpticalFlowParameter::getNormSigma1", vec![(pred!(const, [], []), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_getNormSigma1_const(const cv::optflow::RLOFOpticalFlowParameter* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getNormSigma1();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSmallWinSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:174
	// ("cv::optflow::RLOFOpticalFlowParameter::setSmallWinSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_setSmallWinSize_int(cv::optflow::RLOFOpticalFlowParameter* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setSmallWinSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSmallWinSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:175
	// ("cv::optflow::RLOFOpticalFlowParameter::getSmallWinSize", vec![(pred!(const, [], []), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_getSmallWinSize_const(const cv::optflow::RLOFOpticalFlowParameter* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSmallWinSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setLargeWinSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:177
	// ("cv::optflow::RLOFOpticalFlowParameter::setLargeWinSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_setLargeWinSize_int(cv::optflow::RLOFOpticalFlowParameter* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setLargeWinSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLargeWinSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:178
	// ("cv::optflow::RLOFOpticalFlowParameter::getLargeWinSize", vec![(pred!(const, [], []), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_getLargeWinSize_const(const cv::optflow::RLOFOpticalFlowParameter* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getLargeWinSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCrossSegmentationThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:180
	// ("cv::optflow::RLOFOpticalFlowParameter::setCrossSegmentationThreshold", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_setCrossSegmentationThreshold_int(cv::optflow::RLOFOpticalFlowParameter* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setCrossSegmentationThreshold(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCrossSegmentationThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:181
	// ("cv::optflow::RLOFOpticalFlowParameter::getCrossSegmentationThreshold", vec![(pred!(const, [], []), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_getCrossSegmentationThreshold_const(const cv::optflow::RLOFOpticalFlowParameter* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getCrossSegmentationThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxLevel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:183
	// ("cv::optflow::RLOFOpticalFlowParameter::setMaxLevel", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_setMaxLevel_int(cv::optflow::RLOFOpticalFlowParameter* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxLevel(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxLevel()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:184
	// ("cv::optflow::RLOFOpticalFlowParameter::getMaxLevel", vec![(pred!(const, [], []), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_getMaxLevel_const(const cv::optflow::RLOFOpticalFlowParameter* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxLevel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseInitialFlow(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:186
	// ("cv::optflow::RLOFOpticalFlowParameter::setUseInitialFlow", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_setUseInitialFlow_bool(cv::optflow::RLOFOpticalFlowParameter* instance, bool val, ResultVoid* ocvrs_return) {
		try {
			instance->setUseInitialFlow(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUseInitialFlow()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:187
	// ("cv::optflow::RLOFOpticalFlowParameter::getUseInitialFlow", vec![(pred!(const, [], []), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_getUseInitialFlow_const(const cv::optflow::RLOFOpticalFlowParameter* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseInitialFlow();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseIlluminationModel(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:189
	// ("cv::optflow::RLOFOpticalFlowParameter::setUseIlluminationModel", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_setUseIlluminationModel_bool(cv::optflow::RLOFOpticalFlowParameter* instance, bool val, ResultVoid* ocvrs_return) {
		try {
			instance->setUseIlluminationModel(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUseIlluminationModel()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:190
	// ("cv::optflow::RLOFOpticalFlowParameter::getUseIlluminationModel", vec![(pred!(const, [], []), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_getUseIlluminationModel_const(const cv::optflow::RLOFOpticalFlowParameter* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseIlluminationModel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseGlobalMotionPrior(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:192
	// ("cv::optflow::RLOFOpticalFlowParameter::setUseGlobalMotionPrior", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_setUseGlobalMotionPrior_bool(cv::optflow::RLOFOpticalFlowParameter* instance, bool val, ResultVoid* ocvrs_return) {
		try {
			instance->setUseGlobalMotionPrior(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUseGlobalMotionPrior()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:193
	// ("cv::optflow::RLOFOpticalFlowParameter::getUseGlobalMotionPrior", vec![(pred!(const, [], []), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_getUseGlobalMotionPrior_const(const cv::optflow::RLOFOpticalFlowParameter* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseGlobalMotionPrior();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxIteration(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:195
	// ("cv::optflow::RLOFOpticalFlowParameter::setMaxIteration", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_setMaxIteration_int(cv::optflow::RLOFOpticalFlowParameter* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxIteration(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxIteration()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:196
	// ("cv::optflow::RLOFOpticalFlowParameter::getMaxIteration", vec![(pred!(const, [], []), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_getMaxIteration_const(const cv::optflow::RLOFOpticalFlowParameter* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxIteration();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinEigenValue(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:198
	// ("cv::optflow::RLOFOpticalFlowParameter::setMinEigenValue", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_setMinEigenValue_float(cv::optflow::RLOFOpticalFlowParameter* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setMinEigenValue(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinEigenValue()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:199
	// ("cv::optflow::RLOFOpticalFlowParameter::getMinEigenValue", vec![(pred!(const, [], []), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_getMinEigenValue_const(const cv::optflow::RLOFOpticalFlowParameter* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMinEigenValue();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setGlobalMotionRansacThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:201
	// ("cv::optflow::RLOFOpticalFlowParameter::setGlobalMotionRansacThreshold", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_setGlobalMotionRansacThreshold_float(cv::optflow::RLOFOpticalFlowParameter* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setGlobalMotionRansacThreshold(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGlobalMotionRansacThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:202
	// ("cv::optflow::RLOFOpticalFlowParameter::getGlobalMotionRansacThreshold", vec![(pred!(const, [], []), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_getGlobalMotionRansacThreshold_const(const cv::optflow::RLOFOpticalFlowParameter* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getGlobalMotionRansacThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:205
	// ("cv::optflow::RLOFOpticalFlowParameter::create", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_create(Result<cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::optflow::RLOFOpticalFlowParameter> ret = cv::optflow::RLOFOpticalFlowParameter::create();
			Ok(new cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::optflow::RLOFOpticalFlowParameter::solverType() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:83
	// ("cv::optflow::RLOFOpticalFlowParameter::solverType", vec![(pred!(const, [], []), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_propSolverType_const(const cv::optflow::RLOFOpticalFlowParameter* instance, cv::optflow::SolverType* ocvrs_return) {
			cv::optflow::SolverType ret = instance->solverType;
			*ocvrs_return = ret;
	}

	// cv::optflow::RLOFOpticalFlowParameter::setSolverType(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:83
	// ("cv::optflow::RLOFOpticalFlowParameter::setSolverType", vec![(pred!(mut, ["val"], ["const cv::optflow::SolverType"]), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_propSolverType_const_SolverType(cv::optflow::RLOFOpticalFlowParameter* instance, const cv::optflow::SolverType val) {
			instance->solverType = val;
	}

	// cv::optflow::RLOFOpticalFlowParameter::supportRegionType() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:88
	// ("cv::optflow::RLOFOpticalFlowParameter::supportRegionType", vec![(pred!(const, [], []), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_propSupportRegionType_const(const cv::optflow::RLOFOpticalFlowParameter* instance, cv::optflow::SupportRegionType* ocvrs_return) {
			cv::optflow::SupportRegionType ret = instance->supportRegionType;
			*ocvrs_return = ret;
	}

	// cv::optflow::RLOFOpticalFlowParameter::setSupportRegionType(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:88
	// ("cv::optflow::RLOFOpticalFlowParameter::setSupportRegionType", vec![(pred!(mut, ["val"], ["const cv::optflow::SupportRegionType"]), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_propSupportRegionType_const_SupportRegionType(cv::optflow::RLOFOpticalFlowParameter* instance, const cv::optflow::SupportRegionType val) {
			instance->supportRegionType = val;
	}

	// cv::optflow::RLOFOpticalFlowParameter::normSigma0() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:92
	// ("cv::optflow::RLOFOpticalFlowParameter::normSigma0", vec![(pred!(const, [], []), _)]),
	float cv_optflow_RLOFOpticalFlowParameter_propNormSigma0_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
			float ret = instance->normSigma0;
			return ret;
	}

	// cv::optflow::RLOFOpticalFlowParameter::setNormSigma0(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:92
	// ("cv::optflow::RLOFOpticalFlowParameter::setNormSigma0", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_propNormSigma0_const_float(cv::optflow::RLOFOpticalFlowParameter* instance, const float val) {
			instance->normSigma0 = val;
	}

	// cv::optflow::RLOFOpticalFlowParameter::normSigma1() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:98
	// ("cv::optflow::RLOFOpticalFlowParameter::normSigma1", vec![(pred!(const, [], []), _)]),
	float cv_optflow_RLOFOpticalFlowParameter_propNormSigma1_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
			float ret = instance->normSigma1;
			return ret;
	}

	// cv::optflow::RLOFOpticalFlowParameter::setNormSigma1(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:98
	// ("cv::optflow::RLOFOpticalFlowParameter::setNormSigma1", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_propNormSigma1_const_float(cv::optflow::RLOFOpticalFlowParameter* instance, const float val) {
			instance->normSigma1 = val;
	}

	// cv::optflow::RLOFOpticalFlowParameter::smallWinSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:104
	// ("cv::optflow::RLOFOpticalFlowParameter::smallWinSize", vec![(pred!(const, [], []), _)]),
	int cv_optflow_RLOFOpticalFlowParameter_propSmallWinSize_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
			int ret = instance->smallWinSize;
			return ret;
	}

	// cv::optflow::RLOFOpticalFlowParameter::setSmallWinSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:104
	// ("cv::optflow::RLOFOpticalFlowParameter::setSmallWinSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_propSmallWinSize_const_int(cv::optflow::RLOFOpticalFlowParameter* instance, const int val) {
			instance->smallWinSize = val;
	}

	// cv::optflow::RLOFOpticalFlowParameter::largeWinSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:107
	// ("cv::optflow::RLOFOpticalFlowParameter::largeWinSize", vec![(pred!(const, [], []), _)]),
	int cv_optflow_RLOFOpticalFlowParameter_propLargeWinSize_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
			int ret = instance->largeWinSize;
			return ret;
	}

	// cv::optflow::RLOFOpticalFlowParameter::setLargeWinSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:107
	// ("cv::optflow::RLOFOpticalFlowParameter::setLargeWinSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_propLargeWinSize_const_int(cv::optflow::RLOFOpticalFlowParameter* instance, const int val) {
			instance->largeWinSize = val;
	}

	// cv::optflow::RLOFOpticalFlowParameter::crossSegmentationThreshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:112
	// ("cv::optflow::RLOFOpticalFlowParameter::crossSegmentationThreshold", vec![(pred!(const, [], []), _)]),
	int cv_optflow_RLOFOpticalFlowParameter_propCrossSegmentationThreshold_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
			int ret = instance->crossSegmentationThreshold;
			return ret;
	}

	// cv::optflow::RLOFOpticalFlowParameter::setCrossSegmentationThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:112
	// ("cv::optflow::RLOFOpticalFlowParameter::setCrossSegmentationThreshold", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_propCrossSegmentationThreshold_const_int(cv::optflow::RLOFOpticalFlowParameter* instance, const int val) {
			instance->crossSegmentationThreshold = val;
	}

	// cv::optflow::RLOFOpticalFlowParameter::maxLevel() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:117
	// ("cv::optflow::RLOFOpticalFlowParameter::maxLevel", vec![(pred!(const, [], []), _)]),
	int cv_optflow_RLOFOpticalFlowParameter_propMaxLevel_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
			int ret = instance->maxLevel;
			return ret;
	}

	// cv::optflow::RLOFOpticalFlowParameter::setMaxLevel(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:117
	// ("cv::optflow::RLOFOpticalFlowParameter::setMaxLevel", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_propMaxLevel_const_int(cv::optflow::RLOFOpticalFlowParameter* instance, const int val) {
			instance->maxLevel = val;
	}

	// cv::optflow::RLOFOpticalFlowParameter::useInitialFlow() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:122
	// ("cv::optflow::RLOFOpticalFlowParameter::useInitialFlow", vec![(pred!(const, [], []), _)]),
	bool cv_optflow_RLOFOpticalFlowParameter_propUseInitialFlow_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
			bool ret = instance->useInitialFlow;
			return ret;
	}

	// cv::optflow::RLOFOpticalFlowParameter::setUseInitialFlow(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:122
	// ("cv::optflow::RLOFOpticalFlowParameter::setUseInitialFlow", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_propUseInitialFlow_const_bool(cv::optflow::RLOFOpticalFlowParameter* instance, const bool val) {
			instance->useInitialFlow = val;
	}

	// cv::optflow::RLOFOpticalFlowParameter::useIlluminationModel() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:126
	// ("cv::optflow::RLOFOpticalFlowParameter::useIlluminationModel", vec![(pred!(const, [], []), _)]),
	bool cv_optflow_RLOFOpticalFlowParameter_propUseIlluminationModel_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
			bool ret = instance->useIlluminationModel;
			return ret;
	}

	// cv::optflow::RLOFOpticalFlowParameter::setUseIlluminationModel(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:126
	// ("cv::optflow::RLOFOpticalFlowParameter::setUseIlluminationModel", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_propUseIlluminationModel_const_bool(cv::optflow::RLOFOpticalFlowParameter* instance, const bool val) {
			instance->useIlluminationModel = val;
	}

	// cv::optflow::RLOFOpticalFlowParameter::useGlobalMotionPrior() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:134
	// ("cv::optflow::RLOFOpticalFlowParameter::useGlobalMotionPrior", vec![(pred!(const, [], []), _)]),
	bool cv_optflow_RLOFOpticalFlowParameter_propUseGlobalMotionPrior_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
			bool ret = instance->useGlobalMotionPrior;
			return ret;
	}

	// cv::optflow::RLOFOpticalFlowParameter::setUseGlobalMotionPrior(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:134
	// ("cv::optflow::RLOFOpticalFlowParameter::setUseGlobalMotionPrior", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_propUseGlobalMotionPrior_const_bool(cv::optflow::RLOFOpticalFlowParameter* instance, const bool val) {
			instance->useGlobalMotionPrior = val;
	}

	// cv::optflow::RLOFOpticalFlowParameter::maxIteration() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:139
	// ("cv::optflow::RLOFOpticalFlowParameter::maxIteration", vec![(pred!(const, [], []), _)]),
	int cv_optflow_RLOFOpticalFlowParameter_propMaxIteration_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
			int ret = instance->maxIteration;
			return ret;
	}

	// cv::optflow::RLOFOpticalFlowParameter::setMaxIteration(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:139
	// ("cv::optflow::RLOFOpticalFlowParameter::setMaxIteration", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_propMaxIteration_const_int(cv::optflow::RLOFOpticalFlowParameter* instance, const int val) {
			instance->maxIteration = val;
	}

	// cv::optflow::RLOFOpticalFlowParameter::minEigenValue() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:143
	// ("cv::optflow::RLOFOpticalFlowParameter::minEigenValue", vec![(pred!(const, [], []), _)]),
	float cv_optflow_RLOFOpticalFlowParameter_propMinEigenValue_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
			float ret = instance->minEigenValue;
			return ret;
	}

	// cv::optflow::RLOFOpticalFlowParameter::setMinEigenValue(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:143
	// ("cv::optflow::RLOFOpticalFlowParameter::setMinEigenValue", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_propMinEigenValue_const_float(cv::optflow::RLOFOpticalFlowParameter* instance, const float val) {
			instance->minEigenValue = val;
	}

	// cv::optflow::RLOFOpticalFlowParameter::globalMotionRansacThreshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:147
	// ("cv::optflow::RLOFOpticalFlowParameter::globalMotionRansacThreshold", vec![(pred!(const, [], []), _)]),
	float cv_optflow_RLOFOpticalFlowParameter_propGlobalMotionRansacThreshold_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
			float ret = instance->globalMotionRansacThreshold;
			return ret;
	}

	// cv::optflow::RLOFOpticalFlowParameter::setGlobalMotionRansacThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:147
	// ("cv::optflow::RLOFOpticalFlowParameter::setGlobalMotionRansacThreshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_propGlobalMotionRansacThreshold_const_float(cv::optflow::RLOFOpticalFlowParameter* instance, const float val) {
			instance->globalMotionRansacThreshold = val;
	}

	// cv::optflow::RLOFOpticalFlowParameter::delete() generated
	// ("cv::optflow::RLOFOpticalFlowParameter::delete", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_RLOFOpticalFlowParameter_delete(cv::optflow::RLOFOpticalFlowParameter* instance) {
			delete instance;
	}

	// setRLOFOpticalFlowParameter(Ptr<RLOFOpticalFlowParameter>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:417
	// ("cv::optflow::SparseRLOFOpticalFlow::setRLOFOpticalFlowParameter", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>"]), _)]),
	void cv_optflow_SparseRLOFOpticalFlow_setRLOFOpticalFlowParameter_PtrLRLOFOpticalFlowParameterG(cv::optflow::SparseRLOFOpticalFlow* instance, cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>* val, ResultVoid* ocvrs_return) {
		try {
			instance->setRLOFOpticalFlowParameter(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRLOFOpticalFlowParameter()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:421
	// ("cv::optflow::SparseRLOFOpticalFlow::getRLOFOpticalFlowParameter", vec![(pred!(const, [], []), _)]),
	void cv_optflow_SparseRLOFOpticalFlow_getRLOFOpticalFlowParameter_const(const cv::optflow::SparseRLOFOpticalFlow* instance, Result<cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::optflow::RLOFOpticalFlowParameter> ret = instance->getRLOFOpticalFlowParameter();
			Ok(new cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setForwardBackward(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:430
	// ("cv::optflow::SparseRLOFOpticalFlow::setForwardBackward", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_optflow_SparseRLOFOpticalFlow_setForwardBackward_float(cv::optflow::SparseRLOFOpticalFlow* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setForwardBackward(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getForwardBackward()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:434
	// ("cv::optflow::SparseRLOFOpticalFlow::getForwardBackward", vec![(pred!(const, [], []), _)]),
	void cv_optflow_SparseRLOFOpticalFlow_getForwardBackward_const(const cv::optflow::SparseRLOFOpticalFlow* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getForwardBackward();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(Ptr<RLOFOpticalFlowParameter>, float)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:441
	// ("cv::optflow::SparseRLOFOpticalFlow::create", vec![(pred!(mut, ["rlofParam", "forwardBackwardThreshold"], ["cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>", "float"]), _)]),
	void cv_optflow_SparseRLOFOpticalFlow_create_PtrLRLOFOpticalFlowParameterG_float(cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>* rlofParam, float forwardBackwardThreshold, Result<cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::optflow::SparseRLOFOpticalFlow> ret = cv::optflow::SparseRLOFOpticalFlow::create(*rlofParam, forwardBackwardThreshold);
			Ok(new cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::optflow::SparseRLOFOpticalFlow::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/optflow/rlofflow.hpp:441
	// ("cv::optflow::SparseRLOFOpticalFlow::create", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_SparseRLOFOpticalFlow_create(Result<cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::optflow::SparseRLOFOpticalFlow> ret = cv::optflow::SparseRLOFOpticalFlow::create();
			Ok(new cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::optflow::SparseRLOFOpticalFlow::to_Algorithm() generated
	// ("cv::optflow::SparseRLOFOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_optflow_SparseRLOFOpticalFlow_to_Algorithm(cv::optflow::SparseRLOFOpticalFlow* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::optflow::SparseRLOFOpticalFlow::to_SparseOpticalFlow() generated
	// ("cv::optflow::SparseRLOFOpticalFlow::to_SparseOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::SparseOpticalFlow* cv_optflow_SparseRLOFOpticalFlow_to_SparseOpticalFlow(cv::optflow::SparseRLOFOpticalFlow* instance) {
			return dynamic_cast<cv::SparseOpticalFlow*>(instance);
	}

	// cv::optflow::SparseRLOFOpticalFlow::delete() generated
	// ("cv::optflow::SparseRLOFOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
	void cv_optflow_SparseRLOFOpticalFlow_delete(cv::optflow::SparseRLOFOpticalFlow* instance) {
			delete instance;
	}

}
