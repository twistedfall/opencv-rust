extern "C" {
	// cv::Ptr<cv::text::OCRHolisticWordRecognizer>::getInnerPtr() generated
	// ("cv::Ptr<cv::text::OCRHolisticWordRecognizer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::text::OCRHolisticWordRecognizer* cv_PtrLcv_text_OCRHolisticWordRecognizerG_getInnerPtr_const(const cv::Ptr<cv::text::OCRHolisticWordRecognizer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::text::OCRHolisticWordRecognizer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::text::OCRHolisticWordRecognizer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::text::OCRHolisticWordRecognizer* cv_PtrLcv_text_OCRHolisticWordRecognizerG_getInnerPtrMut(cv::Ptr<cv::text::OCRHolisticWordRecognizer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::text::OCRHolisticWordRecognizer>::new_null() generated
	// ("cv::Ptr<cv::text::OCRHolisticWordRecognizer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::text::OCRHolisticWordRecognizer>* cv_PtrLcv_text_OCRHolisticWordRecognizerG_new_null_const() {
			return new cv::Ptr<cv::text::OCRHolisticWordRecognizer>();
	}

	// cv::Ptr<cv::text::OCRHolisticWordRecognizer>::delete() generated
	// ("cv::Ptr<cv::text::OCRHolisticWordRecognizer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_text_OCRHolisticWordRecognizerG_delete(cv::Ptr<cv::text::OCRHolisticWordRecognizer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::text::OCRHolisticWordRecognizer>::to_PtrOfBaseOCR() generated
	// ("cv::Ptr<cv::text::OCRHolisticWordRecognizer>::to_PtrOfBaseOCR", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::text::BaseOCR>* cv_PtrLcv_text_OCRHolisticWordRecognizerG_to_PtrOfBaseOCR(cv::Ptr<cv::text::OCRHolisticWordRecognizer>* instance) {
			return new cv::Ptr<cv::text::BaseOCR>(instance->dynamicCast<cv::text::BaseOCR>());
	}

}

