template struct Result<cv::Ptr<cv::xphoto::GrayworldWB>*>;
template struct Result<cv::Ptr<cv::xphoto::LearningBasedWB>*>;
template struct Result<cv::Ptr<cv::xphoto::SimpleWB>*>;
template struct Result<cv::Ptr<cv::xphoto::TonemapDurand>*>;
template struct Result<float>;
template struct Result<int>;
extern "C" {
	void cv_PtrOfGrayworldWB_delete(cv::Ptr<cv::xphoto::GrayworldWB>* instance) {
		delete instance;
	}

	const cv::xphoto::GrayworldWB* cv_PtrOfGrayworldWB_get_inner_ptr(const cv::Ptr<cv::xphoto::GrayworldWB>* instance) {
		return instance->get();
	}

	cv::xphoto::GrayworldWB* cv_PtrOfGrayworldWB_get_inner_ptr_mut(cv::Ptr<cv::xphoto::GrayworldWB>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfLearningBasedWB_delete(cv::Ptr<cv::xphoto::LearningBasedWB>* instance) {
		delete instance;
	}

	const cv::xphoto::LearningBasedWB* cv_PtrOfLearningBasedWB_get_inner_ptr(const cv::Ptr<cv::xphoto::LearningBasedWB>* instance) {
		return instance->get();
	}

	cv::xphoto::LearningBasedWB* cv_PtrOfLearningBasedWB_get_inner_ptr_mut(cv::Ptr<cv::xphoto::LearningBasedWB>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfSimpleWB_delete(cv::Ptr<cv::xphoto::SimpleWB>* instance) {
		delete instance;
	}

	const cv::xphoto::SimpleWB* cv_PtrOfSimpleWB_get_inner_ptr(const cv::Ptr<cv::xphoto::SimpleWB>* instance) {
		return instance->get();
	}

	cv::xphoto::SimpleWB* cv_PtrOfSimpleWB_get_inner_ptr_mut(cv::Ptr<cv::xphoto::SimpleWB>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfTonemapDurand_delete(cv::Ptr<cv::xphoto::TonemapDurand>* instance) {
		delete instance;
	}

	const cv::xphoto::TonemapDurand* cv_PtrOfTonemapDurand_get_inner_ptr(const cv::Ptr<cv::xphoto::TonemapDurand>* instance) {
		return instance->get();
	}

	cv::xphoto::TonemapDurand* cv_PtrOfTonemapDurand_get_inner_ptr_mut(cv::Ptr<cv::xphoto::TonemapDurand>* instance) {
		return instance->get();
	}
}

