template struct Result<bool>;
template struct Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>*>;
template struct Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>*>;
template struct Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>*>;
template struct Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>*>;
template struct Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>*>;
template struct Result<cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>*>;
template struct Result<cv::bgsegm::SyntheticSequenceGenerator*>;
template struct Result<double>;
template struct Result<int>;
extern "C" {
	void cv_PtrOfBackgroundSubtractorCNT_delete(cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>* instance) {
		delete instance;
	}

	const cv::bgsegm::BackgroundSubtractorCNT* cv_PtrOfBackgroundSubtractorCNT_get_inner_ptr(const cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>* instance) {
		return instance->get();
	}

	cv::bgsegm::BackgroundSubtractorCNT* cv_PtrOfBackgroundSubtractorCNT_get_inner_ptr_mut(cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfBackgroundSubtractorGMG_delete(cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>* instance) {
		delete instance;
	}

	const cv::bgsegm::BackgroundSubtractorGMG* cv_PtrOfBackgroundSubtractorGMG_get_inner_ptr(const cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>* instance) {
		return instance->get();
	}

	cv::bgsegm::BackgroundSubtractorGMG* cv_PtrOfBackgroundSubtractorGMG_get_inner_ptr_mut(cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfBackgroundSubtractorGSOC_delete(cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>* instance) {
		delete instance;
	}

	const cv::bgsegm::BackgroundSubtractorGSOC* cv_PtrOfBackgroundSubtractorGSOC_get_inner_ptr(const cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>* instance) {
		return instance->get();
	}

	cv::bgsegm::BackgroundSubtractorGSOC* cv_PtrOfBackgroundSubtractorGSOC_get_inner_ptr_mut(cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfBackgroundSubtractorLSBP_delete(cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>* instance) {
		delete instance;
	}

	const cv::bgsegm::BackgroundSubtractorLSBP* cv_PtrOfBackgroundSubtractorLSBP_get_inner_ptr(const cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>* instance) {
		return instance->get();
	}

	cv::bgsegm::BackgroundSubtractorLSBP* cv_PtrOfBackgroundSubtractorLSBP_get_inner_ptr_mut(cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfBackgroundSubtractorMOG_delete(cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>* instance) {
		delete instance;
	}

	const cv::bgsegm::BackgroundSubtractorMOG* cv_PtrOfBackgroundSubtractorMOG_get_inner_ptr(const cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>* instance) {
		return instance->get();
	}

	cv::bgsegm::BackgroundSubtractorMOG* cv_PtrOfBackgroundSubtractorMOG_get_inner_ptr_mut(cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>* cv_PtrOfSyntheticSequenceGenerator_new(cv::bgsegm::SyntheticSequenceGenerator* val) {
		return new cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>(val);
	}
	
	void cv_PtrOfSyntheticSequenceGenerator_delete(cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>* instance) {
		delete instance;
	}

	const cv::bgsegm::SyntheticSequenceGenerator* cv_PtrOfSyntheticSequenceGenerator_get_inner_ptr(const cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>* instance) {
		return instance->get();
	}

	cv::bgsegm::SyntheticSequenceGenerator* cv_PtrOfSyntheticSequenceGenerator_get_inner_ptr_mut(cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>* instance) {
		return instance->get();
	}
}

