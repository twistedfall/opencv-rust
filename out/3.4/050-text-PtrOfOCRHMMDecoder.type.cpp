extern "C" {
	// cv::Ptr<cv::text::OCRHMMDecoder>::getInnerPtr() generated
	// ("cv::Ptr<cv::text::OCRHMMDecoder>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::text::OCRHMMDecoder* cv_PtrLcv_text_OCRHMMDecoderG_getInnerPtr_const(const cv::Ptr<cv::text::OCRHMMDecoder>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::text::OCRHMMDecoder>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::text::OCRHMMDecoder>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::text::OCRHMMDecoder* cv_PtrLcv_text_OCRHMMDecoderG_getInnerPtrMut(cv::Ptr<cv::text::OCRHMMDecoder>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::text::OCRHMMDecoder>::new_null() generated
	// ("cv::Ptr<cv::text::OCRHMMDecoder>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::text::OCRHMMDecoder>* cv_PtrLcv_text_OCRHMMDecoderG_new_null_const() {
			return new cv::Ptr<cv::text::OCRHMMDecoder>();
	}

	// cv::Ptr<cv::text::OCRHMMDecoder>::delete() generated
	// ("cv::Ptr<cv::text::OCRHMMDecoder>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_text_OCRHMMDecoderG_delete(cv::Ptr<cv::text::OCRHMMDecoder>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::text::OCRHMMDecoder>::to_PtrOfBaseOCR() generated
	// ("cv::Ptr<cv::text::OCRHMMDecoder>::to_PtrOfBaseOCR", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::text::BaseOCR>* cv_PtrLcv_text_OCRHMMDecoderG_to_PtrOfBaseOCR(cv::Ptr<cv::text::OCRHMMDecoder>* instance) {
			return new cv::Ptr<cv::text::BaseOCR>(instance->dynamicCast<cv::text::BaseOCR>());
	}

	// cv::Ptr<cv::text::OCRHMMDecoder>::new(TraitClass) generated
	// ("cv::Ptr<cv::text::OCRHMMDecoder>::new", vec![(pred!(const, ["val"], ["cv::text::OCRHMMDecoder"]), _)]),
	cv::Ptr<cv::text::OCRHMMDecoder>* cv_PtrLcv_text_OCRHMMDecoderG_new_const_OCRHMMDecoder(cv::text::OCRHMMDecoder* val) {
			return new cv::Ptr<cv::text::OCRHMMDecoder>(val);
	}

}

