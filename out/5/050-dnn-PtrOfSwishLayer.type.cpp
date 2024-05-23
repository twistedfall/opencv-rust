extern "C" {
	// cv::Ptr<cv::dnn::SwishLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::SwishLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::SwishLayer* cv_PtrLcv_dnn_SwishLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::SwishLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::SwishLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::SwishLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SwishLayer* cv_PtrLcv_dnn_SwishLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::SwishLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::SwishLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::SwishLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::SwishLayer>* cv_PtrLcv_dnn_SwishLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::SwishLayer>();
	}

	// cv::Ptr<cv::dnn::SwishLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::SwishLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_SwishLayerG_delete(cv::Ptr<cv::dnn::SwishLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::SwishLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::SwishLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_SwishLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::SwishLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::SwishLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::SwishLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_SwishLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::SwishLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::SwishLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::SwishLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_SwishLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::SwishLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::SwishLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::SwishLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::SwishLayer"]), _)]),
	cv::Ptr<cv::dnn::SwishLayer>* cv_PtrLcv_dnn_SwishLayerG_new_const_SwishLayer(cv::dnn::SwishLayer* val) {
			return new cv::Ptr<cv::dnn::SwishLayer>(val);
	}

}

