extern "C" {
	// cv::Ptr<cv::dnn::AsinLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::AsinLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::AsinLayer* cv_PtrLcv_dnn_AsinLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::AsinLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::AsinLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::AsinLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::AsinLayer* cv_PtrLcv_dnn_AsinLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::AsinLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::AsinLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::AsinLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::AsinLayer>* cv_PtrLcv_dnn_AsinLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::AsinLayer>();
	}

	// cv::Ptr<cv::dnn::AsinLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::AsinLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_AsinLayerG_delete(cv::Ptr<cv::dnn::AsinLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::AsinLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::AsinLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_AsinLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::AsinLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::AsinLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::AsinLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_AsinLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::AsinLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::AsinLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::AsinLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_AsinLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::AsinLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::AsinLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::AsinLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::AsinLayer"]), _)]),
	cv::Ptr<cv::dnn::AsinLayer>* cv_PtrLcv_dnn_AsinLayerG_new_const_AsinLayer(cv::dnn::AsinLayer* val) {
			return new cv::Ptr<cv::dnn::AsinLayer>(val);
	}

}

