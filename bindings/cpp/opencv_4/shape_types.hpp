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
	
	cv::Ptr<cv::ShapeTransformer>* cv_PtrOfAffineTransformer_to_PtrOfShapeTransformer(cv::Ptr<cv::AffineTransformer>* instance) {
		return new cv::Ptr<cv::ShapeTransformer>(instance->dynamicCast<cv::ShapeTransformer>());
	}
}

extern "C" {
	void cv_PtrOfChiHistogramCostExtractor_delete(cv::Ptr<cv::ChiHistogramCostExtractor>* instance) {
		delete instance;
	}

	const cv::ChiHistogramCostExtractor* cv_PtrOfChiHistogramCostExtractor_get_inner_ptr(const cv::Ptr<cv::ChiHistogramCostExtractor>* instance) {
		return instance->get();
	}

	cv::ChiHistogramCostExtractor* cv_PtrOfChiHistogramCostExtractor_get_inner_ptr_mut(cv::Ptr<cv::ChiHistogramCostExtractor>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::HistogramCostExtractor>* cv_PtrOfChiHistogramCostExtractor_to_PtrOfHistogramCostExtractor(cv::Ptr<cv::ChiHistogramCostExtractor>* instance) {
		return new cv::Ptr<cv::HistogramCostExtractor>(instance->dynamicCast<cv::HistogramCostExtractor>());
	}
}

extern "C" {
	void cv_PtrOfEMDHistogramCostExtractor_delete(cv::Ptr<cv::EMDHistogramCostExtractor>* instance) {
		delete instance;
	}

	const cv::EMDHistogramCostExtractor* cv_PtrOfEMDHistogramCostExtractor_get_inner_ptr(const cv::Ptr<cv::EMDHistogramCostExtractor>* instance) {
		return instance->get();
	}

	cv::EMDHistogramCostExtractor* cv_PtrOfEMDHistogramCostExtractor_get_inner_ptr_mut(cv::Ptr<cv::EMDHistogramCostExtractor>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::HistogramCostExtractor>* cv_PtrOfEMDHistogramCostExtractor_to_PtrOfHistogramCostExtractor(cv::Ptr<cv::EMDHistogramCostExtractor>* instance) {
		return new cv::Ptr<cv::HistogramCostExtractor>(instance->dynamicCast<cv::HistogramCostExtractor>());
	}
}

extern "C" {
	void cv_PtrOfEMDL1HistogramCostExtractor_delete(cv::Ptr<cv::EMDL1HistogramCostExtractor>* instance) {
		delete instance;
	}

	const cv::EMDL1HistogramCostExtractor* cv_PtrOfEMDL1HistogramCostExtractor_get_inner_ptr(const cv::Ptr<cv::EMDL1HistogramCostExtractor>* instance) {
		return instance->get();
	}

	cv::EMDL1HistogramCostExtractor* cv_PtrOfEMDL1HistogramCostExtractor_get_inner_ptr_mut(cv::Ptr<cv::EMDL1HistogramCostExtractor>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::HistogramCostExtractor>* cv_PtrOfEMDL1HistogramCostExtractor_to_PtrOfHistogramCostExtractor(cv::Ptr<cv::EMDL1HistogramCostExtractor>* instance) {
		return new cv::Ptr<cv::HistogramCostExtractor>(instance->dynamicCast<cv::HistogramCostExtractor>());
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
	void cv_PtrOfNormHistogramCostExtractor_delete(cv::Ptr<cv::NormHistogramCostExtractor>* instance) {
		delete instance;
	}

	const cv::NormHistogramCostExtractor* cv_PtrOfNormHistogramCostExtractor_get_inner_ptr(const cv::Ptr<cv::NormHistogramCostExtractor>* instance) {
		return instance->get();
	}

	cv::NormHistogramCostExtractor* cv_PtrOfNormHistogramCostExtractor_get_inner_ptr_mut(cv::Ptr<cv::NormHistogramCostExtractor>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::HistogramCostExtractor>* cv_PtrOfNormHistogramCostExtractor_to_PtrOfHistogramCostExtractor(cv::Ptr<cv::NormHistogramCostExtractor>* instance) {
		return new cv::Ptr<cv::HistogramCostExtractor>(instance->dynamicCast<cv::HistogramCostExtractor>());
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
	
	cv::Ptr<cv::ShapeTransformer>* cv_PtrOfThinPlateSplineShapeTransformer_to_PtrOfShapeTransformer(cv::Ptr<cv::ThinPlateSplineShapeTransformer>* instance) {
		return new cv::Ptr<cv::ShapeTransformer>(instance->dynamicCast<cv::ShapeTransformer>());
	}
}

