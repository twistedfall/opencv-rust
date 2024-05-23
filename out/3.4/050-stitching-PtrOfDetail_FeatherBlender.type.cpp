extern "C" {
	// cv::Ptr<cv::detail::FeatherBlender>::getInnerPtr() generated
	// ("cv::Ptr<cv::detail::FeatherBlender>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::detail::FeatherBlender* cv_PtrLcv_detail_FeatherBlenderG_getInnerPtr_const(const cv::Ptr<cv::detail::FeatherBlender>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::FeatherBlender>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::detail::FeatherBlender>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::FeatherBlender* cv_PtrLcv_detail_FeatherBlenderG_getInnerPtrMut(cv::Ptr<cv::detail::FeatherBlender>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::FeatherBlender>::new_null() generated
	// ("cv::Ptr<cv::detail::FeatherBlender>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::detail::FeatherBlender>* cv_PtrLcv_detail_FeatherBlenderG_new_null_const() {
			return new cv::Ptr<cv::detail::FeatherBlender>();
	}

	// cv::Ptr<cv::detail::FeatherBlender>::delete() generated
	// ("cv::Ptr<cv::detail::FeatherBlender>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_detail_FeatherBlenderG_delete(cv::Ptr<cv::detail::FeatherBlender>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::detail::FeatherBlender>::to_PtrOfDetail_Blender() generated
	// ("cv::Ptr<cv::detail::FeatherBlender>::to_PtrOfDetail_Blender", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::Blender>* cv_PtrLcv_detail_FeatherBlenderG_to_PtrOfDetail_Blender(cv::Ptr<cv::detail::FeatherBlender>* instance) {
			return new cv::Ptr<cv::detail::Blender>(instance->dynamicCast<cv::detail::Blender>());
	}

	// cv::Ptr<cv::detail::FeatherBlender>::new(TraitClass) generated
	// ("cv::Ptr<cv::detail::FeatherBlender>::new", vec![(pred!(const, ["val"], ["cv::detail::FeatherBlender"]), _)]),
	cv::Ptr<cv::detail::FeatherBlender>* cv_PtrLcv_detail_FeatherBlenderG_new_const_FeatherBlender(cv::detail::FeatherBlender* val) {
			return new cv::Ptr<cv::detail::FeatherBlender>(val);
	}

}

