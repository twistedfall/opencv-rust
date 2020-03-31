template struct Result<bool>;
template struct Result<cv::CompressedRectilinearPortraitWarper*>;
template struct Result<cv::CompressedRectilinearWarper*>;
template struct Result<cv::InterpolationFlags>;
template struct Result<cv::Mat*>;
template struct Result<cv::PaniniPortraitWarper*>;
template struct Result<cv::PaniniWarper*>;
template struct Result<cv::Point_<float>>;
template struct Result<cv::Point_<int>>;
template struct Result<cv::Ptr<cv::Feature2D>*>;
template struct Result<cv::Ptr<cv::Stitcher>*>;
template struct Result<cv::Ptr<cv::WarperCreator>*>;
template struct Result<cv::Ptr<cv::detail::BestOf2NearestMatcher>*>;
template struct Result<cv::Ptr<cv::detail::Blender>*>;
template struct Result<cv::Ptr<cv::detail::BundleAdjusterBase>*>;
template struct Result<cv::Ptr<cv::detail::Estimator>*>;
template struct Result<cv::Ptr<cv::detail::ExposureCompensator>*>;
template struct Result<cv::Ptr<cv::detail::FeaturesMatcher>*>;
template struct Result<cv::Ptr<cv::detail::RotationWarper>*>;
template struct Result<cv::Ptr<cv::detail::SeamFinder>*>;
template struct Result<cv::PyRotationWarper*>;
template struct Result<cv::Rect_<int>>;
template struct Result<cv::Size_<int>>;
template struct Result<cv::Stitcher::Status>;
template struct Result<cv::TermCriteria>;
template struct Result<cv::UMat*>;
template struct Result<cv::detail::AffineBasedEstimator*>;
template struct Result<cv::detail::AffineBestOf2NearestMatcher*>;
template struct Result<cv::detail::AffineWarper*>;
template struct Result<cv::detail::BestOf2NearestMatcher*>;
template struct Result<cv::detail::BestOf2NearestRangeMatcher*>;
template struct Result<cv::detail::BlocksChannelsCompensator*>;
template struct Result<cv::detail::BlocksGainCompensator*>;
template struct Result<cv::detail::BundleAdjusterAffinePartial*>;
template struct Result<cv::detail::BundleAdjusterAffine*>;
template struct Result<cv::detail::BundleAdjusterRay*>;
template struct Result<cv::detail::BundleAdjusterReproj*>;
template struct Result<cv::detail::CameraParams*>;
template struct Result<cv::detail::ChannelsCompensator*>;
template struct Result<cv::detail::CompressedRectilinearPortraitWarper*>;
template struct Result<cv::detail::CompressedRectilinearWarper*>;
template struct Result<cv::detail::CylindricalPortraitWarper*>;
template struct Result<cv::detail::CylindricalWarperGpu*>;
template struct Result<cv::detail::CylindricalWarper*>;
template struct Result<cv::detail::DisjointSets*>;
template struct Result<cv::detail::DpSeamFinder*>;
template struct Result<cv::detail::DpSeamFinder::CostFunction>;
template struct Result<cv::detail::FeatherBlender*>;
template struct Result<cv::detail::FisheyeWarper*>;
template struct Result<cv::detail::GainCompensator*>;
template struct Result<cv::detail::GraphCutSeamFinder*>;
template struct Result<cv::detail::GraphEdge*>;
template struct Result<cv::detail::Graph*>;
template struct Result<cv::detail::HomographyBasedEstimator*>;
template struct Result<cv::detail::ImageFeatures*>;
template struct Result<cv::detail::MatchesInfo*>;
template struct Result<cv::detail::MercatorWarper*>;
template struct Result<cv::detail::MultiBandBlender*>;
template struct Result<cv::detail::NoBundleAdjuster*>;
template struct Result<cv::detail::PaniniPortraitWarper*>;
template struct Result<cv::detail::PaniniWarper*>;
template struct Result<cv::detail::PlanePortraitWarper*>;
template struct Result<cv::detail::PlaneWarperGpu*>;
template struct Result<cv::detail::PlaneWarper*>;
template struct Result<cv::detail::SphericalPortraitWarper*>;
template struct Result<cv::detail::SphericalWarperGpu*>;
template struct Result<cv::detail::SphericalWarper*>;
template struct Result<cv::detail::StereographicWarper*>;
template struct Result<cv::detail::TransverseMercatorWarper*>;
template struct Result<cv::detail::WaveCorrectKind>;
template struct Result<double>;
template struct Result<float>;
template struct Result<float(*)[3]>;
template struct Result<float(*)[9]>;
template struct Result<int>;
template struct Result<std::vector<cv::DMatch>*>;
template struct Result<std::vector<cv::KeyPoint>*>;
template struct Result<std::vector<cv::Mat>*>;
template struct Result<std::vector<cv::Point_<int>>*>;
template struct Result<std::vector<cv::Scalar_<double>>*>;
template struct Result<std::vector<cv::Size_<int>>*>;
template struct Result<std::vector<cv::UMat>*>;
template struct Result<std::vector<cv::detail::CameraParams>*>;
template struct Result<std::vector<cv::detail::ImageFeatures>*>;
template struct Result<std::vector<cv::detail::MatchesInfo>*>;
template struct Result<std::vector<double>*>;
template struct Result<std::vector<int>*>;
template struct Result<std::vector<std::string>*>;
template struct Result<std::vector<unsigned char>*>;
template struct Result<void*>;
extern "C" void cv_PtrOfDetail_BestOf2NearestMatcher_delete(cv::Ptr<cv::detail::BestOf2NearestMatcher>* instance) {
	delete instance;
}

