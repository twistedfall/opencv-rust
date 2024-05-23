extern "C" {
	// cv::Ptr<cv::dnn::LogLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::LogLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::LogLayer* cv_PtrLcv_dnn_LogLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::LogLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::LogLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::LogLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::LogLayer* cv_PtrLcv_dnn_LogLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::LogLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::LogLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::LogLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::LogLayer>* cv_PtrLcv_dnn_LogLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::LogLayer>();
	}

	// cv::Ptr<cv::dnn::LogLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::LogLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_LogLayerG_delete(cv::Ptr<cv::dnn::LogLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::LogLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::LogLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_LogLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::LogLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::LogLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::LogLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_LogLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::LogLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::LogLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::LogLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_LogLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::LogLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::LogLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::LogLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::LogLayer"]), _)]),
	cv::Ptr<cv::dnn::LogLayer>* cv_PtrLcv_dnn_LogLayerG_new_const_LogLayer(cv::dnn::LogLayer* val) {
			return new cv::Ptr<cv::dnn::LogLayer>(val);
	}

}

