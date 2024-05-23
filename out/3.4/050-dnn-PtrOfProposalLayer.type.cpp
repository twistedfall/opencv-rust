extern "C" {
	// cv::Ptr<cv::dnn::ProposalLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::ProposalLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::ProposalLayer* cv_PtrLcv_dnn_ProposalLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::ProposalLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ProposalLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::ProposalLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ProposalLayer* cv_PtrLcv_dnn_ProposalLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::ProposalLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ProposalLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::ProposalLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::ProposalLayer>* cv_PtrLcv_dnn_ProposalLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::ProposalLayer>();
	}

	// cv::Ptr<cv::dnn::ProposalLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::ProposalLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_ProposalLayerG_delete(cv::Ptr<cv::dnn::ProposalLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::ProposalLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::ProposalLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_ProposalLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::ProposalLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::ProposalLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::ProposalLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_ProposalLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::ProposalLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::ProposalLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::ProposalLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::ProposalLayer"]), _)]),
	cv::Ptr<cv::dnn::ProposalLayer>* cv_PtrLcv_dnn_ProposalLayerG_new_const_ProposalLayer(cv::dnn::ProposalLayer* val) {
			return new cv::Ptr<cv::dnn::ProposalLayer>(val);
	}

}

