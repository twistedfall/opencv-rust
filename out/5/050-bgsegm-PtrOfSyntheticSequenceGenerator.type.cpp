extern "C" {
	// cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>::getInnerPtr() generated
	// ("cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::bgsegm::SyntheticSequenceGenerator* cv_PtrLcv_bgsegm_SyntheticSequenceGeneratorG_getInnerPtr_const(const cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::bgsegm::SyntheticSequenceGenerator* cv_PtrLcv_bgsegm_SyntheticSequenceGeneratorG_getInnerPtrMut(cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>::new_null() generated
	// ("cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>* cv_PtrLcv_bgsegm_SyntheticSequenceGeneratorG_new_null_const() {
			return new cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>();
	}

	// cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>::delete() generated
	// ("cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_bgsegm_SyntheticSequenceGeneratorG_delete(cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_bgsegm_SyntheticSequenceGeneratorG_to_PtrOfAlgorithm(cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>::new(TraitClass) generated
	// ("cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>::new", vec![(pred!(const, ["val"], ["cv::bgsegm::SyntheticSequenceGenerator"]), _)]),
	cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>* cv_PtrLcv_bgsegm_SyntheticSequenceGeneratorG_new_const_SyntheticSequenceGenerator(cv::bgsegm::SyntheticSequenceGenerator* val) {
			return new cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>(val);
	}

}

