extern "C" {
	// cv::Ptr<cv::MinProblemSolver>::getInnerPtr() generated
	// ("cv::Ptr<cv::MinProblemSolver>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::MinProblemSolver* cv_PtrLcv_MinProblemSolverG_getInnerPtr_const(const cv::Ptr<cv::MinProblemSolver>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::MinProblemSolver>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::MinProblemSolver>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::MinProblemSolver* cv_PtrLcv_MinProblemSolverG_getInnerPtrMut(cv::Ptr<cv::MinProblemSolver>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::MinProblemSolver>::new_null() generated
	// ("cv::Ptr<cv::MinProblemSolver>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::MinProblemSolver>* cv_PtrLcv_MinProblemSolverG_new_null_const() {
			return new cv::Ptr<cv::MinProblemSolver>();
	}

	// cv::Ptr<cv::MinProblemSolver>::delete() generated
	// ("cv::Ptr<cv::MinProblemSolver>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_MinProblemSolverG_delete(cv::Ptr<cv::MinProblemSolver>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::MinProblemSolver>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::MinProblemSolver>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_MinProblemSolverG_to_PtrOfAlgorithm(cv::Ptr<cv::MinProblemSolver>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

