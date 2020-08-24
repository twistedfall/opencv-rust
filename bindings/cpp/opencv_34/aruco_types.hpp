template struct Result<bool>;
template struct Result<cv::Mat*>;
template struct Result<cv::Point3_<float>>;
template struct Result<cv::Ptr<cv::aruco::Board>*>;
template struct Result<cv::Ptr<cv::aruco::CharucoBoard>*>;
template struct Result<cv::Ptr<cv::aruco::DetectorParameters>*>;
template struct Result<cv::Ptr<cv::aruco::Dictionary>*>;
template struct Result<cv::Ptr<cv::aruco::GridBoard>*>;
template struct Result<cv::Size_<int>>;
template struct Result<cv::Vec<double, 3>>;
template struct Result<cv::Vec<float, 3>>;
template struct Result<cv::aruco::DetectorParameters*>;
template struct Result<cv::aruco::Dictionary*>;
template struct Result<double>;
template struct Result<float>;
template struct Result<int>;
template struct Result<std::vector<cv::Point3_<float>>*>;
template struct Result<std::vector<int>*>;
template struct Result<std::vector<std::vector<cv::Point3_<float>>>*>;
template struct Result<std::vector<std::vector<int>>*>;
extern "C" {
	cv::Ptr<cv::aruco::Board>* cv_PtrOfBoard_new(cv::aruco::Board* val) {
		return new cv::Ptr<cv::aruco::Board>(val);
	}
	
	void cv_PtrOfBoard_delete(cv::Ptr<cv::aruco::Board>* instance) {
		delete instance;
	}

	const cv::aruco::Board* cv_PtrOfBoard_get_inner_ptr(const cv::Ptr<cv::aruco::Board>* instance) {
		return instance->get();
	}

	cv::aruco::Board* cv_PtrOfBoard_get_inner_ptr_mut(cv::Ptr<cv::aruco::Board>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::aruco::CharucoBoard>* cv_PtrOfCharucoBoard_new(cv::aruco::CharucoBoard* val) {
		return new cv::Ptr<cv::aruco::CharucoBoard>(val);
	}
	
	void cv_PtrOfCharucoBoard_delete(cv::Ptr<cv::aruco::CharucoBoard>* instance) {
		delete instance;
	}

	const cv::aruco::CharucoBoard* cv_PtrOfCharucoBoard_get_inner_ptr(const cv::Ptr<cv::aruco::CharucoBoard>* instance) {
		return instance->get();
	}

	cv::aruco::CharucoBoard* cv_PtrOfCharucoBoard_get_inner_ptr_mut(cv::Ptr<cv::aruco::CharucoBoard>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::aruco::DetectorParameters>* cv_PtrOfDetectorParameters_new(cv::aruco::DetectorParameters* val) {
		return new cv::Ptr<cv::aruco::DetectorParameters>(val);
	}
	
	void cv_PtrOfDetectorParameters_delete(cv::Ptr<cv::aruco::DetectorParameters>* instance) {
		delete instance;
	}

	const cv::aruco::DetectorParameters* cv_PtrOfDetectorParameters_get_inner_ptr(const cv::Ptr<cv::aruco::DetectorParameters>* instance) {
		return instance->get();
	}

	cv::aruco::DetectorParameters* cv_PtrOfDetectorParameters_get_inner_ptr_mut(cv::Ptr<cv::aruco::DetectorParameters>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::aruco::Dictionary>* cv_PtrOfDictionary_new(cv::aruco::Dictionary* val) {
		return new cv::Ptr<cv::aruco::Dictionary>(val);
	}
	
	void cv_PtrOfDictionary_delete(cv::Ptr<cv::aruco::Dictionary>* instance) {
		delete instance;
	}

	const cv::aruco::Dictionary* cv_PtrOfDictionary_get_inner_ptr(const cv::Ptr<cv::aruco::Dictionary>* instance) {
		return instance->get();
	}

	cv::aruco::Dictionary* cv_PtrOfDictionary_get_inner_ptr_mut(cv::Ptr<cv::aruco::Dictionary>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::aruco::GridBoard>* cv_PtrOfGridBoard_new(cv::aruco::GridBoard* val) {
		return new cv::Ptr<cv::aruco::GridBoard>(val);
	}
	
	void cv_PtrOfGridBoard_delete(cv::Ptr<cv::aruco::GridBoard>* instance) {
		delete instance;
	}

	const cv::aruco::GridBoard* cv_PtrOfGridBoard_get_inner_ptr(const cv::Ptr<cv::aruco::GridBoard>* instance) {
		return instance->get();
	}

	cv::aruco::GridBoard* cv_PtrOfGridBoard_get_inner_ptr_mut(cv::Ptr<cv::aruco::GridBoard>* instance) {
		return instance->get();
	}
}

