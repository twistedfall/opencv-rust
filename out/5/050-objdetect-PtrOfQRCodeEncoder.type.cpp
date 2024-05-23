extern "C" {
	// cv::Ptr<cv::QRCodeEncoder>::getInnerPtr() generated
	// ("cv::Ptr<cv::QRCodeEncoder>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::QRCodeEncoder* cv_PtrLcv_QRCodeEncoderG_getInnerPtr_const(const cv::Ptr<cv::QRCodeEncoder>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::QRCodeEncoder>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::QRCodeEncoder>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::QRCodeEncoder* cv_PtrLcv_QRCodeEncoderG_getInnerPtrMut(cv::Ptr<cv::QRCodeEncoder>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::QRCodeEncoder>::new_null() generated
	// ("cv::Ptr<cv::QRCodeEncoder>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::QRCodeEncoder>* cv_PtrLcv_QRCodeEncoderG_new_null_const() {
			return new cv::Ptr<cv::QRCodeEncoder>();
	}

	// cv::Ptr<cv::QRCodeEncoder>::delete() generated
	// ("cv::Ptr<cv::QRCodeEncoder>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_QRCodeEncoderG_delete(cv::Ptr<cv::QRCodeEncoder>* instance) {
			delete instance;
	}

}

