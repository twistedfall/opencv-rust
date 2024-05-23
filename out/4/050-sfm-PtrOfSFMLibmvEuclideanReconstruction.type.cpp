extern "C" {
	// cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>::getInnerPtr() generated
	// ("cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::sfm::SFMLibmvEuclideanReconstruction* cv_PtrLcv_sfm_SFMLibmvEuclideanReconstructionG_getInnerPtr_const(const cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::sfm::SFMLibmvEuclideanReconstruction* cv_PtrLcv_sfm_SFMLibmvEuclideanReconstructionG_getInnerPtrMut(cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>::new_null() generated
	// ("cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>* cv_PtrLcv_sfm_SFMLibmvEuclideanReconstructionG_new_null_const() {
			return new cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>();
	}

	// cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>::delete() generated
	// ("cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_sfm_SFMLibmvEuclideanReconstructionG_delete(cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>::to_PtrOfBaseSFM() generated
	// ("cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>::to_PtrOfBaseSFM", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::sfm::BaseSFM>* cv_PtrLcv_sfm_SFMLibmvEuclideanReconstructionG_to_PtrOfBaseSFM(cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>* instance) {
			return new cv::Ptr<cv::sfm::BaseSFM>(instance->dynamicCast<cv::sfm::BaseSFM>());
	}

}

