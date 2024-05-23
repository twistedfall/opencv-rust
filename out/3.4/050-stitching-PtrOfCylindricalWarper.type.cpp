extern "C" {
	// cv::Ptr<cv::CylindricalWarper>::getInnerPtr() generated
	// ("cv::Ptr<cv::CylindricalWarper>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::CylindricalWarper* cv_PtrLcv_CylindricalWarperG_getInnerPtr_const(const cv::Ptr<cv::CylindricalWarper>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::CylindricalWarper>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::CylindricalWarper>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::CylindricalWarper* cv_PtrLcv_CylindricalWarperG_getInnerPtrMut(cv::Ptr<cv::CylindricalWarper>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::CylindricalWarper>::new_null() generated
	// ("cv::Ptr<cv::CylindricalWarper>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::CylindricalWarper>* cv_PtrLcv_CylindricalWarperG_new_null_const() {
			return new cv::Ptr<cv::CylindricalWarper>();
	}

	// cv::Ptr<cv::CylindricalWarper>::delete() generated
	// ("cv::Ptr<cv::CylindricalWarper>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_CylindricalWarperG_delete(cv::Ptr<cv::CylindricalWarper>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::CylindricalWarper>::to_PtrOfWarperCreator() generated
	// ("cv::Ptr<cv::CylindricalWarper>::to_PtrOfWarperCreator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::WarperCreator>* cv_PtrLcv_CylindricalWarperG_to_PtrOfWarperCreator(cv::Ptr<cv::CylindricalWarper>* instance) {
			return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}

	// cv::Ptr<cv::CylindricalWarper>::new(TraitClass) generated
	// ("cv::Ptr<cv::CylindricalWarper>::new", vec![(pred!(const, ["val"], ["cv::CylindricalWarper"]), _)]),
	cv::Ptr<cv::CylindricalWarper>* cv_PtrLcv_CylindricalWarperG_new_const_CylindricalWarper(cv::CylindricalWarper* val) {
			return new cv::Ptr<cv::CylindricalWarper>(val);
	}

}

