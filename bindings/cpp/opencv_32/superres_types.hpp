template struct Result<bool>;
template struct Result<cv::Ptr<cv::superres::BroxOpticalFlow>*>;
template struct Result<cv::Ptr<cv::superres::DenseOpticalFlowExt>*>;
template struct Result<cv::Ptr<cv::superres::DualTVL1OpticalFlow>*>;
template struct Result<cv::Ptr<cv::superres::FarnebackOpticalFlow>*>;
template struct Result<cv::Ptr<cv::superres::FrameSource>*>;
template struct Result<cv::Ptr<cv::superres::PyrLKOpticalFlow>*>;
template struct Result<cv::Ptr<cv::superres::SuperResolution>*>;
template struct Result<double>;
template struct Result<int>;
extern "C" void cv_PtrOfSuperres_BroxOpticalFlow_delete(cv::Ptr<cv::superres::BroxOpticalFlow>* instance) {
	delete instance;
}

extern "C" cv::superres::BroxOpticalFlow* cv_PtrOfSuperres_BroxOpticalFlow_get_inner_ptr(cv::Ptr<cv::superres::BroxOpticalFlow>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfSuperres_DenseOpticalFlowExt_delete(cv::Ptr<cv::superres::DenseOpticalFlowExt>* instance) {
	delete instance;
}

extern "C" cv::superres::DenseOpticalFlowExt* cv_PtrOfSuperres_DenseOpticalFlowExt_get_inner_ptr(cv::Ptr<cv::superres::DenseOpticalFlowExt>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfSuperres_DualTVL1OpticalFlow_delete(cv::Ptr<cv::superres::DualTVL1OpticalFlow>* instance) {
	delete instance;
}

extern "C" cv::superres::DualTVL1OpticalFlow* cv_PtrOfSuperres_DualTVL1OpticalFlow_get_inner_ptr(cv::Ptr<cv::superres::DualTVL1OpticalFlow>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfSuperres_FarnebackOpticalFlow_delete(cv::Ptr<cv::superres::FarnebackOpticalFlow>* instance) {
	delete instance;
}

extern "C" cv::superres::FarnebackOpticalFlow* cv_PtrOfSuperres_FarnebackOpticalFlow_get_inner_ptr(cv::Ptr<cv::superres::FarnebackOpticalFlow>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfSuperres_FrameSource_delete(cv::Ptr<cv::superres::FrameSource>* instance) {
	delete instance;
}

extern "C" cv::superres::FrameSource* cv_PtrOfSuperres_FrameSource_get_inner_ptr(cv::Ptr<cv::superres::FrameSource>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfSuperres_PyrLKOpticalFlow_delete(cv::Ptr<cv::superres::PyrLKOpticalFlow>* instance) {
	delete instance;
}

extern "C" cv::superres::PyrLKOpticalFlow* cv_PtrOfSuperres_PyrLKOpticalFlow_get_inner_ptr(cv::Ptr<cv::superres::PyrLKOpticalFlow>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfSuperres_SuperResolution_delete(cv::Ptr<cv::superres::SuperResolution>* instance) {
	delete instance;
}

extern "C" cv::superres::SuperResolution* cv_PtrOfSuperres_SuperResolution_get_inner_ptr(cv::Ptr<cv::superres::SuperResolution>* instance) {
	return instance->get();
}

