extern "C" {
	// cv::Ptr<cv::videostab::NullWobbleSuppressor>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::NullWobbleSuppressor>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::NullWobbleSuppressor* cv_PtrLcv_videostab_NullWobbleSuppressorG_getInnerPtr_const(const cv::Ptr<cv::videostab::NullWobbleSuppressor>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::NullWobbleSuppressor>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::NullWobbleSuppressor>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::NullWobbleSuppressor* cv_PtrLcv_videostab_NullWobbleSuppressorG_getInnerPtrMut(cv::Ptr<cv::videostab::NullWobbleSuppressor>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::NullWobbleSuppressor>::new_null() generated
	// ("cv::Ptr<cv::videostab::NullWobbleSuppressor>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::NullWobbleSuppressor>* cv_PtrLcv_videostab_NullWobbleSuppressorG_new_null_const() {
			return new cv::Ptr<cv::videostab::NullWobbleSuppressor>();
	}

	// cv::Ptr<cv::videostab::NullWobbleSuppressor>::delete() generated
	// ("cv::Ptr<cv::videostab::NullWobbleSuppressor>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_NullWobbleSuppressorG_delete(cv::Ptr<cv::videostab::NullWobbleSuppressor>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::NullWobbleSuppressor>::to_PtrOfWobbleSuppressorBase() generated
	// ("cv::Ptr<cv::videostab::NullWobbleSuppressor>::to_PtrOfWobbleSuppressorBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::WobbleSuppressorBase>* cv_PtrLcv_videostab_NullWobbleSuppressorG_to_PtrOfWobbleSuppressorBase(cv::Ptr<cv::videostab::NullWobbleSuppressor>* instance) {
			return new cv::Ptr<cv::videostab::WobbleSuppressorBase>(instance->dynamicCast<cv::videostab::WobbleSuppressorBase>());
	}

	// cv::Ptr<cv::videostab::NullWobbleSuppressor>::new(TraitClass) generated
	// ("cv::Ptr<cv::videostab::NullWobbleSuppressor>::new", vec![(pred!(const, ["val"], ["cv::videostab::NullWobbleSuppressor"]), _)]),
	cv::Ptr<cv::videostab::NullWobbleSuppressor>* cv_PtrLcv_videostab_NullWobbleSuppressorG_new_const_NullWobbleSuppressor(cv::videostab::NullWobbleSuppressor* val) {
			return new cv::Ptr<cv::videostab::NullWobbleSuppressor>(val);
	}

}

