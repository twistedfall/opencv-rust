template struct Result<cv::Ptr<cv::freetype::FreeType2>*>;
extern "C" {
	void cv_PtrOfFreeType2_delete(cv::Ptr<cv::freetype::FreeType2>* instance) {
		delete instance;
	}

	cv::freetype::FreeType2* cv_PtrOfFreeType2_get_inner_ptr(cv::Ptr<cv::freetype::FreeType2>* instance) {
		return instance->get();
	}
}

