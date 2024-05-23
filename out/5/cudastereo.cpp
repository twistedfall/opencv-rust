#include "ocvrs_common.hpp"
#include <opencv2/cudastereo.hpp>
#include "cudastereo_types.hpp"

extern "C" {
	// cv::cuda::createDisparityBilateralFilter() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:340
	// ("cv::cuda::createDisparityBilateralFilter", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_createDisparityBilateralFilter(Result<cv::Ptr<cv::cuda::DisparityBilateralFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::DisparityBilateralFilter> ret = cv::cuda::createDisparityBilateralFilter();
			Ok(new cv::Ptr<cv::cuda::DisparityBilateralFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createDisparityBilateralFilter(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:340
	// ("cv::cuda::createDisparityBilateralFilter", vec![(pred!(mut, ["ndisp", "radius", "iters"], ["int", "int", "int"]), _)]),
	void cv_cuda_createDisparityBilateralFilter_int_int_int(int ndisp, int radius, int iters, Result<cv::Ptr<cv::cuda::DisparityBilateralFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::DisparityBilateralFilter> ret = cv::cuda::createDisparityBilateralFilter(ndisp, radius, iters);
			Ok(new cv::Ptr<cv::cuda::DisparityBilateralFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::createStereoBM() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:91
	// ("cv::cuda::createStereoBM", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_createStereoBM(Result<cv::Ptr<cv::cuda::StereoBM>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::StereoBM> ret = cv::cuda::createStereoBM();
			Ok(new cv::Ptr<cv::cuda::StereoBM>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createStereoBM(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:91
	// ("cv::cuda::createStereoBM", vec![(pred!(mut, ["numDisparities", "blockSize"], ["int", "int"]), _)]),
	void cv_cuda_createStereoBM_int_int(int numDisparities, int blockSize, Result<cv::Ptr<cv::cuda::StereoBM>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::StereoBM> ret = cv::cuda::createStereoBM(numDisparities, blockSize);
			Ok(new cv::Ptr<cv::cuda::StereoBM>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::createStereoBeliefPropagation() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:190
	// ("cv::cuda::createStereoBeliefPropagation", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_createStereoBeliefPropagation(Result<cv::Ptr<cv::cuda::StereoBeliefPropagation>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::StereoBeliefPropagation> ret = cv::cuda::createStereoBeliefPropagation();
			Ok(new cv::Ptr<cv::cuda::StereoBeliefPropagation>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createStereoBeliefPropagation(int, int, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:190
	// ("cv::cuda::createStereoBeliefPropagation", vec![(pred!(mut, ["ndisp", "iters", "levels", "msg_type"], ["int", "int", "int", "int"]), _)]),
	void cv_cuda_createStereoBeliefPropagation_int_int_int_int(int ndisp, int iters, int levels, int msg_type, Result<cv::Ptr<cv::cuda::StereoBeliefPropagation>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::StereoBeliefPropagation> ret = cv::cuda::createStereoBeliefPropagation(ndisp, iters, levels, msg_type);
			Ok(new cv::Ptr<cv::cuda::StereoBeliefPropagation>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::createStereoConstantSpaceBP() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:243
	// ("cv::cuda::createStereoConstantSpaceBP", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_createStereoConstantSpaceBP(Result<cv::Ptr<cv::cuda::StereoConstantSpaceBP>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::StereoConstantSpaceBP> ret = cv::cuda::createStereoConstantSpaceBP();
			Ok(new cv::Ptr<cv::cuda::StereoConstantSpaceBP>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createStereoConstantSpaceBP(int, int, int, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:243
	// ("cv::cuda::createStereoConstantSpaceBP", vec![(pred!(mut, ["ndisp", "iters", "levels", "nr_plane", "msg_type"], ["int", "int", "int", "int", "int"]), _)]),
	void cv_cuda_createStereoConstantSpaceBP_int_int_int_int_int(int ndisp, int iters, int levels, int nr_plane, int msg_type, Result<cv::Ptr<cv::cuda::StereoConstantSpaceBP>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::StereoConstantSpaceBP> ret = cv::cuda::createStereoConstantSpaceBP(ndisp, iters, levels, nr_plane, msg_type);
			Ok(new cv::Ptr<cv::cuda::StereoConstantSpaceBP>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::createStereoSGM() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:290
	// ("cv::cuda::createStereoSGM", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_createStereoSGM(Result<cv::Ptr<cv::cuda::StereoSGM>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::StereoSGM> ret = cv::cuda::createStereoSGM();
			Ok(new cv::Ptr<cv::cuda::StereoSGM>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createStereoSGM(int, int, int, int, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:290
	// ("cv::cuda::createStereoSGM", vec![(pred!(mut, ["minDisparity", "numDisparities", "P1", "P2", "uniquenessRatio", "mode"], ["int", "int", "int", "int", "int", "int"]), _)]),
	void cv_cuda_createStereoSGM_int_int_int_int_int_int(int minDisparity, int numDisparities, int P1, int P2, int uniquenessRatio, int mode, Result<cv::Ptr<cv::cuda::StereoSGM>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::StereoSGM> ret = cv::cuda::createStereoSGM(minDisparity, numDisparities, P1, P2, uniquenessRatio, mode);
			Ok(new cv::Ptr<cv::cuda::StereoSGM>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::drawColorDisp(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:378
	// ("cv::cuda::drawColorDisp", vec![(pred!(mut, ["src_disp", "dst_disp", "ndisp"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_cuda_drawColorDisp_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src_disp, const cv::_OutputArray* dst_disp, int ndisp, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::drawColorDisp(*src_disp, *dst_disp, ndisp);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawColorDisp(InputArray, OutputArray, int, Stream &)(InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:378
	// ("cv::cuda::drawColorDisp", vec![(pred!(mut, ["src_disp", "dst_disp", "ndisp", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_drawColorDisp_const__InputArrayR_const__OutputArrayR_int_StreamR(const cv::_InputArray* src_disp, const cv::_OutputArray* dst_disp, int ndisp, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::drawColorDisp(*src_disp, *dst_disp, ndisp, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::reprojectImageTo3D(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:360
	// ("cv::cuda::reprojectImageTo3D", vec![(pred!(mut, ["disp", "xyzw", "Q"], ["cv::cuda::GpuMat", "cv::cuda::GpuMat*", "cv::Mat"]), _)]),
	void cv_cuda_reprojectImageTo3D_GpuMat_GpuMatR_Mat(cv::cuda::GpuMat* disp, cv::cuda::GpuMat* xyzw, cv::Mat* Q, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::reprojectImageTo3D(*disp, *xyzw, *Q);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reprojectImageTo3D(GpuMat, GpuMat &, Mat, int, Stream &)(TraitClass, TraitClass, TraitClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:360
	// ("cv::cuda::reprojectImageTo3D", vec![(pred!(mut, ["disp", "xyzw", "Q", "dst_cn", "stream"], ["cv::cuda::GpuMat", "cv::cuda::GpuMat*", "cv::Mat", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_reprojectImageTo3D_GpuMat_GpuMatR_Mat_int_StreamR(cv::cuda::GpuMat* disp, cv::cuda::GpuMat* xyzw, cv::Mat* Q, int dst_cn, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::reprojectImageTo3D(*disp, *xyzw, *Q, dst_cn, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::reprojectImageTo3D(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:359
	// ("cv::cuda::reprojectImageTo3D", vec![(pred!(mut, ["disp", "xyzw", "Q"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_cuda_reprojectImageTo3D_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* disp, const cv::_OutputArray* xyzw, const cv::_InputArray* Q, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::reprojectImageTo3D(*disp, *xyzw, *Q);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reprojectImageTo3D(InputArray, OutputArray, InputArray, int, Stream &)(InputArray, OutputArray, InputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:359
	// ("cv::cuda::reprojectImageTo3D", vec![(pred!(mut, ["disp", "xyzw", "Q", "dst_cn", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_reprojectImageTo3D_const__InputArrayR_const__OutputArrayR_const__InputArrayR_int_StreamR(const cv::_InputArray* disp, const cv::_OutputArray* xyzw, const cv::_InputArray* Q, int dst_cn, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::reprojectImageTo3D(*disp, *xyzw, *Q, dst_cn, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// apply(InputArray, InputArray, OutputArray, Stream &)(InputArray, InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:309
	// ("cv::cuda::DisparityBilateralFilter::apply", vec![(pred!(mut, ["disparity", "image", "dst", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_DisparityBilateralFilter_apply_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::DisparityBilateralFilter* instance, const cv::_InputArray* disparity, const cv::_InputArray* image, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*disparity, *image, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::DisparityBilateralFilter::apply(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:309
	// ("cv::cuda::DisparityBilateralFilter::apply", vec![(pred!(mut, ["disparity", "image", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_DisparityBilateralFilter_apply_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::cuda::DisparityBilateralFilter* instance, const cv::_InputArray* disparity, const cv::_InputArray* image, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*disparity, *image, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumDisparities()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:311
	// ("cv::cuda::DisparityBilateralFilter::getNumDisparities", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DisparityBilateralFilter_getNumDisparities_const(const cv::cuda::DisparityBilateralFilter* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumDisparities();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNumDisparities(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:312
	// ("cv::cuda::DisparityBilateralFilter::setNumDisparities", vec![(pred!(mut, ["numDisparities"], ["int"]), _)]),
	void cv_cuda_DisparityBilateralFilter_setNumDisparities_int(cv::cuda::DisparityBilateralFilter* instance, int numDisparities, ResultVoid* ocvrs_return) {
		try {
			instance->setNumDisparities(numDisparities);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRadius()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:314
	// ("cv::cuda::DisparityBilateralFilter::getRadius", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DisparityBilateralFilter_getRadius_const(const cv::cuda::DisparityBilateralFilter* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getRadius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:315
	// ("cv::cuda::DisparityBilateralFilter::setRadius", vec![(pred!(mut, ["radius"], ["int"]), _)]),
	void cv_cuda_DisparityBilateralFilter_setRadius_int(cv::cuda::DisparityBilateralFilter* instance, int radius, ResultVoid* ocvrs_return) {
		try {
			instance->setRadius(radius);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumIters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:317
	// ("cv::cuda::DisparityBilateralFilter::getNumIters", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DisparityBilateralFilter_getNumIters_const(const cv::cuda::DisparityBilateralFilter* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumIters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNumIters(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:318
	// ("cv::cuda::DisparityBilateralFilter::setNumIters", vec![(pred!(mut, ["iters"], ["int"]), _)]),
	void cv_cuda_DisparityBilateralFilter_setNumIters_int(cv::cuda::DisparityBilateralFilter* instance, int iters, ResultVoid* ocvrs_return) {
		try {
			instance->setNumIters(iters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEdgeThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:321
	// ("cv::cuda::DisparityBilateralFilter::getEdgeThreshold", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DisparityBilateralFilter_getEdgeThreshold_const(const cv::cuda::DisparityBilateralFilter* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getEdgeThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEdgeThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:322
	// ("cv::cuda::DisparityBilateralFilter::setEdgeThreshold", vec![(pred!(mut, ["edge_threshold"], ["double"]), _)]),
	void cv_cuda_DisparityBilateralFilter_setEdgeThreshold_double(cv::cuda::DisparityBilateralFilter* instance, double edge_threshold, ResultVoid* ocvrs_return) {
		try {
			instance->setEdgeThreshold(edge_threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxDiscThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:325
	// ("cv::cuda::DisparityBilateralFilter::getMaxDiscThreshold", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DisparityBilateralFilter_getMaxDiscThreshold_const(const cv::cuda::DisparityBilateralFilter* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxDiscThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxDiscThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:326
	// ("cv::cuda::DisparityBilateralFilter::setMaxDiscThreshold", vec![(pred!(mut, ["max_disc_threshold"], ["double"]), _)]),
	void cv_cuda_DisparityBilateralFilter_setMaxDiscThreshold_double(cv::cuda::DisparityBilateralFilter* instance, double max_disc_threshold, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxDiscThreshold(max_disc_threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSigmaRange()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:329
	// ("cv::cuda::DisparityBilateralFilter::getSigmaRange", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DisparityBilateralFilter_getSigmaRange_const(const cv::cuda::DisparityBilateralFilter* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSigmaRange();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSigmaRange(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:330
	// ("cv::cuda::DisparityBilateralFilter::setSigmaRange", vec![(pred!(mut, ["sigma_range"], ["double"]), _)]),
	void cv_cuda_DisparityBilateralFilter_setSigmaRange_double(cv::cuda::DisparityBilateralFilter* instance, double sigma_range, ResultVoid* ocvrs_return) {
		try {
			instance->setSigmaRange(sigma_range);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::DisparityBilateralFilter::to_Algorithm() generated
	// ("cv::cuda::DisparityBilateralFilter::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_DisparityBilateralFilter_to_Algorithm(cv::cuda::DisparityBilateralFilter* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::DisparityBilateralFilter::delete() generated
	// ("cv::cuda::DisparityBilateralFilter::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_DisparityBilateralFilter_delete(cv::cuda::DisparityBilateralFilter* instance) {
			delete instance;
	}

	// compute(InputArray, InputArray, OutputArray, Stream &)(InputArray, InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:78
	// ("cv::cuda::StereoBM::compute", vec![(pred!(mut, ["left", "right", "disparity", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_StereoBM_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::StereoBM* instance, const cv::_InputArray* left, const cv::_InputArray* right, const cv::_OutputArray* disparity, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*left, *right, *disparity, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::StereoBM::to_Algorithm() generated
	// ("cv::cuda::StereoBM::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_StereoBM_to_Algorithm(cv::cuda::StereoBM* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::StereoBM::to_StereoBM() generated
	// ("cv::cuda::StereoBM::to_StereoBM", vec![(pred!(mut, [], []), _)]),
	cv::StereoBM* cv_cuda_StereoBM_to_StereoBM(cv::cuda::StereoBM* instance) {
			return dynamic_cast<cv::StereoBM*>(instance);
	}

	// cv::cuda::StereoBM::to_StereoMatcher() generated
	// ("cv::cuda::StereoBM::to_StereoMatcher", vec![(pred!(mut, [], []), _)]),
	cv::StereoMatcher* cv_cuda_StereoBM_to_StereoMatcher(cv::cuda::StereoBM* instance) {
			return dynamic_cast<cv::StereoMatcher*>(instance);
	}

	// cv::cuda::StereoBM::delete() generated
	// ("cv::cuda::StereoBM::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_StereoBM_delete(cv::cuda::StereoBM* instance) {
			delete instance;
	}

	// compute(InputArray, InputArray, OutputArray, Stream &)(InputArray, InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:135
	// ("cv::cuda::StereoBeliefPropagation::compute", vec![(pred!(mut, ["left", "right", "disparity", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_StereoBeliefPropagation_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::StereoBeliefPropagation* instance, const cv::_InputArray* left, const cv::_InputArray* right, const cv::_OutputArray* disparity, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*left, *right, *disparity, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:146
	// ("cv::cuda::StereoBeliefPropagation::compute", vec![(pred!(mut, ["data", "disparity", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_StereoBeliefPropagation_compute_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::StereoBeliefPropagation* instance, const cv::_InputArray* data, const cv::_OutputArray* disparity, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*data, *disparity, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::StereoBeliefPropagation::compute(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:146
	// ("cv::cuda::StereoBeliefPropagation::compute", vec![(pred!(mut, ["data", "disparity"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_StereoBeliefPropagation_compute_const__InputArrayR_const__OutputArrayR(cv::cuda::StereoBeliefPropagation* instance, const cv::_InputArray* data, const cv::_OutputArray* disparity, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*data, *disparity);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumIters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:149
	// ("cv::cuda::StereoBeliefPropagation::getNumIters", vec![(pred!(const, [], []), _)]),
	void cv_cuda_StereoBeliefPropagation_getNumIters_const(const cv::cuda::StereoBeliefPropagation* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumIters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNumIters(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:150
	// ("cv::cuda::StereoBeliefPropagation::setNumIters", vec![(pred!(mut, ["iters"], ["int"]), _)]),
	void cv_cuda_StereoBeliefPropagation_setNumIters_int(cv::cuda::StereoBeliefPropagation* instance, int iters, ResultVoid* ocvrs_return) {
		try {
			instance->setNumIters(iters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumLevels()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:153
	// ("cv::cuda::StereoBeliefPropagation::getNumLevels", vec![(pred!(const, [], []), _)]),
	void cv_cuda_StereoBeliefPropagation_getNumLevels_const(const cv::cuda::StereoBeliefPropagation* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumLevels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNumLevels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:154
	// ("cv::cuda::StereoBeliefPropagation::setNumLevels", vec![(pred!(mut, ["levels"], ["int"]), _)]),
	void cv_cuda_StereoBeliefPropagation_setNumLevels_int(cv::cuda::StereoBeliefPropagation* instance, int levels, ResultVoid* ocvrs_return) {
		try {
			instance->setNumLevels(levels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxDataTerm()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:157
	// ("cv::cuda::StereoBeliefPropagation::getMaxDataTerm", vec![(pred!(const, [], []), _)]),
	void cv_cuda_StereoBeliefPropagation_getMaxDataTerm_const(const cv::cuda::StereoBeliefPropagation* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxDataTerm();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxDataTerm(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:158
	// ("cv::cuda::StereoBeliefPropagation::setMaxDataTerm", vec![(pred!(mut, ["max_data_term"], ["double"]), _)]),
	void cv_cuda_StereoBeliefPropagation_setMaxDataTerm_double(cv::cuda::StereoBeliefPropagation* instance, double max_data_term, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxDataTerm(max_data_term);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDataWeight()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:161
	// ("cv::cuda::StereoBeliefPropagation::getDataWeight", vec![(pred!(const, [], []), _)]),
	void cv_cuda_StereoBeliefPropagation_getDataWeight_const(const cv::cuda::StereoBeliefPropagation* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getDataWeight();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDataWeight(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:162
	// ("cv::cuda::StereoBeliefPropagation::setDataWeight", vec![(pred!(mut, ["data_weight"], ["double"]), _)]),
	void cv_cuda_StereoBeliefPropagation_setDataWeight_double(cv::cuda::StereoBeliefPropagation* instance, double data_weight, ResultVoid* ocvrs_return) {
		try {
			instance->setDataWeight(data_weight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxDiscTerm()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:165
	// ("cv::cuda::StereoBeliefPropagation::getMaxDiscTerm", vec![(pred!(const, [], []), _)]),
	void cv_cuda_StereoBeliefPropagation_getMaxDiscTerm_const(const cv::cuda::StereoBeliefPropagation* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxDiscTerm();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxDiscTerm(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:166
	// ("cv::cuda::StereoBeliefPropagation::setMaxDiscTerm", vec![(pred!(mut, ["max_disc_term"], ["double"]), _)]),
	void cv_cuda_StereoBeliefPropagation_setMaxDiscTerm_double(cv::cuda::StereoBeliefPropagation* instance, double max_disc_term, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxDiscTerm(max_disc_term);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDiscSingleJump()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:169
	// ("cv::cuda::StereoBeliefPropagation::getDiscSingleJump", vec![(pred!(const, [], []), _)]),
	void cv_cuda_StereoBeliefPropagation_getDiscSingleJump_const(const cv::cuda::StereoBeliefPropagation* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getDiscSingleJump();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDiscSingleJump(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:170
	// ("cv::cuda::StereoBeliefPropagation::setDiscSingleJump", vec![(pred!(mut, ["disc_single_jump"], ["double"]), _)]),
	void cv_cuda_StereoBeliefPropagation_setDiscSingleJump_double(cv::cuda::StereoBeliefPropagation* instance, double disc_single_jump, ResultVoid* ocvrs_return) {
		try {
			instance->setDiscSingleJump(disc_single_jump);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMsgType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:173
	// ("cv::cuda::StereoBeliefPropagation::getMsgType", vec![(pred!(const, [], []), _)]),
	void cv_cuda_StereoBeliefPropagation_getMsgType_const(const cv::cuda::StereoBeliefPropagation* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMsgType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMsgType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:174
	// ("cv::cuda::StereoBeliefPropagation::setMsgType", vec![(pred!(mut, ["msg_type"], ["int"]), _)]),
	void cv_cuda_StereoBeliefPropagation_setMsgType_int(cv::cuda::StereoBeliefPropagation* instance, int msg_type, ResultVoid* ocvrs_return) {
		try {
			instance->setMsgType(msg_type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimateRecommendedParams(int, int, int &, int &, int &)(Primitive, Primitive, Indirect, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:179
	// ("cv::cuda::StereoBeliefPropagation::estimateRecommendedParams", vec![(pred!(mut, ["width", "height", "ndisp", "iters", "levels"], ["int", "int", "int*", "int*", "int*"]), _)]),
	void cv_cuda_StereoBeliefPropagation_estimateRecommendedParams_int_int_intR_intR_intR(int width, int height, int* ndisp, int* iters, int* levels, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::StereoBeliefPropagation::estimateRecommendedParams(width, height, *ndisp, *iters, *levels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::StereoBeliefPropagation::to_CUDA_StereoConstantSpaceBP() generated
	// ("cv::cuda::StereoBeliefPropagation::to_CUDA_StereoConstantSpaceBP", vec![(pred!(mut, [], []), _)]),
	cv::cuda::StereoConstantSpaceBP* cv_cuda_StereoBeliefPropagation_to_CUDA_StereoConstantSpaceBP(cv::cuda::StereoBeliefPropagation* instance) {
			return dynamic_cast<cv::cuda::StereoConstantSpaceBP*>(instance);
	}

	// cv::cuda::StereoBeliefPropagation::to_Algorithm() generated
	// ("cv::cuda::StereoBeliefPropagation::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_StereoBeliefPropagation_to_Algorithm(cv::cuda::StereoBeliefPropagation* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::StereoBeliefPropagation::to_StereoMatcher() generated
	// ("cv::cuda::StereoBeliefPropagation::to_StereoMatcher", vec![(pred!(mut, [], []), _)]),
	cv::StereoMatcher* cv_cuda_StereoBeliefPropagation_to_StereoMatcher(cv::cuda::StereoBeliefPropagation* instance) {
			return dynamic_cast<cv::StereoMatcher*>(instance);
	}

	// cv::cuda::StereoBeliefPropagation::delete() generated
	// ("cv::cuda::StereoBeliefPropagation::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_StereoBeliefPropagation_delete(cv::cuda::StereoBeliefPropagation* instance) {
			delete instance;
	}

	// getNrPlane()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:222
	// ("cv::cuda::StereoConstantSpaceBP::getNrPlane", vec![(pred!(const, [], []), _)]),
	void cv_cuda_StereoConstantSpaceBP_getNrPlane_const(const cv::cuda::StereoConstantSpaceBP* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNrPlane();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNrPlane(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:223
	// ("cv::cuda::StereoConstantSpaceBP::setNrPlane", vec![(pred!(mut, ["nr_plane"], ["int"]), _)]),
	void cv_cuda_StereoConstantSpaceBP_setNrPlane_int(cv::cuda::StereoConstantSpaceBP* instance, int nr_plane, ResultVoid* ocvrs_return) {
		try {
			instance->setNrPlane(nr_plane);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUseLocalInitDataCost()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:225
	// ("cv::cuda::StereoConstantSpaceBP::getUseLocalInitDataCost", vec![(pred!(const, [], []), _)]),
	void cv_cuda_StereoConstantSpaceBP_getUseLocalInitDataCost_const(const cv::cuda::StereoConstantSpaceBP* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseLocalInitDataCost();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseLocalInitDataCost(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:226
	// ("cv::cuda::StereoConstantSpaceBP::setUseLocalInitDataCost", vec![(pred!(mut, ["use_local_init_data_cost"], ["bool"]), _)]),
	void cv_cuda_StereoConstantSpaceBP_setUseLocalInitDataCost_bool(cv::cuda::StereoConstantSpaceBP* instance, bool use_local_init_data_cost, ResultVoid* ocvrs_return) {
		try {
			instance->setUseLocalInitDataCost(use_local_init_data_cost);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimateRecommendedParams(int, int, int &, int &, int &, int &)(Primitive, Primitive, Indirect, Indirect, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:231
	// ("cv::cuda::StereoConstantSpaceBP::estimateRecommendedParams", vec![(pred!(mut, ["width", "height", "ndisp", "iters", "levels", "nr_plane"], ["int", "int", "int*", "int*", "int*", "int*"]), _)]),
	void cv_cuda_StereoConstantSpaceBP_estimateRecommendedParams_int_int_intR_intR_intR_intR(int width, int height, int* ndisp, int* iters, int* levels, int* nr_plane, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::StereoConstantSpaceBP::estimateRecommendedParams(width, height, *ndisp, *iters, *levels, *nr_plane);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::StereoConstantSpaceBP::to_Algorithm() generated
	// ("cv::cuda::StereoConstantSpaceBP::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_StereoConstantSpaceBP_to_Algorithm(cv::cuda::StereoConstantSpaceBP* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::StereoConstantSpaceBP::to_CUDA_StereoBeliefPropagation() generated
	// ("cv::cuda::StereoConstantSpaceBP::to_CUDA_StereoBeliefPropagation", vec![(pred!(mut, [], []), _)]),
	cv::cuda::StereoBeliefPropagation* cv_cuda_StereoConstantSpaceBP_to_CUDA_StereoBeliefPropagation(cv::cuda::StereoConstantSpaceBP* instance) {
			return dynamic_cast<cv::cuda::StereoBeliefPropagation*>(instance);
	}

	// cv::cuda::StereoConstantSpaceBP::to_StereoMatcher() generated
	// ("cv::cuda::StereoConstantSpaceBP::to_StereoMatcher", vec![(pred!(mut, [], []), _)]),
	cv::StereoMatcher* cv_cuda_StereoConstantSpaceBP_to_StereoMatcher(cv::cuda::StereoConstantSpaceBP* instance) {
			return dynamic_cast<cv::StereoMatcher*>(instance);
	}

	// cv::cuda::StereoConstantSpaceBP::delete() generated
	// ("cv::cuda::StereoConstantSpaceBP::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_StereoConstantSpaceBP_delete(cv::cuda::StereoConstantSpaceBP* instance) {
			delete instance;
	}

	// compute(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:269
	// ("cv::cuda::StereoSGM::compute", vec![(pred!(mut, ["left", "right", "disparity"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_StereoSGM_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::cuda::StereoSGM* instance, const cv::_InputArray* left, const cv::_InputArray* right, const cv::_OutputArray* disparity, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*left, *right, *disparity);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArray, InputArray, OutputArray, Stream &)(InputArray, InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudastereo.hpp:275
	// ("cv::cuda::StereoSGM::compute", vec![(pred!(mut, ["left", "right", "disparity", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_StereoSGM_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::StereoSGM* instance, const cv::_InputArray* left, const cv::_InputArray* right, const cv::_OutputArray* disparity, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*left, *right, *disparity, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::StereoSGM::to_Algorithm() generated
	// ("cv::cuda::StereoSGM::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_StereoSGM_to_Algorithm(cv::cuda::StereoSGM* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::StereoSGM::to_StereoMatcher() generated
	// ("cv::cuda::StereoSGM::to_StereoMatcher", vec![(pred!(mut, [], []), _)]),
	cv::StereoMatcher* cv_cuda_StereoSGM_to_StereoMatcher(cv::cuda::StereoSGM* instance) {
			return dynamic_cast<cv::StereoMatcher*>(instance);
	}

	// cv::cuda::StereoSGM::to_StereoSGBM() generated
	// ("cv::cuda::StereoSGM::to_StereoSGBM", vec![(pred!(mut, [], []), _)]),
	cv::StereoSGBM* cv_cuda_StereoSGM_to_StereoSGBM(cv::cuda::StereoSGM* instance) {
			return dynamic_cast<cv::StereoSGBM*>(instance);
	}

	// cv::cuda::StereoSGM::delete() generated
	// ("cv::cuda::StereoSGM::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_StereoSGM_delete(cv::cuda::StereoSGM* instance) {
			delete instance;
	}

}
