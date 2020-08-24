template struct Result<cv::Ptr<cv::freetype::FreeType2>*>;
template struct Result<cv::Size_<int>>;
extern "C" {
	void cv_PtrOfFreeType2_delete(cv::Ptr<cv::freetype::FreeType2>* instance) {
		delete instance;
	}

	const cv::freetype::FreeType2* cv_PtrOfFreeType2_get_inner_ptr(const cv::Ptr<cv::freetype::FreeType2>* instance) {
		return instance->get();
	}

	cv::freetype::FreeType2* cv_PtrOfFreeType2_get_inner_ptr_mut(cv::Ptr<cv::freetype::FreeType2>* instance) {
		return instance->get();
	}
}

