template struct Result<bool>;
template struct Result<cv::Mat*>;
template struct Result<cv::Point_<int>>;
template struct Result<cv::Ptr<cv::AlignMTB>*>;
template struct Result<cv::Ptr<cv::CalibrateDebevec>*>;
template struct Result<cv::Ptr<cv::CalibrateRobertson>*>;
template struct Result<cv::Ptr<cv::MergeDebevec>*>;
template struct Result<cv::Ptr<cv::MergeMertens>*>;
template struct Result<cv::Ptr<cv::MergeRobertson>*>;
template struct Result<cv::Ptr<cv::TonemapDrago>*>;
template struct Result<cv::Ptr<cv::TonemapMantiuk>*>;
template struct Result<cv::Ptr<cv::TonemapReinhard>*>;
template struct Result<cv::Ptr<cv::Tonemap>*>;
template struct Result<float>;
template struct Result<int>;
template struct Result<std::vector<cv::Mat>*>;
template struct Result<std::vector<float>*>;
extern "C" void cv_PtrOfAlignMTB_delete(cv::Ptr<cv::AlignMTB>* instance) {
	delete instance;
}

extern "C" cv::AlignMTB* cv_PtrOfAlignMTB_get_inner_ptr(cv::Ptr<cv::AlignMTB>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfCalibrateDebevec_delete(cv::Ptr<cv::CalibrateDebevec>* instance) {
	delete instance;
}

extern "C" cv::CalibrateDebevec* cv_PtrOfCalibrateDebevec_get_inner_ptr(cv::Ptr<cv::CalibrateDebevec>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfCalibrateRobertson_delete(cv::Ptr<cv::CalibrateRobertson>* instance) {
	delete instance;
}

extern "C" cv::CalibrateRobertson* cv_PtrOfCalibrateRobertson_get_inner_ptr(cv::Ptr<cv::CalibrateRobertson>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfMergeDebevec_delete(cv::Ptr<cv::MergeDebevec>* instance) {
	delete instance;
}

extern "C" cv::MergeDebevec* cv_PtrOfMergeDebevec_get_inner_ptr(cv::Ptr<cv::MergeDebevec>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfMergeMertens_delete(cv::Ptr<cv::MergeMertens>* instance) {
	delete instance;
}

extern "C" cv::MergeMertens* cv_PtrOfMergeMertens_get_inner_ptr(cv::Ptr<cv::MergeMertens>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfMergeRobertson_delete(cv::Ptr<cv::MergeRobertson>* instance) {
	delete instance;
}

extern "C" cv::MergeRobertson* cv_PtrOfMergeRobertson_get_inner_ptr(cv::Ptr<cv::MergeRobertson>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfTonemap_delete(cv::Ptr<cv::Tonemap>* instance) {
	delete instance;
}

extern "C" cv::Tonemap* cv_PtrOfTonemap_get_inner_ptr(cv::Ptr<cv::Tonemap>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfTonemapDrago_delete(cv::Ptr<cv::TonemapDrago>* instance) {
	delete instance;
}

extern "C" cv::TonemapDrago* cv_PtrOfTonemapDrago_get_inner_ptr(cv::Ptr<cv::TonemapDrago>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfTonemapMantiuk_delete(cv::Ptr<cv::TonemapMantiuk>* instance) {
	delete instance;
}

extern "C" cv::TonemapMantiuk* cv_PtrOfTonemapMantiuk_get_inner_ptr(cv::Ptr<cv::TonemapMantiuk>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfTonemapReinhard_delete(cv::Ptr<cv::TonemapReinhard>* instance) {
	delete instance;
}

extern "C" cv::TonemapReinhard* cv_PtrOfTonemapReinhard_get_inner_ptr(cv::Ptr<cv::TonemapReinhard>* instance) {
	return instance->get();
}

