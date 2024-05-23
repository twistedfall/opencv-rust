extern "C" {
	// cv::Ptr<cv::text::OCRTesseract>::getInnerPtr() generated
	// ("cv::Ptr<cv::text::OCRTesseract>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::text::OCRTesseract* cv_PtrLcv_text_OCRTesseractG_getInnerPtr_const(const cv::Ptr<cv::text::OCRTesseract>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::text::OCRTesseract>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::text::OCRTesseract>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::text::OCRTesseract* cv_PtrLcv_text_OCRTesseractG_getInnerPtrMut(cv::Ptr<cv::text::OCRTesseract>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::text::OCRTesseract>::new_null() generated
	// ("cv::Ptr<cv::text::OCRTesseract>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::text::OCRTesseract>* cv_PtrLcv_text_OCRTesseractG_new_null_const() {
			return new cv::Ptr<cv::text::OCRTesseract>();
	}

	// cv::Ptr<cv::text::OCRTesseract>::delete() generated
	// ("cv::Ptr<cv::text::OCRTesseract>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_text_OCRTesseractG_delete(cv::Ptr<cv::text::OCRTesseract>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::text::OCRTesseract>::to_PtrOfBaseOCR() generated
	// ("cv::Ptr<cv::text::OCRTesseract>::to_PtrOfBaseOCR", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::text::BaseOCR>* cv_PtrLcv_text_OCRTesseractG_to_PtrOfBaseOCR(cv::Ptr<cv::text::OCRTesseract>* instance) {
			return new cv::Ptr<cv::text::BaseOCR>(instance->dynamicCast<cv::text::BaseOCR>());
	}

}

