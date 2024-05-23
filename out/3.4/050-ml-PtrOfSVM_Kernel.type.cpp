extern "C" {
	// cv::Ptr<cv::ml::SVM::Kernel>::getInnerPtr() generated
	// ("cv::Ptr<cv::ml::SVM::Kernel>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ml::SVM::Kernel* cv_PtrLcv_ml_SVM_KernelG_getInnerPtr_const(const cv::Ptr<cv::ml::SVM::Kernel>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ml::SVM::Kernel>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ml::SVM::Kernel>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ml::SVM::Kernel* cv_PtrLcv_ml_SVM_KernelG_getInnerPtrMut(cv::Ptr<cv::ml::SVM::Kernel>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ml::SVM::Kernel>::new_null() generated
	// ("cv::Ptr<cv::ml::SVM::Kernel>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ml::SVM::Kernel>* cv_PtrLcv_ml_SVM_KernelG_new_null_const() {
			return new cv::Ptr<cv::ml::SVM::Kernel>();
	}

	// cv::Ptr<cv::ml::SVM::Kernel>::delete() generated
	// ("cv::Ptr<cv::ml::SVM::Kernel>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ml_SVM_KernelG_delete(cv::Ptr<cv::ml::SVM::Kernel>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ml::SVM::Kernel>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ml::SVM::Kernel>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ml_SVM_KernelG_to_PtrOfAlgorithm(cv::Ptr<cv::ml::SVM::Kernel>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

