extern "C" {
	// cv::Ptr<cv::videostab::FromFileMotionReader>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::FromFileMotionReader>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::FromFileMotionReader* cv_PtrLcv_videostab_FromFileMotionReaderG_getInnerPtr_const(const cv::Ptr<cv::videostab::FromFileMotionReader>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::FromFileMotionReader>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::FromFileMotionReader>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::FromFileMotionReader* cv_PtrLcv_videostab_FromFileMotionReaderG_getInnerPtrMut(cv::Ptr<cv::videostab::FromFileMotionReader>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::FromFileMotionReader>::new_null() generated
	// ("cv::Ptr<cv::videostab::FromFileMotionReader>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::FromFileMotionReader>* cv_PtrLcv_videostab_FromFileMotionReaderG_new_null_const() {
			return new cv::Ptr<cv::videostab::FromFileMotionReader>();
	}

	// cv::Ptr<cv::videostab::FromFileMotionReader>::delete() generated
	// ("cv::Ptr<cv::videostab::FromFileMotionReader>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_FromFileMotionReaderG_delete(cv::Ptr<cv::videostab::FromFileMotionReader>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::FromFileMotionReader>::to_PtrOfImageMotionEstimatorBase() generated
	// ("cv::Ptr<cv::videostab::FromFileMotionReader>::to_PtrOfImageMotionEstimatorBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::ImageMotionEstimatorBase>* cv_PtrLcv_videostab_FromFileMotionReaderG_to_PtrOfImageMotionEstimatorBase(cv::Ptr<cv::videostab::FromFileMotionReader>* instance) {
			return new cv::Ptr<cv::videostab::ImageMotionEstimatorBase>(instance->dynamicCast<cv::videostab::ImageMotionEstimatorBase>());
	}

	// cv::Ptr<cv::videostab::FromFileMotionReader>::new(TraitClass) generated
	// ("cv::Ptr<cv::videostab::FromFileMotionReader>::new", vec![(pred!(const, ["val"], ["cv::videostab::FromFileMotionReader"]), _)]),
	cv::Ptr<cv::videostab::FromFileMotionReader>* cv_PtrLcv_videostab_FromFileMotionReaderG_new_const_FromFileMotionReader(cv::videostab::FromFileMotionReader* val) {
			return new cv::Ptr<cv::videostab::FromFileMotionReader>(val);
	}

}

