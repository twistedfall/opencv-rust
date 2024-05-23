extern "C" {
	// cv::Ptr<cv::videostab::ToFileMotionWriter>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::ToFileMotionWriter>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::ToFileMotionWriter* cv_PtrLcv_videostab_ToFileMotionWriterG_getInnerPtr_const(const cv::Ptr<cv::videostab::ToFileMotionWriter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::ToFileMotionWriter>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::ToFileMotionWriter>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::ToFileMotionWriter* cv_PtrLcv_videostab_ToFileMotionWriterG_getInnerPtrMut(cv::Ptr<cv::videostab::ToFileMotionWriter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::ToFileMotionWriter>::new_null() generated
	// ("cv::Ptr<cv::videostab::ToFileMotionWriter>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::ToFileMotionWriter>* cv_PtrLcv_videostab_ToFileMotionWriterG_new_null_const() {
			return new cv::Ptr<cv::videostab::ToFileMotionWriter>();
	}

	// cv::Ptr<cv::videostab::ToFileMotionWriter>::delete() generated
	// ("cv::Ptr<cv::videostab::ToFileMotionWriter>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_ToFileMotionWriterG_delete(cv::Ptr<cv::videostab::ToFileMotionWriter>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::ToFileMotionWriter>::to_PtrOfImageMotionEstimatorBase() generated
	// ("cv::Ptr<cv::videostab::ToFileMotionWriter>::to_PtrOfImageMotionEstimatorBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::ImageMotionEstimatorBase>* cv_PtrLcv_videostab_ToFileMotionWriterG_to_PtrOfImageMotionEstimatorBase(cv::Ptr<cv::videostab::ToFileMotionWriter>* instance) {
			return new cv::Ptr<cv::videostab::ImageMotionEstimatorBase>(instance->dynamicCast<cv::videostab::ImageMotionEstimatorBase>());
	}

	// cv::Ptr<cv::videostab::ToFileMotionWriter>::new(TraitClass) generated
	// ("cv::Ptr<cv::videostab::ToFileMotionWriter>::new", vec![(pred!(const, ["val"], ["cv::videostab::ToFileMotionWriter"]), _)]),
	cv::Ptr<cv::videostab::ToFileMotionWriter>* cv_PtrLcv_videostab_ToFileMotionWriterG_new_const_ToFileMotionWriter(cv::videostab::ToFileMotionWriter* val) {
			return new cv::Ptr<cv::videostab::ToFileMotionWriter>(val);
	}

}

