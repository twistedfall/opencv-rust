extern "C" {
	// cv::Ptr<cv::ppf_match_3d::Pose3D>::getInnerPtr() generated
	// ("cv::Ptr<cv::ppf_match_3d::Pose3D>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ppf_match_3d::Pose3D* cv_PtrLcv_ppf_match_3d_Pose3DG_getInnerPtr_const(const cv::Ptr<cv::ppf_match_3d::Pose3D>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ppf_match_3d::Pose3D>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ppf_match_3d::Pose3D>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ppf_match_3d::Pose3D* cv_PtrLcv_ppf_match_3d_Pose3DG_getInnerPtrMut(cv::Ptr<cv::ppf_match_3d::Pose3D>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ppf_match_3d::Pose3D>::new_null() generated
	// ("cv::Ptr<cv::ppf_match_3d::Pose3D>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ppf_match_3d::Pose3D>* cv_PtrLcv_ppf_match_3d_Pose3DG_new_null_const() {
			return new cv::Ptr<cv::ppf_match_3d::Pose3D>();
	}

	// cv::Ptr<cv::ppf_match_3d::Pose3D>::delete() generated
	// ("cv::Ptr<cv::ppf_match_3d::Pose3D>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ppf_match_3d_Pose3DG_delete(cv::Ptr<cv::ppf_match_3d::Pose3D>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ppf_match_3d::Pose3D>::new(TraitClass) generated
	// ("cv::Ptr<cv::ppf_match_3d::Pose3D>::new", vec![(pred!(const, ["val"], ["cv::ppf_match_3d::Pose3D"]), _)]),
	cv::Ptr<cv::ppf_match_3d::Pose3D>* cv_PtrLcv_ppf_match_3d_Pose3DG_new_const_Pose3D(cv::ppf_match_3d::Pose3D* val) {
			return new cv::Ptr<cv::ppf_match_3d::Pose3D>(val);
	}

}

