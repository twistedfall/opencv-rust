extern "C" {
	// cv::Ptr<cv::linemod::ColorGradient>::getInnerPtr() generated
	// ("cv::Ptr<cv::linemod::ColorGradient>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::linemod::ColorGradient* cv_PtrLcv_linemod_ColorGradientG_getInnerPtr_const(const cv::Ptr<cv::linemod::ColorGradient>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::linemod::ColorGradient>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::linemod::ColorGradient>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::linemod::ColorGradient* cv_PtrLcv_linemod_ColorGradientG_getInnerPtrMut(cv::Ptr<cv::linemod::ColorGradient>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::linemod::ColorGradient>::new_null() generated
	// ("cv::Ptr<cv::linemod::ColorGradient>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::linemod::ColorGradient>* cv_PtrLcv_linemod_ColorGradientG_new_null_const() {
			return new cv::Ptr<cv::linemod::ColorGradient>();
	}

	// cv::Ptr<cv::linemod::ColorGradient>::delete() generated
	// ("cv::Ptr<cv::linemod::ColorGradient>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_linemod_ColorGradientG_delete(cv::Ptr<cv::linemod::ColorGradient>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::linemod::ColorGradient>::to_PtrOfLineMod_Modality() generated
	// ("cv::Ptr<cv::linemod::ColorGradient>::to_PtrOfLineMod_Modality", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::linemod::Modality>* cv_PtrLcv_linemod_ColorGradientG_to_PtrOfLineMod_Modality(cv::Ptr<cv::linemod::ColorGradient>* instance) {
			return new cv::Ptr<cv::linemod::Modality>(instance->dynamicCast<cv::linemod::Modality>());
	}

	// cv::Ptr<cv::linemod::ColorGradient>::new(TraitClass) generated
	// ("cv::Ptr<cv::linemod::ColorGradient>::new", vec![(pred!(const, ["val"], ["cv::linemod::ColorGradient"]), _)]),
	cv::Ptr<cv::linemod::ColorGradient>* cv_PtrLcv_linemod_ColorGradientG_new_const_ColorGradient(cv::linemod::ColorGradient* val) {
			return new cv::Ptr<cv::linemod::ColorGradient>(val);
	}

}

