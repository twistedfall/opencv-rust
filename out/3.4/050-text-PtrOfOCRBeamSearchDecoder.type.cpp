extern "C" {
	// cv::Ptr<cv::text::OCRBeamSearchDecoder>::getInnerPtr() generated
	// ("cv::Ptr<cv::text::OCRBeamSearchDecoder>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::text::OCRBeamSearchDecoder* cv_PtrLcv_text_OCRBeamSearchDecoderG_getInnerPtr_const(const cv::Ptr<cv::text::OCRBeamSearchDecoder>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::text::OCRBeamSearchDecoder>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::text::OCRBeamSearchDecoder>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::text::OCRBeamSearchDecoder* cv_PtrLcv_text_OCRBeamSearchDecoderG_getInnerPtrMut(cv::Ptr<cv::text::OCRBeamSearchDecoder>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::text::OCRBeamSearchDecoder>::new_null() generated
	// ("cv::Ptr<cv::text::OCRBeamSearchDecoder>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::text::OCRBeamSearchDecoder>* cv_PtrLcv_text_OCRBeamSearchDecoderG_new_null_const() {
			return new cv::Ptr<cv::text::OCRBeamSearchDecoder>();
	}

	// cv::Ptr<cv::text::OCRBeamSearchDecoder>::delete() generated
	// ("cv::Ptr<cv::text::OCRBeamSearchDecoder>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_text_OCRBeamSearchDecoderG_delete(cv::Ptr<cv::text::OCRBeamSearchDecoder>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::text::OCRBeamSearchDecoder>::to_PtrOfBaseOCR() generated
	// ("cv::Ptr<cv::text::OCRBeamSearchDecoder>::to_PtrOfBaseOCR", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::text::BaseOCR>* cv_PtrLcv_text_OCRBeamSearchDecoderG_to_PtrOfBaseOCR(cv::Ptr<cv::text::OCRBeamSearchDecoder>* instance) {
			return new cv::Ptr<cv::text::BaseOCR>(instance->dynamicCast<cv::text::BaseOCR>());
	}

	// cv::Ptr<cv::text::OCRBeamSearchDecoder>::new(TraitClass) generated
	// ("cv::Ptr<cv::text::OCRBeamSearchDecoder>::new", vec![(pred!(const, ["val"], ["cv::text::OCRBeamSearchDecoder"]), _)]),
	cv::Ptr<cv::text::OCRBeamSearchDecoder>* cv_PtrLcv_text_OCRBeamSearchDecoderG_new_const_OCRBeamSearchDecoder(cv::text::OCRBeamSearchDecoder* val) {
			return new cv::Ptr<cv::text::OCRBeamSearchDecoder>(val);
	}

}

