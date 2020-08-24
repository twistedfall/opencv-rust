template struct Result<bool>;
template struct Result<cv::Mat*>;
template struct Result<cv::Point_<int>>;
template struct Result<cv::Ptr<cv::AlignMTB>*>;
template struct Result<cv::Ptr<cv::CalibrateDebevec>*>;
template struct Result<cv::Ptr<cv::CalibrateRobertson>*>;
template struct Result<cv::Ptr<cv::MergeDebevec>*>;
template struct Result<cv::Ptr<cv::MergeMertens>*>;
template struct Result<cv::Ptr<cv::MergeRobertson>*>;
template struct Result<cv::Ptr<cv::TonemapDrago>*>;
template struct Result<cv::Ptr<cv::TonemapMantiuk>*>;
template struct Result<cv::Ptr<cv::TonemapReinhard>*>;
template struct Result<cv::Ptr<cv::Tonemap>*>;
template struct Result<float>;
template struct Result<int>;
extern "C" {
	void cv_PtrOfAlignMTB_delete(cv::Ptr<cv::AlignMTB>* instance) {
		delete instance;
	}

	const cv::AlignMTB* cv_PtrOfAlignMTB_get_inner_ptr(const cv::Ptr<cv::AlignMTB>* instance) {
		return instance->get();
	}

	cv::AlignMTB* cv_PtrOfAlignMTB_get_inner_ptr_mut(cv::Ptr<cv::AlignMTB>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfCalibrateDebevec_delete(cv::Ptr<cv::CalibrateDebevec>* instance) {
		delete instance;
	}

	const cv::CalibrateDebevec* cv_PtrOfCalibrateDebevec_get_inner_ptr(const cv::Ptr<cv::CalibrateDebevec>* instance) {
		return instance->get();
	}

	cv::CalibrateDebevec* cv_PtrOfCalibrateDebevec_get_inner_ptr_mut(cv::Ptr<cv::CalibrateDebevec>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfCalibrateRobertson_delete(cv::Ptr<cv::CalibrateRobertson>* instance) {
		delete instance;
	}

	const cv::CalibrateRobertson* cv_PtrOfCalibrateRobertson_get_inner_ptr(const cv::Ptr<cv::CalibrateRobertson>* instance) {
		return instance->get();
	}

	cv::CalibrateRobertson* cv_PtrOfCalibrateRobertson_get_inner_ptr_mut(cv::Ptr<cv::CalibrateRobertson>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfMergeDebevec_delete(cv::Ptr<cv::MergeDebevec>* instance) {
		delete instance;
	}

	const cv::MergeDebevec* cv_PtrOfMergeDebevec_get_inner_ptr(const cv::Ptr<cv::MergeDebevec>* instance) {
		return instance->get();
	}

	cv::MergeDebevec* cv_PtrOfMergeDebevec_get_inner_ptr_mut(cv::Ptr<cv::MergeDebevec>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfMergeMertens_delete(cv::Ptr<cv::MergeMertens>* instance) {
		delete instance;
	}

	const cv::MergeMertens* cv_PtrOfMergeMertens_get_inner_ptr(const cv::Ptr<cv::MergeMertens>* instance) {
		return instance->get();
	}

	cv::MergeMertens* cv_PtrOfMergeMertens_get_inner_ptr_mut(cv::Ptr<cv::MergeMertens>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfMergeRobertson_delete(cv::Ptr<cv::MergeRobertson>* instance) {
		delete instance;
	}

	const cv::MergeRobertson* cv_PtrOfMergeRobertson_get_inner_ptr(const cv::Ptr<cv::MergeRobertson>* instance) {
		return instance->get();
	}

	cv::MergeRobertson* cv_PtrOfMergeRobertson_get_inner_ptr_mut(cv::Ptr<cv::MergeRobertson>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfTonemap_delete(cv::Ptr<cv::Tonemap>* instance) {
		delete instance;
	}

	const cv::Tonemap* cv_PtrOfTonemap_get_inner_ptr(const cv::Ptr<cv::Tonemap>* instance) {
		return instance->get();
	}

	cv::Tonemap* cv_PtrOfTonemap_get_inner_ptr_mut(cv::Ptr<cv::Tonemap>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfTonemapDrago_delete(cv::Ptr<cv::TonemapDrago>* instance) {
		delete instance;
	}

	const cv::TonemapDrago* cv_PtrOfTonemapDrago_get_inner_ptr(const cv::Ptr<cv::TonemapDrago>* instance) {
		return instance->get();
	}

	cv::TonemapDrago* cv_PtrOfTonemapDrago_get_inner_ptr_mut(cv::Ptr<cv::TonemapDrago>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfTonemapMantiuk_delete(cv::Ptr<cv::TonemapMantiuk>* instance) {
		delete instance;
	}

	const cv::TonemapMantiuk* cv_PtrOfTonemapMantiuk_get_inner_ptr(const cv::Ptr<cv::TonemapMantiuk>* instance) {
		return instance->get();
	}

	cv::TonemapMantiuk* cv_PtrOfTonemapMantiuk_get_inner_ptr_mut(cv::Ptr<cv::TonemapMantiuk>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfTonemapReinhard_delete(cv::Ptr<cv::TonemapReinhard>* instance) {
		delete instance;
	}

	const cv::TonemapReinhard* cv_PtrOfTonemapReinhard_get_inner_ptr(const cv::Ptr<cv::TonemapReinhard>* instance) {
		return instance->get();
	}

	cv::TonemapReinhard* cv_PtrOfTonemapReinhard_get_inner_ptr_mut(cv::Ptr<cv::TonemapReinhard>* instance) {
		return instance->get();
	}
}

