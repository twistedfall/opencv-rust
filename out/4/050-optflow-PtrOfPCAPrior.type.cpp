extern "C" {
	// cv::Ptr<cv::optflow::PCAPrior>::getInnerPtr() generated
	// ("cv::Ptr<cv::optflow::PCAPrior>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::optflow::PCAPrior* cv_PtrLcv_optflow_PCAPriorG_getInnerPtr_const(const cv::Ptr<cv::optflow::PCAPrior>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::optflow::PCAPrior>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::optflow::PCAPrior>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::optflow::PCAPrior* cv_PtrLcv_optflow_PCAPriorG_getInnerPtrMut(cv::Ptr<cv::optflow::PCAPrior>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::optflow::PCAPrior>::new_null() generated
	// ("cv::Ptr<cv::optflow::PCAPrior>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::optflow::PCAPrior>* cv_PtrLcv_optflow_PCAPriorG_new_null_const() {
			return new cv::Ptr<cv::optflow::PCAPrior>();
	}

	// cv::Ptr<cv::optflow::PCAPrior>::delete() generated
	// ("cv::Ptr<cv::optflow::PCAPrior>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_optflow_PCAPriorG_delete(cv::Ptr<cv::optflow::PCAPrior>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::optflow::PCAPrior>::new(TraitClass) generated
	// ("cv::Ptr<cv::optflow::PCAPrior>::new", vec![(pred!(const, ["val"], ["cv::optflow::PCAPrior"]), _)]),
	cv::Ptr<cv::optflow::PCAPrior>* cv_PtrLcv_optflow_PCAPriorG_new_const_PCAPrior(cv::optflow::PCAPrior* val) {
			return new cv::Ptr<cv::optflow::PCAPrior>(val);
	}

}

