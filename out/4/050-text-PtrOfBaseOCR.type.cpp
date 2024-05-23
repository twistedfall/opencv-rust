extern "C" {
	// cv::Ptr<cv::text::BaseOCR>::getInnerPtr() generated
	// ("cv::Ptr<cv::text::BaseOCR>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::text::BaseOCR* cv_PtrLcv_text_BaseOCRG_getInnerPtr_const(const cv::Ptr<cv::text::BaseOCR>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::text::BaseOCR>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::text::BaseOCR>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::text::BaseOCR* cv_PtrLcv_text_BaseOCRG_getInnerPtrMut(cv::Ptr<cv::text::BaseOCR>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::text::BaseOCR>::new_null() generated
	// ("cv::Ptr<cv::text::BaseOCR>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::text::BaseOCR>* cv_PtrLcv_text_BaseOCRG_new_null_const() {
			return new cv::Ptr<cv::text::BaseOCR>();
	}

	// cv::Ptr<cv::text::BaseOCR>::delete() generated
	// ("cv::Ptr<cv::text::BaseOCR>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_text_BaseOCRG_delete(cv::Ptr<cv::text::BaseOCR>* instance) {
			delete instance;
	}

}

