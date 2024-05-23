extern "C" {
	// cv::Ptr<cv::dnn::PowerLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::PowerLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::PowerLayer* cv_PtrLcv_dnn_PowerLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::PowerLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::PowerLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::PowerLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::PowerLayer* cv_PtrLcv_dnn_PowerLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::PowerLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::PowerLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::PowerLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::PowerLayer>* cv_PtrLcv_dnn_PowerLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::PowerLayer>();
	}

	// cv::Ptr<cv::dnn::PowerLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::PowerLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_PowerLayerG_delete(cv::Ptr<cv::dnn::PowerLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::PowerLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::PowerLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_PowerLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::PowerLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::PowerLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::PowerLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_PowerLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::PowerLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::PowerLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::PowerLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_PowerLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::PowerLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::PowerLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::PowerLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::PowerLayer"]), _)]),
	cv::Ptr<cv::dnn::PowerLayer>* cv_PtrLcv_dnn_PowerLayerG_new_const_PowerLayer(cv::dnn::PowerLayer* val) {
			return new cv::Ptr<cv::dnn::PowerLayer>(val);
	}

}

