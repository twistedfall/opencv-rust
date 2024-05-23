extern "C" {
	// cv::Ptr<cv::line_descriptor::LSDDetector>::getInnerPtr() generated
	// ("cv::Ptr<cv::line_descriptor::LSDDetector>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::line_descriptor::LSDDetector* cv_PtrLcv_line_descriptor_LSDDetectorG_getInnerPtr_const(const cv::Ptr<cv::line_descriptor::LSDDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::line_descriptor::LSDDetector>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::line_descriptor::LSDDetector>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::line_descriptor::LSDDetector* cv_PtrLcv_line_descriptor_LSDDetectorG_getInnerPtrMut(cv::Ptr<cv::line_descriptor::LSDDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::line_descriptor::LSDDetector>::new_null() generated
	// ("cv::Ptr<cv::line_descriptor::LSDDetector>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::line_descriptor::LSDDetector>* cv_PtrLcv_line_descriptor_LSDDetectorG_new_null_const() {
			return new cv::Ptr<cv::line_descriptor::LSDDetector>();
	}

	// cv::Ptr<cv::line_descriptor::LSDDetector>::delete() generated
	// ("cv::Ptr<cv::line_descriptor::LSDDetector>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_line_descriptor_LSDDetectorG_delete(cv::Ptr<cv::line_descriptor::LSDDetector>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::line_descriptor::LSDDetector>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::line_descriptor::LSDDetector>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_line_descriptor_LSDDetectorG_to_PtrOfAlgorithm(cv::Ptr<cv::line_descriptor::LSDDetector>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::line_descriptor::LSDDetector>::new(TraitClass) generated
	// ("cv::Ptr<cv::line_descriptor::LSDDetector>::new", vec![(pred!(const, ["val"], ["cv::line_descriptor::LSDDetector"]), _)]),
	cv::Ptr<cv::line_descriptor::LSDDetector>* cv_PtrLcv_line_descriptor_LSDDetectorG_new_const_LSDDetector(cv::line_descriptor::LSDDetector* val) {
			return new cv::Ptr<cv::line_descriptor::LSDDetector>(val);
	}

}

