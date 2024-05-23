extern "C" {
	// cv::Ptr<cv::dnn::AcosLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::AcosLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::AcosLayer* cv_PtrLcv_dnn_AcosLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::AcosLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::AcosLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::AcosLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::AcosLayer* cv_PtrLcv_dnn_AcosLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::AcosLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::AcosLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::AcosLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::AcosLayer>* cv_PtrLcv_dnn_AcosLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::AcosLayer>();
	}

	// cv::Ptr<cv::dnn::AcosLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::AcosLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_AcosLayerG_delete(cv::Ptr<cv::dnn::AcosLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::AcosLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::AcosLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_AcosLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::AcosLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::AcosLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::AcosLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_AcosLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::AcosLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::AcosLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::AcosLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_AcosLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::AcosLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::AcosLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::AcosLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::AcosLayer"]), _)]),
	cv::Ptr<cv::dnn::AcosLayer>* cv_PtrLcv_dnn_AcosLayerG_new_const_AcosLayer(cv::dnn::AcosLayer* val) {
			return new cv::Ptr<cv::dnn::AcosLayer>(val);
	}

}

