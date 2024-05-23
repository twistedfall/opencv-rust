extern "C" {
	// cv::Ptr<cv::linemod::DepthNormal>::getInnerPtr() generated
	// ("cv::Ptr<cv::linemod::DepthNormal>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::linemod::DepthNormal* cv_PtrLcv_linemod_DepthNormalG_getInnerPtr_const(const cv::Ptr<cv::linemod::DepthNormal>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::linemod::DepthNormal>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::linemod::DepthNormal>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::linemod::DepthNormal* cv_PtrLcv_linemod_DepthNormalG_getInnerPtrMut(cv::Ptr<cv::linemod::DepthNormal>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::linemod::DepthNormal>::new_null() generated
	// ("cv::Ptr<cv::linemod::DepthNormal>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::linemod::DepthNormal>* cv_PtrLcv_linemod_DepthNormalG_new_null_const() {
			return new cv::Ptr<cv::linemod::DepthNormal>();
	}

	// cv::Ptr<cv::linemod::DepthNormal>::delete() generated
	// ("cv::Ptr<cv::linemod::DepthNormal>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_linemod_DepthNormalG_delete(cv::Ptr<cv::linemod::DepthNormal>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::linemod::DepthNormal>::to_PtrOfLineMod_Modality() generated
	// ("cv::Ptr<cv::linemod::DepthNormal>::to_PtrOfLineMod_Modality", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::linemod::Modality>* cv_PtrLcv_linemod_DepthNormalG_to_PtrOfLineMod_Modality(cv::Ptr<cv::linemod::DepthNormal>* instance) {
			return new cv::Ptr<cv::linemod::Modality>(instance->dynamicCast<cv::linemod::Modality>());
	}

	// cv::Ptr<cv::linemod::DepthNormal>::new(TraitClass) generated
	// ("cv::Ptr<cv::linemod::DepthNormal>::new", vec![(pred!(const, ["val"], ["cv::linemod::DepthNormal"]), _)]),
	cv::Ptr<cv::linemod::DepthNormal>* cv_PtrLcv_linemod_DepthNormalG_new_const_DepthNormal(cv::linemod::DepthNormal* val) {
			return new cv::Ptr<cv::linemod::DepthNormal>(val);
	}

}

