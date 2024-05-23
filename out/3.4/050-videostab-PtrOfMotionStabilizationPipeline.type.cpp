extern "C" {
	// cv::Ptr<cv::videostab::MotionStabilizationPipeline>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::MotionStabilizationPipeline>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::MotionStabilizationPipeline* cv_PtrLcv_videostab_MotionStabilizationPipelineG_getInnerPtr_const(const cv::Ptr<cv::videostab::MotionStabilizationPipeline>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::MotionStabilizationPipeline>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::MotionStabilizationPipeline>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::MotionStabilizationPipeline* cv_PtrLcv_videostab_MotionStabilizationPipelineG_getInnerPtrMut(cv::Ptr<cv::videostab::MotionStabilizationPipeline>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::MotionStabilizationPipeline>::new_null() generated
	// ("cv::Ptr<cv::videostab::MotionStabilizationPipeline>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::MotionStabilizationPipeline>* cv_PtrLcv_videostab_MotionStabilizationPipelineG_new_null_const() {
			return new cv::Ptr<cv::videostab::MotionStabilizationPipeline>();
	}

	// cv::Ptr<cv::videostab::MotionStabilizationPipeline>::delete() generated
	// ("cv::Ptr<cv::videostab::MotionStabilizationPipeline>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_MotionStabilizationPipelineG_delete(cv::Ptr<cv::videostab::MotionStabilizationPipeline>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::MotionStabilizationPipeline>::to_PtrOfIMotionStabilizer() generated
	// ("cv::Ptr<cv::videostab::MotionStabilizationPipeline>::to_PtrOfIMotionStabilizer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::IMotionStabilizer>* cv_PtrLcv_videostab_MotionStabilizationPipelineG_to_PtrOfIMotionStabilizer(cv::Ptr<cv::videostab::MotionStabilizationPipeline>* instance) {
			return new cv::Ptr<cv::videostab::IMotionStabilizer>(instance->dynamicCast<cv::videostab::IMotionStabilizer>());
	}

	// cv::Ptr<cv::videostab::MotionStabilizationPipeline>::new(TraitClass) generated
	// ("cv::Ptr<cv::videostab::MotionStabilizationPipeline>::new", vec![(pred!(const, ["val"], ["cv::videostab::MotionStabilizationPipeline"]), _)]),
	cv::Ptr<cv::videostab::MotionStabilizationPipeline>* cv_PtrLcv_videostab_MotionStabilizationPipelineG_new_const_MotionStabilizationPipeline(cv::videostab::MotionStabilizationPipeline* val) {
			return new cv::Ptr<cv::videostab::MotionStabilizationPipeline>(val);
	}

}

