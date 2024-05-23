extern "C" {
	// cv::Ptr<cv::ml::ParamGrid>::getInnerPtr() generated
	// ("cv::Ptr<cv::ml::ParamGrid>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ml::ParamGrid* cv_PtrLcv_ml_ParamGridG_getInnerPtr_const(const cv::Ptr<cv::ml::ParamGrid>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ml::ParamGrid>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ml::ParamGrid>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ml::ParamGrid* cv_PtrLcv_ml_ParamGridG_getInnerPtrMut(cv::Ptr<cv::ml::ParamGrid>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ml::ParamGrid>::new_null() generated
	// ("cv::Ptr<cv::ml::ParamGrid>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ml::ParamGrid>* cv_PtrLcv_ml_ParamGridG_new_null_const() {
			return new cv::Ptr<cv::ml::ParamGrid>();
	}

	// cv::Ptr<cv::ml::ParamGrid>::delete() generated
	// ("cv::Ptr<cv::ml::ParamGrid>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ml_ParamGridG_delete(cv::Ptr<cv::ml::ParamGrid>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ml::ParamGrid>::new(TraitClass) generated
	// ("cv::Ptr<cv::ml::ParamGrid>::new", vec![(pred!(const, ["val"], ["cv::ml::ParamGrid"]), _)]),
	cv::Ptr<cv::ml::ParamGrid>* cv_PtrLcv_ml_ParamGridG_new_const_ParamGrid(cv::ml::ParamGrid* val) {
			return new cv::Ptr<cv::ml::ParamGrid>(val);
	}

}

