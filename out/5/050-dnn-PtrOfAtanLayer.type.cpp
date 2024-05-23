extern "C" {
	// cv::Ptr<cv::dnn::AtanLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::AtanLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::AtanLayer* cv_PtrLcv_dnn_AtanLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::AtanLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::AtanLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::AtanLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::AtanLayer* cv_PtrLcv_dnn_AtanLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::AtanLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::AtanLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::AtanLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::AtanLayer>* cv_PtrLcv_dnn_AtanLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::AtanLayer>();
	}

	// cv::Ptr<cv::dnn::AtanLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::AtanLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_AtanLayerG_delete(cv::Ptr<cv::dnn::AtanLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::AtanLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::AtanLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_AtanLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::AtanLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::AtanLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::AtanLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_AtanLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::AtanLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::AtanLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::AtanLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_AtanLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::AtanLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::AtanLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::AtanLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::AtanLayer"]), _)]),
	cv::Ptr<cv::dnn::AtanLayer>* cv_PtrLcv_dnn_AtanLayerG_new_const_AtanLayer(cv::dnn::AtanLayer* val) {
			return new cv::Ptr<cv::dnn::AtanLayer>(val);
	}

}

