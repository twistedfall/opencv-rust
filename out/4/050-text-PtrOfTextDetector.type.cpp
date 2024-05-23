extern "C" {
	// cv::Ptr<cv::text::TextDetector>::getInnerPtr() generated
	// ("cv::Ptr<cv::text::TextDetector>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::text::TextDetector* cv_PtrLcv_text_TextDetectorG_getInnerPtr_const(const cv::Ptr<cv::text::TextDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::text::TextDetector>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::text::TextDetector>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::text::TextDetector* cv_PtrLcv_text_TextDetectorG_getInnerPtrMut(cv::Ptr<cv::text::TextDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::text::TextDetector>::new_null() generated
	// ("cv::Ptr<cv::text::TextDetector>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::text::TextDetector>* cv_PtrLcv_text_TextDetectorG_new_null_const() {
			return new cv::Ptr<cv::text::TextDetector>();
	}

	// cv::Ptr<cv::text::TextDetector>::delete() generated
	// ("cv::Ptr<cv::text::TextDetector>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_text_TextDetectorG_delete(cv::Ptr<cv::text::TextDetector>* instance) {
			delete instance;
	}

}