extern "C" cv::detail::BestOf2NearestMatcher* cv_PtrOfDetail_BestOf2NearestMatcher_get_inner_ptr(cv::Ptr<cv::detail::BestOf2NearestMatcher>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfDetail_Blender_delete(cv::Ptr<cv::detail::Blender>* instance) {
	delete instance;
}

extern "C" cv::detail::Blender* cv_PtrOfDetail_Blender_get_inner_ptr(cv::Ptr<cv::detail::Blender>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfDetail_BundleAdjusterBase_delete(cv::Ptr<cv::detail::BundleAdjusterBase>* instance) {
	delete instance;
}

extern "C" cv::detail::BundleAdjusterBase* cv_PtrOfDetail_BundleAdjusterBase_get_inner_ptr(cv::Ptr<cv::detail::BundleAdjusterBase>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfDetail_Estimator_delete(cv::Ptr<cv::detail::Estimator>* instance) {
	delete instance;
}

extern "C" cv::detail::Estimator* cv_PtrOfDetail_Estimator_get_inner_ptr(cv::Ptr<cv::detail::Estimator>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfDetail_ExposureCompensator_delete(cv::Ptr<cv::detail::ExposureCompensator>* instance) {
	delete instance;
}

extern "C" cv::detail::ExposureCompensator* cv_PtrOfDetail_ExposureCompensator_get_inner_ptr(cv::Ptr<cv::detail::ExposureCompensator>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfDetail_FeaturesMatcher_delete(cv::Ptr<cv::detail::FeaturesMatcher>* instance) {
	delete instance;
}

extern "C" cv::detail::FeaturesMatcher* cv_PtrOfDetail_FeaturesMatcher_get_inner_ptr(cv::Ptr<cv::detail::FeaturesMatcher>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfDetail_RotationWarper_delete(cv::Ptr<cv::detail::RotationWarper>* instance) {
	delete instance;
}

extern "C" cv::detail::RotationWarper* cv_PtrOfDetail_RotationWarper_get_inner_ptr(cv::Ptr<cv::detail::RotationWarper>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfDetail_SeamFinder_delete(cv::Ptr<cv::detail::SeamFinder>* instance) {
	delete instance;
}

extern "C" cv::detail::SeamFinder* cv_PtrOfDetail_SeamFinder_get_inner_ptr(cv::Ptr<cv::detail::SeamFinder>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfStitcher_delete(cv::Ptr<cv::Stitcher>* instance) {
	delete instance;
}

extern "C" cv::Stitcher* cv_PtrOfStitcher_get_inner_ptr(cv::Ptr<cv::Stitcher>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfWarperCreator_delete(cv::Ptr<cv::WarperCreator>* instance) {
	delete instance;
}

extern "C" cv::WarperCreator* cv_PtrOfWarperCreator_get_inner_ptr(cv::Ptr<cv::WarperCreator>* instance) {
	return instance->get();
}

extern "C" {
	void cv_VectorOfDetail_CameraParams_delete(std::vector<cv::detail::CameraParams>* instance) {
		delete instance;
	}

	std::vector<cv::detail::CameraParams>* cv_VectorOfDetail_CameraParams_new() {
		return new std::vector<cv::detail::CameraParams>();
	}

	size_t cv_VectorOfDetail_CameraParams_len(const std::vector<cv::detail::CameraParams>* instance) {
		return instance->size();
	}

	bool cv_VectorOfDetail_CameraParams_is_empty(const std::vector<cv::detail::CameraParams>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfDetail_CameraParams_capacity(const std::vector<cv::detail::CameraParams>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfDetail_CameraParams_shrink_to_fit(std::vector<cv::detail::CameraParams>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfDetail_CameraParams_reserve(std::vector<cv::detail::CameraParams>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfDetail_CameraParams_remove(std::vector<cv::detail::CameraParams>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfDetail_CameraParams_swap(std::vector<cv::detail::CameraParams>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfDetail_CameraParams_clear(std::vector<cv::detail::CameraParams>* instance) {
		instance->clear();
	}

	void cv_VectorOfDetail_CameraParams_push(std::vector<cv::detail::CameraParams>* instance, cv::detail::CameraParams* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfDetail_CameraParams_insert(std::vector<cv::detail::CameraParams>* instance, size_t index, cv::detail::CameraParams* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::detail::CameraParams*> cv_VectorOfDetail_CameraParams_get(const std::vector<cv::detail::CameraParams>* instance, size_t index) {
		return Ok<cv::detail::CameraParams*>(new cv::detail::CameraParams((*instance)[index]));
	}

	void cv_VectorOfDetail_CameraParams_set(std::vector<cv::detail::CameraParams>* instance, size_t index, cv::detail::CameraParams* val) {
		(*instance)[index] = *val;
	}

}


extern "C" {
	void cv_VectorOfDetail_ImageFeatures_delete(std::vector<cv::detail::ImageFeatures>* instance) {
		delete instance;
	}

	std::vector<cv::detail::ImageFeatures>* cv_VectorOfDetail_ImageFeatures_new() {
		return new std::vector<cv::detail::ImageFeatures>();
	}

	size_t cv_VectorOfDetail_ImageFeatures_len(const std::vector<cv::detail::ImageFeatures>* instance) {
		return instance->size();
	}

	bool cv_VectorOfDetail_ImageFeatures_is_empty(const std::vector<cv::detail::ImageFeatures>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfDetail_ImageFeatures_capacity(const std::vector<cv::detail::ImageFeatures>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfDetail_ImageFeatures_shrink_to_fit(std::vector<cv::detail::ImageFeatures>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfDetail_ImageFeatures_reserve(std::vector<cv::detail::ImageFeatures>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfDetail_ImageFeatures_remove(std::vector<cv::detail::ImageFeatures>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfDetail_ImageFeatures_swap(std::vector<cv::detail::ImageFeatures>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfDetail_ImageFeatures_clear(std::vector<cv::detail::ImageFeatures>* instance) {
		instance->clear();
	}

	void cv_VectorOfDetail_ImageFeatures_push(std::vector<cv::detail::ImageFeatures>* instance, cv::detail::ImageFeatures* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfDetail_ImageFeatures_insert(std::vector<cv::detail::ImageFeatures>* instance, size_t index, cv::detail::ImageFeatures* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::detail::ImageFeatures*> cv_VectorOfDetail_ImageFeatures_get(const std::vector<cv::detail::ImageFeatures>* instance, size_t index) {
		return Ok<cv::detail::ImageFeatures*>(new cv::detail::ImageFeatures((*instance)[index]));
	}

	void cv_VectorOfDetail_ImageFeatures_set(std::vector<cv::detail::ImageFeatures>* instance, size_t index, cv::detail::ImageFeatures* val) {
		(*instance)[index] = *val;
	}

}


extern "C" {
	void cv_VectorOfDetail_MatchesInfo_delete(std::vector<cv::detail::MatchesInfo>* instance) {
		delete instance;
	}

	std::vector<cv::detail::MatchesInfo>* cv_VectorOfDetail_MatchesInfo_new() {
		return new std::vector<cv::detail::MatchesInfo>();
	}

	size_t cv_VectorOfDetail_MatchesInfo_len(const std::vector<cv::detail::MatchesInfo>* instance) {
		return instance->size();
	}

	bool cv_VectorOfDetail_MatchesInfo_is_empty(const std::vector<cv::detail::MatchesInfo>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfDetail_MatchesInfo_capacity(const std::vector<cv::detail::MatchesInfo>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfDetail_MatchesInfo_shrink_to_fit(std::vector<cv::detail::MatchesInfo>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfDetail_MatchesInfo_reserve(std::vector<cv::detail::MatchesInfo>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfDetail_MatchesInfo_remove(std::vector<cv::detail::MatchesInfo>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfDetail_MatchesInfo_swap(std::vector<cv::detail::MatchesInfo>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfDetail_MatchesInfo_clear(std::vector<cv::detail::MatchesInfo>* instance) {
		instance->clear();
	}

	void cv_VectorOfDetail_MatchesInfo_push(std::vector<cv::detail::MatchesInfo>* instance, cv::detail::MatchesInfo* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfDetail_MatchesInfo_insert(std::vector<cv::detail::MatchesInfo>* instance, size_t index, cv::detail::MatchesInfo* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::detail::MatchesInfo*> cv_VectorOfDetail_MatchesInfo_get(const std::vector<cv::detail::MatchesInfo>* instance, size_t index) {
		return Ok<cv::detail::MatchesInfo*>(new cv::detail::MatchesInfo((*instance)[index]));
	}

	void cv_VectorOfDetail_MatchesInfo_set(std::vector<cv::detail::MatchesInfo>* instance, size_t index, cv::detail::MatchesInfo* val) {
		(*instance)[index] = *val;
	}

}


