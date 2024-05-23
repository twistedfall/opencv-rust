extern "C" {
	// cv::Ptr<cv::detail::Blender>::getInnerPtr() generated
	// ("cv::Ptr<cv::detail::Blender>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::detail::Blender* cv_PtrLcv_detail_BlenderG_getInnerPtr_const(const cv::Ptr<cv::detail::Blender>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::Blender>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::detail::Blender>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::Blender* cv_PtrLcv_detail_BlenderG_getInnerPtrMut(cv::Ptr<cv::detail::Blender>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::Blender>::new_null() generated
	// ("cv::Ptr<cv::detail::Blender>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::detail::Blender>* cv_PtrLcv_detail_BlenderG_new_null_const() {
			return new cv::Ptr<cv::detail::Blender>();
	}

	// cv::Ptr<cv::detail::Blender>::delete() generated
	// ("cv::Ptr<cv::detail::Blender>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_detail_BlenderG_delete(cv::Ptr<cv::detail::Blender>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::detail::Blender>::new(TraitClass) generated
	// ("cv::Ptr<cv::detail::Blender>::new", vec![(pred!(const, ["val"], ["cv::detail::Blender"]), _)]),
	cv::Ptr<cv::detail::Blender>* cv_PtrLcv_detail_BlenderG_new_const_Blender(cv::detail::Blender* val) {
			return new cv::Ptr<cv::detail::Blender>(val);
	}

}

