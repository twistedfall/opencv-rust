template struct Result<bool>;
template struct Result<double>;
template struct Result<float>;
template struct Result<int>;
template struct Result<void*>;
extern "C" void cv_PtrOfAffineTransformer_delete(cv::Ptr<cv::AffineTransformer>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfAffineTransformer_get_inner_ptr(cv::Ptr<cv::AffineTransformer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfHausdorffDistanceExtractor_delete(cv::Ptr<cv::HausdorffDistanceExtractor>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfHausdorffDistanceExtractor_get_inner_ptr(cv::Ptr<cv::HausdorffDistanceExtractor>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfHistogramCostExtractor_delete(cv::Ptr<cv::HistogramCostExtractor>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfHistogramCostExtractor_get_inner_ptr(cv::Ptr<cv::HistogramCostExtractor>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfShapeContextDistanceExtractor_delete(cv::Ptr<cv::ShapeContextDistanceExtractor>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfShapeContextDistanceExtractor_get_inner_ptr(cv::Ptr<cv::ShapeContextDistanceExtractor>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfShapeTransformer_delete(cv::Ptr<cv::ShapeTransformer>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfShapeTransformer_get_inner_ptr(cv::Ptr<cv::ShapeTransformer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfThinPlateSplineShapeTransformer_delete(cv::Ptr<cv::ThinPlateSplineShapeTransformer>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfThinPlateSplineShapeTransformer_get_inner_ptr(cv::Ptr<cv::ThinPlateSplineShapeTransformer>* instance) {
	return instance->get();
}

