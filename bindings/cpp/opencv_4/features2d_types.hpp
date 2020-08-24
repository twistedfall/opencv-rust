template struct Result<bool>;
template struct Result<char>;
template struct Result<const cv::Mat*>;
template struct Result<const std::vector<cv::Mat>*>;
template struct Result<cv::AKAZE::DescriptorType>;
template struct Result<cv::AgastFeatureDetector::DetectorType>;
template struct Result<cv::BFMatcher*>;
template struct Result<cv::BOWImgDescriptorExtractor*>;
template struct Result<cv::BOWKMeansTrainer*>;
template struct Result<cv::DMatch>;
template struct Result<cv::FastFeatureDetector::DetectorType>;
template struct Result<cv::FlannBasedMatcher*>;
template struct Result<cv::KAZE::DiffusivityType>;
template struct Result<cv::KeyPoint>;
template struct Result<cv::KeyPointsFilter*>;
template struct Result<cv::Mat*>;
template struct Result<cv::ORB::ScoreType>;
template struct Result<cv::Point_<float>>;
template struct Result<cv::Point_<int>>;
template struct Result<cv::Ptr<cv::AKAZE>*>;
template struct Result<cv::Ptr<cv::AgastFeatureDetector>*>;
template struct Result<cv::Ptr<cv::BFMatcher>*>;
template struct Result<cv::Ptr<cv::BRISK>*>;
template struct Result<cv::Ptr<cv::DescriptorMatcher>*>;
template struct Result<cv::Ptr<cv::FastFeatureDetector>*>;
template struct Result<cv::Ptr<cv::FlannBasedMatcher>*>;
template struct Result<cv::Ptr<cv::GFTTDetector>*>;
template struct Result<cv::Ptr<cv::KAZE>*>;
template struct Result<cv::Ptr<cv::MSER>*>;
template struct Result<cv::Ptr<cv::ORB>*>;
template struct Result<cv::Ptr<cv::SIFT>*>;
template struct Result<cv::Ptr<cv::SimpleBlobDetector>*>;
template struct Result<cv::Rect_<int>>;
template struct Result<cv::SimpleBlobDetector::Params>;
template struct Result<double>;
template struct Result<float>;
template struct Result<int>;
template struct Result<std::vector<char>*>;
template struct Result<std::vector<cv::DMatch>*>;
template struct Result<std::vector<cv::KeyPoint>*>;
template struct Result<std::vector<cv::Point_<int>>*>;
template struct Result<std::vector<int>*>;
template struct Result<std::vector<unsigned char>*>;
template struct Result<unsigned char>;
template struct Result<unsigned long>;
template struct Result<void*>;
extern "C" {
	void cv_PtrOfAKAZE_delete(cv::Ptr<cv::AKAZE>* instance) {
		delete instance;
	}

	const cv::AKAZE* cv_PtrOfAKAZE_get_inner_ptr(const cv::Ptr<cv::AKAZE>* instance) {
		return instance->get();
	}

	cv::AKAZE* cv_PtrOfAKAZE_get_inner_ptr_mut(cv::Ptr<cv::AKAZE>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfAgastFeatureDetector_delete(cv::Ptr<cv::AgastFeatureDetector>* instance) {
		delete instance;
	}

	const cv::AgastFeatureDetector* cv_PtrOfAgastFeatureDetector_get_inner_ptr(const cv::Ptr<cv::AgastFeatureDetector>* instance) {
		return instance->get();
	}

	cv::AgastFeatureDetector* cv_PtrOfAgastFeatureDetector_get_inner_ptr_mut(cv::Ptr<cv::AgastFeatureDetector>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::BFMatcher>* cv_PtrOfBFMatcher_new(cv::BFMatcher* val) {
		return new cv::Ptr<cv::BFMatcher>(val);
	}
	
	void cv_PtrOfBFMatcher_delete(cv::Ptr<cv::BFMatcher>* instance) {
		delete instance;
	}

	const cv::BFMatcher* cv_PtrOfBFMatcher_get_inner_ptr(const cv::Ptr<cv::BFMatcher>* instance) {
		return instance->get();
	}

	cv::BFMatcher* cv_PtrOfBFMatcher_get_inner_ptr_mut(cv::Ptr<cv::BFMatcher>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::BRISK>* cv_PtrOfBRISK_new(cv::BRISK* val) {
		return new cv::Ptr<cv::BRISK>(val);
	}
	
	void cv_PtrOfBRISK_delete(cv::Ptr<cv::BRISK>* instance) {
		delete instance;
	}

	const cv::BRISK* cv_PtrOfBRISK_get_inner_ptr(const cv::Ptr<cv::BRISK>* instance) {
		return instance->get();
	}

	cv::BRISK* cv_PtrOfBRISK_get_inner_ptr_mut(cv::Ptr<cv::BRISK>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfDescriptorMatcher_delete(cv::Ptr<cv::DescriptorMatcher>* instance) {
		delete instance;
	}

	const cv::DescriptorMatcher* cv_PtrOfDescriptorMatcher_get_inner_ptr(const cv::Ptr<cv::DescriptorMatcher>* instance) {
		return instance->get();
	}

	cv::DescriptorMatcher* cv_PtrOfDescriptorMatcher_get_inner_ptr_mut(cv::Ptr<cv::DescriptorMatcher>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfFastFeatureDetector_delete(cv::Ptr<cv::FastFeatureDetector>* instance) {
		delete instance;
	}

	const cv::FastFeatureDetector* cv_PtrOfFastFeatureDetector_get_inner_ptr(const cv::Ptr<cv::FastFeatureDetector>* instance) {
		return instance->get();
	}

	cv::FastFeatureDetector* cv_PtrOfFastFeatureDetector_get_inner_ptr_mut(cv::Ptr<cv::FastFeatureDetector>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::Feature2D>* cv_PtrOfFeature2D_new(cv::Feature2D* val) {
		return new cv::Ptr<cv::Feature2D>(val);
	}
	
	void cv_PtrOfFeature2D_delete(cv::Ptr<cv::Feature2D>* instance) {
		delete instance;
	}

	const cv::Feature2D* cv_PtrOfFeature2D_get_inner_ptr(const cv::Ptr<cv::Feature2D>* instance) {
		return instance->get();
	}

	cv::Feature2D* cv_PtrOfFeature2D_get_inner_ptr_mut(cv::Ptr<cv::Feature2D>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::FlannBasedMatcher>* cv_PtrOfFlannBasedMatcher_new(cv::FlannBasedMatcher* val) {
		return new cv::Ptr<cv::FlannBasedMatcher>(val);
	}
	
	void cv_PtrOfFlannBasedMatcher_delete(cv::Ptr<cv::FlannBasedMatcher>* instance) {
		delete instance;
	}

	const cv::FlannBasedMatcher* cv_PtrOfFlannBasedMatcher_get_inner_ptr(const cv::Ptr<cv::FlannBasedMatcher>* instance) {
		return instance->get();
	}

	cv::FlannBasedMatcher* cv_PtrOfFlannBasedMatcher_get_inner_ptr_mut(cv::Ptr<cv::FlannBasedMatcher>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfGFTTDetector_delete(cv::Ptr<cv::GFTTDetector>* instance) {
		delete instance;
	}

	const cv::GFTTDetector* cv_PtrOfGFTTDetector_get_inner_ptr(const cv::Ptr<cv::GFTTDetector>* instance) {
		return instance->get();
	}

	cv::GFTTDetector* cv_PtrOfGFTTDetector_get_inner_ptr_mut(cv::Ptr<cv::GFTTDetector>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfKAZE_delete(cv::Ptr<cv::KAZE>* instance) {
		delete instance;
	}

	const cv::KAZE* cv_PtrOfKAZE_get_inner_ptr(const cv::Ptr<cv::KAZE>* instance) {
		return instance->get();
	}

	cv::KAZE* cv_PtrOfKAZE_get_inner_ptr_mut(cv::Ptr<cv::KAZE>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfMSER_delete(cv::Ptr<cv::MSER>* instance) {
		delete instance;
	}

	const cv::MSER* cv_PtrOfMSER_get_inner_ptr(const cv::Ptr<cv::MSER>* instance) {
		return instance->get();
	}

	cv::MSER* cv_PtrOfMSER_get_inner_ptr_mut(cv::Ptr<cv::MSER>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfORB_delete(cv::Ptr<cv::ORB>* instance) {
		delete instance;
	}

	const cv::ORB* cv_PtrOfORB_get_inner_ptr(const cv::Ptr<cv::ORB>* instance) {
		return instance->get();
	}

	cv::ORB* cv_PtrOfORB_get_inner_ptr_mut(cv::Ptr<cv::ORB>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::SIFT>* cv_PtrOfSIFT_new(cv::SIFT* val) {
		return new cv::Ptr<cv::SIFT>(val);
	}
	
	void cv_PtrOfSIFT_delete(cv::Ptr<cv::SIFT>* instance) {
		delete instance;
	}

	const cv::SIFT* cv_PtrOfSIFT_get_inner_ptr(const cv::Ptr<cv::SIFT>* instance) {
		return instance->get();
	}

	cv::SIFT* cv_PtrOfSIFT_get_inner_ptr_mut(cv::Ptr<cv::SIFT>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::SimpleBlobDetector>* cv_PtrOfSimpleBlobDetector_new(cv::SimpleBlobDetector* val) {
		return new cv::Ptr<cv::SimpleBlobDetector>(val);
	}
	
	void cv_PtrOfSimpleBlobDetector_delete(cv::Ptr<cv::SimpleBlobDetector>* instance) {
		delete instance;
	}

	const cv::SimpleBlobDetector* cv_PtrOfSimpleBlobDetector_get_inner_ptr(const cv::Ptr<cv::SimpleBlobDetector>* instance) {
		return instance->get();
	}

	cv::SimpleBlobDetector* cv_PtrOfSimpleBlobDetector_get_inner_ptr_mut(cv::Ptr<cv::SimpleBlobDetector>* instance) {
		return instance->get();
	}
}

