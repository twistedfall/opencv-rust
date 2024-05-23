extern "C" {
	// cv::Ptr<cv::dnn::ExpLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::ExpLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::ExpLayer* cv_PtrLcv_dnn_ExpLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::ExpLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ExpLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::ExpLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ExpLayer* cv_PtrLcv_dnn_ExpLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::ExpLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ExpLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::ExpLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::ExpLayer>* cv_PtrLcv_dnn_ExpLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::ExpLayer>();
	}

	// cv::Ptr<cv::dnn::ExpLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::ExpLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_ExpLayerG_delete(cv::Ptr<cv::dnn::ExpLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::ExpLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::ExpLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_ExpLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::ExpLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::ExpLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::ExpLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_ExpLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::ExpLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::ExpLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::ExpLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_ExpLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::ExpLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

}

