template struct Result<bool>;
template struct Result<cv::Ptr<cv::ml::SVM>*>;
template struct Result<cv::Ptr<cv::quality::QualityBRISQUE>*>;
template struct Result<cv::Ptr<cv::quality::QualityGMSD>*>;
template struct Result<cv::Ptr<cv::quality::QualityMSE>*>;
template struct Result<cv::Ptr<cv::quality::QualityPSNR>*>;
template struct Result<cv::Ptr<cv::quality::QualitySSIM>*>;
template struct Result<cv::Scalar_<double>>;
template struct Result<double>;
extern "C" void cv_PtrOfQualityBRISQUE_delete(cv::Ptr<cv::quality::QualityBRISQUE>* instance) {
	delete instance;
}

extern "C" cv::quality::QualityBRISQUE* cv_PtrOfQualityBRISQUE_get_inner_ptr(cv::Ptr<cv::quality::QualityBRISQUE>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfQualityGMSD_delete(cv::Ptr<cv::quality::QualityGMSD>* instance) {
	delete instance;
}

extern "C" cv::quality::QualityGMSD* cv_PtrOfQualityGMSD_get_inner_ptr(cv::Ptr<cv::quality::QualityGMSD>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfQualityMSE_delete(cv::Ptr<cv::quality::QualityMSE>* instance) {
	delete instance;
}

extern "C" cv::quality::QualityMSE* cv_PtrOfQualityMSE_get_inner_ptr(cv::Ptr<cv::quality::QualityMSE>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfQualityPSNR_delete(cv::Ptr<cv::quality::QualityPSNR>* instance) {
	delete instance;
}

extern "C" cv::quality::QualityPSNR* cv_PtrOfQualityPSNR_get_inner_ptr(cv::Ptr<cv::quality::QualityPSNR>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfQualitySSIM_delete(cv::Ptr<cv::quality::QualitySSIM>* instance) {
	delete instance;
}

extern "C" cv::quality::QualitySSIM* cv_PtrOfQualitySSIM_get_inner_ptr(cv::Ptr<cv::quality::QualitySSIM>* instance) {
	return instance->get();
}

