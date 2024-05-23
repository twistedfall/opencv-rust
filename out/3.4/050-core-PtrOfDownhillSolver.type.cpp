extern "C" {
	// cv::Ptr<cv::DownhillSolver>::getInnerPtr() generated
	// ("cv::Ptr<cv::DownhillSolver>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::DownhillSolver* cv_PtrLcv_DownhillSolverG_getInnerPtr_const(const cv::Ptr<cv::DownhillSolver>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::DownhillSolver>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::DownhillSolver>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::DownhillSolver* cv_PtrLcv_DownhillSolverG_getInnerPtrMut(cv::Ptr<cv::DownhillSolver>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::DownhillSolver>::new_null() generated
	// ("cv::Ptr<cv::DownhillSolver>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::DownhillSolver>* cv_PtrLcv_DownhillSolverG_new_null_const() {
			return new cv::Ptr<cv::DownhillSolver>();
	}

	// cv::Ptr<cv::DownhillSolver>::delete() generated
	// ("cv::Ptr<cv::DownhillSolver>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_DownhillSolverG_delete(cv::Ptr<cv::DownhillSolver>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::DownhillSolver>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::DownhillSolver>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_DownhillSolverG_to_PtrOfAlgorithm(cv::Ptr<cv::DownhillSolver>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::DownhillSolver>::to_PtrOfMinProblemSolver() generated
	// ("cv::Ptr<cv::DownhillSolver>::to_PtrOfMinProblemSolver", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::MinProblemSolver>* cv_PtrLcv_DownhillSolverG_to_PtrOfMinProblemSolver(cv::Ptr<cv::DownhillSolver>* instance) {
			return new cv::Ptr<cv::MinProblemSolver>(instance->dynamicCast<cv::MinProblemSolver>());
	}

}

