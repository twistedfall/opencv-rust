template struct Result<bool>;
template struct Result<cv::Ptr<cv::AffineTransformer>*>;
template struct Result<cv::Ptr<cv::HausdorffDistanceExtractor>*>;
template struct Result<cv::Ptr<cv::HistogramCostExtractor>*>;
template struct Result<cv::Ptr<cv::ShapeContextDistanceExtractor>*>;
template struct Result<cv::Ptr<cv::ShapeTransformer>*>;
template struct Result<cv::Ptr<cv::ThinPlateSplineShapeTransformer>*>;
template struct Result<double>;
template struct Result<float>;
template struct Result<int>;
template struct Result<std::vector<cv::DMatch>*>;
extern "C" void cv_PtrOfAffineTransformer_delete(cv::Ptr<cv::AffineTransformer>* instance) {
	delete instance;
}

extern "C" cv::AffineTransformer* cv_PtrOfAffineTransformer_get_inner_ptr(cv::Ptr<cv::AffineTransformer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfHausdorffDistanceExtractor_delete(cv::Ptr<cv::HausdorffDistanceExtractor>* instance) {
	delete instance;
}

extern "C" cv::HausdorffDistanceExtractor* cv_PtrOfHausdorffDistanceExtractor_get_inner_ptr(cv::Ptr<cv::HausdorffDistanceExtractor>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfHistogramCostExtractor_delete(cv::Ptr<cv::HistogramCostExtractor>* instance) {
	delete instance;
}

extern "C" cv::HistogramCostExtractor* cv_PtrOfHistogramCostExtractor_get_inner_ptr(cv::Ptr<cv::HistogramCostExtractor>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfShapeContextDistanceExtractor_delete(cv::Ptr<cv::ShapeContextDistanceExtractor>* instance) {
	delete instance;
}

extern "C" cv::ShapeContextDistanceExtractor* cv_PtrOfShapeContextDistanceExtractor_get_inner_ptr(cv::Ptr<cv::ShapeContextDistanceExtractor>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfShapeTransformer_delete(cv::Ptr<cv::ShapeTransformer>* instance) {
	delete instance;
}

extern "C" cv::ShapeTransformer* cv_PtrOfShapeTransformer_get_inner_ptr(cv::Ptr<cv::ShapeTransformer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfThinPlateSplineShapeTransformer_delete(cv::Ptr<cv::ThinPlateSplineShapeTransformer>* instance) {
	delete instance;
}

extern "C" cv::ThinPlateSplineShapeTransformer* cv_PtrOfThinPlateSplineShapeTransformer_get_inner_ptr(cv::Ptr<cv::ThinPlateSplineShapeTransformer>* instance) {
	return instance->get();
}

