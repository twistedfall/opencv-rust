template struct Result<bool>;
template struct Result<cv::Ptr<cv::quality::QualityBRISQUE>*>;
template struct Result<cv::Ptr<cv::quality::QualityGMSD>*>;
template struct Result<cv::Ptr<cv::quality::QualityMSE>*>;
template struct Result<cv::Ptr<cv::quality::QualityPSNR>*>;
template struct Result<cv::Ptr<cv::quality::QualitySSIM>*>;
template struct Result<cv::Scalar_<double>>;
template struct Result<double>;
extern "C" {
	cv::Ptr<cv::quality::QualityBRISQUE>* cv_PtrOfQualityBRISQUE_new(cv::quality::QualityBRISQUE* val) {
		return new cv::Ptr<cv::quality::QualityBRISQUE>(val);
	}
	
	void cv_PtrOfQualityBRISQUE_delete(cv::Ptr<cv::quality::QualityBRISQUE>* instance) {
		delete instance;
	}

	const cv::quality::QualityBRISQUE* cv_PtrOfQualityBRISQUE_get_inner_ptr(const cv::Ptr<cv::quality::QualityBRISQUE>* instance) {
		return instance->get();
	}

	cv::quality::QualityBRISQUE* cv_PtrOfQualityBRISQUE_get_inner_ptr_mut(cv::Ptr<cv::quality::QualityBRISQUE>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::quality::QualityGMSD>* cv_PtrOfQualityGMSD_new(cv::quality::QualityGMSD* val) {
		return new cv::Ptr<cv::quality::QualityGMSD>(val);
	}
	
	void cv_PtrOfQualityGMSD_delete(cv::Ptr<cv::quality::QualityGMSD>* instance) {
		delete instance;
	}

	const cv::quality::QualityGMSD* cv_PtrOfQualityGMSD_get_inner_ptr(const cv::Ptr<cv::quality::QualityGMSD>* instance) {
		return instance->get();
	}

	cv::quality::QualityGMSD* cv_PtrOfQualityGMSD_get_inner_ptr_mut(cv::Ptr<cv::quality::QualityGMSD>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::quality::QualityMSE>* cv_PtrOfQualityMSE_new(cv::quality::QualityMSE* val) {
		return new cv::Ptr<cv::quality::QualityMSE>(val);
	}
	
	void cv_PtrOfQualityMSE_delete(cv::Ptr<cv::quality::QualityMSE>* instance) {
		delete instance;
	}

	const cv::quality::QualityMSE* cv_PtrOfQualityMSE_get_inner_ptr(const cv::Ptr<cv::quality::QualityMSE>* instance) {
		return instance->get();
	}

	cv::quality::QualityMSE* cv_PtrOfQualityMSE_get_inner_ptr_mut(cv::Ptr<cv::quality::QualityMSE>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::quality::QualityPSNR>* cv_PtrOfQualityPSNR_new(cv::quality::QualityPSNR* val) {
		return new cv::Ptr<cv::quality::QualityPSNR>(val);
	}
	
	void cv_PtrOfQualityPSNR_delete(cv::Ptr<cv::quality::QualityPSNR>* instance) {
		delete instance;
	}

	const cv::quality::QualityPSNR* cv_PtrOfQualityPSNR_get_inner_ptr(const cv::Ptr<cv::quality::QualityPSNR>* instance) {
		return instance->get();
	}

	cv::quality::QualityPSNR* cv_PtrOfQualityPSNR_get_inner_ptr_mut(cv::Ptr<cv::quality::QualityPSNR>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::quality::QualitySSIM>* cv_PtrOfQualitySSIM_new(cv::quality::QualitySSIM* val) {
		return new cv::Ptr<cv::quality::QualitySSIM>(val);
	}
	
	void cv_PtrOfQualitySSIM_delete(cv::Ptr<cv::quality::QualitySSIM>* instance) {
		delete instance;
	}

	const cv::quality::QualitySSIM* cv_PtrOfQualitySSIM_get_inner_ptr(const cv::Ptr<cv::quality::QualitySSIM>* instance) {
		return instance->get();
	}

	cv::quality::QualitySSIM* cv_PtrOfQualitySSIM_get_inner_ptr_mut(cv::Ptr<cv::quality::QualitySSIM>* instance) {
		return instance->get();
	}
}

