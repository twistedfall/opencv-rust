extern "C" {
	// cv::Ptr<cv::dnn::TanLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::TanLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::TanLayer* cv_PtrLcv_dnn_TanLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::TanLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::TanLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::TanLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::TanLayer* cv_PtrLcv_dnn_TanLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::TanLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::TanLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::TanLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::TanLayer>* cv_PtrLcv_dnn_TanLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::TanLayer>();
	}

	// cv::Ptr<cv::dnn::TanLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::TanLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_TanLayerG_delete(cv::Ptr<cv::dnn::TanLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::TanLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::TanLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_TanLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::TanLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::TanLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::TanLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_TanLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::TanLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::TanLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::TanLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_TanLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::TanLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::TanLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::TanLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::TanLayer"]), _)]),
	cv::Ptr<cv::dnn::TanLayer>* cv_PtrLcv_dnn_TanLayerG_new_const_TanLayer(cv::dnn::TanLayer* val) {
			return new cv::Ptr<cv::dnn::TanLayer>(val);
	}

}

