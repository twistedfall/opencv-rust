extern "C" {
	// cv::Ptr<cv::dnn::CoshLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::CoshLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::CoshLayer* cv_PtrLcv_dnn_CoshLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::CoshLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::CoshLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::CoshLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::CoshLayer* cv_PtrLcv_dnn_CoshLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::CoshLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::CoshLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::CoshLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::CoshLayer>* cv_PtrLcv_dnn_CoshLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::CoshLayer>();
	}

	// cv::Ptr<cv::dnn::CoshLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::CoshLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_CoshLayerG_delete(cv::Ptr<cv::dnn::CoshLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::CoshLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::CoshLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_CoshLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::CoshLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::CoshLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::CoshLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_CoshLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::CoshLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::CoshLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::CoshLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_CoshLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::CoshLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::CoshLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::CoshLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::CoshLayer"]), _)]),
	cv::Ptr<cv::dnn::CoshLayer>* cv_PtrLcv_dnn_CoshLayerG_new_const_CoshLayer(cv::dnn::CoshLayer* val) {
			return new cv::Ptr<cv::dnn::CoshLayer>(val);
	}

}

