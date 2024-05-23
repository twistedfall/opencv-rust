extern "C" {
	// cv::Ptr<cv::ConjGradSolver>::getInnerPtr() generated
	// ("cv::Ptr<cv::ConjGradSolver>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ConjGradSolver* cv_PtrLcv_ConjGradSolverG_getInnerPtr_const(const cv::Ptr<cv::ConjGradSolver>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ConjGradSolver>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ConjGradSolver>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ConjGradSolver* cv_PtrLcv_ConjGradSolverG_getInnerPtrMut(cv::Ptr<cv::ConjGradSolver>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ConjGradSolver>::new_null() generated
	// ("cv::Ptr<cv::ConjGradSolver>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ConjGradSolver>* cv_PtrLcv_ConjGradSolverG_new_null_const() {
			return new cv::Ptr<cv::ConjGradSolver>();
	}

	// cv::Ptr<cv::ConjGradSolver>::delete() generated
	// ("cv::Ptr<cv::ConjGradSolver>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ConjGradSolverG_delete(cv::Ptr<cv::ConjGradSolver>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ConjGradSolver>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ConjGradSolver>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ConjGradSolverG_to_PtrOfAlgorithm(cv::Ptr<cv::ConjGradSolver>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::ConjGradSolver>::to_PtrOfMinProblemSolver() generated
	// ("cv::Ptr<cv::ConjGradSolver>::to_PtrOfMinProblemSolver", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::MinProblemSolver>* cv_PtrLcv_ConjGradSolverG_to_PtrOfMinProblemSolver(cv::Ptr<cv::ConjGradSolver>* instance) {
			return new cv::Ptr<cv::MinProblemSolver>(instance->dynamicCast<cv::MinProblemSolver>());
	}

}

