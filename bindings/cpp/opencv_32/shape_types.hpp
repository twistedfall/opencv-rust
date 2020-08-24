template struct Result<bool>;
template struct Result<cv::DMatch>;
template struct Result<cv::Ptr<cv::AffineTransformer>*>;
template struct Result<cv::Ptr<cv::HausdorffDistanceExtractor>*>;
template struct Result<cv::Ptr<cv::HistogramCostExtractor>*>;
template struct Result<cv::Ptr<cv::ShapeContextDistanceExtractor>*>;
template struct Result<cv::Ptr<cv::ShapeTransformer>*>;
template struct Result<cv::Ptr<cv::ThinPlateSplineShapeTransformer>*>;
template struct Result<double>;
template struct Result<float>;
template struct Result<int>;
extern "C" {
	void cv_PtrOfAffineTransformer_delete(cv::Ptr<cv::AffineTransformer>* instance) {
		delete instance;
	}

	const cv::AffineTransformer* cv_PtrOfAffineTransformer_get_inner_ptr(const cv::Ptr<cv::AffineTransformer>* instance) {
		return instance->get();
	}

	cv::AffineTransformer* cv_PtrOfAffineTransformer_get_inner_ptr_mut(cv::Ptr<cv::AffineTransformer>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfHausdorffDistanceExtractor_delete(cv::Ptr<cv::HausdorffDistanceExtractor>* instance) {
		delete instance;
	}

	const cv::HausdorffDistanceExtractor* cv_PtrOfHausdorffDistanceExtractor_get_inner_ptr(const cv::Ptr<cv::HausdorffDistanceExtractor>* instance) {
		return instance->get();
	}

	cv::HausdorffDistanceExtractor* cv_PtrOfHausdorffDistanceExtractor_get_inner_ptr_mut(cv::Ptr<cv::HausdorffDistanceExtractor>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfHistogramCostExtractor_delete(cv::Ptr<cv::HistogramCostExtractor>* instance) {
		delete instance;
	}

	const cv::HistogramCostExtractor* cv_PtrOfHistogramCostExtractor_get_inner_ptr(const cv::Ptr<cv::HistogramCostExtractor>* instance) {
		return instance->get();
	}

	cv::HistogramCostExtractor* cv_PtrOfHistogramCostExtractor_get_inner_ptr_mut(cv::Ptr<cv::HistogramCostExtractor>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfShapeContextDistanceExtractor_delete(cv::Ptr<cv::ShapeContextDistanceExtractor>* instance) {
		delete instance;
	}

	const cv::ShapeContextDistanceExtractor* cv_PtrOfShapeContextDistanceExtractor_get_inner_ptr(const cv::Ptr<cv::ShapeContextDistanceExtractor>* instance) {
		return instance->get();
	}

	cv::ShapeContextDistanceExtractor* cv_PtrOfShapeContextDistanceExtractor_get_inner_ptr_mut(cv::Ptr<cv::ShapeContextDistanceExtractor>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfShapeTransformer_delete(cv::Ptr<cv::ShapeTransformer>* instance) {
		delete instance;
	}

	const cv::ShapeTransformer* cv_PtrOfShapeTransformer_get_inner_ptr(const cv::Ptr<cv::ShapeTransformer>* instance) {
		return instance->get();
	}

	cv::ShapeTransformer* cv_PtrOfShapeTransformer_get_inner_ptr_mut(cv::Ptr<cv::ShapeTransformer>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfThinPlateSplineShapeTransformer_delete(cv::Ptr<cv::ThinPlateSplineShapeTransformer>* instance) {
		delete instance;
	}

	const cv::ThinPlateSplineShapeTransformer* cv_PtrOfThinPlateSplineShapeTransformer_get_inner_ptr(const cv::Ptr<cv::ThinPlateSplineShapeTransformer>* instance) {
		return instance->get();
	}

	cv::ThinPlateSplineShapeTransformer* cv_PtrOfThinPlateSplineShapeTransformer_get_inner_ptr_mut(cv::Ptr<cv::ThinPlateSplineShapeTransformer>* instance) {
		return instance->get();
	}
}

