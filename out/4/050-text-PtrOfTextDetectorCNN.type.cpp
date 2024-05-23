extern "C" {
	// cv::Ptr<cv::text::TextDetectorCNN>::getInnerPtr() generated
	// ("cv::Ptr<cv::text::TextDetectorCNN>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::text::TextDetectorCNN* cv_PtrLcv_text_TextDetectorCNNG_getInnerPtr_const(const cv::Ptr<cv::text::TextDetectorCNN>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::text::TextDetectorCNN>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::text::TextDetectorCNN>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::text::TextDetectorCNN* cv_PtrLcv_text_TextDetectorCNNG_getInnerPtrMut(cv::Ptr<cv::text::TextDetectorCNN>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::text::TextDetectorCNN>::new_null() generated
	// ("cv::Ptr<cv::text::TextDetectorCNN>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::text::TextDetectorCNN>* cv_PtrLcv_text_TextDetectorCNNG_new_null_const() {
			return new cv::Ptr<cv::text::TextDetectorCNN>();
	}

	// cv::Ptr<cv::text::TextDetectorCNN>::delete() generated
	// ("cv::Ptr<cv::text::TextDetectorCNN>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_text_TextDetectorCNNG_delete(cv::Ptr<cv::text::TextDetectorCNN>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::text::TextDetectorCNN>::to_PtrOfTextDetector() generated
	// ("cv::Ptr<cv::text::TextDetectorCNN>::to_PtrOfTextDetector", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::text::TextDetector>* cv_PtrLcv_text_TextDetectorCNNG_to_PtrOfTextDetector(cv::Ptr<cv::text::TextDetectorCNN>* instance) {
			return new cv::Ptr<cv::text::TextDetector>(instance->dynamicCast<cv::text::TextDetector>());
	}

}

