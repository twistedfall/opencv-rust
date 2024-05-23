extern "C" {
	// cv::Ptr<cv::videostab::InpaintingPipeline>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::InpaintingPipeline>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::InpaintingPipeline* cv_PtrLcv_videostab_InpaintingPipelineG_getInnerPtr_const(const cv::Ptr<cv::videostab::InpaintingPipeline>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::InpaintingPipeline>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::InpaintingPipeline>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::InpaintingPipeline* cv_PtrLcv_videostab_InpaintingPipelineG_getInnerPtrMut(cv::Ptr<cv::videostab::InpaintingPipeline>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::InpaintingPipeline>::new_null() generated
	// ("cv::Ptr<cv::videostab::InpaintingPipeline>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::InpaintingPipeline>* cv_PtrLcv_videostab_InpaintingPipelineG_new_null_const() {
			return new cv::Ptr<cv::videostab::InpaintingPipeline>();
	}

	// cv::Ptr<cv::videostab::InpaintingPipeline>::delete() generated
	// ("cv::Ptr<cv::videostab::InpaintingPipeline>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_InpaintingPipelineG_delete(cv::Ptr<cv::videostab::InpaintingPipeline>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::InpaintingPipeline>::to_PtrOfInpainterBase() generated
	// ("cv::Ptr<cv::videostab::InpaintingPipeline>::to_PtrOfInpainterBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::InpainterBase>* cv_PtrLcv_videostab_InpaintingPipelineG_to_PtrOfInpainterBase(cv::Ptr<cv::videostab::InpaintingPipeline>* instance) {
			return new cv::Ptr<cv::videostab::InpainterBase>(instance->dynamicCast<cv::videostab::InpainterBase>());
	}

	// cv::Ptr<cv::videostab::InpaintingPipeline>::new(TraitClass) generated
	// ("cv::Ptr<cv::videostab::InpaintingPipeline>::new", vec![(pred!(const, ["val"], ["cv::videostab::InpaintingPipeline"]), _)]),
	cv::Ptr<cv::videostab::InpaintingPipeline>* cv_PtrLcv_videostab_InpaintingPipelineG_new_const_InpaintingPipeline(cv::videostab::InpaintingPipeline* val) {
			return new cv::Ptr<cv::videostab::InpaintingPipeline>(val);
	}

}

