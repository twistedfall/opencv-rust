extern "C" {
	// cv::Ptr<cv::LMSolver>::getInnerPtr() generated
	// ("cv::Ptr<cv::LMSolver>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::LMSolver* cv_PtrLcv_LMSolverG_getInnerPtr_const(const cv::Ptr<cv::LMSolver>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::LMSolver>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::LMSolver>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::LMSolver* cv_PtrLcv_LMSolverG_getInnerPtrMut(cv::Ptr<cv::LMSolver>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::LMSolver>::new_null() generated
	// ("cv::Ptr<cv::LMSolver>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::LMSolver>* cv_PtrLcv_LMSolverG_new_null_const() {
			return new cv::Ptr<cv::LMSolver>();
	}

	// cv::Ptr<cv::LMSolver>::delete() generated
	// ("cv::Ptr<cv::LMSolver>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_LMSolverG_delete(cv::Ptr<cv::LMSolver>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::LMSolver>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::LMSolver>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_LMSolverG_to_PtrOfAlgorithm(cv::Ptr<cv::LMSolver>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

