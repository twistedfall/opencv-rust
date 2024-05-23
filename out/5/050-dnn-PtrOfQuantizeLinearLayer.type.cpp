extern "C" {
	// cv::Ptr<cv::dnn::QuantizeLinearLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::QuantizeLinearLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::QuantizeLinearLayer* cv_PtrLcv_dnn_QuantizeLinearLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::QuantizeLinearLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::QuantizeLinearLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::QuantizeLinearLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::QuantizeLinearLayer* cv_PtrLcv_dnn_QuantizeLinearLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::QuantizeLinearLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::QuantizeLinearLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::QuantizeLinearLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::QuantizeLinearLayer>* cv_PtrLcv_dnn_QuantizeLinearLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::QuantizeLinearLayer>();
	}

	// cv::Ptr<cv::dnn::QuantizeLinearLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::QuantizeLinearLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_QuantizeLinearLayerG_delete(cv::Ptr<cv::dnn::QuantizeLinearLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::QuantizeLinearLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::QuantizeLinearLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_QuantizeLinearLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::QuantizeLinearLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::QuantizeLinearLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::QuantizeLinearLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_QuantizeLinearLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::QuantizeLinearLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::QuantizeLinearLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::QuantizeLinearLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::QuantizeLinearLayer"]), _)]),
	cv::Ptr<cv::dnn::QuantizeLinearLayer>* cv_PtrLcv_dnn_QuantizeLinearLayerG_new_const_QuantizeLinearLayer(cv::dnn::QuantizeLinearLayer* val) {
			return new cv::Ptr<cv::dnn::QuantizeLinearLayer>(val);
	}

}

