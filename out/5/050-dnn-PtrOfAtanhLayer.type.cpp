extern "C" {
	// cv::Ptr<cv::dnn::AtanhLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::AtanhLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::AtanhLayer* cv_PtrLcv_dnn_AtanhLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::AtanhLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::AtanhLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::AtanhLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::AtanhLayer* cv_PtrLcv_dnn_AtanhLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::AtanhLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::AtanhLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::AtanhLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::AtanhLayer>* cv_PtrLcv_dnn_AtanhLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::AtanhLayer>();
	}

	// cv::Ptr<cv::dnn::AtanhLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::AtanhLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_AtanhLayerG_delete(cv::Ptr<cv::dnn::AtanhLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::AtanhLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::AtanhLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_AtanhLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::AtanhLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::AtanhLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::AtanhLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_AtanhLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::AtanhLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::AtanhLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::AtanhLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_AtanhLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::AtanhLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::AtanhLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::AtanhLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::AtanhLayer"]), _)]),
	cv::Ptr<cv::dnn::AtanhLayer>* cv_PtrLcv_dnn_AtanhLayerG_new_const_AtanhLayer(cv::dnn::AtanhLayer* val) {
			return new cv::Ptr<cv::dnn::AtanhLayer>(val);
	}

}

