extern "C" {
	// cv::Ptr<cv::freetype::FreeType2>::getInnerPtr() generated
	// ("cv::Ptr<cv::freetype::FreeType2>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::freetype::FreeType2* cv_PtrLcv_freetype_FreeType2G_getInnerPtr_const(const cv::Ptr<cv::freetype::FreeType2>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::freetype::FreeType2>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::freetype::FreeType2>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::freetype::FreeType2* cv_PtrLcv_freetype_FreeType2G_getInnerPtrMut(cv::Ptr<cv::freetype::FreeType2>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::freetype::FreeType2>::new_null() generated
	// ("cv::Ptr<cv::freetype::FreeType2>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::freetype::FreeType2>* cv_PtrLcv_freetype_FreeType2G_new_null_const() {
			return new cv::Ptr<cv::freetype::FreeType2>();
	}

	// cv::Ptr<cv::freetype::FreeType2>::delete() generated
	// ("cv::Ptr<cv::freetype::FreeType2>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_freetype_FreeType2G_delete(cv::Ptr<cv::freetype::FreeType2>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::freetype::FreeType2>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::freetype::FreeType2>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_freetype_FreeType2G_to_PtrOfAlgorithm(cv::Ptr<cv::freetype::FreeType2>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

