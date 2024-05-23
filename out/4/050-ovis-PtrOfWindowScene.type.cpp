extern "C" {
	// cv::Ptr<cv::ovis::WindowScene>::getInnerPtr() generated
	// ("cv::Ptr<cv::ovis::WindowScene>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ovis::WindowScene* cv_PtrLcv_ovis_WindowSceneG_getInnerPtr_const(const cv::Ptr<cv::ovis::WindowScene>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ovis::WindowScene>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ovis::WindowScene>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ovis::WindowScene* cv_PtrLcv_ovis_WindowSceneG_getInnerPtrMut(cv::Ptr<cv::ovis::WindowScene>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ovis::WindowScene>::new_null() generated
	// ("cv::Ptr<cv::ovis::WindowScene>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ovis::WindowScene>* cv_PtrLcv_ovis_WindowSceneG_new_null_const() {
			return new cv::Ptr<cv::ovis::WindowScene>();
	}

	// cv::Ptr<cv::ovis::WindowScene>::delete() generated
	// ("cv::Ptr<cv::ovis::WindowScene>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ovis_WindowSceneG_delete(cv::Ptr<cv::ovis::WindowScene>* instance) {
			delete instance;
	}

}

