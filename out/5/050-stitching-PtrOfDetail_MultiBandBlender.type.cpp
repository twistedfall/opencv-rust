extern "C" {
	// cv::Ptr<cv::detail::MultiBandBlender>::getInnerPtr() generated
	// ("cv::Ptr<cv::detail::MultiBandBlender>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::detail::MultiBandBlender* cv_PtrLcv_detail_MultiBandBlenderG_getInnerPtr_const(const cv::Ptr<cv::detail::MultiBandBlender>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::MultiBandBlender>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::detail::MultiBandBlender>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::MultiBandBlender* cv_PtrLcv_detail_MultiBandBlenderG_getInnerPtrMut(cv::Ptr<cv::detail::MultiBandBlender>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::MultiBandBlender>::new_null() generated
	// ("cv::Ptr<cv::detail::MultiBandBlender>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::detail::MultiBandBlender>* cv_PtrLcv_detail_MultiBandBlenderG_new_null_const() {
			return new cv::Ptr<cv::detail::MultiBandBlender>();
	}

	// cv::Ptr<cv::detail::MultiBandBlender>::delete() generated
	// ("cv::Ptr<cv::detail::MultiBandBlender>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_detail_MultiBandBlenderG_delete(cv::Ptr<cv::detail::MultiBandBlender>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::detail::MultiBandBlender>::to_PtrOfDetail_Blender() generated
	// ("cv::Ptr<cv::detail::MultiBandBlender>::to_PtrOfDetail_Blender", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::Blender>* cv_PtrLcv_detail_MultiBandBlenderG_to_PtrOfDetail_Blender(cv::Ptr<cv::detail::MultiBandBlender>* instance) {
			return new cv::Ptr<cv::detail::Blender>(instance->dynamicCast<cv::detail::Blender>());
	}

	// cv::Ptr<cv::detail::MultiBandBlender>::new(TraitClass) generated
	// ("cv::Ptr<cv::detail::MultiBandBlender>::new", vec![(pred!(const, ["val"], ["cv::detail::MultiBandBlender"]), _)]),
	cv::Ptr<cv::detail::MultiBandBlender>* cv_PtrLcv_detail_MultiBandBlenderG_new_const_MultiBandBlender(cv::detail::MultiBandBlender* val) {
			return new cv::Ptr<cv::detail::MultiBandBlender>(val);
	}

}

