template struct Result<bool>;
template struct Result<const cv::Mat*>;
template struct Result<const cv::Ptr<cv::WarperCreator>*>;
template struct Result<const cv::Ptr<cv::detail::Blender>*>;
template struct Result<const cv::Ptr<cv::detail::BundleAdjusterBase>*>;
template struct Result<const cv::Ptr<cv::detail::ExposureCompensator>*>;
template struct Result<const cv::Ptr<cv::detail::FeaturesFinder>*>;
template struct Result<const cv::Ptr<cv::detail::FeaturesMatcher>*>;
template struct Result<const cv::Ptr<cv::detail::SeamFinder>*>;
template struct Result<const cv::UMat*>;
template struct Result<cv::CompressedRectilinearPortraitWarper*>;
template struct Result<cv::CompressedRectilinearWarper*>;
template struct Result<cv::DMatch>;
template struct Result<cv::KeyPoint>;
template struct Result<cv::Mat*>;
template struct Result<cv::PaniniPortraitWarper*>;
template struct Result<cv::PaniniWarper*>;
template struct Result<cv::Point_<float>>;
template struct Result<cv::Point_<int>>;
template struct Result<cv::Ptr<cv::Stitcher>*>;
template struct Result<cv::Ptr<cv::WarperCreator>*>;
template struct Result<cv::Ptr<cv::detail::Blender>*>;
template struct Result<cv::Ptr<cv::detail::BundleAdjusterBase>*>;
template struct Result<cv::Ptr<cv::detail::ExposureCompensator>*>;
template struct Result<cv::Ptr<cv::detail::FeaturesFinder>*>;
template struct Result<cv::Ptr<cv::detail::FeaturesMatcher>*>;
template struct Result<cv::Ptr<cv::detail::RotationWarper>*>;
template struct Result<cv::Ptr<cv::detail::SeamFinder>*>;
template struct Result<cv::Rect_<int>>;
template struct Result<cv::Size_<int>>;
template struct Result<cv::Stitcher*>;
template struct Result<cv::Stitcher::Status>;
template struct Result<cv::TermCriteria>;
template struct Result<cv::UMat*>;
template struct Result<cv::detail::AKAZEFeaturesFinder*>;
template struct Result<cv::detail::AffineBestOf2NearestMatcher*>;
template struct Result<cv::detail::AffineWarper*>;
template struct Result<cv::detail::BestOf2NearestMatcher*>;
template struct Result<cv::detail::BestOf2NearestRangeMatcher*>;
template struct Result<cv::detail::BlocksGainCompensator*>;
template struct Result<cv::detail::BundleAdjusterAffinePartial*>;
template struct Result<cv::detail::BundleAdjusterAffine*>;
template struct Result<cv::detail::BundleAdjusterRay*>;
template struct Result<cv::detail::BundleAdjusterReproj*>;
template struct Result<cv::detail::CameraParams*>;
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
template struct Result<cv::detail::GraphCutSeamFinderGpu*>;
template struct Result<cv::detail::GraphCutSeamFinder*>;
template struct Result<cv::detail::GraphEdge*>;
template struct Result<cv::detail::Graph*>;
template struct Result<cv::detail::HomographyBasedEstimator*>;
template struct Result<cv::detail::ImageFeatures*>;
template struct Result<cv::detail::MatchesInfo*>;
template struct Result<cv::detail::MercatorWarper*>;
template struct Result<cv::detail::MultiBandBlender*>;
template struct Result<cv::detail::NoBundleAdjuster*>;
template struct Result<cv::detail::OrbFeaturesFinder*>;
template struct Result<cv::detail::PaniniPortraitWarper*>;
template struct Result<cv::detail::PaniniWarper*>;
template struct Result<cv::detail::PlanePortraitWarper*>;
template struct Result<cv::detail::PlaneWarperGpu*>;
template struct Result<cv::detail::PlaneWarper*>;
template struct Result<cv::detail::SiftFeaturesFinder*>;
template struct Result<cv::detail::SphericalPortraitWarper*>;
template struct Result<cv::detail::SphericalWarperGpu*>;
template struct Result<cv::detail::SphericalWarper*>;
template struct Result<cv::detail::StereographicWarper*>;
template struct Result<cv::detail::SurfFeaturesFinderGpu*>;
template struct Result<cv::detail::SurfFeaturesFinder*>;
template struct Result<cv::detail::TransverseMercatorWarper*>;
template struct Result<cv::detail::WaveCorrectKind>;
template struct Result<double>;
template struct Result<float>;
template struct Result<float(*)[3]>;
template struct Result<float(*)[9]>;
template struct Result<int>;
template struct Result<std::vector<cv::DMatch>*>;
template struct Result<std::vector<cv::KeyPoint>*>;
template struct Result<std::vector<cv::Rect_<int>>*>;
template struct Result<std::vector<cv::detail::CameraParams>*>;
template struct Result<std::vector<double>*>;
template struct Result<std::vector<int>*>;
template struct Result<std::vector<unsigned char>*>;
template struct Result<unsigned char>;
template struct Result<void*>;
extern "C" {
	cv::Ptr<cv::AffineWarper>* cv_PtrOfAffineWarper_new(cv::AffineWarper* val) {
		return new cv::Ptr<cv::AffineWarper>(val);
	}
	
	void cv_PtrOfAffineWarper_delete(cv::Ptr<cv::AffineWarper>* instance) {
		delete instance;
	}

	const cv::AffineWarper* cv_PtrOfAffineWarper_get_inner_ptr(const cv::Ptr<cv::AffineWarper>* instance) {
		return instance->get();
	}

	cv::AffineWarper* cv_PtrOfAffineWarper_get_inner_ptr_mut(cv::Ptr<cv::AffineWarper>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrOfAffineWarper_to_PtrOfWarperCreator(cv::Ptr<cv::AffineWarper>* instance) {
		return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
}

extern "C" {
	cv::Ptr<cv::CompressedRectilinearPortraitWarper>* cv_PtrOfCompressedRectilinearPortraitWarper_new(cv::CompressedRectilinearPortraitWarper* val) {
		return new cv::Ptr<cv::CompressedRectilinearPortraitWarper>(val);
	}
	
	void cv_PtrOfCompressedRectilinearPortraitWarper_delete(cv::Ptr<cv::CompressedRectilinearPortraitWarper>* instance) {
		delete instance;
	}

	const cv::CompressedRectilinearPortraitWarper* cv_PtrOfCompressedRectilinearPortraitWarper_get_inner_ptr(const cv::Ptr<cv::CompressedRectilinearPortraitWarper>* instance) {
		return instance->get();
	}

	cv::CompressedRectilinearPortraitWarper* cv_PtrOfCompressedRectilinearPortraitWarper_get_inner_ptr_mut(cv::Ptr<cv::CompressedRectilinearPortraitWarper>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrOfCompressedRectilinearPortraitWarper_to_PtrOfWarperCreator(cv::Ptr<cv::CompressedRectilinearPortraitWarper>* instance) {
		return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
}

extern "C" {
	cv::Ptr<cv::CompressedRectilinearWarper>* cv_PtrOfCompressedRectilinearWarper_new(cv::CompressedRectilinearWarper* val) {
		return new cv::Ptr<cv::CompressedRectilinearWarper>(val);
	}
	
	void cv_PtrOfCompressedRectilinearWarper_delete(cv::Ptr<cv::CompressedRectilinearWarper>* instance) {
		delete instance;
	}

	const cv::CompressedRectilinearWarper* cv_PtrOfCompressedRectilinearWarper_get_inner_ptr(const cv::Ptr<cv::CompressedRectilinearWarper>* instance) {
		return instance->get();
	}

	cv::CompressedRectilinearWarper* cv_PtrOfCompressedRectilinearWarper_get_inner_ptr_mut(cv::Ptr<cv::CompressedRectilinearWarper>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrOfCompressedRectilinearWarper_to_PtrOfWarperCreator(cv::Ptr<cv::CompressedRectilinearWarper>* instance) {
		return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
}

extern "C" {
	cv::Ptr<cv::CylindricalWarper>* cv_PtrOfCylindricalWarper_new(cv::CylindricalWarper* val) {
		return new cv::Ptr<cv::CylindricalWarper>(val);
	}
	
	void cv_PtrOfCylindricalWarper_delete(cv::Ptr<cv::CylindricalWarper>* instance) {
		delete instance;
	}

	const cv::CylindricalWarper* cv_PtrOfCylindricalWarper_get_inner_ptr(const cv::Ptr<cv::CylindricalWarper>* instance) {
		return instance->get();
	}

	cv::CylindricalWarper* cv_PtrOfCylindricalWarper_get_inner_ptr_mut(cv::Ptr<cv::CylindricalWarper>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrOfCylindricalWarper_to_PtrOfWarperCreator(cv::Ptr<cv::CylindricalWarper>* instance) {
		return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
}

extern "C" {
	cv::Ptr<cv::CylindricalWarperGpu>* cv_PtrOfCylindricalWarperGpu_new(cv::CylindricalWarperGpu* val) {
		return new cv::Ptr<cv::CylindricalWarperGpu>(val);
	}
	
	void cv_PtrOfCylindricalWarperGpu_delete(cv::Ptr<cv::CylindricalWarperGpu>* instance) {
		delete instance;
	}

	const cv::CylindricalWarperGpu* cv_PtrOfCylindricalWarperGpu_get_inner_ptr(const cv::Ptr<cv::CylindricalWarperGpu>* instance) {
		return instance->get();
	}

	cv::CylindricalWarperGpu* cv_PtrOfCylindricalWarperGpu_get_inner_ptr_mut(cv::Ptr<cv::CylindricalWarperGpu>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrOfCylindricalWarperGpu_to_PtrOfWarperCreator(cv::Ptr<cv::CylindricalWarperGpu>* instance) {
		return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
}

extern "C" {
	cv::Ptr<cv::detail::AKAZEFeaturesFinder>* cv_PtrOfDetail_AKAZEFeaturesFinder_new(cv::detail::AKAZEFeaturesFinder* val) {
		return new cv::Ptr<cv::detail::AKAZEFeaturesFinder>(val);
	}
	
	void cv_PtrOfDetail_AKAZEFeaturesFinder_delete(cv::Ptr<cv::detail::AKAZEFeaturesFinder>* instance) {
		delete instance;
	}

	const cv::detail::AKAZEFeaturesFinder* cv_PtrOfDetail_AKAZEFeaturesFinder_get_inner_ptr(const cv::Ptr<cv::detail::AKAZEFeaturesFinder>* instance) {
		return instance->get();
	}

	cv::detail::AKAZEFeaturesFinder* cv_PtrOfDetail_AKAZEFeaturesFinder_get_inner_ptr_mut(cv::Ptr<cv::detail::AKAZEFeaturesFinder>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::FeaturesFinder>* cv_PtrOfDetail_AKAZEFeaturesFinder_to_PtrOfDetail_FeaturesFinder(cv::Ptr<cv::detail::AKAZEFeaturesFinder>* instance) {
		return new cv::Ptr<cv::detail::FeaturesFinder>(instance->dynamicCast<cv::detail::FeaturesFinder>());
	}
}

extern "C" {
	cv::Ptr<cv::detail::BestOf2NearestMatcher>* cv_PtrOfDetail_BestOf2NearestMatcher_new(cv::detail::BestOf2NearestMatcher* val) {
		return new cv::Ptr<cv::detail::BestOf2NearestMatcher>(val);
	}
	
	void cv_PtrOfDetail_BestOf2NearestMatcher_delete(cv::Ptr<cv::detail::BestOf2NearestMatcher>* instance) {
		delete instance;
	}

	const cv::detail::BestOf2NearestMatcher* cv_PtrOfDetail_BestOf2NearestMatcher_get_inner_ptr(const cv::Ptr<cv::detail::BestOf2NearestMatcher>* instance) {
		return instance->get();
	}

	cv::detail::BestOf2NearestMatcher* cv_PtrOfDetail_BestOf2NearestMatcher_get_inner_ptr_mut(cv::Ptr<cv::detail::BestOf2NearestMatcher>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::FeaturesMatcher>* cv_PtrOfDetail_BestOf2NearestMatcher_to_PtrOfDetail_FeaturesMatcher(cv::Ptr<cv::detail::BestOf2NearestMatcher>* instance) {
		return new cv::Ptr<cv::detail::FeaturesMatcher>(instance->dynamicCast<cv::detail::FeaturesMatcher>());
	}
}

extern "C" {
	cv::Ptr<cv::detail::Blender>* cv_PtrOfDetail_Blender_new(cv::detail::Blender* val) {
		return new cv::Ptr<cv::detail::Blender>(val);
	}
	
	void cv_PtrOfDetail_Blender_delete(cv::Ptr<cv::detail::Blender>* instance) {
		delete instance;
	}

	const cv::detail::Blender* cv_PtrOfDetail_Blender_get_inner_ptr(const cv::Ptr<cv::detail::Blender>* instance) {
		return instance->get();
	}

	cv::detail::Blender* cv_PtrOfDetail_Blender_get_inner_ptr_mut(cv::Ptr<cv::detail::Blender>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::detail::BlocksGainCompensator>* cv_PtrOfDetail_BlocksGainCompensator_new(cv::detail::BlocksGainCompensator* val) {
		return new cv::Ptr<cv::detail::BlocksGainCompensator>(val);
	}
	
	void cv_PtrOfDetail_BlocksGainCompensator_delete(cv::Ptr<cv::detail::BlocksGainCompensator>* instance) {
		delete instance;
	}

	const cv::detail::BlocksGainCompensator* cv_PtrOfDetail_BlocksGainCompensator_get_inner_ptr(const cv::Ptr<cv::detail::BlocksGainCompensator>* instance) {
		return instance->get();
	}

	cv::detail::BlocksGainCompensator* cv_PtrOfDetail_BlocksGainCompensator_get_inner_ptr_mut(cv::Ptr<cv::detail::BlocksGainCompensator>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::ExposureCompensator>* cv_PtrOfDetail_BlocksGainCompensator_to_PtrOfDetail_ExposureCompensator(cv::Ptr<cv::detail::BlocksGainCompensator>* instance) {
		return new cv::Ptr<cv::detail::ExposureCompensator>(instance->dynamicCast<cv::detail::ExposureCompensator>());
	}
}

extern "C" {
	cv::Ptr<cv::detail::BundleAdjusterAffine>* cv_PtrOfDetail_BundleAdjusterAffine_new(cv::detail::BundleAdjusterAffine* val) {
		return new cv::Ptr<cv::detail::BundleAdjusterAffine>(val);
	}
	
	void cv_PtrOfDetail_BundleAdjusterAffine_delete(cv::Ptr<cv::detail::BundleAdjusterAffine>* instance) {
		delete instance;
	}

	const cv::detail::BundleAdjusterAffine* cv_PtrOfDetail_BundleAdjusterAffine_get_inner_ptr(const cv::Ptr<cv::detail::BundleAdjusterAffine>* instance) {
		return instance->get();
	}

	cv::detail::BundleAdjusterAffine* cv_PtrOfDetail_BundleAdjusterAffine_get_inner_ptr_mut(cv::Ptr<cv::detail::BundleAdjusterAffine>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::BundleAdjusterBase>* cv_PtrOfDetail_BundleAdjusterAffine_to_PtrOfDetail_BundleAdjusterBase(cv::Ptr<cv::detail::BundleAdjusterAffine>* instance) {
		return new cv::Ptr<cv::detail::BundleAdjusterBase>(instance->dynamicCast<cv::detail::BundleAdjusterBase>());
	}
}

extern "C" {
	cv::Ptr<cv::detail::BundleAdjusterAffinePartial>* cv_PtrOfDetail_BundleAdjusterAffinePartial_new(cv::detail::BundleAdjusterAffinePartial* val) {
		return new cv::Ptr<cv::detail::BundleAdjusterAffinePartial>(val);
	}
	
	void cv_PtrOfDetail_BundleAdjusterAffinePartial_delete(cv::Ptr<cv::detail::BundleAdjusterAffinePartial>* instance) {
		delete instance;
	}

	const cv::detail::BundleAdjusterAffinePartial* cv_PtrOfDetail_BundleAdjusterAffinePartial_get_inner_ptr(const cv::Ptr<cv::detail::BundleAdjusterAffinePartial>* instance) {
		return instance->get();
	}

	cv::detail::BundleAdjusterAffinePartial* cv_PtrOfDetail_BundleAdjusterAffinePartial_get_inner_ptr_mut(cv::Ptr<cv::detail::BundleAdjusterAffinePartial>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::BundleAdjusterBase>* cv_PtrOfDetail_BundleAdjusterAffinePartial_to_PtrOfDetail_BundleAdjusterBase(cv::Ptr<cv::detail::BundleAdjusterAffinePartial>* instance) {
		return new cv::Ptr<cv::detail::BundleAdjusterBase>(instance->dynamicCast<cv::detail::BundleAdjusterBase>());
	}
}

extern "C" {
	void cv_PtrOfDetail_BundleAdjusterBase_delete(cv::Ptr<cv::detail::BundleAdjusterBase>* instance) {
		delete instance;
	}

	const cv::detail::BundleAdjusterBase* cv_PtrOfDetail_BundleAdjusterBase_get_inner_ptr(const cv::Ptr<cv::detail::BundleAdjusterBase>* instance) {
		return instance->get();
	}

	cv::detail::BundleAdjusterBase* cv_PtrOfDetail_BundleAdjusterBase_get_inner_ptr_mut(cv::Ptr<cv::detail::BundleAdjusterBase>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::detail::BundleAdjusterRay>* cv_PtrOfDetail_BundleAdjusterRay_new(cv::detail::BundleAdjusterRay* val) {
		return new cv::Ptr<cv::detail::BundleAdjusterRay>(val);
	}
	
	void cv_PtrOfDetail_BundleAdjusterRay_delete(cv::Ptr<cv::detail::BundleAdjusterRay>* instance) {
		delete instance;
	}

	const cv::detail::BundleAdjusterRay* cv_PtrOfDetail_BundleAdjusterRay_get_inner_ptr(const cv::Ptr<cv::detail::BundleAdjusterRay>* instance) {
		return instance->get();
	}

	cv::detail::BundleAdjusterRay* cv_PtrOfDetail_BundleAdjusterRay_get_inner_ptr_mut(cv::Ptr<cv::detail::BundleAdjusterRay>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::BundleAdjusterBase>* cv_PtrOfDetail_BundleAdjusterRay_to_PtrOfDetail_BundleAdjusterBase(cv::Ptr<cv::detail::BundleAdjusterRay>* instance) {
		return new cv::Ptr<cv::detail::BundleAdjusterBase>(instance->dynamicCast<cv::detail::BundleAdjusterBase>());
	}
}

extern "C" {
	cv::Ptr<cv::detail::BundleAdjusterReproj>* cv_PtrOfDetail_BundleAdjusterReproj_new(cv::detail::BundleAdjusterReproj* val) {
		return new cv::Ptr<cv::detail::BundleAdjusterReproj>(val);
	}
	
	void cv_PtrOfDetail_BundleAdjusterReproj_delete(cv::Ptr<cv::detail::BundleAdjusterReproj>* instance) {
		delete instance;
	}

	const cv::detail::BundleAdjusterReproj* cv_PtrOfDetail_BundleAdjusterReproj_get_inner_ptr(const cv::Ptr<cv::detail::BundleAdjusterReproj>* instance) {
		return instance->get();
	}

	cv::detail::BundleAdjusterReproj* cv_PtrOfDetail_BundleAdjusterReproj_get_inner_ptr_mut(cv::Ptr<cv::detail::BundleAdjusterReproj>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::BundleAdjusterBase>* cv_PtrOfDetail_BundleAdjusterReproj_to_PtrOfDetail_BundleAdjusterBase(cv::Ptr<cv::detail::BundleAdjusterReproj>* instance) {
		return new cv::Ptr<cv::detail::BundleAdjusterBase>(instance->dynamicCast<cv::detail::BundleAdjusterBase>());
	}
}

extern "C" {
	cv::Ptr<cv::detail::DpSeamFinder>* cv_PtrOfDetail_DpSeamFinder_new(cv::detail::DpSeamFinder* val) {
		return new cv::Ptr<cv::detail::DpSeamFinder>(val);
	}
	
	void cv_PtrOfDetail_DpSeamFinder_delete(cv::Ptr<cv::detail::DpSeamFinder>* instance) {
		delete instance;
	}

	const cv::detail::DpSeamFinder* cv_PtrOfDetail_DpSeamFinder_get_inner_ptr(const cv::Ptr<cv::detail::DpSeamFinder>* instance) {
		return instance->get();
	}

	cv::detail::DpSeamFinder* cv_PtrOfDetail_DpSeamFinder_get_inner_ptr_mut(cv::Ptr<cv::detail::DpSeamFinder>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::SeamFinder>* cv_PtrOfDetail_DpSeamFinder_to_PtrOfDetail_SeamFinder(cv::Ptr<cv::detail::DpSeamFinder>* instance) {
		return new cv::Ptr<cv::detail::SeamFinder>(instance->dynamicCast<cv::detail::SeamFinder>());
	}
}

extern "C" {
	void cv_PtrOfDetail_ExposureCompensator_delete(cv::Ptr<cv::detail::ExposureCompensator>* instance) {
		delete instance;
	}

	const cv::detail::ExposureCompensator* cv_PtrOfDetail_ExposureCompensator_get_inner_ptr(const cv::Ptr<cv::detail::ExposureCompensator>* instance) {
		return instance->get();
	}

	cv::detail::ExposureCompensator* cv_PtrOfDetail_ExposureCompensator_get_inner_ptr_mut(cv::Ptr<cv::detail::ExposureCompensator>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::detail::FeatherBlender>* cv_PtrOfDetail_FeatherBlender_new(cv::detail::FeatherBlender* val) {
		return new cv::Ptr<cv::detail::FeatherBlender>(val);
	}
	
	void cv_PtrOfDetail_FeatherBlender_delete(cv::Ptr<cv::detail::FeatherBlender>* instance) {
		delete instance;
	}

	const cv::detail::FeatherBlender* cv_PtrOfDetail_FeatherBlender_get_inner_ptr(const cv::Ptr<cv::detail::FeatherBlender>* instance) {
		return instance->get();
	}

	cv::detail::FeatherBlender* cv_PtrOfDetail_FeatherBlender_get_inner_ptr_mut(cv::Ptr<cv::detail::FeatherBlender>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::Blender>* cv_PtrOfDetail_FeatherBlender_to_PtrOfDetail_Blender(cv::Ptr<cv::detail::FeatherBlender>* instance) {
		return new cv::Ptr<cv::detail::Blender>(instance->dynamicCast<cv::detail::Blender>());
	}
}

extern "C" {
	void cv_PtrOfDetail_FeaturesFinder_delete(cv::Ptr<cv::detail::FeaturesFinder>* instance) {
		delete instance;
	}

	const cv::detail::FeaturesFinder* cv_PtrOfDetail_FeaturesFinder_get_inner_ptr(const cv::Ptr<cv::detail::FeaturesFinder>* instance) {
		return instance->get();
	}

	cv::detail::FeaturesFinder* cv_PtrOfDetail_FeaturesFinder_get_inner_ptr_mut(cv::Ptr<cv::detail::FeaturesFinder>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfDetail_FeaturesMatcher_delete(cv::Ptr<cv::detail::FeaturesMatcher>* instance) {
		delete instance;
	}

	const cv::detail::FeaturesMatcher* cv_PtrOfDetail_FeaturesMatcher_get_inner_ptr(const cv::Ptr<cv::detail::FeaturesMatcher>* instance) {
		return instance->get();
	}

	cv::detail::FeaturesMatcher* cv_PtrOfDetail_FeaturesMatcher_get_inner_ptr_mut(cv::Ptr<cv::detail::FeaturesMatcher>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::detail::GainCompensator>* cv_PtrOfDetail_GainCompensator_new(cv::detail::GainCompensator* val) {
		return new cv::Ptr<cv::detail::GainCompensator>(val);
	}
	
	void cv_PtrOfDetail_GainCompensator_delete(cv::Ptr<cv::detail::GainCompensator>* instance) {
		delete instance;
	}

	const cv::detail::GainCompensator* cv_PtrOfDetail_GainCompensator_get_inner_ptr(const cv::Ptr<cv::detail::GainCompensator>* instance) {
		return instance->get();
	}

	cv::detail::GainCompensator* cv_PtrOfDetail_GainCompensator_get_inner_ptr_mut(cv::Ptr<cv::detail::GainCompensator>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::ExposureCompensator>* cv_PtrOfDetail_GainCompensator_to_PtrOfDetail_ExposureCompensator(cv::Ptr<cv::detail::GainCompensator>* instance) {
		return new cv::Ptr<cv::detail::ExposureCompensator>(instance->dynamicCast<cv::detail::ExposureCompensator>());
	}
}

extern "C" {
	cv::Ptr<cv::detail::GraphCutSeamFinder>* cv_PtrOfDetail_GraphCutSeamFinder_new(cv::detail::GraphCutSeamFinder* val) {
		return new cv::Ptr<cv::detail::GraphCutSeamFinder>(val);
	}
	
	void cv_PtrOfDetail_GraphCutSeamFinder_delete(cv::Ptr<cv::detail::GraphCutSeamFinder>* instance) {
		delete instance;
	}

	const cv::detail::GraphCutSeamFinder* cv_PtrOfDetail_GraphCutSeamFinder_get_inner_ptr(const cv::Ptr<cv::detail::GraphCutSeamFinder>* instance) {
		return instance->get();
	}

	cv::detail::GraphCutSeamFinder* cv_PtrOfDetail_GraphCutSeamFinder_get_inner_ptr_mut(cv::Ptr<cv::detail::GraphCutSeamFinder>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::SeamFinder>* cv_PtrOfDetail_GraphCutSeamFinder_to_PtrOfDetail_SeamFinder(cv::Ptr<cv::detail::GraphCutSeamFinder>* instance) {
		return new cv::Ptr<cv::detail::SeamFinder>(instance->dynamicCast<cv::detail::SeamFinder>());
	}
}

extern "C" {
	cv::Ptr<cv::detail::MultiBandBlender>* cv_PtrOfDetail_MultiBandBlender_new(cv::detail::MultiBandBlender* val) {
		return new cv::Ptr<cv::detail::MultiBandBlender>(val);
	}
	
	void cv_PtrOfDetail_MultiBandBlender_delete(cv::Ptr<cv::detail::MultiBandBlender>* instance) {
		delete instance;
	}

	const cv::detail::MultiBandBlender* cv_PtrOfDetail_MultiBandBlender_get_inner_ptr(const cv::Ptr<cv::detail::MultiBandBlender>* instance) {
		return instance->get();
	}

	cv::detail::MultiBandBlender* cv_PtrOfDetail_MultiBandBlender_get_inner_ptr_mut(cv::Ptr<cv::detail::MultiBandBlender>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::Blender>* cv_PtrOfDetail_MultiBandBlender_to_PtrOfDetail_Blender(cv::Ptr<cv::detail::MultiBandBlender>* instance) {
		return new cv::Ptr<cv::detail::Blender>(instance->dynamicCast<cv::detail::Blender>());
	}
}

extern "C" {
	cv::Ptr<cv::detail::NoBundleAdjuster>* cv_PtrOfDetail_NoBundleAdjuster_new(cv::detail::NoBundleAdjuster* val) {
		return new cv::Ptr<cv::detail::NoBundleAdjuster>(val);
	}
	
	void cv_PtrOfDetail_NoBundleAdjuster_delete(cv::Ptr<cv::detail::NoBundleAdjuster>* instance) {
		delete instance;
	}

	const cv::detail::NoBundleAdjuster* cv_PtrOfDetail_NoBundleAdjuster_get_inner_ptr(const cv::Ptr<cv::detail::NoBundleAdjuster>* instance) {
		return instance->get();
	}

	cv::detail::NoBundleAdjuster* cv_PtrOfDetail_NoBundleAdjuster_get_inner_ptr_mut(cv::Ptr<cv::detail::NoBundleAdjuster>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::BundleAdjusterBase>* cv_PtrOfDetail_NoBundleAdjuster_to_PtrOfDetail_BundleAdjusterBase(cv::Ptr<cv::detail::NoBundleAdjuster>* instance) {
		return new cv::Ptr<cv::detail::BundleAdjusterBase>(instance->dynamicCast<cv::detail::BundleAdjusterBase>());
	}
}

extern "C" {
	cv::Ptr<cv::detail::NoExposureCompensator>* cv_PtrOfDetail_NoExposureCompensator_new(cv::detail::NoExposureCompensator* val) {
		return new cv::Ptr<cv::detail::NoExposureCompensator>(val);
	}
	
	void cv_PtrOfDetail_NoExposureCompensator_delete(cv::Ptr<cv::detail::NoExposureCompensator>* instance) {
		delete instance;
	}

	const cv::detail::NoExposureCompensator* cv_PtrOfDetail_NoExposureCompensator_get_inner_ptr(const cv::Ptr<cv::detail::NoExposureCompensator>* instance) {
		return instance->get();
	}

	cv::detail::NoExposureCompensator* cv_PtrOfDetail_NoExposureCompensator_get_inner_ptr_mut(cv::Ptr<cv::detail::NoExposureCompensator>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::ExposureCompensator>* cv_PtrOfDetail_NoExposureCompensator_to_PtrOfDetail_ExposureCompensator(cv::Ptr<cv::detail::NoExposureCompensator>* instance) {
		return new cv::Ptr<cv::detail::ExposureCompensator>(instance->dynamicCast<cv::detail::ExposureCompensator>());
	}
}

extern "C" {
	cv::Ptr<cv::detail::NoSeamFinder>* cv_PtrOfDetail_NoSeamFinder_new(cv::detail::NoSeamFinder* val) {
		return new cv::Ptr<cv::detail::NoSeamFinder>(val);
	}
	
	void cv_PtrOfDetail_NoSeamFinder_delete(cv::Ptr<cv::detail::NoSeamFinder>* instance) {
		delete instance;
	}

	const cv::detail::NoSeamFinder* cv_PtrOfDetail_NoSeamFinder_get_inner_ptr(const cv::Ptr<cv::detail::NoSeamFinder>* instance) {
		return instance->get();
	}

	cv::detail::NoSeamFinder* cv_PtrOfDetail_NoSeamFinder_get_inner_ptr_mut(cv::Ptr<cv::detail::NoSeamFinder>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::SeamFinder>* cv_PtrOfDetail_NoSeamFinder_to_PtrOfDetail_SeamFinder(cv::Ptr<cv::detail::NoSeamFinder>* instance) {
		return new cv::Ptr<cv::detail::SeamFinder>(instance->dynamicCast<cv::detail::SeamFinder>());
	}
}

extern "C" {
	cv::Ptr<cv::detail::OrbFeaturesFinder>* cv_PtrOfDetail_OrbFeaturesFinder_new(cv::detail::OrbFeaturesFinder* val) {
		return new cv::Ptr<cv::detail::OrbFeaturesFinder>(val);
	}
	
	void cv_PtrOfDetail_OrbFeaturesFinder_delete(cv::Ptr<cv::detail::OrbFeaturesFinder>* instance) {
		delete instance;
	}

	const cv::detail::OrbFeaturesFinder* cv_PtrOfDetail_OrbFeaturesFinder_get_inner_ptr(const cv::Ptr<cv::detail::OrbFeaturesFinder>* instance) {
		return instance->get();
	}

	cv::detail::OrbFeaturesFinder* cv_PtrOfDetail_OrbFeaturesFinder_get_inner_ptr_mut(cv::Ptr<cv::detail::OrbFeaturesFinder>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::FeaturesFinder>* cv_PtrOfDetail_OrbFeaturesFinder_to_PtrOfDetail_FeaturesFinder(cv::Ptr<cv::detail::OrbFeaturesFinder>* instance) {
		return new cv::Ptr<cv::detail::FeaturesFinder>(instance->dynamicCast<cv::detail::FeaturesFinder>());
	}
}

extern "C" {
	void cv_PtrOfDetail_PairwiseSeamFinder_delete(cv::Ptr<cv::detail::PairwiseSeamFinder>* instance) {
		delete instance;
	}

	const cv::detail::PairwiseSeamFinder* cv_PtrOfDetail_PairwiseSeamFinder_get_inner_ptr(const cv::Ptr<cv::detail::PairwiseSeamFinder>* instance) {
		return instance->get();
	}

	cv::detail::PairwiseSeamFinder* cv_PtrOfDetail_PairwiseSeamFinder_get_inner_ptr_mut(cv::Ptr<cv::detail::PairwiseSeamFinder>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::SeamFinder>* cv_PtrOfDetail_PairwiseSeamFinder_to_PtrOfDetail_SeamFinder(cv::Ptr<cv::detail::PairwiseSeamFinder>* instance) {
		return new cv::Ptr<cv::detail::SeamFinder>(instance->dynamicCast<cv::detail::SeamFinder>());
	}
}

extern "C" {
	void cv_PtrOfDetail_RotationWarper_delete(cv::Ptr<cv::detail::RotationWarper>* instance) {
		delete instance;
	}

	const cv::detail::RotationWarper* cv_PtrOfDetail_RotationWarper_get_inner_ptr(const cv::Ptr<cv::detail::RotationWarper>* instance) {
		return instance->get();
	}

	cv::detail::RotationWarper* cv_PtrOfDetail_RotationWarper_get_inner_ptr_mut(cv::Ptr<cv::detail::RotationWarper>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfDetail_SeamFinder_delete(cv::Ptr<cv::detail::SeamFinder>* instance) {
		delete instance;
	}

	const cv::detail::SeamFinder* cv_PtrOfDetail_SeamFinder_get_inner_ptr(const cv::Ptr<cv::detail::SeamFinder>* instance) {
		return instance->get();
	}

	cv::detail::SeamFinder* cv_PtrOfDetail_SeamFinder_get_inner_ptr_mut(cv::Ptr<cv::detail::SeamFinder>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::detail::SiftFeaturesFinder>* cv_PtrOfDetail_SiftFeaturesFinder_new(cv::detail::SiftFeaturesFinder* val) {
		return new cv::Ptr<cv::detail::SiftFeaturesFinder>(val);
	}
	
	void cv_PtrOfDetail_SiftFeaturesFinder_delete(cv::Ptr<cv::detail::SiftFeaturesFinder>* instance) {
		delete instance;
	}

	const cv::detail::SiftFeaturesFinder* cv_PtrOfDetail_SiftFeaturesFinder_get_inner_ptr(const cv::Ptr<cv::detail::SiftFeaturesFinder>* instance) {
		return instance->get();
	}

	cv::detail::SiftFeaturesFinder* cv_PtrOfDetail_SiftFeaturesFinder_get_inner_ptr_mut(cv::Ptr<cv::detail::SiftFeaturesFinder>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::FeaturesFinder>* cv_PtrOfDetail_SiftFeaturesFinder_to_PtrOfDetail_FeaturesFinder(cv::Ptr<cv::detail::SiftFeaturesFinder>* instance) {
		return new cv::Ptr<cv::detail::FeaturesFinder>(instance->dynamicCast<cv::detail::FeaturesFinder>());
	}
}

extern "C" {
	cv::Ptr<cv::detail::SurfFeaturesFinder>* cv_PtrOfDetail_SurfFeaturesFinder_new(cv::detail::SurfFeaturesFinder* val) {
		return new cv::Ptr<cv::detail::SurfFeaturesFinder>(val);
	}
	
	void cv_PtrOfDetail_SurfFeaturesFinder_delete(cv::Ptr<cv::detail::SurfFeaturesFinder>* instance) {
		delete instance;
	}

	const cv::detail::SurfFeaturesFinder* cv_PtrOfDetail_SurfFeaturesFinder_get_inner_ptr(const cv::Ptr<cv::detail::SurfFeaturesFinder>* instance) {
		return instance->get();
	}

	cv::detail::SurfFeaturesFinder* cv_PtrOfDetail_SurfFeaturesFinder_get_inner_ptr_mut(cv::Ptr<cv::detail::SurfFeaturesFinder>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::FeaturesFinder>* cv_PtrOfDetail_SurfFeaturesFinder_to_PtrOfDetail_FeaturesFinder(cv::Ptr<cv::detail::SurfFeaturesFinder>* instance) {
		return new cv::Ptr<cv::detail::FeaturesFinder>(instance->dynamicCast<cv::detail::FeaturesFinder>());
	}
}

extern "C" {
	cv::Ptr<cv::detail::SurfFeaturesFinderGpu>* cv_PtrOfDetail_SurfFeaturesFinderGpu_new(cv::detail::SurfFeaturesFinderGpu* val) {
		return new cv::Ptr<cv::detail::SurfFeaturesFinderGpu>(val);
	}
	
	void cv_PtrOfDetail_SurfFeaturesFinderGpu_delete(cv::Ptr<cv::detail::SurfFeaturesFinderGpu>* instance) {
		delete instance;
	}

	const cv::detail::SurfFeaturesFinderGpu* cv_PtrOfDetail_SurfFeaturesFinderGpu_get_inner_ptr(const cv::Ptr<cv::detail::SurfFeaturesFinderGpu>* instance) {
		return instance->get();
	}

	cv::detail::SurfFeaturesFinderGpu* cv_PtrOfDetail_SurfFeaturesFinderGpu_get_inner_ptr_mut(cv::Ptr<cv::detail::SurfFeaturesFinderGpu>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::FeaturesFinder>* cv_PtrOfDetail_SurfFeaturesFinderGpu_to_PtrOfDetail_FeaturesFinder(cv::Ptr<cv::detail::SurfFeaturesFinderGpu>* instance) {
		return new cv::Ptr<cv::detail::FeaturesFinder>(instance->dynamicCast<cv::detail::FeaturesFinder>());
	}
}

extern "C" {
	cv::Ptr<cv::FisheyeWarper>* cv_PtrOfFisheyeWarper_new(cv::FisheyeWarper* val) {
		return new cv::Ptr<cv::FisheyeWarper>(val);
	}
	
	void cv_PtrOfFisheyeWarper_delete(cv::Ptr<cv::FisheyeWarper>* instance) {
		delete instance;
	}

	const cv::FisheyeWarper* cv_PtrOfFisheyeWarper_get_inner_ptr(const cv::Ptr<cv::FisheyeWarper>* instance) {
		return instance->get();
	}

	cv::FisheyeWarper* cv_PtrOfFisheyeWarper_get_inner_ptr_mut(cv::Ptr<cv::FisheyeWarper>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrOfFisheyeWarper_to_PtrOfWarperCreator(cv::Ptr<cv::FisheyeWarper>* instance) {
		return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
}

extern "C" {
	cv::Ptr<cv::MercatorWarper>* cv_PtrOfMercatorWarper_new(cv::MercatorWarper* val) {
		return new cv::Ptr<cv::MercatorWarper>(val);
	}
	
	void cv_PtrOfMercatorWarper_delete(cv::Ptr<cv::MercatorWarper>* instance) {
		delete instance;
	}

	const cv::MercatorWarper* cv_PtrOfMercatorWarper_get_inner_ptr(const cv::Ptr<cv::MercatorWarper>* instance) {
		return instance->get();
	}

	cv::MercatorWarper* cv_PtrOfMercatorWarper_get_inner_ptr_mut(cv::Ptr<cv::MercatorWarper>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrOfMercatorWarper_to_PtrOfWarperCreator(cv::Ptr<cv::MercatorWarper>* instance) {
		return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
}

extern "C" {
	cv::Ptr<cv::PaniniPortraitWarper>* cv_PtrOfPaniniPortraitWarper_new(cv::PaniniPortraitWarper* val) {
		return new cv::Ptr<cv::PaniniPortraitWarper>(val);
	}
	
	void cv_PtrOfPaniniPortraitWarper_delete(cv::Ptr<cv::PaniniPortraitWarper>* instance) {
		delete instance;
	}

	const cv::PaniniPortraitWarper* cv_PtrOfPaniniPortraitWarper_get_inner_ptr(const cv::Ptr<cv::PaniniPortraitWarper>* instance) {
		return instance->get();
	}

	cv::PaniniPortraitWarper* cv_PtrOfPaniniPortraitWarper_get_inner_ptr_mut(cv::Ptr<cv::PaniniPortraitWarper>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrOfPaniniPortraitWarper_to_PtrOfWarperCreator(cv::Ptr<cv::PaniniPortraitWarper>* instance) {
		return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
}

extern "C" {
	cv::Ptr<cv::PaniniWarper>* cv_PtrOfPaniniWarper_new(cv::PaniniWarper* val) {
		return new cv::Ptr<cv::PaniniWarper>(val);
	}
	
	void cv_PtrOfPaniniWarper_delete(cv::Ptr<cv::PaniniWarper>* instance) {
		delete instance;
	}

	const cv::PaniniWarper* cv_PtrOfPaniniWarper_get_inner_ptr(const cv::Ptr<cv::PaniniWarper>* instance) {
		return instance->get();
	}

	cv::PaniniWarper* cv_PtrOfPaniniWarper_get_inner_ptr_mut(cv::Ptr<cv::PaniniWarper>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrOfPaniniWarper_to_PtrOfWarperCreator(cv::Ptr<cv::PaniniWarper>* instance) {
		return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
}

extern "C" {
	cv::Ptr<cv::PlaneWarper>* cv_PtrOfPlaneWarper_new(cv::PlaneWarper* val) {
		return new cv::Ptr<cv::PlaneWarper>(val);
	}
	
	void cv_PtrOfPlaneWarper_delete(cv::Ptr<cv::PlaneWarper>* instance) {
		delete instance;
	}

	const cv::PlaneWarper* cv_PtrOfPlaneWarper_get_inner_ptr(const cv::Ptr<cv::PlaneWarper>* instance) {
		return instance->get();
	}

	cv::PlaneWarper* cv_PtrOfPlaneWarper_get_inner_ptr_mut(cv::Ptr<cv::PlaneWarper>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrOfPlaneWarper_to_PtrOfWarperCreator(cv::Ptr<cv::PlaneWarper>* instance) {
		return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
}

extern "C" {
	cv::Ptr<cv::PlaneWarperGpu>* cv_PtrOfPlaneWarperGpu_new(cv::PlaneWarperGpu* val) {
		return new cv::Ptr<cv::PlaneWarperGpu>(val);
	}
	
	void cv_PtrOfPlaneWarperGpu_delete(cv::Ptr<cv::PlaneWarperGpu>* instance) {
		delete instance;
	}

	const cv::PlaneWarperGpu* cv_PtrOfPlaneWarperGpu_get_inner_ptr(const cv::Ptr<cv::PlaneWarperGpu>* instance) {
		return instance->get();
	}

	cv::PlaneWarperGpu* cv_PtrOfPlaneWarperGpu_get_inner_ptr_mut(cv::Ptr<cv::PlaneWarperGpu>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrOfPlaneWarperGpu_to_PtrOfWarperCreator(cv::Ptr<cv::PlaneWarperGpu>* instance) {
		return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
}

extern "C" {
	cv::Ptr<cv::SphericalWarper>* cv_PtrOfSphericalWarper_new(cv::SphericalWarper* val) {
		return new cv::Ptr<cv::SphericalWarper>(val);
	}
	
	void cv_PtrOfSphericalWarper_delete(cv::Ptr<cv::SphericalWarper>* instance) {
		delete instance;
	}

	const cv::SphericalWarper* cv_PtrOfSphericalWarper_get_inner_ptr(const cv::Ptr<cv::SphericalWarper>* instance) {
		return instance->get();
	}

	cv::SphericalWarper* cv_PtrOfSphericalWarper_get_inner_ptr_mut(cv::Ptr<cv::SphericalWarper>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrOfSphericalWarper_to_PtrOfWarperCreator(cv::Ptr<cv::SphericalWarper>* instance) {
		return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
}

extern "C" {
	cv::Ptr<cv::SphericalWarperGpu>* cv_PtrOfSphericalWarperGpu_new(cv::SphericalWarperGpu* val) {
		return new cv::Ptr<cv::SphericalWarperGpu>(val);
	}
	
	void cv_PtrOfSphericalWarperGpu_delete(cv::Ptr<cv::SphericalWarperGpu>* instance) {
		delete instance;
	}

	const cv::SphericalWarperGpu* cv_PtrOfSphericalWarperGpu_get_inner_ptr(const cv::Ptr<cv::SphericalWarperGpu>* instance) {
		return instance->get();
	}

	cv::SphericalWarperGpu* cv_PtrOfSphericalWarperGpu_get_inner_ptr_mut(cv::Ptr<cv::SphericalWarperGpu>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrOfSphericalWarperGpu_to_PtrOfWarperCreator(cv::Ptr<cv::SphericalWarperGpu>* instance) {
		return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
}

extern "C" {
	cv::Ptr<cv::StereographicWarper>* cv_PtrOfStereographicWarper_new(cv::StereographicWarper* val) {
		return new cv::Ptr<cv::StereographicWarper>(val);
	}
	
	void cv_PtrOfStereographicWarper_delete(cv::Ptr<cv::StereographicWarper>* instance) {
		delete instance;
	}

	const cv::StereographicWarper* cv_PtrOfStereographicWarper_get_inner_ptr(const cv::Ptr<cv::StereographicWarper>* instance) {
		return instance->get();
	}

	cv::StereographicWarper* cv_PtrOfStereographicWarper_get_inner_ptr_mut(cv::Ptr<cv::StereographicWarper>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrOfStereographicWarper_to_PtrOfWarperCreator(cv::Ptr<cv::StereographicWarper>* instance) {
		return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
}

extern "C" {
	cv::Ptr<cv::Stitcher>* cv_PtrOfStitcher_new(cv::Stitcher* val) {
		return new cv::Ptr<cv::Stitcher>(val);
	}
	
	void cv_PtrOfStitcher_delete(cv::Ptr<cv::Stitcher>* instance) {
		delete instance;
	}

	const cv::Stitcher* cv_PtrOfStitcher_get_inner_ptr(const cv::Ptr<cv::Stitcher>* instance) {
		return instance->get();
	}

	cv::Stitcher* cv_PtrOfStitcher_get_inner_ptr_mut(cv::Ptr<cv::Stitcher>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::TransverseMercatorWarper>* cv_PtrOfTransverseMercatorWarper_new(cv::TransverseMercatorWarper* val) {
		return new cv::Ptr<cv::TransverseMercatorWarper>(val);
	}
	
	void cv_PtrOfTransverseMercatorWarper_delete(cv::Ptr<cv::TransverseMercatorWarper>* instance) {
		delete instance;
	}

	const cv::TransverseMercatorWarper* cv_PtrOfTransverseMercatorWarper_get_inner_ptr(const cv::Ptr<cv::TransverseMercatorWarper>* instance) {
		return instance->get();
	}

	cv::TransverseMercatorWarper* cv_PtrOfTransverseMercatorWarper_get_inner_ptr_mut(cv::Ptr<cv::TransverseMercatorWarper>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrOfTransverseMercatorWarper_to_PtrOfWarperCreator(cv::Ptr<cv::TransverseMercatorWarper>* instance) {
		return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
}

extern "C" {
	void cv_PtrOfWarperCreator_delete(cv::Ptr<cv::WarperCreator>* instance) {
		delete instance;
	}

	const cv::WarperCreator* cv_PtrOfWarperCreator_get_inner_ptr(const cv::Ptr<cv::WarperCreator>* instance) {
		return instance->get();
	}

	cv::WarperCreator* cv_PtrOfWarperCreator_get_inner_ptr_mut(cv::Ptr<cv::WarperCreator>* instance) {
		return instance->get();
	}
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


