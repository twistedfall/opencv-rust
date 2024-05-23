extern "C" {
	// cv::Ptr<cv::dnn::RoundLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::RoundLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::RoundLayer* cv_PtrLcv_dnn_RoundLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::RoundLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::RoundLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::RoundLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::RoundLayer* cv_PtrLcv_dnn_RoundLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::RoundLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::RoundLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::RoundLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::RoundLayer>* cv_PtrLcv_dnn_RoundLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::RoundLayer>();
	}

	// cv::Ptr<cv::dnn::RoundLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::RoundLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_RoundLayerG_delete(cv::Ptr<cv::dnn::RoundLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::RoundLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::RoundLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_RoundLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::RoundLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::RoundLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::RoundLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_RoundLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::RoundLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::RoundLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::RoundLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_RoundLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::RoundLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::RoundLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::RoundLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::RoundLayer"]), _)]),
	cv::Ptr<cv::dnn::RoundLayer>* cv_PtrLcv_dnn_RoundLayerG_new_const_RoundLayer(cv::dnn::RoundLayer* val) {
			return new cv::Ptr<cv::dnn::RoundLayer>(val);
	}

}

