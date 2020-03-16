template struct Result<bool>;
template struct Result<cv::Mat*>;
template struct Result<cv::Ptr<cv::Feature2D>*>;
template struct Result<cv::Ptr<cv::videostab::DeblurerBase>*>;
template struct Result<cv::Ptr<cv::videostab::IDenseOptFlowEstimator>*>;
template struct Result<cv::Ptr<cv::videostab::IFrameSource>*>;
template struct Result<cv::Ptr<cv::videostab::ILog>*>;
template struct Result<cv::Ptr<cv::videostab::IMotionStabilizer>*>;
template struct Result<cv::Ptr<cv::videostab::IOutlierRejector>*>;
template struct Result<cv::Ptr<cv::videostab::ISparseOptFlowEstimator>*>;
template struct Result<cv::Ptr<cv::videostab::ImageMotionEstimatorBase>*>;
template struct Result<cv::Ptr<cv::videostab::InpainterBase>*>;
template struct Result<cv::Ptr<cv::videostab::MotionEstimatorBase>*>;
template struct Result<cv::Ptr<cv::videostab::MotionFilterBase>*>;
template struct Result<cv::Ptr<cv::videostab::WobbleSuppressorBase>*>;
template struct Result<cv::Size_<int>>;
template struct Result<cv::videostab::ColorInpainter*>;
template struct Result<cv::videostab::ConsistentMosaicInpainter*>;
template struct Result<cv::videostab::FastMarchingMethod*>;
template struct Result<cv::videostab::FromFileMotionReader*>;
template struct Result<cv::videostab::GaussianMotionFilter*>;
template struct Result<cv::videostab::KeypointBasedMotionEstimator*>;
template struct Result<cv::videostab::LpMotionStabilizer*>;
template struct Result<cv::videostab::MotionEstimatorL1*>;
template struct Result<cv::videostab::MotionEstimatorRansacL2*>;
template struct Result<cv::videostab::MotionInpainter*>;
template struct Result<cv::videostab::MotionModel>;
template struct Result<cv::videostab::OnePassStabilizer*>;
template struct Result<cv::videostab::PyrLkOptFlowEstimatorBase*>;
template struct Result<cv::videostab::RansacParams*>;
template struct Result<cv::videostab::ToFileMotionWriter*>;
template struct Result<cv::videostab::TranslationBasedLocalOutlierRejector*>;
template struct Result<cv::videostab::TwoPassStabilizer*>;
template struct Result<cv::videostab::VideoFileSource*>;
template struct Result<cv::videostab::WeightingDeblurer*>;
template struct Result<double>;
template struct Result<float>;
template struct Result<int>;
template struct Result<std::vector<cv::Mat>*>;
template struct Result<std::vector<float>*>;
extern "C" void cv_PtrOfDeblurerBase_delete(cv::Ptr<cv::videostab::DeblurerBase>* instance) {
	delete instance;
}

extern "C" cv::videostab::DeblurerBase* cv_PtrOfDeblurerBase_get_inner_ptr(cv::Ptr<cv::videostab::DeblurerBase>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfIDenseOptFlowEstimator_delete(cv::Ptr<cv::videostab::IDenseOptFlowEstimator>* instance) {
	delete instance;
}

extern "C" cv::videostab::IDenseOptFlowEstimator* cv_PtrOfIDenseOptFlowEstimator_get_inner_ptr(cv::Ptr<cv::videostab::IDenseOptFlowEstimator>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfIFrameSource_delete(cv::Ptr<cv::videostab::IFrameSource>* instance) {
	delete instance;
}

extern "C" cv::videostab::IFrameSource* cv_PtrOfIFrameSource_get_inner_ptr(cv::Ptr<cv::videostab::IFrameSource>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfILog_delete(cv::Ptr<cv::videostab::ILog>* instance) {
	delete instance;
}

extern "C" cv::videostab::ILog* cv_PtrOfILog_get_inner_ptr(cv::Ptr<cv::videostab::ILog>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfIMotionStabilizer_delete(cv::Ptr<cv::videostab::IMotionStabilizer>* instance) {
	delete instance;
}

extern "C" cv::videostab::IMotionStabilizer* cv_PtrOfIMotionStabilizer_get_inner_ptr(cv::Ptr<cv::videostab::IMotionStabilizer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfIOutlierRejector_delete(cv::Ptr<cv::videostab::IOutlierRejector>* instance) {
	delete instance;
}

extern "C" cv::videostab::IOutlierRejector* cv_PtrOfIOutlierRejector_get_inner_ptr(cv::Ptr<cv::videostab::IOutlierRejector>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfISparseOptFlowEstimator_delete(cv::Ptr<cv::videostab::ISparseOptFlowEstimator>* instance) {
	delete instance;
}

extern "C" cv::videostab::ISparseOptFlowEstimator* cv_PtrOfISparseOptFlowEstimator_get_inner_ptr(cv::Ptr<cv::videostab::ISparseOptFlowEstimator>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfImageMotionEstimatorBase_delete(cv::Ptr<cv::videostab::ImageMotionEstimatorBase>* instance) {
	delete instance;
}

extern "C" cv::videostab::ImageMotionEstimatorBase* cv_PtrOfImageMotionEstimatorBase_get_inner_ptr(cv::Ptr<cv::videostab::ImageMotionEstimatorBase>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfInpainterBase_delete(cv::Ptr<cv::videostab::InpainterBase>* instance) {
	delete instance;
}

extern "C" cv::videostab::InpainterBase* cv_PtrOfInpainterBase_get_inner_ptr(cv::Ptr<cv::videostab::InpainterBase>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfMotionEstimatorBase_delete(cv::Ptr<cv::videostab::MotionEstimatorBase>* instance) {
	delete instance;
}

extern "C" cv::videostab::MotionEstimatorBase* cv_PtrOfMotionEstimatorBase_get_inner_ptr(cv::Ptr<cv::videostab::MotionEstimatorBase>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfMotionFilterBase_delete(cv::Ptr<cv::videostab::MotionFilterBase>* instance) {
	delete instance;
}

extern "C" cv::videostab::MotionFilterBase* cv_PtrOfMotionFilterBase_get_inner_ptr(cv::Ptr<cv::videostab::MotionFilterBase>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfWobbleSuppressorBase_delete(cv::Ptr<cv::videostab::WobbleSuppressorBase>* instance) {
	delete instance;
}

extern "C" cv::videostab::WobbleSuppressorBase* cv_PtrOfWobbleSuppressorBase_get_inner_ptr(cv::Ptr<cv::videostab::WobbleSuppressorBase>* instance) {
	return instance->get();
}

