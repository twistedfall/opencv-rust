template struct Result<bool>;
template struct Result<cv::Ptr<cv::saliency::MotionSaliencyBinWangApr2014>*>;
template struct Result<cv::Ptr<cv::saliency::ObjectnessBING>*>;
template struct Result<cv::Ptr<cv::saliency::StaticSaliencyFineGrained>*>;
template struct Result<cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>*>;
template struct Result<cv::saliency::MotionSaliencyBinWangApr2014*>;
template struct Result<cv::saliency::ObjectnessBING*>;
template struct Result<cv::saliency::StaticSaliencyFineGrained*>;
template struct Result<cv::saliency::StaticSaliencySpectralResidual*>;
template struct Result<double>;
template struct Result<int>;
template struct Result<std::vector<float>*>;
extern "C" {
	cv::Ptr<cv::saliency::MotionSaliencyBinWangApr2014>* cv_PtrOfMotionSaliencyBinWangApr2014_new(cv::saliency::MotionSaliencyBinWangApr2014* val) {
		return new cv::Ptr<cv::saliency::MotionSaliencyBinWangApr2014>(val);
	}
	
	void cv_PtrOfMotionSaliencyBinWangApr2014_delete(cv::Ptr<cv::saliency::MotionSaliencyBinWangApr2014>* instance) {
		delete instance;
	}

	cv::saliency::MotionSaliencyBinWangApr2014* cv_PtrOfMotionSaliencyBinWangApr2014_get_inner_ptr(cv::Ptr<cv::saliency::MotionSaliencyBinWangApr2014>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::saliency::ObjectnessBING>* cv_PtrOfObjectnessBING_new(cv::saliency::ObjectnessBING* val) {
		return new cv::Ptr<cv::saliency::ObjectnessBING>(val);
	}
	
	void cv_PtrOfObjectnessBING_delete(cv::Ptr<cv::saliency::ObjectnessBING>* instance) {
		delete instance;
	}

	cv::saliency::ObjectnessBING* cv_PtrOfObjectnessBING_get_inner_ptr(cv::Ptr<cv::saliency::ObjectnessBING>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::saliency::StaticSaliencyFineGrained>* cv_PtrOfStaticSaliencyFineGrained_new(cv::saliency::StaticSaliencyFineGrained* val) {
		return new cv::Ptr<cv::saliency::StaticSaliencyFineGrained>(val);
	}
	
	void cv_PtrOfStaticSaliencyFineGrained_delete(cv::Ptr<cv::saliency::StaticSaliencyFineGrained>* instance) {
		delete instance;
	}

	cv::saliency::StaticSaliencyFineGrained* cv_PtrOfStaticSaliencyFineGrained_get_inner_ptr(cv::Ptr<cv::saliency::StaticSaliencyFineGrained>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>* cv_PtrOfStaticSaliencySpectralResidual_new(cv::saliency::StaticSaliencySpectralResidual* val) {
		return new cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>(val);
	}
	
	void cv_PtrOfStaticSaliencySpectralResidual_delete(cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>* instance) {
		delete instance;
	}

	cv::saliency::StaticSaliencySpectralResidual* cv_PtrOfStaticSaliencySpectralResidual_get_inner_ptr(cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>* instance) {
		return instance->get();
	}
}

